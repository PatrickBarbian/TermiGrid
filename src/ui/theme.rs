use std::sync::Arc;

use egui::{
    Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Frame, Margin, Shadow,
    Stroke, TextStyle, Theme, Vec2, Visuals,
};

/// Loads crisp native system fonts (Windows/macOS/Android) with egui defaults as fallback.
pub fn install_system_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    let proportional_candidates = [
        r"C:\Windows\Fonts\segoeui.ttf",
        "/System/Library/Fonts/SFNS.ttf",
        "/System/Library/Fonts/Supplemental/Arial.ttf",
        "/system/fonts/Roboto-Regular.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ];
    for path in proportional_candidates {
        if let Ok(bytes) = std::fs::read(path) {
            fonts
                .font_data
                .insert("system_sans".to_owned(), Arc::new(FontData::from_owned(bytes)));
            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, "system_sans".to_owned());
            break;
        }
    }

    let mono_candidates = [
        r"C:\Windows\Fonts\consola.ttf",
        "/System/Library/Fonts/Supplemental/Courier New.ttf",
        "/System/Library/Fonts/Monaco.ttf",
        "/system/fonts/DroidSansMono.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
    ];
    for path in mono_candidates {
        if let Ok(bytes) = std::fs::read(path) {
            fonts
                .font_data
                .insert("system_mono".to_owned(), Arc::new(FontData::from_owned(bytes)));
            fonts
                .families
                .entry(FontFamily::Monospace)
                .or_default()
                .insert(0, "system_mono".to_owned());
            break;
        }
    }

    ctx.set_fonts(fonts);
}

/// Calibrated 60/30/10 design tokens for TermiGrid's Light & Dark themes.
#[derive(Debug, Clone, Copy)]
pub struct ThemeTokens {
    pub dark: bool,
    pub bg_canvas: Color32,
    pub bg_panel: Color32,
    pub bg_surface: Color32,
    pub bg_surface_hover: Color32,
    pub bg_surface_elevated: Color32,
    pub bg_recessed: Color32,
    pub border_hairline: Color32,
    pub border_strong: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub accent_active: Color32,
    pub accent_soft: Color32,
    pub accent_text: Color32,
    pub success: Color32,
    pub success_hover: Color32,
    pub success_soft: Color32,
    pub danger: Color32,
    pub danger_hover: Color32,
    pub danger_soft: Color32,
    pub warning: Color32,
    pub warning_soft: Color32,
}

impl ThemeTokens {
    pub fn for_mode(dark: bool) -> Self {
        if dark {
            Self {
                dark: true,
                bg_canvas: Color32::from_rgb(11, 14, 20),
                bg_panel: Color32::from_rgb(17, 21, 30),
                bg_surface: Color32::from_rgb(24, 29, 41),
                bg_surface_hover: Color32::from_rgb(33, 40, 56),
                bg_surface_elevated: Color32::from_rgb(29, 35, 49),
                bg_recessed: Color32::from_rgb(13, 16, 23),
                border_hairline: Color32::from_rgba_unmultiplied(255, 255, 255, 22),
                border_strong: Color32::from_rgba_unmultiplied(255, 255, 255, 48),
                text_primary: Color32::from_rgb(241, 245, 249),
                text_secondary: Color32::from_rgb(163, 177, 198),
                text_muted: Color32::from_rgb(107, 122, 144),
                accent: Color32::from_rgb(14, 165, 233),
                accent_hover: Color32::from_rgb(56, 189, 248),
                accent_active: Color32::from_rgb(2, 132, 199),
                accent_soft: Color32::from_rgba_unmultiplied(14, 165, 233, 34),
                accent_text: Color32::from_rgb(125, 211, 252),
                success: Color32::from_rgb(22, 163, 74),
                success_hover: Color32::from_rgb(34, 197, 94),
                success_soft: Color32::from_rgba_unmultiplied(34, 197, 94, 30),
                danger: Color32::from_rgb(220, 38, 38),
                danger_hover: Color32::from_rgb(239, 68, 68),
                danger_soft: Color32::from_rgba_unmultiplied(239, 68, 68, 28),
                warning: Color32::from_rgb(245, 158, 11),
                warning_soft: Color32::from_rgba_unmultiplied(245, 158, 11, 28),
            }
        } else {
            Self {
                dark: false,
                bg_canvas: Color32::from_rgb(241, 244, 249),
                bg_panel: Color32::from_rgb(255, 255, 255),
                bg_surface: Color32::from_rgb(248, 250, 252),
                bg_surface_hover: Color32::from_rgb(236, 242, 249),
                bg_surface_elevated: Color32::from_rgb(255, 255, 255),
                bg_recessed: Color32::from_rgb(232, 237, 244),
                border_hairline: Color32::from_rgb(224, 230, 239),
                border_strong: Color32::from_rgb(194, 204, 218),
                text_primary: Color32::from_rgb(15, 23, 42),
                text_secondary: Color32::from_rgb(71, 85, 105),
                text_muted: Color32::from_rgb(100, 116, 139),
                accent: Color32::from_rgb(2, 132, 199),
                accent_hover: Color32::from_rgb(14, 165, 233),
                accent_active: Color32::from_rgb(3, 105, 161),
                accent_soft: Color32::from_rgba_unmultiplied(2, 132, 199, 24),
                accent_text: Color32::from_rgb(3, 105, 161),
                success: Color32::from_rgb(22, 163, 74),
                success_hover: Color32::from_rgb(21, 128, 61),
                success_soft: Color32::from_rgba_unmultiplied(22, 163, 74, 22),
                danger: Color32::from_rgb(220, 38, 38),
                danger_hover: Color32::from_rgb(185, 28, 28),
                danger_soft: Color32::from_rgba_unmultiplied(220, 38, 38, 20),
                warning: Color32::from_rgb(217, 119, 6),
                warning_soft: Color32::from_rgba_unmultiplied(217, 119, 6, 22),
            }
        }
    }
}

