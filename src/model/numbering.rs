use crate::catalog::{TerminalCatalog, TerminalSpec};
use crate::model::{
    NeutralLabelMode, NumberingOrder, PeLabelMode, TerminalStrip, TierBehavior,
};

/// Controls how a newly created strip determines its initial number of physical terminal blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StripGenerationMode {
    /// Generate enough blocks so sequential labels cover `start_number..=end_number`.
    ByNumberRange,
    /// Generate an exact number of physical terminal blocks.
    ByBlockCount(usize),
}

impl TerminalStrip {
    /// Generates a new `TerminalStrip` covering `start_number..=end_number` with `default_spec`
    /// and immediately calculates all tier labels.
    pub fn generate_from_range(
        name: impl Into<String>,
        start_number: u32,
        end_number: u32,
        default_spec: &TerminalSpec,
    ) -> Self {
        let (lo, hi) = if end_number >= start_number {
            (start_number, end_number)
        } else {
            (end_number, start_number)
        };

        let total_numbers = (hi - lo) + 1;
        let seq_per_block = default_spec.sequential_tier_count();
        let block_count = if seq_per_block == 0 {
            total_numbers as usize
        } else {
            (total_numbers as usize).div_ceil(seq_per_block)
        };

        Self::generate_with_block_count(name, lo, hi, block_count, default_spec)
    }

    /// Generates a new `TerminalStrip` with an explicit `block_count` starting at `start_number`.
    pub fn generate_with_block_count(
        name: impl Into<String>,
        start_number: u32,
        target_end_number: u32,
        block_count: usize,
        default_spec: &TerminalSpec,
    ) -> Self {
        let mut strip = Self::new(name, start_number, target_end_number, &default_spec.id);
        for _ in 0..block_count {
            let id = strip.allocate_block_id();
            strip.blocks.push(default_spec.instantiate_block(id));
        }
        strip.recalculate_labels();
        strip
    }

    /// Recomputes `computed_label` and `cad_display_label` for every tier of every block
    /// by distributing the linear sequence starting at `self.start_number`, applying N/PE modes
    /// and per-tier prefix/suffix special characters.
    pub fn recalculate_labels(&mut self) {
        let mut next_number = self.start_number;
        let mut next_n = 1u32;
        let mut next_pe = 1u32;

        for block in &mut self.blocks {
            let tier_count = block.tiers.len();
            let indices: Vec<usize> = match block.numbering_order {
                NumberingOrder::BottomToTop => (0..tier_count).collect(),
                NumberingOrder::TopToBottom => (0..tier_count).rev().collect(),
            };

            for idx in indices {
                let tier = &mut block.tiers[idx];
                tier.level_index = idx;

                let wrap_affix = |base: &str, prefix: &str, suffix: &str| -> String {
                    if prefix.is_empty() && suffix.is_empty() {
                        base.to_string()
                    } else {
                        format!("{prefix}{base}{suffix}")
                    }
                };

                if let Some(override_text) = &tier.custom_override {
                    let trimmed = override_text.trim();
                    let label = if trimmed.is_empty() {
                        String::new()
                    } else {
                        wrap_affix(trimmed, &tier.label_prefix, &tier.label_suffix)
                    };
                    tier.computed_label = label.clone();
                    tier.cad_display_label = if label.is_empty() && tier.behavior.is_pe() {
                        wrap_affix("PE", &tier.label_prefix, &tier.label_suffix)
                    } else {
                        label
                    };
                    continue;
                }

                if tier.behavior.is_neutral() {
                    let base = match self.neutral_mode {
                        NeutralLabelMode::Fixed => "N".to_string(),
                        NeutralLabelMode::SharedSequenceNumber => {
                            let v = next_number.to_string();
                            next_number = next_number.saturating_add(1);
                            v
                        }
                        NeutralLabelMode::SharedSequenceSuffixN => {
                            let v = format!("{next_number}N");
                            next_number = next_number.saturating_add(1);
                            v
                        }
                        NeutralLabelMode::SeparateCounterSuffixN => {
                            let v = format!("{next_n}N");
                            next_n = next_n.saturating_add(1);
                            v
                        }
                    };
                    let full = wrap_affix(&base, &tier.label_prefix, &tier.label_suffix);
                    tier.computed_label = full.clone();
                    tier.cad_display_label = full;
                    continue;
                }

                match &tier.behavior {
                    TierBehavior::Sequential => {
                        let base = next_number.to_string();
                        next_number = next_number.saturating_add(1);
                        let full = wrap_affix(&base, &tier.label_prefix, &tier.label_suffix);
                        tier.computed_label = full.clone();
                        tier.cad_display_label = full;
                    }
                    TierBehavior::Fixed(fixed) => {
                        let full = wrap_affix(fixed, &tier.label_prefix, &tier.label_suffix);
                        tier.computed_label = full.clone();
                        tier.cad_display_label = full;
                    }
                    TierBehavior::ProtectiveEarth => match self.pe_mode {
                        PeLabelMode::Fixed => {
                            tier.computed_label.clear();
                            tier.cad_display_label =
                                wrap_affix("PE", &tier.label_prefix, &tier.label_suffix);
                        }
                        PeLabelMode::SharedSequenceNumber => {
                            let base = next_number.to_string();
                            next_number = next_number.saturating_add(1);
                            let full = wrap_affix(&base, &tier.label_prefix, &tier.label_suffix);
                            tier.computed_label = full.clone();
                            tier.cad_display_label = full;
                        }
                        PeLabelMode::SharedSequenceSuffixPe => {
                            let base = format!("{next_number}PE");
                            next_number = next_number.saturating_add(1);
                            let full = wrap_affix(&base, &tier.label_prefix, &tier.label_suffix);
                            tier.computed_label = full.clone();
                            tier.cad_display_label = full;
                        }
                        PeLabelMode::SeparateCounterSuffixPe => {
                            let base = format!("{next_pe}PE");
                            next_pe = next_pe.saturating_add(1);
                            let full = wrap_affix(&base, &tier.label_prefix, &tier.label_suffix);
                            tier.computed_label = full.clone();
                            tier.cad_display_label = full;
                        }
                    },
                    TierBehavior::Unlabeled => {
                        tier.computed_label.clear();
                        tier.cad_display_label.clear();
                    }
                }
            }
        }
    }

