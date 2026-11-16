use crate::catalog::{TerminalCatalog, TerminalSpec};
use crate::model::{NumberingOrder, TerminalStrip, TierBehavior};

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
    /// by distributing the linear sequence starting at `self.start_number`.
    pub fn recalculate_labels(&mut self) {
        let mut next_number = self.start_number;

        for block in &mut self.blocks {
            let tier_count = block.tiers.len();
            let indices: Vec<usize> = match block.numbering_order {
                NumberingOrder::BottomToTop => (0..tier_count).collect(),
                NumberingOrder::TopToBottom => (0..tier_count).rev().collect(),
            };

            for idx in indices {
                let tier = &mut block.tiers[idx];
                tier.level_index = idx;

                if let Some(override_text) = &tier.custom_override {
                    let trimmed = override_text.trim().to_string();
                    tier.computed_label = trimmed.clone();
                    tier.cad_display_label = if trimmed.is_empty() && tier.behavior.is_pe() {
                        "PE".to_string()
                    } else {
                        trimmed
                    };
                    continue;
                }

                match &tier.behavior {
                    TierBehavior::Sequential => {
                        let label = next_number.to_string();
                        next_number = next_number.saturating_add(1);
                        tier.computed_label = label.clone();
                        tier.cad_display_label = label;
                    }
                    TierBehavior::Fixed(fixed) => {
                        tier.computed_label = fixed.clone();
                        tier.cad_display_label = fixed.clone();
                    }
                    TierBehavior::ProtectiveEarth => {
                        // PE connections are normally not labeled on terminal markers.
                        tier.computed_label.clear();
                        tier.cad_display_label = "PE".to_string();
                    }
                    TierBehavior::Unlabeled => {
                        tier.computed_label.clear();
                        tier.cad_display_label.clear();
                    }
                }
            }
        }
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
}