#[inline]
pub fn tokens(ui: &egui::Ui) -> ThemeTokens {
    ThemeTokens::for_mode(ui.visuals().dark_mode)
}

#[inline]
pub fn tokens_ctx(ctx: &egui::Context) -> ThemeTokens {
    ThemeTokens::for_mode(ctx.theme() == egui::Theme::Dark)
}

/// User-selectable color theme mode; defaults to OS/System setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

impl ThemeMode {
    pub fn next(self) -> Self {
        match self {
            Self::System => Self::Light,
            Self::Light => Self::Dark,
            Self::Dark => Self::System,
        }
    }
}

/// Applies calm, high-craft visuals for BOTH Light and Dark mode plus `ThemeMode` preference
/// (defaulting to OS/System theme).
pub fn apply_theme_mode(ctx: &egui::Context, mode: ThemeMode) {
    apply_termi_theme(ctx, true);
    apply_termi_theme(ctx, false);

    let pref = match mode {
        ThemeMode::System => egui::ThemePreference::System,
        ThemeMode::Light => egui::ThemePreference::Light,
        ThemeMode::Dark => egui::ThemePreference::Dark,
    };
    ctx.set_theme(pref);
}

/// Applies calibrated typography, hairline surfaces, and low-noise visual hierarchy.
pub fn apply_termi_theme(ctx: &egui::Context, dark: bool) {
    let theme = if dark { Theme::Dark } else { Theme::Light };
    let t = ThemeTokens::for_mode(dark);
    let mut visuals = if dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };

    visuals.panel_fill = t.bg_panel;
    visuals.window_fill = t.bg_surface_elevated;
    visuals.extreme_bg_color = t.bg_recessed;
    visuals.faint_bg_color = t.bg_surface;
    visuals.window_stroke = Stroke::new(1.0, t.border_strong);
    visuals.window_corner_radius = CornerRadius::same(10);
    visuals.window_shadow = Shadow {
        offset: [0, 10],
        blur: 28,
        spread: 0,
        color: Color32::from_black_alpha(if dark { 115 } else { 38 }),
    };
    visuals.popup_shadow = Shadow {
        offset: [0, 6],
        blur: 18,
        spread: 0,
        color: Color32::from_black_alpha(if dark { 95 } else { 28 }),
    };

    visuals.widgets.noninteractive.bg_fill = t.bg_panel;
    visuals.widgets.noninteractive.weak_bg_fill = t.bg_surface;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.border_hairline);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, t.text_primary);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(6);

    visuals.widgets.inactive.bg_fill = t.bg_surface;
    visuals.widgets.inactive.weak_bg_fill = t.bg_surface;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, t.border_hairline);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, t.text_primary);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(6);

    visuals.widgets.hovered.bg_fill = t.bg_surface_hover;
    visuals.widgets.hovered.weak_bg_fill = t.bg_surface_hover;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, t.border_strong);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, t.text_primary);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(6);

    visuals.widgets.active.bg_fill = t.accent_soft;
    visuals.widgets.active.weak_bg_fill = t.accent_soft;
    visuals.widgets.active.bg_stroke = Stroke::new(1.2, t.accent);
    visuals.widgets.active.fg_stroke = Stroke::new(1.5, t.text_primary);
    visuals.widgets.active.corner_radius = CornerRadius::same(6);

    visuals.widgets.open.bg_fill = t.bg_surface_hover;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, t.accent);
    visuals.widgets.open.corner_radius = CornerRadius::same(6);

    visuals.selection.bg_fill = t.accent;
    visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.text_cursor.stroke = Stroke::new(1.5, t.accent);

    ctx.set_visuals_of(theme, visuals);
    ctx.all_styles_mut(|style| {
        style
            .text_styles
            .insert(TextStyle::Body, FontId::proportional(14.0));
        style
            .text_styles
            .insert(TextStyle::Button, FontId::proportional(13.5));
        style
            .text_styles
            .insert(TextStyle::Small, FontId::proportional(12.0));
        style
            .text_styles
            .insert(TextStyle::Monospace, FontId::monospace(13.5));
        style
            .text_styles
            .insert(TextStyle::Heading, FontId::proportional(18.0));
        style.spacing.item_spacing = Vec2::new(8.0, 6.0);
        style.spacing.window_margin = Margin::same(14);
        if cfg!(target_os = "android") {
            style.spacing.button_padding = Vec2::new(12.0, 8.0);
            style.spacing.interact_size = Vec2::new(42.0, 34.0);
            style.spacing.scroll.bar_width = 10.0;
            style.spacing.scroll.handle_min_length = 28.0;
            style.interaction.selectable_labels = false;
        } else {
            style.spacing.button_padding = Vec2::new(10.0, 5.5);
            style.spacing.interact_size = Vec2::new(36.0, 26.0);
            style.spacing.scroll.bar_width = 7.0;
            style.spacing.scroll.handle_min_length = 24.0;
        }
    });
}

