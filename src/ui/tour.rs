use std::collections::HashMap;

use egui::{
    Align, Color32, Context, CornerRadius, Frame, Key, Layout, Margin, Order, Pos2, Rect, RichText,
    Sense, Stroke, StrokeKind, Vec2,
};

use crate::i18n::{Language, tr};
use crate::ui::icons::{UiIcon, ghost_icon_button, icon_button, primary_icon_button};
use crate::ui::theme::{ThemeTokens, tokens_ctx};

/// Identifies which of the two guided onboarding tours is running.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TourId {
    Basic,
    Advanced,
}

/// Stable semantic target anchors bound by UI views each frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TourTarget {
    CreateStripControl,
    CadCanvas,
    MultiLevelControls,
    ExcelExportButton,
    InspectorReassign,
    SpecialNumbering,
    ProjectToolbar,
    LanguageAndTheme,
}

/// Preferred card placement relative to the highlighted target element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TourPlacement {
    Center,
    Auto,
    Top,
    Bottom,
    Left,
    Right,
}

/// Card template variant for welcome, standard step, or dual-CTA outro.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TourCardKind {
    Welcome,
    Step,
    Outro,
}

/// Lifecycle preparation hook invoked right before entering a step (`onBeforeStep`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TourPrepareHook {
    ShowEditorAndStripControls,
    FocusCadPreview,
    FocusMultiLevelSample,
    ShowExcelExportReady,
    SelectMixedRangeInCad,
    FocusInspectorReassign,
    FocusSpecialNumbering,
    FocusHeaderTools,
}

/// Data-driven definition for a single guided tour step.
#[derive(Clone, Copy)]
pub struct TourStepDef {
    pub kind: TourCardKind,
    pub target: Option<TourTarget>,
    pub placement: TourPlacement,
    pub title: fn(Language) -> &'static str,
    pub body: fn(Language) -> &'static str,
    pub on_before_step: Option<TourPrepareHook>,
}

/// Frame-scoped registry collecting live screen-space rectangles for tour targets.
#[derive(Debug, Default)]
pub struct TourTargetRegistry {
    rects: HashMap<TourTarget, Rect>,
}

impl TourTargetRegistry {
    pub fn clear(&mut self) {
        self.rects.clear();
    }

    pub fn record(&mut self, target: TourTarget, rect: Rect) {
        if rect.width() > 1.0 && rect.height() > 1.0 {
            self.rects
                .entry(target)
                .and_modify(|existing| *existing = existing.union(rect))
                .or_insert(rect);
        }
    }

    pub fn get(&self, target: TourTarget) -> Option<Rect> {
        self.rects.get(&target).copied()
    }
}

/// Outcome returned by the tour renderer so the host app can react cleanly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TourAction {
    None,
    PrepareStep(TourPrepareHook),
    SwitchToAdvanced,
    Closed { completed: bool },
}

/// Stateful controller for the active tour, step index, and safe demo state lifecycle.
#[derive(Debug, Default)]
pub struct TourState {
    pub active_tour: Option<TourId>,
    pub step_idx: usize,
    pub needs_prepare: bool,
    pub has_seen_onboarding: bool,
    pub targets: TourTargetRegistry,
    pub demo_active: bool,
}

impl TourState {
    pub fn is_active(&self) -> bool {
        self.active_tour.is_some()
    }

    pub fn start(&mut self, tour: TourId) {
        self.active_tour = Some(tour);
        self.step_idx = 0;
        self.needs_prepare = true;
    }

    pub fn close(&mut self) {
        self.active_tour = None;
        self.step_idx = 0;
        self.needs_prepare = false;
    }

