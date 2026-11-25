use std::collections::BTreeMap;
use std::path::Path;

use crate::catalog::accessories::{
    default_end_bracket_for_manufacturer, dominant_strip_manufacturer,
};
use crate::catalog::{TerminalCatalog, TerminalSpec, TierSpec};
use crate::eplan::config::{normalize_article_key, EplanConfig};
use crate::import::rules::{
    find_matching_end_bracket, BomRowDisposition, CompiledImportRules, ManufacturerDetection,
};
use crate::model::{NumberingOrder, Project, TerminalStrip, TierBehavior, TierColor};

/// Summary of an EPLAN CSV import operation.
#[derive(Debug, Clone, Default)]
pub struct EplanCsvImportReport {
    pub strips: Vec<TerminalStrip>,
    pub custom_specs: Vec<TerminalSpec>,
    pub total_blocks: usize,
    pub total_preexisting_accessories: usize,
    pub unknown_articles: Vec<String>,
}

#[derive(Debug, Clone)]
struct ColumnIndices {
    bmk: usize,
    terminal_no: Option<usize>,
    level: Option<usize>,
    article_no: Option<usize>,
    type_no: Option<usize>,
    qty: Option<usize>,
    description: Option<usize>,
    supplier: Option<usize>,
}

struct PendingMultiBlock {
    spec: TerminalSpec,
    filled_tiers: Vec<bool>,
    tier_labels: Vec<Option<String>>,
}

struct StripBuilder {
    strip: TerminalStrip,
    pending: Option<PendingMultiBlock>,
    saw_end_bracket: bool,
}

impl StripBuilder {
    fn new(bmk: String) -> Self {
        let mut strip = TerminalStrip::new(bmk, 1, 1, "wago-2002-1201");
        strip.end_plates_generated = true;
        Self {
            strip,
            pending: None,
            saw_end_bracket: false,
        }
    }

    fn flush_pending(&mut self) {
        if let Some(pending) = self.pending.take() {
            let block_id = self.strip.allocate_block_id();
            let mut block = pending.spec.instantiate_block(block_id);
            for (idx, maybe_lbl) in pending.tier_labels.into_iter().enumerate() {
                if let (Some(lbl), Some(tier)) = (maybe_lbl, block.tiers.get_mut(idx)) {
                    tier.custom_override = Some(lbl);
                }
            }
            self.strip.blocks.push(block);
        }
    }

    fn finalize(mut self) -> TerminalStrip {
        self.flush_pending();
        if !self.saw_end_bracket
            && let Some(mfg) = dominant_strip_manufacturer(&self.strip.blocks)
        {
            let eb = default_end_bracket_for_manufacturer(&mfg, &self.strip.blocks);
            self.strip.end_bracket_id = eb.id.to_string();
        }
        if let Some(first) = self.strip.blocks.first() {
            self.strip.default_spec_id = first.spec_id.clone();
        }
        self.strip.sync_next_block_id();
        self.strip.recalculate_labels();
        self.strip.end_plates_generated = true;
        self.strip
    }
}

/// Imports an EPLAN P8 terminal CSV file into a new `Project` ready for the CAD editor.
pub fn import_eplan_csv_to_project(
    path: &Path,
    catalog: &TerminalCatalog,
    config: &EplanConfig,
) -> Result<(Project, EplanCsvImportReport), String> {
    let report = import_eplan_csv_path(path, catalog, config)?;
    let proj_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("EPLAN-Import")
        .to_string();

    let mut project = Project {
        format_version: 1,
        project_name: proj_name,
        strips: report.strips.clone(),
        custom_specs: report.custom_specs.clone(),
    };
    project.normalize();
    Ok((project, report))
}

/// Parses an EPLAN P8 terminal CSV file on disk using `config` and matches rows against `catalog`.
pub fn import_eplan_csv_path(
    path: &Path,
    catalog: &TerminalCatalog,
    config: &EplanConfig,
) -> Result<EplanCsvImportReport, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("EPLAN CSV konnte nicht gelesen werden ({}): {e}", path.display()))?;
    let raw = decode_csv_text(&bytes);
    parse_eplan_csv_str(&raw, catalog, config)
}

