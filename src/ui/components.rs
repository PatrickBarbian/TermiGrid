use egui::{
    Align2, Color32, Context, CornerRadius, Frame, Margin, Pos2, Rect, Sense, Stroke, StrokeKind,
    Vec2,
};

use crate::catalog::TerminalCatalog;
use crate::model::TerminalStrip;
use crate::ui::icons::{compact_mfg_badge, draw_icon, icon_only_button, UiIcon};
use crate::ui::theme::{kbd_chip, ThemeTokens};

/// Action triggered by an inline toast button (Sonner-style).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastActionKind {
    Undo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastTone {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct ToastItem {
    pub id: u64,
    pub tone: ToastTone,
    pub message: String,
    pub created_at: f64,
    pub duration_secs: f64,
    pub action: Option<(String, ToastActionKind)>,
}

/// Non-intrusive, auto-dismissing bottom-right toast stack inspired by Sonner.
#[derive(Debug, Default)]
pub struct ToastQueue {
    next_id: u64,
    items: Vec<ToastItem>,
}

impl ToastQueue {
    pub fn push(&mut self, time: f64, tone: ToastTone, message: impl Into<String>) {
        self.push_with_action(time, tone, message, None);
    }

    pub fn push_with_action(
        &mut self,
        time: f64,
        tone: ToastTone,
        message: impl Into<String>,
        action: Option<(String, ToastActionKind)>,
    ) {
        self.next_id += 1;
        let duration_secs = match tone {
            ToastTone::Error => 6.0,
            ToastTone::Warning => 5.0,
            _ => if action.is_some() { 5.0 } else { 3.6 },
        };
        self.items.push(ToastItem {
            id: self.next_id,
            tone,
            message: message.into(),
            created_at: time,
            duration_secs,
            action,
        });
        if self.items.len() > 3 {
            let drain = self.items.len() - 3;
            self.items.drain(0..drain);
        }
    }

    /// Renders active toasts in the bottom-right corner above the status bar.
    /// Returns an optional action if the user clicked an inline button (e.g., Undo).
    pub fn show(&mut self, ctx: &Context, bottom_inset: f32) -> Option<ToastActionKind> {
        let now = ctx.input(|i| i.time);
        self.items
            .retain(|item| (now - item.created_at) < item.duration_secs);
        if self.items.is_empty() {
            return None;
        }

        // Request smooth repaint while toasts are visible/fading out
        ctx.request_repaint_after(std::time::Duration::from_millis(45));

        let dark = ctx.theme() == egui::Theme::Dark;
        let t = ThemeTokens::for_mode(dark);
        let mut triggered: Option<ToastActionKind> = None;
        let mut dismiss_id: Option<u64> = None;

        let viewport = ctx.content_rect();
        let toast_w = (viewport.width() - 28.0).clamp(240.0, 380.0);
        let anchor_offset = Vec2::new(-14.0, -(32.0 + bottom_inset));

        egui::Area::new(egui::Id::unique("termi_toast_stack"))
            .order(egui::Order::Foreground)
            .anchor(Align2::RIGHT_BOTTOM, anchor_offset)
            .show(ctx, |ui| {
                ui.set_max_width(toast_w);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 6.0;
                    for item in &self.items {
                        let age = (now - item.created_at).max(0.0);
                        let enter_t = (age / 0.16).clamp(0.0, 1.0) as f32;
                        let remain = (item.duration_secs - age).max(0.0);
                        let exit_t = (remain / 0.25).clamp(0.0, 1.0) as f32;
                        // Cubic ease-out feel
                        let ease = 1.0 - (1.0 - enter_t).powi(3);
                        let alpha = (ease * exit_t).clamp(0.0, 1.0);

                        let (accent_col, icon) = match item.tone {
                            ToastTone::Info => (t.accent, UiIcon::Info),
                            ToastTone::Success => (t.success_hover, UiIcon::Check),
                            ToastTone::Warning => (t.warning, UiIcon::Bolt),
                            ToastTone::Error => (t.danger_hover, UiIcon::Close),
                        };

                        let bg = t.bg_surface_elevated.gamma_multiply(alpha);
                        let border = t.border_strong.gamma_multiply(alpha);
                        let fg = t.text_primary.gamma_multiply(alpha);

                        Frame::NONE
                            .fill(bg)
                            .stroke(Stroke::new(1.0, border))
                            .corner_radius(CornerRadius::same(8))
                            .inner_margin(Margin::symmetric(12, 9))
                            .show(ui, |ui| {
                                ui.set_width(toast_w - 24.0);
                                ui.horizontal(|ui| {
                                    let (icon_r, _) = ui.allocate_exact_size(
                                        Vec2::splat(16.0),
                                        Sense::hover(),
                                    );
                                    draw_icon(
                                        ui.painter(),
                                        icon_r,
                                        icon,
                                        accent_col.gamma_multiply(alpha),
                                    );
                                    ui.add_space(4.0);
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(&item.message)
                                                .color(fg)
                                                .size(13.0),
                                        )
                                        .wrap(),
                                    );
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if icon_only_button(
                                                ui,
                                                UiIcon::Close,
                                                false,
                                                "Schließen",
                                            )
                                            .clicked()
                                            {
                                                dismiss_id = Some(item.id);
                                            }
                                            if let Some((act_lbl, act_kind)) = &item.action
                                                && ui
                                                    .add(
                                                        egui::Button::new(
                                                            egui::RichText::new(act_lbl)
                                                                .color(t.accent_text)
                                                                .strong()
                                                                .size(12.5),
                                                        )
                                                        .fill(t.accent_soft)
                                                        .stroke(Stroke::new(1.0, t.accent))
                                                        .corner_radius(CornerRadius::same(5)),
                                                    )
                                                    .clicked()
                                            {
                                                triggered = Some(*act_kind);
                                                dismiss_id = Some(item.id);
                                            }
                                        },
                                    );
                                });
                            });
                    }
                });
            });

        if let Some(id) = dismiss_id {
            self.items.retain(|it| it.id != id);
        }
        triggered
    }
}

