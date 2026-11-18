pub mod android_insets;
pub mod cad_canvas;
pub mod guide_modal;
pub mod logos;
pub mod net;
pub mod new_strip_modal;
pub mod platform_io;
pub mod product_media;
pub mod theme;
pub mod updater;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use eframe::egui;

use crate::catalog::accessories::{
    calculate_end_plates, end_plate_width_tenths_mm, find_end_bracket,
};
use crate::catalog::TerminalCatalog;
use crate::export::{
    csv::export_to_csv, dxf::export_to_dxf, xlsx::export_to_xlsx, ExportOptions, PrintOrder,
};
use crate::i18n::{strings, Language};
use crate::model::bridging::{
    calculate_bridges, CombJumperSegment, WireBridgeReason, POTENTIAL_PALETTE,
};
use crate::model::{
    NeutralLabelMode, NumberingOrder, PeLabelMode, Project, TerminalStrip,
};
use crate::ui::android_insets::{current_safe_insets, SafeAreaInsets};
use crate::ui::cad_canvas::{
    show_cad_canvas, CadCanvasState, CanvasInteractionMode, TypeDisplayMode,
};
use crate::ui::guide_modal::{
    show_about_modal, show_database_tab, show_guide_tab, CustomSpecDraft,
};
use crate::ui::logos::{show_logo_badge, BrandLogos};
use crate::ui::new_strip_modal::{
    show_end_bracket_picker, show_install_supply_prompt_modal, show_new_strip_modal,
    show_spec_picker, InstallPromptDialogState, InstallPromptResolution, InstallSupplyPlacement,
    NewStripDialogState, PendingInstallAction,
};
use crate::ui::platform_io::{
    pick_export_csv_path, pick_export_dxf_path, pick_export_xlsx_path, pick_open_project_path,
    pick_save_project_path,
};
use crate::ui::product_media::{
    show_end_bracket_preview_card, show_product_preview_card, ProductMediaCache,
};
use crate::ui::theme::{
    apply_theme_mode, danger_button, install_system_fonts, primary_button, secondary_text_color,
    success_button, ThemeMode,
};
use crate::ui::updater::{
    open_external_url, UpdateChecker, UpdateStatus, BUYMEACOFFEE_URL, GITHUB_REPO_URL,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActiveTab {
    Editor,
    Database,
    Guide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MobilePane {
    Strips,
    Cad,
    Inspector,
}

pub struct TermiGridApp {
    catalog: TerminalCatalog,
    logos: BrandLogos,
    project: Project,
    active_strip: usize,
    selected_blocks: BTreeSet<usize>,
    selected_end_bracket: Option<bool>,
    canvas_state: CadCanvasState,
    new_strip_modal: NewStripDialogState,
    install_prompt_modal: InstallPromptDialogState,

    // Reassign / insert picker state
    inspector_mfg_filter: Option<String>,
    inspector_search: String,
    inspector_spec_id: String,
    last_selection_sync: Option<(usize, usize, String)>,

    // Database tab filter state
    db_mfg_filter: Option<String>,
    db_search: String,
    custom_spec_draft: CustomSpecDraft,

    // Export & File state
    export_options: ExportOptions,
    current_file: Option<PathBuf>,
    status_message: String,

    // UX preferences & Guided Project Pipeline
    lang: Language,
    theme_mode: ThemeMode,
    show_welcome: bool,
    show_left_sidebar: bool,
    show_inspector: bool,
    show_about_modal: bool,
    active_tab: ActiveTab,
    mobile_pane: MobilePane,
    updater: UpdateChecker,

    // Cached Android system-bar / display-cutout safe area
    safe_insets: SafeAreaInsets,
    last_screen_size: egui::Vec2,
    insets_frames: u32,

    // Batch affix / special character state
    batch_prefix: String,
    batch_suffix: String,
    batch_target_row: Option<isize>,

    // Jumper / Bridge mode & product photo cache
    new_potential_name: String,
    active_potential_id: Option<u64>,
    product_media: ProductMediaCache,

    // Undo / Redo history snapshots
    undo_stack: Vec<Project>,
    redo_stack: Vec<Project>,
    last_snapshot: Project,
}

impl TermiGridApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let catalog = TerminalCatalog::load_embedded();
        let logos = BrandLogos::load(&cc.egui_ctx);
        let project = Project::default();
        let last_snapshot = project.clone();

        // Load native system fonts & apply high-contrast Light+Dark themes with System default
        install_system_fonts(&cc.egui_ctx);
        apply_theme_mode(&cc.egui_ctx, ThemeMode::System);

        let updater = UpdateChecker::default();
        updater.check_async(cc.egui_ctx.clone());

        Self {
            catalog,
            logos,
            project,
            active_strip: 0,
            selected_blocks: BTreeSet::new(),
            selected_end_bracket: None,
            canvas_state: CadCanvasState::new(),
            new_strip_modal: NewStripDialogState::default(),
            install_prompt_modal: InstallPromptDialogState::default(),
            inspector_mfg_filter: None,
            inspector_search: String::new(),
            inspector_spec_id: "wago-2002-2204".to_string(),
            last_selection_sync: None,
            db_mfg_filter: None,
            db_search: String::new(),
            custom_spec_draft: CustomSpecDraft::default(),
            export_options: ExportOptions::default(),
            current_file: None,
            status_message: strings(Language::De).status_ready.to_string(),
            lang: Language::De,
            theme_mode: ThemeMode::System,
            show_welcome: true,
            show_left_sidebar: true,
            show_inspector: true,
            show_about_modal: false,
            active_tab: ActiveTab::Editor,
            mobile_pane: MobilePane::Cad,
            updater,
            safe_insets: SafeAreaInsets::default(),
            last_screen_size: egui::Vec2::ZERO,
            insets_frames: 0,
            batch_prefix: String::new(),
            batch_suffix: String::new(),
            batch_target_row: None,
            new_potential_name: "+24V".to_string(),
            active_potential_id: None,
            product_media: ProductMediaCache::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_snapshot,
        }
    }

    fn perform_undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.project.clone());
            self.project = prev;
            self.catalog.reload_with_custom(&self.project.custom_specs);
            self.last_snapshot = self.project.clone();
            self.active_strip = self
                .active_strip
                .min(self.project.strips.len().saturating_sub(1));
            self.selected_blocks.clear();
            self.selected_end_bracket = None;
        }
    }

    fn perform_redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.project.clone());
            self.project = next;
            self.catalog.reload_with_custom(&self.project.custom_specs);
            self.last_snapshot = self.project.clone();
            self.active_strip = self
                .active_strip
                .min(self.project.strips.len().saturating_sub(1));
            self.selected_blocks.clear();
            self.selected_end_bracket = None;
        }
    }

    fn save_project(&mut self) {
        let s = strings(self.lang);
        let default_name = format!("{}.termi", sanitize_filename(&self.project.project_name));
        let path = if let Some(p) = &self.current_file {
            Some(p.clone())
        } else {
            pick_save_project_path(&default_name)
        };

        if let Some(path) = path {
            match serde_json::to_string_pretty(&self.project) {
                Ok(json) => match std::fs::write(&path, json) {
                    Ok(()) => {
                        self.current_file = Some(path.clone());
                        self.status_message = format!("{}: {}", s.status_saved, path.display());
                    }
                    Err(e) => {
                        self.status_message = format!("{}: {e}", s.status_error);
                    }
                },
                Err(e) => {
                    self.status_message = format!("{}: {e}", s.status_error);
                }
            }
        }
    }

    fn open_project(&mut self) {
        let s = strings(self.lang);
        if let Some(path) = pick_open_project_path() {
            match std::fs::read_to_string(&path) {
                Ok(raw) => match serde_json::from_str::<Project>(&raw) {
                    Ok(mut proj) => {
                        proj.normalize();
                        self.catalog.reload_with_custom(&proj.custom_specs);
                        self.project = proj;
                        self.undo_stack.clear();
                        self.redo_stack.clear();
                        self.last_snapshot = self.project.clone();
                        self.active_strip = 0;
                        self.selected_blocks.clear();
                        self.selected_end_bracket = None;
                        self.current_file = Some(path.clone());
                        self.show_welcome = false;
                        self.active_tab = ActiveTab::Editor;
                        self.status_message = format!("{}: {}", s.status_loaded, path.display());
                    }
                    Err(e) => {
                        self.status_message = format!("{}: {e}", s.status_error);
                    }
                },
                Err(e) => {
                    self.status_message = format!("{}: {e}", s.status_error);
                }
            }
        }
    }

    fn export_xlsx(&mut self) {
        let s = strings(self.lang);
        let default_name = format!("{}_labels.xlsx", sanitize_filename(&self.project.project_name));
        if let Some(path) = pick_export_xlsx_path(&default_name) {
            let german = self.lang == Language::De;
            match export_to_xlsx(
                &path,
                &self.project,
                self.active_strip,
                &self.export_options,
                german,
            ) {
                Ok(count) => {
                    self.status_message =
                        format!("{}: {} ({} {})", s.status_exported, path.display(), count, s.labels_unit);
                }
                Err(e) => {
                    self.status_message = format!("{}: {e}", s.status_error);
                }
            }
        }
    }

    fn export_csv(&mut self) {
        let s = strings(self.lang);
        let default_name = format!("{}_labels.csv", sanitize_filename(&self.project.project_name));
        if let Some(path) = pick_export_csv_path(&default_name) {
            let german = self.lang == Language::De;
            match export_to_csv(
                &path,
                &self.project,
                self.active_strip,
                &self.export_options,
                german,
            ) {
                Ok(count) => {
                    self.status_message =
                        format!("{}: {} ({} {})", s.status_exported, path.display(), count, s.labels_unit);
                }
                Err(e) => {
                    self.status_message = format!("{}: {e}", s.status_error);
                }
            }
        }
    }

    fn export_dxf(&mut self) {
        let s = strings(self.lang);
        let default_name = format!(
            "{}_eplan_aufbaubild.dxf",
            sanitize_filename(&self.project.project_name)
        );
        if let Some(path) = pick_export_dxf_path(&default_name) {
            match export_to_dxf(&path, &self.project, &self.catalog) {
                Ok(strip_count) => {
                    self.status_message = format!(
                        "{}: {} ({} {}, 1:1 mm)",
                        s.status_exported,
                        path.display(),
                        strip_count,
                        s.strips_heading
                    );
                }
                Err(e) => {
                    self.status_message = format!("{}: {e}", s.status_error);
                }
            }
        }
    }
}

