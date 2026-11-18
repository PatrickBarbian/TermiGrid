use std::collections::{BTreeMap, BTreeSet};

use crate::catalog::accessories::calculate_end_plates;
use crate::catalog::TerminalCatalog;
use crate::model::{JumperPotential, TerminalStrip};

/// Palette of high-contrast colors cycled when creating new potentials.
pub const POTENTIAL_PALETTE: [[u8; 3]; 8] = [
    [239, 68, 68],   // Red (L1 / +24V)
    [59, 130, 246],  // Blue (0V / N / GND)
    [245, 158, 11],  // Amber (L2 / Aux)
    [168, 85, 247],  // Purple (L3 / Signal)
    [16, 185, 129],  // Emerald
    [236, 72, 153],  // Pink
    [6, 182, 212],   // Cyan
    [234, 88, 12],   // Orange
];

/// Why a wire bridge (`Drahtbrücke`) is needed instead of a single comb jumper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireBridgeReason {
    /// Same potential spans different physical tiers (e.g., E1 <-> E2 on multi-level blocks).
    CrossTier,
    /// Span exceeds max catalog jumper poles on a single-chamber terminal (cannot cascade).
    ExceedsMaxPolesSingleSlot,
    /// Single jumper chamber on this block/gap is already occupied by another potential.
    SingleChannelOccupied,
    /// Separated by an end/partition plate (`Enddeckel` / `Zwischenplatte`) blocking comb jumpers.
    EndPlateBarrier,
    /// Different manufacturer/pitch/family or large gap between blocks.
    IncompatiblePitchOrGap,
}

/// One horizontal comb jumper (`Kammbrücke`) piece spanning contiguous/near-contiguous blocks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombJumperSegment {
    pub potential_id: u64,
    pub potential_name: String,
    pub color_rgb: [u8; 3],
    pub visual_row: isize,
    /// `0` = Slot 1 (upper/only track), `1` = Slot 2 (second track on dual-chamber blocks).
    pub channel_slot: u8,
    pub start_block_idx: usize,
    pub end_block_idx: usize,
    pub pole_count: usize,
    /// Block indices that actively belong to this potential in the segment.
    pub pin_blocks: Vec<usize>,
    /// Intermediate block indices inside `[start..=end]` whose jumper pin must be cut/broken out.
    pub cut_pins: Vec<usize>,
    pub part_hint: String,
    pub uses_double_chamber_cascade: bool,
}

/// One wire bridge (`Drahtbrücke`) connecting two specific `(block_idx, level_index)` terminals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireBridge {
    pub potential_id: u64,
    pub potential_name: String,
    pub color_rgb: [u8; 3],
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub reason: WireBridgeReason,
}

/// Complete calculated bridging plan & BOM for a terminal strip.
#[derive(Debug, Clone, Default)]
pub struct BridgePlan {
    pub comb_segments: Vec<CombJumperSegment>,
    pub wire_bridges: Vec<WireBridge>,
    /// Assigned points for fast CAD rendering: `(block_idx, level_index) -> (potential_id, name, rgb)`.
    pub point_assignments: BTreeMap<(usize, usize), (u64, String, [u8; 3])>,
}

#[derive(Debug, Clone)]
struct BlockJumperMeta {
    family: String,
    end_plate_key: String,
    slots: u8,
    max_poles: u8,
    part_hint_spec: Option<usize>,
}

impl TerminalStrip {
    /// Creates a new potential and returns its allocated ID.
    pub fn add_potential(&mut self, name: impl Into<String>, color_rgb: [u8; 3]) -> u64 {
        let id = self.allocate_potential_id();
        self.potentials.push(JumperPotential {
            id,
            name: name.into(),
            color_rgb,
            members: Vec::new(),
        });
        id
    }

    /// Removes a potential by ID.
    pub fn remove_potential(&mut self, potential_id: u64) {
        self.potentials.retain(|p| p.id != potential_id);
    }