    pub fn current_steps(&self) -> Option<&'static [TourStepDef]> {
        self.active_tour.map(steps_for_tour)
    }

    pub fn current_step(&self) -> Option<&'static TourStepDef> {
        let steps = self.current_steps()?;
        steps.get(self.step_idx)
    }

    pub fn next_step_with_hook(&mut self, on_before_step: &mut dyn FnMut(TourPrepareHook)) -> bool {
        let Some(steps) = self.current_steps() else {
            return false;
        };
        if self.step_idx + 1 < steps.len() {
            self.step_idx += 1;
            self.needs_prepare = true;
            if let Some(hook) = steps[self.step_idx].on_before_step {
                on_before_step(hook);
                self.needs_prepare = false;
            }
            true
        } else {
            false
        }
    }

    pub fn prev_step_with_hook(&mut self, on_before_step: &mut dyn FnMut(TourPrepareHook)) -> bool {
        let Some(steps) = self.current_steps() else {
            return false;
        };
        if self.step_idx > 0 {
            self.step_idx -= 1;
            self.needs_prepare = true;
            if let Some(hook) = steps[self.step_idx].on_before_step {
                on_before_step(hook);
                self.needs_prepare = false;
            }
            true
        } else {
            false
        }
    }
}

pub const BASIC_TOUR_STEPS: &[TourStepDef] = &[
    TourStepDef {
        kind: TourCardKind::Welcome,
        target: None,
        placement: TourPlacement::Center,
        title: tr::tour_basic_1_title,
        body: tr::tour_basic_1_body,
        on_before_step: None,
    },
    TourStepDef {
        kind: TourCardKind::Step,
        target: Some(TourTarget::CreateStripControl),
        placement: TourPlacement::Right,
        title: tr::tour_basic_2_title,
        body: tr::tour_basic_2_body,
        on_before_step: Some(TourPrepareHook::ShowEditorAndStripControls),
    },
    TourStepDef {
        kind: TourCardKind::Step,
        target: Some(TourTarget::CadCanvas),
        placement: TourPlacement::Top,
        title: tr::tour_basic_3_title,
        body: tr::tour_basic_3_body,
        on_before_step: Some(TourPrepareHook::FocusCadPreview),
    },
    TourStepDef {
        kind: TourCardKind::Step,
        target: Some(TourTarget::MultiLevelControls),
        placement: TourPlacement::Bottom,
        title: tr::tour_basic_4_title,
        body: tr::tour_basic_4_body,
        on_before_step: Some(TourPrepareHook::FocusMultiLevelSample),
    },
    TourStepDef {
        kind: TourCardKind::Step,
        target: Some(TourTarget::ExcelExportButton),
        placement: TourPlacement::Bottom,
        title: tr::tour_basic_5_title,
        body: tr::tour_basic_5_body,
        on_before_step: Some(TourPrepareHook::ShowExcelExportReady),
    },
    TourStepDef {
        kind: TourCardKind::Outro,
        target: None,
        placement: TourPlacement::Center,
        title: tr::tour_basic_6_title,
        body: tr::tour_basic_6_body,
        on_before_step: None,
    },
];

pub const ADVANCED_TOUR_STEPS: &[TourStepDef] = &[
    TourStepDef {
        kind: TourCardKind::Step,
        target: Some(TourTarget::CadCanvas),
        placement: TourPlacement::Top,
        title: tr::tour_adv_1_title,
        body: tr::tour_adv_1_body,
        on_before_step: Some(TourPrepareHook::SelectMixedRangeInCad),
    },
    TourStepDef {
        kind: TourCardKind::Step,
        target: Some(TourTarget::InspectorReassign),
        placement: TourPlacement::Left,
        title: tr::tour_adv_2_title,
        body: tr::tour_adv_2_body,
        on_before_step: Some(TourPrepareHook::FocusInspectorReassign),
    },
    TourStepDef {
        kind: TourCardKind::Step,
        target: Some(TourTarget::SpecialNumbering),
        placement: TourPlacement::Left,
        title: tr::tour_adv_3_title,
        body: tr::tour_adv_3_body,
        on_before_step: Some(TourPrepareHook::FocusSpecialNumbering),
    },
    TourStepDef {
        kind: TourCardKind::Step,
        target: Some(TourTarget::ProjectToolbar),
        placement: TourPlacement::Bottom,
        title: tr::tour_adv_4_title,
        body: tr::tour_adv_4_body,
        on_before_step: Some(TourPrepareHook::FocusHeaderTools),
    },
    TourStepDef {
        kind: TourCardKind::Step,
        target: Some(TourTarget::LanguageAndTheme),
        placement: TourPlacement::Bottom,
        title: tr::tour_adv_5_title,
        body: tr::tour_adv_5_body,
        on_before_step: Some(TourPrepareHook::FocusHeaderTools),
    },
];

