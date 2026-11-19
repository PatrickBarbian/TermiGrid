pub mod android_insets;
pub mod cad_canvas;
pub mod components;
pub mod guide_modal;
pub mod icons;
pub mod library_modal;
pub mod logos;
pub mod net;
pub mod new_strip_modal;
pub mod pdf_import_modal;
pub mod platform_io;
pub mod product_media;
pub mod theme;
pub mod updater;

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::PathBuf;

use eframe::egui;

use crate::catalog::accessories::{
    calculate_end_plates, end_plate_width_tenths_mm, find_end_bracket,
};
use crate::catalog::{TerminalCatalog, TerminalSpec};
use crate::export::{
    csv::export_to_csv, dxf::export_to_dxf, xlsx::export_to_xlsx, ExportOptions, PrintOrder,
};
use crate::i18n::{strings, Language};
use crate::import::{import_pdf_bytes, import_pdf_path, planner::instantiate_imported_strip};
use crate::model::bridging::{
    calculate_bridges, CombJumperSegment, WireBridgeReason, POTENTIAL_PALETTE,
};
use crate::model::library::{
    instantiate_strip_from_template, load_global_custom_specs, merge_custom_specs_into,
    merge_template_custom_specs, save_global_custom_specs, StripLibrary,
};
use crate::model::{
    NeutralLabelMode, NumberingOrder, PeLabelMode, Project, TerminalStrip,
};
use crate::ui::android_insets::{current_safe_insets, SafeAreaInsets};
use crate::ui::cad_canvas::{
    draw_mini_eplan_preview, show_cad_canvas, CadCanvasState, CanvasInteractionMode,
    TypeDisplayMode,
};
use crate::ui::components::{
    CommandPaletteState, PaletteCommand, ToastActionKind, ToastQueue, ToastTone,
};
use crate::ui::guide_modal::{
    show_about_modal, show_database_tab, show_guide_tab, CustomSpecDraft,
};
use crate::ui::icons::{
    danger_icon_button, draw_icon, ghost_icon_button, icon_button, icon_only_button,
    primary_icon_button, segmented_control, selectable_icon_button, show_responsive_modal,
    subtle_success_icon_button, UiIcon,
};
use crate::ui::library_modal::{
    show_save_strip_to_library_modal, show_strip_library_modal, SaveStripLibraryDialogState,
    StripLibraryAction, StripLibraryDialogState,
};
use crate::ui::logos::{show_logo_badge, BrandLogos};
use crate::ui::new_strip_modal::{
    show_end_bracket_picker, show_install_supply_prompt_modal, show_new_strip_modal,
    show_spec_picker, InstallPromptDialogState, InstallPromptResolution, InstallSupplyPlacement,
    NewStripDialogState, PendingInstallAction,
};
use crate::ui::pdf_import_modal::{show_pdf_import_modal, PdfImportDialogState};
use crate::ui::platform_io::{
    pick_export_csv_path, pick_export_dxf_path, pick_export_xlsx_path, pick_import_pdf_path,
    pick_open_project_path, pick_save_project_path,
};
use crate::ui::product_media::{
    show_end_bracket_preview_card, show_product_preview_card, ProductMediaCache,
};
use crate::ui::theme::{
    apply_theme_mode, install_system_fonts, kbd_chip, pill_badge, recessed_card,
    secondary_text_color, section_card, tokens, ThemeMode,
};
use crate::ui::updater::{
    open_external_url, UpdateChecker, UpdateStatus, BUYMEACOFFEE_URL, CURRENT_VERSION,
    GITHUB_REPO_URL,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum SidebarDeckTab {
    #[default]
    Strips,
    Library,
}

#[derive(Debug, Clone)]
struct NtQueueEntry {
    strip_idx: usize,
    strip_bmk: String,
    anchor_start: usize,
    anchor_end: usize,
    base_spec_id: String,
    nt_hints: Vec<String>,
    ordinal: usize,
    total: usize,
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

    // Stacked left sidebar ("Klemmleisten" <-> "Bibliothek" sheet deck)
    sidebar_deck_tab: SidebarDeckTab,
    sidebar_lib_search: String,

    // Unsaved-changes guard when closing the window
    clean_snapshot: Project,
    show_close_confirm_modal: bool,
    allow_close: bool,

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

    // Personal Terminal Strip Library & global custom specs (persisted across all projects on this PC)
    strip_library: StripLibrary,
    global_custom_specs: Vec<TerminalSpec>,
    save_strip_library_modal: SaveStripLibraryDialogState,
    strip_library_modal: StripLibraryDialogState,

    // Intelligent PDF BOM Importer & sequential NT configuration queue
    pdf_import_modal: PdfImportDialogState,
    nt_popup_queue: VecDeque<NtQueueEntry>,

    // Modern UI components (Sonner-style toasts + Strg+K command palette)
    toast_queue: ToastQueue,
    command_palette: CommandPaletteState,
    ui_time: f64,
}

impl TermiGridApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut catalog = TerminalCatalog::load_embedded();
        let logos = BrandLogos::load(&cc.egui_ctx);
        let project = Project::default();
        let last_snapshot = project.clone();
        let clean_snapshot = project.clone();
        let strip_library = StripLibrary::load_default();
        let global_custom_specs = load_global_custom_specs();
        if !global_custom_specs.is_empty() {
            catalog.reload_with_custom(&global_custom_specs);
        }

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
            sidebar_deck_tab: SidebarDeckTab::Strips,
            sidebar_lib_search: String::new(),
            clean_snapshot,
            show_close_confirm_modal: false,
            allow_close: false,
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
            strip_library,
            global_custom_specs,
            save_strip_library_modal: SaveStripLibraryDialogState::default(),
            strip_library_modal: StripLibraryDialogState::default(),
            pdf_import_modal: PdfImportDialogState::default(),
            nt_popup_queue: VecDeque::new(),
            toast_queue: ToastQueue::default(),
            command_palette: CommandPaletteState::default(),
            ui_time: 0.0,
        }
    }

    fn has_unsaved_changes(&self) -> bool {
        if self.show_welcome && self.project.strips.is_empty() {
            return false;
        }
        self.project != self.clean_snapshot
    }

    fn notify(&mut self, tone: ToastTone, msg: impl Into<String>) {
        let text = msg.into();
        self.status_message = text.clone();
        self.toast_queue.push(self.ui_time, tone, text);
    }

    fn notify_undoable(&mut self, tone: ToastTone, msg: impl Into<String>) {
        let text = msg.into();
        self.status_message = text.clone();
        self.toast_queue.push_with_action(
            self.ui_time,
            tone,
            text,
            Some(("Rückgängig".to_string(), ToastActionKind::Undo)),
        );
    }

    fn reload_catalog_with_all_custom(&mut self) {
        let mut combined = self.global_custom_specs.clone();
        merge_custom_specs_into(&self.project.custom_specs, &mut combined);
        self.catalog.reload_with_custom(&combined);
    }

    fn perform_undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.project.clone());
            self.project = prev;
            self.reload_catalog_with_all_custom();
            self.last_snapshot = self.project.clone();
            self.active_strip = self
                .active_strip
                .min(self.project.strips.len().saturating_sub(1));
            self.selected_blocks.clear();
            self.selected_end_bracket = None;
            self.notify(ToastTone::Info, "Schritt rückgängig gemacht.");
        }
    }

    fn perform_redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.project.clone());
            self.project = next;
            self.reload_catalog_with_all_custom();
            self.last_snapshot = self.project.clone();
            self.active_strip = self
                .active_strip
                .min(self.project.strips.len().saturating_sub(1));
            self.selected_blocks.clear();
            self.selected_end_bracket = None;
            self.notify(ToastTone::Info, "Schritt wiederhergestellt.");
        }
    }

    fn save_project(&mut self) -> bool {
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
                        self.clean_snapshot = self.project.clone();
                        let name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .map(str::to_string)
                            .unwrap_or_else(|| path.display().to_string());
                        self.notify(ToastTone::Success, format!("{}: {name}", s.status_saved));
                        return true;
                    }
                    Err(e) => {
                        self.notify(ToastTone::Error, format!("{}: {e}", s.status_error));
                    }
                },
                Err(e) => {
                    self.notify(ToastTone::Error, format!("{}: {e}", s.status_error));
                }
            }
        }
        false
    }

    fn open_project(&mut self) {
        let s = strings(self.lang);
        if let Some(path) = pick_open_project_path() {
            match std::fs::read_to_string(&path) {
                Ok(raw) => match serde_json::from_str::<Project>(&raw) {
                    Ok(mut proj) => {
                        proj.normalize();
                        self.project = proj;
                        self.reload_catalog_with_all_custom();
                        self.undo_stack.clear();
                        self.redo_stack.clear();
                        self.last_snapshot = self.project.clone();
                        self.clean_snapshot = self.project.clone();
                        self.active_strip = 0;
                        self.selected_blocks.clear();
                        self.selected_end_bracket = None;
                        self.current_file = Some(path.clone());
                        self.show_welcome = false;
                        self.active_tab = ActiveTab::Editor;
                        let name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .map(str::to_string)
                            .unwrap_or_else(|| path.display().to_string());
                        self.notify(ToastTone::Success, format!("{}: {name}", s.status_loaded));
                    }
                    Err(e) => {
                        self.notify(ToastTone::Error, format!("{}: {e}", s.status_error));
                    }
                },
                Err(e) => {
                    self.notify(ToastTone::Error, format!("{}: {e}", s.status_error));
                }
            }
        }
    }

    fn insert_library_template(&mut self, template_id: &str, target_designation: Option<&str>) {
        if let Some(tpl) = self.strip_library.find_by_id(template_id).cloned() {
            if self.show_welcome {
                self.project.strips.clear();
                self.project.custom_specs.clear();
                self.reload_catalog_with_all_custom();
                self.undo_stack.clear();
                self.redo_stack.clear();
                self.current_file = None;
            }
            if merge_template_custom_specs(&tpl, &mut self.project.custom_specs) {
                if merge_custom_specs_into(&tpl.custom_specs, &mut self.global_custom_specs) {
                    let _ = save_global_custom_specs(&self.global_custom_specs);
                }
                self.reload_catalog_with_all_custom();
            }
            let new_strip = instantiate_strip_from_template(
                &tpl,
                target_designation,
                &self.project.strips,
            );
            let inserted_name = new_strip.name.clone();
            self.project.strips.push(new_strip);
            self.active_strip = self.project.strips.len().saturating_sub(1);
            self.selected_blocks.clear();
            self.selected_end_bracket = None;
            self.show_welcome = false;
            self.active_tab = ActiveTab::Editor;
            self.mobile_pane = MobilePane::Cad;
            self.notify(
                ToastTone::Success,
                format!(
                    "Klemmleiste „{}“ aus Vorlage „{}“ eingefügt.",
                    inserted_name, tpl.name
                ),
            );
        }
    }

    fn import_pdf_dialog(&mut self) {
        if let Some(path) = pick_import_pdf_path() {
            let label = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Stückliste.pdf")
                .to_string();
            match import_pdf_path(&path, &self.catalog) {
                Ok(plan) => {
                    self.pdf_import_modal.open_with_plan(label, plan);
                }
                Err(err) => {
                    self.pdf_import_modal.open_with_error(label, err.clone());
                    self.notify(ToastTone::Error, format!("PDF-Import fehlgeschlagen: {err}"));
                }
            }
        }
    }

    fn import_pdf_bytes_with_label(&mut self, source_label: String, bytes: &[u8]) {
        match import_pdf_bytes(bytes, &self.catalog) {
            Ok(plan) => {
                self.pdf_import_modal.open_with_plan(source_label, plan);
            }
            Err(err) => {
                self.pdf_import_modal
                    .open_with_error(source_label, err.clone());
                self.notify(ToastTone::Error, format!("PDF-Import fehlgeschlagen: {err}"));
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
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(str::to_string)
                        .unwrap_or_else(|| path.display().to_string());
                    self.notify(
                        ToastTone::Success,
                        format!("{}: {name} ({count} {})", s.status_exported, s.labels_unit),
                    );
                }
                Err(e) => {
                    self.notify(ToastTone::Error, format!("{}: {e}", s.status_error));
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
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(str::to_string)
                        .unwrap_or_else(|| path.display().to_string());
                    self.notify(
                        ToastTone::Success,
                        format!("{}: {name} ({count} {})", s.status_exported, s.labels_unit),
                    );
                }
                Err(e) => {
                    self.notify(ToastTone::Error, format!("{}: {e}", s.status_error));
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
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(str::to_string)
                        .unwrap_or_else(|| path.display().to_string());
                    self.notify(
                        ToastTone::Success,
                        format!(
                            "{}: {name} ({strip_count} {}, 1:1 mm)",
                            s.status_exported, s.strips_heading
                        ),
                    );
                }
                Err(e) => {
                    self.notify(ToastTone::Error, format!("{}: {e}", s.status_error));
                }
            }
        }
    }
}