/// Parses EPLAN P8 terminal CSV text (`;` separated by default, UTF-8 with optional BOM).
pub fn parse_eplan_csv_str(
    raw: &str,
    catalog: &TerminalCatalog,
    config: &EplanConfig,
) -> Result<EplanCsvImportReport, String> {
    let cleaned = raw.trim_start_matches('\u{feff}');
    let delimiter = detect_effective_delimiter(cleaned, config.csv_delimiter);
    let rows = parse_rfc4180_csv(cleaned, delimiter);

    let (header_idx, cols) = find_header_row(&rows).ok_or_else(|| {
        "EPLAN CSV enthält keine gültige Kopfzeile (erwartet z.B. BMK;Klemmennummer;Etage;Artikelnummer;Typnummer;Menge)."
            .to_string()
    })?;

    let rules = CompiledImportRules::load_default();
    let mut strip_order: Vec<String> = Vec::new();
    let mut strip_map: BTreeMap<String, StripBuilder> = BTreeMap::new();
    let mut custom_specs: Vec<TerminalSpec> = Vec::new();
    let mut unknown_articles: Vec<String> = Vec::new();
    let mut total_preexisting_accessories = 0usize;

    for row in rows.iter().skip(header_idx + 1) {
        if row.iter().all(|c| c.trim().is_empty()) {
            continue;
        }
        let raw_bmk = cell(row, Some(cols.bmk));
        let bmk = CompiledImportRules::normalize_bmk(raw_bmk);
        if bmk.is_empty() {
            continue;
        }

        let terminal_no = cell(row, cols.terminal_no).trim().to_string();
        let level_raw = cell(row, cols.level).trim();
        let raw_article = cell(row, cols.article_no).trim();
        let raw_type = cell(row, cols.type_no).trim();
        let qty_raw = cell(row, cols.qty).trim();
        let description = cell(row, cols.description).trim();
        let supplier = cell(row, cols.supplier).trim();

        let level_opt = parse_level_number(level_raw);
        let parsed_qty = parse_qty(qty_raw);

        // Strip configured EPLAN vendor prefixes (PXC., WAG., WDM., ...)
        let (mfg_from_art_prefix, clean_art) = config.strip_vendor_prefix(raw_article);
        let (mfg_from_type_prefix, clean_type) = config.strip_vendor_prefix(raw_type);

        let mfg_det = if let Some(m) = mfg_from_art_prefix.or(mfg_from_type_prefix) {
            ManufacturerDetection::Supported(m.to_string())
        } else {
            rules.detect_manufacturer(supplier, raw_article, &clean_type, catalog)
        };
        let mfg_hint = match &mfg_det {
            ManufacturerDetection::Supported(m) => Some(m.as_str()),
            _ => None,
        };

        let builder = strip_map.entry(bmk.clone()).or_insert_with(|| {
            strip_order.push(bmk.clone());
            StripBuilder::new(bmk.clone())
        });

        // 1. Pre-existing End Bracket in CSV?
        if let Some(eb) = find_matching_end_bracket(&clean_art, &clean_type, mfg_hint) {
            builder.flush_pending();
            let qty = parsed_qty.unwrap_or(1).max(1);
            let key = normalize_article_key(eb.article_number);
            if !key.is_empty() {
                *builder
                    .strip
                    .eplan_preexisting_accessories
                    .entry(key)
                    .or_insert(0) += qty;
                total_preexisting_accessories += qty;
            }
            builder.strip.end_bracket_id = eb.id.to_string();
            builder.saw_end_bracket = true;
            continue;
        }

        // 2. Match terminal block against active catalog (and already created custom specs)
        let matched_spec = catalog
            .find_by_article_or_type(mfg_hint, &clean_art, &clean_type)
            .cloned()
            .or_else(|| {
                // Retry without manufacturer hint if prefix/supplier differed slightly
                catalog
                    .find_by_article_or_type(None, &clean_art, &clean_type)
                    .cloned()
            })
            .or_else(|| find_synthetic_spec(&custom_specs, &clean_art, &clean_type));

        let spec = if let Some(sp) = matched_spec {
            if sp.is_busbar_carrier() && terminal_no.is_empty() {
                builder.flush_pending();
                let qty = parsed_qty.unwrap_or(1).max(1);
                let art_key = sp
                    .effective_article_number()
                    .map(normalize_article_key)
                    .filter(|k| !k.is_empty())
                    .unwrap_or_else(|| normalize_article_key(&clean_art));
                if !art_key.is_empty() {
                    *builder
                        .strip
                        .eplan_preexisting_accessories
                        .entry(art_key)
                        .or_insert(0) += qty;
                    total_preexisting_accessories += qty;
                }
                continue;
            }
            sp
        } else {
            // Check if this row is an EPLAN accessory (end plate, jumper, etc.)
            let disp = rules.classify_bom_row(
                supplier,
                raw_article,
                &clean_type,
                description,
                catalog,
            );
            if matches!(disp, BomRowDisposition::Accessory { .. }) {
                builder.flush_pending();
                let qty = parsed_qty.unwrap_or(1).max(1);
                let art_key = normalize_article_key(if !clean_art.is_empty() {
                    &clean_art
                } else {
                    &clean_type
                });
                if !art_key.is_empty() {
                    *builder
                        .strip
                        .eplan_preexisting_accessories
                        .entry(art_key)
                        .or_insert(0) += qty;
                    total_preexisting_accessories += qty;
                }
                continue;
            }

            // Unknown terminal: preserve order & numbering via synthetic spec and flag warning
            let label = if !raw_article.is_empty() {
                raw_article.to_string()
            } else {
                raw_type.to_string()
            };
            if !label.is_empty() && !unknown_articles.contains(&label) {
                unknown_articles.push(label);
            }
            let synth = build_synthetic_spec(
                mfg_hint.unwrap_or("Unbekannt"),
                &clean_art,
                &clean_type,
                description,
                level_opt.unwrap_or(1),
            );
            if !custom_specs.iter().any(|s| s.id == synth.id) {
                custom_specs.push(synth.clone());
            }
            synth
        };

        // 3. Place terminal block(s) while preserving exact EPLAN order, level & numbering
        let qty_eff = parsed_qty.unwrap_or(1);
        let can_merge_level =
            spec.levels > 1 && level_opt.is_some() && qty_eff <= 1;

        if can_merge_level {
            let etage = level_opt.unwrap_or(1);
            let tier_idx = resolve_tier_index(&spec, etage, level_raw);

            let can_append_to_pending = builder.pending.as_ref().is_some_and(|p| {
                p.spec.id == spec.id && !p.filled_tiers.get(tier_idx).copied().unwrap_or(true)
            });

            if !can_append_to_pending {
                builder.flush_pending();
                builder.pending = Some(PendingMultiBlock {
                    filled_tiers: vec![false; spec.levels],
                    tier_labels: vec![None; spec.levels],
                    spec,
                });
            }

            if let Some(p) = builder.pending.as_mut() {
                p.filled_tiers[tier_idx] = true;
                p.tier_labels[tier_idx] = Some(terminal_no);
            }
        } else {
            builder.flush_pending();
            let count = qty_eff.max(1);
            for copy_idx in 0..count {
                let block_id = builder.strip.allocate_block_id();
                let mut block = spec.instantiate_block(block_id);
                apply_single_row_terminal_label(
                    &mut block,
                    &terminal_no,
                    copy_idx,
                    count,
                    level_opt,
                    level_raw,
                );
                builder.strip.blocks.push(block);
            }
        }
    }

    let mut strips = Vec::with_capacity(strip_order.len());
    let mut total_blocks = 0usize;
    for bmk in strip_order {
        if let Some(builder) = strip_map.remove(&bmk) {
            let strip = builder.finalize();
            total_blocks += strip.blocks.len();
            strips.push(strip);
        }
    }

    if strips.is_empty() {
        return Err(
            "In der EPLAN-CSV-Datei wurden keine verwertbaren Klemmenleisten gefunden."
                .to_string(),
        );
    }

    Ok(EplanCsvImportReport {
        strips,
        custom_specs,
        total_blocks,
        total_preexisting_accessories,
        unknown_articles,
    })
}

