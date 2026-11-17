pub mod accessories;

use serde::{Deserialize, Serialize};

use crate::catalog::accessories::{resolve_accessories, TerminalAccessories};
use crate::model::{ConnectionPoint, NumberingOrder, TerminalBlock, TierBehavior, TierColor};

/// Embedded hardcoded database of common WAGO, Phoenix Contact, and Weidmüller terminal blocks.
const EMBEDDED_SPECS_JSON: &str = include_str!("../../assets/terminal_specs.json");

/// Blueprint for a single tier within a catalog specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TierSpec {
    pub behavior: TierBehavior,
    pub color: TierColor,
}

/// Technical specification of a terminal block type from the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalSpec {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub series: String,
    pub levels: usize,
    pub width_tenths_mm: u16,
    pub numbering_order: NumberingOrder,
    pub description_de: String,
    pub description_en: String,
    pub tiers: Vec<TierSpec>,
    #[serde(default)]
    pub accessories: Option<TerminalAccessories>,
    /// Jumper chambers per tier (`Some(1)` = single slot, `Some(2)` = double slot; `None` infers from series).
    #[serde(default)]
    pub jumper_slots_per_tier: Option<u8>,
    /// Maximum comb-jumper poles available in standard accessories (e.g., 10, 5, 20).
    #[serde(default)]
    pub max_jumper_poles: Option<u8>,
    /// Optional manufacturer order / article number (e.g., "3209510", "1521850000").
    #[serde(default)]
    pub article_number: Option<String>,
    /// Optional explicit manufacturer product page URL.
    #[serde(default)]
    pub product_url: Option<String>,
}

impl TerminalSpec {
    /// Instantiates a new `TerminalBlock` with the given ID from this specification.
    pub fn instantiate_block(&self, block_id: u64) -> TerminalBlock {
        let tiers = self
            .tiers
            .iter()
            .enumerate()
            .map(|(idx, ts)| ConnectionPoint::new(idx, ts.behavior.clone(), ts.color))
            .collect();

        TerminalBlock {
            id: block_id,
            spec_id: self.id.clone(),
            spec_name: self.name.clone(),
            manufacturer: self.manufacturer.clone(),
            width_tenths_mm: self.width_tenths_mm,
            numbering_order: self.numbering_order,
            tiers,
        }
    }

    /// Counts how many tiers in this spec consume a sequential number.
    pub fn sequential_tier_count(&self) -> usize {
        self.tiers
            .iter()
            .filter(|t| t.behavior.consumes_sequence())
            .count()
    }

    /// Returns a compact pattern summary such as "L / L", "PE / L / N", or "N".
    pub fn pattern_summary(&self) -> String {
        self.tiers
            .iter()
            .map(|t| match &t.behavior {
                TierBehavior::Sequential => "L".to_string(),
                TierBehavior::Fixed(s) => s.clone(),
                TierBehavior::ProtectiveEarth => "PE".to_string(),
                TierBehavior::Unlabeled => "—".to_string(),
            })
            .collect::<Vec<_>>()
            .join(" / ")
    }

    /// Number of jumper/bridge chambers per tier (`1` = single chamber, `2` = dual chamber).
    pub fn effective_jumper_slots(&self) -> u8 {
        if let Some(slots) = self.jumper_slots_per_tier {
            return slots.clamp(1, 2);
        }
        let id = self.id.as_str();
        // Single-chamber series: compact 1.5mm², installation multi-tier (PTI/AITB/2003/2005/DLI),
        // disconnect/fuse/sensor terminals, high-current blocks, and legacy 280/281.
        if id.contains("2000-")
            || id.contains("2001-")
            || id.contains("2003-")
            || id.contains("2005-")
            || id.contains("2200-")
            || id.contains("2201-")
            || id.contains("2203-")
            || id.contains("2010-")
            || id.contains("2016-")
            || id.contains("2110-")
            || id.contains("2116-")
            || id.contains("2210-")
            || id.contains("2216-")
            || id.contains("2022-")
            || id.contains("2007-")
            || id.contains("280-")
            || id.contains("281-")
            || id.contains("285-")
            || id.contains("-161")
            || id.contains("-166")
            || id.contains("-168")
            || id.contains("-171")
            || id.contains("-181")
            || id.contains("-198")
            || id.contains("-260")
            || id.contains("-296")
            || id.contains("15s")
            || id.contains("pti-")
            || id.contains("ptio-")
            || id.contains("sti-")
            || id.contains("uti-")
            || id.contains("ptpower")
            || id.contains("pt-10")
            || id.contains("pt-16")
            || id.contains("pt-35")
            || id.contains("ptme")
            || id.contains("-hesi")
            || id.contains("-mt")
            || id.contains("-tg")
            || id.contains("uk-")
            || id.contains("uslkg")
            || id.contains("a2c-15")
            || id.contains("a3c-15")
            || id.contains("a4c-15")
            || id.contains("a2c-10")
            || id.contains("a2c-16")
            || id.contains("a2c-35")
            || id.contains("aio")
            || id.contains("aitb")
            || id.contains("dli")
            || id.contains("zdl")
            || id.contains("wdu-10")
            || id.contains("wdu-16")
            || id.contains("wdu-35")
            || id.contains("wtr")
            || id.contains("wsi")
        {
            1
        } else {
            2
        }
    }