pub fn steps_for_tour(tour: TourId) -> &'static [TourStepDef] {
    match tour {
        TourId::Basic => BASIC_TOUR_STEPS,
        TourId::Advanced => ADVANCED_TOUR_STEPS,
    }
}

/// Computes an unobstructed, viewport-clamped popover rectangle near `target_rect` (or centered).
pub fn compute_popover_rect(
    screen_rect: Rect,
    target_rect: Option<Rect>,
    placement: TourPlacement,
    card_size: Vec2,
) -> Rect {
    let margin = 12.0;
    let gap = 14.0;
    let clamped_w = card_size.x.min((screen_rect.width() - margin * 2.0).max(240.0));
    let clamped_h = card_size.y.min((screen_rect.height() - margin * 2.0).max(160.0));
    let size = Vec2::new(clamped_w, clamped_h);

    let min_x = screen_rect.min.x + margin;
    let max_x = (screen_rect.max.x - margin - size.x).max(min_x);
    let min_y = screen_rect.min.y + margin;
    let max_y = (screen_rect.max.y - margin - size.y).max(min_y);

    let center_rect = || {
        let pos = Pos2::new(
            (screen_rect.center().x - size.x * 0.5).clamp(min_x, max_x),
            (screen_rect.center().y - size.y * 0.5).clamp(min_y, max_y),
        );
        Rect::from_min_size(pos, size)
    };

    if placement == TourPlacement::Center {
        return center_rect();
    }
    let Some(target) = target_rect else {
        return center_rect();
    };

    // Order candidate sides cleanly according to preferred placement
    let candidates = match placement {
        TourPlacement::Right => [
            TourPlacement::Right,
            TourPlacement::Left,
            TourPlacement::Bottom,
            TourPlacement::Top,
        ],
        TourPlacement::Left => [
            TourPlacement::Left,
            TourPlacement::Right,
            TourPlacement::Bottom,
            TourPlacement::Top,
        ],
        TourPlacement::Top => [
            TourPlacement::Top,
            TourPlacement::Bottom,
            TourPlacement::Right,
            TourPlacement::Left,
        ],
        TourPlacement::Bottom | TourPlacement::Auto => [
            TourPlacement::Bottom,
            TourPlacement::Top,
            TourPlacement::Right,
            TourPlacement::Left,
        ],
        TourPlacement::Center => return center_rect(),
    };

    let space_right = screen_rect.max.x - margin - (target.max.x + gap);
    let space_left = (target.min.x - gap) - (screen_rect.min.x + margin);
    let space_bottom = screen_rect.max.y - margin - (target.max.y + gap);
    let space_top = (target.min.y - gap) - (screen_rect.min.y + margin);

    let rect_for_side = |side: TourPlacement| -> Rect {
        let pos = match side {
            TourPlacement::Right => Pos2::new(
                (target.max.x + gap).clamp(min_x, max_x),
                (target.center().y - size.y * 0.5).clamp(min_y, max_y),
            ),
            TourPlacement::Left => Pos2::new(
                (target.min.x - gap - size.x).clamp(min_x, max_x),
                (target.center().y - size.y * 0.5).clamp(min_y, max_y),
            ),
            TourPlacement::Bottom => Pos2::new(
                (target.center().x - size.x * 0.5).clamp(min_x, max_x),
                (target.max.y + gap).clamp(min_y, max_y),
            ),
            TourPlacement::Top => Pos2::new(
                (target.center().x - size.x * 0.5).clamp(min_x, max_x),
                (target.min.y - gap - size.y).clamp(min_y, max_y),
            ),
            _ => center_rect().min,
        };
        Rect::from_min_size(pos, size)
    };

    for side in candidates {
        let fits = match side {
            TourPlacement::Right => space_right >= size.x,
            TourPlacement::Left => space_left >= size.x,
            TourPlacement::Bottom => space_bottom >= size.y,
            TourPlacement::Top => space_top >= size.y,
            _ => false,
        };
        if fits {
            let candidate_rect = rect_for_side(side);
            if !candidate_rect.intersects(target) {
                return candidate_rect;
            }
        }
    }

    // Fallback on compact viewports: pick the non-overlapping / largest-clearance vertical or horizontal side
    let best_side = [
        (TourPlacement::Bottom, space_bottom),
        (TourPlacement::Top, space_top),
        (TourPlacement::Right, space_right),
        (TourPlacement::Left, space_left),
    ]
    .into_iter()
    .max_by(|a, b| a.1.total_cmp(&b.1))
    .map(|(s, _)| s)
    .unwrap_or(TourPlacement::Bottom);

    rect_for_side(best_side)
}