    /// Toggles membership of `(block_idx, level_index)` in `potential_id`.
    /// Removes the point from any other potential first so each connection point has at most one active potential.
    pub fn toggle_potential_member(
        &mut self,
        potential_id: u64,
        block_idx: usize,
        level_index: usize,
    ) {
        let Some(block) = self.blocks.get(block_idx) else {
            return;
        };
        if level_index >= block.tiers.len() {
            return;
        }
        let target_key = (block.id, level_index);

        let already_in_this = self
            .potentials
            .iter()
            .find(|p| p.id == potential_id)
            .is_some_and(|p| p.members.contains(&target_key));

        for p in &mut self.potentials {
            p.members.retain(|&m| m != target_key);
        }

        if !already_in_this
            && let Some(pot) = self.potentials.iter_mut().find(|p| p.id == potential_id)
        {
            pot.members.push(target_key);
        }
    }

    /// Explicitly adds (`present = true`) or removes (`present = false`) `(block_idx, level_index)`
    /// for `potential_id` (used by continuous drag-painting in Bridge Mode).
    pub fn set_potential_member(
        &mut self,
        potential_id: u64,
        block_idx: usize,
        level_index: usize,
        present: bool,
    ) {
        let Some(block) = self.blocks.get(block_idx) else {
            return;
        };
        if level_index >= block.tiers.len() {
            return;
        }
        let target_key = (block.id, level_index);

        for p in &mut self.potentials {
            p.members.retain(|&m| m != target_key);
        }

        if present && let Some(pot) = self.potentials.iter_mut().find(|p| p.id == potential_id) {
            pot.members.push(target_key);
        }
    }
}

