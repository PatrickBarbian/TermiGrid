use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::catalog::TerminalCatalog;
use crate::import::pdf_parser::RawBomRow;
use crate::import::rules::{AccessoryKind, BomRowDisposition, CompiledImportRules};
use crate::model::TerminalStrip;

/// Summary of an article skipped because its manufacturer is outside WAGO / Phoenix Contact / Weidmüller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedForeignItem {
    pub bmk: String,
    pub manufacturer: String,
    pub quantity: usize,
    pub article_number: String,
    pub description: String,
}

/// Summary of an ignored non-terminal BOM row (e.g., `-12T1`, `-Q2.18`, `-W1`, `PLC-RSC`, `UNO2-PS`, `Han 16A`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IgnoredNonTerminalRow {
    pub bmk: String,
    pub quantity: usize,
    pub article_number: String,
}

/// Recognized terminal article entry within a single BOM strip.
#[derive(Debug, Clone)]
pub struct ImportedTerminalEntry {
    pub spec_id: String,
    pub article_number: String,
    pub display_name: String,
    pub manufacturer: String,
    pub quantity: usize,
    pub levels: usize,
    pub height_tenths_mm: u16,
    pub width_tenths_mm: u16,
    pub is_nt_or_install: bool,
    pub is_nt_disconnect: bool,
    pub is_pure_neutral: bool,
    pub is_pure_pe: bool,
}

/// Unknown terminal article detected via heuristic fallback (not in internal database).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownArticleWarning {
    pub manufacturer: String,
    pub quantity: usize,
    pub article_number: String,
    pub type_number: String,
    pub description: String,
    pub heuristic_matched: bool,
    pub suggested_spec_id: Option<String>,
}

/// Physical end clamps / end plates extracted directly from the PDF BOM.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfEndPartEntry {
    pub article_number: String,
    pub quantity: usize,
    pub matched_bracket_id: Option<String>,
}

/// Resolved mounting accessories plan for a strip (either from PDF or auto-complemented).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndPartsPlan {
    pub end_bracket_id: String,
    pub bracket_left: bool,
    pub bracket_right: bool,
    pub has_pdf_end_clamps: bool,
    pub has_pdf_end_plates: bool,
    pub pdf_end_clamp_count: usize,
    pub pdf_end_plate_count: usize,
    pub auto_complement_brackets: bool,
}

/// One deduplicated physical ordering variant for an imported strip.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StripImportVariant {
    pub label: String,
    pub description: String,
    /// Exact ordered sequence of `spec_id`s (invariant: multiset equals PDF terminal counts).
    pub block_spec_ids: Vec<String>,
    /// Contiguous `[start..end)` range of NT/install blocks inside `block_spec_ids`, if any.
    pub nt_zone: Option<(usize, usize)>,
}

/// Complete import plan for one terminal strip merged across all PDF rows/pages.
#[derive(Debug, Clone)]
pub struct StripImportPlan {
    pub bmk: String,
    pub dominant_manufacturer: String,
    /// All distinct manufacturers present on this strip (supports mixed-manufacturer rails).
    pub manufacturers: Vec<String>,
    pub pages: Vec<u32>,
    pub terminals: Vec<ImportedTerminalEntry>,
    pub total_terminal_count: usize,
    pub unknown_articles: Vec<UnknownArticleWarning>,
    pub end_clamps_from_pdf: Vec<PdfEndPartEntry>,
    pub end_plates_from_pdf: Vec<PdfEndPartEntry>,
    pub ignored_jumpers: Vec<String>,
    pub ignored_misc: Vec<String>,
    pub nt_hints: Vec<String>,
    pub has_nt_or_install: bool,
    pub nt_anchor_spec_id: Option<String>,
    pub end_parts: EndPartsPlan,
    pub variants: Vec<StripImportVariant>,
}

