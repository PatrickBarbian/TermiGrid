use std::collections::{BTreeMap, BTreeSet};
use std::f32::consts::FRAC_PI_2;

use egui::{
    Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2,
};

use crate::i18n::Strings;
use crate::model::bridging::BridgePlan;
use crate::model::{TerminalBlock, TerminalStrip, TierColor};

/// Controls how terminal block type names are displayed underneath the CAD strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeDisplayMode {
    /// Group consecutive identical blocks under horizontal type bars.
    HorizontalGroups,
    /// Render every block's type name vertically (rotated 90°) beneath its position number.
    #[default]
    VerticalPerBlock,
    /// Hide type labels below blocks.
    Off,
}

/// Active pointer/touch interaction tool inside the CAD editor canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasInteractionMode {
    /// Tap/Click selects blocks; Drag anywhere pans/scrolls (default on touch/Android).
    Pan,
    /// Left-drag draws marquee selection box; Middle/Right-drag pans (default on desktop).
    SelectBox,
    /// Tap/Click any tier connection point to assign/toggle it in the active jumper potential; Drag pans.
    BridgeAssign,
}

impl Default for CanvasInteractionMode {
    fn default() -> Self {
        if cfg!(target_os = "android") {
            Self::Pan
        } else {
            Self::SelectBox
        }
    }
}

/// Persistent interaction state for the 2D CAD terminal strip canvas.
#[derive(Debug, Default)]
pub struct CadCanvasState {
    pub zoom: f32,
    pub type_display: TypeDisplayMode,
    pub interaction_mode: CanvasInteractionMode,
    pub drag_start: Option<Pos2>,
    pub drag_current: Option<Pos2>,
    pub is_marquee_active: bool,
    pub selection_anchor: Option<usize>,
    pub pre_drag_selection: BTreeSet<usize>,
    /// Active inline label edit `(block_idx, tier_idx)` triggered by double-click.
    pub editing_tier: Option<(usize, usize)>,
    pub edit_buffer: String,
    pub edit_just_opened: bool,
}

impl CadCanvasState {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            type_display: TypeDisplayMode::VerticalPerBlock,
            interaction_mode: CanvasInteractionMode::default(),
            ..Default::default()
        }
    }
}