    /// Maximum consecutive poles offered in standard comb jumper accessories for this block.
    pub fn effective_max_jumper_poles(&self) -> u8 {
        if let Some(poles) = self.max_jumper_poles {
            return poles.max(2);
        }
        let id = self.id.as_str();
        if id.contains("285-")
            || id.contains("ptpower")
            || id.contains("2010-")
            || id.contains("2016-")
            || id.contains("2110-")
            || id.contains("2116-")
            || id.contains("2210-")
            || id.contains("2216-")
            || id.contains("pt-10")
            || id.contains("pt-16")
            || id.contains("pt-35")
            || id.contains("ut-10")
            || id.contains("ut-16")
            || id.contains("a2c-10")
            || id.contains("a2c-16")
            || id.contains("a2c-35")
            || id.contains("wdu-10")
            || id.contains("wdu-16")
            || id.contains("wdu-35")
            || id.contains("wpe-16")
            || id.contains("wpe-35")
        {
            2
        } else if id.contains("2006-")
            || id.contains("2106-")
            || id.contains("2206-")
            || id.contains("pt-6")
            || id.contains("pti-6")
            || id.contains("ut-6")
            || id.contains("a2c-6")
            || id.contains("aitb-6")
            || id.contains("wdu-6")
            || id.contains("zdu-6")
        {
            5
        } else {
            10
        }
    }

    /// Compatibility key for horizontal comb jumpers (manufacturer + pitch + jumper family).
    pub fn jumper_family_key(&self) -> String {
        let family = match self.manufacturer.as_str() {
            "WAGO" => {
                if self.id.starts_with("wago-28") {
                    "280"
                } else {
                    "topjob"
                }
            }
            "Phoenix Contact" => {
                if self.id.contains("-ut") || self.id.contains("-uk") || self.id.contains("-uslkg") {
                    "fbs-ut"
                } else {
                    "fbs"
                }
            }
            "Weidmüller" => {
                if self.id.contains("wdu") || self.id.contains("wpe") || self.id.contains("wdk") || self.id.contains("dli") {
                    "wqv"
                } else {
                    "zqv"
                }
            }
            _ => "generic",
        };
        format!("{}:{}:{}", self.manufacturer, family, self.width_tenths_mm)
    }

    /// Comb jumper part number hint for `poles` poles on this terminal specification.
    pub fn jumper_part_hint(&self, poles: usize) -> String {
        let id = self.id.as_str();
        match self.manufacturer.as_str() {
            "WAGO" => {
                let prefix = if id.contains("2000") || id.contains("2200") {
                    "2000"
                } else if id.contains("2001") || id.contains("2201") {
                    "2001"
                } else if id.contains("2004") || id.contains("2005") || id.contains("2104") || id.contains("2204") {
                    "2004"
                } else if id.contains("2006") || id.contains("2106") || id.contains("2206") {
                    "2006"
                } else if id.contains("2010") || id.contains("2110") || id.contains("2210") {
                    "2010"
                } else if id.contains("2016") || id.contains("2116") || id.contains("2216") {
                    "2016"
                } else {
                    "2002"
                };
                format!("{prefix}-4{poles:02} ({poles}-fach)")
            }
            "Phoenix Contact" => {
                let pitch = if self.width_tenths_mm <= 38 {
                    "3,5"
                } else if self.width_tenths_mm <= 55 {
                    "5"
                } else if self.width_tenths_mm <= 65 {
                    "6"
                } else if self.width_tenths_mm <= 85 {
                    "8"
                } else {
                    "10"
                };
                format!("FBS {poles}-{pitch} ({poles}-fach)")
            }
            "Weidmüller" => {
                if id.contains("wdu") || id.contains("wpe") || id.contains("wdk") {
                    format!("WQV 2.5/{poles} ({poles}-fach)")
                } else if self.width_tenths_mm >= 60 {
                    format!("ZQV 4N/{poles} ({poles}-fach)")
                } else {
                    format!("ZQV 2.5N/{poles} ({poles}-fach)")
                }
            }
            _ => format!("{poles}-fach Kammbrücke"),
        }
    }