impl eframe::App for TermiGridApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let s = strings(self.lang);

        // Global keyboard shortcuts when no text field is active
        if !ui.ctx().egui_wants_keyboard_input() && self.canvas_state.editing_tier.is_none() {
            let (undo_pressed, redo_pressed, move_left, move_right, duplicate_pressed) =
                ui.input(|i| {
                    let ctrl = i.modifiers.ctrl || i.modifiers.mac_cmd;
                    let shift = i.modifiers.shift;
                    let alt = i.modifiers.alt;
                    (
                        ctrl && !shift && i.key_pressed(egui::Key::Z),
                        ctrl && ((shift && i.key_pressed(egui::Key::Z))
                            || i.key_pressed(egui::Key::Y)),
                        alt && i.key_pressed(egui::Key::ArrowLeft),
                        alt && i.key_pressed(egui::Key::ArrowRight),
                        ctrl && i.key_pressed(egui::Key::D),
                    )
                });
            if undo_pressed {
                self.perform_undo();
            } else if redo_pressed {
                self.perform_redo();
            } else if self.active_strip < self.project.strips.len()
                && !self.selected_blocks.is_empty()
            {
                if move_left {
                    self.selected_blocks = self.project.strips[self.active_strip]
                        .move_selected_blocks(&self.selected_blocks, -1);
                } else if move_right {
                    self.selected_blocks = self.project.strips[self.active_strip]
                        .move_selected_blocks(&self.selected_blocks, 1);
                } else if duplicate_pressed {
                    self.selected_blocks = self.project.strips[self.active_strip]
                        .duplicate_selected_blocks(&self.selected_blocks);
                }
            }
        }

        // Refresh Android safe-area insets on launch, orientation/fold change, or keyboard toggle
        let screen_size = ui.ctx().content_rect().size();
        self.insets_frames = self.insets_frames.saturating_add(1);
        let size_changed = (screen_size - self.last_screen_size).length_sq() > 1.0;
        let kb_focus = ui.ctx().egui_wants_keyboard_input();
        if size_changed
            || self.insets_frames <= 20
            || self.insets_frames.is_multiple_of(30)
            || (kb_focus && self.insets_frames.is_multiple_of(6))
        {
            self.safe_insets = current_safe_insets(ui.ctx().pixels_per_point());
            self.last_screen_size = screen_size;
        }

        // Reserve physical safe areas so Android status bar, camera cutout, and nav pill never
        // overlap interactive controls or text.
        let panel_bg = ui.visuals().panel_fill;
        if self.safe_insets.top > 0.5 {
            egui::Panel::top("android_safe_top")
                .exact_size(self.safe_insets.top)
                .show_separator_line(false)
                .frame(egui::Frame::NONE.fill(panel_bg))
                .show(ui, |_| {});
        }
        if self.safe_insets.bottom > 0.5 {
            egui::Panel::bottom("android_safe_bottom")
                .exact_size(self.safe_insets.bottom)
                .show_separator_line(false)
                .frame(egui::Frame::NONE.fill(panel_bg))
                .show(ui, |_| {});
        }
        if self.safe_insets.left > 0.5 {
            egui::Panel::left("android_safe_left")
                .exact_size(self.safe_insets.left)
                .show_separator_line(false)
                .frame(egui::Frame::NONE.fill(panel_bg))
                .show(ui, |_| {});
        }
        if self.safe_insets.right > 0.5 {
            egui::Panel::right("android_safe_right")
                .exact_size(self.safe_insets.right)
                .show_separator_line(false)
                .frame(egui::Frame::NONE.fill(panel_bg))
                .show(ui, |_| {});
        }

        // Top Menu & Toolbar
        egui::Panel::top("top_bar").show(ui, |ui| {
            let avail_w = ui.available_width();
            let wide_header = avail_w >= 760.0;
            ui.add_space(4.0);

            let menu_label = match self.updater.status() {
                UpdateStatus::UpdateAvailable { .. } => "☰ Info ●",
                _ => "☰ Info ▾",
            };
            let draw_top_left_menu = |ui: &mut egui::Ui,
                                      show_about: &mut bool,
                                      updater: &UpdateChecker,
                                      s: &crate::i18n::Strings| {
                ui.menu_button(menu_label, |ui| {
                    ui.set_min_width(210.0);
                    if ui.button("ℹ Über TermiGrid & Lizenzen").clicked() {
                        *show_about = true;
                        ui.close();
                    }
                    ui.separator();
                    match updater.status() {
                        UpdateStatus::UpdateAvailable {
                            latest_tag,
                            release_url,
                        } => {
                            if ui
                                .button(
                                    egui::RichText::new(format!("⬆ Update: {latest_tag}"))
                                        .strong(),
                                )
                                .on_hover_text(s.update_open_release)
                                .clicked()
                            {
                                open_external_url(ui.ctx(), &release_url);
                                ui.close();
                            }
                        }
                        UpdateStatus::Checking => {
                            ui.label(s.update_checking);
                        }
                        _ => {
                            if ui.button(s.btn_check_update).clicked() {
                                updater.check_async(ui.ctx().clone());
                                ui.close();
                            }
                        }
                    }
                    if ui.button(s.btn_github).clicked() {
                        open_external_url(ui.ctx(), GITHUB_REPO_URL);
                        ui.close();
                    }
                    if ui.button(s.btn_coffee).clicked() {
                        open_external_url(ui.ctx(), BUYMEACOFFEE_URL);
                        ui.close();
                    }
                });
            };

            if wide_header {
                ui.horizontal(|ui| {
                    draw_top_left_menu(ui, &mut self.show_about_modal, &self.updater, s);
                    ui.heading(egui::RichText::new("TermiGrid").strong());
                    ui.separator();

                    ui.selectable_value(&mut self.active_tab, ActiveTab::Editor, s.tab_editor);
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Database, s.tab_database);
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Guide, s.tab_guide);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        self.show_header_prefs(ui, 110.0, avail_w >= 960.0);
                    });
                });
            } else {
                ui.horizontal(|ui| {
                    draw_top_left_menu(ui, &mut self.show_about_modal, &self.updater, s);
                    ui.heading(egui::RichText::new("TermiGrid").strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        self.show_header_prefs(ui, 88.0, false);
                    });
                });
                ui.horizontal_wrapped(|ui| {
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Editor, s.tab_editor);
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Database, s.tab_database);
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Guide, s.tab_guide);
                });
            }

            ui.separator();
            ui.horizontal_wrapped(|ui| {
                if primary_button(ui, s.menu_new_project).clicked() {
                    self.project = Project::default();
                    self.catalog = TerminalCatalog::load_embedded();
                    self.undo_stack.clear();
                    self.redo_stack.clear();
                    self.last_snapshot = self.project.clone();
                    self.current_file = None;
                    self.active_strip = 0;
                    self.selected_blocks.clear();
                    self.selected_end_bracket = None;
                    self.show_welcome = true;
                    self.active_tab = ActiveTab::Editor;
                    self.new_strip_modal.open_for_next_strip(0);
                }
                if ui.button(s.menu_open_project).clicked() {
                    self.open_project();
                }
                if success_button(ui, s.menu_save_project).clicked() {
                    self.save_project();
                }

                ui.separator();

                if ui
                    .add_enabled(!self.undo_stack.is_empty(), egui::Button::new("↩"))
                    .on_hover_text("Rückgängig / Undo (Strg+Z)")
                    .clicked()
                {
                    self.perform_undo();
                }
                if ui
                    .add_enabled(!self.redo_stack.is_empty(), egui::Button::new("↪"))
                    .on_hover_text("Wiederholen / Redo (Strg+Y)")
                    .clicked()
                {
                    self.perform_redo();
                }

                ui.separator();

                if primary_button(ui, s.menu_export_xlsx).clicked() {
                    self.export_xlsx();
                }
                if ui.button(s.menu_export_csv).clicked() {
                    self.export_csv();
                }
                if ui
                    .button(s.menu_export_dxf)
                    .on_hover_text(
                        "Exportiert alle Klemmleisten maßstabsgetreu (1:1 mm) als eine DXF-Datei für EPLAN Electric P8.",
                    )
                    .clicked()
                {
                    self.export_dxf();
                }

                if avail_w >= 1120.0 {
                    ui.separator();
                    ui.label(
                        egui::RichText::new(s.app_subtitle).color(secondary_text_color(ui)),
                    );
                }
            });
            ui.add_space(4.0);
        });

        // Bottom Status Bar (never overlapping on narrow screens)
        egui::Panel::bottom("status_bar").show(ui, |ui| {
            let avail_w = ui.available_width();
            if self.active_tab == ActiveTab::Editor && avail_w >= 1020.0 {
                ui.horizontal(|ui| {
                    let left_max = (avail_w - 580.0).max(160.0);
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(left_max, 18.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.add(egui::Label::new(&self.status_message).truncate());
                        },
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(s.cad_footer_hint)
                                    .color(secondary_text_color(ui)),
                            )
                            .truncate(),
                        );
                    });
                });
            } else {
                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(&self.status_message).truncate());
                });
            }
        });

        match self.active_tab {
            ActiveTab::Guide => {
                egui::CentralPanel::default().show(ui, |ui| {
                    show_guide_tab(ui, &self.updater, s);
                });
            }
            ActiveTab::Database => {
                egui::CentralPanel::default().show(ui, |ui| {
                    show_database_tab(
                        ui,
                        &mut self.catalog,
                        &self.logos,
                        &mut self.project.custom_specs,
                        &mut self.db_mfg_filter,
                        &mut self.db_search,
                        &mut self.custom_spec_draft,
                        s,
                        self.lang,
                    );
                });
            }
            ActiveTab::Editor => {
                if self.show_welcome {
                    self.show_welcome_pipeline(ui);
                } else {
                    self.show_editor_panels(ui);
                }
            }
        }

        // Modal dialog for new strip creation
        if let Some(new_strip) = show_new_strip_modal(
            ui.ctx(),
            &mut self.new_strip_modal,
            &self.catalog,
            &self.logos,
            &self.product_media,
            s,
            self.lang,
        ) {
            if let Some(spec) = self.catalog.find_by_id(&new_strip.default_spec_id)
                && spec.is_installation_or_nt_terminal()
            {
                self.install_prompt_modal.open_for_action(
                    PendingInstallAction::CreateStrip(new_strip),
                    spec,
                    false,
                );
            } else {
                self.project.strips.push(new_strip);
                self.active_strip = self.project.strips.len().saturating_sub(1);
                self.selected_blocks.clear();
                self.selected_end_bracket = None;
                self.show_welcome = false;
                self.active_tab = ActiveTab::Editor;
            }
        }

        // Modal dialog for NT / 2003 / 2203 / 2005 / 2205 installation terminal & N-Einspeiseklemme
        if let Some(res) = show_install_supply_prompt_modal(
            ui.ctx(),
            &mut self.install_prompt_modal,
            &self.catalog,
            &self.logos,
            &self.product_media,
            s,
        ) {
            self.apply_install_prompt_resolution(res);
        }

        // Cross-platform Info & Open-Source Licenses modal dialog
        show_about_modal(ui.ctx(), &mut self.show_about_modal, &self.updater, s);

        // Automatically record project state changes onto the Undo stack once pointer drag finishes
        if !ui.input(|i| i.pointer.any_down()) && self.project != self.last_snapshot {
            if self.undo_stack.len() >= 50 {
                self.undo_stack.remove(0);
            }
            self.undo_stack.push(self.last_snapshot.clone());
            self.redo_stack.clear();
            self.last_snapshot = self.project.clone();
        }
    }
}