/// Renders the interactive 2D CAD view of `strip`, updating `selected` indices, handling
/// inline double-click label editing, panning, and optional jumper bridge overlay/assignment.
#[allow(clippy::too_many_arguments)]
pub fn show_cad_canvas(
    ui: &mut Ui,
    strip: &mut TerminalStrip,
    selected: &mut BTreeSet<usize>,
    state: &mut CadCanvasState,
    strings: &Strings,
    bridge_plan: Option<&BridgePlan>,
    active_potential_id: Option<u64>,
) {
    let dark = ui.visuals().dark_mode;

    // Clear stale inline editor if strip shrank
    if let Some((b_idx, t_idx)) = state.editing_tier
        && strip
            .blocks
            .get(b_idx)
            .and_then(|b| b.tiers.get(t_idx))
            .is_none()
    {
        state.editing_tier = None;
    }

    // Handle Ctrl+Scroll zoom when pointer is inside the canvas area.
    let ctrl_held = ui.input(|i| i.modifiers.ctrl || i.modifiers.mac_cmd);
    let shift_held = ui.input(|i| i.modifiers.shift);
    if ctrl_held {
        let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll_delta.abs() > 0.1 {
            let factor = if scroll_delta > 0.0 { 1.08 } else { 1.0 / 1.08 };
            state.zoom = (state.zoom * factor).clamp(0.5, 2.2);
        }
    }

    let z = state.zoom.clamp(0.5, 2.2);
    let px_per_mm = 8.2 * z;
    let tier_height = 56.0 * z;
    let tier_gap = 5.0 * z;
    let block_gap = 3.0 * z;
    let margin_x = 46.0 * z;
    let margin_top = 42.0 * z;
    let footer_height = match state.type_display {
        TypeDisplayMode::Off => 36.0 * z,
        TypeDisplayMode::HorizontalGroups => 74.0 * z,
        TypeDisplayMode::VerticalPerBlock => 155.0 * z,
    };

    let mut max_visual_row: isize = 0;
    let mut min_visual_row: isize = 0;
    for block in &strip.blocks {
        for level in 0..block.tiers.len() {
            let r = block.tier_visual_row(level);
            max_visual_row = max_visual_row.max(r);
            min_visual_row = min_visual_row.min(r);
        }
    }

    let above_rows = (max_visual_row + 1) as f32;
    let below_rows = if min_visual_row < 0 {
        (-min_visual_row) as f32
    } else {
        0.0
    };

    let total_blocks_width: f32 = strip
        .blocks
        .iter()
        .map(|b| (f32::from(b.width_tenths_mm) / 10.0) * px_per_mm + block_gap)
        .sum();

    let stack_height = (above_rows + below_rows) * (tier_height + tier_gap);
    let content_width = (margin_x * 2.0 + total_blocks_width).max(ui.available_width());
    let content_height =
        (margin_top + stack_height + footer_height + 24.0).max(ui.available_height());

    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let (canvas_rect, response) = ui.allocate_exact_size(
                Vec2::new(content_width, content_height),
                Sense::click_and_drag(),
            );
            let painter = ui.painter_at(canvas_rect);

            // 1. Canvas background & CAD drafting grid
            let bg_color = if dark {
                Color32::from_rgb(20, 24, 31)
            } else {
                Color32::from_rgb(245, 247, 250)
            };
            painter.rect_filled(canvas_rect, 0.0, bg_color);
            draw_cad_grid(&painter, canvas_rect, 24.0 * z, dark);

            // Strip title watermark at top-left
            let title_color = if dark {
                Color32::from_rgb(226, 232, 240)
            } else {
                Color32::from_rgb(30, 41, 59)
            };
            painter.text(
                canvas_rect.min + Vec2::new(14.0, 10.0),
                Align2::LEFT_TOP,
                format!(
                    "{}  |  {} {}  |  {:.1} mm",
                    strip.name,
                    strip.blocks.len(),
                    strings.blocks_unit,
                    strip.total_width_mm()
                ),
                FontId::proportional(15.0),
                title_color,
            );

            // Row 0 baseline bottom Y (all numbered lower tiers sit immediately above this line;
            // PE tiers drop below it into row -1).
            let baseline_bottom_y =
                canvas_rect.min.y + margin_top + above_rows * (tier_height + tier_gap);
            let lowest_bottom_y = baseline_bottom_y + below_rows * (tier_height + tier_gap);

            // Draw subtle row labels on the left margin (E3, E2, E1, PE)
            for row in min_visual_row..=max_visual_row {
                let row_bottom =
                    baseline_bottom_y - (row as f32) * (tier_height + tier_gap) - tier_gap;
                let row_center_y = row_bottom - tier_height * 0.5;
                let row_name = match row {
                    -1 => "PE".to_string(),
                    r if r >= 0 => format!("E{}", r + 1),
                    _ => String::new(),
                };
                let row_lbl_color = if row == -1 {
                    if dark {
                        Color32::from_rgb(74, 222, 128)
                    } else {
                        Color32::from_rgb(21, 128, 61)
                    }
                } else {
                    title_color
                };
                painter.text(
                    Pos2::new(canvas_rect.min.x + 12.0, row_center_y),
                    Align2::LEFT_CENTER,
                    row_name,
                    FontId::proportional((13.0 * z).max(10.5)),
                    row_lbl_color,
                );
            }

            // 2. Compute block & tier bounding boxes and render each terminal block
            let mut block_rects: Vec<Rect> = Vec::with_capacity(strip.blocks.len());
            let mut block_x_spans: Vec<(f32, f32)> = Vec::with_capacity(strip.blocks.len());
            let mut tier_hit_rects: Vec<(usize, usize, Rect, Rect)> = Vec::new();
            let mut cursor_x = canvas_rect.min.x + margin_x;

            for (idx, block) in strip.blocks.iter().enumerate() {
                let w = ((f32::from(block.width_tenths_mm) / 10.0) * px_per_mm).max(24.0 * z);

                let block_min_row = (0..block.tiers.len())
                    .map(|l| block.tier_visual_row(l))
                    .min()
                    .unwrap_or(0);
                let block_max_row = (0..block.tiers.len())
                    .map(|l| block.tier_visual_row(l))
                    .max()
                    .unwrap_or(0);

                let block_top =
                    baseline_bottom_y - ((block_max_row + 1) as f32) * (tier_height + tier_gap);
                let block_bottom =
                    baseline_bottom_y - (block_min_row as f32) * (tier_height + tier_gap);

                let block_rect = Rect::from_min_max(
                    Pos2::new(cursor_x, block_top),
                    Pos2::new(cursor_x + w, block_bottom.max(lowest_bottom_y) + 20.0 * z),
                );
                block_rects.push(block_rect);
                block_x_spans.push((cursor_x, cursor_x + w));

                let is_selected = selected.contains(&idx);
                draw_terminal_block(
                    &painter,
                    block,
                    idx,
                    cursor_x,
                    w,
                    baseline_bottom_y,
                    lowest_bottom_y,
                    tier_height,
                    tier_gap,
                    is_selected,
                    dark,
                    z,
                    state.editing_tier,
                    &mut tier_hit_rects,
                );

                cursor_x += w + block_gap;
            }

            // Draw terminal type annotations below the block numbers (#1, #2, ...)
            let type_row_top_y = lowest_bottom_y + 26.0 * z;
            match state.type_display {
                TypeDisplayMode::Off => {}
                TypeDisplayMode::HorizontalGroups => {
                    draw_horizontal_type_groups(
                        &painter,
                        strip,
                        &block_x_spans,
                        type_row_top_y,
                        dark,
                        z,
                    );
                }
                TypeDisplayMode::VerticalPerBlock => {
                    draw_vertical_type_labels(
                        &painter,
                        strip,
                        &block_x_spans,
                        type_row_top_y,
                        dark,
                        z,
                    );
                }
            }

            // 2b. Render jumper bridge overlay (comb jumpers, cut-out pins, dual-chamber tracks, wire bridges)
            if let Some(plan) = bridge_plan {
                draw_bridge_overlay(&painter, strip, plan, &tier_hit_rects, z, dark);
            }

            // 3. Inline text editor if user double-clicked a tier label
            let mut commit_edit: Option<(usize, usize, String)> = None;
            let mut cancel_edit = false;
            if let Some((edit_b, edit_t)) = state.editing_tier
                && let Some((_, _, tier_rect, tag_rect)) = tier_hit_rects
                    .iter()
                    .find(|(b, t, _, _)| *b == edit_b && *t == edit_t)
            {
                let edit_rect = Rect::from_center_size(
                    tag_rect.center(),
                    Vec2::new(
                        tag_rect.width().max(38.0 * z),
                        tag_rect.height().max(22.0 * z),
                    ),
                )
                .intersect(tier_rect.expand(4.0));

                let mut text_edit = egui::TextEdit::singleline(&mut state.edit_buffer)
                    .align(Align2::CENTER_CENTER)
                    .font(FontId::monospace((11.5 * z).max(9.0)));
                if state.edit_just_opened {
                    text_edit = text_edit.cursor_at_end(true);
                }

                let edit_resp = ui.put(edit_rect, text_edit);
                if state.edit_just_opened {
                    edit_resp.request_focus();
                    state.edit_just_opened = false;
                } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    cancel_edit = true;
                } else if ui.input(|i| i.key_pressed(egui::Key::Enter)) || edit_resp.lost_focus() {
                    commit_edit = Some((edit_b, edit_t, state.edit_buffer.clone()));
                }
            }

            if cancel_edit {
                state.editing_tier = None;
            } else if let Some((b_idx, t_idx, new_text)) = commit_edit {
                if let Some(tier) = strip.blocks.get_mut(b_idx).and_then(|b| b.tiers.get_mut(t_idx))
                {
                    let trimmed = new_text.trim();
                    tier.custom_override = if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed.to_string())
                    };
                    strip.recalculate_labels();
                }
                state.editing_tier = None;
            }

            // 4. Handle Pointer Interactions (Double-Click, Pan/Scroll, Bridge Assign, Click, Marquee)
            let space_pan = ui.input(|i| i.key_down(egui::Key::Space));
            if response.dragged_by(egui::PointerButton::Middle)
                || response.dragged_by(egui::PointerButton::Secondary)
                || (response.dragged_by(egui::PointerButton::Primary)
                    && (space_pan
                        || matches!(
                            state.interaction_mode,
                            CanvasInteractionMode::Pan | CanvasInteractionMode::BridgeAssign
                        )))
            {
                let delta = response.drag_delta();
                if delta.length_sq() > 0.0 {
                    ui.scroll_with_delta(delta);
                }
            }

            if state.interaction_mode != CanvasInteractionMode::BridgeAssign
                && response.double_clicked_by(egui::PointerButton::Primary)
                && let Some(pos) = response.interact_pointer_pos()
                && let Some((b_idx, t_idx, _, _)) = tier_hit_rects
                    .iter()
                    .find(|(_, _, tier_rect, _)| tier_rect.contains(pos))
                    .copied()
            {
                selected.clear();
                selected.insert(b_idx);
                state.selection_anchor = Some(b_idx);
                let tier = &strip.blocks[b_idx].tiers[t_idx];
                state.edit_buffer = tier
                    .custom_override
                    .clone()
                    .unwrap_or_else(|| tier.computed_label.clone());
                state.editing_tier = Some((b_idx, t_idx));
                state.edit_just_opened = true;
                state.is_marquee_active = false;
            }

            if state.editing_tier.is_none() {
                let use_marquee =
                    state.interaction_mode == CanvasInteractionMode::SelectBox && !space_pan;

                if use_marquee
                    && response.drag_started_by(egui::PointerButton::Primary)
                    && let Some(pos) = response.interact_pointer_pos()
                {
                    state.drag_start = Some(pos);
                    state.drag_current = Some(pos);
                    state.is_marquee_active = true;
                    state.pre_drag_selection = if ctrl_held {
                        selected.clone()
                    } else {
                        BTreeSet::new()
                    };
                }

                if use_marquee
                    && response.dragged_by(egui::PointerButton::Primary)
                    && state.is_marquee_active
                    && let Some(pos) = response.interact_pointer_pos()
                {
                    state.drag_current = Some(pos);
                    if let (Some(start), Some(curr)) = (state.drag_start, state.drag_current) {
                        let marquee = Rect::from_two_pos(start, curr);
                        let mut next_sel = state.pre_drag_selection.clone();
                        for (idx, brect) in block_rects.iter().enumerate() {
                            if brect.intersects(marquee) {
                                next_sel.insert(idx);
                            }
                        }
                        *selected = next_sel;
                    }
                }

                if response.drag_stopped_by(egui::PointerButton::Primary) {
                    state.is_marquee_active = false;
                    state.drag_start = None;
                    state.drag_current = None;
                    state.pre_drag_selection.clear();
                }

                if response.clicked_by(egui::PointerButton::Primary)
                    && let Some(pos) = response.interact_pointer_pos()
                {
                    if state.interaction_mode == CanvasInteractionMode::BridgeAssign {
                        if let Some((b_idx, t_idx, _, _)) = tier_hit_rects
                            .iter()
                            .find(|(_, _, tier_rect, _)| tier_rect.contains(pos))
                            .copied()
                            && let Some(pot_id) = active_potential_id
                        {
                            strip.toggle_potential_member(pot_id, b_idx, t_idx);
                        }
                    } else {
                        let hit = block_rects.iter().position(|r| r.contains(pos));
                        match hit {
                            Some(idx) => {
                                if shift_held {
                                    let anchor = state.selection_anchor.unwrap_or(idx);
                                    let (lo, hi) = if anchor <= idx {
                                        (anchor, idx)
                                    } else {
                                        (idx, anchor)
                                    };
                                    if !ctrl_held {
                                        selected.clear();
                                    }
                                    for i in lo..=hi {
                                        selected.insert(i);
                                    }
                                } else if ctrl_held {
                                    if !selected.remove(&idx) {
                                        selected.insert(idx);
                                    }
                                    state.selection_anchor = Some(idx);
                                } else {
                                    selected.clear();
                                    selected.insert(idx);
                                    state.selection_anchor = Some(idx);
                                }
                            }
                            None => {
                                if !ctrl_held && !shift_held {
                                    selected.clear();
                                    state.selection_anchor = None;
                                }
                            }
                        }
                    }
                }
            }

            // Draw marquee overlay if dragging
            if state.is_marquee_active
                && let (Some(start), Some(curr)) = (state.drag_start, state.drag_current)
            {
                let marquee = Rect::from_two_pos(start, curr);
                painter.rect_filled(
                    marquee,
                    2.0,
                    Color32::from_rgba_unmultiplied(56, 189, 248, 45),
                );
                painter.rect_stroke(
                    marquee,
                    2.0,
                    Stroke::new(1.5, Color32::from_rgb(56, 189, 248)),
                    StrokeKind::Inside,
                );
            }

            // 5. Hover tooltip with detailed block & tier info
            if !state.is_marquee_active
                && state.editing_tier.is_none()
                && let Some(hover_pos) = response.hover_pos()
                && let Some(hovered_idx) = block_rects.iter().position(|r| r.contains(hover_pos))
            {
                let block = &strip.blocks[hovered_idx];
                let mut lines = vec![format!(
                    "#{} — {} ({})",
                    hovered_idx + 1,
                    block.spec_name,
                    block.manufacturer
                )];
                lines.push(format!(
                    "{:.1} mm | {} {}",
                    f32::from(block.width_tenths_mm) / 10.0,
                    block.tiers.len(),
                    if block.tiers.len() == 1 {
                        "Ebene"
                    } else {
                        "Ebenen"
                    }
                ));
                for (t_idx, tier) in block.tiers.iter().enumerate().rev() {
                    let lbl = if tier.computed_label.is_empty() {
                        if tier.behavior.is_pe() {
                            "PE (unbeschriftet / unlabeled)".to_string()
                        } else {
                            "—".to_string()
                        }
                    } else if tier.custom_override.is_some() {
                        format!("{} *", tier.computed_label)
                    } else {
                        tier.computed_label.clone()
                    };
                    lines.push(format!("  E{}: {}", t_idx + 1, lbl));
                }
                response.on_hover_text_at_pointer(lines.join("\n"));
            }
        });
}