/// Renders the active Spotlight scrim + Smart Popover Card and returns a `TourAction` if navigation or exit happened.
pub fn show_tour_overlay(ctx: &Context, state: &mut TourState, lang: Language) -> TourAction {
    let Some(tour_id) = state.active_tour else {
        return TourAction::None;
    };

    if state.needs_prepare {
        state.needs_prepare = false;
        if let Some(step) = state.current_step()
            && let Some(hook) = step.on_before_step
        {
            return TourAction::PrepareStep(hook);
        }
    }

    let steps = steps_for_tour(tour_id);
    if state.step_idx >= steps.len() {
        state.close();
        return TourAction::Closed { completed: true };
    }
    let step = &steps[state.step_idx];
    let total = steps.len();
    let step_num = state.step_idx + 1;
    let is_last = step_num == total;

    // Keyboard shortcuts while the tour overlay is visible
    let wants_kb = ctx.egui_wants_keyboard_input();
    let (esc_pressed, next_key, prev_key) = ctx.input(|i| {
        (
            i.key_pressed(Key::Escape),
            !wants_kb && (i.key_pressed(Key::ArrowRight) || i.key_pressed(Key::Enter)),
            !wants_kb && i.key_pressed(Key::ArrowLeft),
        )
    });

    if esc_pressed {
        state.close();
        return TourAction::Closed { completed: false };
    }

    let screen_rect = ctx.content_rect();
    let t = tokens_ctx(ctx);
    let dark = t.dark;

    let raw_target_rect = step.target.and_then(|target_id| state.targets.get(target_id));
    let spotlight_rect = raw_target_rect.map(|r| {
        r.expand(6.0)
            .intersect(screen_rect.shrink(2.0))
    });

    // 1) Spotlight Overlay Scrim (Order::Foreground sits cleanly above panels & Middle modals, below Tooltip popover)
    draw_spotlight_scrim(ctx, screen_rect, spotlight_rect, &t, dark);

    // 2) Smart Popover Card (Order::Tooltip guarantees the card is always crisp & interactive above the scrim)
    let card_w = (screen_rect.width() - 24.0).clamp(260.0, 416.0);
    let est_h = if step.kind == TourCardKind::Step {
        265.0
    } else {
        290.0
    };
    let placement = if spotlight_rect.is_some() {
        step.placement
    } else {
        TourPlacement::Center
    };
    let anchor_rect = compute_popover_rect(screen_rect, spotlight_rect, placement, Vec2::new(card_w, est_h));

    let mut action = TourAction::None;
    let mut go_next = next_key && step.kind != TourCardKind::Outro;
    let mut finish_from_outro = next_key && step.kind == TourCardKind::Outro;
    let mut go_prev = prev_key;
    let mut skip_clicked = false;
    let mut switch_advanced = false;

    egui::Area::new(egui::Id::unique("termi_tour_popover_card"))
        .order(Order::Tooltip)
        .fixed_pos(anchor_rect.min)
        .show(ctx, |ui| {
            ui.set_width(card_w);
            ui.set_max_width(card_w);

            Frame::NONE
                .fill(t.bg_surface_elevated)
                .stroke(Stroke::new(1.25, t.accent))
                .corner_radius(CornerRadius::same(12))
                .inner_margin(Margin::same(16))
                .shadow(egui::Shadow {
                    offset: [0, 14],
                    blur: 36,
                    spread: 2,
                    color: Color32::from_black_alpha(if dark { 170 } else { 64 }),
                })
                .show(ui, |ui| {
                    // Header row: tour badge + tabular step counter
                    ui.horizontal(|ui| {
                        let badge_text = match tour_id {
                            TourId::Basic => tr::tour_badge_basic(lang),
                            TourId::Advanced => tr::tour_badge_advanced(lang),
                        };
                        Frame::NONE
                            .fill(t.accent_soft)
                            .stroke(Stroke::new(1.0, t.border_hairline))
                            .corner_radius(CornerRadius::same(99))
                            .inner_margin(Margin::symmetric(9, 3))
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(badge_text)
                                        .size(11.5)
                                        .strong()
                                        .color(t.accent_text),
                                );
                            });

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(
                                RichText::new(tr::tour_step_counter(lang, step_num, total))
                                    .size(12.0)
                                    .monospace()
                                    .color(t.text_muted),
                            );
                        });
                    });

                    ui.add_space(10.0);

                    // Step title & body
                    ui.add(
                        egui::Label::new(
                            RichText::new((step.title)(lang))
                                .size(16.5)
                                .strong()
                                .color(t.text_primary),
                        )
                        .wrap(),
                    );
                    ui.add_space(6.0);
                    ui.add(
                        egui::Label::new(
                            RichText::new((step.body)(lang))
                                .size(13.2)
                                .color(t.text_secondary),
                        )
                        .wrap(),
                    );

                    ui.add_space(12.0);

                    // Visual progress indicator dots
                    draw_progress_dots(ui, state.step_idx, total, &t);

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // Footer Controls: Back / Skip / Next / Finish / Outro Dual-CTAs
                    ui.horizontal_wrapped(|ui| {
                        if step.kind != TourCardKind::Welcome && state.step_idx > 0
                            && ghost_icon_button(ui, UiIcon::MoveLeft, tr::tour_btn_back(lang))
                                .clicked()
                        {
                            go_prev = true;
                        }

                        if ghost_icon_button(ui, UiIcon::Close, tr::tour_btn_skip(lang)).clicked() {
                            skip_clicked = true;
                        }

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            match step.kind {
                                TourCardKind::Welcome => {
                                    if primary_icon_button(
                                        ui,
                                        UiIcon::Guide,
                                        tr::tour_start_basic(lang),
                                    )
                                    .clicked()
                                    {
                                        go_next = true;
                                    }
                                }
                                TourCardKind::Outro => {
                                    if primary_icon_button(
                                        ui,
                                        UiIcon::Check,
                                        tr::tour_btn_finish_work(lang),
                                    )
                                    .clicked()
                                    {
                                        finish_from_outro = true;
                                    }
                                    if icon_button(
                                        ui,
                                        UiIcon::Bolt,
                                        tr::tour_start_advanced(lang),
                                    )
                                    .clicked()
                                    {
                                        switch_advanced = true;
                                    }
                                }
                                TourCardKind::Step => {
                                    let cta_label = if is_last {
                                        tr::tour_btn_finish(lang)
                                    } else {
                                        tr::tour_btn_next(lang)
                                    };
                                    let cta_icon = if is_last {
                                        UiIcon::Check
                                    } else {
                                        UiIcon::MoveRight
                                    };
                                    if primary_icon_button(ui, cta_icon, cta_label).clicked() {
                                        go_next = true;
                                    }
                                }
                            }
                        });
                    });

                    if card_w >= 310.0 {
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(tr::tour_shortcut_hint(lang))
                                .size(11.0)
                                .color(t.text_muted),
                        );
                    }
                });
        });

    if skip_clicked {
        state.close();
        return TourAction::Closed { completed: false };
    }
    if switch_advanced {
        state.start(TourId::Advanced);
        return TourAction::SwitchToAdvanced;
    }
    if finish_from_outro {
        state.close();
        return TourAction::Closed { completed: true };
    }
    if go_prev && state.step_idx > 0 {
        state.step_idx -= 1;
        state.needs_prepare = true;
        if let Some(hook) = steps[state.step_idx].on_before_step {
            state.needs_prepare = false;
            action = TourAction::PrepareStep(hook);
        }
    } else if go_next {
        if state.step_idx + 1 < total {
            state.step_idx += 1;
            state.needs_prepare = true;
            if let Some(hook) = steps[state.step_idx].on_before_step {
                state.needs_prepare = false;
                action = TourAction::PrepareStep(hook);
            }
        } else {
            state.close();
            return TourAction::Closed { completed: true };
        }
    }

    action
}