/// Complete result of parsing & planning an EPLAN/CAD PDF BOM.
#[derive(Debug, Clone)]
pub struct PdfBomImportPlan {
    pub strips: Vec<StripImportPlan>,
    pub skipped_foreign: Vec<SkippedForeignItem>,
    pub ignored_non_x_rows: Vec<IgnoredNonTerminalRow>,
    pub total_raw_rows: usize,
}

#[derive(Default)]
struct StripAccumulator {
    bmk: String,
    pages: Vec<u32>,
    terminal_order: Vec<String>,
    terminal_counts: HashMap<String, ImportedTerminalEntry>,
    mfg_counts: HashMap<String, usize>,
    unknown_articles: Vec<UnknownArticleWarning>,
    end_clamps_from_pdf: Vec<PdfEndPartEntry>,
    end_plates_from_pdf: Vec<PdfEndPartEntry>,
    ignored_jumpers: Vec<String>,
    ignored_misc: Vec<String>,
    nt_hints: Vec<String>,
    foreign_items: Vec<SkippedForeignItem>,
    accessory_raw_rows: Vec<IgnoredNonTerminalRow>,
}

fn suggest_fallback_spec_id(
    manufacturer: &str,
    type_number: &str,
    description: &str,
    catalog: &TerminalCatalog,
) -> Option<String> {
    let combined = format!("{type_number} {description}").to_lowercase();
    let candidate = match manufacturer {
        "Phoenix Contact" => {
            if combined.contains("schutzleiter") || combined.contains("-pe") {
                "phoenix-pt-25-pe"
            } else if combined.contains("urtk") || combined.contains("prüftrenn") || combined.contains("mess") {
                "phoenix-pt-6-t"
            } else if combined.contains("trenn") {
                "phoenix-pt-25-mt"
            } else if combined.contains("doppelstock") || combined.contains("udk") {
                "phoenix-pttb-25"
            } else if combined.contains("mehrstock") || combined.contains("3-stock") {
                "phoenix-pt-25-3l"
            } else {
                "phoenix-pt-25"
            }
        }
        "WAGO" => {
            if combined.contains("schutzleiter") || combined.contains("-pe") {
                "wago-2002-1207"
            } else if combined.contains("wandler") || combined.contains("2007") || combined.contains("trenn") {
                "wago-2006-1671"
            } else if combined.contains("doppelstock") {
                "wago-2002-2201"
            } else {
                "wago-2002-1201"
            }
        }
        "Weidmüller" => {
            if combined.contains("schutzleiter") || combined.contains("-pe") {
                "weidmuller-a2c-25-pe"
            } else if combined.contains("doppelstock") {
                "weidmuller-a2t-25"
            } else {
                "weidmuller-a2c-25"
            }
        }
        _ => "phoenix-pt-25",
    };
    catalog.find_by_id(candidate).map(|s| s.id.clone())
}

fn row_display_art(row: &RawBomRow) -> String {
    if !row.type_number.trim().is_empty() {
        row.type_number.trim().to_string()
    } else {
        row.article_number.trim().to_string()
    }
}