impl TermiGridApp {
    fn strip_has_n_supply(&self, strip_idx: usize) -> bool {
        self.project
            .strips
            .get(strip_idx)
            .is_some_and(|strip| {
                strip.blocks.iter().any(|b| {
                    self.catalog
                        .find_by_id(&b.spec_id)
                        .is_some_and(|s| s.is_n_supply_terminal())
                })
            })
    }

    fn apply_install_prompt_resolution(&mut self, res: InstallPromptResolution) {
        let Some(effective_spec) = self.catalog.find_by_id(&res.effective_spec_id) else {
            return;
        };
        let supply_spec = if res.supply_placement != InstallSupplyPlacement::None {
            self.catalog.find_by_id(&res.supply_spec_id)
        } else {
            None
        };

        match res.action {
            PendingInstallAction::CreateStrip(mut new_strip) => {
                if new_strip.default_spec_id != effective_spec.id {
                    new_strip.default_spec_id = effective_spec.id.clone();
                    let all_indices: Vec<usize> = (0..new_strip.blocks.len()).collect();
                    new_strip.reassign_blocks(&all_indices, effective_spec);
                }
                if let Some(sup) = supply_spec {
                    match res.supply_placement {
                        InstallSupplyPlacement::BeforeGroup => {
                            new_strip.insert_blocks(0, 1, sup);
                        }
                        InstallSupplyPlacement::AfterGroup => {
                            new_strip.append_blocks(1, sup);
                        }
                        InstallSupplyPlacement::None => {}
                    }
                }
                self.project.strips.push(new_strip);
                self.active_strip = self.project.strips.len().saturating_sub(1);
                self.selected_blocks.clear();
                self.selected_end_bracket = None;
                self.show_welcome = false;
                self.active_tab = ActiveTab::Editor;
            }
            PendingInstallAction::ReassignSelection {
                strip_idx,
                indices,
                ..
            } => {
                if let Some(strip) = self.project.strips.get_mut(strip_idx) {
                    strip.reassign_blocks(&indices, effective_spec);
                    if let Some(sup) = supply_spec {
                        match res.supply_placement {
                            InstallSupplyPlacement::BeforeGroup => {
                                if let Some(&first) = indices.first() {
                                    strip.insert_blocks(first, 1, sup);
                                    self.selected_blocks = indices
                                        .iter()
                                        .map(|&i| if i >= first { i + 1 } else { i })
                                        .collect();
                                }
                            }
                            InstallSupplyPlacement::AfterGroup => {
                                if let Some(&last) = indices.last() {
                                    strip.insert_blocks(last + 1, 1, sup);
                                }
                            }
                            InstallSupplyPlacement::None => {}
                        }
                    }
                }
                self.inspector_spec_id = effective_spec.id.clone();
            }
            PendingInstallAction::InsertBlocks {
                strip_idx,
                at_index,
                count,
                select_inserted,
                ..
            } => {
                if let Some(strip) = self.project.strips.get_mut(strip_idx) {
                    let insert_pos = at_index.min(strip.blocks.len());
                    strip.insert_blocks(insert_pos, count, effective_spec);
                    if let Some(sup) = supply_spec {
                        match res.supply_placement {
                            InstallSupplyPlacement::BeforeGroup => {
                                strip.insert_blocks(insert_pos, 1, sup);
                                if select_inserted {
                                    self.selected_blocks =
                                        (insert_pos + 1..=insert_pos + count).collect();
                                }
                            }
                            InstallSupplyPlacement::AfterGroup => {
                                strip.insert_blocks(insert_pos + count, 1, sup);
                                if select_inserted {
                                    self.selected_blocks =
                                        (insert_pos..insert_pos + count).collect();
                                }
                            }
                            InstallSupplyPlacement::None => {}
                        }
                    } else if select_inserted {
                        self.selected_blocks = (insert_pos..insert_pos + count).collect();
                    }
                }
                self.inspector_spec_id = effective_spec.id.clone();
            }
        }
    }

    fn show_header_prefs(
        &mut self,
        ui: &mut egui::Ui,
        combo_width: f32,
        show_sidebar_toggles: bool,
    ) {
        let s = strings(self.lang);
        egui::ComboBox::from_id_salt("language_selector")
            .width(combo_width)
            .selected_text(format!("🌐 {}", self.lang.code()))
            .show_ui(ui, |ui| {
                for lang_opt in Language::ALL {
                    ui.selectable_value(&mut self.lang, lang_opt, lang_opt.display_name());
                }
            });

        let theme_btn = match self.theme_mode {
            ThemeMode::System => format!("◐ {}", s.theme_system),
            ThemeMode::Light => format!("☀ {}", s.theme_light),
            ThemeMode::Dark => format!("🌙 {}", s.theme_dark),
        };
        if ui.button(theme_btn).clicked() {
            self.theme_mode = self.theme_mode.next();
            apply_theme_mode(ui.ctx(), self.theme_mode);
        }

        if show_sidebar_toggles && self.active_tab == ActiveTab::Editor && !self.show_welcome {
            ui.separator();
            // Right-to-left layout: paint right icon first, then left icon, so visual order is [Left] [Right]
            if sidebar_toggle_icon_button(
                ui,
                false,
                self.show_inspector,
                "Rechte Seitenleiste (Eigenschaften) ein-/ausblenden",
            )
            .clicked()
            {
                self.show_inspector = !self.show_inspector;
            }
            if sidebar_toggle_icon_button(
                ui,
                true,
                self.show_left_sidebar,
                "Linke Seitenleiste (Klemmleisten) ein-/ausblenden",
            )
            .clicked()
            {
                self.show_left_sidebar = !self.show_left_sidebar;
            }
        }
    }