fn decode_csv_text(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(s) => s.to_string(),
        Err(_) => {
            // Best-effort Windows-1252 / Latin-1 fallback for legacy German EPLAN exports
            bytes.iter().map(|&b| b as char).collect()
        }
    }
}

fn detect_effective_delimiter(text: &str, preferred: char) -> char {
    let first_line = text
        .lines()
        .find(|l| !l.trim().is_empty())
        .unwrap_or_default();
    let pref_cols = split_csv_line(first_line, preferred).len();
    if pref_cols >= 2 {
        return preferred;
    }
    for cand in [';', ',', '\t'] {
        if cand != preferred && split_csv_line(first_line, cand).len() >= 2 {
            return cand;
        }
    }
    preferred
}

fn parse_rfc4180_csv(text: &str, delimiter: char) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut current_row = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    field.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                field.push(ch);
            }
        } else if ch == '"' && field.is_empty() {
            in_quotes = true;
        } else if ch == delimiter {
            current_row.push(std::mem::take(&mut field));
        } else if ch == '\r' {
            if chars.peek() == Some(&'\n') {
                chars.next();
            }
            current_row.push(std::mem::take(&mut field));
            rows.push(std::mem::take(&mut current_row));
        } else if ch == '\n' {
            current_row.push(std::mem::take(&mut field));
            rows.push(std::mem::take(&mut current_row));
        } else {
            field.push(ch);
        }
    }

    if !field.is_empty() || !current_row.is_empty() {
        current_row.push(field);
        rows.push(current_row);
    }
    rows
}