/// Groups, classifies, and plans terminal strips from raw BOM rows using content-based classification.
pub fn build_import_plan(
    raw_rows: &[RawBomRow],
    rules: &CompiledImportRules,
    catalog: &TerminalCatalog,
) -> PdfBomImportPlan {
    let mut ignored_non_x_rows = Vec::new();
    let mut skipped_foreign = Vec::new();
    let mut strip_order: Vec<String> = Vec::new();
    let mut accumulators: HashMap<String, StripAccumulator> = HashMap::new();

    for row in raw_rows {
        let bmk_clean = CompiledImportRules::normalize_bmk(&row.bmk);
        if bmk_clean.is_empty() {
            ignored_non_x_rows.push(IgnoredNonTerminalRow {
                bmk: String::new(),
                quantity: row.quantity,
                article_number: row.article_number.clone(),
            });
            continue;
        }

        let disposition = rules.classify_bom_row(
            &row.supplier,
            &row.article_number,
            &row.type_number,
            &row.description,
            catalog,
        );

        if matches!(disposition, BomRowDisposition::NonTerminal) {
            ignored_non_x_rows.push(IgnoredNonTerminalRow {
                bmk: bmk_clean,
                quantity: row.quantity,
                article_number: row.article_number.clone(),
            });
            continue;
        }

        let key = bmk_clean;
        if !accumulators.contains_key(&key) {
            strip_order.push(key.clone());
            accumulators.insert(
                key.clone(),
                StripAccumulator {
                    bmk: key.clone(),
                    ..StripAccumulator::default()
                },
            );
        }
        let acc = accumulators.get_mut(&key).expect("just inserted");
        if !acc.pages.contains(&row.page) {
            acc.pages.push(row.page);
        }

        match disposition {
            BomRowDisposition::NonTerminal => unreachable!(),
            BomRowDisposition::Foreign(mfg_label) => {
                acc.foreign_items.push(SkippedForeignItem {
                    bmk: key,
                    manufacturer: mfg_label,
                    quantity: row.quantity,
                    article_number: row.article_number.clone(),
                    description: row.description.clone(),
                });
            }
            BomRowDisposition::Terminal { manufacturer, spec } => {
                *acc.mfg_counts.entry(manufacturer).or_insert(0) += row.quantity;
                if let Some(existing) = acc.terminal_counts.get_mut(&spec.id) {
                    existing.quantity += row.quantity;
                } else {
                    acc.terminal_order.push(spec.id.clone());
                    acc.terminal_counts.insert(
                        spec.id.clone(),
                        ImportedTerminalEntry {
                            spec_id: spec.id.clone(),
                            article_number: spec
                                .effective_article_number()
                                .unwrap_or(&row.type_number)
                                .to_string(),
                            display_name: spec.name.clone(),
                            manufacturer: spec.manufacturer.clone(),
                            quantity: row.quantity,
                            levels: spec.levels,
                            height_tenths_mm: spec.effective_height_tenths_mm(),
                            width_tenths_mm: spec.width_tenths_mm,
                            is_nt_or_install: spec.is_installation_or_nt_terminal(),
                            is_nt_disconnect: spec.is_nt_disconnect_terminal(),
                            is_pure_neutral: !spec.tiers.is_empty()
                                && spec.tiers.iter().all(|t| t.behavior.is_neutral()),
                            is_pure_pe: !spec.tiers.is_empty()
                                && spec.tiers.iter().all(|t| t.behavior.is_pe()),
                        },
                    );
                }
            }
            BomRowDisposition::HeuristicTerminal { manufacturer } => {
                if manufacturer != "Unbekannt" {
                    *acc.mfg_counts.entry(manufacturer.clone()).or_insert(0) += row.quantity;
                }
                let suggested_spec_id = suggest_fallback_spec_id(
                    &manufacturer,
                    &row.type_number,
                    &row.description,
                    catalog,
                );
                acc.unknown_articles.push(UnknownArticleWarning {
                    manufacturer,
                    quantity: row.quantity,
                    article_number: row.article_number.clone(),
                    type_number: row.type_number.clone(),
                    description: row.description.clone(),
                    heuristic_matched: true,
                    suggested_spec_id,
                });
            }
            BomRowDisposition::Accessory { manufacturer, kind } => {
                if manufacturer != "Unbekannt" {
                    *acc.mfg_counts.entry(manufacturer).or_insert(0) += row.quantity;
                }
                acc.accessory_raw_rows.push(IgnoredNonTerminalRow {
                    bmk: key,
                    quantity: row.quantity,
                    article_number: row.article_number.clone(),
                });
                let art = row_display_art(row);
                match kind {
                    AccessoryKind::Jumper => {
                        acc.ignored_jumpers.push(format!("{}× {}", row.quantity, art));
                    }
                    AccessoryKind::EndClamp { matched_bracket_id } => {
                        acc.end_clamps_from_pdf.push(PdfEndPartEntry {
                            article_number: art,
                            quantity: row.quantity,
                            matched_bracket_id,
                        });
                    }
                    AccessoryKind::EndPlate => {
                        acc.end_plates_from_pdf.push(PdfEndPartEntry {
                            article_number: art,
                            quantity: row.quantity,
                            matched_bracket_id: None,
                        });
                    }
                    AccessoryKind::NtAccessory => {
                        acc.nt_hints
                            .push(format!("{}× {} ({})", row.quantity, art, row.description));
                    }
                    AccessoryKind::MiscAccessory => {
                        acc.ignored_misc.push(format!("{}× {}", row.quantity, art));
                    }
                }
            }
        }
    }

    let mut strips = Vec::new();
    for key in strip_order {
        let Some(acc) = accumulators.remove(&key) else {
            continue;
        };

        // A valid terminal strip must contain at least one DB-matched terminal or heuristic unknown terminal.
        let has_terminals =
            !acc.terminal_order.is_empty() || !acc.unknown_articles.is_empty();

        if !has_terminals {
            skipped_foreign.extend(acc.foreign_items);
            ignored_non_x_rows.extend(acc.accessory_raw_rows);
            continue;
        }
        // Any stray foreign items on an otherwise supported strip still go to the skipped log
        skipped_foreign.extend(acc.foreign_items);

        let mut mfg_entries: Vec<(String, usize)> = acc.mfg_counts.into_iter().collect();
        mfg_entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        let dominant_manufacturer = mfg_entries
            .first()
            .map(|(m, _)| m.clone())
            .or_else(|| {
                acc.unknown_articles
                    .first()
                    .map(|u| u.manufacturer.clone())
            })
            .unwrap_or_else(|| "WAGO".to_string());

        let mut manufacturers: Vec<String> = Vec::new();
        for id in &acc.terminal_order {
            if let Some(t) = acc.terminal_counts.get(id)
                && !manufacturers.contains(&t.manufacturer)
            {
                manufacturers.push(t.manufacturer.clone());
            }
        }
        for u in &acc.unknown_articles {
            if u.manufacturer != "Unbekannt" && !manufacturers.contains(&u.manufacturer) {
                manufacturers.push(u.manufacturer.clone());
            }
        }
        if manufacturers.is_empty() {
            manufacturers.push(dominant_manufacturer.clone());
        }

        let terminals: Vec<ImportedTerminalEntry> = acc
            .terminal_order
            .iter()
            .filter_map(|id| acc.terminal_counts.get(id).cloned())
            .collect();
        let total_terminal_count: usize = terminals.iter().map(|t| t.quantity).sum();

        let nt_anchor_spec_id = terminals
            .iter()
            .find(|t| t.is_nt_disconnect)
            .map(|t| t.spec_id.clone());
        let has_nt_or_install = nt_anchor_spec_id.is_some();

        let pdf_end_clamp_count: usize = acc.end_clamps_from_pdf.iter().map(|e| e.quantity).sum();
        let pdf_end_plate_count: usize = acc.end_plates_from_pdf.iter().map(|e| e.quantity).sum();
        let has_pdf_end_clamps = pdf_end_clamp_count > 0;
        let has_pdf_end_plates = pdf_end_plate_count > 0;
        let end_bracket_id = acc
            .end_clamps_from_pdf
            .iter()
            .find_map(|e| e.matched_bracket_id.clone())
            .unwrap_or_else(|| {
                rules
                    .default_end_bracket_for(&dominant_manufacturer)
                    .to_string()
            });

        let end_parts = EndPartsPlan {
            end_bracket_id,
            bracket_left: true,
            bracket_right: if has_pdf_end_clamps {
                pdf_end_clamp_count >= 2
            } else {
                true
            },
            has_pdf_end_clamps,
            has_pdf_end_plates,
            pdf_end_clamp_count,
            pdf_end_plate_count,
            auto_complement_brackets: !has_pdf_end_clamps
                && (total_terminal_count > 0 || !acc.unknown_articles.is_empty()),
        };

        let variants = generate_variants(&terminals);

        strips.push(StripImportPlan {
            bmk: acc.bmk,
            dominant_manufacturer,
            manufacturers,
            pages: acc.pages,
            terminals,
            total_terminal_count,
            unknown_articles: acc.unknown_articles,
            end_clamps_from_pdf: acc.end_clamps_from_pdf,
            end_plates_from_pdf: acc.end_plates_from_pdf,
            ignored_jumpers: acc.ignored_jumpers,
            ignored_misc: acc.ignored_misc,
            nt_hints: acc.nt_hints,
            has_nt_or_install,
            nt_anchor_spec_id,
            end_parts,
            variants,
        });
    }

    PdfBomImportPlan {
        strips,
        skipped_foreign,
        ignored_non_x_rows,
        total_raw_rows: raw_rows.len(),
    }
}

