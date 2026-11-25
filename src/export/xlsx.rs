use std::collections::BTreeMap;
use std::path::Path;

use rust_xlsxwriter::{Format, Workbook, XlsxError};

use crate::catalog::accessories::{calculate_end_plates, find_end_bracket};
use crate::catalog::TerminalCatalog;
use crate::export::{
    build_print_rows_for_strip, build_tier_columns_for_strips, ExportOptions,
};
use crate::model::bridging::calculate_bridges;
use crate::model::{Project, TerminalStrip};

/// Sanitizes a terminal strip designation into a valid, unique Excel worksheet tab name
/// (`<= 31` chars, no `\ / ? * [ ] :`, no leading/trailing `'`, and no case-insensitive duplicates).
pub fn sanitize_excel_sheet_name(
    raw: &str,
    used_lower: &mut Vec<String>,
    reserved_lower: &[&str],
) -> String {
    let mut base: String = raw
        .trim()
        .chars()
        .map(|c| match c {
            '\\' | '/' | '?' | '*' | '[' | ']' | ':' => '_',
            _ => c,
        })
        .collect();
    while base.starts_with('\'') {
        base.remove(0);
    }
    while base.ends_with('\'') {
        base.pop();
    }
    if base.is_empty() {
        base = "Strip".to_string();
    }

    let truncate_chars = |s: &str, max_chars: usize| -> String {
        s.chars().take(max_chars).collect()
    };

    let is_taken = |candidate: &str, used: &[String]| -> bool {
        let lc = candidate.to_lowercase();
        used.iter().any(|u| u == &lc) || reserved_lower.iter().any(|r| *r == lc)
    };

    let first = truncate_chars(&base, 31);
    if !first.eq_ignore_ascii_case("history") && !is_taken(&first, used_lower) {
        used_lower.push(first.to_lowercase());
        return first;
    }

    let mut suffix_num = 2usize;
    loop {
        let suffix = format!("_{suffix_num}");
        let max_prefix = 31usize.saturating_sub(suffix.chars().count());
        let candidate = format!("{}{}", truncate_chars(&base, max_prefix), suffix);
        if !candidate.eq_ignore_ascii_case("history") && !is_taken(&candidate, used_lower) {
            used_lower.push(candidate.to_lowercase());
            return candidate;
        }
        suffix_num += 1;
    }
}

