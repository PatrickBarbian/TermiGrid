use std::sync::Arc;

use egui::{
    Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle,
    Theme, Vec2, Visuals,
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

/// Applies high-contrast visuals for BOTH Light and Dark mode plus `ThemeMode` preference
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

/// Applies a high-contrast, legible typography and color theme for either Light or Dark mode.
pub fn apply_termi_theme(ctx: &egui::Context, dark: bool) {
    let theme = if dark { Theme::Dark } else { Theme::Light };
    let mut visuals = if dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };

    if dark {
        let bg_panel = Color32::from_rgb(17, 24, 39);
        let bg_window = Color32::from_rgb(30, 41, 59);
        let bg_input = Color32::from_rgb(15, 23, 42);
        let text_primary = Color32::from_rgb(248, 250, 252);
        let border_subtle = Color32::from_rgb(71, 85, 105);

        visuals.panel_fill = bg_panel;
        visuals.window_fill = bg_window;
        visuals.extreme_bg_color = bg_input;
        visuals.faint_bg_color = Color32::from_rgb(26, 34, 48);
        visuals.window_stroke = Stroke::new(1.0, border_subtle);

        visuals.widgets.noninteractive.bg_fill = bg_panel;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(51, 65, 85));
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, text_primary);

        visuals.widgets.inactive.bg_fill = Color32::from_rgb(30, 41, 59);
        visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(30, 41, 59);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, border_subtle);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, text_primary);

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(51, 65, 85);
        visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(51, 65, 85);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, Color32::from_rgb(56, 189, 248));
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, Color32::WHITE);

        visuals.widgets.active.bg_fill = Color32::from_rgb(14, 116, 144);
        visuals.widgets.active.fg_stroke = Stroke::new(1.5, Color32::WHITE);

        visuals.selection.bg_fill = Color32::from_rgb(2, 132, 199);
        visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);
    } else {
        let bg_panel = Color32::from_rgb(241, 245, 249);
        let bg_window = Color32::from_rgb(255, 255, 255);
        let bg_input = Color32::from_rgb(255, 255, 255);
        let text_primary = Color32::from_rgb(15, 23, 42);
        let border_subtle = Color32::from_rgb(148, 163, 184);

        visuals.panel_fill = bg_panel;
        visuals.window_fill = bg_window;
        visuals.extreme_bg_color = bg_input;
        visuals.faint_bg_color = Color32::from_rgb(226, 232, 240);
        visuals.window_stroke = Stroke::new(1.0, border_subtle);

        visuals.widgets.noninteractive.bg_fill = bg_panel;
        visuals.widgets.noninteractive.bg_stroke =
            Stroke::new(1.0, Color32::from_rgb(203, 213, 225));
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, text_primary);

        visuals.widgets.inactive.bg_fill = Color32::from_rgb(255, 255, 255);
        visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(226, 232, 240);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, border_subtle);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, text_primary);

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(224, 242, 254);
        visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(224, 242, 254);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, Color32::from_rgb(2, 132, 199));
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, text_primary);

        visuals.widgets.active.bg_fill = Color32::from_rgb(186, 230, 253);
        visuals.widgets.active.fg_stroke = Stroke::new(1.5, text_primary);

        visuals.selection.bg_fill = Color32::from_rgb(2, 132, 199);
        visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);
    }

    visuals.widgets.inactive.corner_radius = CornerRadius::same(4);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(4);
    visuals.widgets.active.corner_radius = CornerRadius::same(4);

    ctx.set_visuals_of(theme, visuals);
    ctx.all_styles_mut(|style| {
        style
            .text_styles
            .insert(TextStyle::Body, FontId::proportional(15.0));
        style
            .text_styles
            .insert(TextStyle::Button, FontId::proportional(14.5));
        style
            .text_styles
            .insert(TextStyle::Small, FontId::proportional(13.0));
        style
            .text_styles
            .insert(TextStyle::Monospace, FontId::monospace(14.5));
        style
            .text_styles
            .insert(TextStyle::Heading, FontId::proportional(19.0));
        style.spacing.item_spacing = Vec2::new(8.0, 6.0);
        if cfg!(target_os = "android") {
            style.spacing.button_padding = Vec2::new(11.0, 8.0);
            style.spacing.interact_size = Vec2::new(42.0, 34.0);
            style.spacing.scroll.bar_width = 10.0;
            style.spacing.scroll.handle_min_length = 28.0;
            style.interaction.selectable_labels = false;
        } else {
            style.spacing.button_padding = Vec2::new(10.0, 5.0);
        }
    });
}

/// High-contrast secondary text color (replaces faint `.weak()`).
pub fn secondary_text_color(ui: &egui::Ui) -> Color32 {
    if ui.visuals().dark_mode {
        Color32::from_rgb(203, 213, 225)
    } else {
        Color32::from_rgb(51, 65, 85)
    }
}

/// High-contrast primary accent button (Start/Create/Apply/Primary action).
pub fn primary_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let fill = if ui.visuals().dark_mode {
        Color32::from_rgb(2, 132, 199)
    } else {
        Color32::from_rgb(3, 105, 161)
    };
    let btn = egui::Button::new(egui::RichText::new(label).color(Color32::WHITE).strong())
        .fill(fill)
        .stroke(Stroke::new(1.0, Color32::from_rgb(56, 189, 248)));
    ui.add(btn)
}

/// High-contrast positive/export accent button (Export / Save / Add).
pub fn success_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let fill = if ui.visuals().dark_mode {
        Color32::from_rgb(21, 128, 61)
    } else {
        Color32::from_rgb(22, 163, 74)
    };
    let btn = egui::Button::new(egui::RichText::new(label).color(Color32::WHITE).strong())
        .fill(fill)
        .stroke(Stroke::new(1.0, Color32::from_rgb(74, 222, 128)));
    ui.add(btn)
}

/// High-contrast destructive accent button (Delete).
pub fn danger_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let fill = if ui.visuals().dark_mode {
        Color32::from_rgb(185, 28, 28)
    } else {
        Color32::from_rgb(220, 38, 38)
    };
    let btn = egui::Button::new(egui::RichText::new(label).color(Color32::WHITE).strong())
        .fill(fill)
        .stroke(Stroke::new(1.0, Color32::from_rgb(248, 113, 113)));
    ui.add(btn)
}