/// Generates up to 3 deduplicated block-arrangement variants while strictly preserving
/// exact PDF quantities AND keeping all NT/install terminals as one closed contiguous block.
fn generate_variants(terminals: &[ImportedTerminalEntry]) -> Vec<StripImportVariant> {
    if terminals.is_empty() {
        return Vec::new();
    }

    let expected_counts: BTreeMap<&str, usize> = terminals
        .iter()
        .map(|t| (t.spec_id.as_str(), t.quantity))
        .collect();

    // Homogeneous strip (single terminal type) -> single Direct variant, no radio choice in UI
    if terminals.len() == 1 {
        let t = &terminals[0];
        let block_spec_ids = vec![t.spec_id.clone(); t.quantity];
        let nt_zone = t.is_nt_or_install.then_some((0, t.quantity));
        return vec![StripImportVariant {
            label: "Direkt (homogene Leiste)".to_string(),
            description: format!("{}× {}", t.quantity, t.display_name),
            block_spec_ids,
            nt_zone,
        }];
    }

    // Partition into ONE closed contiguous NT/install block vs residual entries.
    let nt_entries: Vec<&ImportedTerminalEntry> =
        terminals.iter().filter(|t| t.is_nt_or_install).collect();
    let rest_entries: Vec<&ImportedTerminalEntry> =
        terminals.iter().filter(|t| !t.is_nt_or_install).collect();

    let mut nt_block: Vec<String> = Vec::new();
    for e in &nt_entries {
        nt_block.extend(std::iter::repeat_n(e.spec_id.clone(), e.quantity));
    }
    let nt_zone = (!nt_block.is_empty()).then_some((0, nt_block.len()));

    let mut candidates: Vec<StripImportVariant> = Vec::new();

    // Strategy 1: 3-phase feeders (3× L + 1× N + 1× PE)
    if let Some(rest_seq) = build_feeder_pattern(&rest_entries, 3) {
        let mut seq = nt_block.clone();
        seq.extend(rest_seq);
        let nt_note = if nt_block.is_empty() {
            ""
        } else {
            ", NT-Block links"
        };
        candidates.push(StripImportVariant {
            label: format!("Drehstrom-Abgänge (3×L + N + PE{nt_note})"),
            description:
                "Gruppiert je 3 Außenleiter mit zugehörigem N & PE (NT-Block geschlossen links)"
                    .to_string(),
            block_spec_ids: seq,
            nt_zone,
        });
    }

    // Strategy 2: Multi-level / height descending (with closed NT block first)
    {
        let mut sorted_rest = rest_entries.clone();
        sorted_rest.sort_by_key(|e| {
            (
                Reverse(e.levels),
                Reverse(e.height_tenths_mm),
                Reverse(e.width_tenths_mm),
            )
        });
        let mut seq = nt_block.clone();
        for e in &sorted_rest {
            seq.extend(std::iter::repeat_n(e.spec_id.clone(), e.quantity));
        }
        candidates.push(StripImportVariant {
            label: "Nach Etagen / Bauhöhe absteigend".to_string(),
            description:
                "Größte Bauhöhe / meiste Etagen zuerst, abfallend bis Einzelklemme (NT-Block links)"
                    .to_string(),
            block_spec_ids: seq,
            nt_zone,
        });
    }

    // Strategy 3: 1-phase feeders (1× L + 1× N + 1× PE)
    if let Some(rest_seq) = build_feeder_pattern(&rest_entries, 1) {
        let mut seq = nt_block.clone();
        seq.extend(rest_seq);
        let nt_note = if nt_block.is_empty() {
            ""
        } else {
            ", NT-Block links"
        };
        candidates.push(StripImportVariant {
            label: format!("Wechselstrom-Abgänge (1×L + N + PE{nt_note})"),
            description:
                "Gruppiert 1× L mit N & PE je Abgang (NT-Block geschlossen links)".to_string(),
            block_spec_ids: seq,
            nt_zone,
        });
    }

    // Strategy 4: Blockwise by PDF type order
    {
        let mut seq = nt_block.clone();
        for e in &rest_entries {
            seq.extend(std::iter::repeat_n(e.spec_id.clone(), e.quantity));
        }
        candidates.push(StripImportVariant {
            label: "Blockweise nach Klemmentyp".to_string(),
            description: "Geschlossene Typ-Blöcke in Stücklisten-Reihenfolge (NT-Block links)"
                .to_string(),
            block_spec_ids: seq,
            nt_zone,
        });
    }

    // Verify exact quantity invariant and deduplicate identical sequences (max 3 variants)
    let mut seen: HashSet<Vec<String>> = HashSet::new();
    let mut unique = Vec::new();
    for cand in candidates {
        let mut actual: BTreeMap<&str, usize> = BTreeMap::new();
        for id in &cand.block_spec_ids {
            *actual.entry(id.as_str()).or_insert(0) += 1;
        }
        if actual != expected_counts {
            continue;
        }
        if seen.insert(cand.block_spec_ids.clone()) {
            unique.push(cand);
            if unique.len() >= 3 {
                break;
            }
        }
    }

    unique
}