/// High-contrast secondary text color (replaces faint `.weak()`).
pub fn secondary_text_color(ui: &egui::Ui) -> Color32 {
    tokens(ui).text_secondary
}

/// Clean hairline section card with consistent surface fill and generous inner padding.
pub fn section_card<R>(
    ui: &mut egui::Ui,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    let t = tokens(ui);
    Frame::NONE
        .fill(t.bg_surface)
        .stroke(Stroke::new(1.0, t.border_hairline))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(Margin::same(10))
        .show(ui, add_contents)
}

/// Slightly recessed container card for nested summaries or subtle callouts.
pub fn recessed_card<R>(
    ui: &mut egui::Ui,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    let t = tokens(ui);
    Frame::NONE
        .fill(t.bg_recessed)
        .stroke(Stroke::new(1.0, t.border_hairline))
        .corner_radius(CornerRadius::same(7))
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, add_contents)
}

/// Compact keyboard shortcut keycap pill (`Strg+K`, `Entf`, `Strg+Z`).
pub fn kbd_chip(ui: &mut egui::Ui, text: &str) {
    let t = tokens(ui);
    Frame::NONE
        .fill(t.bg_recessed)
        .stroke(Stroke::new(1.0, t.border_hairline))
        .corner_radius(CornerRadius::same(4))
        .inner_margin(Margin::symmetric(5, 1))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(text)
                    .monospace()
                    .size(11.0)
                    .color(t.text_secondary),
            );
        });
}

/// Subtle semantic status pill badge.
pub fn pill_badge(ui: &mut egui::Ui, text: &str, fg: Color32, bg: Color32) {
    Frame::NONE
        .fill(bg)
        .corner_radius(CornerRadius::same(99))
        .inner_margin(Margin::symmetric(8, 2))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(text).color(fg).strong().size(11.5));
        });
}
