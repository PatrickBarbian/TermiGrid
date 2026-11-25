use std::collections::{BTreeMap, BTreeSet};
use std::f32::consts::FRAC_PI_2;

use egui::{
    Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2,
};

use crate::catalog::accessories::{find_end_bracket, EndBracketSpec, EndPlatePlacement};
use crate::catalog::TerminalCatalog;
use crate::i18n::{tr, Language, Strings};
use crate::model::bridging::BridgePlan;
use crate::model::{TerminalBlock, TerminalStrip, TierColor};
use crate::ui::icons::{draw_icon, UiIcon};

/// Actions emitted by interactive widgets painted directly inside the CAD canvas.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CadCanvasResult {
    pub quick_add_after_block: Option<usize>,
}

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
    /// When true, renders a strict 1:1 millimeter front-elevation ("e-Planer Aufbaubild")
    /// with metric dimension lines, TS 35 rail callout, and EPLAN title block.
    pub e_planer_mode: bool,
    pub drag_start: Option<Pos2>,
    pub drag_current: Option<Pos2>,
    pub is_marquee_active: bool,
    pub selection_anchor: Option<usize>,
    pub pre_drag_selection: BTreeSet<usize>,
    /// Tracks middle-mouse / touch drag panning smoothly across frames.
    pub middle_panning: bool,
    pub pan_last_pos: Option<Pos2>,
    /// Accumulates finger/pointer movement during Pan mode so short jittery taps still select.
    pub pan_touch_total_dist: f32,
    /// Active left-drag mass-bridging state (`Some(true)` = adding, `Some(false)` = removing).
    pub bridge_drag_action: Option<bool>,
    pub bridge_painted: BTreeSet<(usize, usize)>,
    pub bridge_last_pos: Option<Pos2>,
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

fn block_front_bounds_mm(block: &TerminalBlock, catalog: &TerminalCatalog) -> (f32, f32, f32) {
    let (h, offset) = if let Some(sp) = catalog.find_by_id(&block.spec_id) {
        (
            f32::from(sp.effective_height_tenths_mm()) / 10.0,
            f32::from(sp.effective_rail_offset_tenths_mm()) / 10.0,
        )
    } else {
        let h = match block.tiers.len() {
            1 => 48.5,
            2 => 69.7,
            _ => 93.3,
        };
        (h, 0.0)
    };
    let above = (h * 0.5 + offset).max(5.0);
    let below = (h * 0.5 - offset).max(5.0);
    (h, above, below)
}

fn pos_label_and_font(block_w: f32, index: usize, base_sz: f32) -> (String, f32) {
    let label = if block_w >= 26.0 {
        format!("#{}", index + 1)
    } else {
        format!("{}", index + 1)
    };
    let max_for_w = (block_w - 2.0).max(4.0) / (label.len().max(1) as f32 * 0.62);
    let font_sz = base_sz.min(max_for_w).clamp(5.5, 13.5);
    (label, font_sz)
}