    /// Guided startup pipeline ("Projekt-Line"): Choose New Project -> Configure First Strip -> CAD.
    fn show_welcome_pipeline(&mut self, ui: &mut egui::Ui) {
        let s = strings(self.lang);
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(24.0);
                let card_w = (ui.available_width() - 32.0).clamp(280.0, 560.0);
                ui.vertical_centered(|ui| {
                    egui::Frame::group(ui.style())
                        .inner_margin(egui::Margin::same(20))
                        .show(ui, |ui| {
                            ui.set_width(card_w);
                            ui.heading(egui::RichText::new(s.welcome_heading).size(24.0).strong());
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(s.welcome_subtitle)
                                    .color(secondary_text_color(ui)),
                            );
                            ui.add_space(12.0);

                            // Visual 3-step pipeline indicator
                            let step2_active = self.new_strip_modal.open;
                            ui.horizontal_wrapped(|ui| {
                                let step_chip = |ui: &mut egui::Ui, label: &str, active: bool| {
                                    let bg = if active {
                                        egui::Color32::from_rgb(2, 132, 199)
                                    } else {
                                        ui.visuals().faint_bg_color
                                    };
                                    let fg = if active {
                                        egui::Color32::WHITE
                                    } else {
                                        secondary_text_color(ui)
                                    };
                                    egui::Frame::NONE
                                        .fill(bg)
                                        .corner_radius(egui::CornerRadius::same(4))
                                        .inner_margin(egui::Margin::symmetric(8, 4))
                                        .show(ui, |ui| {
                                            ui.label(egui::RichText::new(label).color(fg).strong());
                                        });
                                };
                                step_chip(ui, s.welcome_step_1, !step2_active);
                                ui.label("->");
                                step_chip(ui, s.welcome_step_2, step2_active);
                                ui.label("->");
                                step_chip(ui, s.welcome_step_3, false);
                            });

                            ui.separator();
                            ui.add_space(8.0);
                            ui.label(egui::RichText::new(s.project_name).strong());
                            ui.add(
                                egui::TextEdit::singleline(&mut self.project.project_name)
                                    .desired_width(card_w - 16.0),
                            );
                            ui.add_space(14.0);

                            // High-contrast primary actions
                            ui.horizontal_wrapped(|ui| {
                                if primary_button(ui, s.welcome_new_project).clicked() {
                                    self.project.strips.clear();
                                    self.catalog = TerminalCatalog::load_embedded();
                                    self.current_file = None;
                                    self.active_strip = 0;
                                    self.selected_blocks.clear();
                                    self.selected_end_bracket = None;
                                    self.new_strip_modal.open_for_next_strip(0);
                                }
                                if success_button(ui, s.welcome_load_project).clicked() {
                                    self.open_project();
                                }
                            });

                            ui.add_space(12.0);
                            ui.separator();
                            ui.add_space(6.0);
                            if ui.button(s.welcome_demo_project).clicked() {
                                self.project = create_demo_project(&self.catalog);
                                self.active_strip = 0;
                                self.selected_blocks.clear();
                                self.selected_end_bracket = None;
                                self.show_welcome = false;
                            }
                        });
                });
            });
        });
    }

    fn show_editor_panels(&mut self, ui: &mut egui::Ui) {
        let s = strings(self.lang);
        let avail = ui.available_size();
        // Use tabbed mobile panes on phones (both Portrait and Landscape) and folded screens;
        // use 3-column workspace on Desktop, large Tablets, and unfolded Foldables.
        let is_compact = avail.x < 960.0 || avail.y < 460.0;

        if is_compact {
            // Smartphone / Folded Cover layout: touch-friendly sub-pane bar
            egui::Panel::bottom("mobile_nav_bar").show(ui, |ui| {
                ui.add_space(4.0);
                let tab_w = ((ui.available_width() - 24.0) / 3.0).clamp(80.0, 200.0);
                ui.horizontal(|ui| {
                    ui.add_sized(
                        egui::Vec2::new(tab_w, 36.0),
                        egui::Button::selectable(
                            self.mobile_pane == MobilePane::Strips,
                            s.strips_heading,
                        ),
                    )
                    .clicked()
                    .then(|| self.mobile_pane = MobilePane::Strips);
                    ui.add_sized(
                        egui::Vec2::new(tab_w, 36.0),
                        egui::Button::selectable(self.mobile_pane == MobilePane::Cad, s.tab_editor),
                    )
                    .clicked()
                    .then(|| self.mobile_pane = MobilePane::Cad);
                    let insp_label = if self.selected_end_bracket.is_some() {
                        format!("{} (EH)", s.inspector_title)
                    } else if self.selected_blocks.is_empty() {
                        s.inspector_title.to_string()
                    } else {
                        format!("{} ({})", s.inspector_title, self.selected_blocks.len())
                    };
                    ui.add_sized(
                        egui::Vec2::new(tab_w, 36.0),
                        egui::Button::selectable(
                            self.mobile_pane == MobilePane::Inspector,
                            insp_label,
                        ),
                    )
                    .clicked()
                    .then(|| self.mobile_pane = MobilePane::Inspector);
                });
                ui.add_space(4.0);
            });

            egui::CentralPanel::default().show(ui, |ui| match self.mobile_pane {
                MobilePane::Strips => {
                    self.show_strips_sidebar_contents(ui, true);
                }
                MobilePane::Cad => {
                    self.show_cad_workspace_contents(ui, true);
                }
                MobilePane::Inspector => {
                    self.show_inspector_contents(ui, true);
                }
            });
            return;
        }

        // Desktop / Tablet / Unfolded Foldable: collapsible & freely resizable 3-column workspace
        if self.show_left_sidebar {
            egui::Panel::left("left_sidebar")
                .resizable(true)
                .default_size(240.0)
                .min_size(180.0)
                .max_size(420.0)
                .show(ui, |ui| {
                    self.show_strips_sidebar_contents(ui, false);
                });
        }

        if self.show_inspector {
            egui::Panel::right("right_inspector")
                .resizable(true)
                .default_size(350.0)
                .min_size(240.0)
                .max_size(640.0)
                .show(ui, |ui| {
                    self.show_inspector_contents(ui, false);
                });
        }

        egui::CentralPanel::default().show(ui, |ui| {
            self.show_cad_workspace_contents(ui, false);
        });
    }

    fn show_strips_sidebar_contents(&mut self, ui: &mut egui::Ui, _is_compact: bool) {
        let s = strings(self.lang);
        let avail_w = ui.available_width();
        ui.add_space(4.0);
        ui.label(egui::RichText::new(s.project_name).strong());
        ui.add(
            egui::TextEdit::singleline(&mut self.project.project_name)
                .desired_width((avail_w - 12.0).clamp(100.0, 260.0)),
        );
        ui.separator();

        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new(s.strips_heading).strong());
            if success_button(ui, s.btn_new_strip).clicked() {
                self.new_strip_modal
                    .open_for_next_strip(self.project.strips.len());
            }
        });
        ui.add_space(4.0);

        let mut delete_idx: Option<usize> = None;
        let left_max_w = (ui.available_width() - 12.0).max(120.0);
        egui::ScrollArea::both()
            .id_salt("strips_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_max_width(left_max_w);
                for (idx, strip) in self.project.strips.iter().enumerate() {
                    let is_active = idx == self.active_strip;
                    let resp = ui.add(
                        egui::Button::selectable(
                            is_active,
                            format!(
                                "{}  ({} {} / {} {})",
                                strip.name,
                                strip.blocks.len(),
                                s.blocks_unit,
                                strip.printable_label_count(),
                                s.labels_unit
                            ),
                        )
                        .wrap(),
                    );
                    if resp.clicked() {
                        self.active_strip = idx;
                        self.selected_blocks.clear();
                        self.selected_end_bracket = None;
                        self.mobile_pane = MobilePane::Cad;
                    }
                    if is_active && !strip.description.is_empty() {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(&strip.description)
                                    .color(secondary_text_color(ui)),
                            )
                            .wrap(),
                        );
                    }
                }
            });

        if !self.project.strips.is_empty() {
            ui.add_space(8.0);
            if danger_button(ui, s.btn_delete_strip).clicked() {
                delete_idx = Some(self.active_strip);
            }
        }

        if let Some(idx) = delete_idx
            && idx < self.project.strips.len()
        {
            self.project.strips.remove(idx);
            if self.active_strip >= self.project.strips.len() {
                self.active_strip = self.project.strips.len().saturating_sub(1);
            }
            self.selected_blocks.clear();
            self.selected_end_bracket = None;
        }
    }

    fn show_inspector_contents(&mut self, ui: &mut egui::Ui, _is_compact: bool) {
        let s = strings(self.lang);
        let sec = secondary_text_color(ui);
        let panel_max_w = (ui.available_width() - 16.0).max(160.0);
        egui::ScrollArea::both()
            .id_salt("inspector_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_max_width(panel_max_w);
                ui.add_space(4.0);
                ui.heading(s.inspector_title);
                ui.separator();

                let has_strip = self.active_strip < self.project.strips.len();
                if !has_strip {
                    ui.add(
                        egui::Label::new(egui::RichText::new(s.cad_empty_hint).color(sec)).wrap(),
                    );
                } else if self.canvas_state.interaction_mode == CanvasInteractionMode::BridgeAssign
                {
                    // Bridge Mode / Potentials & Calculated Jumper BOM + Virtual Pin Preview
                    // (Only shown while Bridge Mode is active; hidden in SelectBox / Pan modes.)
                    ui.label(egui::RichText::new(s.bridge_heading).strong());
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(s.bridge_assign_hint).color(sec).small(),
                        )
                        .wrap(),
                    );
                    ui.add_space(4.0);

                    let bridge_plan = {
                        let strip = &mut self.project.strips[self.active_strip];
                        ui.horizontal_wrapped(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.new_potential_name)
                                    .hint_text(s.bridge_new_potential)
                                    .desired_width(110.0),
                            );
                            if success_button(ui, s.bridge_add_btn).clicked() {
                                let name = if self.new_potential_name.trim().is_empty() {
                                    format!("P{}", strip.potentials.len() + 1)
                                } else {
                                    self.new_potential_name.trim().to_string()
                                };
                                let rgb = POTENTIAL_PALETTE
                                    [strip.potentials.len() % POTENTIAL_PALETTE.len()];
                                let id = strip.add_potential(name, rgb);
                                self.active_potential_id = Some(id);
                                self.canvas_state.interaction_mode =
                                    CanvasInteractionMode::BridgeAssign;
                            }
                        });

                        if let Some(active_id) = self.active_potential_id
                            && !strip.potentials.iter().any(|p| p.id == active_id)
                        {
                            self.active_potential_id = strip.potentials.first().map(|p| p.id);
                        }

                        let mut del_pot: Option<u64> = None;
                        for pot in &strip.potentials {
                            let is_active = self.active_potential_id == Some(pot.id);
                            let col = egui::Color32::from_rgb(
                                pot.color_rgb[0],
                                pot.color_rgb[1],
                                pot.color_rgb[2],
                            );
                            ui.horizontal_wrapped(|ui| {
                                let (dot_r, _) = ui.allocate_exact_size(
                                    egui::Vec2::splat(12.0),
                                    egui::Sense::hover(),
                                );
                                ui.painter().circle_filled(dot_r.center(), 5.5, col);
                                if ui
                                    .selectable_label(
                                        is_active,
                                        format!("{} ({})", pot.name, pot.members.len()),
                                    )
                                    .clicked()
                                {
                                    self.active_potential_id = Some(pot.id);
                                    self.canvas_state.interaction_mode =
                                        CanvasInteractionMode::BridgeAssign;
                                }
                                if ui.small_button("×").clicked() {
                                    del_pot = Some(pot.id);
                                }
                            });
                        }
                        if let Some(pid) = del_pot {
                            strip.remove_potential(pid);
                            if self.active_potential_id == Some(pid) {
                                self.active_potential_id = strip.potentials.first().map(|p| p.id);
                            }
                        }

                        if !self.selected_blocks.is_empty()
                            && let Some(active_id) = self.active_potential_id
                            && ui.button(s.bridge_assign_selected).clicked()
                        {
                            for &b_idx in &self.selected_blocks {
                                strip.toggle_potential_member(active_id, b_idx, 0);
                            }
                        }

                        calculate_bridges(strip, &self.catalog)
                    };

                    if !bridge_plan.comb_segments.is_empty()
                        || !bridge_plan.wire_bridges.is_empty()
                    {
                        ui.add_space(4.0);
                        ui.group(|ui| {
                            ui.label(egui::RichText::new(s.bridge_bom_heading).strong());
                            for seg in &bridge_plan.comb_segments {
                                let col = egui::Color32::from_rgb(
                                    seg.color_rgb[0],
                                    seg.color_rgb[1],
                                    seg.color_rgb[2],
                                );
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(
                                        egui::RichText::new(&seg.potential_name)
                                            .color(col)
                                            .strong(),
                                    );
                                    let row_name = if seg.visual_row < 0 {
                                        "PE".to_string()
                                    } else {
                                        format!("E{}", seg.visual_row + 1)
                                    };
                                    ui.add(
                                        egui::Label::new(format!(
                                            "{} [{row_name}: #{}..#{}] (K{})",
                                            seg.part_hint,
                                            seg.start_block_idx + 1,
                                            seg.end_block_idx + 1,
                                            seg.channel_slot + 1
                                        ))
                                        .wrap(),
                                    );
                                });
                                if !seg.cut_pins.is_empty() {
                                    let pins: Vec<String> =
                                        seg.cut_pins.iter().map(|i| format!("#{}", i + 1)).collect();
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(format!(
                                                "  {}: {}",
                                                s.bridge_cut_pins,
                                                pins.join(", ")
                                            ))
                                            .color(egui::Color32::from_rgb(239, 68, 68))
                                            .small(),
                                        )
                                        .wrap(),
                                    );
                                }
                                draw_virtual_jumper_strip(ui, seg);
                                if seg.uses_double_chamber_cascade {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(format!(
                                                "  {}",
                                                s.bridge_double_cascade
                                            ))
                                            .color(sec)
                                            .small(),
                                        )
                                        .wrap(),
                                    );
                                }
                                ui.add_space(4.0);
                            }
                            for wb in &bridge_plan.wire_bridges {
                                let col = egui::Color32::from_rgb(
                                    wb.color_rgb[0],
                                    wb.color_rgb[1],
                                    wb.color_rgb[2],
                                );
                                let reason_str = match wb.reason {
                                    WireBridgeReason::CrossTier => s.bridge_reason_tier,
                                    WireBridgeReason::ExceedsMaxPolesSingleSlot => {
                                        s.bridge_reason_single_max
                                    }
                                    WireBridgeReason::SingleChannelOccupied => {
                                        s.bridge_reason_single_occ
                                    }
                                    WireBridgeReason::EndPlateBarrier => {
                                        s.bridge_reason_end_plate
                                    }
                                    WireBridgeReason::IncompatiblePitchOrGap => {
                                        s.bridge_reason_pitch
                                    }
                                };
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(
                                        egui::RichText::new(&wb.potential_name)
                                            .color(col)
                                            .strong(),
                                    );
                                    ui.add(
                                        egui::Label::new(format!(
                                            "{}: #{}:E{} -> #{}:E{} ({})",
                                            s.bridge_wire_label,
                                            wb.from.0 + 1,
                                            wb.from.1 + 1,
                                            wb.to.0 + 1,
                                            wb.to.1 + 1,
                                            reason_str
                                        ))
                                        .wrap(),
                                    );
                                });
                            }
                        });
                    }
                } else if let Some(is_left) = self.selected_end_bracket {
                    // End Bracket selected on CAD canvas -> show ONLY End Bracket config + preview
                    let side_label = if is_left {
                        s.end_bracket_left
                    } else {
                        s.end_bracket_right
                    };
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(format!(
                                "{}: {}",
                                s.end_bracket_selected_hint, side_label
                            ))
                            .strong(),
                        )
                        .wrap(),
                    );
                    ui.add_space(4.0);

                    let (eb_id, keep_selected) = {
                        let strip = &mut self.project.strips[self.active_strip];
                        ui.group(|ui| {
                            show_end_bracket_picker(
                                ui,
                                &self.logos,
                                &mut strip.end_bracket_id,
                                &mut strip.end_bracket_left,
                                &mut strip.end_bracket_right,
                                s,
                                self.lang,
                                "inspector_end_bracket",
                            );
                        });
                        let keep = if is_left {
                            strip.end_bracket_left
                        } else {
                            strip.end_bracket_right
                        };
                        (strip.end_bracket_id.clone(), keep)
                    };
                    if !keep_selected {
                        self.selected_end_bracket = None;
                    }

                    ui.add_space(6.0);
                    let eb = find_end_bracket(&eb_id);
                    show_end_bracket_preview_card(
                        ui,
                        eb,
                        &self.logos,
                        &self.product_media,
                        s.open_product_page,
                        s.height_label,
                        self.lang,
                    );
                } else if !self.selected_blocks.is_empty() {
                    // Terminal Block(s) selected -> show ONLY Terminal Block tools, live preview & accessories
                    if let Some(&first_idx) = self.selected_blocks.iter().next()
                        && let Some(block) =
                            self.project.strips[self.active_strip].blocks.get(first_idx)
                    {
                        let sync_key = (self.active_strip, first_idx, block.spec_id.clone());
                        if self.last_selection_sync.as_ref() != Some(&sync_key) {
                            self.inspector_mfg_filter = Some(block.manufacturer.clone());
                            self.inspector_spec_id = block.spec_id.clone();
                            self.inspector_search.clear();
                            self.last_selection_sync = Some(sync_key);
                        }
                    } else {
                        self.last_selection_sync = None;
                    }

                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(format!(
                                "{}: {}",
                                s.selected_count,
                                self.selected_blocks.len()
                            ))
                            .strong(),
                        )
                        .wrap(),
                    );
                    ui.add_space(4.0);

                    ui.group(|ui| {
                        ui.label(egui::RichText::new(s.reassign_heading).strong());
                        show_spec_picker(
                            ui,
                            &self.catalog,
                            &self.logos,
                            &mut self.inspector_mfg_filter,
                            &mut self.inspector_search,
                            &mut self.inspector_spec_id,
                            s.filter_manufacturer,
                            s.default_terminal_type,
                            s.all_manufacturers,
                            s.search_hint,
                            self.lang,
                        );

                        // Live preview of the selected candidate type BEFORE clicking Apply / Insert
                        if let Some(candidate_spec) =
                            self.catalog.find_by_id(&self.inspector_spec_id)
                        {
                            ui.add_space(4.0);
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(s.preview_before_apply).color(sec).small(),
                                )
                                .wrap(),
                            );
                            show_product_preview_card(
                                ui,
                                candidate_spec,
                                &self.logos,
                                &self.product_media,
                                s.open_product_page,
                                s.bridge_chambers_unit,
                                s.height_label,
                            );
                        }

                        ui.add_space(4.0);

                        if primary_button(ui, s.btn_apply_type).clicked()
                            && let Some(spec) = self.catalog.find_by_id(&self.inspector_spec_id)
                        {
                            let indices: Vec<usize> =
                                self.selected_blocks.iter().copied().collect();
                            self.project.strips[self.active_strip].reassign_blocks(&indices, spec);
                        }

                        ui.horizontal_wrapped(|ui| {
                            if ui.button(s.btn_insert_before).clicked()
                                && let (Some(&first), Some(spec)) = (
                                    self.selected_blocks.iter().next(),
                                    self.catalog.find_by_id(&self.inspector_spec_id),
                                )
                            {
                                self.project.strips[self.active_strip]
                                    .insert_blocks(first, 1, spec);
                                self.selected_blocks.clear();
                                self.selected_blocks.insert(first);
                            }
                            if ui.button(s.btn_insert_after).clicked()
                                && let (Some(&last), Some(spec)) = (
                                    self.selected_blocks.iter().next_back(),
                                    self.catalog.find_by_id(&self.inspector_spec_id),
                                )
                            {
                                self.project.strips[self.active_strip]
                                    .insert_blocks(last + 1, 1, spec);
                                self.selected_blocks.clear();
                                self.selected_blocks.insert(last + 1);
                            }
                        });

                        ui.horizontal_wrapped(|ui| {
                            if ui
                                .button("◀")
                                .on_hover_text("Nach links verschieben (Alt+Links)")
                                .clicked()
                            {
                                self.selected_blocks = self.project.strips[self.active_strip]
                                    .move_selected_blocks(&self.selected_blocks, -1);
                            }
                            if ui
                                .button("▶")
                                .on_hover_text("Nach rechts verschieben (Alt+Rechts)")
                                .clicked()
                            {
                                self.selected_blocks = self.project.strips[self.active_strip]
                                    .move_selected_blocks(&self.selected_blocks, 1);
                            }
                            if ui
                                .button("⧉ Duplizieren")
                                .on_hover_text("Auswahl klonen (Strg+D)")
                                .clicked()
                            {
                                self.selected_blocks = self.project.strips[self.active_strip]
                                    .duplicate_selected_blocks(&self.selected_blocks);
                            }
                            if danger_button(ui, s.btn_delete_selected).clicked() {
                                let indices: Vec<usize> =
                                    self.selected_blocks.iter().copied().collect();
                                self.project.strips[self.active_strip].delete_blocks(&indices);
                                self.selected_blocks.clear();
                            }
                        });
                    });

                    // Single block fine-tuning (Tier order & custom overrides)
                    if self.selected_blocks.len() == 1 {
                        let idx = *self.selected_blocks.iter().next().unwrap();
                        let block_dims = self.project.strips[self.active_strip]
                            .blocks
                            .get(idx)
                            .and_then(|b| self.catalog.find_by_id(&b.spec_id))
                            .map(|sp| (sp.width_tenths_mm, sp.effective_height_tenths_mm()));

                        let strip = &mut self.project.strips[self.active_strip];
                        if idx < strip.blocks.len() {
                            ui.add_space(6.0);
                            let mut needs_recalc = false;
                            ui.group(|ui| {
                                let block = &mut strip.blocks[idx];
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{} (#{})",
                                        s.single_block_details,
                                        idx + 1
                                    ))
                                    .strong(),
                                );
                                ui.add(
                                    egui::Label::new(format!(
                                        "{} — {}",
                                        block.manufacturer, block.spec_name
                                    ))
                                    .wrap(),
                                );
                                if let Some((w10, h10)) = block_dims {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(format!(
                                                "{}: {:.1} mm | {}: {:.1} mm",
                                                s.db_col_width,
                                                f32::from(w10) / 10.0,
                                                s.height_label,
                                                f32::from(h10) / 10.0,
                                            ))
                                            .color(sec)
                                            .small(),
                                        )
                                        .wrap(),
                                    );
                                }

                                if block.tiers.len() > 1 {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.label(s.tier_order_label);
                                        let before = block.numbering_order;
                                        ui.selectable_value(
                                            &mut block.numbering_order,
                                            NumberingOrder::BottomToTop,
                                            s.order_bottom_to_top,
                                        );
                                        ui.selectable_value(
                                            &mut block.numbering_order,
                                            NumberingOrder::TopToBottom,
                                            s.order_top_to_bottom,
                                        );
                                        if block.numbering_order != before {
                                            needs_recalc = true;
                                        }
                                    });
                                }

                                ui.separator();
                                let total_tiers = block.tiers.len();
                                let pe_count =
                                    block.tiers.iter().filter(|t| t.behavior.is_pe()).count();
                                let field_w = (ui.available_width() - 24.0).clamp(90.0, 220.0);
                                for (t_idx, tier) in block.tiers.iter_mut().enumerate().rev() {
                                    let tier_title = match (total_tiers, t_idx) {
                                        (1, 0) => s.tier_lower,
                                        (2, 0) => s.tier_lower,
                                        (2, 1) => s.tier_upper,
                                        (3, 0) => s.tier_lower,
                                        (3, 1) => s.tier_middle,
                                        (3, 2) => s.tier_upper,
                                        _ => s.tier_middle,
                                    };
                                    let pe_tag = if tier.behavior.is_pe() {
                                        if pe_count > 1 {
                                            format!(" (PE{})", t_idx + 1)
                                        } else {
                                            " (PE)".to_string()
                                        }
                                    } else {
                                        String::new()
                                    };

                                    ui.add(
                                        egui::Label::new(format!(
                                            "{}{}: [{}]",
                                            tier_title, pe_tag, tier.cad_display_label
                                        ))
                                        .wrap(),
                                    );

                                    let mut text =
                                        tier.custom_override.clone().unwrap_or_default();
                                    ui.horizontal_wrapped(|ui| {
                                        let edit = egui::TextEdit::singleline(&mut text)
                                            .hint_text(s.custom_override_hint)
                                            .desired_width(field_w);
                                        if ui.add(edit).changed() {
                                            tier.custom_override = if text.is_empty() {
                                                None
                                            } else {
                                                Some(text)
                                            };
                                            needs_recalc = true;
                                        }
                                        if tier.custom_override.is_some()
                                            && ui.button(s.btn_reset_override).clicked()
                                        {
                                            tier.custom_override = None;
                                            needs_recalc = true;
                                        }
                                    });
                                }
                            });
                            if needs_recalc {
                                strip.recalculate_labels();
                            }
                        }
                    }

                // Batch prefix / suffix / special characters for all selected terminals
                ui.add_space(6.0);
                egui::CollapsingHeader::new(s.affix_heading)
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(s.affix_prefix);
                            ui.add(
                                egui::TextEdit::singleline(&mut self.batch_prefix)
                                    .desired_width(68.0),
                            );
                        });
                        ui.horizontal_wrapped(|ui| {
                            for sym in ["+", "-", "~", "=", "/", ":"] {
                                if ui.small_button(sym).clicked() {
                                    self.batch_prefix = sym.to_string();
                                }
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label(s.affix_suffix);
                            ui.add(
                                egui::TextEdit::singleline(&mut self.batch_suffix)
                                    .desired_width(68.0),
                            );
                        });
                        ui.horizontal_wrapped(|ui| {
                            for sym in ["+", "-", "~", ".1", ".2", "*", "Ω", "°"] {
                                if ui.small_button(sym).clicked() {
                                    self.batch_suffix = sym.to_string();
                                }
                            }
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.selectable_value(
                                &mut self.batch_target_row,
                                None,
                                s.affix_target_all,
                            );
                            ui.selectable_value(&mut self.batch_target_row, Some(0), "E1");
                            ui.selectable_value(&mut self.batch_target_row, Some(1), "E2");
                            ui.selectable_value(&mut self.batch_target_row, Some(2), "E3");
                            ui.selectable_value(&mut self.batch_target_row, Some(-1), "PE");
                        });
                        ui.horizontal_wrapped(|ui| {
                            if primary_button(ui, s.affix_apply).clicked() {
                                let indices: Vec<usize> =
                                    self.selected_blocks.iter().copied().collect();
                                self.project.strips[self.active_strip]
                                    .apply_affix_to_selection(
                                        &indices,
                                        self.batch_target_row,
                                        &self.batch_prefix,
                                        &self.batch_suffix,
                                    );
                            }
                            if ui.button(s.affix_clear).clicked() {
                                self.batch_prefix.clear();
                                self.batch_suffix.clear();
                                let indices: Vec<usize> =
                                    self.selected_blocks.iter().copied().collect();
                                self.project.strips[self.active_strip]
                                    .apply_affix_to_selection(
                                        &indices,
                                        self.batch_target_row,
                                        "",
                                        "",
                                    );
                            }
                        });
                    });

                // Terminal Accessories & Automatic End Plates
                ui.add_space(10.0);
                ui.separator();
                ui.label(egui::RichText::new(s.accessories_heading).strong());
                ui.add_space(4.0);

                let strip = &self.project.strips[self.active_strip];
                let plates = calculate_end_plates(strip, &self.catalog);
                if !plates.is_empty() {
                    let mut plate_groups: BTreeMap<String, (String, u16, Vec<usize>)> =
                        BTreeMap::new();
                    for p in &plates {
                        let entry = plate_groups
                            .entry(p.end_plate.clone())
                            .or_insert_with(|| {
                                (p.manufacturer.clone(), p.width_tenths_mm, Vec::new())
                            });
                        entry.2.push(p.after_block_idx + 1);
                    }
                    ui.group(|ui| {
                        ui.label(
                            egui::RichText::new(format!(
                                "{} ({})",
                                s.acc_end_plate,
                                plates.len()
                            ))
                            .strong()
                            .small(),
                        );
                        for (plate_name, (mfg, w10, positions)) in plate_groups {
                            let pos_str = positions
                                .iter()
                                .map(|i| format!("#{i}"))
                                .collect::<Vec<_>>()
                                .join(", ");
                            ui.horizontal_wrapped(|ui| {
                                show_logo_badge(ui, &self.logos, &mfg, 16.0);
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{}× {} ({:.1} mm)",
                                        positions.len(),
                                        plate_name,
                                        f32::from(w10) / 10.0
                                    ))
                                    .small(),
                                );
                                ui.label(
                                    egui::RichText::new(format!("[{pos_str}]"))
                                        .color(sec)
                                        .small(),
                                );
                            });
                        }
                    });
                    ui.add_space(4.0);
                }

                let mut seen_specs: Vec<(String, usize)> = Vec::new();
                for &b_idx in &self.selected_blocks {
                    if let Some(block) = strip.blocks.get(b_idx) {
                        if let Some((_, cnt)) =
                            seen_specs.iter_mut().find(|(id, _)| id == &block.spec_id)
                        {
                            *cnt += 1;
                        } else {
                            seen_specs.push((block.spec_id.clone(), 1));
                        }
                    }
                }

                for (spec_id, count) in seen_specs {
                    if let Some(spec) = self.catalog.find_by_id(&spec_id) {
                        let acc = self.catalog.accessories_for(spec);
                        ui.group(|ui| {
                            ui.horizontal_wrapped(|ui| {
                                show_logo_badge(ui, &self.logos, &spec.manufacturer, 18.0);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(format!(
                                            "{count}× {} ({:.1} × {}: {:.1} mm)",
                                            spec.name,
                                            f32::from(spec.width_tenths_mm) / 10.0,
                                            s.height_label,
                                            f32::from(spec.effective_height_tenths_mm()) / 10.0,
                                        ))
                                        .strong(),
                                    )
                                    .wrap(),
                                );
                            });
                            ui.add_space(4.0);

                            let pw = end_plate_width_tenths_mm(spec);
                            let end_plate_val = if pw > 0 {
                                egui::RichText::new(format!(
                                    "{} ({:.1} mm)",
                                    acc.end_plate,
                                    f32::from(pw) / 10.0
                                ))
                                .strong()
                            } else {
                                egui::RichText::new(&acc.end_plate).strong()
                            };

                            let acc_row = |ui: &mut egui::Ui, lbl: &str, val: egui::RichText| {
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(
                                        egui::RichText::new(format!("{lbl}:"))
                                            .color(sec)
                                            .small(),
                                    );
                                    ui.add(egui::Label::new(val.small()).wrap());
                                });
                            };
                            acc_row(ui, s.acc_end_plate, end_plate_val);
                            acc_row(ui, s.acc_jumpers, egui::RichText::new(&acc.jumpers));
                            acc_row(ui, s.acc_markers, egui::RichText::new(&acc.markers));
                            acc_row(ui, s.acc_end_bracket, egui::RichText::new(&acc.end_bracket));
                            if let Some(extra) = &acc.extra {
                                acc_row(ui, s.acc_extra, egui::RichText::new(extra));
                            }
                        });
                    }
                }
            } else {
                // Nothing selected in SelectBox / Pan mode
                self.last_selection_sync = None;
                ui.add(
                    egui::Label::new(egui::RichText::new(s.no_selection_hint).color(sec)).wrap(),
                );
                ui.add_space(6.0);

                let (eb_id, has_eb) = {
                    let strip = &mut self.project.strips[self.active_strip];
                    ui.group(|ui| {
                        show_end_bracket_picker(
                            ui,
                            &self.logos,
                            &mut strip.end_bracket_id,
                            &mut strip.end_bracket_left,
                            &mut strip.end_bracket_right,
                            s,
                            self.lang,
                            "inspector_end_bracket",
                        );
                    });
                    (strip.end_bracket_id.clone(), strip.end_bracket_count() > 0)
                };
                if has_eb {
                    ui.add_space(4.0);
                    let eb = find_end_bracket(&eb_id);
                    show_end_bracket_preview_card(
                        ui,
                        eb,
                        &self.logos,
                        &self.product_media,
                        s.open_product_page,
                        s.height_label,
                        self.lang,
                    );
                }

                ui.add_space(6.0);
                let strip = &self.project.strips[self.active_strip];
                let plates = calculate_end_plates(strip, &self.catalog);
                if !plates.is_empty() {
                    let mut plate_groups: BTreeMap<String, (String, u16, Vec<usize>)> =
                        BTreeMap::new();
                    for p in &plates {
                        let entry = plate_groups
                            .entry(p.end_plate.clone())
                            .or_insert_with(|| {
                                (p.manufacturer.clone(), p.width_tenths_mm, Vec::new())
                            });
                        entry.2.push(p.after_block_idx + 1);
                    }
                    ui.group(|ui| {
                        ui.label(
                            egui::RichText::new(format!(
                                "{} ({})",
                                s.acc_end_plate,
                                plates.len()
                            ))
                            .strong()
                            .small(),
                        );
                        for (plate_name, (mfg, w10, positions)) in plate_groups {
                            let pos_str = positions
                                .iter()
                                .map(|i| format!("#{i}"))
                                .collect::<Vec<_>>()
                                .join(", ");
                            ui.horizontal_wrapped(|ui| {
                                show_logo_badge(ui, &self.logos, &mfg, 16.0);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(format!(
                                            "{}× {} ({:.1} mm)",
                                            positions.len(),
                                            plate_name,
                                            f32::from(w10) / 10.0
                                        ))
                                        .small(),
                                    )
                                    .wrap(),
                                );
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(format!("[{pos_str}]"))
                                            .color(sec)
                                            .small(),
                                    )
                                    .wrap(),
                                );
                            });
                        }
                    });
                    ui.add_space(4.0);
                }

                ui.add(
                    egui::Label::new(egui::RichText::new(s.acc_none_selected).color(sec)).wrap(),
                );
            }

            ui.add_space(6.0);
            ui.separator();
            egui::CollapsingHeader::new(s.export_settings_heading)
                .default_open(false)
                .show(ui, |ui| {
                    ui.label(s.print_order_label);
                    let order_text = match self.export_options.print_order {
                        PrintOrder::ByBlockBottomToTop => s.print_order_block_btt,
                        PrintOrder::ByBlockTopToBottom => s.print_order_block_ttb,
                        PrintOrder::ByTierStrip => s.print_order_tier_strip,
                    };
                    let combo_w = (ui.available_width() - 16.0).clamp(100.0, 280.0);
                    egui::ComboBox::from_id_salt("print_order_combo")
                        .width(combo_w)
                        .truncate()
                        .selected_text(order_text)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.export_options.print_order,
                                PrintOrder::ByBlockBottomToTop,
                                s.print_order_block_btt,
                            );
                            ui.selectable_value(
                                &mut self.export_options.print_order,
                                PrintOrder::ByBlockTopToBottom,
                                s.print_order_block_ttb,
                            );
                            ui.selectable_value(
                                &mut self.export_options.print_order,
                                PrintOrder::ByTierStrip,
                                s.print_order_tier_strip,
                            );
                        });

                    ui.checkbox(
                        &mut self.export_options.include_pe_labels,
                        s.include_pe_checkbox,
                    );
                    ui.checkbox(
                        &mut self.export_options.all_strips,
                        s.export_all_strips_checkbox,
                    );
                });
        });
    }

    fn show_cad_workspace_contents(&mut self, ui: &mut egui::Ui, is_compact: bool) {
        let s = strings(self.lang);
        if self.active_strip >= self.project.strips.len() {
            ui.centered_and_justified(|ui| {
                ui.label(s.cad_empty_hint);
            });
            return;
        }

        // Strip action toolbar
        let mut recalc_needed = false;
        let mut append_spec_id: Option<String> = None;

        ui.horizontal_wrapped(|ui| {
            // CAD Tool Switcher: Pan/Tap (mobile/touch) vs Box Select vs Bridge Mode
            #[cfg(any(target_os = "android", target_os = "ios"))]
            ui.selectable_value(
                &mut self.canvas_state.interaction_mode,
                CanvasInteractionMode::Pan,
                s.mode_pan,
            )
            .on_hover_text(s.mode_pan_tip);
            ui.selectable_value(
                &mut self.canvas_state.interaction_mode,
                CanvasInteractionMode::SelectBox,
                s.mode_select,
            )
            .on_hover_text(s.mode_select_tip);
            if ui
                .selectable_value(
                    &mut self.canvas_state.interaction_mode,
                    CanvasInteractionMode::BridgeAssign,
                    s.mode_bridge,
                )
                .on_hover_text(s.mode_bridge_tip)
                .clicked()
            {
                self.selected_end_bracket = None;
                let strip = &mut self.project.strips[self.active_strip];
                if strip.potentials.is_empty() {
                    let id = strip.add_potential("+24V", POTENTIAL_PALETTE[0]);
                    self.active_potential_id = Some(id);
                } else if self.active_potential_id.is_none() {
                    self.active_potential_id = strip.potentials.first().map(|p| p.id);
                }
            }

            ui.separator();
            let strip = &mut self.project.strips[self.active_strip];
            ui.label(s.strip_start_number);
            if ui
                .add(egui::DragValue::new(&mut strip.start_number).range(1..=99999))
                .changed()
            {
                recalc_needed = true;
            }

            ui.separator();
            ui.label(s.n_mode_label);
            let n_text = match strip.neutral_mode {
                NeutralLabelMode::Fixed => s.n_mode_fixed,
                NeutralLabelMode::SharedSequenceNumber => s.n_mode_shared_num,
                NeutralLabelMode::SharedSequenceSuffixN => s.n_mode_shared_n,
                NeutralLabelMode::SeparateCounterSuffixN => s.n_mode_own_n,
            };
            egui::ComboBox::from_id_salt("n_mode_combo")
                .truncate()
                .selected_text(n_text)
                .show_ui(ui, |ui| {
                    for (mode, lbl) in [
                        (NeutralLabelMode::Fixed, s.n_mode_fixed),
                        (NeutralLabelMode::SharedSequenceNumber, s.n_mode_shared_num),
                        (NeutralLabelMode::SharedSequenceSuffixN, s.n_mode_shared_n),
                        (NeutralLabelMode::SeparateCounterSuffixN, s.n_mode_own_n),
                    ] {
                        if ui.selectable_value(&mut strip.neutral_mode, mode, lbl).changed() {
                            recalc_needed = true;
                        }
                    }
                });

            ui.label(s.pe_mode_label);
            let pe_text = match strip.pe_mode {
                PeLabelMode::Fixed => s.pe_mode_fixed,
                PeLabelMode::SharedSequenceNumber => s.pe_mode_shared_num,
                PeLabelMode::SharedSequenceSuffixPe => s.pe_mode_shared_pe,
                PeLabelMode::SeparateCounterSuffixPe => s.pe_mode_own_pe,
            };
            egui::ComboBox::from_id_salt("pe_mode_combo")
                .truncate()
                .selected_text(pe_text)
                .show_ui(ui, |ui| {
                    for (mode, lbl) in [
                        (PeLabelMode::Fixed, s.pe_mode_fixed),
                        (PeLabelMode::SharedSequenceNumber, s.pe_mode_shared_num),
                        (PeLabelMode::SharedSequenceSuffixPe, s.pe_mode_shared_pe),
                        (PeLabelMode::SeparateCounterSuffixPe, s.pe_mode_own_pe),
                    ] {
                        if ui.selectable_value(&mut strip.pe_mode, mode, lbl).changed() {
                            recalc_needed = true;
                        }
                    }
                });

            ui.separator();
            ui.label(s.zoom_label);
            if ui
                .button("−")
                .on_hover_text("Verkleinern")
                .clicked()
            {
                self.canvas_state.zoom = (self.canvas_state.zoom / 1.15).clamp(0.4, 2.5);
            }
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = if is_compact { 76.0 } else { 96.0 };
                ui.visuals_mut().widgets.inactive.bg_fill = if ui.visuals().dark_mode {
                    egui::Color32::from_rgb(51, 65, 85)
                } else {
                    egui::Color32::from_rgb(203, 213, 225)
                };
                ui.add(
                    egui::Slider::new(&mut self.canvas_state.zoom, 0.4..=2.5).show_value(false),
                );
            });
            if ui
                .button("+")
                .on_hover_text("Vergrößern")
                .clicked()
            {
                self.canvas_state.zoom = (self.canvas_state.zoom * 1.15).clamp(0.4, 2.5);
            }
            ui.label(
                egui::RichText::new(format!("{:>3.0}%", self.canvas_state.zoom * 100.0))
                    .monospace()
                    .strong(),
            );
            if ui.button(format!("↺ {}", s.btn_fit_zoom)).clicked() {
                self.canvas_state.zoom = 1.0;
            }

            ui.separator();
            if ui.button(s.btn_select_all).clicked() {
                self.selected_blocks = (0..strip.blocks.len()).collect();
                self.selected_end_bracket = None;
            }
            if ui.button(s.btn_clear_selection).clicked() {
                self.selected_blocks.clear();
                self.selected_end_bracket = None;
            }

            ui.separator();
            if success_button(ui, s.btn_append_block).clicked() {
                append_spec_id = Some(strip.default_spec_id.clone());
            }

            ui.separator();
            ui.label(s.type_display_label);
            let td_text = match self.canvas_state.type_display {
                TypeDisplayMode::HorizontalGroups => s.type_display_groups,
                TypeDisplayMode::VerticalPerBlock => s.type_display_vertical,
                TypeDisplayMode::Off => s.type_display_off,
            };
            egui::ComboBox::from_id_salt("type_display_combo")
                .truncate()
                .selected_text(td_text)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.canvas_state.type_display,
                        TypeDisplayMode::HorizontalGroups,
                        s.type_display_groups,
                    );
                    ui.selectable_value(
                        &mut self.canvas_state.type_display,
                        TypeDisplayMode::VerticalPerBlock,
                        s.type_display_vertical,
                    );
                    ui.selectable_value(
                        &mut self.canvas_state.type_display,
                        TypeDisplayMode::Off,
                        s.type_display_off,
                    );
                });

            ui.separator();
            let eplaner_btn = egui::Button::new(s.mode_eplaner)
                .selected(self.canvas_state.e_planer_mode);
            if ui
                .add(eplaner_btn)
                .on_hover_text(s.mode_eplaner_tip)
                .clicked()
            {
                self.canvas_state.e_planer_mode = !self.canvas_state.e_planer_mode;
                if self.canvas_state.e_planer_mode
                    && self.canvas_state.interaction_mode == CanvasInteractionMode::BridgeAssign
                {
                    self.canvas_state.interaction_mode = CanvasInteractionMode::SelectBox;
                }
            }
            if is_compact
                && (!self.selected_blocks.is_empty() || self.selected_end_bracket.is_some())
            {
                ui.separator();
                if primary_button(ui, "✏ Bearbeiten").clicked() {
                    self.mobile_pane = MobilePane::Inspector;
                }
            }
        });

        if recalc_needed {
            self.project.strips[self.active_strip].recalculate_labels();
        }
        if let Some(spec_id) = append_spec_id
            && let Some(spec) = self.catalog.find_by_id(&spec_id)
        {
            if spec.is_installation_or_nt_terminal() {
                let has_supply = self.strip_has_n_supply(self.active_strip);
                let len = self.project.strips[self.active_strip].blocks.len();
                self.install_prompt_modal.open_for_action(
                    PendingInstallAction::InsertBlocks {
                        strip_idx: self.active_strip,
                        at_index: len,
                        count: 1,
                        spec_id: spec.id.clone(),
                        select_inserted: false,
                    },
                    spec,
                    has_supply,
                );
            } else {
                self.project.strips[self.active_strip].append_blocks(1, spec);
            }
        }

        // Inline Potential Quick-Bar directly inside CAD workspace when Bridge Mode is active
        if self.canvas_state.interaction_mode == CanvasInteractionMode::BridgeAssign {
            let strip = &mut self.project.strips[self.active_strip];
            if strip.potentials.is_empty() {
                let id = strip.add_potential("+24V", POTENTIAL_PALETTE[0]);
                self.active_potential_id = Some(id);
            } else if self
                .active_potential_id
                .is_none_or(|id| !strip.potentials.iter().any(|p| p.id == id))
            {
                self.active_potential_id = strip.potentials.first().map(|p| p.id);
            }

            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new(s.bridge_heading)
                        .strong()
                        .small(),
                );
                let mut remove_id: Option<u64> = None;
                for pot in &strip.potentials {
                    let is_active = self.active_potential_id == Some(pot.id);
                    let rgb = pot.color_rgb;
                    let col = egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
                    let lbl = egui::RichText::new(format!("{} ({})", pot.name, pot.members.len()))
                        .small();
                    let btn = egui::Button::new(lbl)
                        .selected(is_active)
                        .stroke(egui::Stroke::new(
                            if is_active { 2.0 } else { 1.0 },
                            col,
                        ));
                    if ui.add(btn).clicked() {
                        self.active_potential_id = Some(pot.id);
                    }
                    if is_active && strip.potentials.len() > 1 && ui.small_button("×").clicked() {
                        remove_id = Some(pot.id);
                    }
                }
                if let Some(id) = remove_id {
                    strip.remove_potential(id);
                    if self.active_potential_id == Some(id) {
                        self.active_potential_id = strip.potentials.first().map(|p| p.id);
                    }
                }

                ui.separator();
                ui.add(
                    egui::TextEdit::singleline(&mut self.new_potential_name)
                        .hint_text(s.bridge_new_potential)
                        .desired_width(110.0),
                );
                if success_button(ui, s.bridge_add_btn).clicked() {
                    let name = if self.new_potential_name.trim().is_empty() {
                        format!("P{}", strip.potentials.len() + 1)
                    } else {
                        self.new_potential_name.trim().to_string()
                    };
                    let color = POTENTIAL_PALETTE[strip.potentials.len() % POTENTIAL_PALETTE.len()];
                    let id = strip.add_potential(&name, color);
                    self.active_potential_id = Some(id);
                    self.new_potential_name.clear();
                }

                ui.label(
                    egui::RichText::new(s.bridge_assign_hint)
                        .color(secondary_text_color(ui))
                        .small(),
                );
            });
        }

        ui.separator();

        let strip = &mut self.project.strips[self.active_strip];
        let bridge_plan = if strip.potentials.is_empty() {
            None
        } else {
            Some(calculate_bridges(strip, &self.catalog))
        };
        let end_plates = calculate_end_plates(strip, &self.catalog);
        show_cad_canvas(
            ui,
            strip,
            &self.catalog,
            &mut self.selected_blocks,
            &mut self.selected_end_bracket,
            &mut self.canvas_state,
            s,
            bridge_plan.as_ref(),
            self.active_potential_id,
            &end_plates,
        );
    }
}