fn draw_progress_dots(ui: &mut egui::Ui, active_idx: usize, total: usize, t: &ThemeTokens) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        for idx in 0..total {
            let is_active = idx == active_idx;
            let is_done = idx < active_idx;
            let dot_w = if is_active { 22.0 } else { 8.0 };
            let (rect, _) = ui.allocate_exact_size(Vec2::new(dot_w, 6.0), Sense::hover());
            let fill = if is_active {
                t.accent
            } else if is_done {
                t.success
            } else {
                t.border_strong
            };
            ui.painter()
                .rect_filled(rect, CornerRadius::same(3), fill);
        }
    });
}

fn draw_spotlight_scrim(
    ctx: &Context,
    screen_rect: Rect,
    spotlight_rect: Option<Rect>,
    t: &ThemeTokens,
    dark: bool,
) {
    let scrim_color = Color32::from_black_alpha(if dark { 166 } else { 132 });

    egui::Area::new(egui::Id::unique("termi_tour_spotlight_scrim"))
        .order(Order::Foreground)
        .fixed_pos(screen_rect.min)
        .interactable(true)
        .show(ctx, |ui| {
            ui.set_min_size(screen_rect.size());
            ui.set_max_size(screen_rect.size());

            if let Some(cut) = spotlight_rect {
                // Paint 4 surrounding dark panels leaving the spotlight cutout crystal-clear
                let strips = [
                    // Top bar
                    Rect::from_min_max(screen_rect.min, Pos2::new(screen_rect.max.x, cut.min.y)),
                    // Bottom bar
                    Rect::from_min_max(Pos2::new(screen_rect.min.x, cut.max.y), screen_rect.max),
                    // Left middle strip
                    Rect::from_min_max(
                        Pos2::new(screen_rect.min.x, cut.min.y),
                        Pos2::new(cut.min.x, cut.max.y),
                    ),
                    // Right middle strip
                    Rect::from_min_max(
                        Pos2::new(cut.max.x, cut.min.y),
                        Pos2::new(screen_rect.max.x, cut.max.y),
                    ),
                ];

                for strip in strips {
                    if strip.width() > 0.5 && strip.height() > 0.5 {
                        ui.painter()
                            .rect_filled(strip, CornerRadius::ZERO, scrim_color);
                        let _ = ui.allocate_rect(strip, Sense::click());
                    }
                }

                // Soft glowing outer halo + sharp accent border around target cutout
                let halo_rect = cut.expand(3.5);
                let halo_color =
                    Color32::from_rgba_unmultiplied(t.accent.r(), t.accent.g(), t.accent.b(), 64);
                let painter = ui.painter();
                painter.rect_stroke(
                    halo_rect,
                    CornerRadius::same(11),
                    Stroke::new(4.0, halo_color),
                    StrokeKind::Outside,
                );
                painter.rect_stroke(
                    cut,
                    CornerRadius::same(8),
                    Stroke::new(2.0, t.accent),
                    StrokeKind::Outside,
                );
            } else {
                ui.painter()
                    .rect_filled(screen_rect, CornerRadius::ZERO, scrim_color);
                let _ = ui.allocate_rect(screen_rect, Sense::click());
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tour_definitions_have_complete_steps_and_localized_strings() {
        assert_eq!(BASIC_TOUR_STEPS.len(), 6);
        assert_eq!(ADVANCED_TOUR_STEPS.len(), 5);

        let langs = [
            Language::De,
            Language::En,
            Language::Fr,
            Language::It,
            Language::Es,
            Language::Pl,
            Language::Nl,
        ];
        for lang in langs {
            for step in BASIC_TOUR_STEPS.iter().chain(ADVANCED_TOUR_STEPS.iter()) {
                assert!(!(step.title)(lang).trim().is_empty());
                assert!(!(step.body)(lang).trim().is_empty());
            }
            assert!(!tr::tour_start_basic(lang).trim().is_empty());
            assert!(!tr::tour_start_advanced(lang).trim().is_empty());
            assert!(!tr::tour_btn_skip(lang).trim().is_empty());
        }
    }

    #[test]
    fn tour_navigation_runs_lifecycle_hooks_and_clamps_bounds() {
        let mut state = TourState::default();
        assert!(!state.is_active());

        state.start(TourId::Basic);
        assert!(state.is_active());
        assert_eq!(state.step_idx, 0);

        let mut recorded_hooks = Vec::new();
        assert!(!state.prev_step_with_hook(&mut |h| recorded_hooks.push(h)));
        assert_eq!(state.step_idx, 0);

        for expected_idx in 1..BASIC_TOUR_STEPS.len() {
            assert!(state.next_step_with_hook(&mut |h| recorded_hooks.push(h)));
            assert_eq!(state.step_idx, expected_idx);
        }
        // Step 6 is Outro -> next_step returns false cleanly
        assert!(!state.next_step_with_hook(&mut |h| recorded_hooks.push(h)));
        assert!(recorded_hooks.contains(&TourPrepareHook::ShowEditorAndStripControls));
        assert!(recorded_hooks.contains(&TourPrepareHook::ShowExcelExportReady));

        assert!(state.prev_step_with_hook(&mut |_| {}));
        assert_eq!(state.step_idx, BASIC_TOUR_STEPS.len() - 2);

        state.close();
        assert!(!state.is_active());
    }

    #[test]
    fn smart_popover_stays_inside_viewport_and_avoids_target_when_room_exists() {
        let screen = Rect::from_min_size(Pos2::ZERO, Vec2::new(1280.0, 800.0));
        let card_size = Vec2::new(380.0, 260.0);

        // Centered modal
        let centered = compute_popover_rect(screen, None, TourPlacement::Center, card_size);
        assert!(screen.contains_rect(centered));
        assert!((centered.center().x - 640.0).abs() < 1.0);

        // Target near right edge requesting Right -> should auto-flip to Left
        let right_target = Rect::from_min_size(Pos2::new(980.0, 120.0), Vec2::new(280.0, 420.0));
        let flipped = compute_popover_rect(screen, Some(right_target), TourPlacement::Right, card_size);
        assert!(screen.contains_rect(flipped));
        assert!(!flipped.intersects(right_target));
        assert!(flipped.max.x < right_target.min.x);

        // Small mobile viewport -> still clamped cleanly within viewport bounds
        let mobile = Rect::from_min_size(Pos2::ZERO, Vec2::new(390.0, 844.0));
        let target_top = Rect::from_min_size(Pos2::new(12.0, 90.0), Vec2::new(366.0, 80.0));
        let mobile_card = compute_popover_rect(mobile, Some(target_top), TourPlacement::Top, Vec2::new(360.0, 250.0));
        assert!(mobile.contains_rect(mobile_card));
        assert!(mobile_card.min.y >= target_top.max.y);
    }
}