/// Exports each selected terminal strip cleanly onto its own dedicated Excel worksheet tab
/// (e.g., `-X1`, `-X2`) along with `Ebenen-Matrix` / `Tier Matrix` and `Stückliste & Montage` / `BOM & Assembly`.
///
/// Every cell in every worksheet is explicitly written as a string with the Excel `'@'`
/// Text number format so Excel never auto-converts labels like `"1-2"`, `"01"`, or `"10/2"`.
/// Required end plates, partition plates, and end brackets are automatically calculated
/// even if the user did not trigger accessory generation manually beforehand.
pub fn export_to_xlsx(
    path: &Path,
    project: &Project,
    active_strip_index: usize,
    options: &ExportOptions,
    german_headers: bool,
) -> Result<usize, XlsxError> {
    let mut workbook = Workbook::new();

    // Strict Excel Text format ('@') applied to ALL cells.
    let header_text_fmt = Format::new().set_num_format("@").set_bold();
    let cell_text_fmt = Format::new().set_num_format("@");

    let mut catalog = TerminalCatalog::load_embedded();
    let mut combined_custom = crate::model::library::load_global_custom_specs();
    crate::model::library::merge_custom_specs_into(&project.custom_specs, &mut combined_custom);
    catalog.reload_with_custom(&combined_custom);

    let strips: Vec<&TerminalStrip> = if options.all_strips {
        project.strips.iter().collect()
    } else {
        project
            .strips
            .get(active_strip_index)
            .into_iter()
            .collect()
    };

    let matrix_sheet_name = if german_headers {
        "Ebenen-Matrix"
    } else {
        "Tier Matrix"
    };
    let bom_sheet_name = if german_headers {
        "Stückliste & Montage"
    } else {
        "BOM & Assembly"
    };
    let reserved_lower = [
        matrix_sheet_name.to_lowercase(),
        bom_sheet_name.to_lowercase(),
    ];
    let reserved_refs: Vec<&str> = reserved_lower.iter().map(String::as_str).collect();
    let mut used_sheet_names_lower: Vec<String> = Vec::new();

    let mut total_labels = 0usize;

    // ── Dedicated Worksheet per Terminal Strip (e.g., -X1, -X2, ...) ─────────
    let strip_headers = if german_headers {
        [
            "Lfd. Nr.",
            "Klemmleiste",
            "Klemme Pos.",
            "Ebene",
            "Beschriftung",
            "Vollbezeichnung",
            "Hersteller",
            "Klemmentyp",
            "Beschriftungsmaterial",
        ]
    } else {
        [
            "Seq.",
            "Strip",
            "Block Pos.",
            "Tier",
            "Label",
            "Full Designation",
            "Manufacturer",
            "Terminal Type",
            "Marker Type",
        ]
    };

    for &strip in &strips {
        let tab_name =
            sanitize_excel_sheet_name(&strip.name, &mut used_sheet_names_lower, &reserved_refs);
        let ws = workbook.add_worksheet();
        ws.set_name(&tab_name)?;

        for (col, h) in strip_headers.iter().enumerate() {
            ws.write_string_with_format(0, col as u16, *h, &header_text_fmt)?;
        }

        // Resolve marker designation per block position (1-based string -> marker type)
        let block_markers: Vec<String> = strip
            .blocks
            .iter()
            .map(|blk| {
                catalog
                    .find_by_id(&blk.spec_id)
                    .map(|sp| catalog.accessories_for(sp).markers)
                    .unwrap_or_else(|| "-".to_string())
            })
            .collect();

        let mut seq = 1usize;
        let strip_rows = build_print_rows_for_strip(strip, options, &mut seq);
        total_labels += strip_rows.len();

        for (i, r) in strip_rows.iter().enumerate() {
            let row_idx = (i + 1) as u32;
            let pos_1based = r.block_position.parse::<usize>().unwrap_or(0);
            let marker_str = pos_1based
                .checked_sub(1)
                .and_then(|idx| block_markers.get(idx))
                .map(String::as_str)
                .unwrap_or("-");

            ws.write_string_with_format(row_idx, 0, &r.sequence_no, &cell_text_fmt)?;
            ws.write_string_with_format(row_idx, 1, &r.strip_name, &cell_text_fmt)?;
            ws.write_string_with_format(row_idx, 2, &r.block_position, &cell_text_fmt)?;
            ws.write_string_with_format(row_idx, 3, &r.tier_level, &cell_text_fmt)?;
            ws.write_string_with_format(row_idx, 4, &r.label, &cell_text_fmt)?;
            ws.write_string_with_format(row_idx, 5, &r.combined_designation, &cell_text_fmt)?;
            ws.write_string_with_format(row_idx, 6, &r.manufacturer, &cell_text_fmt)?;
            ws.write_string_with_format(row_idx, 7, &r.terminal_type, &cell_text_fmt)?;
            ws.write_string_with_format(row_idx, 8, marker_str, &cell_text_fmt)?;
        }

        ws.set_column_width(0, 9)?;
        ws.set_column_width(1, 14)?;
        ws.set_column_width(2, 13)?;
        ws.set_column_width(3, 9)?;
        ws.set_column_width(4, 16)?;
        ws.set_column_width(5, 20)?;
        ws.set_column_width(6, 18)?;
        ws.set_column_width(7, 26)?;
        ws.set_column_width(8, 30)?;
        ws.set_column_width(9, 3)?; // spacer column

        // Contiguous per-tier print columns strictly for THIS terminal strip
        let tier_cols = build_tier_columns_for_strips(&[strip], options, german_headers);
        let tier_start_col: u16 = 10;
        for (offset, (header, labels)) in tier_cols.iter().enumerate() {
            let col_idx = tier_start_col + offset as u16;
            ws.write_string_with_format(0, col_idx, header, &header_text_fmt)?;
            for (r_idx, label) in labels.iter().enumerate() {
                ws.write_string_with_format((r_idx + 1) as u32, col_idx, label, &cell_text_fmt)?;
            }
            ws.set_column_width(col_idx, 15)?;
        }

        // Required Marker Type summary block right beside the tier columns
        let marker_sum_start_col = tier_start_col + tier_cols.len() as u16 + 1;
        ws.set_column_width(marker_sum_start_col - 1, 3)?;
        let marker_sum_headers = if german_headers {
            [
                "Benötigtes Beschriftungsmaterial",
                "Hersteller",
                "Anzahl Klemmen",
                "Klemmentypen",
            ]
        } else {
            [
                "Required Marker Type",
                "Manufacturer",
                "Terminal Qty",
                "Terminal Types",
            ]
        };
        for (o, h) in marker_sum_headers.iter().enumerate() {
            ws.write_string_with_format(0, marker_sum_start_col + o as u16, *h, &header_text_fmt)?;
        }

        let mut marker_groups: BTreeMap<(String, String), (usize, Vec<String>)> = BTreeMap::new();
        for blk in &strip.blocks {
            let marker = catalog
                .find_by_id(&blk.spec_id)
                .map(|sp| catalog.accessories_for(sp).markers)
                .unwrap_or_else(|| "-".to_string());
            let entry = marker_groups
                .entry((marker, blk.manufacturer.clone()))
                .or_insert_with(|| (0, Vec::new()));
            entry.0 += 1;
            let short = blk.short_type_name().to_string();
            if !entry.1.contains(&short) {
                entry.1.push(short);
            }
        }
        for (idx, ((marker_name, mfg), (qty, type_list))) in marker_groups.into_iter().enumerate() {
            let r = (idx + 1) as u32;
            ws.write_string_with_format(r, marker_sum_start_col, &marker_name, &cell_text_fmt)?;
            ws.write_string_with_format(r, marker_sum_start_col + 1, &mfg, &cell_text_fmt)?;
            ws.write_string_with_format(
                r,
                marker_sum_start_col + 2,
                qty.to_string(),
                &cell_text_fmt,
            )?;
            ws.write_string_with_format(
                r,
                marker_sum_start_col + 3,
                type_list.join(", "),
                &cell_text_fmt,
            )?;
        }
        ws.set_column_width(marker_sum_start_col, 32)?;
        ws.set_column_width(marker_sum_start_col + 1, 18)?;
        ws.set_column_width(marker_sum_start_col + 2, 16)?;
        ws.set_column_width(marker_sum_start_col + 3, 28)?;
    }

    // ── Summary Sheet: Tier Matrix Overview (Ebenen-Matrix) ──────────────────
    let ws_matrix = workbook.add_worksheet();
    ws_matrix.set_name(matrix_sheet_name)?;

    let max_row = strips
        .iter()
        .flat_map(|s| s.blocks.iter())
        .flat_map(|b| {
            b.tiers
                .iter()
                .enumerate()
                .filter(|(_, t)| !t.behavior.is_pe())
                .map(|(l, _)| b.tier_visual_row(l))
        })
        .max()
        .unwrap_or(0)
        .max(0) as usize;
    let has_non_pe = strips
        .iter()
        .flat_map(|s| s.blocks.iter())
        .any(|b| b.tiers.iter().any(|t| !t.behavior.is_pe()));
    let has_pe = strips
        .iter()
        .flat_map(|s| s.blocks.iter())
        .any(|b| b.tiers.iter().any(|t| t.behavior.is_pe()));

    let mut headers_matrix = if german_headers {
        vec![
            "Klemmleiste".to_string(),
            "Pos.".to_string(),
            "Hersteller".to_string(),
            "Klemmentyp".to_string(),
        ]
    } else {
        vec![
            "Strip".to_string(),
            "Pos.".to_string(),
            "Manufacturer".to_string(),
            "Terminal Type".to_string(),
        ]
    };

    if has_non_pe {
        for row in 0..=max_row {
            headers_matrix.push(if german_headers {
                format!("Ebene {}", row + 1)
            } else {
                format!("Tier {}", row + 1)
            });
        }
    }
    if has_pe {
        headers_matrix.push("PE".to_string());
    }

    for (col, h) in headers_matrix.iter().enumerate() {
        ws_matrix.write_string_with_format(0, col as u16, h, &header_text_fmt)?;
    }

    let mut row_idx = 1u32;
    for &strip in &strips {
        for (b_idx, block) in strip.blocks.iter().enumerate() {
            ws_matrix.write_string_with_format(row_idx, 0, &strip.name, &cell_text_fmt)?;
            ws_matrix.write_string_with_format(
                row_idx,
                1,
                (b_idx + 1).to_string(),
                &cell_text_fmt,
            )?;
            ws_matrix.write_string_with_format(row_idx, 2, &block.manufacturer, &cell_text_fmt)?;
            ws_matrix.write_string_with_format(
                row_idx,
                3,
                block.short_type_name(),
                &cell_text_fmt,
            )?;

            let mut col_cursor = 4u16;
            if has_non_pe {
                let is_pure_multi_pe =
                    block.tiers.len() >= 2 && block.tiers.iter().all(|t| t.behavior.is_pe());
                for target_row in 0..=max_row {
                    let text = block
                        .tiers
                        .iter()
                        .enumerate()
                        .find(|(l, t)| {
                            block.tier_visual_row(*l) == target_row as isize
                                && (!t.behavior.is_pe() || !is_pure_multi_pe)
                        })
                        .map(|(_, t)| {
                            if !t.computed_label.is_empty() {
                                t.computed_label.as_str()
                            } else if t.behavior.is_pe() {
                                "PE"
                            } else {
                                ""
                            }
                        })
                        .unwrap_or("");
                    ws_matrix.write_string_with_format(
                        row_idx,
                        col_cursor,
                        text,
                        &cell_text_fmt,
                    )?;
                    col_cursor += 1;
                }
            }

            if has_pe {
                let pe_tiers: Vec<_> = block
                    .tiers
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| t.behavior.is_pe())
                    .collect();
                let pe_text = if pe_tiers.is_empty() {
                    String::new()
                } else if pe_tiers.len() == 1 {
                    let t = pe_tiers[0].1;
                    if !t.computed_label.is_empty() {
                        t.computed_label.clone()
                    } else if options.include_pe_labels {
                        "PE".to_string()
                    } else {
                        "PE (unbeschriftet)".to_string()
                    }
                } else {
                    pe_tiers
                        .iter()
                        .map(|(l, t)| {
                            let lbl = if !t.computed_label.is_empty() {
                                t.computed_label.as_str()
                            } else if options.include_pe_labels {
                                "PE"
                            } else {
                                "unbeschriftet"
                            };
                            format!("PE{}: {}", l + 1, lbl)
                        })
                        .collect::<Vec<_>>()
                        .join(" / ")
                };
                ws_matrix.write_string_with_format(
                    row_idx,
                    col_cursor,
                    &pe_text,
                    &cell_text_fmt,
                )?;
            }

            row_idx += 1;
        }
    }

    ws_matrix.set_column_width(0, 14)?;
    ws_matrix.set_column_width(1, 8)?;
    ws_matrix.set_column_width(2, 18)?;
    ws_matrix.set_column_width(3, 26)?;
    for c in 4..headers_matrix.len() {
        ws_matrix.set_column_width(c as u16, 14)?;
    }

    // ── Summary Sheet: Workshop Assembly Plan & Bill of Materials ────────────
    let ws_bom = workbook.add_worksheet();
    ws_bom.set_name(bom_sheet_name)?;

    let bom_headers = if german_headers {
        [
            "Kategorie",
            "Klemmleiste",
            "Hersteller",
            "Bezeichnung / Typ",
            "Art.-Nr.",
            "Breite (mm)",
            "Menge",
            "Einbauort / Montagehinweis",
        ]
    } else {
        [
            "Category",
            "Strip",
            "Manufacturer",
            "Designation / Type",
            "Article No.",
            "Width (mm)",
            "Qty",
            "Placement / Assembly Note",
        ]
    };
    for (c, h) in bom_headers.iter().enumerate() {
        ws_bom.write_string_with_format(0, c as u16, *h, &header_text_fmt)?;
    }

    let mut r4 = 1u32;
    for &strip in &strips {
        // Automatically calculate all required end plates, partition plates, bridges & end brackets
        let plates = calculate_end_plates(strip, &catalog);
        let bridges = calculate_bridges(strip, &catalog);
        let blocks_mm = strip.total_width_mm();
        let plates_mm: f32 = plates
            .iter()
            .map(|p| f32::from(p.width_tenths_mm) / 10.0)
            .sum();
        let bracket = find_end_bracket(&strip.end_bracket_id);
        let bracket_qty = strip.end_bracket_count();
        let brackets_mm = strip.end_brackets_width_mm();
        let total_rail_mm = blocks_mm + plates_mm + brackets_mm;

        // 1. Strip dimension summary row
        ws_bom.write_string_with_format(
            r4,
            0,
            if german_headers {
                "Klemmleiste (Maße)"
            } else {
                "Strip (Dimensions)"
            },
            &header_text_fmt,
        )?;
        ws_bom.write_string_with_format(r4, 1, &strip.name, &cell_text_fmt)?;
        ws_bom.write_string_with_format(r4, 2, "TS35", &cell_text_fmt)?;
        ws_bom.write_string_with_format(r4, 3, &strip.description, &cell_text_fmt)?;
        ws_bom.write_string_with_format(r4, 4, "-", &cell_text_fmt)?;
        ws_bom.write_string_with_format(r4, 5, format!("{total_rail_mm:.1}"), &cell_text_fmt)?;
        ws_bom.write_string_with_format(
            r4,
            6,
            strip.blocks.len().to_string(),
            &cell_text_fmt,
        )?;
        ws_bom.write_string_with_format(
            r4,
            7,
            format!(
                "Klemmen: {blocks_mm:.1} mm + {}× Deckel: {plates_mm:.1} mm + {bracket_qty}× Endhalter: {brackets_mm:.1} mm = {total_rail_mm:.1} mm",
                plates.len(),
            ),
            &cell_text_fmt,
        )?;
        r4 += 1;

        // 2. Aggregated terminal blocks per strip
        let mut block_counts: BTreeMap<String, (String, String, String, u16, usize)> =
            BTreeMap::new();
        for blk in &strip.blocks {
            let art = catalog
                .find_by_id(&blk.spec_id)
                .and_then(|s| s.effective_article_number())
                .unwrap_or("-")
                .to_string();
            let entry = block_counts.entry(blk.spec_id.clone()).or_insert_with(|| {
                (
                    blk.manufacturer.clone(),
                    blk.spec_name.clone(),
                    art,
                    blk.width_tenths_mm,
                    0,
                )
            });
            entry.4 += 1;
        }
        for (spec_id, (mfg, name, art, w10, qty)) in block_counts {
            let is_carrier = catalog
                .find_by_id(&spec_id)
                .is_some_and(|s| s.is_busbar_carrier());
            let category_label = if is_carrier {
                if german_headers {
                    "Sammelschienenträger"
                } else {
                    "Busbar Carrier"
                }
            } else if german_headers {
                "Reihenklemme"
            } else {
                "Terminal Block"
            };
            ws_bom.write_string_with_format(r4, 0, category_label, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 1, &strip.name, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 2, &mfg, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 3, &name, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 4, &art, &cell_text_fmt)?;
            ws_bom.write_string_with_format(
                r4,
                5,
                format!("{:.1}", f32::from(w10) / 10.0),
                &cell_text_fmt,
            )?;
            ws_bom.write_string_with_format(r4, 6, qty.to_string(), &cell_text_fmt)?;
            ws_bom.write_string_with_format(
                r4,
                7,
                format!("Gesamt: {:.1} mm", (f32::from(w10) / 10.0) * (qty as f32)),
                &cell_text_fmt,
            )?;
            r4 += 1;
        }

        // 3. Automatically calculated end plates (strip end) & partition plates (contour transitions)
        let block_len = strip.blocks.len();
        let mut plate_groups: BTreeMap<(bool, String), (String, u16, Vec<usize>)> = BTreeMap::new();
        for p in &plates {
            let is_strip_end = p.after_block_idx + 1 >= block_len;
            let entry = plate_groups
                .entry((is_strip_end, p.end_plate.clone()))
                .or_insert_with(|| (p.manufacturer.clone(), p.width_tenths_mm, Vec::new()));
            entry.2.push(p.after_block_idx + 1);
        }
        for ((is_strip_end, plate_name), (mfg, w10, positions)) in plate_groups {
            let category = if is_strip_end {
                if german_headers {
                    "Endplatte / Abschlussplatte"
                } else {
                    "End Plate"
                }
            } else if german_headers {
                "Zwischenplatte"
            } else {
                "Partition Plate"
            };
            let pos_str = positions
                .iter()
                .map(|i| {
                    if is_strip_end {
                        format!("nach #{i} (Leistenende)")
                    } else {
                        format!("nach #{i} (Profilwechsel)")
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            ws_bom.write_string_with_format(r4, 0, category, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 1, &strip.name, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 2, &mfg, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 3, &plate_name, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 4, "-", &cell_text_fmt)?;
            ws_bom.write_string_with_format(
                r4,
                5,
                format!("{:.1}", f32::from(w10) / 10.0),
                &cell_text_fmt,
            )?;
            ws_bom.write_string_with_format(r4, 6, positions.len().to_string(), &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 7, &pos_str, &cell_text_fmt)?;
            r4 += 1;
        }

        // 3b. DIN-rail end brackets (Endhalter)
        if bracket_qty > 0 {
            let mut pos_parts = Vec::new();
            if strip.end_bracket_left {
                pos_parts.push(if german_headers {
                    "vor #1".to_string()
                } else {
                    "before #1".to_string()
                });
            }
            if strip.end_bracket_right && !strip.blocks.is_empty() {
                pos_parts.push(if german_headers {
                    format!("nach #{}", strip.blocks.len())
                } else {
                    format!("after #{}", strip.blocks.len())
                });
            }
            ws_bom.write_string_with_format(
                r4,
                0,
                if german_headers {
                    "Endhalter"
                } else {
                    "End Bracket"
                },
                &cell_text_fmt,
            )?;
            ws_bom.write_string_with_format(r4, 1, &strip.name, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 2, bracket.manufacturer, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 3, bracket.name, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 4, bracket.article_number, &cell_text_fmt)?;
            ws_bom.write_string_with_format(
                r4,
                5,
                format!("{:.1}", f32::from(bracket.width_tenths_mm) / 10.0),
                &cell_text_fmt,
            )?;
            ws_bom.write_string_with_format(r4, 6, bracket_qty.to_string(), &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 7, pos_parts.join(" & "), &cell_text_fmt)?;
            r4 += 1;
        }

        // 4. Comb jumpers & cut-out pin instructions
        for seg in &bridges.comb_segments {
            let pot_name = seg.potential_name.as_str();
            let seg_mfg = strip
                .blocks
                .get(seg.start_block_idx)
                .map(|b| b.manufacturer.as_str())
                .unwrap_or("-");
            let row_lbl = if seg.visual_row == -1 {
                "PE".to_string()
            } else {
                format!("E{}", seg.visual_row + 1)
            };
            let cut_note = if seg.cut_pins.is_empty() {
                format!(
                    "Potential {pot_name}: #{}..#{} ({row_lbl})",
                    seg.start_block_idx + 1,
                    seg.end_block_idx + 1
                )
            } else {
                let cuts = seg
                    .cut_pins
                    .iter()
                    .map(|i| format!("#{}", i + 1))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "Potential {pot_name}: #{}..#{} ({row_lbl}) — PIN ENTFERNEN BEI: {cuts}",
                    seg.start_block_idx + 1,
                    seg.end_block_idx + 1
                )
            };
            ws_bom.write_string_with_format(
                r4,
                0,
                if german_headers {
                    "Kammbrücke"
                } else {
                    "Comb Jumper"
                },
                &cell_text_fmt,
            )?;
            ws_bom.write_string_with_format(r4, 1, &strip.name, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 2, seg_mfg, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 3, &seg.part_hint, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 4, "-", &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 5, "-", &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 6, "1", &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 7, &cut_note, &cell_text_fmt)?;
            r4 += 1;
        }

        // 5. Wire bridges
        for wb in &bridges.wire_bridges {
            let reason_txt = match wb.reason {
                crate::model::bridging::WireBridgeReason::CrossTier => "Etagenwechsel",
                crate::model::bridging::WireBridgeReason::ExceedsMaxPolesSingleSlot => {
                    "1-Kammer max. Polzahl überschritten"
                }
                crate::model::bridging::WireBridgeReason::SingleChannelOccupied => {
                    if german_headers {
                        "Brückungskanal belegt"
                    } else {
                        "Jumper channel occupied"
                    }
                }
                crate::model::bridging::WireBridgeReason::EndPlateBarrier => {
                    if german_headers {
                        "Enddeckel / Zwischenplatte"
                    } else {
                        "End / partition plate"
                    }
                }
                crate::model::bridging::WireBridgeReason::IncompatiblePitchOrGap => {
                    if german_headers {
                        "Raster-/Typ-Sprung"
                    } else {
                        "Pitch / type jump"
                    }
                }
            };
            let note = format!(
                "Potential {}: #{} (E{}) -> #{} (E{}) ({})",
                wb.potential_name,
                wb.from.0 + 1,
                wb.from.1 + 1,
                wb.to.0 + 1,
                wb.to.1 + 1,
                reason_txt
            );
            ws_bom.write_string_with_format(
                r4,
                0,
                if german_headers {
                    "Drahtbrücke"
                } else {
                    "Wire Bridge"
                },
                &cell_text_fmt,
            )?;
            ws_bom.write_string_with_format(r4, 1, &strip.name, &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 2, "-", &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 3, "Drahtbrücke / Schaltlitze", &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 4, "-", &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 5, "-", &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 6, "1", &cell_text_fmt)?;
            ws_bom.write_string_with_format(r4, 7, &note, &cell_text_fmt)?;
            r4 += 1;
        }
    }

    ws_bom.set_column_width(0, 26)?;
    ws_bom.set_column_width(1, 14)?;
    ws_bom.set_column_width(2, 18)?;
    ws_bom.set_column_width(3, 34)?;
    ws_bom.set_column_width(4, 16)?;
    ws_bom.set_column_width(5, 14)?;
    ws_bom.set_column_width(6, 10)?;
    ws_bom.set_column_width(7, 52)?;

    workbook.save(path)?;
    Ok(total_labels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excel_sheet_name_sanitizer_handles_invalid_chars_length_and_collisions() {
        let mut used = Vec::new();
        let reserved = ["ebenen-matrix", "stückliste & montage"];

        assert_eq!(sanitize_excel_sheet_name("-X1", &mut used, &reserved), "-X1");
        assert_eq!(
            sanitize_excel_sheet_name("-X1", &mut used, &reserved),
            "-X1_2"
        );
        assert_eq!(
            sanitize_excel_sheet_name("=G1/K2:A*B?[3]", &mut used, &reserved),
            "=G1_K2_A_B__3_"
        );
        assert_eq!(
            sanitize_excel_sheet_name("'Ebenen-Matrix'", &mut used, &reserved),
            "Ebenen-Matrix_2"
        );

        let long = "ABCDEFGHIJKLMNOPQRSTUVWXYZ_1234567890";
        let truncated = sanitize_excel_sheet_name(long, &mut used, &reserved);
        assert_eq!(truncated.chars().count(), 31);
    }
}