/// Builds a realistic initial project showcasing mixed multi-level numbering right on launch.
fn create_demo_project(catalog: &TerminalCatalog) -> Project {
    let single = catalog
        .find_by_id("wago-2002-1201")
        .expect("WAGO 2002-1201 must exist");
    let double_ll = catalog
        .find_by_id("wago-2002-2201")
        .expect("WAGO 2002-2201 must exist");
    let install_peln = catalog
        .find_by_id("wago-2003-7641")
        .expect("WAGO 2003-7641 must exist");

    // Strip -X1: 6x 2003-7641 (PE/L/N), 3x 2002-2201 (L/L), 5x 2002-1201 (L)
    let mut x1 = TerminalStrip::generate_with_block_count("-X1", 1, 20, 14, single);
    x1.description = "Installations- & Ausgangsklemmen (PE-Zeile unten ausgerichtet)".to_string();
    x1.reassign_blocks(&[0, 1, 2, 3, 4, 5], install_peln);
    x1.reassign_blocks(&[6, 7, 8], double_ll);

    Project {
        format_version: 1,
        project_name: "Schaltschrank_Hauptverteilung".to_string(),
        strips: vec![x1],
        ..Default::default()
    }
}

fn sanitize_filename(name: &str) -> String {
    let clean: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if clean.is_empty() {
        "TermiGrid".to_string()
    } else {
        clean
    }
}