    /// Sets `label_prefix` / `label_suffix` across selected blocks (optionally filtered to one
    /// aligned `target_visual_row`, e.g. `Some(0)` = E1, `Some(-1)` = PE, `None` = all tiers).
    pub fn apply_affix_to_selection(
        &mut self,
        block_indices: &[usize],
        target_visual_row: Option<isize>,
        prefix: &str,
        suffix: &str,
    ) {
        let len = self.blocks.len();
        for &b_idx in block_indices {
            if b_idx >= len {
                continue;
            }
            let block = &mut self.blocks[b_idx];
            for t_idx in 0..block.tiers.len() {
                if target_visual_row.is_none_or(|r| block.tier_visual_row(t_idx) == r) {
                    block.tiers[t_idx].label_prefix = prefix.to_string();
                    block.tiers[t_idx].label_suffix = suffix.to_string();
                }
            }
        }
        self.recalculate_labels();
    }

    /// Reassigns all blocks at the given `indices` to `new_spec` and recalculates labels.
    pub fn reassign_blocks(&mut self, indices: &[usize], new_spec: &TerminalSpec) {
        let len = self.blocks.len();
        for &idx in indices {
            if idx < len {
                TerminalCatalog::apply_spec_to_block(&mut self.blocks[idx], new_spec);
            }
        }
        self.recalculate_labels();
    }

    /// Appends `count` new blocks of `spec` at the end of the strip and recalculates labels.
    pub fn append_blocks(&mut self, count: usize, spec: &TerminalSpec) {
        let pos = self.blocks.len();
        self.insert_blocks(pos, count, spec);
    }

    /// Inserts `count` new blocks of `spec` at `at_index` and recalculates labels.
    pub fn insert_blocks(&mut self, at_index: usize, count: usize, spec: &TerminalSpec) {
        let clamped = at_index.min(self.blocks.len());
        let mut new_blocks = Vec::with_capacity(count);
        for _ in 0..count {
            let id = self.allocate_block_id();
            new_blocks.push(spec.instantiate_block(id));
        }
        self.blocks.splice(clamped..clamped, new_blocks);
        self.recalculate_labels();
    }