/// Calculates comb jumpers (`Kammbrücken`), cut-out pins (`Pin entfernen`),
/// dual/single chamber allocation, and wire bridges (`Drahtbrücken`) for `strip`.
pub fn calculate_bridges(strip: &TerminalStrip, catalog: &TerminalCatalog) -> BridgePlan {
    let mut plan = BridgePlan::default();
    if strip.blocks.is_empty() || strip.potentials.is_empty() {
        return plan;
    }

    // Map block ID -> current physical index
    let mut id_to_idx: BTreeMap<u64, usize> = BTreeMap::new();
    let mut block_metas: Vec<BlockJumperMeta> = Vec::with_capacity(strip.blocks.len());
    for (idx, block) in strip.blocks.iter().enumerate() {
        id_to_idx.insert(block.id, idx);
        let spec = catalog.find_by_id(&block.spec_id);
        let (family, end_plate_key, slots, max_poles) = if let Some(sp) = spec {
            let acc = catalog.accessories_for(sp);
            (
                sp.jumper_family_key(),
                acc.end_plate,
                sp.effective_jumper_slots(),
                sp.effective_max_jumper_poles(),
            )
        } else {
            (
                format!("{}:generic:{}", block.manufacturer, block.width_tenths_mm),
                format!("generic:{}", block.spec_id),
                1,
                10,
            )
        };
        block_metas.push(BlockJumperMeta {
            family,
            end_plate_key,
            slots,
            max_poles,
            part_hint_spec: spec.and_then(|sp| {
                catalog
                    .all()
                    .iter()
                    .position(|s| std::ptr::eq(s, sp) || s.id == sp.id)
            }),
        });
    }

    // Physical end/partition plates placed between `after_block_idx` and `after_block_idx + 1`.
    // A rigid comb jumper can never pass through an end plate wall or bridge across its added thickness.
    let end_plate_after: BTreeSet<usize> = calculate_end_plates(strip, catalog)
        .into_iter()
        .map(|p| p.after_block_idx)
        .collect();

    // Track chamber occupancy `(block_idx, visual_row, channel_slot) -> potential_id`
    let mut chamber_owner: BTreeMap<(usize, isize, u8), u64> = BTreeMap::new();

    for pot in &strip.potentials {
        let mut nodes: Vec<(usize, usize, isize)> = Vec::new();
        for &(b_id, l_idx) in &pot.members {
            if let Some(&b_idx) = id_to_idx.get(&b_id)
                && let Some(block) = strip.blocks.get(b_idx)
                && l_idx < block.tiers.len()
            {
                let vrow = block.tier_visual_row(l_idx);
                nodes.push((b_idx, l_idx, vrow));
                plan.point_assignments.insert(
                    (b_idx, l_idx),
                    (pot.id, pot.name.clone(), pot.color_rgb),
                );
            }
        }
        nodes.sort_unstable();
        nodes.dedup();
        if nodes.len() < 2 {
            continue;
        }

        // Group nodes by `(visual_row, jumper_family)` and process horizontally
        let mut by_row_family: BTreeMap<(isize, String), Vec<(usize, usize)>> = BTreeMap::new();
        for &(b_idx, l_idx, vrow) in &nodes {
            let fam = block_metas[b_idx].family.clone();
            by_row_family
                .entry((vrow, fam))
                .or_default()
                .push((b_idx, l_idx));
        }

        // Connected components within this potential (`Dsu` over `nodes` index) to add wire bridges between islands/tiers
        let mut components: Vec<Vec<(usize, usize, isize)>> = Vec::new();

        for ((vrow, fam), group) in by_row_family {
            // Split into horizontal runs that can potentially share a comb jumper (max gap <= 3 compatible blocks,
            // no end/partition plate between blocks, no incompatible block in between, and no occupied single chamber).
            let mut runs: Vec<Vec<(usize, usize)>> = Vec::new();
            let mut current_run: Vec<(usize, usize)> = Vec::new();

            for &(b_idx, l_idx) in &group {
                if let Some(&(prev_b, _)) = current_run.last() {
                    if can_comb_span(
                        strip,
                        &block_metas,
                        &end_plate_after,
                        &chamber_owner,
                        pot.id,
                        vrow,
                        &fam,
                        prev_b,
                        b_idx,
                    ) {
                        current_run.push((b_idx, l_idx));
                    } else {
                        runs.push(std::mem::take(&mut current_run));
                        current_run.push((b_idx, l_idx));
                    }
                } else {
                    current_run.push((b_idx, l_idx));
                }
            }
            if !current_run.is_empty() {
                runs.push(current_run);
            }

            // Convert each run into 1+ comb jumper segments respecting `max_poles` and `1` vs `2` jumper chambers!
            for run in runs {
                if run.len() == 1 {
                    let (b, l) = run[0];
                    components.push(vec![(b, l, vrow)]);
                    continue;
                }

                let max_poles = usize::from(block_metas[run[0].0].max_poles.max(2));
                let mut seg_cursor = 0usize;
                let mut current_slot: u8 = find_free_slot(&chamber_owner, &block_metas, pot.id, vrow, run[0].0).unwrap_or(0);
                let mut active_component: Vec<(usize, usize, isize)> =
                    run.iter().map(|&(b, l)| (b, l, vrow)).collect();

                while seg_cursor < run.len() {
                    // Find furthest reachable member index `end_k` from `seg_cursor` within `max_poles` span
                    let start_b = run[seg_cursor].0;
                    let mut end_k = seg_cursor;
                    for (k, &(cand_b, _)) in run.iter().enumerate().skip(seg_cursor + 1) {
                        let span = cand_b - start_b + 1;
                        if span <= max_poles {
                            end_k = k;
                        } else {
                            break;
                        }
                    }

                    if end_k == seg_cursor {
                        // Edge case: even two neighboring members are > max_poles apart (can't happen since gap <= 3 and max_poles >= 2 unless max_poles==2 and gap>=1)
                        if seg_cursor + 1 < run.len() {
                            let (fb, fl) = run[seg_cursor];
                            let (tb, tl) = run[seg_cursor + 1];
                            plan.wire_bridges.push(WireBridge {
                                potential_id: pot.id,
                                potential_name: pot.name.clone(),
                                color_rgb: pot.color_rgb,
                                from: (fb, fl),
                                to: (tb, tl),
                                reason: WireBridgeReason::ExceedsMaxPolesSingleSlot,
                            });
                        }
                        seg_cursor += 1;
                        continue;
                    }

                    let end_b = run[end_k].0;
                    let pole_count = end_b - start_b + 1;
                    let pin_set: BTreeSet<usize> = run[seg_cursor..=end_k].iter().map(|&(b, _)| b).collect();
                    let pin_blocks: Vec<usize> = pin_set.iter().copied().collect();
                    let cut_pins: Vec<usize> = (start_b..=end_b)
                        .filter(|b| !pin_set.contains(b))
                        .collect();

                    // Claim chambers along this comb segment
                    let slot_to_use = current_slot;
                    for (b, meta) in block_metas.iter().enumerate().take(end_b + 1).skip(start_b) {
                        let max_slots = meta.slots.max(1);
                        let clamped_slot = slot_to_use.min(max_slots - 1);
                        chamber_owner.insert((b, vrow, clamped_slot), pot.id);
                    }

                    let part_hint = block_metas[start_b]
                        .part_hint_spec
                        .and_then(|si| catalog.all().get(si))
                        .map(|sp| sp.jumper_part_hint(pole_count))
                        .unwrap_or_else(|| format!("{pole_count}-fach Brücke"));

                    let has_more = end_k + 1 < run.len();
                    let join_slots = block_metas[end_b].slots;
                    let cascaded = has_more && join_slots >= 2;

                    plan.comb_segments.push(CombJumperSegment {
                        potential_id: pot.id,
                        potential_name: pot.name.clone(),
                        color_rgb: pot.color_rgb,
                        visual_row: vrow,
                        channel_slot: slot_to_use,
                        start_block_idx: start_b,
                        end_block_idx: end_b,
                        pole_count,
                        pin_blocks,
                        cut_pins,
                        part_hint,
                        uses_double_chamber_cascade: cascaded,
                    });

                    if !has_more {
                        break;
                    }

                    if join_slots >= 2 {
                        // Double chamber allows cascading right on `end_b` in the OTHER slot!
                        current_slot = if slot_to_use == 0 { 1 } else { 0 };
                        seg_cursor = end_k;
                    } else {
                        // Single chamber CANNOT overlap two comb jumpers on `end_b`:
                        // end segment cleanly at `end_b`, bridge to `end_k + 1` with a Wire Bridge!
                        let (fb, fl) = run[end_k];
                        let (tb, tl) = run[end_k + 1];
                        plan.wire_bridges.push(WireBridge {
                            potential_id: pot.id,
                            potential_name: pot.name.clone(),
                            color_rgb: pot.color_rgb,
                            from: (fb, fl),
                            to: (tb, tl),
                            reason: WireBridgeReason::ExceedsMaxPolesSingleSlot,
                        });
                        current_slot = 0;
                        seg_cursor = end_k + 1;
                    }
                }

                if !active_component.is_empty() {
                    components.push(std::mem::take(&mut active_component));
                }
            }
        }

        // Finally, connect any remaining disconnected components of `pot` (different tiers / different pitch / single-chamber blocks)
        // via minimum-distance Wire Bridges (`Drahtbrücken`).
        if components.len() > 1 {
            type NodePos = (usize, usize, isize);
            type BestCandidate = (usize, usize, NodePos, NodePos);

            let mut merged = vec![false; components.len()];
            merged[0] = true;
            let mut frontier = components[0].clone();

            for _ in 1..components.len() {
                let mut best: Option<BestCandidate> = None;

                for (c_idx, comp) in components.iter().enumerate() {
                    if merged[c_idx] {
                        continue;
                    }
                    for &a in &frontier {
                        for &b in comp {
                            // Distance metric: strongly prefer vertical same-column links first, then nearest block distance
                            let dx = a.0.abs_diff(b.0) * 10;
                            let dy = a.2.abs_diff(b.2);
                            let dist = dx + dy;
                            if best.as_ref().is_none_or(|(best_d, _, _, _)| dist < *best_d) {
                                best = Some((dist, c_idx, a, b));
                            }
                        }
                    }
                }

                if let Some((_, c_idx, a, b)) = best {
                    let (lo, hi) = if (a.0, a.1) <= (b.0, b.1) {
                        (a, b)
                    } else {
                        (b, a)
                    };
                    let reason = if lo.2 != hi.2 {
                        WireBridgeReason::CrossTier
                    } else {
                        let fam_a = &block_metas[lo.0].family;
                        let fam_b = &block_metas[hi.0].family;
                        let has_end_plate_barrier = (lo.0..hi.0).any(|mid| {
                            end_plate_after.contains(&mid)
                                || block_metas[mid].end_plate_key
                                    != block_metas[mid + 1].end_plate_key
                        });
                        if fam_a != fam_b {
                            WireBridgeReason::IncompatiblePitchOrGap
                        } else if has_end_plate_barrier {
                            WireBridgeReason::EndPlateBarrier
                        } else if hi.0.abs_diff(lo.0) > 4 {
                            WireBridgeReason::IncompatiblePitchOrGap
                        } else {
                            WireBridgeReason::SingleChannelOccupied
                        }
                    };
                    plan.wire_bridges.push(WireBridge {
                        potential_id: pot.id,
                        potential_name: pot.name.clone(),
                        color_rgb: pot.color_rgb,
                        from: (lo.0, lo.1),
                        to: (hi.0, hi.1),
                        reason,
                    });
                    merged[c_idx] = true;
                    frontier.extend_from_slice(&components[c_idx]);
                }
            }
        }
    }

    plan
}