fn build_feeder_pattern(
    rest_entries: &[&ImportedTerminalEntry],
    l_per_circuit: usize,
) -> Option<Vec<String>> {
    let mut l_queue: Vec<(String, usize)> = Vec::new();
    let mut n_queue: Vec<(String, usize)> = Vec::new();
    let mut pe_queue: Vec<(String, usize)> = Vec::new();
    let mut other_queue: Vec<(String, usize)> = Vec::new();

    for e in rest_entries {
        if e.is_pure_pe {
            pe_queue.push((e.spec_id.clone(), e.quantity));
        } else if e.is_pure_neutral {
            n_queue.push((e.spec_id.clone(), e.quantity));
        } else if e.levels == 1 {
            l_queue.push((e.spec_id.clone(), e.quantity));
        } else {
            other_queue.push((e.spec_id.clone(), e.quantity));
        }
    }

    let total_l: usize = l_queue.iter().map(|(_, q)| *q).sum();
    let total_n: usize = n_queue.iter().map(|(_, q)| *q).sum();
    let total_pe: usize = pe_queue.iter().map(|(_, q)| *q).sum();

    if total_l < l_per_circuit || (total_n == 0 && total_pe == 0) {
        return None;
    }

    let pop_one = |q: &mut Vec<(String, usize)>| -> Option<String> {
        for item in q.iter_mut() {
            if item.1 > 0 {
                item.1 -= 1;
                return Some(item.0.clone());
            }
        }
        None
    };

    let mut out = Vec::with_capacity(total_l + total_n + total_pe);
    for (id, qty) in other_queue {
        out.extend(std::iter::repeat_n(id, qty));
    }

    let full_cycles = (total_l / l_per_circuit)
        .min(if total_n > 0 { total_n } else { usize::MAX })
        .min(if total_pe > 0 { total_pe } else { usize::MAX });

    if full_cycles == 0 {
        return None;
    }

    for _ in 0..full_cycles {
        for _ in 0..l_per_circuit {
            if let Some(id) = pop_one(&mut l_queue) {
                out.push(id);
            }
        }
        if let Some(id) = pop_one(&mut n_queue) {
            out.push(id);
        }
        if let Some(id) = pop_one(&mut pe_queue) {
            out.push(id);
        }
    }

    // Drain any remaining L/N/PE blocks cleanly so quantity invariant holds 100%
    while let Some(id) = pop_one(&mut l_queue) {
        out.push(id);
    }
    while let Some(id) = pop_one(&mut n_queue) {
        out.push(id);
    }
    while let Some(id) = pop_one(&mut pe_queue) {
        out.push(id);
    }

    Some(out)
}