impl eframe::App for TermiGridApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let s = strings(self.lang);
        let t = tokens(ui);
        self.ui_time = ui.ctx().input(|i| i.time);

        // Prompt Save / Discard when closing with unsaved project changes
        if ui.ctx().input(|i| i.viewport().close_requested())
            && !self.allow_close
            && self.has_unsaved_changes()
        {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.show_close_confirm_modal = true;
        }

        // Drag & Drop support for EPLAN PDF parts lists
        let dropped_files = ui.ctx().input(|i| i.raw.dropped_files.clone());
        for file in dropped_files {
            let path = file.path();
            let is_pdf = path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("pdf"));
            if is_pdf {
                let label = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Stückliste.pdf")
                    .to_string();
                match file.bytes() {
                    Ok(bytes) => self.import_pdf_bytes_with_label(label, &bytes),
                    Err(e) => self.pdf_import_modal.open_with_error(
                        label,
                        format!("PDF-Datei konnte nicht gelesen werden: {e}"),
                    ),
                }
                break;
            }
        }

        // Global keyboard shortcuts when no modal is blocking
        let any_modal_open = self.new_strip_modal.open
            || self.install_prompt_modal.open
            || self.pdf_import_modal.open
            || self.strip_library_modal.open
            || self.save_strip_library_modal.open
            || self.show_about_modal
            || self.show_close_confirm_modal;

        if !any_modal_open
            && ui.input(|i| (i.modifiers.ctrl || i.modifiers.mac_cmd) && i.key_pressed(egui::Key::K))
        {
            if self.command_palette.open {
                self.command_palette.close();
            } else {
                self.command_palette.open_now();
            }
        }

        if !any_modal_open
            && !self.command_palette.open
            && !ui.ctx().egui_wants_keyboard_input()
            && self.canvas_state.editing_tier.is_none()
        {
            let (undo_pressed, redo_pressed, move_left, move_right, duplicate_pressed, delete_pressed) =
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
                        !ctrl && !alt && i.key_pressed(egui::Key::Delete),
                    )
                });
            if undo_pressed {
                self.perform_undo();
            } else if redo_pressed {
                self.perform_redo();
            } else if self.active_tab == ActiveTab::Editor
                && !self.show_welcome
                && self.active_strip < self.project.strips.len()
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
                } else if delete_pressed {
                    let indices: Vec<usize> = self.selected_blocks.iter().copied().collect();
                    let deleted_count = indices.len();
                    self.project.strips[self.active_strip].delete_blocks(&indices);
                    self.selected_blocks.clear();
                    self.selected_end_bracket = None;
                    self.canvas_state.selection_anchor = None;
                    self.notify_undoable(
                        ToastTone::Info,
                        format!("{deleted_count} Klemme(n) gelöscht."),
                    );
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

        // Top Menu & Calm Toolbar
        egui::Panel::top("top_bar")
            .frame(
                egui::Frame::NONE
                    .fill(t.bg_panel)
                    .stroke(egui::Stroke::new(1.0, t.border_hairline))
                    .inner_margin(egui::Margin::symmetric(10, 6)),
            )
            .show(ui, |ui| {
                let avail_w = ui.available_width();
                let wide_header = avail_w >= 820.0;

                let has_update = matches!(self.updater.status(), UpdateStatus::UpdateAvailable { .. });
                let draw_top_left_menu = |ui: &mut egui::Ui,
                                          show_about: &mut bool,
                                          updater: &UpdateChecker,
                                          s: &crate::i18n::Strings| {
                    let menu_btn = if has_update {
                        primary_icon_button(ui, UiIcon::Menu, "Update")
                    } else {
                        ghost_icon_button(ui, UiIcon::Menu, "")
                            .on_hover_text("Info, Updates & Open-Source-Lizenzen")
                    };
                    egui::Popup::menu(&menu_btn).show(|ui| {
                        ui.set_min_width(230.0);
                        if icon_button(ui, UiIcon::Info, "Über TermiGrid & Lizenzen").clicked() {
                            *show_about = true;
                            ui.close();
                        }
                        ui.separator();
                        match updater.status() {
                            UpdateStatus::UpdateAvailable {
                                latest_tag,
                                release_url,
                            } => {
                                if primary_icon_button(
                                    ui,
                                    UiIcon::DownloadUpdate,
                                    &format!("Update: {latest_tag}"),
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
                                if icon_button(ui, UiIcon::RefreshCheck, s.btn_check_update).clicked()
                                {
                                    updater.check_async(ui.ctx().clone());
                                    ui.close();
                                }
                            }
                        }
                        if icon_button(ui, UiIcon::GitHub, s.btn_github).clicked() {
                            open_external_url(ui.ctx(), GITHUB_REPO_URL);
                            ui.close();
                        }
                        if icon_button(ui, UiIcon::Coffee, s.btn_coffee).clicked() {
                            open_external_url(ui.ctx(), BUYMEACOFFEE_URL);
                            ui.close();
                        }
                    });
                };

                let tab_items = [
                    (ActiveTab::Editor, UiIcon::CadEditor, s.tab_editor, ""),
                    (ActiveTab::Database, UiIcon::Database, s.tab_database, ""),
                    (ActiveTab::Guide, UiIcon::Guide, s.tab_guide, ""),
                ];

                if wide_header {
                    ui.horizontal(|ui| {
                        draw_top_left_menu(ui, &mut self.show_about_modal, &self.updater, s);
                        ui.label(
                            egui::RichText::new("TermiGrid")
                                .size(16.5)
                                .strong()
                                .color(t.text_primary),
                        );
                        pill_badge(
                            ui,
                            &format!("v{CURRENT_VERSION}"),
                            t.text_secondary,
                            t.bg_recessed,
                        );
                        ui.add_space(4.0);
                        segmented_control(ui, &mut self.active_tab, &tab_items);

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            self.show_header_prefs(ui, 82.0, avail_w >= 960.0);
                            if !cfg!(target_os = "android") {
                                let cmd_btn = ui
                                    .add(
                                        egui::Button::new(
                                            egui::RichText::new("Befehle  Strg+K")
                                                .size(12.0)
                                                .color(t.text_secondary),
                                        )
                                        .fill(t.bg_recessed)
                                        .stroke(egui::Stroke::new(1.0, t.border_hairline))
                                        .corner_radius(egui::CornerRadius::same(6)),
                                    )
                                    .on_hover_text("Befehlspalette & Klemmen-Schnellsuche öffnen (Strg+K)");
                                if cmd_btn.clicked() {
                                    self.command_palette.open_now();
                                }
                            }
                        });
                    });
                } else {
                    ui.horizontal(|ui| {
                        draw_top_left_menu(ui, &mut self.show_about_modal, &self.updater, s);
                        ui.label(
                            egui::RichText::new("TermiGrid")
                                .size(16.0)
                                .strong()
                                .color(t.text_primary),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            self.show_header_prefs(ui, 72.0, false);
                        });
                    });
                    ui.add_space(4.0);
                    ui.horizontal_wrapped(|ui| {
                        segmented_control(ui, &mut self.active_tab, &tab_items);
                        if icon_only_button(
                            ui,
                            UiIcon::Search,
                            self.command_palette.open,
                            "Befehlspalette öffnen (Strg+K)",
                        )
                        .clicked()
                        {
                            self.command_palette.open_now();
                        }
                    });
                }

                let has_active_project = !self.show_welcome && !self.project.strips.is_empty();
                let in_cad_editor = self.active_tab == ActiveTab::Editor && has_active_project;

                ui.add_space(5.0);
                ui.horizontal_wrapped(|ui| {
                    // Group 1: Project file actions (calm, neutral surfaces)
                    if icon_button(ui, UiIcon::NewFile, s.menu_new_project).clicked() {
                        self.project = Project::default();
                        self.reload_catalog_with_all_custom();
                        self.undo_stack.clear();
                        self.redo_stack.clear();
                        self.last_snapshot = self.project.clone();
                        self.clean_snapshot = self.project.clone();
                        self.current_file = None;
                        self.active_strip = 0;
                        self.selected_blocks.clear();
                        self.selected_end_bracket = None;
                        self.show_welcome = true;
                        self.active_tab = ActiveTab::Editor;
                        self.new_strip_modal.open_for_next_strip(0);
                    }
                    if icon_button(ui, UiIcon::FolderOpen, s.menu_open_project).clicked() {
                        self.open_project();
                    }
                    if icon_button(ui, UiIcon::ImportPdf, "PDF-Stückliste")
                        .on_hover_text(
                            "EPLAN-Stückliste (.pdf) intelligent importieren (auch per Drag & Drop)",
                        )
                        .clicked()
                    {
                        self.import_pdf_dialog();
                    }
                    if has_active_project {
                        let dirty = self.has_unsaved_changes();
                        let save_lbl = if dirty {
                            format!("{} *", s.menu_save_project)
                        } else {
                            s.menu_save_project.to_string()
                        };
                        let clicked = if dirty {
                            subtle_success_icon_button(ui, UiIcon::Save, &save_lbl).clicked()
                        } else {
                            icon_button(ui, UiIcon::Save, &save_lbl).clicked()
                        };
                        if clicked {
                            let _ = self.save_project();
                        }
                    }

                    // Group 2: Undo / Redo (only in active CAD Editor with a project)
                    if in_cad_editor {
                        ui.separator();
                        let undo_clicked = ui
                            .add_enabled_ui(!self.undo_stack.is_empty(), |ui| {
                                icon_only_button(
                                    ui,
                                    UiIcon::Undo,
                                    false,
                                    "Rückgängig / Undo (Strg+Z)",
                                )
                            })
                            .inner
                            .clicked();
                        if undo_clicked {
                            self.perform_undo();
                        }
                        let redo_clicked = ui
                            .add_enabled_ui(!self.redo_stack.is_empty(), |ui| {
                                icon_only_button(
                                    ui,
                                    UiIcon::Redo,
                                    false,
                                    "Wiederholen / Redo (Strg+Y)",
                                )
                            })
                            .inner
                            .clicked();
                        if redo_clicked {
                            self.perform_redo();
                        }
                    }

                    if avail_w >= 1180.0 {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if let Some(strip) = self.project.strips.get(self.active_strip)
                                && has_active_project
                            {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{} · {} {} · {:.1} mm",
                                        strip.name,
                                        strip.blocks.len(),
                                        s.blocks_unit,
                                        strip.total_width_mm()
                                    ))
                                    .monospace()
                                    .size(12.5)
                                    .color(t.text_secondary),
                                );
                            } else {
                                ui.label(
                                    egui::RichText::new(s.app_subtitle)
                                        .size(12.5)
                                        .color(t.text_muted),
                                );
                            }
                        });
                    }
                });
            });

        // Bottom Status Bar (calm tabular metrics + clean shortcuts)
        egui::Panel::bottom("status_bar")
            .frame(
                egui::Frame::NONE
                    .fill(t.bg_panel)
                    .stroke(egui::Stroke::new(1.0, t.border_hairline))
                    .inner_margin(egui::Margin::symmetric(10, 4)),
            )
            .show(ui, |ui| {
                let avail_w = ui.available_width();
                if self.active_tab == ActiveTab::Editor && !self.show_welcome && avail_w >= 960.0 {
                    ui.horizontal(|ui| {
                        let left_max = (avail_w - 580.0).max(180.0);
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(left_max, 18.0),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(&self.status_message)
                                            .size(12.0)
                                            .color(t.text_secondary),
                                    )
                                    .truncate(),
                                );
                            },
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            kbd_chip(ui, "Entf");
                            ui.label(
                                egui::RichText::new("Löschen")
                                    .size(11.5)
                                    .color(t.text_muted),
                            );
                            ui.add_space(4.0);
                            kbd_chip(ui, "Strg+K");
                            ui.label(
                                egui::RichText::new("Befehle")
                                    .size(11.5)
                                    .color(t.text_muted),
                            );
                            ui.add_space(4.0);
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(s.cad_footer_hint)
                                        .size(11.5)
                                        .color(t.text_muted),
                                )
                                .truncate(),
                            );
                        });
                    });
                } else {
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(&self.status_message)
                                    .size(12.0)
                                    .color(t.text_secondary),
                            )
                            .truncate(),
                        );
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
                    let prev_len = self.project.custom_specs.len();
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
                    if self.project.custom_specs.len() > prev_len
                        && merge_custom_specs_into(
                            &self.project.custom_specs[prev_len..],
                            &mut self.global_custom_specs,
                        )
                    {
                        let _ = save_global_custom_specs(&self.global_custom_specs);
                    }
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
            let strip_name = new_strip.name.clone();
            self.project.strips.push(new_strip);
            self.active_strip = self.project.strips.len().saturating_sub(1);
            self.selected_blocks.clear();
            self.selected_end_bracket = None;
            self.show_welcome = false;
            self.active_tab = ActiveTab::Editor;
            self.notify(
                ToastTone::Success,
                format!(
                    "Klemmleiste „{strip_name}“ angelegt — Klemmen anpassen und dann Endplatten generieren."
                ),
            );
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

        // Intelligent EPLAN PDF BOM import modal
        if let Some(selected_imports) = show_pdf_import_modal(
            ui.ctx(),
            &mut self.pdf_import_modal,
            &self.catalog,
            s,
        ) {
            if self.show_welcome {
                self.project.strips.clear();
                self.undo_stack.clear();
                self.redo_stack.clear();
                self.current_file = None;
                if self.project.project_name.trim().is_empty()
                    || self.project.project_name == "Neues Projekt"
                {
                    let stem = self
                        .pdf_import_modal
                        .source_label
                        .strip_suffix(".pdf")
                        .or_else(|| self.pdf_import_modal.source_label.strip_suffix(".PDF"))
                        .unwrap_or(&self.pdf_import_modal.source_label)
                        .trim();
                    if !stem.is_empty() {
                        self.project.project_name = stem.to_string();
                    }
                }
            }

            let first_new_idx = self.project.strips.len();
            let mut nt_pending = Vec::new();
            let mut imported_count = 0usize;

            for item in selected_imports {
                if let Some((mut strip, nt_zone)) = instantiate_imported_strip(
                    &item.plan,
                    item.variant_index,
                    item.include_end_clamps,
                    &self.catalog,
                ) {
                    strip.end_plates_generated = !item.plan.has_nt_or_install;
                    let strip_idx = self.project.strips.len();
                    let bmk = strip.name.clone();
                    let block_count = strip.blocks.len();
                    self.project.strips.push(strip);
                    imported_count += 1;

                    if item.plan.has_nt_or_install
                        && let Some(base_spec_id) = item.plan.nt_anchor_spec_id
                    {
                        let (a0, a1) = nt_zone.unwrap_or((0, block_count));
                        nt_pending.push((strip_idx, bmk, a0, a1, base_spec_id, item.plan.nt_hints));
                    }
                }
            }

            self.nt_popup_queue.clear();
            let total_nt = nt_pending.len();
            for (i, (strip_idx, strip_bmk, a0, a1, base_spec_id, nt_hints)) in
                nt_pending.into_iter().enumerate()
            {
                self.nt_popup_queue.push_back(NtQueueEntry {
                    strip_idx,
                    strip_bmk,
                    anchor_start: a0,
                    anchor_end: a1,
                    base_spec_id,
                    nt_hints,
                    ordinal: i + 1,
                    total: total_nt,
                });
            }

            if imported_count > 0 {
                self.active_strip = first_new_idx.min(self.project.strips.len().saturating_sub(1));
                self.selected_blocks.clear();
                self.selected_end_bracket = None;
                self.show_welcome = false;
                self.active_tab = ActiveTab::Editor;
                self.mobile_pane = MobilePane::Cad;
                if total_nt > 0 {
                    self.notify(
                        ToastTone::Info,
                        format!(
                            "{imported_count} Klemmleisten aus PDF importiert ({total_nt}× NT-Konfiguration ausstehend)."
                        ),
                    );
                } else {
                    self.notify(
                        ToastTone::Success,
                        format!("{imported_count} Klemmleisten aus PDF importiert."),
                    );
                }
            }
        }

        // Sequential NT configuration queue (shows one imported NT strip at a time, never overlapping)
        if !self.install_prompt_modal.open
            && self.install_prompt_modal.action.is_none()
            && !self.pdf_import_modal.open
            && let Some(next_nt) = self.nt_popup_queue.pop_front()
            && let Some(spec) = self.catalog.find_by_id(&next_nt.base_spec_id)
        {
            let has_supply = self.strip_has_n_supply(next_nt.strip_idx);
            let has_carriers = self.strip_has_busbar_carriers(next_nt.strip_idx);
            let title_ctx = format!(
                "NT-Konfiguration für {} ({}/{})",
                next_nt.strip_bmk, next_nt.ordinal, next_nt.total
            );
            self.active_strip = next_nt
                .strip_idx
                .min(self.project.strips.len().saturating_sub(1));
            self.install_prompt_modal.open_for_imported_strip(
                PendingInstallAction::ConfigureStripNt {
                    strip_idx: next_nt.strip_idx,
                    anchor_start: next_nt.anchor_start,
                    anchor_end: next_nt.anchor_end,
                },
                spec,
                title_ctx,
                next_nt.nt_hints,
                has_supply,
                has_carriers,
            );
        }

        // Cross-platform Info & Open-Source Licenses modal dialog
        show_about_modal(ui.ctx(), &mut self.show_about_modal, &self.updater, s);

        // Unsaved changes confirmation dialog when closing the application
        self.show_close_confirm_dialog(ui.ctx());

        // Save active terminal strip into personal library
        let target_strip = self
            .project
            .strips
            .get(self.save_strip_library_modal.strip_idx);
        if let Some(confirm) = show_save_strip_to_library_modal(
            ui.ctx(),
            &mut self.save_strip_library_modal,
            target_strip,
            &self.strip_library,
            s,
        ) && let Some(strip) = self.project.strips.get(confirm.strip_idx)
        {
            self.strip_library.save_template(
                confirm.template_name.clone(),
                confirm.description,
                strip,
                &self.project.custom_specs,
                confirm.overwrite_id.as_deref(),
            );
            match self.strip_library.save_default() {
                Ok(_) => {
                    self.notify(
                        ToastTone::Success,
                        format!(
                            "Vorlage „{}“ in persönlicher Bibliothek gespeichert.",
                            confirm.template_name
                        ),
                    );
                }
                Err(e) => {
                    self.notify(ToastTone::Error, format!("{}: {e}", s.status_error));
                }
            }
        }

        // Personal Terminal Strip Library browser modal (search, rename, delete, insert)
        let has_active_project = !self.show_welcome && !self.project.strips.is_empty();
        let can_save_active =
            has_active_project && self.active_strip < self.project.strips.len();
        if let Some(lib_action) = show_strip_library_modal(
            ui.ctx(),
            &mut self.strip_library_modal,
            &mut self.strip_library,
            &self.catalog,
            has_active_project,
            can_save_active,
            s,
            &mut self.status_message,
        ) {
            match lib_action {
                StripLibraryAction::InsertTemplate {
                    template_id,
                    target_designation,
                } => {
                    self.insert_library_template(&template_id, target_designation.as_deref());
                }
                StripLibraryAction::SaveActiveStrip => {
                    if self.active_strip < self.project.strips.len() {
                        self.strip_library.refresh_from_default();
                        let strip = &self.project.strips[self.active_strip];
                        self.save_strip_library_modal
                            .open_for_strip(self.active_strip, strip);
                    }
                }
            }
        }

        // Sonner-style floating toast notifications (with inline Undo when deleting blocks)
        if let Some(ToastActionKind::Undo) =
            self.toast_queue.show(ui.ctx(), self.safe_insets.bottom)
        {
            self.perform_undo();
        }

        // Strg+K Command Palette (instant keyboard workflow)
        if let Some(cmd) = self.command_palette.show(
            ui.ctx(),
            &self.catalog,
            &self.project.strips,
            has_active_project,
            !self.undo_stack.is_empty(),
            !self.redo_stack.is_empty(),
        ) {
            self.execute_palette_command(ui.ctx(), cmd);
        }

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
    fn show_close_confirm_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_close_confirm_modal {
            return;
        }
        enum CloseChoice {
            SaveAndClose,
            DiscardAndClose,
            Cancel,
        }
        struct CloseModalData<'a> {
            project_name: &'a str,
            strip_count: usize,
            block_count: usize,
            choice: Option<CloseChoice>,
        }

        let block_count: usize = self.project.strips.iter().map(|st| st.blocks.len()).sum();
        let mut data = CloseModalData {
            project_name: &self.project.project_name,
            strip_count: self.project.strips.len(),
            block_count,
            choice: None,
        };
        let mut open = self.show_close_confirm_modal;

        show_responsive_modal(
            ctx,
            "close_confirm_modal",
            UiIcon::Save,
            "Änderungen vor dem Schließen speichern?",
            470.0,
            &mut open,
            &mut data,
            |ui, data| {
                let t = tokens(ui);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!(
                            "Das Projekt „{}“ enthält ungespeicherte Änderungen.",
                            data.project_name
                        ))
                        .strong()
                        .color(t.text_primary),
                    )
                    .wrap(),
                );
                ui.add_space(6.0);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!(
                            "{} Klemmleiste(n) · {} Klemme(n) — Möchten Sie diese Änderungen jetzt speichern oder verwerfen?",
                            data.strip_count, data.block_count
                        ))
                        .color(t.text_secondary),
                    )
                    .wrap(),
                );
            },
            |ui, data, close_requested| {
                if primary_icon_button(ui, UiIcon::Save, "Speichern & Schließen").clicked() {
                    data.choice = Some(CloseChoice::SaveAndClose);
                }
                if danger_icon_button(ui, UiIcon::Delete, "Verwerfen").clicked() {
                    data.choice = Some(CloseChoice::DiscardAndClose);
                }
                if icon_button(ui, UiIcon::Close, "Abbrechen").clicked() {
                    data.choice = Some(CloseChoice::Cancel);
                    *close_requested = true;
                }
            },
        );

        self.show_close_confirm_modal = open;
        if let Some(choice) = data.choice {
            match choice {
                CloseChoice::SaveAndClose => {
                    if self.save_project() {
                        self.show_close_confirm_modal = false;
                        self.allow_close = true;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
                CloseChoice::DiscardAndClose => {
                    self.show_close_confirm_modal = false;
                    self.allow_close = true;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                CloseChoice::Cancel => {
                    self.show_close_confirm_modal = false;
                }
            }
        }
    }

    fn execute_palette_command(&mut self, ctx: &egui::Context, cmd: PaletteCommand) {
        match cmd {
            PaletteCommand::GenerateActiveStrip => self.generate_active_strip(),
            PaletteCommand::NewStrip => {
                self.new_strip_modal
                    .open_for_next_strip(self.project.strips.len());
            }
            PaletteCommand::NewProject => {
                self.project = Project::default();
                self.reload_catalog_with_all_custom();
                self.undo_stack.clear();
                self.redo_stack.clear();
                self.last_snapshot = self.project.clone();
                self.clean_snapshot = self.project.clone();
                self.current_file = None;
                self.active_strip = 0;
                self.selected_blocks.clear();
                self.selected_end_bracket = None;
                self.show_welcome = true;
                self.active_tab = ActiveTab::Editor;
                self.new_strip_modal.open_for_next_strip(0);
            }
            PaletteCommand::OpenProject => self.open_project(),
            PaletteCommand::SaveProject => {
                let _ = self.save_project();
            }
            PaletteCommand::ImportPdf => self.import_pdf_dialog(),
            PaletteCommand::OpenLibrary => {
                self.strip_library.refresh_from_default();
                self.strip_library_modal.open_browser();
            }
            PaletteCommand::ExportXlsx => self.export_xlsx(),
            PaletteCommand::ExportCsv => self.export_csv(),
            PaletteCommand::ExportDxf => self.export_dxf(),
            PaletteCommand::ToggleBridgeMode => {
                if self.canvas_state.interaction_mode == CanvasInteractionMode::BridgeAssign {
                    self.canvas_state.interaction_mode = CanvasInteractionMode::default();
                    self.notify(ToastTone::Info, "Brücken-Modus beendet.");
                } else {
                    self.canvas_state.interaction_mode = CanvasInteractionMode::BridgeAssign;
                    self.canvas_state.e_planer_mode = false;
                    self.selected_end_bracket = None;
                    if let Some(strip) = self.project.strips.get_mut(self.active_strip) {
                        if strip.potentials.is_empty() {
                            let id = strip.add_potential("+24V", POTENTIAL_PALETTE[0]);
                            self.active_potential_id = Some(id);
                        } else if self.active_potential_id.is_none() {
                            self.active_potential_id = strip.potentials.first().map(|p| p.id);
                        }
                    }
                    self.notify(
                        ToastTone::Info,
                        "Brücken-Modus aktiv — Klemmen-Ebenen anklicken oder überstreichen.",
                    );
                }
            }
            PaletteCommand::ToggleEplanMode => {
                self.canvas_state.e_planer_mode = !self.canvas_state.e_planer_mode;
                if self.canvas_state.e_planer_mode
                    && self.canvas_state.interaction_mode == CanvasInteractionMode::BridgeAssign
                {
                    self.canvas_state.interaction_mode = CanvasInteractionMode::SelectBox;
                }
                if self.canvas_state.e_planer_mode {
                    self.notify(
                        ToastTone::Info,
                        "ePlan 1:1 Aufbaubild aktiv — maßstabsgetreuer DXF-Export bereit.",
                    );
                } else {
                    self.notify(ToastTone::Info, "Standard CAD-Ansicht aktiv.");
                }
            }
            PaletteCommand::ConvertNToNt => {
                if let Some(strip) = self.project.strips.get_mut(self.active_strip) {
                    let converted = strip.convert_n_to_nt_blocks(&self.catalog);
                    if converted > 0 {
                        self.notify(
                            ToastTone::Info,
                            format!("{converted} N-Klemme(n) in NT-Trennklemmen umgewandelt."),
                        );
                    }
                }
            }
            PaletteCommand::ConvertNtToN => {
                if let Some(strip) = self.project.strips.get_mut(self.active_strip) {
                    let converted = strip.convert_nt_to_n_blocks(&self.catalog);
                    if converted > 0 {
                        self.notify(
                            ToastTone::Info,
                            format!("{converted} NT-Trennklemme(n) in Standard-N-Klemmen zurückgewandelt."),
                        );
                    }
                }
            }
            PaletteCommand::Undo => self.perform_undo(),
            PaletteCommand::Redo => self.perform_redo(),
            PaletteCommand::CycleTheme => {
                self.theme_mode = self.theme_mode.next();
                apply_theme_mode(ctx, self.theme_mode);
            }
            PaletteCommand::SwitchStrip(idx) => {
                if idx < self.project.strips.len() {
                    self.active_strip = idx;
                    self.selected_blocks.clear();
                    self.selected_end_bracket = None;
                    self.show_welcome = false;
                    self.active_tab = ActiveTab::Editor;
                    self.mobile_pane = MobilePane::Cad;
                }
            }
            PaletteCommand::InsertTerminal(spec_id) => {
                if self.active_strip < self.project.strips.len() {
                    let target_pos = self
                        .selected_blocks
                        .iter()
                        .next_back()
                        .copied()
                        .map(|i| i + 1)
                        .unwrap_or_else(|| self.project.strips[self.active_strip].blocks.len());
                    self.insert_blocks_direct(self.active_strip, target_pos, 1, &spec_id, true);
                    self.show_welcome = false;
                    self.active_tab = ActiveTab::Editor;
                }
            }
        }
    }

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

    fn strip_has_busbar_carriers(&self, strip_idx: usize) -> bool {
        self.project
            .strips
            .get(strip_idx)
            .is_some_and(|strip| {
                strip.blocks.iter().any(|b| {
                    self.catalog
                        .find_by_id(&b.spec_id)
                        .is_some_and(|s| s.is_busbar_carrier())
                })
            })
    }

    fn effective_append_pos(&self, strip_idx: usize, requested_pos: usize) -> usize {
        let Some(strip) = self.project.strips.get(strip_idx) else {
            return requested_pos;
        };
        let mut insert_pos = requested_pos.min(strip.blocks.len());
        if insert_pos == strip.blocks.len() && strip.blocks.len() >= 2 {
            let last_is_carrier = self
                .catalog
                .find_by_id(&strip.blocks[insert_pos - 1].spec_id)
                .is_some_and(|s| s.is_busbar_carrier());
            let prev_is_install_zone = self
                .catalog
                .find_by_id(&strip.blocks[insert_pos - 2].spec_id)
                .is_some_and(|s| {
                    s.is_installation_or_nt_terminal() || s.is_n_supply_terminal()
                });
            if last_is_carrier && prev_is_install_zone {
                insert_pos -= 1;
            }
        }
        insert_pos
    }

    fn insert_blocks_direct(
        &mut self,
        strip_idx: usize,
        at_index: usize,
        count: usize,
        spec_id: &str,
        select_inserted: bool,
    ) {
        let Some(spec) = self.catalog.find_by_id(spec_id) else {
            return;
        };
        if strip_idx >= self.project.strips.len() || count == 0 {
            return;
        }
        let insert_pos = self.effective_append_pos(strip_idx, at_index);
        self.project.strips[strip_idx].insert_blocks(insert_pos, count, spec);
        if select_inserted {
            self.selected_blocks = (insert_pos..insert_pos + count).collect();
            self.canvas_state.selection_anchor = Some(insert_pos + count - 1);
            self.selected_end_bracket = None;
        }
        self.notify(ToastTone::Info, format!("{}× {} eingefügt.", count, spec.name));
    }

    fn insert_sequence_direct(
        &mut self,
        strip_idx: usize,
        at_index: usize,
        spec_ids: Vec<String>,
        select_inserted: bool,
        summary: &str,
    ) {
        if strip_idx >= self.project.strips.len() || spec_ids.is_empty() {
            return;
        }
        let insert_pos = self.effective_append_pos(strip_idx, at_index);
        let specs: Vec<_> = spec_ids
            .iter()
            .filter_map(|id| self.catalog.find_by_id(id))
            .collect();
        if specs.is_empty() {
            return;
        }
        let inserted = self.project.strips[strip_idx].insert_spec_sequence(insert_pos, &specs);
        if select_inserted && !inserted.is_empty() {
            self.canvas_state.selection_anchor = inserted.iter().next_back().copied();
            self.selected_blocks = inserted;
            self.selected_end_bracket = None;
        }
        self.notify(ToastTone::Success, format!("Block eingefügt: {summary}"));
    }

    /// Computes & reveals end/partition plates for the active strip and opens the NT configuration
    /// prompt if genuine NT disconnect terminals are placed without N-supply or busbar carriers.
    fn generate_active_strip(&mut self) {
        let strip_idx = self.active_strip;
        if strip_idx >= self.project.strips.len() {
            return;
        }
        if self.project.strips[strip_idx].blocks.is_empty() {
            self.notify(ToastTone::Warning, "Klemmleiste enthält noch keine Klemmen.");
            return;
        }

        self.project.strips[strip_idx].end_plates_generated = true;
        self.project.strips[strip_idx].recalculate_labels();

        let strip = &self.project.strips[strip_idx];
        let strip_name = strip.name.clone();
        let plates_count = calculate_end_plates(strip, &self.catalog).len();

        let nt_indices: Vec<usize> = strip
            .blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| {
                self.catalog
                    .find_by_id(&b.spec_id)
                    .is_some_and(|sp| sp.is_nt_disconnect_terminal())
            })
            .map(|(i, _)| i)
            .collect();

        if !nt_indices.is_empty() {
            let has_supply = self.strip_has_n_supply(strip_idx);
            let has_carriers = self.strip_has_busbar_carriers(strip_idx);
            if (!has_supply || !has_carriers)
                && let Some(first_nt) = nt_indices.first().copied()
                && let Some(last_nt) = nt_indices.last().copied()
                && let Some(nt_spec) = self
                    .catalog
                    .find_by_id(&self.project.strips[strip_idx].blocks[first_nt].spec_id)
            {
                self.install_prompt_modal.open_for_imported_strip(
                    PendingInstallAction::ConfigureStripNt {
                        strip_idx,
                        anchor_start: first_nt,
                        anchor_end: last_nt + 1,
                    },
                    nt_spec,
                    format!("NT-Konfiguration für {strip_name}"),
                    Vec::new(),
                    has_supply,
                    has_carriers,
                );
                self.notify(
                    ToastTone::Info,
                    format!(
                        "Endplatten ({plates_count}×) berechnet — NT-Konfiguration für „{strip_name}“ prüfen."
                    ),
                );
                return;
            }
            self.notify(
                ToastTone::Success,
                format!(
                    "Endplatten für „{strip_name}“ generiert ({plates_count}×). Weiter: Brücken, Excel oder ePlan."
                ),
            );
        } else {
            self.notify(
                ToastTone::Success,
                format!(
                    "Endplatten für „{strip_name}“ generiert ({plates_count}×). Weiter: Brücken, Excel oder ePlan."
                ),
            );
        }
    }

    fn apply_install_prompt_resolution(&mut self, res: InstallPromptResolution) {
        use crate::model::numbering::{
            apply_n_busbar_carriers_around, place_n_supply_in_zone, BusbarCarrierPlan,
        };

        let Some(effective_spec) = self.catalog.find_by_id(&res.effective_spec_id) else {
            return;
        };
        let supply_spec = if res.supply_placement != InstallSupplyPlacement::None {
            self.catalog.find_by_id(&res.supply_spec_id)
        } else {
            None
        };
        let outer_carrier = res
            .outer_carrier_spec_id
            .as_deref()
            .and_then(|id| self.catalog.find_by_id(id));
        let intermediate_support = res
            .intermediate_support_spec_id
            .as_deref()
            .and_then(|id| self.catalog.find_by_id(id));
        let carrier_plan = BusbarCarrierPlan {
            outer_carrier,
            intermediate_support,
            max_unsupported_span_tenths_mm: 2000,
        };

        match res.action {
            PendingInstallAction::ConfigureStripNt {
                strip_idx,
                anchor_start,
                anchor_end,
            } => {
                if let Some(strip) = self.project.strips.get_mut(strip_idx) {
                    let start = anchor_start.min(strip.blocks.len());
                    let end = anchor_end.min(strip.blocks.len());
                    let mut z0 = start;
                    let mut z1 = end;

                    if z1 > z0 && res.spec_changed {
                        let convert_indices: Vec<usize> = (z0..z1)
                            .filter(|&i| {
                                self.catalog
                                    .find_by_id(&strip.blocks[i].spec_id)
                                    .is_some_and(|sp| sp.is_nt_disconnect_terminal())
                            })
                            .collect();
                        if !convert_indices.is_empty() {
                            strip.reassign_blocks(&convert_indices, effective_spec);
                        }
                    }

                    if let Some(sup) = supply_spec {
                        match res.supply_placement {
                            InstallSupplyPlacement::BeforeGroup => {
                                (z0, z1) = place_n_supply_in_zone(
                                    strip,
                                    z0,
                                    z1,
                                    sup,
                                    true,
                                    &self.catalog,
                                );
                            }
                            InstallSupplyPlacement::AfterGroup => {
                                (z0, z1) = place_n_supply_in_zone(
                                    strip,
                                    z0,
                                    z1,
                                    sup,
                                    false,
                                    &self.catalog,
                                );
                            }
                            InstallSupplyPlacement::None => {}
                        }
                    }

                    if z1 > z0
                        && z1 <= strip.blocks.len()
                        && (outer_carrier.is_some() || intermediate_support.is_some())
                    {
                        apply_n_busbar_carriers_around(
                            strip,
                            z0,
                            z1,
                            &self.catalog,
                            &carrier_plan,
                        );
                    }
                    strip.end_plates_generated = true;
                    strip.recalculate_labels();
                    let plates_count = calculate_end_plates(strip, &self.catalog).len();
                    let strip_name = strip.name.clone();
                    self.active_strip = strip_idx;
                    self.selected_blocks.clear();
                    self.selected_end_bracket = None;
                    self.notify(
                        ToastTone::Success,
                        format!(
                            "Endplatten ({plates_count}×) & NT-Zubehör für „{strip_name}“ übernommen."
                        ),
                    );
                }
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
            .selected_text(self.lang.code())
            .show_ui(ui, |ui| {
                for lang_opt in Language::ALL {
                    ui.selectable_value(&mut self.lang, lang_opt, lang_opt.display_name());
                }
            });
        let (lang_icon_r, _) =
            ui.allocate_exact_size(egui::Vec2::splat(16.0), egui::Sense::hover());
        draw_icon(
            ui.painter(),
            lang_icon_r,
            UiIcon::Language,
            secondary_text_color(ui),
        );

        let (theme_icon, theme_label) = match self.theme_mode {
            ThemeMode::System => (UiIcon::ThemeSystem, s.theme_system),
            ThemeMode::Light => (UiIcon::ThemeLight, s.theme_light),
            ThemeMode::Dark => (UiIcon::ThemeDark, s.theme_dark),
        };
        if icon_button(ui, theme_icon, theme_label).clicked() {
            self.theme_mode = self.theme_mode.next();
            apply_theme_mode(ui.ctx(), self.theme_mode);
        }

        if show_sidebar_toggles && self.active_tab == ActiveTab::Editor && !self.show_welcome {
            ui.separator();
            // Right-to-left layout: paint right icon first, then left icon, so visual order is [Left] [Right]
            if icon_only_button(
                ui,
                UiIcon::SidebarRight(self.show_inspector),
                self.show_inspector,
                "Rechte Seitenleiste (Eigenschaften) ein-/ausblenden",
            )
            .clicked()
            {
                self.show_inspector = !self.show_inspector;
            }
            if icon_only_button(
                ui,
                UiIcon::SidebarLeft(self.show_left_sidebar),
                self.show_left_sidebar,
                "Linke Seitenleiste (Klemmleisten & Bibliothek) ein-/ausblenden",
            )
            .clicked()
            {
                self.show_left_sidebar = !self.show_left_sidebar;
            }
        }
    }

    /// Guided startup pipeline ("Projekt-Line"): New Project / Open Project / PDF Import -> CAD -> Generate -> Next Steps.
    fn show_welcome_pipeline(&mut self, ui: &mut egui::Ui) {
        let s = strings(self.lang);
        let t = tokens(ui);
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(t.bg_canvas))
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(36.0);
                    let card_w = (ui.available_width() - 32.0).clamp(280.0, 620.0);
                    ui.vertical_centered(|ui| {
                        egui::Frame::NONE
                            .fill(t.bg_panel)
                            .stroke(egui::Stroke::new(1.0, t.border_strong))
                            .corner_radius(egui::CornerRadius::same(12))
                            .inner_margin(egui::Margin::same(24))
                            .show(ui, |ui| {
                                ui.set_width(card_w);
                                ui.label(
                                    egui::RichText::new(s.welcome_heading)
                                        .size(23.0)
                                        .strong()
                                        .color(t.text_primary),
                                );
                                ui.add_space(4.0);
                                ui.label(
                                    egui::RichText::new(
                                        "Geradliniger Ablauf: 1. Startquelle wählen → 2. Klemmen im CAD anpassen → 3. Endplatten generieren → 4. Brücken, Excel-Export oder ePlan 1:1.",
                                    )
                                    .size(13.5)
                                    .color(t.text_secondary),
                                );
                                ui.add_space(14.0);

                                // Visual 4-step pipeline indicator
                                let step2_active = self.new_strip_modal.open;
                                ui.horizontal_wrapped(|ui| {
                                    let step_chip = |ui: &mut egui::Ui, label: &str, active: bool| {
                                        let (bg, border, fg) = if active {
                                            (t.accent_soft, t.accent, t.accent_text)
                                        } else {
                                            (t.bg_recessed, t.border_hairline, t.text_muted)
                                        };
                                        egui::Frame::NONE
                                            .fill(bg)
                                            .stroke(egui::Stroke::new(1.0, border))
                                            .corner_radius(egui::CornerRadius::same(99))
                                            .inner_margin(egui::Margin::symmetric(10, 4))
                                            .show(ui, |ui| {
                                                ui.label(
                                                    egui::RichText::new(label)
                                                        .size(12.0)
                                                        .color(fg)
                                                        .strong(),
                                                );
                                            });
                                    };
                                    step_chip(ui, s.welcome_step_1, !step2_active);
                                    ui.label(egui::RichText::new("→").color(t.text_muted));
                                    step_chip(ui, s.welcome_step_2, step2_active);
                                    ui.label(egui::RichText::new("→").color(t.text_muted));
                                    step_chip(ui, "3. Endplatten", false);
                                    ui.label(egui::RichText::new("→").color(t.text_muted));
                                    step_chip(ui, "4. Export / ePlan", false);
                                });

                                ui.add_space(14.0);
                                ui.separator();
                                ui.add_space(10.0);
                                ui.label(
                                    egui::RichText::new(s.project_name)
                                        .size(12.5)
                                        .color(t.text_secondary)
                                        .strong(),
                                );
                                ui.add_space(3.0);
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.project.project_name)
                                        .desired_width(card_w - 8.0),
                                );
                                ui.add_space(16.0);

                                // The 3 crisp linear start choices
                                ui.label(
                                    egui::RichText::new("Projektstart wählen")
                                        .size(12.5)
                                        .color(t.text_secondary)
                                        .strong(),
                                );
                                ui.add_space(6.0);
                                ui.horizontal_wrapped(|ui| {
                                    if primary_icon_button(ui, UiIcon::Add, "Neues Projekt")
                                        .clicked()
                                    {
                                        self.project.strips.clear();
                                        self.project.custom_specs.clear();
                                        self.reload_catalog_with_all_custom();
                                        self.current_file = None;
                                        self.active_strip = 0;
                                        self.selected_blocks.clear();
                                        self.selected_end_bracket = None;
                                        self.new_strip_modal.open_for_next_strip(0);
                                    }
                                    if icon_button(
                                        ui,
                                        UiIcon::FolderOpen,
                                        "Bestehendes Projekt laden",
                                    )
                                    .clicked()
                                    {
                                        self.open_project();
                                    }
                                    if icon_button(
                                        ui,
                                        UiIcon::ImportPdf,
                                        "PDF-Stückliste laden",
                                    )
                                    .on_hover_text(
                                        "EPLAN-Stückliste (.pdf) automatisch erkennen & übernehmen (auch per Drag & Drop)",
                                    )
                                    .clicked()
                                    {
                                        self.import_pdf_dialog();
                                    }
                                });

                                ui.add_space(14.0);
                                ui.separator();
                                ui.add_space(8.0);
                                ui.horizontal_wrapped(|ui| {
                                    let lib_btn_label = if self.strip_library.templates.is_empty() {
                                        "Aus Bibliothek".to_string()
                                    } else {
                                        format!(
                                            "Aus Bibliothek ({})",
                                            self.strip_library.templates.len()
                                        )
                                    };
                                    if ghost_icon_button(ui, UiIcon::Library, &lib_btn_label)
                                        .on_hover_text(
                                            "Persönliche Klemmleisten-Bibliothek durchsuchen & direkt starten",
                                        )
                                        .clicked()
                                    {
                                        self.strip_library.refresh_from_default();
                                        self.strip_library_modal.open_browser();
                                    }
                                    if ghost_icon_button(
                                        ui,
                                        UiIcon::CadEditor,
                                        s.welcome_demo_project,
                                    )
                                    .clicked()
                                    {
                                        self.project = create_demo_project(&self.catalog);
                                        self.last_snapshot = self.project.clone();
                                        self.clean_snapshot = self.project.clone();
                                        self.active_strip = 0;
                                        self.selected_blocks.clear();
                                        self.selected_end_bracket = None;
                                        self.show_welcome = false;
                                        self.notify(
                                            ToastTone::Info,
                                            "Beispielprojekt (-X1) geladen.",
                                        );
                                    }
                                });
                            });
                    });
                });
            });
    }

    fn show_editor_panels(&mut self, ui: &mut egui::Ui) {
        let s = strings(self.lang);
        let t = tokens(ui);
        let avail = ui.available_size();
        // Use tabbed mobile panes on phones (both Portrait and Landscape) and folded screens;
        // use 3-column workspace on Desktop, large Tablets, and unfolded Foldables.
        let is_compact = avail.x < 960.0 || avail.y < 460.0;

        if is_compact {
            // Smartphone / Folded Cover layout: touch-friendly sub-pane bar
            egui::Panel::bottom("mobile_nav_bar")
                .frame(
                    egui::Frame::NONE
                        .fill(t.bg_panel)
                        .stroke(egui::Stroke::new(1.0, t.border_hairline))
                        .inner_margin(egui::Margin::symmetric(10, 6)),
                )
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        if selectable_icon_button(
                            ui,
                            self.mobile_pane == MobilePane::Strips,
                            UiIcon::SidebarLeft(true),
                            s.strips_heading,
                        )
                        .clicked()
                        {
                            self.mobile_pane = MobilePane::Strips;
                        }
                        if selectable_icon_button(
                            ui,
                            self.mobile_pane == MobilePane::Cad,
                            UiIcon::CadEditor,
                            s.tab_editor,
                        )
                        .clicked()
                        {
                            self.mobile_pane = MobilePane::Cad;
                        }
                        let insp_label = if self.selected_end_bracket.is_some() {
                            format!("{} (EH)", s.inspector_title)
                        } else if self.selected_blocks.is_empty() {
                            s.inspector_title.to_string()
                        } else {
                            format!("{} ({})", s.inspector_title, self.selected_blocks.len())
                        };
                        if selectable_icon_button(
                            ui,
                            self.mobile_pane == MobilePane::Inspector,
                            UiIcon::Edit,
                            &insp_label,
                        )
                        .clicked()
                        {
                            self.mobile_pane = MobilePane::Inspector;
                        }
                    });
                });

            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(t.bg_canvas).inner_margin(egui::Margin::same(8)))
                .show(ui, |ui| match self.mobile_pane {
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
                .default_size(278.0)
                .min_size(216.0)
                .max_size(440.0)
                .frame(
                    egui::Frame::NONE
                        .fill(t.bg_panel)
                        .stroke(egui::Stroke::new(1.0, t.border_hairline))
                        .inner_margin(egui::Margin::same(10)),
                )
                .show(ui, |ui| {
                    self.show_strips_sidebar_contents(ui, false);
                });
        }

        if self.show_inspector {
            egui::Panel::right("right_inspector")
                .resizable(true)
                .default_size(356.0)
                .min_size(250.0)
                .max_size(640.0)
                .frame(
                    egui::Frame::NONE
                        .fill(t.bg_panel)
                        .stroke(egui::Stroke::new(1.0, t.border_hairline))
                        .inner_margin(egui::Margin::same(10)),
                )
                .show(ui, |ui| {
                    self.show_inspector_contents(ui, false);
                });
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(t.bg_canvas).inner_margin(egui::Margin::same(8)))
            .show(ui, |ui| {
                self.show_cad_workspace_contents(ui, false);
            });
    }

    fn show_strips_sidebar_contents(&mut self, ui: &mut egui::Ui, _is_compact: bool) {
        let s = strings(self.lang);
        let t = tokens(ui);
        let avail_w = ui.available_width();
        ui.label(
            egui::RichText::new(s.project_name)
                .size(12.0)
                .color(t.text_secondary)
                .strong(),
        );
        ui.add_space(2.0);
        ui.add(
            egui::TextEdit::singleline(&mut self.project.project_name)
                .desired_width((avail_w - 4.0).clamp(100.0, 380.0)),
        );
        ui.add_space(8.0);

        // Two overlapping stacked paper sheets ("Klemmleisten" & "Bibliothek") + bottom pagination dots
        let dots_bar_h = 26.0;
        let deck_h = (ui.available_height() - dots_bar_h - 4.0).max(210.0);
        let (deck_rect, _) = ui.allocate_exact_size(
            egui::Vec2::new(avail_w.max(180.0), deck_h),
            egui::Sense::hover(),
        );

        let peek_x = 22.0_f32;
        let peek_y = 6.0_f32;
        let back_rect = egui::Rect::from_min_max(
            egui::Pos2::new(deck_rect.min.x + peek_x, deck_rect.min.y),
            egui::Pos2::new(deck_rect.max.x, deck_rect.max.y - peek_y),
        );
        let front_rect = egui::Rect::from_min_max(
            egui::Pos2::new(deck_rect.min.x, deck_rect.min.y + peek_y),
            egui::Pos2::new(deck_rect.max.x - peek_x, deck_rect.max.y),
        );

        // Interactive exposed regions of the background sheet (right spine + top edge)
        let back_right_tab = egui::Rect::from_min_max(
            egui::Pos2::new(front_rect.max.x, back_rect.min.y),
            back_rect.max,
        );
        let back_top_tab = egui::Rect::from_min_max(
            back_rect.min,
            egui::Pos2::new(back_rect.max.x, front_rect.min.y),
        );
        let back_sheet_is_library = self.sidebar_deck_tab == SidebarDeckTab::Strips;
        let back_label = if back_sheet_is_library {
            if self.strip_library.templates.is_empty() {
                "Bibliothek".to_string()
            } else {
                format!("Bibliothek ({})", self.strip_library.templates.len())
            }
        } else if self.project.strips.is_empty() {
            s.strips_heading.to_string()
        } else {
            format!("{} ({})", s.strips_heading, self.project.strips.len())
        };
        let back_tip = if back_sheet_is_library {
            "Zur Bibliothek wechseln (Vorlagen mit Mini-ePlan-Vorschau)"
        } else {
            "Zu den Projekt-Klemmleisten wechseln"
        };

        let r_resp = ui
            .interact(
                back_right_tab,
                egui::Id::unique("sidebar_back_sheet_right"),
                egui::Sense::click(),
            )
            .on_hover_text(back_tip);
        let t_resp = ui
            .interact(
                back_top_tab,
                egui::Id::unique("sidebar_back_sheet_top"),
                egui::Sense::click(),
            )
            .on_hover_text(back_tip);
        let back_hovered = r_resp.hovered() || t_resp.hovered();
        if back_hovered {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        // Paint back sheet first
        let back_fill = if back_hovered {
            t.bg_surface
        } else {
            t.bg_recessed
        };
        let back_stroke = if back_hovered {
            egui::Stroke::new(1.2, t.accent)
        } else {
            egui::Stroke::new(1.0, t.border_strong)
        };
        ui.painter()
            .rect_filled(back_rect, egui::CornerRadius::same(10), back_fill);
        ui.painter().rect_stroke(
            back_rect,
            egui::CornerRadius::same(10),
            back_stroke,
            egui::StrokeKind::Inside,
        );

        // Vertical rotated tab label + subtle indicator dot along the exposed right edge of the back sheet
        let peek_center_x = (front_rect.max.x + back_rect.max.x) * 0.5;
        let tab_dot_pos = egui::Pos2::new(peek_center_x, back_rect.min.y + 18.0);
        ui.painter().circle_filled(
            tab_dot_pos,
            3.0,
            if back_hovered { t.accent } else { t.text_muted },
        );
        let rot_color = if back_hovered {
            t.accent_text
        } else {
            t.text_secondary
        };
        let rot_galley = ui.painter().layout_no_wrap(
            back_label,
            egui::FontId::proportional(11.5),
            rot_color,
        );
        let rot_w = rot_galley.size().x;
        let rot_h = rot_galley.size().y;
        let rot_cy = (back_rect.min.y + 34.0 + back_rect.max.y) * 0.5;
        let rot_pos = egui::Pos2::new(peek_center_x - rot_h * 0.5, rot_cy + rot_w * 0.5);
        ui.painter()
            .with_clip_rect(back_right_tab)
            .add(egui::epaint::TextShape::new(rot_pos, rot_galley, rot_color).with_angle(-std::f32::consts::FRAC_PI_2));

        // Subtle elevation shadow between front and back paper sheet
        let front_shadow_rect = front_rect.translate(egui::Vec2::new(2.0, 2.0));
        ui.painter().rect_filled(
            front_shadow_rect,
            egui::CornerRadius::same(10),
            egui::Color32::from_black_alpha(if t.dark { 75 } else { 22 }),
        );

        // Paint foreground sheet cleanly over the back sheet
        ui.painter()
            .rect_filled(front_rect, egui::CornerRadius::same(10), t.bg_surface);
        ui.painter().rect_stroke(
            front_rect,
            egui::CornerRadius::same(10),
            egui::Stroke::new(1.0, t.border_strong),
            egui::StrokeKind::Inside,
        );

        // Foreground sheet contents
        let content_rect = front_rect.shrink(9.0);
        let mut sheet_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(content_rect)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        sheet_ui.set_clip_rect(content_rect);
        match self.sidebar_deck_tab {
            SidebarDeckTab::Strips => self.show_strips_sheet_contents(&mut sheet_ui),
            SidebarDeckTab::Library => self.show_library_sheet_contents(&mut sheet_ui),
        }

        if r_resp.clicked() || t_resp.clicked() {
            if self.sidebar_deck_tab == SidebarDeckTab::Strips {
                self.strip_library.refresh_from_default();
                self.sidebar_deck_tab = SidebarDeckTab::Library;
            } else {
                self.sidebar_deck_tab = SidebarDeckTab::Strips;
            }
        }

        // Bottom two-dot page indicator (click dots to switch sheet)
        ui.add_space(4.0);
        let (dots_row_rect, _) = ui.allocate_exact_size(
            egui::Vec2::new(avail_w.max(120.0), dots_bar_h - 4.0),
            egui::Sense::hover(),
        );
        let cx = dots_row_rect.center().x;
        let cy = dots_row_rect.center().y;
        let dot0_hit = egui::Rect::from_center_size(
            egui::Pos2::new(cx - 16.0, cy),
            egui::Vec2::new(26.0, 20.0),
        );
        let dot1_hit = egui::Rect::from_center_size(
            egui::Pos2::new(cx + 16.0, cy),
            egui::Vec2::new(26.0, 20.0),
        );
        let dot0_resp = ui
            .interact(
                dot0_hit,
                egui::Id::unique("sidebar_dot_strips"),
                egui::Sense::click(),
            )
            .on_hover_text("Klemmleisten (Hauptansicht)");
        let dot1_resp = ui
            .interact(
                dot1_hit,
                egui::Id::unique("sidebar_dot_library"),
                egui::Sense::click(),
            )
            .on_hover_text("Bibliothek (Vorlagen mit Mini-ePlan)");

        for (idx, (hit, resp)) in [(dot0_hit, &dot0_resp), (dot1_hit, &dot1_resp)]
            .into_iter()
            .enumerate()
        {
            let active = (idx == 0 && self.sidebar_deck_tab == SidebarDeckTab::Strips)
                || (idx == 1 && self.sidebar_deck_tab == SidebarDeckTab::Library);
            if resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            if active {
                let pill = egui::Rect::from_center_size(hit.center(), egui::Vec2::new(16.0, 7.0));
                ui.painter()
                    .rect_filled(pill, egui::CornerRadius::same(99), t.accent);
            } else {
                let r = if resp.hovered() { 4.5 } else { 3.5 };
                let col = if resp.hovered() {
                    t.text_secondary
                } else {
                    t.border_strong
                };
                ui.painter().circle_filled(hit.center(), r, col);
            }
        }

        if dot0_resp.clicked() {
            self.sidebar_deck_tab = SidebarDeckTab::Strips;
        }
        if dot1_resp.clicked() {
            self.strip_library.refresh_from_default();
            self.sidebar_deck_tab = SidebarDeckTab::Library;
        }
    }

    fn show_strips_sheet_contents(&mut self, ui: &mut egui::Ui) {
        let s = strings(self.lang);
        let t = tokens(ui);

        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new(s.strips_heading).strong().size(14.5));
            if subtle_success_icon_button(ui, UiIcon::Add, s.btn_new_strip).clicked() {
                self.new_strip_modal
                    .open_for_next_strip(self.project.strips.len());
            }
        });
        ui.add_space(6.0);

        let mut delete_idx: Option<usize> = None;
        let sheet_w = ui.available_width().max(120.0);
        let list_max_h = (ui.available_height() - 40.0).max(80.0);
        egui::ScrollArea::vertical()
            .id_salt("strips_scroll")
            .max_height(list_max_h)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.set_max_width(sheet_w);
                ui.spacing_mut().item_spacing.y = 5.0;
                for (idx, strip) in self.project.strips.iter().enumerate() {
                    let is_active = idx == self.active_strip;
                    let card_w = ui.available_width();
                    let has_desc = is_active && !strip.description.is_empty();
                    let card_h = if has_desc { 58.0 } else { 44.0 };
                    let (rect, resp) =
                        ui.allocate_exact_size(egui::Vec2::new(card_w, card_h), egui::Sense::click());
                    if ui.is_rect_visible(rect) {
                        let hovered = resp.hovered();
                        let fill = if is_active {
                            t.accent_soft
                        } else if hovered {
                            t.bg_surface_hover
                        } else {
                            t.bg_panel
                        };
                        let stroke = if is_active {
                            egui::Stroke::new(1.0, t.accent)
                        } else {
                            egui::Stroke::new(1.0, t.border_hairline)
                        };
                        ui.painter()
                            .rect_filled(rect, egui::CornerRadius::same(7), fill);
                        ui.painter().rect_stroke(
                            rect,
                            egui::CornerRadius::same(7),
                            stroke,
                            egui::StrokeKind::Inside,
                        );
                        if is_active {
                            let bar = egui::Rect::from_min_max(
                                egui::Pos2::new(rect.min.x + 1.0, rect.min.y + 6.0),
                                egui::Pos2::new(rect.min.x + 4.0, rect.max.y - 6.0),
                            );
                            ui.painter()
                                .rect_filled(bar, egui::CornerRadius::same(2), t.accent);
                        }

                        let text_x = rect.min.x + 11.0;
                        ui.painter().text(
                            egui::Pos2::new(text_x, rect.min.y + 8.0),
                            egui::Align2::LEFT_TOP,
                            &strip.name,
                            egui::FontId::proportional(14.0),
                            t.text_primary,
                        );
                        ui.painter().text(
                            egui::Pos2::new(rect.max.x - 9.0, rect.min.y + 9.0),
                            egui::Align2::RIGHT_TOP,
                            format!("{:.1} mm", strip.total_width_mm()),
                            egui::FontId::monospace(11.5),
                            if is_active { t.accent_text } else { t.text_muted },
                        );
                        ui.painter().text(
                            egui::Pos2::new(text_x, rect.min.y + 25.5),
                            egui::Align2::LEFT_TOP,
                            format!(
                                "{} {} · {} {}",
                                strip.blocks.len(),
                                s.blocks_unit,
                                strip.printable_label_count(),
                                s.labels_unit
                            ),
                            egui::FontId::proportional(11.5),
                            t.text_secondary,
                        );
                        if has_desc {
                            let desc_galley = ui.painter().layout(
                                strip.description.clone(),
                                egui::FontId::proportional(11.0),
                                t.text_muted,
                                (card_w - 20.0).max(80.0),
                            );
                            ui.painter().galley(
                                egui::Pos2::new(text_x, rect.min.y + 41.0),
                                desc_galley,
                                t.text_muted,
                            );
                        }
                    }
                    if resp.clicked() {
                        self.active_strip = idx;
                        self.selected_blocks.clear();
                        self.selected_end_bracket = None;
                        self.mobile_pane = MobilePane::Cad;
                    }
                }
            });

        if !self.project.strips.is_empty() {
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if icon_button(ui, UiIcon::Bookmark, "In Bibliothek")
                    .on_hover_text(
                        "Aktive Klemmleiste unter eigenem Namen in der Bibliothek speichern",
                    )
                    .clicked()
                    && self.active_strip < self.project.strips.len()
                {
                    self.strip_library.refresh_from_default();
                    let strip = &self.project.strips[self.active_strip];
                    self.save_strip_library_modal
                        .open_for_strip(self.active_strip, strip);
                }
                if danger_icon_button(ui, UiIcon::Delete, s.btn_delete_strip).clicked() {
                    delete_idx = Some(self.active_strip);
                }
            });
        }

        if let Some(idx) = delete_idx
            && idx < self.project.strips.len()
        {
            let removed_name = self.project.strips[idx].name.clone();
            self.project.strips.remove(idx);
            if self.active_strip >= self.project.strips.len() {
                self.active_strip = self.project.strips.len().saturating_sub(1);
            }
            self.selected_blocks.clear();
            self.selected_end_bracket = None;
            self.notify_undoable(
                ToastTone::Info,
                format!("Klemmleiste „{removed_name}“ gelöscht."),
            );
        }
    }

    fn show_library_sheet_contents(&mut self, ui: &mut egui::Ui) {
        let s = strings(self.lang);
        let t = tokens(ui);

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Bibliothek").strong().size(14.5));
            pill_badge(
                ui,
                &format!("{}", self.strip_library.templates.len()),
                t.accent_text,
                t.accent_soft,
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if icon_only_button(
                    ui,
                    UiIcon::Settings,
                    false,
                    "Bibliothek verwalten (Umbenennen / Löschen)",
                )
                .clicked()
                {
                    self.strip_library.refresh_from_default();
                    self.strip_library_modal.open_browser();
                }
                if self.active_strip < self.project.strips.len()
                    && icon_only_button(
                        ui,
                        UiIcon::Bookmark,
                        false,
                        "Aktive Klemmleiste in Bibliothek speichern",
                    )
                    .clicked()
                {
                    self.strip_library.refresh_from_default();
                    let strip = &self.project.strips[self.active_strip];
                    self.save_strip_library_modal
                        .open_for_strip(self.active_strip, strip);
                }
            });
        });
        ui.add_space(5.0);

        // Compact search field inside Library sheet
        egui::Frame::NONE
            .fill(t.bg_recessed)
            .stroke(egui::Stroke::new(1.0, t.border_hairline))
            .corner_radius(egui::CornerRadius::same(6))
            .inner_margin(egui::Margin::symmetric(7, 4))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let (sr, _) =
                        ui.allocate_exact_size(egui::Vec2::splat(13.0), egui::Sense::hover());
                    draw_icon(ui.painter(), sr, UiIcon::Search, t.text_muted);
                    let clear_w = if self.sidebar_lib_search.is_empty() {
                        0.0
                    } else {
                        24.0
                    };
                    let edit_w = (ui.available_width() - clear_w).max(60.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.sidebar_lib_search)
                            .frame(egui::Frame::NONE)
                            .hint_text("Vorlage suchen…")
                            .desired_width(edit_w),
                    );
                    if !self.sidebar_lib_search.is_empty()
                        && icon_only_button(ui, UiIcon::Close, false, "Suche leeren").clicked()
                    {
                        self.sidebar_lib_search.clear();
                    }
                });
            });
        ui.add_space(6.0);

        let matching: Vec<_> = self
            .strip_library
            .filter(&self.sidebar_lib_search, &self.catalog)
            .into_iter()
            .cloned()
            .collect();

        let sheet_w = ui.available_width().max(120.0);
        let mut insert_tpl_id: Option<String> = None;

        egui::ScrollArea::vertical()
            .id_salt("sidebar_library_scroll")
            .max_height(ui.available_height().max(80.0))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_max_width(sheet_w);
                ui.spacing_mut().item_spacing.y = 7.0;

                if self.strip_library.templates.is_empty() {
                    recessed_card(ui, |ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("Noch keine Vorlagen gespeichert.")
                                    .strong()
                                    .color(t.text_primary)
                                    .size(12.5),
                            )
                            .wrap(),
                        );
                        ui.add_space(3.0);
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(
                                    "Aktive Leiste über ‚In Bibliothek‘ sichern, um sie hier mit ePlan-Vorschau abzulegen.",
                                )
                                .color(t.text_secondary)
                                .small(),
                            )
                            .wrap(),
                        );
                    });
                    return;
                }

                if matching.is_empty() {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(format!(
                                "Keine Treffer für „{}“.",
                                self.sidebar_lib_search.trim()
                            ))
                            .color(t.text_secondary)
                            .small(),
                        )
                        .wrap(),
                    );
                    return;
                }

                for tpl in &matching {
                    egui::Frame::NONE
                        .fill(t.bg_panel)
                        .stroke(egui::Stroke::new(1.0, t.border_hairline))
                        .corner_radius(egui::CornerRadius::same(8))
                        .inner_margin(egui::Margin::same(8))
                        .show(ui, |ui| {
                            ui.set_width((ui.available_width() - 2.0).max(110.0));
                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(&tpl.name)
                                            .strong()
                                            .size(13.0)
                                            .color(t.text_primary),
                                    )
                                    .truncate(),
                                );
                            });
                            ui.label(
                                egui::RichText::new(format!(
                                    "{} {} · {:.1} mm",
                                    tpl.strip.blocks.len(),
                                    s.blocks_unit,
                                    tpl.strip.total_width_mm(),
                                ))
                                .monospace()
                                .size(11.0)
                                .color(t.text_muted),
                            );
                            ui.add_space(4.0);

                            // Mini proportional ePlan 1:1 Aufbaubild preview
                            draw_mini_eplan_preview(ui, &tpl.strip, &self.catalog, 40.0);

                            ui.add_space(5.0);
                            if subtle_success_icon_button(ui, UiIcon::Add, "Einfügen")
                                .on_hover_text(
                                    "Diese Klemmleisten-Vorlage ins aktive Projekt einfügen",
                                )
                                .clicked()
                            {
                                insert_tpl_id = Some(tpl.id.clone());
                            }
                        });
                }
            });

        if let Some(tid) = insert_tpl_id {
            self.insert_library_template(&tid, None);
        }
    }

    fn show_inspector_contents(&mut self, ui: &mut egui::Ui, _is_compact: bool) {
        let s = strings(self.lang);
        let t = tokens(ui);
        let sec = secondary_text_color(ui);
        let panel_max_w = (ui.available_width() - 8.0).max(160.0);
        egui::ScrollArea::vertical()
            .id_salt("inspector_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_max_width(panel_max_w);
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(s.inspector_title)
                            .size(15.5)
                            .strong()
                            .color(t.text_primary),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if !self.selected_blocks.is_empty() {
                            pill_badge(
                                ui,
                                &format!("{} markiert", self.selected_blocks.len()),
                                t.accent_text,
                                t.accent_soft,
                            );
                        } else if let Some(is_left) = self.selected_end_bracket {
                            pill_badge(
                                ui,
                                if is_left { "EH Links" } else { "EH Rechts" },
                                t.accent_text,
                                t.accent_soft,
                            );
                        }
                    });
                });
                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

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
                    ui.add_space(6.0);

                    let bridge_plan = {
                        let strip = &mut self.project.strips[self.active_strip];
                        ui.horizontal_wrapped(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.new_potential_name)
                                    .hint_text(s.bridge_new_potential)
                                    .desired_width(110.0),
                            );
                            if subtle_success_icon_button(ui, UiIcon::Potential, s.bridge_add_btn)
                                .clicked()
                            {
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
                                if icon_only_button(
                                    ui,
                                    UiIcon::Close,
                                    false,
                                    "Potential entfernen",
                                )
                                .clicked()
                                {
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
                            && icon_button(ui, UiIcon::BridgeBar, s.bridge_assign_selected)
                                .clicked()
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
                        ui.add_space(6.0);
                        section_card(ui, |ui| {
                            ui.label(egui::RichText::new(s.bridge_bom_heading).strong());
                            ui.add_space(4.0);
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
                    ui.add_space(6.0);

                    let (eb_id, keep_selected) = {
                        let strip = &mut self.project.strips[self.active_strip];
                        section_card(ui, |ui| {
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
                    let sync_idx = self
                        .canvas_state
                        .selection_anchor
                        .filter(|i| self.selected_blocks.contains(i))
                        .or_else(|| self.selected_blocks.iter().next().copied());
                    if let Some(first_idx) = sync_idx
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

                    section_card(ui, |ui| {
                        let selected_ref_spec = self
                            .canvas_state
                            .selection_anchor
                            .filter(|i| self.selected_blocks.contains(i))
                            .or_else(|| self.selected_blocks.iter().next().copied())
                            .and_then(|idx| {
                                self.project.strips[self.active_strip].blocks.get(idx)
                            })
                            .and_then(|b| self.catalog.find_by_id(&b.spec_id));

                        ui.label(egui::RichText::new(s.reassign_heading).strong());
                        ui.add_space(4.0);
                        show_spec_picker(
                            ui,
                            &self.catalog,
                            &self.logos,
                            &mut self.inspector_mfg_filter,
                            &mut self.inspector_search,
                            &mut self.inspector_spec_id,
                            selected_ref_spec,
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
                            ui.add_space(6.0);
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

                        ui.add_space(6.0);

                        let mut deferred_insert: Option<(usize, String)> = None;
                        let mut deferred_seq_insert: Option<(usize, Vec<String>, String)> = None;
                        let mut applied_msg: Option<String> = None;

                        if primary_icon_button(ui, UiIcon::Apply, s.btn_apply_type).clicked()
                            && let Some(spec) = self.catalog.find_by_id(&self.inspector_spec_id)
                        {
                            let indices: Vec<usize> =
                                self.selected_blocks.iter().copied().collect();
                            let count = indices.len();
                            self.project.strips[self.active_strip]
                                .reassign_blocks(&indices, spec);
                            applied_msg =
                                Some(format!("{count}× Klemme auf {} geändert.", spec.name));
                        }

                        // 1-Polig & 3-Polig feeder-block buttons right below "Typ auf Auswahl anwenden"
                        let feeder_ref_spec = self
                            .catalog
                            .find_by_id(&self.inspector_spec_id)
                            .or(selected_ref_spec);
                        let plan_1p =
                            feeder_ref_spec.and_then(|sp| self.catalog.feeder_block_plan(sp, 1));
                        let plan_3p =
                            feeder_ref_spec.and_then(|sp| self.catalog.feeder_block_plan(sp, 3));
                        let target_insert_pos = self
                            .selected_blocks
                            .iter()
                            .next_back()
                            .copied()
                            .map(|i| i + 1)
                            .unwrap_or_else(|| self.project.strips[self.active_strip].blocks.len());

                        ui.horizontal_wrapped(|ui| {
                            let tip_1p = plan_1p
                                .as_ref()
                                .map(|p| format!("1-Polig Blockgruppe einfügen: {}", p.summary))
                                .unwrap_or_else(|| {
                                    "Kein 1-Polig Block (L + N + PE) für diesen Klemmentyp verfügbar"
                                        .to_string()
                                });
                            let btn_1p = ui
                                .add_enabled_ui(plan_1p.is_some(), |ui| {
                                    icon_button(ui, UiIcon::PoleBlock1, "1-Polig")
                                })
                                .inner
                                .on_hover_text(&tip_1p)
                                .on_disabled_hover_text(&tip_1p);
                            if btn_1p.clicked()
                                && let Some(p) = plan_1p
                            {
                                let ids = p.specs.iter().map(|sp| sp.id.clone()).collect();
                                deferred_seq_insert = Some((target_insert_pos, ids, p.summary));
                            }

                            let tip_3p = plan_3p
                                .as_ref()
                                .map(|p| format!("3-Polig Blockgruppe einfügen: {}", p.summary))
                                .unwrap_or_else(|| {
                                    "Kein 3-Polig Block (3× L + N + PE) für diesen Klemmentyp verfügbar"
                                        .to_string()
                                });
                            let btn_3p = ui
                                .add_enabled_ui(plan_3p.is_some(), |ui| {
                                    icon_button(ui, UiIcon::PoleBlock3, "3-Polig")
                                })
                                .inner
                                .on_hover_text(&tip_3p)
                                .on_disabled_hover_text(&tip_3p);
                            if btn_3p.clicked()
                                && let Some(p) = plan_3p
                            {
                                let ids = p.specs.iter().map(|sp| sp.id.clone()).collect();
                                deferred_seq_insert = Some((target_insert_pos, ids, p.summary));
                            }
                        });

                        ui.horizontal_wrapped(|ui| {
                            if icon_button(ui, UiIcon::InsertBefore, s.btn_insert_before).clicked()
                                && let Some(&first) = self.selected_blocks.iter().next()
                            {
                                deferred_insert = Some((first, self.inspector_spec_id.clone()));
                            }
                            if icon_button(ui, UiIcon::InsertAfter, s.btn_insert_after).clicked()
                                && let Some(&last) = self.selected_blocks.iter().next_back()
                            {
                                deferred_insert = Some((last + 1, self.inspector_spec_id.clone()));
                            }
                        });

                        if let Some(msg) = applied_msg {
                            self.notify(ToastTone::Info, msg);
                        } else if let Some((pos, spec_id)) = deferred_insert {
                            self.insert_blocks_direct(
                                self.active_strip,
                                pos,
                                1,
                                &spec_id,
                                true,
                            );
                        } else if let Some((pos, spec_ids, summary)) = deferred_seq_insert {
                            self.insert_sequence_direct(
                                self.active_strip,
                                pos,
                                spec_ids,
                                true,
                                &summary,
                            );
                        }

                        ui.horizontal_wrapped(|ui| {
                            if icon_only_button(
                                ui,
                                UiIcon::MoveLeft,
                                false,
                                "Nach links verschieben (Alt+Links)",
                            )
                            .clicked()
                            {
                                self.selected_blocks = self.project.strips[self.active_strip]
                                    .move_selected_blocks(&self.selected_blocks, -1);
                            }
                            if icon_only_button(
                                ui,
                                UiIcon::MoveRight,
                                false,
                                "Nach rechts verschieben (Alt+Rechts)",
                            )
                            .clicked()
                            {
                                self.selected_blocks = self.project.strips[self.active_strip]
                                    .move_selected_blocks(&self.selected_blocks, 1);
                            }
                            if icon_button(ui, UiIcon::Duplicate, "Duplizieren")
                                .on_hover_text("Auswahl klonen (Strg+D)")
                                .clicked()
                            {
                                self.selected_blocks = self.project.strips[self.active_strip]
                                    .duplicate_selected_blocks(&self.selected_blocks);
                            }
                            if danger_icon_button(ui, UiIcon::Delete, s.btn_delete_selected)
                                .clicked()
                            {
                                let indices: Vec<usize> =
                                    self.selected_blocks.iter().copied().collect();
                                let del_count = indices.len();
                                self.project.strips[self.active_strip].delete_blocks(&indices);
                                self.selected_blocks.clear();
                                self.canvas_state.selection_anchor = None;
                                self.notify_undoable(
                                    ToastTone::Info,
                                    format!("{del_count} Klemme(n) gelöscht."),
                                );
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
                            section_card(ui, |ui| {
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
                                    let tier_title_owned: String;
                                    let tier_title = match (total_tiers, t_idx) {
                                        (1, 0) => s.tier_lower,
                                        (2, 0) => s.tier_lower,
                                        (2, 1) => s.tier_upper,
                                        (3, 0) => s.tier_lower,
                                        (3, 1) => s.tier_middle,
                                        (3, 2) => s.tier_upper,
                                        _ if total_tiers > 3 => {
                                            tier_title_owned = format!("E{}", t_idx + 1);
                                            tier_title_owned.as_str()
                                        }
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
                                            && icon_button(
                                                ui,
                                                UiIcon::FlipOrder,
                                                s.btn_reset_override,
                                            )
                                            .clicked()
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
                            if primary_icon_button(ui, UiIcon::Check, s.affix_apply).clicked() {
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
                            if icon_button(ui, UiIcon::Clear, s.affix_clear).clicked() {
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
                if strip.end_plates_generated {
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
                        section_card(ui, |ui| {
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
                } else if !strip.blocks.is_empty() {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(
                                "Endplatten & NT-Prüfung erscheinen nach Klick auf „Endplatten generieren“.",
                            )
                            .color(sec)
                            .small(),
                        )
                        .wrap(),
                    );
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
                        section_card(ui, |ui| {
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
                    section_card(ui, |ui| {
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
                if strip.end_plates_generated {
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
                        section_card(ui, |ui| {
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
                } else if !strip.blocks.is_empty() {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(
                                "Endplatten & NT-Prüfung erscheinen nach Klick auf „Endplatten generieren“.",
                            )
                            .color(sec)
                            .small(),
                        )
                        .wrap(),
                    );
                    ui.add_space(4.0);
                }

                ui.add(
                    egui::Label::new(egui::RichText::new(s.acc_none_selected).color(sec)).wrap(),
                );
            }

            if !self.canvas_state.e_planer_mode
                && self.canvas_state.interaction_mode != CanvasInteractionMode::BridgeAssign
            {
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
            }
        });
    }

    fn show_cad_workspace_contents(&mut self, ui: &mut egui::Ui, is_compact: bool) {
        let s = strings(self.lang);
        let t = tokens(ui);
        if self.active_strip >= self.project.strips.len() {
            ui.centered_and_justified(|ui| {
                ui.label(s.cad_empty_hint);
            });
            return;
        }

        // Guided linear workflow state & deferred actions
        let mut recalc_needed = false;
        let mut deferred_single_insert: Option<(usize, String, bool)> = None;
        let mut deferred_convert: Option<bool> = None;
        let mut deferred_generate = false;
        let mut deferred_reedit = false;
        let mut deferred_toggle_bridge = false;
        let mut deferred_toggle_eplan = false;
        let mut deferred_export_xlsx = false;
        let mut deferred_export_csv = false;
        let mut deferred_export_dxf = false;

        let is_generated = self.project.strips[self.active_strip].end_plates_generated;
        let has_blocks = !self.project.strips[self.active_strip].blocks.is_empty();
        let plates_count = if is_generated {
            calculate_end_plates(&self.project.strips[self.active_strip], &self.catalog).len()
        } else {
            0
        };
        let is_bridge_mode = self.canvas_state.interaction_mode == CanvasInteractionMode::BridgeAssign;
        let is_eplan_mode = self.canvas_state.e_planer_mode;

        // 1. Top Guided Workflow Card (Linear pipeline: Klemmen anpassen -> Endplatten generieren -> Weiterverarbeitung)
        egui::Frame::NONE
            .fill(t.bg_panel)
            .stroke(egui::Stroke::new(1.0, t.border_hairline))
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(egui::Margin::symmetric(10, 8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                // Row A: Stepper + Stage CTA / Next Steps
                ui.horizontal_wrapped(|ui| {
                    let step_pill = |ui: &mut egui::Ui, text: &str, active: bool, done: bool| {
                        let (bg, border, fg) = if active {
                            (t.accent_soft, t.accent, t.accent_text)
                        } else if done {
                            (t.success_soft, t.success, t.success_hover)
                        } else {
                            (t.bg_recessed, t.border_hairline, t.text_muted)
                        };
                        egui::Frame::NONE
                            .fill(bg)
                            .stroke(egui::Stroke::new(1.0, border))
                            .corner_radius(egui::CornerRadius::same(99))
                            .inner_margin(egui::Margin::symmetric(9, 3))
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new(text).size(11.5).color(fg).strong());
                            });
                    };

                    if !is_generated {
                        step_pill(ui, "1. Klemmen anpassen", true, false);
                        ui.label(egui::RichText::new("→").color(t.text_muted));
                        step_pill(ui, "2. Endplatten generieren", false, false);
                        ui.label(egui::RichText::new("→").color(t.text_muted));
                        step_pill(ui, "3. Brücken / Excel / ePlan", false, false);

                        ui.separator();
                        let gen_tip = if has_blocks {
                            "Abschluss-/Zwischenplatten berechnen und auf NT-Trennklemmen prüfen"
                        } else {
                            "Zuerst Klemmen auf der Klemmleiste platzieren"
                        };
                        let gen_btn = ui
                            .add_enabled_ui(has_blocks, |ui| {
                                primary_icon_button(ui, UiIcon::Bolt, "Endplatten generieren")
                            })
                            .inner
                            .on_hover_text(gen_tip)
                            .on_disabled_hover_text(gen_tip);
                        if gen_btn.clicked() {
                            deferred_generate = true;
                        }
                    } else {
                        step_pill(ui, "✓ 1. Klemmen", false, true);
                        ui.label(egui::RichText::new("→").color(t.text_muted));
                        step_pill(
                            ui,
                            &format!("✓ 2. Endplatten ({plates_count}×)"),
                            false,
                            true,
                        );
                        ui.label(egui::RichText::new("→").color(t.text_muted));
                        step_pill(ui, "3. Nächster Schritt", true, false);

                        ui.separator();
                        if icon_button(ui, UiIcon::Edit, "Erneut bearbeiten")
                            .on_hover_text(
                                "Zurück in den Klemmen-Bearbeitungsmodus wechseln, um Klemmen oder Nummerierung anzupassen",
                            )
                            .clicked()
                        {
                            deferred_reedit = true;
                        }
                        if selectable_icon_button(
                            ui,
                            is_bridge_mode,
                            UiIcon::ModeBridge,
                            "Brücken-Modus",
                        )
                        .on_hover_text(s.mode_bridge_tip)
                        .clicked()
                        {
                            deferred_toggle_bridge = true;
                        }
                        if subtle_success_icon_button(
                            ui,
                            UiIcon::ExportXlsx,
                            "Excel-Export",
                        )
                        .on_hover_text(
                            "Exportiert alle Klemmleisten als 4-Blatt-Excel-Arbeitsmappe (.xlsx)",
                        )
                        .clicked()
                        {
                            deferred_export_xlsx = true;
                        }
                        if ghost_icon_button(ui, UiIcon::ExportCsv, "CSV")
                            .on_hover_text("Flache Druckdaten als .csv exportieren")
                            .clicked()
                        {
                            deferred_export_csv = true;
                        }
                        if selectable_icon_button(
                            ui,
                            is_eplan_mode,
                            UiIcon::ModeEplan,
                            "ePlan-Modus",
                        )
                        .on_hover_text(s.mode_eplaner_tip)
                        .clicked()
                        {
                            deferred_toggle_eplan = true;
                        }
                        if is_eplan_mode
                            && icon_button(ui, UiIcon::ExportDxf, "DXF (1:1 mm)")
                                .on_hover_text(
                                    "Exportiert alle Klemmleisten maßstabsgetreu (1:1 mm) als DXF für EPLAN Electric P8",
                                )
                                .clicked()
                        {
                            deferred_export_dxf = true;
                        }
                    }
                });

                ui.add_space(5.0);
                ui.separator();
                ui.add_space(4.0);

                // Row B: Calm Context Tools (Editing tools only in Step 1; View/Zoom controls in all steps)
                ui.horizontal_wrapped(|ui| {
                    let strip_len = self.project.strips[self.active_strip].blocks.len();
                    let has_selection =
                        !self.selected_blocks.is_empty() || self.selected_end_bracket.is_some();

                    if !is_generated {
                        // Editing & Numbering controls
                        {
                            let strip = &mut self.project.strips[self.active_strip];
                            ui.label(
                                egui::RichText::new(s.strip_start_number)
                                    .size(12.5)
                                    .color(t.text_secondary),
                            );
                            if ui
                                .add(egui::DragValue::new(&mut strip.start_number).range(1..=99999))
                                .changed()
                            {
                                recalc_needed = true;
                            }

                            ui.label(
                                egui::RichText::new(s.n_mode_label)
                                    .size(12.5)
                                    .color(t.text_secondary),
                            );
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
                                        (
                                            NeutralLabelMode::SharedSequenceNumber,
                                            s.n_mode_shared_num,
                                        ),
                                        (
                                            NeutralLabelMode::SharedSequenceSuffixN,
                                            s.n_mode_shared_n,
                                        ),
                                        (
                                            NeutralLabelMode::SeparateCounterSuffixN,
                                            s.n_mode_own_n,
                                        ),
                                    ] {
                                        if ui
                                            .selectable_value(&mut strip.neutral_mode, mode, lbl)
                                            .changed()
                                        {
                                            recalc_needed = true;
                                        }
                                    }
                                });

                            ui.label(
                                egui::RichText::new(s.pe_mode_label)
                                    .size(12.5)
                                    .color(t.text_secondary),
                            );
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
                                        (
                                            PeLabelMode::SharedSequenceNumber,
                                            s.pe_mode_shared_num,
                                        ),
                                        (
                                            PeLabelMode::SharedSequenceSuffixPe,
                                            s.pe_mode_shared_pe,
                                        ),
                                        (
                                            PeLabelMode::SeparateCounterSuffixPe,
                                            s.pe_mode_own_pe,
                                        ),
                                    ] {
                                        if ui
                                            .selectable_value(&mut strip.pe_mode, mode, lbl)
                                            .changed()
                                        {
                                            recalc_needed = true;
                                        }
                                    }
                                });
                        }

                        ui.separator();
                        let strip = &self.project.strips[self.active_strip];
                        let anchor_idx = self
                            .canvas_state
                            .selection_anchor
                            .filter(|i| self.selected_blocks.contains(i))
                            .or_else(|| self.selected_blocks.iter().next_back().copied());
                        let last_selected_idx = self.selected_blocks.iter().next_back().copied();
                        let target_insert_pos =
                            last_selected_idx.map(|i| i + 1).unwrap_or(strip.blocks.len());

                        let marked_spec = anchor_idx
                            .and_then(|idx| strip.blocks.get(idx))
                            .and_then(|b| self.catalog.find_by_id(&b.spec_id));
                        let append_target_spec = marked_spec
                            .or_else(|| self.catalog.find_by_id(&strip.default_spec_id));
                        let append_tip = if let (Some(idx), Some(sp)) = (anchor_idx, marked_spec) {
                            format!(
                                "Gleiche Klemme wie #{} ({}) direkt danach hinzufügen",
                                idx + 1,
                                sp.name
                            )
                        } else if let Some(sp) = append_target_spec {
                            format!("Standardklemme ({}) am Leistenende hinzufügen", sp.name)
                        } else {
                            s.btn_append_block.to_string()
                        };

                        if subtle_success_icon_button(ui, UiIcon::Add, s.btn_append_block)
                            .on_hover_text(append_tip)
                            .clicked()
                            && let Some(sp) = append_target_spec
                        {
                            deferred_single_insert = Some((
                                target_insert_pos,
                                sp.id.clone(),
                                marked_spec.is_some(),
                            ));
                        }

                        let to_nt_count = strip
                            .blocks
                            .iter()
                            .filter(|b| {
                                self.catalog
                                    .find_by_id(&b.spec_id)
                                    .and_then(|sp| sp.matching_nt_equivalent_id())
                                    .is_some()
                            })
                            .count();
                        let to_n_count = strip
                            .blocks
                            .iter()
                            .filter(|b| {
                                self.catalog
                                    .find_by_id(&b.spec_id)
                                    .and_then(|sp| sp.matching_n_from_nt_id())
                                    .is_some()
                            })
                            .count();
                        if to_nt_count > 0 || to_n_count > 0 {
                            let lbl_to_nt = if to_nt_count > 0 {
                                format!("N → NT ({to_nt_count})")
                            } else {
                                "N → NT".to_string()
                            };
                            let tip_to_nt = if to_nt_count > 0 {
                                format!(
                                    "Wandelt alle {to_nt_count} N-Klemmen dieser Leiste in passende NT-Trennklemmen um"
                                )
                            } else {
                                "Keine umwandelbaren N-Klemmen vorhanden".to_string()
                            };
                            if ui
                                .add_enabled_ui(to_nt_count > 0, |ui| {
                                    icon_button(ui, UiIcon::ConvertNt, &lbl_to_nt)
                                })
                                .inner
                                .on_hover_text(&tip_to_nt)
                                .clicked()
                            {
                                deferred_convert = Some(true);
                            }

                            let lbl_to_n = if to_n_count > 0 {
                                format!("NT → N ({to_n_count})")
                            } else {
                                "NT → N".to_string()
                            };
                            let tip_to_n = if to_n_count > 0 {
                                format!(
                                    "Wandelt alle {to_n_count} NT-Trennklemmen wieder in Standard-N-Klemmen um"
                                )
                            } else {
                                "Keine umwandelbaren NT-Trennklemmen vorhanden".to_string()
                            };
                            if ui
                                .add_enabled_ui(to_n_count > 0, |ui| {
                                    icon_button(ui, UiIcon::ConvertNt, &lbl_to_n)
                                })
                                .inner
                                .on_hover_text(&tip_to_n)
                                .clicked()
                            {
                                deferred_convert = Some(false);
                            }
                        }

                        ui.separator();
                    } else if cfg!(any(target_os = "android", target_os = "ios"))
                        && !is_bridge_mode
                    {
                        let pan_or_select = [
                            (
                                CanvasInteractionMode::Pan,
                                UiIcon::ModePan,
                                s.mode_pan,
                                s.mode_pan_tip,
                            ),
                            (
                                CanvasInteractionMode::SelectBox,
                                UiIcon::ModeSelect,
                                s.mode_select,
                                s.mode_select_tip,
                            ),
                        ];
                        segmented_control(
                            ui,
                            &mut self.canvas_state.interaction_mode,
                            &pan_or_select,
                        );
                        ui.separator();
                    }

                    // Clean Zoom & Type Display controls (always calm)
                    if icon_only_button(ui, UiIcon::ZoomOut, false, "Verkleinern").clicked() {
                        self.canvas_state.zoom = (self.canvas_state.zoom / 1.15).clamp(0.4, 2.5);
                    }
                    ui.scope(|ui| {
                        ui.spacing_mut().slider_width = if is_compact { 70.0 } else { 88.0 };
                        ui.visuals_mut().widgets.inactive.bg_fill = t.bg_recessed;
                        ui.add(
                            egui::Slider::new(&mut self.canvas_state.zoom, 0.4..=2.5)
                                .show_value(false),
                        );
                    });
                    if icon_only_button(ui, UiIcon::ZoomIn, false, "Vergrößern").clicked() {
                        self.canvas_state.zoom = (self.canvas_state.zoom * 1.15).clamp(0.4, 2.5);
                    }
                    ui.label(
                        egui::RichText::new(format!("{:>3.0}%", self.canvas_state.zoom * 100.0))
                            .monospace()
                            .size(12.5)
                            .color(t.text_secondary),
                    );
                    if icon_button(ui, UiIcon::ZoomReset, s.btn_fit_zoom).clicked() {
                        self.canvas_state.zoom = 1.0;
                    }

                    ui.label(
                        egui::RichText::new(s.type_display_label)
                            .size(12.5)
                            .color(t.text_secondary),
                    );
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

                    if !is_bridge_mode && strip_len > 0 {
                        ui.separator();
                        if icon_button(ui, UiIcon::SelectAll, s.btn_select_all).clicked() {
                            self.selected_blocks = (0..strip_len).collect();
                            self.selected_end_bracket = None;
                        }
                        if has_selection
                            && icon_button(ui, UiIcon::SelectNone, s.btn_clear_selection)
                                .clicked()
                        {
                            self.selected_blocks.clear();
                            self.selected_end_bracket = None;
                        }
                    }

                    if is_compact
                        && has_selection
                        && primary_icon_button(ui, UiIcon::Edit, "Bearbeiten").clicked()
                    {
                        self.mobile_pane = MobilePane::Inspector;
                    }
                });
            });

        if recalc_needed {
            self.project.strips[self.active_strip].recalculate_labels();
        }
        if deferred_generate {
            if self.canvas_state.interaction_mode == CanvasInteractionMode::BridgeAssign {
                self.canvas_state.interaction_mode = CanvasInteractionMode::default();
            }
            self.generate_active_strip();
        } else if deferred_reedit {
            self.project.strips[self.active_strip].end_plates_generated = false;
            self.canvas_state.e_planer_mode = false;
            if self.canvas_state.interaction_mode == CanvasInteractionMode::BridgeAssign {
                self.canvas_state.interaction_mode = CanvasInteractionMode::default();
            }
            self.notify(
                ToastTone::Info,
                "Bearbeitungsmodus aktiv — Klemmen anpassen und danach Endplatten generieren.",
            );
        } else if deferred_toggle_bridge {
            if self.canvas_state.interaction_mode == CanvasInteractionMode::BridgeAssign {
                self.canvas_state.interaction_mode = CanvasInteractionMode::default();
                self.notify(ToastTone::Info, "Brücken-Modus beendet.");
            } else {
                self.canvas_state.interaction_mode = CanvasInteractionMode::BridgeAssign;
                self.canvas_state.e_planer_mode = false;
                self.selected_end_bracket = None;
                let strip = &mut self.project.strips[self.active_strip];
                if strip.potentials.is_empty() {
                    let id = strip.add_potential("+24V", POTENTIAL_PALETTE[0]);
                    self.active_potential_id = Some(id);
                } else if self.active_potential_id.is_none() {
                    self.active_potential_id = strip.potentials.first().map(|p| p.id);
                }
                self.notify(
                    ToastTone::Info,
                    "Brücken-Modus aktiv — Klemmen-Ebenen im CAD anklicken oder überstreichen.",
                );
            }
        } else if deferred_toggle_eplan {
            self.canvas_state.e_planer_mode = !self.canvas_state.e_planer_mode;
            if self.canvas_state.e_planer_mode
                && self.canvas_state.interaction_mode == CanvasInteractionMode::BridgeAssign
            {
                self.canvas_state.interaction_mode = CanvasInteractionMode::default();
            }
            if self.canvas_state.e_planer_mode {
                self.notify(
                    ToastTone::Info,
                    "ePlan 1:1 Aufbaubild aktiv — maßstabsgetreuer DXF-Export bereit.",
                );
            } else {
                self.notify(ToastTone::Info, "Standard CAD-Ansicht aktiv.");
            }
        } else if deferred_export_xlsx {
            self.export_xlsx();
        } else if deferred_export_csv {
            self.export_csv();
        } else if deferred_export_dxf {
            self.export_dxf();
        } else if let Some((at_idx, spec_id, select_new)) = deferred_single_insert {
            self.insert_blocks_direct(
                self.active_strip,
                at_idx,
                1,
                &spec_id,
                select_new,
            );
        } else if let Some(to_nt) = deferred_convert {
            if to_nt {
                let converted =
                    self.project.strips[self.active_strip].convert_n_to_nt_blocks(&self.catalog);
                if converted > 0 {
                    self.notify(
                        ToastTone::Info,
                        format!(
                            "{converted} N-Klemme(n) in NT-Trennklemmen umgewandelt — anschließend auf „Endplatten generieren“ klicken."
                        ),
                    );
                }
            } else {
                let converted =
                    self.project.strips[self.active_strip].convert_nt_to_n_blocks(&self.catalog);
                if converted > 0 {
                    self.notify(
                        ToastTone::Info,
                        format!(
                            "{converted} NT-Trennklemme(n) in Standard-N-Klemmen zurückgewandelt — anschließend auf „Endplatten generieren“ klicken."
                        ),
                    );
                }
            }
        }

        // Inline Potential Quick-Bar directly inside CAD workspace when Bridge Mode is active
        let mut added_potential_name: Option<String> = None;
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

            ui.add_space(6.0);
            recessed_card(ui, |ui| {
                ui.set_width(ui.available_width());
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
                        let lbl =
                            egui::RichText::new(format!("{} ({})", pot.name, pot.members.len()))
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
                        if is_active
                            && strip.potentials.len() > 1
                            && icon_only_button(ui, UiIcon::Close, false, "Potential entfernen")
                                .clicked()
                        {
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
                    if subtle_success_icon_button(ui, UiIcon::Potential, s.bridge_add_btn)
                        .clicked()
                    {
                        let name = if self.new_potential_name.trim().is_empty() {
                            format!("P{}", strip.potentials.len() + 1)
                        } else {
                            self.new_potential_name.trim().to_string()
                        };
                        let color =
                            POTENTIAL_PALETTE[strip.potentials.len() % POTENTIAL_PALETTE.len()];
                        let id = strip.add_potential(&name, color);
                        self.active_potential_id = Some(id);
                        self.new_potential_name.clear();
                        added_potential_name = Some(name);
                    }

                    ui.label(
                        egui::RichText::new(s.bridge_assign_hint)
                            .color(secondary_text_color(ui))
                            .small(),
                    );
                });
            });
        }
        if let Some(pname) = added_potential_name {
            self.notify(
                ToastTone::Success,
                format!("Potential „{pname}“ hinzugefügt."),
            );
        }

        ui.add_space(6.0);

        let strip = &mut self.project.strips[self.active_strip];
        let bridge_plan = if strip.potentials.is_empty() {
            None
        } else {
            Some(calculate_bridges(strip, &self.catalog))
        };
        let end_plates = if strip.end_plates_generated {
            calculate_end_plates(strip, &self.catalog)
        } else {
            Vec::new()
        };
        let cad_res = egui::Frame::NONE
            .fill(t.bg_canvas)
            .stroke(egui::Stroke::new(1.0, t.border_hairline))
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(egui::Margin::same(1))
            .show(ui, |ui| {
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
                )
            })
            .inner;
        if let Some(after_idx) = cad_res.quick_add_after_block
            && let Some(spec_id) = self.project.strips[self.active_strip]
                .blocks
                .get(after_idx)
                .map(|b| b.spec_id.clone())
        {
            self.insert_blocks_direct(
                self.active_strip,
                after_idx + 1,
                1,
                &spec_id,
                true,
            );
        }
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
    x1.end_plates_generated = true;

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