    /// Deletes blocks at `indices` and recalculates labels.
    pub fn delete_blocks(&mut self, indices: &[usize]) {
        let mut sorted: Vec<usize> = indices
            .iter()
            .copied()
            .filter(|&i| i < self.blocks.len())
            .collect();
        sorted.sort_unstable();
        sorted.dedup();
        for idx in sorted.into_iter().rev() {
            self.blocks.remove(idx);
        }
        self.recalculate_labels();
    }

    /// Moves all selected blocks by `delta` (`-1` = one position left, `+1` = one position right),
    /// preserving block IDs (so assigned jumper potentials stay attached) and returns the new selection.
    pub fn move_selected_blocks(
        &mut self,
        selected: &std::collections::BTreeSet<usize>,
        delta: isize,
    ) -> std::collections::BTreeSet<usize> {
        let len = self.blocks.len();
        if selected.is_empty() || len <= 1 || delta == 0 {
            return selected.clone();
        }
        let mut sel: Vec<usize> = selected.iter().copied().filter(|&i| i < len).collect();
        sel.sort_unstable();
        if sel.is_empty() {
            return selected.clone();
        }

        let mut next_set: std::collections::BTreeSet<usize> = sel.iter().copied().collect();
        if delta < 0 {
            for idx in sel {
                if idx > 0 && !next_set.contains(&(idx - 1)) {
                    self.blocks.swap(idx, idx - 1);
                    next_set.remove(&idx);
                    next_set.insert(idx - 1);
                }
            }
        } else {
            for idx in sel.into_iter().rev() {
                if idx + 1 < len && !next_set.contains(&(idx + 1)) {
                    self.blocks.swap(idx, idx + 1);
                    next_set.remove(&idx);
                    next_set.insert(idx + 1);
                }
            }
        }
        self.recalculate_labels();
        next_set
    }

    /// Duplicates all selected blocks immediately after the last selected block, allocating
    /// fresh block IDs and recalculating labels. Returns the indices of the newly created blocks.
    pub fn duplicate_selected_blocks(
        &mut self,
        selected: &std::collections::BTreeSet<usize>,
    ) -> std::collections::BTreeSet<usize> {
        let len = self.blocks.len();
        let mut sel: Vec<usize> = selected.iter().copied().filter(|&i| i < len).collect();
        sel.sort_unstable();
        if sel.is_empty() {
            return std::collections::BTreeSet::new();
        }
        let insert_at = (sel.last().copied().unwrap_or(0) + 1).min(len);
        let mut clones = Vec::with_capacity(sel.len());
        for &idx in &sel {
            let mut blk = self.blocks[idx].clone();
            blk.id = self.allocate_block_id();
            clones.push(blk);
        }
        let count = clones.len();
        self.blocks.splice(insert_at..insert_at, clones);
        self.recalculate_labels();
        (insert_at..insert_at + count).collect()
    }
}

/// Configuration for automatic placement of N-busbar carriers (`Sammelschienenträger` / `Auflagebock`)
/// framing an N-busbar installation zone and supporting long rows every 200 mm.
#[derive(Debug, Clone, Copy)]
pub struct BusbarCarrierPlan<'a> {
    /// Outer carrier placed before the left-most member (or N-supply terminal) and after the right-most member
    /// (e.g. WAGO `2009-305`, Phoenix `AB-PTI/3`, Weidmüller `AEB ITB 2.5/4 BB BL`).
    pub outer_carrier: Option<&'a TerminalSpec>,
    /// Intermediate support inserted whenever the unsupported terminal span exceeds
    /// `max_unsupported_span_tenths_mm` (e.g. WAGO `2009-304` 1.5 mm, Phoenix `AB-*`).
    pub intermediate_support: Option<&'a TerminalSpec>,
    /// Maximum unsupported distance along the N-busbar in `0.1 mm` (default `2000` = `200.0 mm`).
    pub max_unsupported_span_tenths_mm: u16,
}