fn find_free_slot(
    chamber_owner: &BTreeMap<(usize, isize, u8), u64>,
    metas: &[BlockJumperMeta],
    pot_id: u64,
    vrow: isize,
    b_idx: usize,
) -> Option<u8> {
    let slots = metas[b_idx].slots.max(1);
    for slot in 0..slots {
        match chamber_owner.get(&(b_idx, vrow, slot)) {
            None => return Some(slot),
            Some(&owner) if owner == pot_id => return Some(slot),
            Some(_) => {}
        }
    }
    None
}

#[allow(clippy::too_many_arguments)]
fn can_comb_span(
    strip: &TerminalStrip,
    metas: &[BlockJumperMeta],
    end_plate_after: &BTreeSet<usize>,
    chamber_owner: &BTreeMap<(usize, isize, u8), u64>,
    pot_id: u64,
    vrow: isize,
    family: &str,
    from_b: usize,
    to_b: usize,
) -> bool {
    if to_b <= from_b {
        return false;
    }
    let gap = to_b - from_b - 1;
    // Do not span excessive unrelated blocks (> 3); a wire bridge is cleaner there.
    if gap > 3 {
        return false;
    }

    for mid in from_b..=to_b {
        if mid < to_b
            && (end_plate_after.contains(&mid)
                || metas[mid].end_plate_key != metas[mid + 1].end_plate_key)
        {
            return false;
        }
        if metas[mid].family != family {
            return false;
        }
        let block = &strip.blocks[mid];
        let has_matching_row = (0..block.tiers.len()).any(|l| block.tier_visual_row(l) == vrow);
        if !has_matching_row {
            return false;
        }
        if find_free_slot(chamber_owner, metas, pot_id, vrow, mid).is_none() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::TerminalCatalog;

    #[test]
    fn adjacent_double_chamber_exceeding_max_poles_cascades_without_wire_bridge() {
        let catalog = TerminalCatalog::load_embedded();
        let wago_2002 = catalog.find_by_id("wago-2002-1201").unwrap();
        assert_eq!(wago_2002.effective_jumper_slots(), 2);
        assert_eq!(wago_2002.effective_max_jumper_poles(), 10);

        let mut strip = TerminalStrip::generate_with_block_count("-X1", 1, 14, 14, wago_2002);
        let pot = strip.add_potential("+24V", POTENTIAL_PALETTE[0]);
        for i in 0..14 {
            strip.toggle_potential_member(pot, i, 0);
        }

        let plan = calculate_bridges(&strip, &catalog);
        assert_eq!(plan.comb_segments.len(), 2);
        assert_eq!(plan.comb_segments[0].pole_count, 10);
        assert_eq!(plan.comb_segments[0].channel_slot, 0);
        assert!(plan.comb_segments[0].uses_double_chamber_cascade);
        assert_eq!(plan.comb_segments[1].start_block_idx, 9);
        assert_eq!(plan.comb_segments[1].end_block_idx, 13);
        assert_eq!(plan.comb_segments[1].pole_count, 5);
        assert_eq!(plan.comb_segments[1].channel_slot, 1);
        assert!(plan.wire_bridges.is_empty());
    }

    #[test]
    fn adjacent_single_chamber_exceeding_max_poles_requires_wire_bridge() {
        let catalog = TerminalCatalog::load_embedded();
        let wago_2001 = catalog.find_by_id("wago-2001-1201").unwrap();
        assert_eq!(wago_2001.effective_jumper_slots(), 1);

        let mut strip = TerminalStrip::generate_with_block_count("-X1", 1, 12, 12, wago_2001);
        let pot = strip.add_potential("L1", POTENTIAL_PALETTE[0]);
        for i in 0..12 {
            strip.toggle_potential_member(pot, i, 0);
        }

        let plan = calculate_bridges(&strip, &catalog);
        assert_eq!(plan.comb_segments.len(), 2);
        assert_eq!(plan.comb_segments[0].start_block_idx, 0);
        assert_eq!(plan.comb_segments[0].end_block_idx, 9);
        assert_eq!(plan.comb_segments[1].start_block_idx, 10);
        assert_eq!(plan.comb_segments[1].end_block_idx, 11);
        assert_eq!(plan.wire_bridges.len(), 1);
        assert_eq!(
            plan.wire_bridges[0].reason,
            WireBridgeReason::ExceedsMaxPolesSingleSlot
        );
    }

    #[test]
    fn skipped_compatible_block_detects_cut_pin_and_cross_tier_adds_wire_bridge() {
        let catalog = TerminalCatalog::load_embedded();
        let double_ll = catalog.find_by_id("wago-2002-2201").unwrap();
        let mut strip = TerminalStrip::generate_with_block_count("-X1", 1, 8, 4, double_ll);

        let pot = strip.add_potential("GND", POTENTIAL_PALETTE[1]);
        // E1 on blocks #0 and #2 (skipping #1 -> Cut pin at #1!) + E2 on block #2 (CrossTier wire bridge!)
        strip.toggle_potential_member(pot, 0, 0);
        strip.toggle_potential_member(pot, 2, 0);
        strip.toggle_potential_member(pot, 2, 1);

        let plan = calculate_bridges(&strip, &catalog);
        assert_eq!(plan.comb_segments.len(), 1);
        assert_eq!(plan.comb_segments[0].pole_count, 3);
        assert_eq!(plan.comb_segments[0].cut_pins, vec![1]);
        assert_eq!(plan.wire_bridges.len(), 1);
        assert_eq!(plan.wire_bridges[0].reason, WireBridgeReason::CrossTier);
    }

    #[test]
    fn comb_jumper_never_crosses_end_plate_uses_wire_bridge_instead() {
        let catalog = TerminalCatalog::load_embedded();
        let install_peln = catalog.find_by_id("wago-2003-7641").unwrap();
        let double_ll = catalog.find_by_id("wago-2002-2201").unwrap();
        let single = catalog.find_by_id("wago-2002-1201").unwrap();

        // 6x 2003-7641 (#0..#5), 3x 2002-2201 (#6..#8), 6x 2002-1201 (#9..#14)
        // End plates sit after #5 (2003-7692) and after #8 (2002-2291).
        let mut strip = TerminalStrip::generate_with_block_count("-X1", 1, 20, 15, single);
        strip.reassign_blocks(&[0, 1, 2, 3, 4, 5], install_peln);
        strip.reassign_blocks(&[6, 7, 8], double_ll);

        let pot = strip.add_potential("+24V", POTENTIAL_PALETTE[0]);
        // E1 on 2003-7641 is level_index 1 (level 0 is PE)
        strip.toggle_potential_member(pot, 0, 1);
        strip.toggle_potential_member(pot, 2, 1);
        strip.toggle_potential_member(pot, 5, 1);
        // E1 on 2002-1201 is level_index 0
        strip.toggle_potential_member(pot, 9, 0);
        strip.toggle_potential_member(pot, 10, 0);
        strip.toggle_potential_member(pot, 11, 0);
        strip.toggle_potential_member(pot, 13, 0);
        strip.toggle_potential_member(pot, 14, 0);

        let plan = calculate_bridges(&strip, &catalog);
        assert_eq!(plan.comb_segments.len(), 2);
        assert_eq!(plan.comb_segments[0].start_block_idx, 0);
        assert_eq!(plan.comb_segments[0].end_block_idx, 5);
        assert_eq!(plan.comb_segments[0].pole_count, 6);
        assert_eq!(plan.comb_segments[0].cut_pins, vec![1, 3, 4]);

        assert_eq!(plan.comb_segments[1].start_block_idx, 9);
        assert_eq!(plan.comb_segments[1].end_block_idx, 14);
        assert_eq!(plan.comb_segments[1].pole_count, 6);
        assert_eq!(plan.comb_segments[1].cut_pins, vec![12]);

        assert_eq!(plan.wire_bridges.len(), 1);
        assert_eq!(plan.wire_bridges[0].from, (5, 1));
        assert_eq!(plan.wire_bridges[0].to, (9, 0));
        assert_eq!(plan.wire_bridges[0].reason, WireBridgeReason::EndPlateBarrier);
    }

    #[test]
    fn adjacent_blocks_separated_by_end_plate_use_wire_bridge() {
        let catalog = TerminalCatalog::load_embedded();
        let install_peln = catalog.find_by_id("wago-2003-7641").unwrap();
        let double_ll = catalog.find_by_id("wago-2002-2201").unwrap();

        let mut strip = TerminalStrip::generate_with_block_count("-X1", 1, 6, 4, double_ll);
        strip.reassign_blocks(&[0, 1], install_peln);

        let pot = strip.add_potential("+24V", POTENTIAL_PALETTE[0]);
        strip.toggle_potential_member(pot, 0, 1);
        strip.toggle_potential_member(pot, 1, 1);
        strip.toggle_potential_member(pot, 2, 0);
        strip.toggle_potential_member(pot, 3, 0);

        let plan = calculate_bridges(&strip, &catalog);
        assert_eq!(plan.comb_segments.len(), 2);
        assert_eq!(plan.comb_segments[0].start_block_idx, 0);
        assert_eq!(plan.comb_segments[0].end_block_idx, 1);
        assert_eq!(plan.comb_segments[1].start_block_idx, 2);
        assert_eq!(plan.comb_segments[1].end_block_idx, 3);
        assert_eq!(plan.wire_bridges.len(), 1);
        assert_eq!(plan.wire_bridges[0].from, (1, 1));
        assert_eq!(plan.wire_bridges[0].to, (2, 0));
        assert_eq!(plan.wire_bridges[0].reason, WireBridgeReason::EndPlateBarrier);
    }
}
