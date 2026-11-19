use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::catalog::accessories::all_end_brackets;
use crate::catalog::{TerminalCatalog, TerminalSpec};

const EMBEDDED_PDF_IMPORT_RULES_JSON: &str =
    include_str!("../../assets/pdf_import_rules.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderAliases {
    pub bmk: Vec<String>,
    pub quantity: Vec<String>,
    pub description: Vec<String>,
    pub type_number: Vec<String>,
    pub supplier: Vec<String>,
    pub article_number: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturerRule {
    pub canonical_name: String,
    pub supplier_aliases: Vec<String>,
    pub article_prefixes: Vec<String>,
    pub article_patterns: Vec<String>,
    pub type_patterns: Vec<String>,
    pub default_end_bracket_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessoryPatternSet {
    pub description_keywords: Vec<String>,
    pub article_regexes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessoryRuleConfig {
    pub jumper: AccessoryPatternSet,
    pub end_clamp: AccessoryPatternSet,
    pub end_plate: AccessoryPatternSet,
    pub nt_accessory: AccessoryPatternSet,
    pub misc_accessory: AccessoryPatternSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfImportRules {
    pub bmk_regex: String,
    pub header_aliases: HeaderAliases,
    pub footer_ignore_patterns: Vec<String>,
    pub manufacturers: Vec<ManufacturerRule>,
    pub accessories: AccessoryRuleConfig,
}

impl PdfImportRules {
    pub fn load_embedded() -> Self {
        serde_json::from_str(EMBEDDED_PDF_IMPORT_RULES_JSON)
            .expect("Embedded assets/pdf_import_rules.json must be valid JSON")
    }

    pub fn from_json_str(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }

    pub fn compile(&self) -> Result<CompiledImportRules, regex::Error> {
        CompiledImportRules::from_rules(self)
    }
}

#[derive(Debug, Clone)]
pub struct CompiledManufacturerRule {
    pub canonical_name: String,
    pub supplier_aliases_upper: Vec<String>,
    pub article_prefixes_upper: Vec<String>,
    pub article_patterns: Vec<Regex>,
    pub type_patterns: Vec<Regex>,
    pub default_end_bracket_id: String,
}

#[derive(Debug, Clone)]
pub struct CompiledAccessoryPatterns {
    pub description_keywords_lower: Vec<String>,
    pub article_regexes: Vec<Regex>,
}

impl CompiledAccessoryPatterns {
    fn from_set(set: &AccessoryPatternSet) -> Result<Self, regex::Error> {
        let mut article_regexes = Vec::with_capacity(set.article_regexes.len());
        for pat in &set.article_regexes {
            article_regexes.push(Regex::new(pat)?);
        }
        Ok(Self {
            description_keywords_lower: set
                .description_keywords
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
            article_regexes,
        })
    }

    fn matches(&self, article: &str, type_num: &str, description: &str) -> bool {
        let a = article.trim();
        let t = type_num.trim();
        if !a.is_empty() && self.article_regexes.iter().any(|re| re.is_match(a)) {
            return true;
        }
        if !t.is_empty() && self.article_regexes.iter().any(|re| re.is_match(t)) {
            return true;
        }
        let desc_lower = description.to_lowercase();
        !desc_lower.is_empty()
            && self
                .description_keywords_lower
                .iter()
                .any(|kw| desc_lower.contains(kw))
    }
}

#[derive(Debug, Clone)]
pub struct CompiledImportRules {
    pub raw: PdfImportRules,
    pub bmk_regex: Regex,
    pub footer_regexes: Vec<Regex>,
    pub manufacturers: Vec<CompiledManufacturerRule>,
    pub jumper: CompiledAccessoryPatterns,
    pub end_clamp: CompiledAccessoryPatterns,
    pub end_plate: CompiledAccessoryPatterns,
    pub nt_accessory: CompiledAccessoryPatterns,
    pub misc_accessory: CompiledAccessoryPatterns,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManufacturerDetection {
    Supported(String),
    Foreign(String),
    Unknown,
}

#[derive(Debug, Clone)]
pub enum BomItemClassification<'a> {
    Terminal(&'a TerminalSpec),
    Jumper,
    EndClamp { matched_bracket_id: Option<String> },
    EndPlate,
    NtAccessory,
    MiscAccessory,
    UnknownTerminalArticle,
}

impl CompiledImportRules {
    pub fn load_default() -> Self {
        PdfImportRules::load_embedded()
            .compile()
            .expect("Embedded pdf_import_rules.json regexes must compile cleanly")
    }

    pub fn from_rules(rules: &PdfImportRules) -> Result<Self, regex::Error> {
        let bmk_regex = Regex::new(&rules.bmk_regex)?;
        let mut footer_regexes = Vec::with_capacity(rules.footer_ignore_patterns.len());
        for pat in &rules.footer_ignore_patterns {
            footer_regexes.push(Regex::new(pat)?);
        }
        let mut manufacturers = Vec::with_capacity(rules.manufacturers.len());
        for m in &rules.manufacturers {
            let mut article_patterns = Vec::with_capacity(m.article_patterns.len());
            for pat in &m.article_patterns {
                article_patterns.push(Regex::new(pat)?);
            }
            let mut type_patterns = Vec::with_capacity(m.type_patterns.len());
            for pat in &m.type_patterns {
                type_patterns.push(Regex::new(pat)?);
            }
            manufacturers.push(CompiledManufacturerRule {
                canonical_name: m.canonical_name.clone(),
                supplier_aliases_upper: m
                    .supplier_aliases
                    .iter()
                    .map(|s| s.to_uppercase())
                    .collect(),
                article_prefixes_upper: m
                    .article_prefixes
                    .iter()
                    .map(|s| s.to_uppercase())
                    .collect(),
                article_patterns,
                type_patterns,
                default_end_bracket_id: m.default_end_bracket_id.clone(),
            });
        }
        Ok(Self {
            raw: rules.clone(),
            bmk_regex,
            footer_regexes,
            manufacturers,
            jumper: CompiledAccessoryPatterns::from_set(&rules.accessories.jumper)?,
            end_clamp: CompiledAccessoryPatterns::from_set(&rules.accessories.end_clamp)?,
            end_plate: CompiledAccessoryPatterns::from_set(&rules.accessories.end_plate)?,
            nt_accessory: CompiledAccessoryPatterns::from_set(&rules.accessories.nt_accessory)?,
            misc_accessory: CompiledAccessoryPatterns::from_set(&rules.accessories.misc_accessory)?,
        })
    }

    /// Strictly evaluates the BMK filter regex ONLY against the BMK column value.
    pub fn is_terminal_bmk(&self, bmk: &str) -> bool {
        let trimmed = bmk.trim();
        !trimmed.is_empty() && self.bmk_regex.is_match(trimmed)
    }

    pub fn is_footer_line(&self, text: &str) -> bool {
        let trimmed = text.trim();
        !trimmed.is_empty() && self.footer_regexes.iter().any(|re| re.is_match(trimmed))
    }

    pub fn default_end_bracket_for(&self, manufacturer: &str) -> &str {
        self.manufacturers
            .iter()
            .find(|m| m.canonical_name.eq_ignore_ascii_case(manufacturer))
            .map(|m| m.default_end_bracket_id.as_str())
            .unwrap_or("phoenix-clipfix-35-5")
    }

    /// Extracts `(prefix_opt, clean_article)` from an article field like `"WAGO.2204-1201"` or `"PXC.3209510"`.
    pub fn split_article_prefix(article: &str) -> (Option<String>, String) {
        let trimmed = article.trim();
        for sep in ['.', ':', '/'] {
            if let Some((head, tail)) = trimmed.split_once(sep) {
                let h = head.trim();
                let t = tail.trim();
                if !h.is_empty()
                    && !t.is_empty()
                    && h.len() <= 16
                    && h.chars().all(|c| c.is_ascii_alphabetic() || c == ' ')
                {
                    return (Some(h.to_uppercase()), t.to_string());
                }
            }
        }
        (None, trimmed.to_string())
    }

    /// 3-stage manufacturer recognition:
    /// 1) `supplier` column -> 2) article number prefix / patterns -> 3) direct DB lookup.
    pub fn detect_manufacturer(
        &self,
        supplier: &str,
        article: &str,
        type_number: &str,
        catalog: &TerminalCatalog,
    ) -> ManufacturerDetection {
        let sup_upper = supplier.trim().to_uppercase();
        if !sup_upper.is_empty() {
            if let Some(m) = self.manufacturers.iter().find(|m| {
                m.supplier_aliases_upper
                    .iter()
                    .any(|alias| alias == &sup_upper || sup_upper.contains(alias))
            }) {
                return ManufacturerDetection::Supported(m.canonical_name.clone());
            }
            return ManufacturerDetection::Foreign(supplier.trim().to_string());
        }

        let (prefix_opt, clean_art) = Self::split_article_prefix(article);
        if let Some(prefix) = prefix_opt {
            if let Some(m) = self
                .manufacturers
                .iter()
                .find(|m| m.article_prefixes_upper.iter().any(|p| p == &prefix))
            {
                return ManufacturerDetection::Supported(m.canonical_name.clone());
            }
            return ManufacturerDetection::Foreign(prefix);
        }

        // Fallback Stage 2b: check dash/space prefixes like "WAGO 2002-1201"
        let upper_art = clean_art.to_uppercase();
        for m in &self.manufacturers {
            for p in &m.article_prefixes_upper {
                if upper_art.starts_with(&format!("{p} "))
                    || upper_art.starts_with(&format!("{p}-"))
                {
                    return ManufacturerDetection::Supported(m.canonical_name.clone());
                }
            }
        }

        // Stage 3a: direct catalog match (article / type)
        if let Some(spec) = catalog.find_by_article_or_type(None, &clean_art, type_number) {
            return ManufacturerDetection::Supported(spec.manufacturer.clone());
        }
        if let Some(eb) = find_matching_end_bracket(&clean_art, type_number, None) {
            return ManufacturerDetection::Supported(eb.manufacturer.to_string());
        }

        // Stage 3b: article/type regex patterns
        let type_clean = type_number.trim();
        for m in &self.manufacturers {
            if (!clean_art.is_empty() && m.article_patterns.iter().any(|re| re.is_match(&clean_art)))
                || (!type_clean.is_empty()
                    && m.type_patterns.iter().any(|re| re.is_match(type_clean)))
            {
                return ManufacturerDetection::Supported(m.canonical_name.clone());
            }
        }

        ManufacturerDetection::Unknown
    }

    /// Classifies a BOM item belonging to one of our 3 supported manufacturers.
    /// Prio 1: Direct lookup in existing EndBracket & Terminal DBs.
    /// Prio 2: Configurable regex/keyword accessory rules as fallback.
    pub fn classify_item<'a>(
        &self,
        manufacturer: &str,
        article: &str,
        type_number: &str,
        description: &str,
        catalog: &'a TerminalCatalog,
    ) -> BomItemClassification<'a> {
        let (_, clean_art) = Self::split_article_prefix(article);
        let clean_type = type_number.trim();

        // Prio 1a: EndBracket DB
        if let Some(eb) = find_matching_end_bracket(&clean_art, clean_type, Some(manufacturer)) {
            return BomItemClassification::EndClamp {
                matched_bracket_id: Some(eb.id.to_string()),
            };
        }

        // Prio 1b: Terminal DB (busbar carriers count as NT accessory)
        if let Some(spec) =
            catalog.find_by_article_or_type(Some(manufacturer), &clean_art, clean_type)
        {
            if spec.is_busbar_carrier() {
                return BomItemClassification::NtAccessory;
            }
            return BomItemClassification::Terminal(spec);
        }

        // Prio 2: Configurable accessory rule fallback
        if self.jumper.matches(&clean_art, clean_type, description) {
            return BomItemClassification::Jumper;
        }
        if self.end_clamp.matches(&clean_art, clean_type, description) {
            return BomItemClassification::EndClamp {
                matched_bracket_id: None,
            };
        }
        if self.end_plate.matches(&clean_art, clean_type, description) {
            return BomItemClassification::EndPlate;
        }
        if self.nt_accessory.matches(&clean_art, clean_type, description) {
            return BomItemClassification::NtAccessory;
        }
        if self.misc_accessory.matches(&clean_art, clean_type, description) {
            return BomItemClassification::MiscAccessory;
        }

        BomItemClassification::UnknownTerminalArticle
    }
}

fn find_matching_end_bracket(
    clean_article: &str,
    type_number: &str,
    mfg_hint: Option<&str>,
) -> Option<&'static crate::catalog::accessories::EndBracketSpec> {
    fn norm(s: &str) -> String {
        s.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .flat_map(|c| c.to_lowercase())
            .collect()
    }
    let a = norm(clean_article);
    let t = norm(type_number);
    if a.is_empty() && t.is_empty() {
        return None;
    }
    all_end_brackets().iter().find(|eb| {
        if mfg_hint.is_some_and(|m| !eb.manufacturer.eq_ignore_ascii_case(m)) {
            return false;
        }
        let ea = norm(eb.article_number);
        let en = norm(eb.name);
        (!a.is_empty() && (ea == a || en == a)) || (!t.is_empty() && (ea == t || en == t))
    })
}