/// Frames the contiguous N-busbar installation / supply zone containing `anchor_start..anchor_end`
/// with optional outer carriers and inserts intermediate supports before any unsupported span exceeds
/// `plan.max_unsupported_span_tenths_mm` (200.0 mm). Returns the updated `(zone_start, zone_end)` bounds.
pub fn apply_n_busbar_carriers_around(
    strip: &mut TerminalStrip,
    anchor_start: usize,
    anchor_end: usize,
    catalog: &TerminalCatalog,
    plan: &BusbarCarrierPlan<'_>,
) -> (usize, usize) {
    if strip.blocks.is_empty() || anchor_start >= anchor_end || anchor_end > strip.blocks.len() {
        return (0, 0);
    }

    let is_carrier = |spec_id: &str| {
        catalog
            .find_by_id(spec_id)
            .is_some_and(|s| s.is_busbar_carrier())
    };
    let is_zone_member = |spec_id: &str| {
        catalog.find_by_id(spec_id).is_some_and(|s| {
            s.is_installation_or_nt_terminal()
                || s.is_n_supply_terminal()
                || s.is_busbar_carrier()
        })
    };

    let mut zone_start = anchor_start;
    let mut zone_end = anchor_end;

    while zone_start > 0 && is_zone_member(&strip.blocks[zone_start - 1].spec_id) {
        zone_start -= 1;
    }
    while zone_end < strip.blocks.len() && is_zone_member(&strip.blocks[zone_end].spec_id) {
        zone_end += 1;
    }

    if let Some(outer) = plan.outer_carrier {
        if !is_carrier(&strip.blocks[zone_start].spec_id) {
            let id = strip.allocate_block_id();
            strip.blocks.insert(zone_start, outer.instantiate_block(id));
            zone_end += 1;
        }
        if zone_end == zone_start + 1 || !is_carrier(&strip.blocks[zone_end - 1].spec_id) {
            let id = strip.allocate_block_id();
            strip.blocks.insert(zone_end, outer.instantiate_block(id));
            zone_end += 1;
        }
    }

    if let Some(support) = plan.intermediate_support
        && plan.max_unsupported_span_tenths_mm > 0
    {
        let mut unsupported_tenths: u16 = 0;
        let mut idx = zone_start;
        while idx < zone_end {
            if is_carrier(&strip.blocks[idx].spec_id) {
                unsupported_tenths = 0;
                idx += 1;
                continue;
            }
            let w = strip.blocks[idx].width_tenths_mm;
            if unsupported_tenths > 0
                && unsupported_tenths.saturating_add(w) > plan.max_unsupported_span_tenths_mm
            {
                let id = strip.allocate_block_id();
                strip.blocks.insert(idx, support.instantiate_block(id));
                zone_end += 1;
                unsupported_tenths = w;
                idx += 2;
            } else {
                unsupported_tenths = unsupported_tenths.saturating_add(w);
                idx += 1;
            }
        }
    }

    strip.recalculate_labels();
    (zone_start, zone_end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::TerminalCatalog;

    #[test]
    fn example_1_standard_double_level_numbering() {
        let catalog = TerminalCatalog::load_embedded();
        let double_ll = catalog.find_by_id("wago-2002-2201").unwrap();

        // 4 double-level blocks -> numbers 1..8
        let strip = TerminalStrip::generate_from_range("-X1", 1, 8, double_ll);
        assert_eq!(strip.blocks.len(), 4);

        let lower: Vec<&str> = strip
            .blocks
            .iter()
            .map(|b| b.tiers[0].computed_label.as_str())
            .collect();
        let upper: Vec<&str> = strip
            .blocks
            .iter()
            .map(|b| b.tiers[1].computed_label.as_str())
            .collect();

        assert_eq!(lower, vec!["1", "3", "5", "7"]);
        assert_eq!(upper, vec!["2", "4", "6", "8"]);
    }

    #[test]
    fn example_2_mixed_double_level_with_neutral() {
        let catalog = TerminalCatalog::load_embedded();
        let double_ll = catalog.find_by_id("wago-2002-2201").unwrap();
        let double_ln = catalog.find_by_id("wago-2002-2204").unwrap();

        let mut strip = TerminalStrip::generate_with_block_count("-X1", 1, 6, 4, double_ll);
        // Reassign first 2 blocks to L/N (Lower sequential, Upper fixed "N")
        strip.reassign_blocks(&[0, 1], double_ln);

        let lower: Vec<&str> = strip
            .blocks
            .iter()
            .map(|b| b.tiers[0].computed_label.as_str())
            .collect();
        let upper: Vec<&str> = strip
            .blocks
            .iter()
            .map(|b| b.tiers[1].computed_label.as_str())
            .collect();

        assert_eq!(lower, vec!["1", "2", "3", "5"]);
        assert_eq!(upper, vec!["N", "N", "4", "6"]);
    }

    #[test]
    fn triple_level_pe_skips_pe_in_numbering_and_print_label() {
        let catalog = TerminalCatalog::load_embedded();
        let triple_peln = catalog.find_by_id("wago-2203-7641").unwrap();
        let single = catalog.find_by_id("wago-2002-1201").unwrap();

        let mut strip = TerminalStrip::generate_with_block_count("-X2", 1, 10, 3, triple_peln);
        strip.reassign_blocks(&[2], single);

        // Block 0: PE="", L="1", N="N"
        assert_eq!(strip.blocks[0].tiers[0].computed_label, "");
        assert_eq!(strip.blocks[0].tiers[0].cad_display_label, "PE");
        assert_eq!(strip.blocks[0].tiers[1].computed_label, "1");
        assert_eq!(strip.blocks[0].tiers[2].computed_label, "N");

        // Block 1: PE="", L="2", N="N"
        assert_eq!(strip.blocks[1].tiers[1].computed_label, "2");

        // Block 2 (single): L="3"
        assert_eq!(strip.blocks[2].tiers[0].computed_label, "3");
    }

    #[test]
    fn range_1_to_50_generates_expected_blocks() {
        let catalog = TerminalCatalog::load_embedded();
        let single = catalog.find_by_id("phoenix-pt-25").unwrap();
        let double_ll = catalog.find_by_id("phoenix-pttb-25").unwrap();

        let strip_single = TerminalStrip::generate_from_range("-X1", 1, 50, single);
        assert_eq!(strip_single.blocks.len(), 50);
        assert_eq!(strip_single.blocks[49].tiers[0].computed_label, "50");

        let strip_double = TerminalStrip::generate_from_range("-X1", 1, 50, double_ll);
        assert_eq!(strip_double.blocks.len(), 25);
        assert_eq!(strip_double.blocks[24].tiers[0].computed_label, "49");
        assert_eq!(strip_double.blocks[24].tiers[1].computed_label, "50");
    }

    #[test]
    fn custom_override_does_not_consume_sequence() {
        let catalog = TerminalCatalog::load_embedded();
        let single = catalog.find_by_id("wago-2002-1201").unwrap();
        let mut strip = TerminalStrip::generate_from_range("-X1", 1, 4, single);

        strip.blocks[1].tiers[0].custom_override = Some("24V+".to_string());
        strip.recalculate_labels();

        let labels: Vec<&str> = strip
            .blocks
            .iter()
            .map(|b| b.tiers[0].computed_label.as_str())
            .collect();
        assert_eq!(labels, vec!["1", "24V+", "2", "3"]);
    }

    #[test]
    fn n_and_pe_modes_and_batch_affixes_preserve_dynamic_numbering() {
        let catalog = TerminalCatalog::load_embedded();
        assert!(catalog.all().len() >= 270);
        let inst = catalog.find_by_id("wago-2003-7641").unwrap();
        let mut strip = TerminalStrip::generate_with_block_count("-X1", 1, 5, 2, inst);

        // Mode 1: Own separate N and PE counters -> 1N, 2N and 1PE, 2PE
        strip.neutral_mode = NeutralLabelMode::SeparateCounterSuffixN;
        strip.pe_mode = PeLabelMode::SeparateCounterSuffixPe;
        strip.recalculate_labels();

        assert_eq!(strip.blocks[0].tiers[0].computed_label, "1PE");
        assert_eq!(strip.blocks[0].tiers[1].computed_label, "1");
        assert_eq!(strip.blocks[0].tiers[2].computed_label, "1N");
        assert_eq!(strip.blocks[1].tiers[0].computed_label, "2PE");
        assert_eq!(strip.blocks[1].tiers[1].computed_label, "2");
        assert_eq!(strip.blocks[1].tiers[2].computed_label, "2N");

        // Mode 2: Shared sequence with main terminals
        strip.neutral_mode = NeutralLabelMode::SharedSequenceSuffixN;
        strip.pe_mode = PeLabelMode::SharedSequenceSuffixPe;
        strip.recalculate_labels();

        assert_eq!(strip.blocks[0].tiers[0].computed_label, "1PE");
        assert_eq!(strip.blocks[0].tiers[1].computed_label, "2");
        assert_eq!(strip.blocks[0].tiers[2].computed_label, "3N");
        assert_eq!(strip.blocks[1].tiers[0].computed_label, "4PE");
        assert_eq!(strip.blocks[1].tiers[1].computed_label, "5");
        assert_eq!(strip.blocks[1].tiers[2].computed_label, "6N");

        // Batch affix on E1 (visual row 0) stays attached even if start_number changes
        strip.apply_affix_to_selection(&[0, 1], Some(0), "+", "~");
        assert_eq!(strip.blocks[0].tiers[1].computed_label, "+2~");
        assert_eq!(strip.blocks[1].tiers[1].computed_label, "+5~");

        strip.start_number = 10;
        strip.recalculate_labels();
        assert_eq!(strip.blocks[0].tiers[0].computed_label, "10PE");
        assert_eq!(strip.blocks[0].tiers[1].computed_label, "+11~");
        assert_eq!(strip.blocks[0].tiers[2].computed_label, "12N");
    }

    #[test]
    fn end_plates_and_block_move_duplicate_work() {
        use std::collections::BTreeSet;
        use crate::catalog::accessories::calculate_end_plates;

        let catalog = TerminalCatalog::load_embedded();
        let wago_3l = catalog.find_by_id("wago-2003-7641").unwrap();
        let wago_1l_gray = catalog.find_by_id("wago-2002-1201").unwrap();
        let wago_1l_blue = catalog.find_by_id("wago-2002-1204").unwrap();
        let phx_pt25 = catalog.find_by_id("phoenix-pt-25").unwrap();

        let mut strip = TerminalStrip::generate_with_block_count("-X1", 1, 10, 4, wago_1l_gray);
        strip.reassign_blocks(&[0], wago_3l);
        strip.reassign_blocks(&[2], wago_1l_blue);
        strip.reassign_blocks(&[3], phx_pt25);

        // Block 0 (2003-7641) transitions to Block 1 (2002-1201) -> needs 0.8 mm plate after #0
        // Block 1 (2002-1201) & Block 2 (2002-1204) share contour 2002-1291 -> no plate after #1, plate after #2 (0.8 mm)
        // Block 3 (phoenix-pt-25) at end of strip -> needs 2.2 mm plate after #3
        let plates = calculate_end_plates(&strip, &catalog);
        assert_eq!(plates.len(), 3);
        assert_eq!(plates[0].after_block_idx, 0);
        assert_eq!(plates[0].width_tenths_mm, 8);
        assert_eq!(plates[1].after_block_idx, 2);
        assert_eq!(plates[1].width_tenths_mm, 8);
        assert_eq!(plates[2].after_block_idx, 3);
        assert_eq!(plates[2].width_tenths_mm, 22);

        // Move block 3 left to index 2 and duplicate it
        let mut sel = BTreeSet::new();
        sel.insert(3);
        let sel = strip.move_selected_blocks(&sel, -1);
        assert_eq!(sel, BTreeSet::from([2]));
        assert_eq!(strip.blocks[2].spec_id, "phoenix-pt-25");

        let dup_sel = strip.duplicate_selected_blocks(&sel);
        assert_eq!(dup_sel, BTreeSet::from([3]));
        assert_eq!(strip.blocks.len(), 5);
        assert_eq!(strip.blocks[3].spec_id, "phoenix-pt-25");
        assert_ne!(strip.blocks[2].id, strip.blocks[3].id);
    }
}