/// Builds a ready-to-add `TerminalStrip` from a `StripImportPlan` and user's selected variant/end-clamp option.
pub fn instantiate_imported_strip(
    plan: &StripImportPlan,
    variant_index: usize,
    include_end_clamps: bool,
    catalog: &TerminalCatalog,
) -> Option<(TerminalStrip, Option<(usize, usize)>)> {
    instantiate_imported_strip_with_mappings(plan, variant_index, include_end_clamps, &[], catalog)
}

/// Builds a ready-to-add `TerminalStrip` including user-mapped heuristic/unknown terminal articles.
pub fn instantiate_imported_strip_with_mappings(
    plan: &StripImportPlan,
    variant_index: usize,
    include_end_clamps: bool,
    unknown_mappings: &[Option<String>],
    catalog: &TerminalCatalog,
) -> Option<(TerminalStrip, Option<(usize, usize)>)> {
    let mut all_spec_ids: Vec<String> = Vec::new();
    let mut nt_zone: Option<(usize, usize)> = None;

    if let Some(variant) = plan
        .variants
        .get(variant_index)
        .or_else(|| plan.variants.first())
    {
        all_spec_ids.extend(variant.block_spec_ids.iter().cloned());
        nt_zone = variant.nt_zone;
    }

    for (unk, mapped_opt) in plan.unknown_articles.iter().zip(unknown_mappings.iter()) {
        if let Some(spec_id) = mapped_opt
            && catalog.find_by_id(spec_id).is_some()
        {
            all_spec_ids.extend(std::iter::repeat_n(spec_id.clone(), unk.quantity));
        }
    }

    if all_spec_ids.is_empty() {
        return None;
    }

    let default_spec_id = all_spec_ids
        .first()
        .cloned()
        .unwrap_or_else(|| "wago-2002-1201".to_string());

    let mut strip = TerminalStrip::new(
        plan.bmk.clone(),
        1,
        (all_spec_ids.len().max(1)) as u32,
        default_spec_id,
    );
    let mfg_summary = if plan.manufacturers.len() > 1 {
        plan.manufacturers.join(" + ")
    } else {
        plan.dominant_manufacturer.clone()
    };
    strip.description = format!("Importiert aus PDF ({mfg_summary})");
    if include_end_clamps {
        strip.end_bracket_id = plan.end_parts.end_bracket_id.clone();
        strip.end_bracket_left = plan.end_parts.bracket_left;
        strip.end_bracket_right = plan.end_parts.bracket_right;
    } else {
        strip.end_bracket_id = plan.end_parts.end_bracket_id.clone();
        strip.end_bracket_left = false;
        strip.end_bracket_right = false;
    }

    for id in &all_spec_ids {
        if let Some(spec) = catalog.find_by_id(id) {
            let blk_id = strip.allocate_block_id();
            strip.blocks.push(spec.instantiate_block(blk_id));
        }
    }
    strip.recalculate_labels();
    Some((strip, nt_zone))
}