/// Renders a crisp virtual side-profile of a comb jumper bar (`Kammbrücke`),
/// showing every single pin position (`#start..#end`), active copper/silver legs,
/// and clearly crossed-out (`×`) removed pins directly in the inspector sidebar.
fn draw_virtual_jumper_strip(ui: &mut egui::Ui, seg: &CombJumperSegment) {
    let poles = seg.pole_count.max(2);
    let cell_w = ((ui.available_width() - 12.0) / (poles as f32)).clamp(14.0, 26.0);
    let total_w = (cell_w * poles as f32).min((ui.available_width() - 8.0).max(120.0));
    let height = 38.0;

    let (rect, _) = ui.allocate_exact_size(egui::Vec2::new(total_w, height), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    let pot_color =
        egui::Color32::from_rgb(seg.color_rgb[0], seg.color_rgb[1], seg.color_rgb[2]);
    let dark = ui.visuals().dark_mode;

    // Insulated upper grip spine
    let spine = egui::Rect::from_min_max(
        egui::Pos2::new(rect.min.x + 2.0, rect.min.y + 3.0),
        egui::Pos2::new(rect.max.x - 2.0, rect.min.y + 13.0),
    );
    painter.rect_filled(spine, egui::CornerRadius::same(2), pot_color);
    painter.rect_stroke(
        spine,
        egui::CornerRadius::same(2),
        egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 180)),
        egui::StrokeKind::Inside,
    );

    let step = rect.width() / (poles as f32);
    for offset in 0..poles {
        let block_idx = seg.start_block_idx + offset;
        let cx = rect.min.x + (offset as f32 + 0.5) * step;
        let is_cut = seg.cut_pins.contains(&block_idx);

        if is_cut {
            // Broken/removed pin marked with red circle & white X
            let r = 5.5;
            let c = egui::Pos2::new(cx, rect.min.y + 20.0);
            painter.circle_filled(c, r, egui::Color32::from_rgb(220, 38, 38));
            painter.line_segment(
                [egui::Pos2::new(c.x - 3.0, c.y - 3.0), egui::Pos2::new(c.x + 3.0, c.y + 3.0)],
                egui::Stroke::new(1.6, egui::Color32::WHITE),
            );
            painter.line_segment(
                [egui::Pos2::new(c.x - 3.0, c.y + 3.0), egui::Pos2::new(c.x + 3.0, c.y - 3.0)],
                egui::Stroke::new(1.6, egui::Color32::WHITE),
            );
        } else {
            // Solid metallic contact pin descending from spine
            let pin_rect = egui::Rect::from_min_max(
                egui::Pos2::new(cx - 2.2, rect.min.y + 13.0),
                egui::Pos2::new(cx + 2.2, rect.min.y + 25.5),
            );
            painter.rect_filled(
                pin_rect,
                egui::CornerRadius::same(1),
                egui::Color32::from_rgb(203, 213, 225),
            );
            painter.rect_stroke(
                pin_rect,
                egui::CornerRadius::same(1),
                egui::Stroke::new(0.8, egui::Color32::from_rgb(15, 23, 42)),
                egui::StrokeKind::Inside,
            );
        }

        painter.text(
            egui::Pos2::new(cx, rect.max.y - 1.0),
            egui::Align2::CENTER_BOTTOM,
            format!("{}", block_idx + 1),
            egui::FontId::proportional(8.5),
            if is_cut {
                egui::Color32::from_rgb(239, 68, 68)
            } else if dark {
                egui::Color32::from_rgb(203, 213, 225)
            } else {
                egui::Color32::from_rgb(51, 65, 85)
            },
        );
    }
}