/// Renders the interactive 2D CAD view of `strip`, updating `selected` indices and
/// `selected_end_bracket`, handling inline double-click label editing, panning, and
/// optional jumper bridge overlay/assignment.
#[allow(clippy::too_many_arguments)]
pub fn show_cad_canvas(
    ui: &mut Ui,
    strip: &mut TerminalStrip,
    catalog: &TerminalCatalog,
    selected: &mut BTreeSet<usize>,
    selected_end_bracket: &mut Option<bool>,
    state: &mut CadCanvasState,
    strings: &Strings,
    bridge_plan: Option<&BridgePlan>,
    active_potential_id: Option<u64>,
    end_plates: &[EndPlatePlacement],
) -> CadCanvasResult {
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

    // Handle Ctrl+Scroll zoom and middle-mouse pan gating when pointer is inside the canvas area.
    let canvas_avail = ui.available_rect_before_wrap();
    let pointer_in_canvas = ui.input(|i| {
        i.pointer
            .hover_pos()
            .or_else(|| i.pointer.interact_pos())
            .is_some_and(|p| canvas_avail.contains(p))
    });
    let ctrl_held = ui.input(|i| i.modifiers.ctrl || i.modifiers.mac_cmd);
    let shift_held = ui.input(|i| i.modifiers.shift);
    if pointer_in_canvas {
        let zoom_mul = ui.input(|i| {
            let zd = i.zoom_delta();
            if (zd - 1.0).abs() > 0.001 {
                zd
            } else if ctrl_held {
                let dy = i.smooth_scroll_delta.y;
                if dy > 0.05 {
                    1.09
                } else if dy < -0.05 {
                    1.0 / 1.09
                } else {
                    1.0
                }
            } else {
                1.0
            }
        });
        if (zoom_mul - 1.0).abs() > 0.001 {
            state.zoom = (state.zoom * zoom_mul).clamp(0.4, 2.5);
            // Consume scroll delta so ScrollArea does not jump vertically while zooming.
            ui.ctx().input_mut(|i| {
                i.smooth_scroll_delta = Vec2::ZERO;
            });
        }
    }

    // Track middle-mouse drag panning across the CAD canvas smoothly.
    let (mid_pressed, mid_down, ptr_pos) = ui.input(|i| {
        (
            i.pointer.button_pressed(egui::PointerButton::Middle),
            i.pointer.button_down(egui::PointerButton::Middle),
            i.pointer.latest_pos().or_else(|| i.pointer.hover_pos()),
        )
    });
    if mid_pressed && pointer_in_canvas {
        state.middle_panning = true;
        state.pan_last_pos = ptr_pos;
    }
    if !mid_down {
        state.middle_panning = false;
        if state.interaction_mode != CanvasInteractionMode::Pan {
            state.pan_last_pos = None;
        }
    }

    let z = state.zoom.clamp(0.4, 2.5);
    let end_bracket = find_end_bracket(&strip.end_bracket_id);
    let bracket_count = strip.end_bracket_count();
    let brackets_mm = strip.end_brackets_width_mm();
    let plates_mm: f32 = end_plates
        .iter()
        .map(|p| f32::from(p.width_tenths_mm) / 10.0)
        .sum();
    let total_rail_mm = strip.total_width_mm() + plates_mm + brackets_mm;

    let block_bounds_mm: Vec<(f32, f32, f32)> = strip
        .blocks
        .iter()
        .map(|b| block_front_bounds_mm(b, catalog))
        .collect();
    let eb_height_mm = f32::from(end_bracket.height_tenths_mm) / 10.0;
    let eb_half_mm = eb_height_mm * 0.5;
    let mut max_above_mm = block_bounds_mm
        .iter()
        .map(|&(_, above, _)| above)
        .fold(17.5_f32, f32::max);
    let mut max_below_mm = block_bounds_mm
        .iter()
        .map(|&(_, _, below)| below)
        .fold(17.5_f32, f32::max);
    if !strip.blocks.is_empty() && bracket_count > 0 {
        max_above_mm = max_above_mm.max(eb_half_mm);
        max_below_mm = max_below_mm.max(eb_half_mm);
    }
    let max_strip_h_mm = if strip.blocks.is_empty() {
        35.0
    } else {
        (max_above_mm + max_below_mm).max(35.0)
    };

    // Layout metrics depend on standard CAD mode vs strict 1:1 mm e-Planer Aufbaubild mode
    let e_planer = state.e_planer_mode;
    let px_per_mm = if e_planer { 6.2 * z } else { 8.2 * z };
    let h_px_per_mm = if e_planer { px_per_mm } else { 3.25 * z };

    let tier_height = 48.0 * z;
    let tier_gap = 28.0 * z;
    let block_gap = if e_planer { 0.0 } else { 3.0 * z };
    let margin_x = if e_planer {
        (92.0 * z).max(74.0)
    } else {
        (50.0 * z).max(42.0)
    };

    // Prepare wrapped canvas header before layout calculation so margin_top never collides
    let title_color = if dark {
        Color32::from_rgb(226, 232, 240)
    } else {
        Color32::from_rgb(30, 41, 59)
    };
    let mut detail_parts = Vec::new();
    if !end_plates.is_empty() || bracket_count > 0 {
        detail_parts.push(format!("{:.1}", strip.total_width_mm()));
        if !end_plates.is_empty() {
            detail_parts.push(format!(
                "{:.1} mm {}× {}",
                plates_mm,
                end_plates.len(),
                strings.acc_end_plate
            ));
        }
        if bracket_count > 0 {
            detail_parts.push(format!(
                "{:.1} mm {}× {}",
                brackets_mm, bracket_count, end_bracket.name
            ));
        }
    }
    let scale_badge = if e_planer {
        format!("  |  {}", strings.cad_scale_1_1)
    } else {
        String::new()
    };
    let header_text = if detail_parts.is_empty() {
        format!(
            "{}  |  {} {}  |  {:.1} × {:.1} mm{}",
            strip.name,
            strip.blocks.len(),
            strings.blocks_unit,
            total_rail_mm,
            max_strip_h_mm,
            scale_badge
        )
    } else {
        format!(
            "{}  |  {} {}  |  {:.1} × {:.1} mm ({}){}",
            strip.name,
            strip.blocks.len(),
            strings.blocks_unit,
            total_rail_mm,
            max_strip_h_mm,
            detail_parts.join(" + "),
            scale_badge
        )
    };
    let header_wrap_w = (canvas_avail.width() - 28.0).max(260.0);
    let header_galley = ui.painter().layout(
        header_text,
        FontId::proportional(14.5),
        title_color,
        header_wrap_w,
    );
    let header_h = header_galley.size().y;

    let margin_top = if e_planer {
        let dim_clearance = (18.0 * z).max(16.0)
            + (11.5 * z).clamp(9.5, 14.0) * 1.45
            + (7.0 * z).max(7.0)
            + (10.5 * z).clamp(8.5, 13.0) * 1.45
            + 16.0;
        12.0 + header_h + dim_clearance
    } else {
        12.0 + header_h + 16.0 * z + 10.0
    };

    let max_short_chars = strip
        .blocks
        .iter()
        .map(|b| b.short_type_name().chars().count())
        .max()
        .unwrap_or(10);
    let est_pos_band =
        ((if e_planer { 11.0 } else { 12.5 }) * z).clamp(7.5, 13.5) * 1.45 + (14.0 * z).max(12.0);
    let footer_height = match state.type_display {
        TypeDisplayMode::Off => est_pos_band + 18.0,
        TypeDisplayMode::HorizontalGroups => est_pos_band + (25.0 * z).max(20.0) + 22.0,
        TypeDisplayMode::VerticalPerBlock => {
            let est_type_h = (max_short_chars as f32 * (12.5 * z).clamp(7.5, 13.5) * 0.66)
                .max(120.0 * z);
            est_pos_band + est_type_h + 28.0
        }
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

    let bracket_px = if e_planer {
        (f32::from(end_bracket.width_tenths_mm) / 10.0) * px_per_mm
    } else {
        ((f32::from(end_bracket.width_tenths_mm) / 10.0) * px_per_mm).max(20.0 * z)
    };
    let total_brackets_width = if bracket_count > 0 && !strip.blocks.is_empty() {
        (bracket_px + block_gap) * (bracket_count as f32)
    } else {
        0.0
    };

    let total_blocks_width: f32 = strip
        .blocks
        .iter()
        .map(|b| {
            if e_planer {
                (f32::from(b.width_tenths_mm) / 10.0) * px_per_mm
            } else {
                (f32::from(b.width_tenths_mm) / 10.0) * px_per_mm + block_gap
            }
        })
        .sum();
    let total_plates_width: f32 = end_plates
        .iter()
        .map(|p| {
            if e_planer {
                (f32::from(p.width_tenths_mm) / 10.0) * px_per_mm
            } else {
                ((f32::from(p.width_tenths_mm) / 10.0) * px_per_mm).max(5.5 * z) + block_gap
            }
        })
        .sum();

    let tier_span_rows = (max_visual_row - min_visual_row + 1).max(1) as f32;
    let tier_half_from_rail_px =
        (tier_span_rows * (tier_height + tier_gap) - tier_gap).max(tier_height) * 0.5;
    let max_above_px = max_above_mm * h_px_per_mm;
    let max_below_px = max_below_mm * h_px_per_mm;
    let req_above_px = if e_planer {
        max_above_px
    } else if strip.blocks.is_empty() {
        60.0 * z
    } else {
        max_above_px.max(tier_half_from_rail_px + 6.0 * z)
    };
    let req_below_px = if e_planer {
        max_below_px
    } else if strip.blocks.is_empty() {
        60.0 * z
    } else {
        max_below_px.max(tier_half_from_rail_px + 6.0 * z)
    };
    let stack_height = req_above_px + req_below_px;

    let right_extra = if e_planer { 116.0 * z } else { 0.0 };
    let content_width = (margin_x * 2.0
        + total_blocks_width
        + total_plates_width
        + total_brackets_width
        + right_extra)
        .max(ui.available_width());
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
            let t = crate::ui::theme::ThemeTokens::for_mode(dark);
            let bg_color = if e_planer {
                if dark {
                    Color32::from_rgb(13, 18, 29)
                } else {
                    Color32::from_rgb(252, 253, 255)
                }
            } else {
                t.bg_canvas
            };
            painter.rect_filled(canvas_rect, 0.0, bg_color);
            let grid_step = if e_planer { 10.0 * px_per_mm } else { 24.0 * z };
            draw_cad_grid(&painter, canvas_rect, grid_step, dark);

            // Wrapped strip header pill at top-left
            let hdr_pos = canvas_rect.min + Vec2::new(14.0, 10.0);
            let hdr_pill = Rect::from_min_size(
                hdr_pos - Vec2::new(8.0, 4.0),
                header_galley.size() + Vec2::new(16.0, 8.0),
            );
            painter.rect_filled(
                hdr_pill,
                CornerRadius::same(7),
                t.bg_panel.gamma_multiply(0.92),
            );
            painter.rect_stroke(
                hdr_pill,
                CornerRadius::same(7),
                Stroke::new(1.0, t.border_hairline),
                StrokeKind::Inside,
            );
            painter.galley(hdr_pos, header_galley, title_color);

            // Exact TS 35 DIN-rail center anchor (`rail_center_y`) and visual row baseline
            let rail_center_y = canvas_rect.min.y + margin_top + req_above_px;
            let baseline_bottom_y = rail_center_y
                + (((max_visual_row + min_visual_row + 1) as f32) * (tier_height + tier_gap)
                    + tier_gap)
                    * 0.5;
            let lowest_bottom_y = rail_center_y + req_below_px;

            // Draw subtle row labels on the left margin (E3, E2, E1, PE) in normal CAD mode
            if !e_planer {
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
            }

            // Compute exact horizontal start/end of the assembly to size the TS 35 DIN rail
            let strip_start_x = canvas_rect.min.x + margin_x;
            let assembly_content_w = if strip.blocks.is_empty() {
                120.0 * z
            } else {
                let mut sum_w = total_blocks_width + total_plates_width + total_brackets_width;
                if !e_planer && sum_w > block_gap {
                    sum_w -= block_gap;
                }
                sum_w
            };
            let rail_overhang = if e_planer {
                15.0 * px_per_mm
            } else {
                (34.0 * z).max(28.0)
            };
            draw_ts35_din_rail(
                &painter,
                strip_start_x - rail_overhang,
                strip_start_x + assembly_content_w + rail_overhang,
                rail_center_y,
                h_px_per_mm,
                dark,
                e_planer,
                z,
                rail_overhang,
            );

            // 2. Compute block & tier bounding boxes and render end brackets, terminal blocks & end plates
            let mut block_rects: Vec<Rect> = Vec::with_capacity(strip.blocks.len());
            let mut block_x_spans: Vec<(f32, f32)> = Vec::with_capacity(strip.blocks.len());
            let mut tier_hit_rects: Vec<(usize, usize, Rect, Rect)> = Vec::new();
            let mut plate_hit_rects: Vec<(Rect, &EndPlatePlacement)> = Vec::new();
            let mut bracket_hit_rects: Vec<(Rect, bool)> = Vec::new();
            let mut cursor_x = strip_start_x;

            let pos_top_y = lowest_bottom_y + (5.0 * z).max(4.0);
            let mut max_pos_bottom_y = pos_top_y + 10.0 * z;

            let eb_half_h = (eb_half_mm * h_px_per_mm).max(if e_planer { 0.0 } else { 18.0 * z });
            let eb_top = rail_center_y - eb_half_h;
            let eb_bottom = rail_center_y + eb_half_h;

            if !strip.blocks.is_empty() && strip.end_bracket_left {
                let eb_rect = Rect::from_min_max(
                    Pos2::new(cursor_x, eb_top),
                    Pos2::new(cursor_x + bracket_px, eb_bottom),
                );
                draw_end_bracket_cad(
                    &painter,
                    eb_rect,
                    pos_top_y,
                    &mut max_pos_bottom_y,
                    end_bracket,
                    *selected_end_bracket == Some(true),
                    dark,
                    z,
                );
                bracket_hit_rects.push((eb_rect, true));
                cursor_x += bracket_px + block_gap;
            }

            let quick_add_anchor_idx = state
                .selection_anchor
                .filter(|i| selected.contains(i))
                .or_else(|| selected.iter().next_back().copied());
            let mut anchor_housing_top_left: Option<(usize, Pos2)> = None;

            for (idx, block) in strip.blocks.iter().enumerate() {
                let w = if e_planer {
                    (f32::from(block.width_tenths_mm) / 10.0) * px_per_mm
                } else {
                    ((f32::from(block.width_tenths_mm) / 10.0) * px_per_mm).max(24.0 * z)
                };
                let (_bh_mm, above_mm, below_mm) = block_bounds_mm[idx];
                let above_px = above_mm * h_px_per_mm;
                let below_px = below_mm * h_px_per_mm;

                let is_carrier = catalog
                    .find_by_id(&block.spec_id)
                    .is_some_and(|s| s.is_busbar_carrier());

                if e_planer {
                    let y0 = rail_center_y - above_px;
                    let y1 = rail_center_y + below_px;
                    let block_rect =
                        Rect::from_min_max(Pos2::new(cursor_x, y0), Pos2::new(cursor_x + w, y1));
                    block_rects.push(block_rect);
                    block_x_spans.push((cursor_x, cursor_x + w));
                    if quick_add_anchor_idx == Some(idx) {
                        anchor_housing_top_left = Some((idx, block_rect.min));
                    }

                    let is_selected = selected.contains(&idx);
                    draw_e_planer_block(
                        &painter,
                        block,
                        idx,
                        block_rect,
                        pos_top_y,
                        &mut max_pos_bottom_y,
                        is_selected,
                        is_carrier,
                        dark,
                        z,
                        state.editing_tier,
                        &mut tier_hit_rects,
                    );
                } else {
                    let block_min_row = (0..block.tiers.len())
                        .map(|l| block.tier_visual_row(l))
                        .min()
                        .unwrap_or(0);
                    let block_max_row = (0..block.tiers.len())
                        .map(|l| block.tier_visual_row(l))
                        .max()
                        .unwrap_or(0);

                    let tiers_top = baseline_bottom_y
                        - ((block_max_row + 1) as f32) * (tier_height + tier_gap)
                        - 3.0 * z;
                    let tiers_bottom = baseline_bottom_y
                        - (block_min_row as f32) * (tier_height + tier_gap)
                        - tier_gap
                        + 3.0 * z;
                    let prop_top = rail_center_y - above_px;
                    let prop_bottom = rail_center_y + below_px;
                    let outer_top = tiers_top.min(prop_top);
                    let outer_bottom = tiers_bottom.max(prop_bottom);

                    let hit_rect = Rect::from_min_max(
                        Pos2::new(cursor_x, outer_top),
                        Pos2::new(cursor_x + w, outer_bottom.max(lowest_bottom_y) + 20.0 * z),
                    );
                    block_rects.push(hit_rect);
                    block_x_spans.push((cursor_x, cursor_x + w));
                    if quick_add_anchor_idx == Some(idx) {
                        anchor_housing_top_left = Some((idx, Pos2::new(cursor_x, outer_top)));
                    }

                    let is_selected = selected.contains(&idx);
                    draw_terminal_block(
                        &painter,
                        block,
                        idx,
                        cursor_x,
                        w,
                        outer_top,
                        outer_bottom,
                        baseline_bottom_y,
                        pos_top_y,
                        &mut max_pos_bottom_y,
                        tier_height,
                        tier_gap,
                        is_selected,
                        is_carrier,
                        dark,
                        z,
                        state.editing_tier,
                        &mut tier_hit_rects,
                    );
                }

                cursor_x += w + block_gap;

                // Render end/partition plate immediately after block `idx` matching left block's contour
                if let Some(plate) = end_plates.iter().find(|p| p.after_block_idx == idx) {
                    let plate_px = if e_planer {
                        (f32::from(plate.width_tenths_mm) / 10.0) * px_per_mm
                    } else {
                        ((f32::from(plate.width_tenths_mm) / 10.0) * px_per_mm).max(5.5 * z)
                    };
                    let (p_top, p_bottom) = if e_planer {
                        (rail_center_y - above_px, rail_center_y + below_px)
                    } else {
                        let tier_p_top = baseline_bottom_y
                            - ((plate.max_visual_row + 1) as f32) * (tier_height + tier_gap)
                            - 3.0 * z;
                        let tier_p_bot = baseline_bottom_y
                            - (plate.min_visual_row as f32) * (tier_height + tier_gap)
                            - tier_gap
                            + 3.0 * z;
                        (
                            tier_p_top.min(rail_center_y - above_px),
                            tier_p_bot.max(rail_center_y + below_px),
                        )
                    };
                    let plate_rect = Rect::from_min_max(
                        Pos2::new(cursor_x, p_top),
                        Pos2::new(cursor_x + plate_px, p_bottom),
                    );
                    let plate_fill = if plate.end_plate.contains("orange") {
                        Color32::from_rgb(234, 88, 12)
                    } else if dark {
                        Color32::from_rgb(217, 119, 6)
                    } else {
                        Color32::from_rgb(245, 158, 11)
                    };
                    painter.rect_filled(
                        plate_rect,
                        CornerRadius::same(if e_planer { 0 } else { 2 }),
                        plate_fill,
                    );
                    painter.rect_stroke(
                        plate_rect,
                        CornerRadius::same(if e_planer { 0 } else { 2 }),
                        Stroke::new(1.0, Color32::from_rgb(120, 53, 15)),
                        StrokeKind::Inside,
                    );
                    if !e_planer && plate_rect.width() >= 18.0 {
                        let pw_txt = format!("{:.1}", f32::from(plate.width_tenths_mm) / 10.0);
                        let p_col = if dark {
                            Color32::from_rgb(251, 191, 36)
                        } else {
                            Color32::from_rgb(180, 83, 9)
                        };
                        let g = painter.layout_no_wrap(
                            pw_txt,
                            FontId::proportional((8.0 * z).clamp(6.5, 10.0)),
                            p_col,
                        );
                        max_pos_bottom_y = max_pos_bottom_y.max(pos_top_y + g.size().y);
                        painter.galley(
                            Pos2::new(plate_rect.center().x - g.size().x * 0.5, pos_top_y),
                            g,
                            p_col,
                        );
                    }
                    plate_hit_rects.push((plate_rect, plate));
                    cursor_x += plate_px + block_gap;
                }
            }

            if !strip.blocks.is_empty() && strip.end_bracket_right {
                let eb_rect = Rect::from_min_max(
                    Pos2::new(cursor_x, eb_top),
                    Pos2::new(cursor_x + bracket_px, eb_bottom),
                );
                draw_end_bracket_cad(
                    &painter,
                    eb_rect,
                    pos_top_y,
                    &mut max_pos_bottom_y,
                    end_bracket,
                    *selected_end_bracket == Some(false),
                    dark,
                    z,
                );
                bracket_hit_rects.push((eb_rect, false));
                cursor_x += bracket_px;
            } else if !e_planer && cursor_x > strip_start_x {
                cursor_x -= block_gap;
            }

            let assembly_end_x = if strip.blocks.is_empty() {
                strip_start_x + assembly_content_w
            } else {
                cursor_x
            };

            // e-Planer 1:1 Dimension lines & Title Block (`Maßlinien & Schriftfeld`)
            if e_planer {
                draw_e_planer_dimensions_and_title(
                    &painter,
                    strip,
                    strip_start_x,
                    assembly_end_x,
                    rail_center_y,
                    max_above_mm,
                    max_below_mm,
                    max_strip_h_mm,
                    total_rail_mm,
                    h_px_per_mm,
                    end_bracket,
                    strings,
                    dark,
                    z,
                );
            }

            // Draw terminal type annotations clearly separated beneath all block position numbers
            let type_row_top_y = max_pos_bottom_y + (7.0 * z).max(6.0);
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

            // 2b. Render jumper bridge overlay in standard CAD mode
            if !e_planer && let Some(plan) = bridge_plan {
                draw_bridge_overlay(&painter, strip, plan, &tier_hit_rects, z, dark);
            }

            // 2c. Quick '+' button near top-left of the marked terminal block
            let mut result = CadCanvasResult::default();
            let quick_add_target: Option<(usize, Rect)> = if state.interaction_mode
                != CanvasInteractionMode::BridgeAssign
                && state.editing_tier.is_none()
                && let Some((target_idx, tl)) = anchor_housing_top_left
            {
                let btn_sz = (18.0 * z).clamp(16.0, 22.0);
                let qa_x = (tl.x - btn_sz * 0.45).max(canvas_rect.min.x + 4.0);
                let qa_y = (tl.y - btn_sz - 3.0).max(canvas_rect.min.y + 4.0);
                Some((
                    target_idx,
                    Rect::from_min_size(Pos2::new(qa_x, qa_y), Vec2::splat(btn_sz)),
                ))
            } else {
                None
            };
            if let Some((_, qa_rect)) = quick_add_target {
                let hovered_qa = ptr_pos.is_some_and(|p| qa_rect.expand(4.0).contains(p));
                let qa_fill = if hovered_qa {
                    Color32::from_rgb(14, 165, 233)
                } else {
                    Color32::from_rgb(2, 132, 199)
                };
                let qa_stroke = if dark {
                    Color32::from_rgb(186, 230, 253)
                } else {
                    Color32::WHITE
                };
                painter.rect_filled(qa_rect, CornerRadius::same(4), qa_fill);
                painter.rect_stroke(
                    qa_rect,
                    CornerRadius::same(4),
                    Stroke::new(1.25, qa_stroke),
                    StrokeKind::Inside,
                );
                draw_icon(&painter, qa_rect.shrink(2.5), UiIcon::Add, Color32::WHITE);
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
                        Some(String::new())
                    } else {
                        Some(trimmed.to_string())
                    };
                    strip.recalculate_labels();
                }
                state.editing_tier = None;
            }

            // 4. Handle Pointer & Touch Interactions (Multi-touch, Double-Click, Pan/Scroll, Bridge Assign, Click/Tap, Marquee)
            let space_pan = ui.input(|i| i.key_down(egui::Key::Space));
            let multi_touch = ui.input(|i| i.multi_touch());
            if let Some(mt) = multi_touch
                && mt.translation_delta.length_sq() > 0.0
            {
                ui.scroll_with_delta(mt.translation_delta);
            }

            let over_quick_add =
                |pos: Pos2| quick_add_target.is_some_and(|(_, r)| r.expand(4.0).contains(pos));

            let is_primary_pan = state.interaction_mode == CanvasInteractionMode::Pan || space_pan;
            if is_primary_pan && response.drag_started_by(egui::PointerButton::Primary) {
                state.pan_touch_total_dist = 0.0;
            }

            let mut touch_tap_release_pos: Option<Pos2> = None;
            if state.middle_panning {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
                if let Some(pos) = ptr_pos {
                    let prev = state.pan_last_pos.unwrap_or(pos);
                    let delta = pos - prev;
                    state.pan_last_pos = Some(pos);
                    if delta.length_sq() > 0.0 {
                        ui.scroll_with_delta(delta);
                    }
                }
            } else if multi_touch.is_none()
                && (response.dragged_by(egui::PointerButton::Secondary)
                    || (response.dragged_by(egui::PointerButton::Primary) && is_primary_pan))
            {
                let delta = response.drag_delta();
                if response.dragged_by(egui::PointerButton::Primary) && is_primary_pan {
                    state.pan_touch_total_dist += delta.length();
                }
                if delta.length_sq() > 0.0 {
                    ui.scroll_with_delta(delta);
                }
            } else if response.hovered() && ptr_pos.is_some_and(over_quick_add) {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            } else if state.interaction_mode == CanvasInteractionMode::BridgeAssign
                && response.hovered()
            {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
            }

            if is_primary_pan
                && multi_touch.is_none()
                && response.drag_stopped_by(egui::PointerButton::Primary)
            {
                if state.pan_touch_total_dist < 12.0 {
                    touch_tap_release_pos = response.interact_pointer_pos().or(ptr_pos);
                }
                state.pan_touch_total_dist = 0.0;
            }

            let touch_pad = if cfg!(target_os = "android") {
                6.0
            } else {
                0.0
            };

            if state.interaction_mode != CanvasInteractionMode::BridgeAssign
                && response.double_clicked_by(egui::PointerButton::Primary)
                && let Some(pos) = response.interact_pointer_pos()
                && !over_quick_add(pos)
                && let Some((b_idx, t_idx, _, _)) = tier_hit_rects
                    .iter()
                    .find(|(_, _, tier_rect, _)| tier_rect.expand(touch_pad).contains(pos))
                    .copied()
            {
                *selected_end_bracket = None;
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

            if state.editing_tier.is_none() && multi_touch.is_none() {
                let use_marquee =
                    state.interaction_mode == CanvasInteractionMode::SelectBox && !space_pan;
                let use_bridge =
                    state.interaction_mode == CanvasInteractionMode::BridgeAssign && !space_pan;

                if use_marquee
                    && response.drag_started_by(egui::PointerButton::Primary)
                    && let Some(pos) = response.interact_pointer_pos()
                    && !over_quick_add(pos)
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
                        if !next_sel.is_empty() {
                            *selected_end_bracket = None;
                        }
                        *selected = next_sel;
                    }
                }

                // Left-drag mass-bridging across consecutive terminals (`massenbrücken`)
                if use_bridge && let Some(pot_id) = active_potential_id {
                    if response.drag_started_by(egui::PointerButton::Primary)
                        && let Some(pos) = response.interact_pointer_pos()
                    {
                        let initial_present = tier_hit_rects
                            .iter()
                            .find(|(_, _, tier_rect, _)| tier_rect.expand(touch_pad).contains(pos))
                            .is_some_and(|&(b_idx, t_idx, _, _)| {
                                strip.blocks.get(b_idx).is_some_and(|b| {
                                    strip
                                        .potentials
                                        .iter()
                                        .find(|p| p.id == pot_id)
                                        .is_some_and(|p| p.members.contains(&(b.id, t_idx)))
                                })
                            });
                        state.bridge_drag_action = Some(!initial_present);
                        state.bridge_painted.clear();
                        state.bridge_last_pos = Some(pos);
                    }

                    if response.dragged_by(egui::PointerButton::Primary)
                        && let Some(action) = state.bridge_drag_action
                        && let Some(curr) = response.interact_pointer_pos()
                    {
                        let prev = state.bridge_last_pos.unwrap_or(curr);
                        let dist = (curr - prev).length();
                        let steps = (dist / 6.0).ceil().clamp(1.0, 24.0) as usize;
                        for step in 0..=steps {
                            let t = step as f32 / steps as f32;
                            let sample = prev.lerp(curr, t);
                            for &(b_idx, t_idx, tier_rect, _) in &tier_hit_rects {
                                if tier_rect.expand(touch_pad).contains(sample)
                                    && state.bridge_painted.insert((b_idx, t_idx))
                                {
                                    strip.set_potential_member(pot_id, b_idx, t_idx, action);
                                }
                            }
                        }
                        state.bridge_last_pos = Some(curr);
                    }
                }

                if response.drag_stopped_by(egui::PointerButton::Primary)
                    || !ui.input(|i| i.pointer.button_down(egui::PointerButton::Primary))
                {
                    state.is_marquee_active = false;
                    state.drag_start = None;
                    state.drag_current = None;
                    state.pre_drag_selection.clear();
                    state.bridge_drag_action = None;
                    state.bridge_painted.clear();
                    state.bridge_last_pos = None;
                }

                let tap_pos = if response.clicked_by(egui::PointerButton::Primary) {
                    response.interact_pointer_pos()
                } else {
                    touch_tap_release_pos
                };

                if let Some(pos) = tap_pos {
                    if let Some((target_idx, qa_rect)) = quick_add_target
                        && qa_rect.expand(4.0).contains(pos)
                    {
                        result.quick_add_after_block = Some(target_idx);
                    } else if use_bridge {
                        *selected_end_bracket = None;
                        if let Some((b_idx, t_idx, _, _)) = tier_hit_rects
                            .iter()
                            .find(|(_, _, tier_rect, _)| tier_rect.expand(touch_pad).contains(pos))
                            .copied()
                            && let Some(pot_id) = active_potential_id
                        {
                            strip.toggle_potential_member(pot_id, b_idx, t_idx);
                        }
                    } else if let Some((_, is_left)) = bracket_hit_rects
                        .iter()
                        .find(|(r, _)| r.expand(touch_pad).contains(pos))
                    {
                        *selected_end_bracket = Some(*is_left);
                        selected.clear();
                        state.selection_anchor = None;
                    } else {
                        let hit = block_rects
                            .iter()
                            .position(|r| r.contains(pos))
                            .or_else(|| {
                                if touch_pad > 0.0 {
                                    block_rects.iter().position(|r| r.expand(touch_pad).contains(pos))
                                } else {
                                    None
                                }
                            });
                        match hit {
                            Some(idx) => {
                                *selected_end_bracket = None;
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
                                    *selected_end_bracket = None;
                                }
                            }
                        }
                    }
                }
            } else if multi_touch.is_some() {
                state.is_marquee_active = false;
                state.drag_start = None;
                state.drag_current = None;
                state.bridge_drag_action = None;
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

            // 5. Hover tooltip with detailed block, tier, end-bracket, or end-plate info
            if !state.is_marquee_active
                && state.editing_tier.is_none()
                && let Some(hover_pos) = response.hover_pos()
            {
                if let Some((target_idx, qa_rect)) = quick_add_target
                    && qa_rect.expand(4.0).contains(hover_pos)
                    && let Some(blk) = strip.blocks.get(target_idx)
                {
                    let tip = tr::insert_same_after_canvas(strings.lang, target_idx, &blk.spec_name);
                    response.on_hover_text_at_pointer(tip);
                } else if let Some((_, is_left)) = bracket_hit_rects.iter().find(|(r, _)| r.contains(hover_pos))
                {
                    let side = if *is_left {
                        strings.end_bracket_left
                    } else {
                        strings.end_bracket_right
                    };
                    let mounting = if strings.lang == Language::De {
                        end_bracket.mounting_de
                    } else {
                        end_bracket.mounting_en
                    };
                    let tip = format!(
                        "{} ({}): {}\n{} | Art.-Nr. {} | {:.1} mm | {}: {:.1} mm\n{}",
                        strings.acc_end_bracket,
                        side,
                        end_bracket.name,
                        end_bracket.manufacturer,
                        end_bracket.article_number,
                        f32::from(end_bracket.width_tenths_mm) / 10.0,
                        strings.height_label,
                        f32::from(end_bracket.height_tenths_mm) / 10.0,
                        mounting,
                    );
                    response.on_hover_text_at_pointer(tip);
                } else if let Some((_, plate)) = plate_hit_rects.iter().find(|(r, _)| r.contains(hover_pos))
                {
                    let tip = format!(
                        "{}: {}\n{} (#{}) | {:.1} mm",
                        strings.acc_end_plate,
                        plate.end_plate,
                        plate.manufacturer,
                        plate.after_block_idx + 1,
                        f32::from(plate.width_tenths_mm) / 10.0
                    );
                    response.on_hover_text_at_pointer(tip);
                } else if let Some(hovered_idx) =
                    block_rects.iter().position(|r| r.contains(hover_pos))
                {
                    let block = &strip.blocks[hovered_idx];
                    let height_str = catalog
                        .find_by_id(&block.spec_id)
                        .map(|sp| {
                            format!(
                                " | {}: {:.1} mm",
                                strings.height_label,
                                f32::from(sp.effective_height_tenths_mm()) / 10.0
                            )
                        })
                        .unwrap_or_default();
                    let mut lines = vec![format!(
                        "#{} — {} ({})",
                        hovered_idx + 1,
                        block.spec_name,
                        block.manufacturer
                    )];
                    lines.push(format!(
                        "{:.1} mm{height_str} | {} {}",
                        f32::from(block.width_tenths_mm) / 10.0,
                        block.tiers.len(),
                        tr::levels_summary(strings.lang, block.tiers.len()),
                    ));
                    let pe_count = block.tiers.iter().filter(|t| t.behavior.is_pe()).count();
                    for (t_idx, tier) in block.tiers.iter().enumerate().rev() {
                        let lbl = if tier.is_blank_marker() {
                            tr::blank_marker_tooltip(strings.lang).to_string()
                        } else if tier.computed_label.is_empty() {
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
                        let vrow = block.tier_visual_row(t_idx);
                        let prefix = if tier.behavior.is_pe() {
                            if pe_count > 1 {
                                format!("PE{}", t_idx + 1)
                            } else if vrow >= 0 {
                                format!("E{}", vrow + 1)
                            } else {
                                "PE".to_string()
                            }
                        } else {
                            format!("E{}", vrow.max(0) + 1)
                        };
                        lines.push(format!("  {}: {}", prefix, lbl));
                    }
                    response.on_hover_text_at_pointer(lines.join("\n"));
                }
            }

            result
        })
        .inner
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

    for (idx, block) in strip.blocks.iter().enumerate() {
        let Some(&(x0, x1)) = spans.get(idx) else {
            continue;
        };
        let block_w = x1 - x0;
        if block_w < 8.5 {
            continue;
        }
        let center_x = (x0 + x1) * 0.5;
        let font_sz = (12.5 * zoom)
            .clamp(6.0, 13.5)
            .min((block_w * 0.62).max(5.5));
        let galley = painter.layout_no_wrap(
            block.short_type_name().to_string(),
            FontId::proportional(font_sz),
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
    outer_top: f32,
    outer_bottom: f32,
    baseline_bottom_y: f32,
    pos_top_y: f32,
    max_pos_bottom_y: &mut f32,
    tier_height: f32,
    tier_gap: f32,
    is_selected: bool,
    is_carrier: bool,
    dark: bool,
    zoom: f32,
    editing_tier: Option<(usize, usize)>,
    tier_hit_rects: &mut Vec<(usize, usize, Rect, Rect)>,
) {
    let outer_rect = Rect::from_min_max(Pos2::new(x, outer_top), Pos2::new(x + width, outer_bottom));

    // Proportional front-view outer housing silhouette (reflects real Bauhöhe across TS 35 rail)
    let housing_fill = if dark {
        Color32::from_rgba_unmultiplied(30, 41, 59, 195)
    } else {
        Color32::from_rgba_unmultiplied(226, 232, 240, 210)
    };
    let housing_stroke = if is_selected {
        Color32::from_rgb(14, 165, 233)
    } else if dark {
        Color32::from_rgb(71, 85, 105)
    } else {
        Color32::from_rgb(148, 163, 184)
    };
    painter.rect_filled(outer_rect, CornerRadius::same(3), housing_fill);
    painter.rect_stroke(
        outer_rect,
        CornerRadius::same(3),
        Stroke::new(if is_selected { 2.0 } else { 1.0 }, housing_stroke),
        StrokeKind::Inside,
    );

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
        let inset = ((vrow.max(0) as f32) * 1.2 * zoom).min(width * 0.22);

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

        // PE yellow accent bar at bottom, or copper 10x3 N-busbar accent bar on busbar carriers
        if is_carrier {
            let cu_bar = Rect::from_min_size(
                Pos2::new(tier_rect.min.x, tier_rect.max.y - (5.0 * zoom).max(3.0)),
                Vec2::new(tier_rect.width(), (5.0 * zoom).max(3.0)),
            );
            painter.rect_filled(cu_bar, CornerRadius::same(1), Color32::from_rgb(217, 119, 6));
        } else if tier.behavior.is_pe() {
            let pe_bar = Rect::from_min_size(
                Pos2::new(tier_rect.min.x, tier_rect.max.y - (5.0 * zoom).max(3.0)),
                Vec2::new(tier_rect.width(), (5.0 * zoom).max(3.0)),
            );
            painter.rect_filled(pe_bar, CornerRadius::same(1), Color32::from_rgb(250, 204, 21));
        }

        // Inner marker tag box placed cleanly below the header/corner-badge zone
        let tag_rect = Rect::from_min_max(
            Pos2::new(tier_rect.min.x + 3.0 * zoom, tier_rect.min.y + 13.5 * zoom),
            Pos2::new(tier_rect.max.x - 3.0 * zoom, tier_rect.max.y - 5.5 * zoom),
        );
        tier_hit_rects.push((block_idx, level, tier_rect, tag_rect));

        let is_being_edited = editing_tier == Some((block_idx, level));
        if !is_being_edited && tag_rect.width() > 8.0 && tag_rect.height() > 8.0 {
            painter.rect_filled(tag_rect, CornerRadius::same(2), badge_fill);
            let display = if !tier.cad_display_label.is_empty() {
                tier.cad_display_label.as_str()
            } else if is_carrier {
                "N-ST"
            } else {
                "—"
            };
            let base_sz = if display.len() >= 4 {
                10.5 * zoom
            } else {
                13.5 * zoom
            };
            let max_w_sz =
                (tag_rect.width() - 2.0).max(4.0) / (display.len().max(1) as f32 * 0.66);
            let font_size = base_sz.min(max_w_sz).min(tag_rect.height() * 0.72).clamp(6.0, 14.0);
            let clipped_tag = painter.with_clip_rect(tag_rect);
            clipped_tag.text(
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

        // High-contrast tier level pill (E1, E2, E3, PE, PE1, PE2, ST) in dedicated top-right header strip
        if zoom >= 0.7 && tier_rect.width() >= 22.0 {
            let pe_count = block.tiers.iter().filter(|t| t.behavior.is_pe()).count();
            let row_tag = if is_carrier {
                "ST".to_string()
            } else if tier.behavior.is_pe() {
                if pe_count > 1 {
                    format!("PE{}", level + 1)
                } else if vrow >= 0 {
                    format!("E{}", vrow + 1)
                } else {
                    "PE".to_string()
                }
            } else {
                format!("E{}", (vrow.max(0) + 1))
            };
            let (pill_bg, pill_fg) = if is_carrier {
                (
                    Color32::from_rgba_unmultiplied(154, 52, 18, 235),
                    Color32::from_rgb(254, 243, 199),
                )
            } else if tier.behavior.is_pe() {
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
            let pill_w = ((row_tag.len() as f32 * 6.0 + 5.0) * zoom).min(tier_rect.width() * 0.55);
            let pill_h = 10.5 * zoom;
            let pill_rect = Rect::from_min_size(
                Pos2::new(
                    tier_rect.max.x - pill_w - 2.0 * zoom,
                    tier_rect.min.y + 3.0 * zoom,
                ),
                Vec2::new(pill_w, pill_h),
            );
            painter.rect_filled(pill_rect, CornerRadius::same(2), pill_bg);
            painter.text(
                pill_rect.center(),
                Align2::CENTER_CENTER,
                row_tag,
                FontId::proportional((8.5 * zoom).clamp(7.0, 10.0)),
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

    // Block position index aligned along the bottom of the strip without overflowing neighbor width
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
    if width >= 9.0 {
        let (pos_label, pos_font_sz) = pos_label_and_font(width, block_idx, 12.5 * zoom);
        let pos_galley =
            painter.layout_no_wrap(pos_label, FontId::proportional(pos_font_sz), pos_color);
        *max_pos_bottom_y = (*max_pos_bottom_y).max(pos_top_y + pos_galley.size().y);
        let pos_clip = Rect::from_min_max(
            Pos2::new(x - 1.0, pos_top_y - 2.0),
            Pos2::new(x + width + 1.0, pos_top_y + pos_galley.size().y + 4.0),
        );
        painter.with_clip_rect(pos_clip).galley(
            Pos2::new(x + width * 0.5 - pos_galley.size().x * 0.5, pos_top_y),
            pos_galley,
            pos_color,
        );
    }
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
    _dark: bool,
) {
    let mut tier_rects: BTreeMap<(usize, usize), Rect> = BTreeMap::new();
    let mut row_block_rects: BTreeMap<(usize, isize), Rect> = BTreeMap::new();

    for &(b_idx, l_idx, rect, _) in tier_hit_rects {
        tier_rects.insert((b_idx, l_idx), rect);
        if let Some(block) = strip.blocks.get(b_idx) {
            row_block_rects.insert((b_idx, block.tier_visual_row(l_idx)), rect);
        }
    }

    // 1. Assigned potential indicator dots cleanly in top-left tier header (never overlapping E1/E2 or marker text)
    for (&(b_idx, l_idx), &(_, ref _pot_name, rgb)) in &plan.point_assignments {
        if let Some(t_rect) = tier_rects.get(&(b_idx, l_idx)) {
            let pot_color = Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
            let dot_center = Pos2::new(t_rect.min.x + 6.5 * zoom, t_rect.min.y + 8.5 * zoom);
            painter.circle_filled(dot_center, 3.6 * zoom, pot_color);
            painter.circle_stroke(dot_center, 3.6 * zoom, Stroke::new(1.0, Color32::WHITE));
        }
    }

    // 2. Comb jumper segments (`Kammbrücken`) drawn strictly BELOW their respective tier housing
    // inside `tier_gap` so no E1/E2 badges, marker tags, or block numbers overlap.
    for seg in &plan.comb_segments {
        let Some(start_r) = row_block_rects.get(&(seg.start_block_idx, seg.visual_row)) else {
            continue;
        };
        let Some(end_r) = row_block_rects.get(&(seg.end_block_idx, seg.visual_row)) else {
            continue;
        };

        let pot_color = Color32::from_rgb(seg.color_rgb[0], seg.color_rgb[1], seg.color_rgb[2]);
        let (bar_y, bar_half_h, leg_dx) = if seg.channel_slot == 0 {
            (start_r.max.y + 15.5 * zoom, 6.5 * zoom, 0.0)
        } else {
            (start_r.max.y + 28.5 * zoom, 5.0 * zoom, 4.0 * zoom)
        };
        let x0 = start_r.center().x;
        let x1 = end_r.center().x;

        // Bold metallic pin legs bridging clearly from tier bottom down into comb jumper bar
        let leg_top = start_r.max.y;
        let leg_bottom = bar_y - bar_half_h + 1.0 * zoom;
        let leg_h = (leg_bottom - leg_top).max(4.0 * zoom);
        let leg_cy = (leg_top + leg_bottom) * 0.5;
        let leg_w = (5.4 * zoom).max(3.6);
        for &pin_b in &seg.pin_blocks {
            if let Some(pr) = row_block_rects.get(&(pin_b, seg.visual_row)) {
                let cx = pr.center().x + leg_dx;
                let prong = Rect::from_center_size(Pos2::new(cx, leg_cy), Vec2::new(leg_w, leg_h));
                painter.rect_filled(prong, CornerRadius::same(1), Color32::from_rgb(203, 213, 225));
                painter.rect_stroke(
                    prong,
                    CornerRadius::same(1),
                    Stroke::new(1.1, Color32::from_rgb(15, 23, 42)),
                    StrokeKind::Inside,
                );
            }
        }

        // Insulated comb jumper bar housing underneath the connected tier row
        let bar_rect = Rect::from_min_max(
            Pos2::new(x0 - 6.0 * zoom, bar_y - bar_half_h),
            Pos2::new(x1 + 6.0 * zoom, bar_y + bar_half_h),
        );
        painter.rect_filled(bar_rect, CornerRadius::same(3), pot_color);
        painter.rect_stroke(
            bar_rect,
            CornerRadius::same(3),
            Stroke::new(1.1, Color32::from_rgba_unmultiplied(15, 23, 42, 160)),
            StrokeKind::Inside,
        );

        // Visually highlight removed / cut-out pins (`Pin entfernen`) in the missing pin gap
        // right between the bar and the terminal so bar label text remains completely legible.
        let cut_y = if seg.channel_slot == 0 {
            start_r.max.y + 4.8 * zoom
        } else {
            bar_y - bar_half_h - 4.5 * zoom
        };
        for &cut_b in &seg.cut_pins {
            if let Some(cr) = row_block_rects.get(&(cut_b, seg.visual_row)) {
                let cx = cr.center().x + leg_dx;
                let r = (4.8 * zoom).max(3.6);
                painter.circle_filled(Pos2::new(cx, cut_y), r, Color32::from_rgb(220, 38, 38));
                painter.circle_stroke(Pos2::new(cx, cut_y), r, Stroke::new(1.1, Color32::WHITE));
                painter.line_segment(
                    [
                        Pos2::new(cx - r * 0.48, cut_y - r * 0.48),
                        Pos2::new(cx + r * 0.48, cut_y + r * 0.48),
                    ],
                    Stroke::new(1.6, Color32::WHITE),
                );
                painter.line_segment(
                    [
                        Pos2::new(cx - r * 0.48, cut_y + r * 0.48),
                        Pos2::new(cx + r * 0.48, cut_y - r * 0.48),
                    ],
                    Stroke::new(1.6, Color32::WHITE),
                );
            }
        }

        if zoom >= 0.7 {
            let pot_name = strip
                .potentials
                .iter()
                .find(|p| p.id == seg.potential_id)
                .map(|p| p.name.as_str())
                .unwrap_or("");
            let bar_label = if (x1 - x0) >= 68.0 * zoom && !pot_name.is_empty() {
                format!("{}p · {pot_name}", seg.pole_count)
            } else {
                format!("{}p", seg.pole_count)
            };
            let lbl_font = FontId::proportional((bar_half_h * 1.45).clamp(7.5, 13.0));
            let lum = (seg.color_rgb[0] as u16 * 3
                + seg.color_rgb[1] as u16 * 6
                + seg.color_rgb[2] as u16)
                / 10;
            let text_col = if lum > 175 {
                Color32::BLACK
            } else {
                Color32::WHITE
            };
            if lum <= 175 {
                // Crisp subtle dark shadow for extra contrast on bright red/orange/green bars
                painter.text(
                    Pos2::new((x0 + x1) * 0.5 + 0.8, bar_y + 0.8),
                    Align2::CENTER_CENTER,
                    &bar_label,
                    lbl_font.clone(),
                    Color32::from_rgba_unmultiplied(15, 23, 42, 180),
                );
            }
            painter.text(
                Pos2::new((x0 + x1) * 0.5, bar_y),
                Align2::CENTER_CENTER,
                bar_label,
                lbl_font,
                text_col,
            );
        }
    }

    // 3. Wire bridges (`Drahtbrücken`) for cross-tier, end-plate barriers, single-chamber overflow, or pitch jumps
    for (wb_idx, wb) in plan.wire_bridges.iter().enumerate() {
        let Some(r_from) = tier_rects.get(&wb.from) else {
            continue;
        };
        let Some(r_to) = tier_rects.get(&wb.to) else {
            continue;
        };

        let pot_color = Color32::from_rgb(wb.color_rgb[0], wb.color_rgb[1], wb.color_rgb[2]);
        let (p0, p1, arch) = if wb.from.0 == wb.to.0 {
            let p0 = Pos2::new(r_from.max.x - 5.5 * zoom, r_from.max.y - 4.0 * zoom);
            let p1 = Pos2::new(r_to.max.x - 5.5 * zoom, r_to.max.y - 4.0 * zoom);
            let arch = Pos2::new(r_from.max.x + 9.0 * zoom, (p0.y + p1.y) * 0.5);
            (p0, p1, arch)
        } else {
            let from_x = if wb.from.0 < wb.to.0 {
                r_from.max.x - 5.5 * zoom
            } else {
                r_from.min.x + 5.5 * zoom
            };
            let to_x = if wb.from.0 < wb.to.0 {
                r_to.min.x + 5.5 * zoom
            } else {
                r_to.max.x - 5.5 * zoom
            };
            let p0 = Pos2::new(from_x, r_from.max.y - 3.0 * zoom);
            let p1 = Pos2::new(to_x, r_to.max.y - 3.0 * zoom);
            let lo_b = wb.from.0.min(wb.to.0);
            let hi_b = wb.from.0.max(wb.to.0);
            let from_vrow = strip
                .blocks
                .get(wb.from.0)
                .map(|b| b.tier_visual_row(wb.from.1))
                .unwrap_or(0);
            let crosses_comb_slot0 = plan.comb_segments.iter().any(|seg| {
                seg.visual_row == from_vrow
                    && seg.channel_slot == 0
                    && seg.end_block_idx > lo_b
                    && seg.start_block_idx < hi_b
            });
            let lane = (wb_idx % 3) as f32 * 4.0 * zoom;
            let drop_y = if crosses_comb_slot0 {
                27.5 * zoom + lane
            } else {
                16.0 * zoom + lane
            };
            let arch = Pos2::new((p0.x + p1.x) * 0.5, p0.y.max(p1.y) + drop_y);
            (p0, p1, arch)
        };

        // Two-segment polyline with clear DB badge
        let stroke = Stroke::new((2.2 * zoom).max(1.5), pot_color);
        painter.line_segment([p0, arch], stroke);
        painter.line_segment([arch, p1], stroke);
        painter.circle_filled(p0, 3.0 * zoom, pot_color);
        painter.circle_filled(p1, 3.0 * zoom, pot_color);

        let badge_rect = Rect::from_center_size(arch, Vec2::new(18.0 * zoom, 11.0 * zoom));
        painter.rect_filled(badge_rect, CornerRadius::same(3), pot_color);
        painter.rect_stroke(
            badge_rect,
            CornerRadius::same(3),
            Stroke::new(1.0, Color32::WHITE),
            StrokeKind::Inside,
        );
        painter.text(
            arch,
            Align2::CENTER_CENTER,
            "DB",
            FontId::proportional((7.5 * zoom).max(6.5)),
            Color32::WHITE,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_end_bracket_cad(
    painter: &egui::Painter,
    rect: Rect,
    pos_top_y: f32,
    max_pos_bottom_y: &mut f32,
    spec: &EndBracketSpec,
    is_selected: bool,
    dark: bool,
    zoom: f32,
) {
    if is_selected {
        painter.rect_filled(
            rect.expand(2.5),
            CornerRadius::same(4),
            Color32::from_rgba_unmultiplied(14, 165, 233, 45),
        );
    }

    let body_fill = if dark {
        Color32::from_rgb(51, 65, 85)
    } else {
        Color32::from_rgb(100, 116, 139)
    };
    let border_col = if is_selected {
        Color32::from_rgb(14, 165, 233)
    } else if dark {
        Color32::from_rgb(148, 163, 184)
    } else {
        Color32::from_rgb(30, 41, 59)
    };
    let border_w = if is_selected { 2.2 } else { 1.4 };
    let foot_col = if dark {
        Color32::from_rgb(203, 213, 225)
    } else {
        Color32::from_rgb(226, 232, 240)
    };

    painter.rect_filled(rect, CornerRadius::same(3), body_fill);
    painter.rect_stroke(
        rect,
        CornerRadius::same(3),
        Stroke::new(border_w, border_col),
        StrokeKind::Inside,
    );

    // Upper and lower metallic clamp foot / snap lock bars
    let foot_h = (5.0 * zoom).max(3.5);
    let top_foot = Rect::from_min_max(
        Pos2::new(rect.min.x + 2.0, rect.min.y + 2.0),
        Pos2::new(rect.max.x - 2.0, rect.min.y + 2.0 + foot_h),
    );
    let bot_foot = Rect::from_min_max(
        Pos2::new(rect.min.x + 2.0, rect.max.y - 2.0 - foot_h),
        Pos2::new(rect.max.x - 2.0, rect.max.y - 2.0),
    );
    painter.rect_filled(top_foot, CornerRadius::same(1), foot_col);
    painter.rect_filled(bot_foot, CornerRadius::same(1), foot_col);

    // Center clamp mark
    if rect.width() >= 11.0 {
        let eh_sz = (10.0 * zoom).min(rect.width() * 0.45).clamp(6.0, 11.5);
        painter.with_clip_rect(rect).text(
            rect.center(),
            Align2::CENTER_CENTER,
            "EH",
            FontId::proportional(eh_sz),
            Color32::WHITE,
        );
    }

    if zoom >= 0.75 && rect.width() >= 18.0 {
        let lbl_col = if is_selected {
            if dark {
                Color32::from_rgb(56, 189, 248)
            } else {
                Color32::from_rgb(2, 132, 199)
            }
        } else if dark {
            Color32::from_rgb(148, 163, 184)
        } else {
            Color32::from_rgb(51, 65, 85)
        };
        let w_txt = format!("{:.1}", f32::from(spec.width_tenths_mm) / 10.0);
        let w_sz = (8.5 * zoom)
            .min((rect.width() - 2.0).max(4.0) / (w_txt.len().max(1) as f32 * 0.6))
            .clamp(5.5, 10.0);
        let g = painter.layout_no_wrap(w_txt, FontId::proportional(w_sz), lbl_col);
        *max_pos_bottom_y = (*max_pos_bottom_y).max(pos_top_y + g.size().y);
        let clip = Rect::from_min_max(
            Pos2::new(rect.min.x - 1.0, pos_top_y - 2.0),
            Pos2::new(rect.max.x + 1.0, pos_top_y + g.size().y + 4.0),
        );
        painter.with_clip_rect(clip).galley(
            Pos2::new(rect.center().x - g.size().x * 0.5, pos_top_y),
            g,
            lbl_col,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_ts35_din_rail(
    painter: &egui::Painter,
    x0: f32,
    x1: f32,
    rail_center_y: f32,
    h_px_per_mm: f32,
    dark: bool,
    e_planer: bool,
    zoom: f32,
    rail_overhang: f32,
) {
    let half_h = 17.5 * h_px_per_mm;
    let flange_h = 13.5 * h_px_per_mm;
    let rail_rect = Rect::from_min_max(
        Pos2::new(x0, rail_center_y - half_h),
        Pos2::new(x1, rail_center_y + half_h),
    );
    let web_rect = Rect::from_min_max(
        Pos2::new(x0, rail_center_y - flange_h),
        Pos2::new(x1, rail_center_y + flange_h),
    );

    let (rail_fill, web_fill, rail_stroke, lbl_col) = if e_planer {
        if dark {
            (
                Color32::from_rgba_unmultiplied(100, 116, 139, 55),
                Color32::from_rgba_unmultiplied(148, 163, 184, 30),
                Color32::from_rgb(148, 163, 184),
                Color32::from_rgb(203, 213, 225),
            )
        } else {
            (
                Color32::from_rgb(226, 232, 240),
                Color32::from_rgb(241, 245, 249),
                Color32::from_rgb(100, 116, 139),
                Color32::from_rgb(71, 85, 105),
            )
        }
    } else if dark {
        (
            Color32::from_rgba_unmultiplied(100, 116, 139, 75),
            Color32::from_rgba_unmultiplied(148, 163, 184, 42),
            Color32::from_rgb(100, 116, 139),
            Color32::from_rgb(148, 163, 184),
        )
    } else {
        (
            Color32::from_rgb(203, 213, 225),
            Color32::from_rgb(226, 232, 240),
            Color32::from_rgb(148, 163, 184),
            Color32::from_rgb(71, 85, 105),
        )
    };

    painter.rect_filled(rail_rect, CornerRadius::ZERO, rail_fill);
    painter.rect_filled(web_rect, CornerRadius::ZERO, web_fill);
    painter.rect_stroke(
        rail_rect,
        CornerRadius::ZERO,
        Stroke::new(1.2, rail_stroke),
        StrokeKind::Inside,
    );
    painter.line_segment(
        [
            Pos2::new(x0, rail_center_y - flange_h),
            Pos2::new(x1, rail_center_y - flange_h),
        ],
        Stroke::new(0.8, rail_stroke),
    );
    painter.line_segment(
        [
            Pos2::new(x0, rail_center_y + flange_h),
            Pos2::new(x1, rail_center_y + flange_h),
        ],
        Stroke::new(0.8, rail_stroke),
    );

    if zoom >= 0.55 && rail_overhang >= 24.0 {
        let max_sz = ((rail_overhang - 6.0) / 3.4).clamp(6.5, 10.5);
        let ts_sz = (9.5 * zoom).min(max_sz).clamp(6.5, 10.5);
        let clip = Rect::from_min_max(
            Pos2::new(x0 + 2.0, rail_center_y - half_h),
            Pos2::new(x0 + rail_overhang - 2.0, rail_center_y + half_h),
        );
        painter.with_clip_rect(clip).text(
            Pos2::new(x0 + 4.0, rail_center_y),
            Align2::LEFT_CENTER,
            "TS 35",
            FontId::monospace(ts_sz),
            lbl_col,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_e_planer_block(
    painter: &egui::Painter,
    block: &TerminalBlock,
    block_idx: usize,
    block_rect: Rect,
    pos_top_y: f32,
    max_pos_bottom_y: &mut f32,
    is_selected: bool,
    is_carrier: bool,
    dark: bool,
    zoom: f32,
    editing_tier: Option<(usize, usize)>,
    tier_hit_rects: &mut Vec<(usize, usize, Rect, Rect)>,
) {
    if is_selected {
        painter.rect_filled(
            block_rect.expand(2.5),
            CornerRadius::ZERO,
            Color32::from_rgba_unmultiplied(14, 165, 233, 55),
        );
    }

    // Order tiers top-to-bottom by visual row (highest row at top, PE row -1 at bottom)
    let mut ordered_tiers: Vec<(usize, isize)> = (0..block.tiers.len())
        .map(|t| (t, block.tier_visual_row(t)))
        .collect();
    ordered_tiers.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));

    let n = ordered_tiers.len().max(1);
    let band_h = block_rect.height() / (n as f32);

    let outline_col = if is_selected {
        Color32::from_rgb(14, 165, 233)
    } else if dark {
        Color32::from_rgb(148, 163, 184)
    } else {
        Color32::from_rgb(30, 41, 59)
    };

    for (band_pos, &(t_idx, _vrow)) in ordered_tiers.iter().enumerate() {
        let tier = &block.tiers[t_idx];
        let y0 = block_rect.min.y + (band_pos as f32) * band_h;
        let y1 = y0 + band_h;
        let band_rect = Rect::from_min_max(
            Pos2::new(block_rect.min.x, y0),
            Pos2::new(block_rect.max.x, y1),
        );

        let (body_fill, _tag_text_col, _badge_fill) = tier_palette(tier.color, dark);
        let tint = Color32::from_rgba_unmultiplied(
            body_fill.r(),
            body_fill.g(),
            body_fill.b(),
            if dark { 180 } else { 205 },
        );
        painter.rect_filled(band_rect, CornerRadius::ZERO, tint);

        if is_carrier {
            let cu_slot_h = (band_h * 0.14).clamp(3.0, 9.0);
            let cu_slot = Rect::from_min_max(
                Pos2::new(band_rect.min.x, band_rect.max.y - cu_slot_h),
                Pos2::new(band_rect.max.x, band_rect.max.y),
            );
            painter.rect_filled(cu_slot, CornerRadius::ZERO, Color32::from_rgb(217, 119, 6));
        }

        if band_pos > 0 {
            painter.line_segment(
                [Pos2::new(band_rect.min.x, y0), Pos2::new(band_rect.max.x, y0)],
                Stroke::new(0.8, outline_col),
            );
        }

        let tag_rect = band_rect.shrink(1.5);
        tier_hit_rects.push((block_idx, t_idx, band_rect, tag_rect));

        let is_being_edited = editing_tier == Some((block_idx, t_idx));
        if !is_being_edited && band_rect.width() >= 9.0 && band_rect.height() >= 10.0 {
            let display = if !tier.cad_display_label.is_empty() {
                tier.cad_display_label.as_str()
            } else if is_carrier {
                "ST"
            } else if tier.behavior.is_pe() {
                "PE"
            } else {
                "—"
            };
            let text_col = match tier.color {
                TierColor::StandardGray if !dark => Color32::from_rgb(15, 23, 42),
                _ => Color32::WHITE,
            };
            let max_w_sz =
                (band_rect.width() - 2.0).max(4.0) / (display.len().max(1) as f32 * 0.66);
            let font_sz = (band_rect.width() * 0.42)
                .min(band_rect.height() * 0.34)
                .min(max_w_sz)
                .clamp(5.5, 12.5);
            let clipped = painter.with_clip_rect(band_rect);
            clipped.text(
                band_rect.center(),
                Align2::CENTER_CENTER,
                display,
                FontId::monospace(font_sz),
                text_col,
            );
        }
    }

    // Sharp architectural/CAD outer stroke (true 1:1 profile outline)
    painter.rect_stroke(
        block_rect,
        CornerRadius::ZERO,
        Stroke::new(if is_selected { 2.0 } else { 1.1 }, outline_col),
        StrokeKind::Inside,
    );

    // Position index directly beneath the strip envelope without overflowing adjacent blocks
    let pos_col = if is_selected {
        Color32::from_rgb(14, 165, 233)
    } else if dark {
        Color32::from_rgb(203, 213, 225)
    } else {
        Color32::from_rgb(30, 41, 59)
    };
    if block_rect.width() >= 8.5 {
        let (pos_label, pos_font_sz) =
            pos_label_and_font(block_rect.width(), block_idx, 11.0 * zoom);
        let pos_galley =
            painter.layout_no_wrap(pos_label, FontId::proportional(pos_font_sz), pos_col);
        *max_pos_bottom_y = (*max_pos_bottom_y).max(pos_top_y + pos_galley.size().y);
        let pos_clip = Rect::from_min_max(
            Pos2::new(block_rect.min.x - 1.0, pos_top_y - 2.0),
            Pos2::new(block_rect.max.x + 1.0, pos_top_y + pos_galley.size().y + 4.0),
        );
        painter.with_clip_rect(pos_clip).galley(
            Pos2::new(
                block_rect.center().x - pos_galley.size().x * 0.5,
                pos_top_y,
            ),
            pos_galley,
            pos_col,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_e_planer_dimensions_and_title(
    painter: &egui::Painter,
    strip: &TerminalStrip,
    start_x: f32,
    end_x: f32,
    rail_center_y: f32,
    max_above_mm: f32,
    max_below_mm: f32,
    envelope_h_mm: f32,
    total_w_mm: f32,
    px_per_mm: f32,
    end_bracket: &EndBracketSpec,
    strings: &Strings,
    dark: bool,
    zoom: f32,
) {
    let dim_col = if dark {
        Color32::from_rgb(248, 113, 113)
    } else {
        Color32::from_rgb(185, 28, 28)
    };
    let top_edge_y = rail_center_y - max_above_mm * px_per_mm;
    let bot_edge_y = rail_center_y + max_below_mm * px_per_mm;
    let dim_stroke = Stroke::new(1.1, dim_col);
    let tick = 3.5 * zoom;

    // 1. Horizontal total-width dimension line above the assembly
    let dim_y = top_edge_y - (18.0 * zoom).max(15.0);
    painter.line_segment(
        [Pos2::new(start_x, top_edge_y - 3.0), Pos2::new(start_x, dim_y - 4.0)],
        dim_stroke,
    );
    painter.line_segment(
        [Pos2::new(end_x, top_edge_y - 3.0), Pos2::new(end_x, dim_y - 4.0)],
        dim_stroke,
    );
    painter.line_segment(
        [Pos2::new(start_x, dim_y), Pos2::new(end_x, dim_y)],
        dim_stroke,
    );
    painter.line_segment(
        [Pos2::new(start_x - tick, dim_y + tick), Pos2::new(start_x + tick, dim_y - tick)],
        dim_stroke,
    );
    painter.line_segment(
        [Pos2::new(end_x - tick, dim_y + tick), Pos2::new(end_x + tick, dim_y - tick)],
        dim_stroke,
    );
    let w_galley = painter.layout_no_wrap(
        format!("{total_w_mm:.1} mm"),
        FontId::monospace((11.5 * zoom).clamp(9.5, 14.0)),
        dim_col,
    );
    let w_text_h = w_galley.size().y;
    let w_text_w = w_galley.size().x;
    painter.galley(
        Pos2::new((start_x + end_x) * 0.5 - w_text_w * 0.5, dim_y - 4.0 - w_text_h),
        w_galley,
        dim_col,
    );

    // 2. Vertical max-envelope height dimension line on the right
    let dim_x = end_x + 15.0 * px_per_mm + 18.0 * zoom;
    painter.line_segment(
        [Pos2::new(end_x + 4.0, top_edge_y), Pos2::new(dim_x + 4.0, top_edge_y)],
        dim_stroke,
    );
    painter.line_segment(
        [Pos2::new(end_x + 4.0, bot_edge_y), Pos2::new(dim_x + 4.0, bot_edge_y)],
        dim_stroke,
    );
    painter.line_segment(
        [Pos2::new(dim_x, top_edge_y), Pos2::new(dim_x, bot_edge_y)],
        dim_stroke,
    );
    painter.line_segment(
        [Pos2::new(dim_x - tick, top_edge_y + tick), Pos2::new(dim_x + tick, top_edge_y - tick)],
        dim_stroke,
    );
    painter.line_segment(
        [Pos2::new(dim_x - tick, bot_edge_y + tick), Pos2::new(dim_x + tick, bot_edge_y - tick)],
        dim_stroke,
    );
    let h_galley = painter.layout_no_wrap(
        format!("{envelope_h_mm:.1} mm"),
        FontId::monospace((11.0 * zoom).clamp(9.0, 14.0)),
        dim_col,
    );
    let h_mid_y = (top_edge_y + bot_edge_y) * 0.5;
    let h_pos = Pos2::new(dim_x - h_galley.size().y - 3.0, h_mid_y + h_galley.size().x * 0.5);
    painter.add(egui::epaint::TextShape::new(h_pos, h_galley, dim_col).with_angle(-FRAC_PI_2));

    // 3. Compact EPLAN title strip / legend (`Schriftfeld`) cleanly above width dimension label
    let note_col = if dark {
        Color32::from_rgb(148, 163, 184)
    } else {
        Color32::from_rgb(71, 85, 105)
    };
    let eb_info = if strip.end_bracket_count() > 0 {
        format!("{}× {}", strip.end_bracket_count(), end_bracket.name)
    } else {
        strings.end_bracket_none.to_string()
    };
    let note_bottom_y = dim_y - 4.0 - w_text_h - (7.0 * zoom).max(7.0);
    painter.text(
        Pos2::new(start_x, note_bottom_y),
        Align2::LEFT_BOTTOM,
        format!(
            "EPLAN Aufbaubild 1:1  |  {}  |  {}: {:.1} mm  |  EH: {}  |  TS 35 (35.0 mm)",
            strip.name, strings.max_height_label, envelope_h_mm, eb_info
        ),
        FontId::proportional((10.5 * zoom).clamp(8.5, 13.0)),
        note_col,
    );
}

/// Renders a crisp miniature 1:1-proportioned ePlan Aufbaubild silhouette of `strip`
/// (TS 35 DIN rail, end brackets, scaled terminal blocks with tier colors, and end plates).
pub fn draw_mini_eplan_preview(
    ui: &mut Ui,
    strip: &TerminalStrip,
    catalog: &TerminalCatalog,
    height: f32,
    lang: Language,
) {
    let dark = ui.visuals().dark_mode;
    let t = crate::ui::theme::ThemeTokens::for_mode(dark);
    let avail_w = ui.available_width().max(110.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(avail_w, height), Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }

    let painter = ui.painter_at(rect);
    let bg = if dark {
        Color32::from_rgb(13, 18, 29)
    } else {
        Color32::from_rgb(243, 246, 252)
    };
    painter.rect_filled(rect, CornerRadius::same(6), bg);

    // Subtle blueprint grid lines
    let grid_col = if dark {
        Color32::from_rgba_unmultiplied(255, 255, 255, 10)
    } else {
        Color32::from_rgba_unmultiplied(15, 23, 42, 12)
    };
    let mut gx = rect.min.x + 12.0;
    while gx < rect.max.x {
        painter.line_segment(
            [Pos2::new(gx, rect.min.y), Pos2::new(gx, rect.max.y)],
            Stroke::new(1.0, grid_col),
        );
        gx += 12.0;
    }
    let mid_y = rect.center().y;
    painter.line_segment(
        [Pos2::new(rect.min.x, mid_y), Pos2::new(rect.max.x, mid_y)],
        Stroke::new(1.0, grid_col),
    );

    if strip.blocks.is_empty() {
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            tr::empty_strip_canvas(lang),
            FontId::proportional(10.5),
            t.text_muted,
        );
        painter.rect_stroke(
            rect,
            CornerRadius::same(6),
            Stroke::new(1.0, t.border_hairline),
            StrokeKind::Inside,
        );
        return;
    }

    let end_plates = if strip.end_plates_generated {
        crate::catalog::accessories::calculate_end_plates(strip, catalog)
    } else {
        Vec::new()
    };
    let end_bracket = find_end_bracket(&strip.end_bracket_id);
    let bracket_count = strip.end_bracket_count();
    let brackets_mm = strip.end_brackets_width_mm();
    let plates_mm: f32 = end_plates
        .iter()
        .map(|p| f32::from(p.width_tenths_mm) / 10.0)
        .sum();
    let total_w_mm = (strip.total_width_mm() + plates_mm + brackets_mm).max(8.0);

    let block_bounds: Vec<(f32, f32, f32)> = strip
        .blocks
        .iter()
        .map(|b| block_front_bounds_mm(b, catalog))
        .collect();
    let eb_half_mm = (f32::from(end_bracket.height_tenths_mm) / 10.0) * 0.5;
    let mut max_above_mm = block_bounds
        .iter()
        .map(|&(_, a, _)| a)
        .fold(17.5_f32, f32::max);
    let mut max_below_mm = block_bounds
        .iter()
        .map(|&(_, _, b)| b)
        .fold(17.5_f32, f32::max);
    if bracket_count > 0 {
        max_above_mm = max_above_mm.max(eb_half_mm);
        max_below_mm = max_below_mm.max(eb_half_mm);
    }
    let total_h_mm = (max_above_mm + max_below_mm).max(35.0);

    let inner = rect.shrink2(Vec2::new(10.0, 5.0));
    let scale = (inner.width() / total_w_mm).min(inner.height() / total_h_mm).max(0.15);
    let assembly_w_px = total_w_mm * scale;
    let rail_center_y = inner.center().y + ((max_above_mm - max_below_mm) * 0.5) * scale;
    let start_x = inner.center().x - assembly_w_px * 0.5;

    // Miniature TS 35 DIN rail across assembly + small overhang
    let overhang_px = 6.0;
    let rail_half_h = (17.5 * scale).max(3.5);
    let rail_rect = Rect::from_min_max(
        Pos2::new(start_x - overhang_px, rail_center_y - rail_half_h),
        Pos2::new(start_x + assembly_w_px + overhang_px, rail_center_y + rail_half_h),
    );
    let (rail_fill, rail_stroke) = if dark {
        (
            Color32::from_rgba_unmultiplied(148, 163, 184, 48),
            Color32::from_rgb(100, 116, 139),
        )
    } else {
        (Color32::from_rgb(226, 232, 240), Color32::from_rgb(148, 163, 184))
    };
    painter.rect_filled(rail_rect, CornerRadius::ZERO, rail_fill);
    painter.rect_stroke(
        rail_rect,
        CornerRadius::ZERO,
        Stroke::new(0.8, rail_stroke),
        StrokeKind::Inside,
    );

    let eb_w_px = ((f32::from(end_bracket.width_tenths_mm) / 10.0) * scale).max(2.0);
    let eb_half_h_px = (eb_half_mm * scale).max(4.0);
    let eb_col = if dark {
        Color32::from_rgb(71, 85, 105)
    } else {
        Color32::from_rgb(100, 116, 139)
    };
    let outline_col = if dark {
        Color32::from_rgba_unmultiplied(203, 213, 225, 130)
    } else {
        Color32::from_rgba_unmultiplied(30, 41, 59, 145)
    };

    let mut cx = start_x;
    if strip.end_bracket_left {
        let r = Rect::from_min_max(
            Pos2::new(cx, rail_center_y - eb_half_h_px),
            Pos2::new(cx + eb_w_px, rail_center_y + eb_half_h_px),
        );
        painter.rect_filled(r, CornerRadius::ZERO, eb_col);
        painter.rect_stroke(r, CornerRadius::ZERO, Stroke::new(0.6, outline_col), StrokeKind::Inside);
        cx += eb_w_px;
    }

    for (idx, block) in strip.blocks.iter().enumerate() {
        let bw = ((f32::from(block.width_tenths_mm) / 10.0) * scale).max(1.2);
        let (_, above_mm, below_mm) = block_bounds[idx];
        let b_rect = Rect::from_min_max(
            Pos2::new(cx, rail_center_y - above_mm * scale),
            Pos2::new(cx + bw, rail_center_y + below_mm * scale),
        );

        let mut ordered_tiers: Vec<(usize, isize)> = (0..block.tiers.len())
            .map(|ti| (ti, block.tier_visual_row(ti)))
            .collect();
        ordered_tiers.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));
        let n = ordered_tiers.len().max(1);
        let band_h = b_rect.height() / (n as f32);

        for (pos, &(t_idx, _)) in ordered_tiers.iter().enumerate() {
            let tier = &block.tiers[t_idx];
            let (fill, _, _) = tier_palette(tier.color, dark);
            let band_rect = Rect::from_min_max(
                Pos2::new(b_rect.min.x, b_rect.min.y + pos as f32 * band_h),
                Pos2::new(b_rect.max.x, b_rect.min.y + (pos + 1) as f32 * band_h),
            );
            painter.rect_filled(
                band_rect,
                CornerRadius::ZERO,
                fill.gamma_multiply(if dark { 0.85 } else { 0.92 }),
            );
        }
        painter.rect_stroke(
            b_rect,
            CornerRadius::ZERO,
            Stroke::new(0.5, outline_col),
            StrokeKind::Inside,
        );
        cx += bw;

        if let Some(plate) = end_plates.iter().find(|p| p.after_block_idx == idx) {
            let pw = ((f32::from(plate.width_tenths_mm) / 10.0) * scale).max(1.0);
            let p_rect = Rect::from_min_max(
                Pos2::new(cx, b_rect.min.y),
                Pos2::new(cx + pw, b_rect.max.y),
            );
            painter.rect_filled(p_rect, CornerRadius::ZERO, Color32::from_rgb(245, 158, 11));
            cx += pw;
        }
    }

    if strip.end_bracket_right {
        let r = Rect::from_min_max(
            Pos2::new(cx, rail_center_y - eb_half_h_px),
            Pos2::new(cx + eb_w_px, rail_center_y + eb_half_h_px),
        );
        painter.rect_filled(r, CornerRadius::ZERO, eb_col);
        painter.rect_stroke(r, CornerRadius::ZERO, Stroke::new(0.6, outline_col), StrokeKind::Inside);
    }

    // Subtle 1:1 ePlan tag in corner
    painter.text(
        Pos2::new(rect.max.x - 5.0, rect.min.y + 3.0),
        Align2::RIGHT_TOP,
        "1:1 ePlan",
        FontId::monospace(8.5),
        t.text_muted,
    );

    painter.rect_stroke(
        rect,
        CornerRadius::same(6),
        Stroke::new(1.0, t.border_hairline),
        StrokeKind::Inside,
    );
}




