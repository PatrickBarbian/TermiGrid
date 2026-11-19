use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::catalog::TerminalCatalog;
use crate::import::pdf_parser::RawBomRow;
use crate::import::rules::{
    BomItemClassification, CompiledImportRules, ManufacturerDetection,
};
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

/// Summary of an ignored non-`-X...` BOM row (e.g., `-12T1`, `-Q2.18`, `-W1`).
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
    pub is_pure_neutral: bool,
    pub is_pure_pe: bool,
}

/// Article belonging to WAGO / Phoenix Contact / Weidmüller that was not found in the database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownArticleWarning {
    pub manufacturer: String,
    pub quantity: usize,
    pub article_number: String,
    pub type_number: String,
    pub description: String,
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

/// Complete import plan for one `-X...` terminal strip merged across all PDF rows/pages.
#[derive(Debug, Clone)]
pub struct StripImportPlan {
    pub bmk: String,
    pub dominant_manufacturer: String,
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

/// Complete result of parsing & planning an EPLAN PDF BOM.
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
}

/// Groups, classifies, and plans terminal strips from raw EPLAN BOM rows.
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
        let bmk_clean = row.bmk.trim();
        if !rules.is_terminal_bmk(bmk_clean) {
            ignored_non_x_rows.push(IgnoredNonTerminalRow {
                bmk: bmk_clean.to_string(),
                quantity: row.quantity,
                article_number: row.article_number.clone(),
            });
            continue;
        }

        let key = bmk_clean.to_string();
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

        match rules.detect_manufacturer(
            &row.supplier,
            &row.article_number,
            &row.type_number,
            catalog,
        ) {
            ManufacturerDetection::Foreign(mfg_label) => {
                acc.foreign_items.push(SkippedForeignItem {
                    bmk: key,
                    manufacturer: mfg_label,
                    quantity: row.quantity,
                    article_number: row.article_number.clone(),
                    description: row.description.clone(),
                });
            }
            ManufacturerDetection::Supported(mfg) => {
                *acc.mfg_counts.entry(mfg.clone()).or_insert(0) += row.quantity;
                match rules.classify_item(
                    &mfg,
                    &row.article_number,
                    &row.type_number,
                    &row.description,
                    catalog,
                ) {
                    BomItemClassification::Terminal(spec) => {
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
                                    is_pure_neutral: !spec.tiers.is_empty()
                                        && spec.tiers.iter().all(|t| t.behavior.is_neutral()),
                                    is_pure_pe: !spec.tiers.is_empty()
                                        && spec.tiers.iter().all(|t| t.behavior.is_pe()),
                                },
                            );
                        }
                    }
                    BomItemClassification::Jumper => {
                        let art = if !row.type_number.is_empty() {
                            row.type_number.clone()
                        } else {
                            row.article_number.clone()
                        };
                        acc.ignored_jumpers.push(format!("{}× {}", row.quantity, art));
                    }
                    BomItemClassification::EndClamp { matched_bracket_id } => {
                        let art = if !row.type_number.is_empty() {
                            row.type_number.clone()
                        } else {
                            row.article_number.clone()
                        };
                        acc.end_clamps_from_pdf.push(PdfEndPartEntry {
                            article_number: art,
                            quantity: row.quantity,
                            matched_bracket_id,
                        });
                    }
                    BomItemClassification::EndPlate => {
                        let art = if !row.type_number.is_empty() {
                            row.type_number.clone()
                        } else {
                            row.article_number.clone()
                        };
                        acc.end_plates_from_pdf.push(PdfEndPartEntry {
                            article_number: art,
                            quantity: row.quantity,
                            matched_bracket_id: None,
                        });
                    }
                    BomItemClassification::NtAccessory => {
                        let art = if !row.type_number.is_empty() {
                            row.type_number.clone()
                        } else {
                            row.article_number.clone()
                        };
                        acc.nt_hints
                            .push(format!("{}× {} ({})", row.quantity, art, row.description));
                    }
                    BomItemClassification::MiscAccessory => {
                        let art = if !row.type_number.is_empty() {
                            row.type_number.clone()
                        } else {
                            row.article_number.clone()
                        };
                        acc.ignored_misc.push(format!("{}× {}", row.quantity, art));
                    }
                    BomItemClassification::UnknownTerminalArticle => {
                        acc.unknown_articles.push(UnknownArticleWarning {
                            manufacturer: mfg,
                            quantity: row.quantity,
                            article_number: row.article_number.clone(),
                            type_number: row.type_number.clone(),
                            description: row.description.clone(),
                        });
                    }
                }
            }
            ManufacturerDetection::Unknown => {
                // Unknown supplier without matched article/type: treat as foreign if article/supplier non-empty
                if !row.article_number.trim().is_empty() || !row.supplier.trim().is_empty() {
                    let mfg_label = if !row.supplier.trim().is_empty() {
                        row.supplier.trim().to_string()
                    } else {
                        "Unbekannt".to_string()
                    };
                    acc.foreign_items.push(SkippedForeignItem {
                        bmk: key,
                        manufacturer: mfg_label,
                        quantity: row.quantity,
                        article_number: row.article_number.clone(),
                        description: row.description.clone(),
                    });
                }
            }
        }
    }

    let mut strips = Vec::new();
    for key in strip_order {
        let Some(acc) = accumulators.remove(&key) else {
            continue;
        };

        let has_any_supported = !acc.terminal_order.is_empty()
            || !acc.unknown_articles.is_empty()
            || !acc.end_clamps_from_pdf.is_empty()
            || !acc.end_plates_from_pdf.is_empty()
            || !acc.nt_hints.is_empty();

        if !has_any_supported {
            skipped_foreign.extend(acc.foreign_items);
            continue;
        }
        // Any stray foreign items on an otherwise supported strip still go to the skipped log
        skipped_foreign.extend(acc.foreign_items);

        let dominant_manufacturer = acc
            .mfg_counts
            .iter()
            .max_by_key(|(_, c)| *c)
            .map(|(m, _)| m.clone())
            .unwrap_or_else(|| "WAGO".to_string());

        let terminals: Vec<ImportedTerminalEntry> = acc
            .terminal_order
            .iter()
            .filter_map(|id| acc.terminal_counts.get(id).cloned())
            .collect();
        let total_terminal_count: usize = terminals.iter().map(|t| t.quantity).sum();

        let nt_anchor_spec_id = terminals
            .iter()
            .find(|t| t.is_nt_or_install)
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
            auto_complement_brackets: !has_pdf_end_clamps && total_terminal_count > 0,
        };

        let variants = generate_variants(&terminals);

        strips.push(StripImportPlan {
            bmk: acc.bmk,
            dominant_manufacturer,
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
/// Returns `(strip, nt_anchor_range)` where `nt_anchor_range` marks the contiguous NT/install block range
/// for the sequential NT configuration modal, if applicable.
pub fn instantiate_imported_strip(
    plan: &StripImportPlan,
    variant_index: usize,
    include_end_clamps: bool,
    catalog: &TerminalCatalog,
) -> Option<(TerminalStrip, Option<(usize, usize)>)> {
    let variant = plan
        .variants
        .get(variant_index)
        .or_else(|| plan.variants.first())?;
    let default_spec_id = variant
        .block_spec_ids
        .first()
        .cloned()
        .unwrap_or_else(|| "wago-2002-1201".to_string());

    let mut strip = TerminalStrip::new(
        plan.bmk.clone(),
        1,
        (plan.total_terminal_count.max(1)) as u32,
        default_spec_id,
    );
    strip.description = format!("Importiert aus PDF ({})", plan.dominant_manufacturer);
    if include_end_clamps {
        strip.end_bracket_id = plan.end_parts.end_bracket_id.clone();
        strip.end_bracket_left = plan.end_parts.bracket_left;
        strip.end_bracket_right = plan.end_parts.bracket_right;
    } else {
        strip.end_bracket_id = plan.end_parts.end_bracket_id.clone();
        strip.end_bracket_left = false;
        strip.end_bracket_right = false;
    }

    for id in &variant.block_spec_ids {
        if let Some(spec) = catalog.find_by_id(id) {
            let blk_id = strip.allocate_block_id();
            strip.blocks.push(spec.instantiate_block(blk_id));
        }
    }
    strip.recalculate_labels();
    Some((strip, variant.nt_zone))
}