/// Commands exposed inside the `Strg+K` (`cmdk`-style) Command Palette.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaletteCommand {
    GenerateActiveStrip,
    NewStrip,
    NewProject,
    OpenProject,
    SaveProject,
    ImportPdf,
    OpenLibrary,
    ExportXlsx,
    ExportCsv,
    ExportDxf,
    ToggleBridgeMode,
    ToggleEplanMode,
    ToggleSequenceMode,
    ConvertNToNt,
    ConvertNtToN,
    Undo,
    Redo,
    CycleTheme,
    SwitchStrip(usize),
    InsertTerminal(String),
}

struct PaletteEntry {
    icon: UiIcon,
    title: String,
    subtitle: String,
    shortcut: &'static str,
    mfg_badge: Option<String>,
    command: PaletteCommand,
}

/// Instant, zero-delay keyboard-first Command Palette (`Strg+K` / `Cmd+K`).
#[derive(Debug, Default)]
pub struct CommandPaletteState {
    pub open: bool,
    pub query: String,
    pub selected_idx: usize,
    just_opened: bool,
}

impl CommandPaletteState {
    pub fn open_now(&mut self) {
        self.open = true;
        self.query.clear();
        self.selected_idx = 0;
        self.just_opened = true;
    }

    pub fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.selected_idx = 0;
        self.just_opened = false;
    }

    #[allow(clippy::too_many_arguments)]
    pub fn show(
        &mut self,
        ctx: &Context,
        catalog: &TerminalCatalog,
        strips: &[TerminalStrip],
        has_active_project: bool,
        can_undo: bool,
        can_redo: bool,
    ) -> Option<PaletteCommand> {
        if !self.open {
            return None;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.close();
            return None;
        }

        let dark = ctx.theme() == egui::Theme::Dark;
        let t = ThemeTokens::for_mode(dark);
        let screen_rect = ctx.content_rect();

        // Dim backdrop
        let backdrop_resp = egui::Area::new(egui::Id::unique("cmdk_backdrop"))
            .order(egui::Order::Foreground)
            .fixed_pos(screen_rect.min)
            .show(ctx, |ui| {
                let (r, resp) = ui.allocate_exact_size(screen_rect.size(), Sense::click());
                ui.painter().rect_filled(
                    r,
                    CornerRadius::ZERO,
                    Color32::from_black_alpha(if dark { 145 } else { 85 }),
                );
                resp
            })
            .inner;
        if backdrop_resp.clicked() {
            self.close();
            return None;
        }

        // Build matching command entries
        let mut entries: Vec<PaletteEntry> = Vec::new();
        let q = self.query.trim().to_lowercase();
        let matches_q = |title: &str, sub: &str| {
            q.is_empty()
                || title.to_lowercase().contains(&q)
                || sub.to_lowercase().contains(&q)
        };

        if has_active_project {
            for (title, sub, icon, sc, cmd) in [
                (
                    "Generieren (Endplatten & NT prüfen)",
                    "Berechnet Trennplatten und prüft NT-Einspeisung",
                    UiIcon::Bolt,
                    "",
                    PaletteCommand::GenerateActiveStrip,
                ),
                (
                    "Neue Klemmleiste anlegen",
                    "Leisten-Assistent öffnen",
                    UiIcon::Add,
                    "",
                    PaletteCommand::NewStrip,
                ),
                (
                    "Brücken-Modus umschalten",
                    "Potentiale & Kammbrücken zeichnen",
                    UiIcon::ModeBridge,
                    "",
                    PaletteCommand::ToggleBridgeMode,
                ),
                (
                    "EPLAN 1:1 Aufbaubild umschalten",
                    "Maßstabsgetreue TS 35 Darstellung + DXF",
                    UiIcon::ModeEplan,
                    "",
                    PaletteCommand::ToggleEplanMode,
                ),
                (
                    "Zählfolge umschalten (Klemmenweise ↔ Ebenenweise)",
                    "Umschalten zwischen 1,2 | 3,4 und Ebene 1: 1,2,3 → Ebene 2: 4,5,6",
                    UiIcon::FlipOrder,
                    "",
                    PaletteCommand::ToggleSequenceMode,
                ),
                (
                    "Excel-Export (.xlsx)",
                    "Alle Klemmleisten als 4-Blatt-Arbeitsmappe exportieren",
                    UiIcon::ExportXlsx,
                    "",
                    PaletteCommand::ExportXlsx,
                ),
                (
                    "CSV-Export (.csv)",
                    "Flache Druckdaten für Markierer exportieren",
                    UiIcon::ExportCsv,
                    "",
                    PaletteCommand::ExportCsv,
                ),
                (
                    "DXF-Export (EPLAN 1:1 mm)",
                    "Maßstabsgetreue CAD-Zeichnung exportieren",
                    UiIcon::ExportDxf,
                    "",
                    PaletteCommand::ExportDxf,
                ),
                (
                    "Alle N-Klemmen in NT umwandeln",
                    "Ersetzt Standard-N-Klemmen durch NT-Trennklemmen",
                    UiIcon::ConvertNt,
                    "",
                    PaletteCommand::ConvertNToNt,
                ),
                (
                    "Alle NT-Klemmen in N zurückwandeln",
                    "Ersetzt NT-Trennklemmen durch Standard-N-Klemmen",
                    UiIcon::ConvertNt,
                    "",
                    PaletteCommand::ConvertNtToN,
                ),
                (
                    "Projekt speichern",
                    "Aktuelles Projekt als .termi sichern",
                    UiIcon::Save,
                    "",
                    PaletteCommand::SaveProject,
                ),
            ] {
                if matches_q(title, sub) {
                    entries.push(PaletteEntry {
                        icon,
                        title: title.to_string(),
                        subtitle: sub.to_string(),
                        shortcut: sc,
                        mfg_badge: None,
                        command: cmd,
                    });
                }
            }
            if can_undo && matches_q("Rückgängig Undo", "Letzten Schritt zurücknehmen") {
                entries.push(PaletteEntry {
                    icon: UiIcon::Undo,
                    title: "Rückgängig (Undo)".to_string(),
                    subtitle: "Letzte Änderung rückgängig machen".to_string(),
                    shortcut: "Strg+Z",
                    mfg_badge: None,
                    command: PaletteCommand::Undo,
                });
            }
            if can_redo && matches_q("Wiederholen Redo", "Schritt wiederherstellen") {
                entries.push(PaletteEntry {
                    icon: UiIcon::Redo,
                    title: "Wiederholen (Redo)".to_string(),
                    subtitle: "Rückgängig gemachten Schritt wiederherstellen".to_string(),
                    shortcut: "Strg+Y",
                    mfg_badge: None,
                    command: PaletteCommand::Redo,
                });
            }

            for (idx, strip) in strips.iter().enumerate() {
                let title = format!("Klemmleiste {}", strip.name);
                let sub = format!(
                    "{} Klemmen · {:.1} mm{}",
                    strip.blocks.len(),
                    strip.total_width_mm(),
                    if strip.description.is_empty() {
                        String::new()
                    } else {
                        format!(" · {}", strip.description)
                    }
                );
                if matches_q(&title, &sub) {
                    entries.push(PaletteEntry {
                        icon: UiIcon::SidebarLeft(true),
                        title,
                        subtitle: sub,
                        shortcut: "",
                        mfg_badge: None,
                        command: PaletteCommand::SwitchStrip(idx),
                    });
                }
            }
        }

        for (title, sub, icon, cmd) in [
            (
                "PDF-Stückliste importieren",
                "EPLAN-Stückliste (.pdf) automatisch erkennen",
                UiIcon::ImportPdf,
                PaletteCommand::ImportPdf,
            ),
            (
                "Bibliothek öffnen",
                "PC-weite Klemmleisten-Vorlagen durchsuchen",
                UiIcon::Library,
                PaletteCommand::OpenLibrary,
            ),
            (
                "Projekt öffnen",
                "Vorhandene .termi-Datei laden",
                UiIcon::FolderOpen,
                PaletteCommand::OpenProject,
            ),
            (
                "Neues Projekt starten",
                "Mit leerem Projekt beginnen",
                UiIcon::NewFile,
                PaletteCommand::NewProject,
            ),
            (
                "Farbschema wechseln (System / Hell / Dunkel)",
                "Zwischen Hell-, Dunkel- und System-Design wechseln",
                UiIcon::ThemeSystem,
                PaletteCommand::CycleTheme,
            ),
        ] {
            if matches_q(title, sub) {
                entries.push(PaletteEntry {
                    icon,
                    title: title.to_string(),
                    subtitle: sub.to_string(),
                    shortcut: "",
                    mfg_badge: None,
                    command: cmd,
                });
            }
        }

        // If user typed a query and an active strip exists, append matching terminal specs for instant 1-click insert
        if !q.is_empty() && has_active_project {
            for spec in catalog.filter_specs(None, &self.query).into_iter().take(12) {
                entries.push(PaletteEntry {
                    icon: UiIcon::Add,
                    title: format!("{} einfügen", spec.name),
                    subtitle: format!(
                        "{:.1} mm · {} Etagen · {}",
                        f32::from(spec.width_tenths_mm) / 10.0,
                        spec.levels,
                        spec.description_de
                    ),
                    shortcut: "",
                    mfg_badge: Some(spec.manufacturer.clone()),
                    command: PaletteCommand::InsertTerminal(spec.id.clone()),
                });
            }
        }

        if entries.is_empty() {
            self.selected_idx = 0;
        } else if self.selected_idx >= entries.len() {
            self.selected_idx = entries.len() - 1;
        }

        let (down_pressed, up_pressed, enter_pressed) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::ArrowDown),
                i.key_pressed(egui::Key::ArrowUp),
                i.key_pressed(egui::Key::Enter),
            )
        });
        if down_pressed && !entries.is_empty() {
            self.selected_idx = (self.selected_idx + 1) % entries.len();
        }
        if up_pressed && !entries.is_empty() {
            self.selected_idx = if self.selected_idx == 0 {
                entries.len() - 1
            } else {
                self.selected_idx - 1
            };
        }

        let mut chosen: Option<PaletteCommand> = None;
        if enter_pressed && !entries.is_empty() {
            chosen = Some(entries[self.selected_idx].command.clone());
        }

        let max_w = (screen_rect.width() * 0.92).clamp(280.0, 580.0);
        let top_y = (screen_rect.height() * 0.14).max(36.0);

        egui::Window::new("")
            .id(egui::Id::unique("termi_command_palette"))
            .order(egui::Order::Foreground)
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_TOP, [0.0, top_y])
            .fixed_size(Vec2::new(max_w, 420.0_f32.min(screen_rect.height() * 0.72)))
            .frame(
                Frame::NONE
                    .fill(t.bg_surface_elevated)
                    .stroke(Stroke::new(1.0, t.border_strong))
                    .corner_radius(CornerRadius::same(12))
                    .inner_margin(Margin::same(12))
                    .shadow(egui::Shadow {
                        offset: [0, 12],
                        blur: 32,
                        spread: 0,
                        color: Color32::from_black_alpha(if dark { 135 } else { 44 }),
                    }),
            )
            .show(ctx, |ui| {
                // Search header
                Frame::NONE
                    .fill(t.bg_recessed)
                    .stroke(Stroke::new(1.0, t.border_hairline))
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(Margin::symmetric(10, 8))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let (sr, _) =
                                ui.allocate_exact_size(Vec2::splat(16.0), Sense::hover());
                            draw_icon(ui.painter(), sr, UiIcon::Search, t.accent);
                            ui.add_space(4.0);
                            let edit_id = egui::Id::unique("cmdk_search_input");
                            let resp = ui.add(
                                egui::TextEdit::singleline(&mut self.query)
                                    .id(edit_id)
                                    .frame(Frame::NONE)
                                    .desired_width(ui.available_width() - 48.0)
                                    .hint_text("Befehl, Klemmleiste oder Klemmentyp (z. B. 2002, PTI, Excel)…"),
                            );
                            if self.just_opened {
                                resp.request_focus();
                                self.just_opened = false;
                            }
                            if resp.changed() {
                                self.selected_idx = 0;
                            }
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    kbd_chip(ui, "Esc");
                                },
                            );
                        });
                    });

                ui.add_space(8.0);

                egui::ScrollArea::vertical()
                    .id_salt("cmdk_scroll")
                    .max_height(330.0)
                    .show(ui, |ui| {
                        if entries.is_empty() {
                            ui.add_space(18.0);
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    egui::RichText::new("Keine passenden Befehle oder Klemmen gefunden.")
                                        .color(t.text_secondary),
                                );
                            });
                            ui.add_space(18.0);
                            return;
                        }

                        for (idx, entry) in entries.iter().enumerate() {
                            let is_sel = idx == self.selected_idx;
                            let row_w = ui.available_width();
                            let row_h = 42.0;
                            let (rect, resp) =
                                ui.allocate_exact_size(Vec2::new(row_w, row_h), Sense::click());
                            if ui.is_rect_visible(rect) {
                                let hovered = resp.hovered();
                                if is_sel {
                                    ui.painter().rect_filled(
                                        rect,
                                        CornerRadius::same(7),
                                        t.accent_soft,
                                    );
                                    ui.painter().rect_stroke(
                                        rect,
                                        CornerRadius::same(7),
                                        Stroke::new(1.0, t.accent),
                                        StrokeKind::Inside,
                                    );
                                } else if hovered {
                                    ui.painter().rect_filled(
                                        rect,
                                        CornerRadius::same(7),
                                        t.bg_surface_hover,
                                    );
                                }

                                let icon_r = Rect::from_center_size(
                                    Pos2::new(rect.min.x + 18.0, rect.center().y),
                                    Vec2::splat(16.0),
                                );
                                draw_icon(
                                    ui.painter(),
                                    icon_r,
                                    entry.icon,
                                    if is_sel { t.accent_text } else { t.text_secondary },
                                );

                                let title_x = rect.min.x + 36.0;
                                let title_galley = ui.painter().layout_no_wrap(
                                    entry.title.clone(),
                                    egui::FontId::proportional(13.5),
                                    t.text_primary,
                                );
                                ui.painter().galley(
                                    Pos2::new(title_x, rect.min.y + 6.0),
                                    title_galley,
                                    t.text_primary,
                                );

                                let sub_galley = ui.painter().layout(
                                    entry.subtitle.clone(),
                                    egui::FontId::proportional(11.5),
                                    t.text_secondary,
                                    (rect.width() - 120.0).max(120.0),
                                );
                                ui.painter().galley(
                                    Pos2::new(title_x, rect.min.y + 23.0),
                                    sub_galley,
                                    t.text_secondary,
                                );

                                if !entry.shortcut.is_empty() {
                                    let sc_pos = Pos2::new(rect.max.x - 12.0, rect.center().y);
                                    ui.painter().text(
                                        sc_pos,
                                        Align2::RIGHT_CENTER,
                                        entry.shortcut,
                                        egui::FontId::monospace(11.0),
                                        t.text_muted,
                                    );
                                } else if let Some(mfg) = &entry.mfg_badge {
                                    let pill_rect = Rect::from_min_max(
                                        Pos2::new(rect.max.x - 62.0, rect.center().y - 9.0),
                                        Pos2::new(rect.max.x - 10.0, rect.center().y + 9.0),
                                    );
                                    let mut child = ui.new_child(
                                        egui::UiBuilder::new().max_rect(pill_rect),
                                    );
                                    compact_mfg_badge(&mut child, mfg);
                                }
                            }
                            if resp.clicked() {
                                chosen = Some(entry.command.clone());
                            }
                        }
                    });
            });

        if chosen.is_some() {
            self.close();
        }
        chosen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toast_queue_caps_at_three_items_and_command_palette_resets() {
        let mut q = ToastQueue::default();
        for i in 0..5 {
            q.push(i as f64, ToastTone::Info, format!("Msg {i}"));
        }
        assert_eq!(q.items.len(), 3);
        assert_eq!(q.items[0].message, "Msg 2");
        assert_eq!(q.items[2].message, "Msg 4");

        let mut pal = CommandPaletteState::default();
        pal.open_now();
        assert!(pal.open);
        pal.query = "2002".to_string();
        pal.selected_idx = 2;
        pal.close();
        assert!(!pal.open);
        assert!(pal.query.is_empty());
        assert_eq!(pal.selected_idx, 0);
    }
}