/// Renders a crisp, modern vector layout-panel toggle icon (inspired by Antigravity / VS Code)
/// with a filled left/right sidebar pane slice when `open` is true and an outline when collapsed.
fn sidebar_toggle_icon_button(
    ui: &mut egui::Ui,
    is_left: bool,
    open: bool,
    tooltip: &str,
) -> egui::Response {
    let size = egui::Vec2::new(26.0, 22.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        if hovered {
            ui.painter().rect_filled(
                rect,
                egui::CornerRadius::same(4),
                ui.visuals().widgets.hovered.bg_fill,
            );
        }

        let icon_rect = egui::Rect::from_center_size(rect.center(), egui::Vec2::new(16.0, 12.5));
        let fg = if hovered {
            ui.visuals().strong_text_color()
        } else if open {
            ui.visuals().text_color()
        } else {
            secondary_text_color(ui)
        };

        let split_x = if is_left {
            icon_rect.min.x + icon_rect.width() * 0.36
        } else {
            icon_rect.max.x - icon_rect.width() * 0.36
        };

        if open {
            let pane_rect = if is_left {
                egui::Rect::from_min_max(
                    icon_rect.min,
                    egui::Pos2::new(split_x, icon_rect.max.y),
                )
            } else {
                egui::Rect::from_min_max(
                    egui::Pos2::new(split_x, icon_rect.min.y),
                    icon_rect.max,
                )
            };
            ui.painter()
                .rect_filled(pane_rect, egui::CornerRadius::same(1), fg);
        }

        ui.painter().rect_stroke(
            icon_rect,
            egui::CornerRadius::same(2),
            egui::Stroke::new(1.3, fg),
            egui::StrokeKind::Inside,
        );
        ui.painter().line_segment(
            [
                egui::Pos2::new(split_x, icon_rect.min.y + 1.0),
                egui::Pos2::new(split_x, icon_rect.max.y - 1.0),
            ],
            egui::Stroke::new(1.3, fg),
        );
    }
    response.on_hover_text(tooltip)
}