fn draw_horizontal_type_groups(
    painter: &egui::Painter,
    strip: &TerminalStrip,
    spans: &[(f32, f32)],
    top_y: f32,
    dark: bool,
    zoom: f32,
) {
    if strip.blocks.is_empty() || spans.len() != strip.blocks.len() {
        return;
    }

    let bar_fill = if dark {
        Color32::from_rgb(30, 41, 59)
    } else {
        Color32::from_rgb(226, 232, 240)
    };
    let bar_stroke = if dark {
        Color32::from_rgb(71, 85, 105)
    } else {
        Color32::from_rgb(148, 163, 184)
    };
    let text_color = if dark {
        Color32::from_rgb(226, 232, 240)
    } else {
        Color32::from_rgb(30, 41, 59)
    };

    let bar_h = (25.0 * zoom).max(20.0);
    let font = FontId::proportional((12.5 * zoom).max(10.5));

    let mut i = 0;
    while i < strip.blocks.len() {
        let spec_id = &strip.blocks[i].spec_id;
        let mut j = i + 1;
        while j < strip.blocks.len() && &strip.blocks[j].spec_id == spec_id {
            j += 1;
        }

        let start_x = spans[i].0;
        let end_x = spans[j - 1].1;
        let group_rect = Rect::from_min_max(
            Pos2::new(start_x, top_y),
            Pos2::new(end_x, top_y + bar_h),
        );

        let brand_accent = match strip.blocks[i].manufacturer.as_str() {
            "WAGO" => Color32::from_rgb(234, 88, 12),
            "Phoenix Contact" => Color32::from_rgb(0, 153, 153),
            "Weidmüller" => Color32::from_rgb(202, 138, 4),
            _ => Color32::from_rgb(100, 116, 139),
        };

        painter.rect_filled(group_rect, CornerRadius::same(3), bar_fill);
        painter.rect_stroke(
            group_rect,
            CornerRadius::same(3),
            Stroke::new(1.0, bar_stroke),
            StrokeKind::Inside,
        );
        // Subtle brand indicator line along top edge of group bar
        painter.line_segment(
            [
                Pos2::new(group_rect.min.x + 2.0, group_rect.min.y + 1.5),
                Pos2::new(group_rect.max.x - 2.0, group_rect.min.y + 1.5),
            ],
            Stroke::new(2.0, brand_accent),
        );

        let short = strip.blocks[i].short_type_name();
        let count = j - i;
        let full_label = if count > 1 {
            format!("{count}× {short}")
        } else {
            short.to_string()
        };

        // Clip text cleanly inside the group rect so isolated narrow blocks never overlap neighbors
        let text_painter = painter.with_clip_rect(group_rect.shrink2(Vec2::new(3.0, 0.0)));
        text_painter.text(
            group_rect.center() + Vec2::new(0.0, 1.0),
            Align2::CENTER_CENTER,
            full_label,
            font.clone(),
            text_color,
        );

        i = j;
    }
}

