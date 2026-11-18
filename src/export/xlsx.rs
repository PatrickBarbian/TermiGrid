use std::collections::BTreeMap;
use std::path::Path;

use rust_xlsxwriter::{Format, Workbook, XlsxError};

use crate::catalog::accessories::{calculate_end_plates, find_end_bracket};
use crate::catalog::TerminalCatalog;
use crate::export::{build_print_rows, build_tier_columns, ExportOptions};
use crate::model::bridging::calculate_bridges;
use crate::model::{Project, TerminalStrip};

/// Exports the labeling sequence, contiguous per-tier columns, and a tier-matrix overview
/// to an `.xlsx` workbook.
///
/// Every cell in every worksheet is explicitly written as a string with the Excel `'@'`
/// Text number format so Excel never auto-converts labels like `"1-2"`, `"01"`, or `"10/2"`.
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

    // Sheet 1: Print Sequence (Druckfolge — contiguous per tier by default)
    let sheet1_name = if german_headers {
        "Druckfolge"
    } else {
        "Print Sequence"
    };
    let ws1 = workbook.add_worksheet();
    ws1.set_name(sheet1_name)?;

    let headers1 = if german_headers {
        [
            "Lfd. Nr.",
            "Klemmleiste",
            "Klemme Pos.",
            "Ebene",
            "Beschriftung",
            "Vollbezeichnung",
            "Hersteller",
            "Klemmentyp",
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
        ]
    };

    for (col, h) in headers1.iter().enumerate() {
        ws1.write_string_with_format(0, col as u16, *h, &header_text_fmt)?;
    }

    let rows = build_print_rows(project, active_strip_index, options);
    for (i, r) in rows.iter().enumerate() {
        let row_idx = (i + 1) as u32;
        ws1.write_string_with_format(row_idx, 0, &r.sequence_no, &cell_text_fmt)?;
        ws1.write_string_with_format(row_idx, 1, &r.strip_name, &cell_text_fmt)?;
        ws1.write_string_with_format(row_idx, 2, &r.block_position, &cell_text_fmt)?;
        ws1.write_string_with_format(row_idx, 3, &r.tier_level, &cell_text_fmt)?;
        ws1.write_string_with_format(row_idx, 4, &r.label, &cell_text_fmt)?;
        ws1.write_string_with_format(row_idx, 5, &r.combined_designation, &cell_text_fmt)?;
        ws1.write_string_with_format(row_idx, 6, &r.manufacturer, &cell_text_fmt)?;
        ws1.write_string_with_format(row_idx, 7, &r.terminal_type, &cell_text_fmt)?;
    }

    ws1.set_column_width(0, 9)?;
    ws1.set_column_width(1, 14)?;
    ws1.set_column_width(2, 13)?;
    ws1.set_column_width(3, 9)?;
    ws1.set_column_width(4, 16)?;
    ws1.set_column_width(5, 20)?;
    ws1.set_column_width(6, 18)?;
    ws1.set_column_width(7, 26)?;

    // Sheet 2: Contiguous Tier Columns (Ebenen-Spalten) for direct printer band import
    let sheet2_name = if german_headers {
        "Ebenen-Spalten"
    } else {
        "Tier Columns"
    };
    let ws2 = workbook.add_worksheet();
    ws2.set_name(sheet2_name)?;

    let tier_cols = build_tier_columns(project, active_strip_index, options, german_headers);
    for (col_idx, (header, labels)) in tier_cols.iter().enumerate() {
        ws2.write_string_with_format(0, col_idx as u16, header, &header_text_fmt)?;
        for (r_idx, label) in labels.iter().enumerate() {
            ws2.write_string_with_format((r_idx + 1) as u32, col_idx as u16, label, &cell_text_fmt)?;
        }
        ws2.set_column_width(col_idx as u16, 16)?;
    }

    // Sheet 3: Tier Matrix Overview (Ebenen-Matrix) aligned by logical row (E1, E2, E3, PE)
    let sheet3_name = if german_headers {
        "Ebenen-Matrix"
    } else {
        "Tier Matrix"
    };
    let ws3 = workbook.add_worksheet();
    ws3.set_name(sheet3_name)?;

    let strips: Vec<&TerminalStrip> = if options.all_strips {
        project.strips.iter().collect()
    } else {
        project
            .strips
            .get(active_strip_index)
            .into_iter()
            .collect()
    };

    let max_row = strips
        .iter()
        .flat_map(|s| s.blocks.iter())
        .flat_map(|b| (0..b.tiers.len()).map(|l| b.tier_visual_row(l)))
        .max()
        .unwrap_or(0)
        .max(0) as usize;
    let has_pe = strips
        .iter()
        .flat_map(|s| s.blocks.iter())
        .any(|b| b.tiers.iter().any(|t| t.behavior.is_pe()));

    let mut headers3 = if german_headers {
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

    for row in 0..=max_row {
        headers3.push(if german_headers {
            format!("Ebene {}", row + 1)
        } else {
            format!("Tier {}", row + 1)
        });
    }
    if has_pe {
        headers3.push("PE".to_string());
    }

    for (col, h) in headers3.iter().enumerate() {
        ws3.write_string_with_format(0, col as u16, h, &header_text_fmt)?;
    }

    let mut row_idx = 1u32;
    for &strip in &strips {
        for (b_idx, block) in strip.blocks.iter().enumerate() {
            ws3.write_string_with_format(row_idx, 0, &strip.name, &cell_text_fmt)?;
            ws3.write_string_with_format(row_idx, 1, (b_idx + 1).to_string(), &cell_text_fmt)?;
            ws3.write_string_with_format(row_idx, 2, &block.manufacturer, &cell_text_fmt)?;
            ws3.write_string_with_format(row_idx, 3, block.short_type_name(), &cell_text_fmt)?;

            for target_row in 0..=max_row {
                let text = block
                    .tiers
                    .iter()
                    .enumerate()
                    .find(|(l, _)| block.tier_visual_row(*l) == target_row as isize)
                    .map(|(_, t)| t.computed_label.as_str())
                    .unwrap_or("");
                ws3.write_string_with_format(
                    row_idx,
                    (4 + target_row) as u16,
                    text,
                    &cell_text_fmt,
                )?;
            }

            if has_pe {
                let pe_text = block
                    .tiers
                    .iter()
                    .enumerate()
                    .find(|(l, _)| block.tier_visual_row(*l) == -1)
                    .map(|(_, t)| {
                        if !t.computed_label.is_empty() {
                            t.computed_label.as_str()
                        } else if options.include_pe_labels && t.behavior.is_pe() {
                            "PE"
                        } else {
                            "PE (unbeschriftet)"
                        }
                    })
                    .unwrap_or("");
                ws3.write_string_with_format(
                    row_idx,
                    (5 + max_row) as u16,
                    pe_text,
                    &cell_text_fmt,
                )?;
            }

            row_idx += 1;
        }
    }

    ws3.set_column_width(0, 14)?;
    ws3.set_column_width(1, 8)?;
    ws3.set_column_width(2, 18)?;
    ws3.set_column_width(3, 26)?;
    for c in 4..headers3.len() {
        ws3.set_column_width(c as u16, 14)?;
    }

    // Sheet 4: Workshop Assembly Plan & Bill of Materials (Stückliste & Montage)
    let sheet4_name = if german_headers {
        "Stückliste & Montage"
    } else {
        "BOM & Assembly"
    };
    let ws4 = workbook.add_worksheet();
    ws4.set_name(sheet4_name)?;

    let mut catalog = TerminalCatalog::load_embedded();
    catalog.reload_with_custom(&project.custom_specs);

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
        ws4.write_string_with_format(0, c as u16, *h, &header_text_fmt)?;
    }

    let mut r4 = 1u32;
    for &strip in &strips {
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
        ws4.write_string_with_format(
            r4,
            0,
            if german_headers {
                "Klemmleiste (Maße)"
            } else {
                "Strip (Dimensions)"
            },
            &header_text_fmt,
        )?;
        ws4.write_string_with_format(r4, 1, &strip.name, &cell_text_fmt)?;
        ws4.write_string_with_format(r4, 2, "TS35", &cell_text_fmt)?;
        ws4.write_string_with_format(r4, 3, &strip.description, &cell_text_fmt)?;
        ws4.write_string_with_format(r4, 4, "-", &cell_text_fmt)?;
        ws4.write_string_with_format(
            r4,
            5,
            format!("{total_rail_mm:.1}"),
            &cell_text_fmt,
        )?;
        ws4.write_string_with_format(r4, 6, strip.blocks.len().to_string(), &cell_text_fmt)?;
        ws4.write_string_with_format(
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
        for (_, (mfg, name, art, w10, qty)) in block_counts {
            ws4.write_string_with_format(
                r4,
                0,
                if german_headers {
                    "Reihenklemme"
                } else {
                    "Terminal Block"
                },
                &cell_text_fmt,
            )?;
            ws4.write_string_with_format(r4, 1, &strip.name, &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 2, &mfg, &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 3, &name, &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 4, &art, &cell_text_fmt)?;
            ws4.write_string_with_format(
                r4,
                5,
                format!("{:.1}", f32::from(w10) / 10.0),
                &cell_text_fmt,
            )?;
            ws4.write_string_with_format(r4, 6, qty.to_string(), &cell_text_fmt)?;
            ws4.write_string_with_format(
                r4,
                7,
                format!(
                    "Gesamt: {:.1} mm",
                    (f32::from(w10) / 10.0) * (qty as f32)
                ),
                &cell_text_fmt,
            )?;
            r4 += 1;
        }

        // 3. Automatically calculated end plates with real width and positions
        let mut plate_groups: BTreeMap<String, (String, u16, Vec<usize>)> = BTreeMap::new();
        for p in &plates {
            let entry = plate_groups
                .entry(p.end_plate.clone())
                .or_insert_with(|| (p.manufacturer.clone(), p.width_tenths_mm, Vec::new()));
            entry.2.push(p.after_block_idx + 1);
        }
        for (plate_name, (mfg, w10, positions)) in plate_groups {
            let pos_str = positions
                .iter()
                .map(|i| format!("nach #{i}"))
                .collect::<Vec<_>>()
                .join(", ");
            ws4.write_string_with_format(
                r4,
                0,
                if german_headers {
                    "Abschlussdeckel"
                } else {
                    "End Plate"
                },
                &cell_text_fmt,
            )?;
            ws4.write_string_with_format(r4, 1, &strip.name, &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 2, &mfg, &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 3, &plate_name, &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 4, "-", &cell_text_fmt)?;
            ws4.write_string_with_format(
                r4,
                5,
                format!("{:.1}", f32::from(w10) / 10.0),
                &cell_text_fmt,
            )?;
            ws4.write_string_with_format(r4, 6, positions.len().to_string(), &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 7, &pos_str, &cell_text_fmt)?;
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
            ws4.write_string_with_format(
                r4,
                0,
                if german_headers {
                    "Endhalter"
                } else {
                    "End Bracket"
                },
                &cell_text_fmt,
            )?;
            ws4.write_string_with_format(r4, 1, &strip.name, &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 2, bracket.manufacturer, &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 3, bracket.name, &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 4, bracket.article_number, &cell_text_fmt)?;
            ws4.write_string_with_format(
                r4,
                5,
                format!("{:.1}", f32::from(bracket.width_tenths_mm) / 10.0),
                &cell_text_fmt,
            )?;
            ws4.write_string_with_format(r4, 6, bracket_qty.to_string(), &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 7, pos_parts.join(" & "), &cell_text_fmt)?;
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
            ws4.write_string_with_format(
                r4,
                0,
                if german_headers {
                    "Kammbrücke"
                } else {
                    "Comb Jumper"
                },
                &cell_text_fmt,
            )?;
            ws4.write_string_with_format(r4, 1, &strip.name, &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 2, seg_mfg, &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 3, &seg.part_hint, &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 4, "-", &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 5, "-", &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 6, "1", &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 7, &cut_note, &cell_text_fmt)?;
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
            ws4.write_string_with_format(
                r4,
                0,
                if german_headers {
                    "Drahtbrücke"
                } else {
                    "Wire Bridge"
                },
                &cell_text_fmt,
            )?;
            ws4.write_string_with_format(r4, 1, &strip.name, &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 2, "-", &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 3, "Drahtbrücke / Schaltlitze", &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 4, "-", &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 5, "-", &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 6, "1", &cell_text_fmt)?;
            ws4.write_string_with_format(r4, 7, &note, &cell_text_fmt)?;
            r4 += 1;
        }
    }

    ws4.set_column_width(0, 20)?;
    ws4.set_column_width(1, 14)?;
    ws4.set_column_width(2, 18)?;
    ws4.set_column_width(3, 34)?;
    ws4.set_column_width(4, 16)?;
    ws4.set_column_width(5, 14)?;
    ws4.set_column_width(6, 10)?;
    ws4.set_column_width(7, 52)?;

    workbook.save(path)?;
    Ok(rows.len())
}