fn split_csv_line(line: &str, delimiter: char) -> Vec<String> {
    parse_rfc4180_csv(line, delimiter)
        .into_iter()
        .next()
        .unwrap_or_default()
}

fn norm_header(s: &str) -> String {
    s.trim()
        .trim_start_matches('\u{feff}')
        .to_lowercase()
        .replace('ä', "ae")
        .replace('ö', "oe")
        .replace('ü', "ue")
        .replace('ß', "ss")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

fn find_header_row(rows: &[Vec<String>]) -> Option<(usize, ColumnIndices)> {
    for (idx, row) in rows.iter().take(12).enumerate() {
        if row.len() < 2 {
            continue;
        }
        let mut bmk = None;
        let mut terminal_no = None;
        let mut level = None;
        let mut article_no = None;
        let mut type_no = None;
        let mut qty = None;
        let mut description = None;
        let mut supplier = None;

        for (c_idx, raw_hdr) in row.iter().enumerate() {
            let h = norm_header(raw_hdr);
            if h.is_empty() {
                continue;
            }
            if bmk.is_none()
                && matches!(
                    h.as_str(),
                    "bmk"
                        | "klemmleiste"
                        | "klemmleistenbmk"
                        | "betriebsmittelkennzeichen"
                        | "devicetag"
                        | "dt"
                        | "strip"
                        | "kennzeichen"
                        | "terminalstrip"
                )
            {
                bmk = Some(c_idx);
            } else if terminal_no.is_none()
                && matches!(
                    h.as_str(),
                    "klemmennummer"
                        | "klemmennr"
                        | "klemmnr"
                        | "klemmenbezeichnung"
                        | "terminalno"
                        | "terminalnumber"
                        | "terminaldesignation"
                        | "nummer"
                        | "nr"
                        | "klemme"
                )
            {
                terminal_no = Some(c_idx);
            } else if level.is_none()
                && matches!(
                    h.as_str(),
                    "etage" | "ebene" | "level" | "tier" | "etagennummer" | "etagennr"
                )
            {
                level = Some(c_idx);
            } else if article_no.is_none()
                && matches!(
                    h.as_str(),
                    "artikelnummer"
                        | "artikelnr"
                        | "articleno"
                        | "articlenumber"
                        | "partnumber"
                        | "partno"
                        | "ordernumber"
                        | "orderno"
                        | "artikel"
                )
            {
                article_no = Some(c_idx);
            } else if type_no.is_none()
                && matches!(
                    h.as_str(),
                    "typnummer"
                        | "typnr"
                        | "typenbezeichnung"
                        | "typennummer"
                        | "typeno"
                        | "typenumber"
                        | "type"
                        | "typ"
                        | "bezeichnung1"
                        | "bestellnummer"
                )
            {
                type_no = Some(c_idx);
            } else if qty.is_none()
                && matches!(
                    h.as_str(),
                    "menge" | "anzahl" | "stueckzahl" | "stueck" | "qty" | "quantity" | "count"
                )
            {
                qty = Some(c_idx);
            } else if description.is_none()
                && matches!(
                    h.as_str(),
                    "bezeichnung"
                        | "beschreibung"
                        | "artikelbezeichnung"
                        | "description"
                        | "funktionstext"
                        | "bezeichnung2"
                )
            {
                description = Some(c_idx);
            } else if supplier.is_none()
                && matches!(
                    h.as_str(),
                    "hersteller" | "lieferant" | "manufacturer" | "supplier" | "vendor"
                )
            {
                supplier = Some(c_idx);
            }
        }

        if let Some(bmk_col) = bmk
            && (article_no.is_some() || type_no.is_some() || terminal_no.is_some())
        {
            return Some((
                idx,
                ColumnIndices {
                    bmk: bmk_col,
                    terminal_no,
                    level,
                    article_no,
                    type_no,
                    qty,
                    description,
                    supplier,
                },
            ));
        }
    }
    None
}

fn cell(row: &[String], col: Option<usize>) -> &str {
    col.and_then(|i| row.get(i)).map(String::as_str).unwrap_or("")
}

fn parse_level_number(raw: &str) -> Option<usize> {
    let t = raw.trim();
    if t.is_empty() {
        return None;
    }
    if t.eq_ignore_ascii_case("PE") {
        return Some(1);
    }
    let digits: String = t
        .trim_start_matches(['E', 'e', 'L', 'l', 'T', 't'])
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse::<usize>().ok().filter(|&n| n >= 1)
}

fn parse_qty(raw: &str) -> Option<usize> {
    let t = raw.trim();
    if t.is_empty() {
        return None;
    }
    if let Ok(n) = t.parse::<usize>() {
        return Some(n);
    }
    let normalized = t.replace(',', ".");
    if let Ok(f) = normalized.parse::<f64>()
        && f >= 0.0
    {
        return Some(f.round() as usize);
    }
    None
}

fn resolve_tier_index(spec: &TerminalSpec, etage_1based: usize, raw_level: &str) -> usize {
    if raw_level.trim().eq_ignore_ascii_case("PE")
        && let Some(pe_idx) = spec.tiers.iter().position(|t| t.behavior == TierBehavior::ProtectiveEarth)
    {
        return pe_idx;
    }
    etage_1based.saturating_sub(1).min(spec.levels.saturating_sub(1))
}

fn apply_single_row_terminal_label(
    block: &mut crate::model::TerminalBlock,
    terminal_no: &str,
    copy_idx: usize,
    total_copies: usize,
    level_opt: Option<usize>,
    raw_level: &str,
) {
    if block.tiers.is_empty() {
        return;
    }

    // If multi-level block received a combined slash designation like "1/2", split across tiers
    if block.tiers.len() > 1 && terminal_no.contains('/') {
        let parts: Vec<&str> = terminal_no.split('/').map(str::trim).collect();
        if parts.len() == block.tiers.len() {
            for (tier, part) in block.tiers.iter_mut().zip(parts) {
                tier.custom_override = Some(part.to_string());
            }
            return;
        }
    }

    if total_copies > 1 && !terminal_no.is_empty() {
        let computed = if let Ok(base_num) = terminal_no.parse::<usize>() {
            (base_num + copy_idx).to_string()
        } else {
            terminal_no.to_string()
        };
        block.tiers[0].custom_override = Some(computed);
        return;
    }

    if let Some(etage) = level_opt
        && block.tiers.len() > 1
    {
        let idx = if raw_level.trim().eq_ignore_ascii_case("PE") {
            block
                .tiers
                .iter()
                .position(|t| t.behavior.is_pe())
                .unwrap_or(0)
        } else {
            etage.saturating_sub(1).min(block.tiers.len() - 1)
        };
        block.tiers[idx].custom_override = Some(terminal_no.to_string());
        return;
    }

    if block.tiers.len() == 1 {
        block.tiers[0].custom_override = Some(terminal_no.to_string());
    } else if !terminal_no.is_empty() {
        if terminal_no.eq_ignore_ascii_case("PE")
            && let Some(pe_tier) = block.tiers.iter_mut().find(|t| t.behavior.is_pe())
        {
            pe_tier.custom_override = Some("PE".to_string());
            return;
        }
        if let Some(seq_tier) = block.tiers.iter_mut().find(|t| t.behavior.consumes_sequence()) {
            seq_tier.custom_override = Some(terminal_no.to_string());
        } else {
            block.tiers[0].custom_override = Some(terminal_no.to_string());
        }
    }
}

fn find_synthetic_spec(
    custom_specs: &[TerminalSpec],
    clean_art: &str,
    clean_type: &str,
) -> Option<TerminalSpec> {
    let art_key = normalize_article_key(clean_art);
    let type_key = normalize_article_key(clean_type);
    custom_specs.iter().find(|s| {
        (!art_key.is_empty()
            && s.article_number
                .as_deref()
                .is_some_and(|a| normalize_article_key(a) == art_key))
            || (!type_key.is_empty() && normalize_article_key(&s.name) == type_key)
    }).cloned()
}

fn build_synthetic_spec(
    manufacturer: &str,
    clean_art: &str,
    clean_type: &str,
    description: &str,
    levels: usize,
) -> TerminalSpec {
    let lvl = levels.clamp(1, 4);
    let display = if !clean_type.is_empty() {
        clean_type.to_string()
    } else if !clean_art.is_empty() {
        clean_art.to_string()
    } else {
        "EPLAN-Klemme".to_string()
    };
    let slug = normalize_article_key(if !clean_art.is_empty() {
        clean_art
    } else {
        &display
    });
    let id = format!(
        "eplan-custom-{}",
        if slug.is_empty() { "unknown" } else { &slug }
    );
    TerminalSpec {
        id,
        name: format!("{display} (EPLAN)"),
        manufacturer: manufacturer.to_string(),
        series: "EPLAN Import".to_string(),
        levels: lvl,
        width_tenths_mm: 52,
        numbering_order: NumberingOrder::BottomToTop,
        description_de: if description.is_empty() {
            format!("Aus EPLAN CSV importierte Klemme ({display})")
        } else {
            description.to_string()
        },
        description_en: if description.is_empty() {
            format!("Imported from EPLAN CSV ({display})")
        } else {
            description.to_string()
        },
        tiers: (0..lvl)
            .map(|_| TierSpec {
                behavior: TierBehavior::Sequential,
                color: TierColor::StandardGray,
            })
            .collect(),
        accessories: None,
        jumper_slots_per_tier: Some(2),
        max_jumper_poles: Some(10),
        article_number: (!clean_art.is_empty()).then(|| clean_art.to_string()),
        product_url: None,
        image_url: None,
        height_tenths_mm: Some(match lvl {
            1 => 485,
            2 => 697,
            _ => 933,
        }),
        rail_offset_tenths_mm: Some(0),
        user_modified: false,
    }
}
