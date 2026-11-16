use std::path::Path;

use rust_xlsxwriter::{Format, Workbook, XlsxError};

use crate::export::{build_print_rows, build_tier_columns, ExportOptions};
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
    for strip in strips {
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

    workbook.save(path)?;
    Ok(rows.len())
}