    /// Clean manufacturer product lookup URL for opening in desktop or mobile browser.
    pub fn effective_product_url(&self) -> String {
        if let Some(url) = &self.product_url
            && !url.is_empty()
        {
            return url.clone();
        }
        let short = self.name.split(" (").next().unwrap_or(&self.name).trim();
        let article = self
            .article_number
            .as_deref()
            .filter(|a| !a.is_empty())
            .unwrap_or(short);

        match self.manufacturer.as_str() {
            "WAGO" => {
                let clean_art = short.split('/').next().unwrap_or(short).trim();
                format!("https://www.wago.com/{clean_art}")
            }
            "Phoenix Contact" => {
                let q = url_encode(article);
                format!("https://www.phoenixcontact.com/de-de/suche?q={q}")
            }
            "Weidmüller" => {
                let q = url_encode(article);
                format!("https://eshop.weidmueller.com/de/search/?text={q}")
            }
            _ => format!("https://www.google.com/search?q={}", url_encode(short)),
        }
    }
}

fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' => out.push(b as char),
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

/// In-memory catalog loaded from the embedded JSON terminal database.
#[derive(Debug, Clone)]
pub struct TerminalCatalog {
    specs: Vec<TerminalSpec>,
}

impl Default for TerminalCatalog {
    fn default() -> Self {
        Self::load_embedded()
    }
}

impl TerminalCatalog {
    /// Loads the embedded JSON terminal catalog.
    pub fn load_embedded() -> Self {
        let specs: Vec<TerminalSpec> = serde_json::from_str(EMBEDDED_SPECS_JSON)
            .expect("Embedded terminal_specs.json must be valid JSON");
        Self { specs }
    }

    /// Rebuilds the catalog from embedded specs plus project-specific custom specs.
    pub fn reload_with_custom(&mut self, custom_specs: &[TerminalSpec]) {
        *self = Self::load_embedded();
        for spec in custom_specs {
            self.add_custom_spec(spec.clone());
        }
    }

    /// Adds or replaces a custom terminal specification in the active catalog.
    pub fn add_custom_spec(&mut self, spec: TerminalSpec) {
        if let Some(existing) = self.specs.iter_mut().find(|s| s.id == spec.id) {
            *existing = spec;
        } else {
            self.specs.push(spec);
        }
    }

    /// Returns all available terminal specifications.
    pub fn all(&self) -> &[TerminalSpec] {
        &self.specs
    }

    /// Finds a specification by its unique ID.
    pub fn find_by_id(&self, id: &str) -> Option<&TerminalSpec> {
        self.specs.iter().find(|s| s.id == id)
    }

    /// Returns the distinct manufacturer names in catalog order.
    pub fn manufacturers(&self) -> Vec<&str> {
        let mut out = Vec::new();
        for spec in &self.specs {
            if !out.contains(&spec.manufacturer.as_str()) {
                out.push(spec.manufacturer.as_str());
            }
        }
        out
    }

    /// Resolves matching accessories (end plate, jumpers, markers, end bracket) for `spec`.
    pub fn accessories_for(&self, spec: &TerminalSpec) -> TerminalAccessories {
        spec.accessories
            .clone()
            .unwrap_or_else(|| resolve_accessories(spec))
    }

    /// Applies a new specification to an existing `TerminalBlock`, preserving its `id` and replacing its physical tiers.
    pub fn apply_spec_to_block(block: &mut TerminalBlock, spec: &TerminalSpec) {
        block.spec_id = spec.id.clone();
        block.spec_name = spec.name.clone();
        block.manufacturer = spec.manufacturer.clone();
        block.width_tenths_mm = spec.width_tenths_mm;
        block.numbering_order = spec.numbering_order;
        block.tiers = spec
            .tiers
            .iter()
            .enumerate()
            .map(|(idx, ts)| ConnectionPoint::new(idx, ts.behavior.clone(), ts.color))
            .collect();
    }
}