fn draw_vertical_type_labels(
    painter: &egui::Painter,
    strip: &TerminalStrip,
    spans: &[(f32, f32)],
    top_y: f32,
    dark: bool,
    zoom: f32,
) {
    let text_color = if dark {
        Color32::from_rgb(241, 245, 249)
    } else {
        Color32::from_rgb(15, 23, 42)
    };
    let font = FontId::proportional((13.0 * zoom).max(11.0));

    for (idx, block) in strip.blocks.iter().enumerate() {
        let Some(&(x0, x1)) = spans.get(idx) else {
            continue;
        };
        let center_x = (x0 + x1) * 0.5;
        let galley = painter.layout_no_wrap(
            block.short_type_name().to_string(),
            font.clone(),
            text_color,
        );
        let text_w = galley.size().x;
        let text_h = galley.size().y;
        // Rotate -90° (counter-clockwise) so text reads bottom-to-top (standard DIN/CAD orientation)
        let pos = Pos2::new(center_x - text_h * 0.5, top_y + text_w);
        let shape = egui::epaint::TextShape::new(pos, galley, text_color).with_angle(-FRAC_PI_2);
        painter.add(shape);
    }
}

fn draw_cad_grid(painter: &egui::Painter, rect: Rect, step: f32, dark: bool) {
    let minor_color = if dark {
        Color32::from_rgba_unmultiplied(255, 255, 255, 10)
    } else {
        Color32::from_rgba_unmultiplied(15, 23, 42, 12)
    };
    let major_color = if dark {
        Color32::from_rgba_unmultiplied(255, 255, 255, 22)
    } else {
        Color32::from_rgba_unmultiplied(15, 23, 42, 24)
    };

    let step = step.max(12.0);
    let mut x = rect.min.x;
    let mut i = 0;
    while x <= rect.max.x {
        let color = if i % 5 == 0 { major_color } else { minor_color };
        painter.line_segment(
            [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
            Stroke::new(1.0, color),
        );
        x += step;
        i += 1;
    }

    let mut y = rect.min.y;
    let mut j = 0;
    while y <= rect.max.y {
        let color = if j % 5 == 0 { major_color } else { minor_color };
        painter.line_segment(
            [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
            Stroke::new(1.0, color),
        );
        y += step;
        j += 1;
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_terminal_block(
    painter: &egui::Painter,
    block: &TerminalBlock,
    block_idx: usize,
    x: f32,
    width: f32,
    baseline_bottom_y: f32,
    lowest_bottom_y: f32,
    tier_height: f32,
    tier_gap: f32,
    is_selected: bool,
    dark: bool,
    zoom: f32,
    editing_tier: Option<(usize, usize)>,
    tier_hit_rects: &mut Vec<(usize, usize, Rect, Rect)>,
) {
    let min_row = (0..block.tiers.len())
        .map(|l| block.tier_visual_row(l))
        .min()
        .unwrap_or(0);
    let max_row = (0..block.tiers.len())
        .map(|l| block.tier_visual_row(l))
        .max()
        .unwrap_or(0);

    let top_y = baseline_bottom_y - ((max_row + 1) as f32) * (tier_height + tier_gap);
    let bottom_y = baseline_bottom_y - (min_row as f32) * (tier_height + tier_gap);

    let outer_rect = Rect::from_min_max(Pos2::new(x, top_y), Pos2::new(x + width, bottom_y));

    // Subtle selection backdrop behind entire column
    if is_selected {
        painter.rect_filled(
            outer_rect.expand(2.5),
            CornerRadius::same(4),
            Color32::from_rgba_unmultiplied(14, 165, 233, 40),
        );
    }

    // Manufacturer indicator bar at the top-most tier of the block
    let brand_color = match block.manufacturer.as_str() {
        "WAGO" => Color32::from_rgb(234, 88, 12),
        "Phoenix Contact" => Color32::from_rgb(0, 153, 153),
        "Weidmüller" => Color32::from_rgb(202, 138, 4),
        _ => Color32::from_rgb(100, 116, 139),
    };

    for (level, tier) in block.tiers.iter().enumerate() {
        let vrow = block.tier_visual_row(level);
        let tier_bottom = baseline_bottom_y - (vrow as f32) * (tier_height + tier_gap) - tier_gap;
        let tier_top = tier_bottom - tier_height;
        let inset = (vrow.max(0) as f32) * 1.2 * zoom;

        let tier_rect = Rect::from_min_max(
            Pos2::new(x + inset, tier_top),
            Pos2::new(x + width - inset, tier_bottom),
        );

        let (body_fill, text_color, badge_fill) = tier_palette(tier.color, dark);
        painter.rect_filled(tier_rect, CornerRadius::same(3), body_fill);

        // Manufacturer accent strip on top-most tier
        if level + 1 == block.tiers.len() {
            let strip_rect = Rect::from_min_size(
                tier_rect.min,
                Vec2::new(tier_rect.width(), (4.0 * zoom).max(2.5)),
            );
            painter.rect_filled(strip_rect, CornerRadius::same(2), brand_color);
        }

        // PE yellow accent bar at bottom
        if tier.behavior.is_pe() {
            let pe_bar = Rect::from_min_size(
                Pos2::new(tier_rect.min.x, tier_rect.max.y - (5.0 * zoom).max(3.0)),
                Vec2::new(tier_rect.width(), (5.0 * zoom).max(3.0)),
            );
            painter.rect_filled(pe_bar, CornerRadius::same(1), Color32::from_rgb(250, 204, 21));
        }

        // Inner marker tag box placed cleanly below the header/corner-badge zone
        let tag_rect = Rect::from_min_max(
            Pos2::new(tier_rect.min.x + 3.5 * zoom, tier_rect.min.y + 15.5 * zoom),
            Pos2::new(tier_rect.max.x - 3.5 * zoom, tier_rect.max.y - 6.5 * zoom),
        );
        tier_hit_rects.push((block_idx, level, tier_rect, tag_rect));

        let is_being_edited = editing_tier == Some((block_idx, level));
        if !is_being_edited && tag_rect.width() > 8.0 && tag_rect.height() > 8.0 {
            painter.rect_filled(tag_rect, CornerRadius::same(2), badge_fill);
            let display = if tier.cad_display_label.is_empty() {
                "—"
            } else {
                tier.cad_display_label.as_str()
            };
            let font_size = if display.len() >= 4 {
                (11.0 * zoom).max(9.0)
            } else {
                (14.5 * zoom).max(11.0)
            };
            painter.text(
                tag_rect.center(),
                Align2::CENTER_CENTER,
                display,
                FontId::monospace(font_size),
                text_color,
            );

            // Subtle amber dot indicator if manually overridden
            if tier.custom_override.is_some() {
                painter.circle_filled(
                    Pos2::new(tag_rect.min.x + 3.5 * zoom, tag_rect.min.y + 3.5 * zoom),
                    2.5 * zoom,
                    Color32::from_rgb(245, 158, 11),
                );
            }
        }

        // High-contrast tier level pill (E1, E2, E3, PE) in dedicated top-right header strip
        if zoom >= 0.65 {
            let row_tag = if tier.behavior.is_pe() && vrow < 0 {
                "PE".to_string()
            } else {
                format!("E{}", (vrow.max(0) + 1))
            };
            let (pill_bg, pill_fg) = if tier.behavior.is_pe() {
                (
                    Color32::from_rgba_unmultiplied(20, 83, 45, 235),
                    Color32::from_rgb(254, 240, 138),
                )
            } else {
                (
                    Color32::from_rgba_unmultiplied(15, 23, 42, 225),
                    Color32::from_rgb(248, 250, 252),
                )
            };
            let pill_w = (row_tag.len() as f32 * 6.5 + 6.0) * zoom;
            let pill_h = 11.5 * zoom;
            let pill_rect = Rect::from_min_size(
                Pos2::new(
                    tier_rect.max.x - pill_w - 2.0 * zoom,
                    tier_rect.min.y + 3.5 * zoom,
                ),
                Vec2::new(pill_w, pill_h),
            );
            painter.rect_filled(pill_rect, CornerRadius::same(2), pill_bg);
            painter.text(
                pill_rect.center(),
                Align2::CENTER_CENTER,
                row_tag,
                FontId::proportional((9.0 * zoom).max(8.0)),
                pill_fg,
            );
        }

        let border_stroke = if is_selected {
            Stroke::new(2.0, Color32::from_rgb(14, 165, 233))
        } else {
            Stroke::new(
                1.0,
                if dark {
                    Color32::from_rgb(71, 85, 105)
                } else {
                    Color32::from_rgb(148, 163, 184)
                },
            )
        };
        painter.rect_stroke(
            tier_rect,
            CornerRadius::same(3),
            border_stroke,
            StrokeKind::Inside,
        );
    }

    // Block position index aligned along the bottom of the strip
    let pos_color = if is_selected {
        if dark {
            Color32::from_rgb(56, 189, 248)
        } else {
            Color32::from_rgb(2, 132, 199)
        }
    } else if dark {
        Color32::from_rgb(203, 213, 225)
    } else {
        Color32::from_rgb(30, 41, 59)
    };
    painter.text(
        Pos2::new(x + width * 0.5, lowest_bottom_y + 6.0 * zoom),
        Align2::CENTER_TOP,
        format!("#{}", block_idx + 1),
        FontId::proportional((12.5 * zoom).max(10.0)),
        pos_color,
    );
}

fn tier_palette(color: TierColor, dark: bool) -> (Color32, Color32, Color32) {
    match color {
        TierColor::StandardGray => {
            if dark {
                (
                    Color32::from_rgb(51, 65, 85),
                    Color32::from_rgb(15, 23, 42),
                    Color32::from_rgb(241, 245, 249),
                )
            } else {
                (
                    Color32::from_rgb(203, 213, 225),
                    Color32::from_rgb(15, 23, 42),
                    Color32::from_rgb(255, 255, 255),
                )
            }
        }
        TierColor::BlueNeutral => (
            Color32::from_rgb(37, 99, 235),
            Color32::from_rgb(30, 58, 138),
            Color32::from_rgb(239, 246, 255),
        ),
        TierColor::GreenYellowPe => (
            Color32::from_rgb(22, 163, 74),
            Color32::from_rgb(20, 83, 45),
            Color32::from_rgb(254, 249, 195),
        ),
        TierColor::RedPhase => (
            Color32::from_rgb(220, 38, 38),
            Color32::from_rgb(127, 29, 29),
            Color32::from_rgb(254, 242, 242),
        ),
        TierColor::OrangeAux => (
            Color32::from_rgb(234, 88, 12),
            Color32::from_rgb(124, 45, 18),
            Color32::from_rgb(255, 247, 237),
        ),
    }
}

fn draw_bridge_overlay(
    painter: &egui::Painter,
    strip: &TerminalStrip,
    plan: &BridgePlan,
    tier_hit_rects: &[(usize, usize, Rect, Rect)],
    zoom: f32,
    dark: bool,
) {
    let mut tier_rects: BTreeMap<(usize, usize), Rect> = BTreeMap::new();
    let mut row_block_rects: BTreeMap<(usize, isize), Rect> = BTreeMap::new();

    for &(b_idx, l_idx, rect, _) in tier_hit_rects {
        tier_rects.insert((b_idx, l_idx), rect);
        if let Some(block) = strip.blocks.get(b_idx) {
            row_block_rects.insert((b_idx, block.tier_visual_row(l_idx)), rect);
        }
    }

    // 1. Assigned potential indicator pills on each member tier
    for (&(b_idx, l_idx), &(_, ref pot_name, rgb)) in &plan.point_assignments {
        if let Some(t_rect) = tier_rects.get(&(b_idx, l_idx)) {
            let pot_color = Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
            let dot_center = Pos2::new(t_rect.min.x + 6.0 * zoom, t_rect.max.y - 6.0 * zoom);
            painter.circle_filled(dot_center, 3.8 * zoom, pot_color);
            painter.circle_stroke(dot_center, 3.8 * zoom, Stroke::new(1.0, Color32::WHITE));
            if zoom >= 0.85 {
                painter.text(
                    Pos2::new(dot_center.x + 5.5 * zoom, dot_center.y),
                    Align2::LEFT_CENTER,
                    pot_name,
                    FontId::proportional((8.5 * zoom).max(7.0)),
                    if dark { Color32::WHITE } else { Color32::BLACK },
                );
            }
        }
    }

    // 2. Comb jumper segments (`Kammbrücken`), slot-1 vs slot-2 offset, and cut-out pins (`X`)
    for seg in &plan.comb_segments {
        let Some(start_r) = row_block_rects.get(&(seg.start_block_idx, seg.visual_row)) else {
            continue;
        };
        let Some(end_r) = row_block_rects.get(&(seg.end_block_idx, seg.visual_row)) else {
            continue;
        };

        let pot_color = Color32::from_rgb(seg.color_rgb[0], seg.color_rgb[1], seg.color_rgb[2]);
        let y_off = if seg.channel_slot == 0 {
            6.5 * zoom
        } else {
            13.5 * zoom
        };
        let bar_y = start_r.min.y + y_off;
        let x0 = start_r.center().x;
        let x1 = end_r.center().x;

        // Insulated comb jumper bar housing along spanned blocks
        let bar_rect = Rect::from_min_max(
            Pos2::new(x0 - 4.0 * zoom, bar_y - 3.0 * zoom),
            Pos2::new(x1 + 4.0 * zoom, bar_y + 3.0 * zoom),
        );
        painter.rect_filled(bar_rect, CornerRadius::same(2), pot_color);
        painter.rect_stroke(
            bar_rect,
            CornerRadius::same(2),
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 190)),
            StrokeKind::Inside,
        );

        // Metallic pin legs descending into actively connected blocks
        for &pin_b in &seg.pin_blocks {
            if let Some(pr) = row_block_rects.get(&(pin_b, seg.visual_row)) {
                let cx = pr.center().x;
                let prong = Rect::from_center_size(
                    Pos2::new(cx, bar_y + 5.5 * zoom),
                    Vec2::new(4.0 * zoom, 7.0 * zoom),
                );
                painter.rect_filled(prong, CornerRadius::same(1), Color32::from_rgb(226, 232, 240));
                painter.rect_stroke(
                    prong,
                    CornerRadius::same(1),
                    Stroke::new(0.8, Color32::from_rgb(15, 23, 42)),
                    StrokeKind::Inside,
                );
            }
        }

        // Visually highlight removed / cut-out pins (`Pin entfernen`) with clear gap + red X
        for &cut_b in &seg.cut_pins {
            if let Some(cr) = row_block_rects.get(&(cut_b, seg.visual_row)) {
                let cx = cr.center().x;
                let r = 6.0 * zoom;
                painter.circle_filled(
                    Pos2::new(cx, bar_y + 1.5 * zoom),
                    r,
                    Color32::from_rgb(220, 38, 38),
                );
                painter.circle_stroke(
                    Pos2::new(cx, bar_y + 1.5 * zoom),
                    r,
                    Stroke::new(1.2, Color32::WHITE),
                );
                painter.line_segment(
                    [
                        Pos2::new(cx - r * 0.5, bar_y + 1.5 * zoom - r * 0.5),
                        Pos2::new(cx + r * 0.5, bar_y + 1.5 * zoom + r * 0.5),
                    ],
                    Stroke::new(1.8, Color32::WHITE),
                );
                painter.line_segment(
                    [
                        Pos2::new(cx - r * 0.5, bar_y + 1.5 * zoom + r * 0.5),
                        Pos2::new(cx + r * 0.5, bar_y + 1.5 * zoom - r * 0.5),
                    ],
                    Stroke::new(1.8, Color32::WHITE),
                );
            }
        }

        if zoom >= 0.85 {
            painter.text(
                Pos2::new((x0 + x1) * 0.5, bar_y),
                Align2::CENTER_CENTER,
                format!("{}p", seg.pole_count),
                FontId::proportional((8.0 * zoom).max(7.0)),
                Color32::WHITE,
            );
        }
    }

    // 3. Wire bridges (`Drahtbrücken`) for cross-tier, single-chamber overflow, or pitch jumps
    for (wb_idx, wb) in plan.wire_bridges.iter().enumerate() {
        let Some(r_from) = tier_rects.get(&wb.from) else {
            continue;
        };
        let Some(r_to) = tier_rects.get(&wb.to) else {
            continue;
        };

        let pot_color = Color32::from_rgb(wb.color_rgb[0], wb.color_rgb[1], wb.color_rgb[2]);
        let p0 = Pos2::new(r_from.center().x + 5.0 * zoom, r_from.center().y - 8.0 * zoom);
        let p1 = Pos2::new(r_to.center().x - 5.0 * zoom, r_to.center().y - 8.0 * zoom);
        let arch = if wb.from.0 == wb.to.0 {
            Pos2::new(r_from.max.x + 9.0 * zoom, (p0.y + p1.y) * 0.5)
        } else {
            let lane = (wb_idx % 3) as f32 * 8.0 * zoom;
            Pos2::new((p0.x + p1.x) * 0.5, p0.y.min(p1.y) - 18.0 * zoom - lane)
        };

        // Two-segment polyline with clear DB badge
        let stroke = Stroke::new((2.2 * zoom).max(1.5), pot_color);
        painter.line_segment([p0, arch], stroke);
        painter.line_segment([arch, p1], stroke);
        painter.circle_filled(p0, 3.0 * zoom, pot_color);
        painter.circle_filled(p1, 3.0 * zoom, pot_color);

        let badge_rect = Rect::from_center_size(arch, Vec2::new(18.0 * zoom, 12.0 * zoom));
        painter.rect_filled(badge_rect, CornerRadius::same(3), pot_color);
        painter.text(
            arch,
            Align2::CENTER_CENTER,
            "DB",
            FontId::proportional((8.0 * zoom).max(7.0)),
            Color32::WHITE,
        );
    }
}
