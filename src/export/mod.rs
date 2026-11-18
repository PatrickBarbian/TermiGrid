pub mod csv;
pub mod dxf;
pub mod xlsx;

use crate::model::{Project, TerminalStrip};

/// Print ordering strategy for terminal marker exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrintOrder {
    /// Tier by tier across the strip: all Tier 1 labels contiguously, then all Tier 2, then Tier 3.
    #[default]
    ByTierStrip,
    /// Block by block, lower tier to upper tier (e.g., 1, 2, 3, 4...).
    ByBlockBottomToTop,
    /// Block by block, upper tier to lower tier.
    ByBlockTopToBottom,
}

/// Configuration for exporting labels to `.xlsx` or `.csv`.
#[derive(Debug, Clone)]
pub struct ExportOptions {
    pub print_order: PrintOrder,
    /// Whether PE tiers should be emitted as `"PE"` (default `false` because PE is usually unlabeled).
    pub include_pe_labels: bool,
    /// Export all strips in the project or only the currently active strip.
    pub all_strips: bool,
    /// Delimiter used for CSV export (e.g., `';'` for German Excel, `','` for international).
    pub csv_delimiter: char,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            print_order: PrintOrder::ByTierStrip,
            include_pe_labels: false,
            all_strips: false,
            csv_delimiter: ';',
        }
    }
}

/// A single flattened label entry ready for printing / export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintLabelRow {
    pub sequence_no: String,
    pub strip_name: String,
    pub block_position: String,
    pub tier_level: String,
    pub label: String,
    pub combined_designation: String,
    pub manufacturer: String,
    pub terminal_type: String,
}

/// Flattens one or more strips into ordered `PrintLabelRow`s according to `options`.
pub fn build_print_rows(
    project: &Project,
    active_strip_index: usize,
    options: &ExportOptions,
) -> Vec<PrintLabelRow> {
    let strips: Vec<&TerminalStrip> = if options.all_strips {
        project.strips.iter().collect()
    } else {
        project
            .strips
            .get(active_strip_index)
            .into_iter()
            .collect()
    };

    let mut rows = Vec::new();
    let mut seq = 1usize;

    for strip in strips {
        match options.print_order {
            PrintOrder::ByBlockBottomToTop | PrintOrder::ByBlockTopToBottom => {
                for (b_idx, block) in strip.blocks.iter().enumerate() {
                    let indices: Vec<usize> = match options.print_order {
                        PrintOrder::ByBlockBottomToTop => (0..block.tiers.len()).collect(),
                        PrintOrder::ByBlockTopToBottom => (0..block.tiers.len()).rev().collect(),
                        PrintOrder::ByTierStrip => unreachable!(),
                    };

                    for t_idx in indices {
                        let vrow = block.tier_visual_row(t_idx);
                        if let Some(row) =
                            extract_row(strip, b_idx, t_idx, vrow, options, &mut seq)
                        {
                            rows.push(row);
                        }
                    }
                }
            }
            PrintOrder::ByTierStrip => {
                let max_row = strip
                    .blocks
                    .iter()
                    .flat_map(|b| (0..b.tiers.len()).map(|l| b.tier_visual_row(l)))
                    .max()
                    .unwrap_or(0);

                let mut row_order: Vec<isize> = (0..=max_row).collect();
                if options.include_pe_labels {
                    row_order.push(-1);
                }

                for target_row in row_order {
                    for (b_idx, block) in strip.blocks.iter().enumerate() {
                        for t_idx in 0..block.tiers.len() {
                            if block.tier_visual_row(t_idx) == target_row
                                && let Some(row) = extract_row(
                                    strip,
                                    b_idx,
                                    t_idx,
                                    target_row,
                                    options,
                                    &mut seq,
                                )
                            {
                                rows.push(row);
                            }
                        }
                    }
                }
            }
        }
    }

    rows
}

/// Builds contiguous label lists per logical level (`Ebene 1`, `Ebene 2`, ..., optionally `PE`)
/// for direct column import / copy-paste into labeling printer software.
pub fn build_tier_columns(
    project: &Project,
    active_strip_index: usize,
    options: &ExportOptions,
    german_headers: bool,
) -> Vec<(String, Vec<String>)> {
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
        .flat_map(|b| {
            b.tiers
                .iter()
                .enumerate()
                .filter(|(_, t)| !t.behavior.is_pe())
                .map(|(l, _)| b.tier_visual_row(l))
        })
        .max()
        .unwrap_or(0)
        .max(0);
    let has_non_pe = strips
        .iter()
        .flat_map(|s| s.blocks.iter())
        .any(|b| b.tiers.iter().any(|t| !t.behavior.is_pe()));

    let mut columns = Vec::new();
    if has_non_pe {
        for target_row in 0..=max_row {
            let header = if german_headers {
                format!("Ebene {}", target_row + 1)
            } else {
                format!("Tier {}", target_row + 1)
            };
            let mut labels = Vec::new();
            for strip in &strips {
                for block in &strip.blocks {
                    for (t_idx, tier) in block.tiers.iter().enumerate() {
                        if !tier.behavior.is_pe()
                            && block.tier_visual_row(t_idx) == target_row
                            && !tier.computed_label.is_empty()
                        {
                            labels.push(tier.computed_label.clone());
                        }
                    }
                }
            }
            columns.push((header, labels));
        }
    }

    if options.include_pe_labels {
        let mut pe_labels = Vec::new();
        for strip in &strips {
            for block in &strip.blocks {
                for tier in &block.tiers {
                    if tier.behavior.is_pe() {
                        let lbl = if !tier.computed_label.is_empty() {
                            tier.computed_label.clone()
                        } else {
                            "PE".to_string()
                        };
                        pe_labels.push(lbl);
                    }
                }
            }
        }
        if !pe_labels.is_empty() {
            columns.push(("PE".to_string(), pe_labels));
        }
    }

    columns
}

fn extract_row(
    strip: &TerminalStrip,
    block_idx: usize,
    tier_idx: usize,
    visual_row: isize,
    options: &ExportOptions,
    seq: &mut usize,
) -> Option<PrintLabelRow> {
    let block = strip.blocks.get(block_idx)?;
    let tier = block.tiers.get(tier_idx)?;

    let label = if !tier.computed_label.is_empty() {
        tier.computed_label.clone()
    } else if options.include_pe_labels && tier.behavior.is_pe() {
        "PE".to_string()
    } else {
        return None;
    };

    let pe_count = block.tiers.iter().filter(|t| t.behavior.is_pe()).count();
    let tier_level = if tier.behavior.is_pe() {
        if pe_count > 1 {
            format!("PE{}", tier_idx + 1)
        } else {
            "PE".to_string()
        }
    } else if visual_row < 0 {
        "PE".to_string()
    } else {
        format!("E{}", visual_row + 1)
    };

    let row = PrintLabelRow {
        sequence_no: seq.to_string(),
        strip_name: strip.name.clone(),
        block_position: (block_idx + 1).to_string(),
        tier_level,
        label: label.clone(),
        combined_designation: format!("{}:{}", strip.name, label),
        manufacturer: block.manufacturer.clone(),
        terminal_type: block.short_type_name().to_string(),
    };
    *seq += 1;
    Some(row)
}
