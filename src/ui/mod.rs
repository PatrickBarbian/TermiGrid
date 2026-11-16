pub mod android_insets;
pub mod cad_canvas;
pub mod guide_modal;
pub mod logos;
pub mod new_strip_modal;
pub mod platform_io;
pub mod theme;
pub mod updater;

use std::collections::BTreeSet;
use std::path::PathBuf;

use eframe::egui;

use crate::catalog::TerminalCatalog;
use crate::export::{csv::export_to_csv, xlsx::export_to_xlsx, ExportOptions, PrintOrder};
use crate::i18n::{strings, Language};
use crate::model::{NumberingOrder, Project, TerminalStrip};
use crate::ui::android_insets::{current_safe_insets, SafeAreaInsets};
use crate::ui::cad_canvas::{show_cad_canvas, CadCanvasState, TypeDisplayMode};
use crate::ui::guide_modal::{show_database_tab, show_guide_tab, CustomSpecDraft};
use crate::ui::logos::{show_logo_badge, BrandLogos};
use crate::ui::new_strip_modal::{show_new_strip_modal, show_spec_picker, NewStripDialogState};
use crate::ui::platform_io::{
    pick_export_csv_path, pick_export_xlsx_path, pick_open_project_path, pick_save_project_path,
};
use crate::ui::theme::{apply_termi_theme, install_system_fonts, secondary_text_color};
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
    canvas_state: CadCanvasState,
    new_strip_modal: NewStripDialogState,

    // Reassign / insert picker state
    inspector_mfg_filter: Option<String>,
    inspector_search: String,
    inspector_spec_id: String,

    // Database tab filter state
    db_mfg_filter: Option<String>,
    db_search: String,
    custom_spec_draft: CustomSpecDraft,

    // Export & File state
    export_options: ExportOptions,
    current_file: Option<PathBuf>,
    status_message: String,

    // UX preferences
    lang: Language,
    dark_mode: bool,
    active_tab: ActiveTab,
    mobile_pane: MobilePane,
    updater: UpdateChecker,

    // Cached Android system-bar / display-cutout safe area
    safe_insets: SafeAreaInsets,
    last_screen_size: egui::Vec2,
    insets_frames: u32,
}

impl TermiGridApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let catalog = TerminalCatalog::load_embedded();
        let logos = BrandLogos::load(&cc.egui_ctx);
        let project = create_demo_project(&catalog);

        // Load native Windows fonts (Segoe UI / Consolas) & apply high-contrast theme
        install_system_fonts(&cc.egui_ctx);
        apply_termi_theme(&cc.egui_ctx, true);

        let updater = UpdateChecker::default();
        #[cfg(not(target_os = "android"))]
        updater.check_async(cc.egui_ctx.clone());

        Self {
            catalog,
            logos,
            project,
            active_strip: 0,
            selected_blocks: BTreeSet::new(),
            canvas_state: CadCanvasState::new(),
            new_strip_modal: NewStripDialogState::default(),
            inspector_mfg_filter: None,
            inspector_search: String::new(),
            inspector_spec_id: "wago-2002-2204".to_string(),
            db_mfg_filter: None,
            db_search: String::new(),
            custom_spec_draft: CustomSpecDraft::default(),
            export_options: ExportOptions::default(),
            current_file: None,
            status_message: strings(Language::De).status_ready.to_string(),
            lang: Language::De,
            dark_mode: true,
            active_tab: ActiveTab::Editor,
            mobile_pane: MobilePane::Cad,
            updater,
            safe_insets: SafeAreaInsets::default(),
            last_screen_size: egui::Vec2::ZERO,
            insets_frames: 0,
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
                        self.active_strip = 0;
                        self.selected_blocks.clear();
                        self.current_file = Some(path.clone());
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
}

impl eframe::App for TermiGridApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let s = strings(self.lang);

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

            if wide_header {
                ui.horizontal(|ui| {
                    ui.heading(egui::RichText::new("TermiGrid").strong());
                    ui.separator();

                    ui.selectable_value(&mut self.active_tab, ActiveTab::Editor, s.tab_editor);
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Database, s.tab_database);
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Guide, s.tab_guide);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        self.show_header_prefs(ui, 120.0);
                    });
                });
            } else {
                ui.horizontal(|ui| {
                    ui.heading(egui::RichText::new("TermiGrid").strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        self.show_header_prefs(ui, 92.0);
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
                if ui.button(s.menu_new_project).clicked() {
                    self.project = Project::default();
                    self.catalog = TerminalCatalog::load_embedded();
                    self.current_file = None;
                    self.active_strip = 0;
                    self.selected_blocks.clear();
                    self.new_strip_modal.open_for_next_strip(0);
                }
                if ui.button(s.menu_open_project).clicked() {
                    self.open_project();
                }
                if ui.button(s.menu_save_project).clicked() {
                    self.save_project();
                }

                ui.separator();

                if ui.button(s.menu_export_xlsx).clicked() {
                    self.export_xlsx();
                }
                if ui.button(s.menu_export_csv).clicked() {
                    self.export_csv();
                }

                ui.separator();

                match self.updater.status() {
                    UpdateStatus::UpdateAvailable {
                        latest_tag,
                        release_url,
                    } => {
                        if ui
                            .button(egui::RichText::new(format!("Update: {latest_tag}")).strong())
                            .on_hover_text(s.update_open_release)
                            .clicked()
                        {
                            open_external_url(ui.ctx(), &release_url);
                        }
                    }
                    UpdateStatus::Checking => {
                        ui.label(s.update_checking);
                    }
                    _ => {
                        if ui.button(s.btn_check_update).clicked() {
                            self.updater.check_async(ui.ctx().clone());
                        }
                    }
                }
                if ui.button(s.btn_github).clicked() {
                    open_external_url(ui.ctx(), GITHUB_REPO_URL);
                }
                if ui.button(s.btn_coffee).clicked() {
                    open_external_url(ui.ctx(), BUYMEACOFFEE_URL);
                }

                if avail_w >= 1180.0 {
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
                self.show_editor_panels(ui);
            }
        }

        // Modal dialog for new strip creation
        if let Some(new_strip) = show_new_strip_modal(
            ui.ctx(),
            &mut self.new_strip_modal,
            &self.catalog,
            &self.logos,
            s,
            self.lang,
        ) {
            self.project.strips.push(new_strip);
            self.active_strip = self.project.strips.len().saturating_sub(1);
            self.selected_blocks.clear();
            self.active_tab = ActiveTab::Editor;
        }
    }
}

impl TermiGridApp {
    fn show_header_prefs(&mut self, ui: &mut egui::Ui, combo_width: f32) {
        let s = strings(self.lang);
        egui::ComboBox::from_id_salt("language_selector")
            .width(combo_width)
            .selected_text(format!("🌐 {}", self.lang.code()))
            .show_ui(ui, |ui| {
                for lang_opt in Language::ALL {
                    ui.selectable_value(&mut self.lang, lang_opt, lang_opt.display_name());
                }
            });

        let theme_btn = if self.dark_mode {
            format!("☀ {}", s.theme_light)
        } else {
            format!("🌙 {}", s.theme_dark)
        };
        if ui.button(theme_btn).clicked() {
            self.dark_mode = !self.dark_mode;
            apply_termi_theme(ui.ctx(), self.dark_mode);
        }
    }

    fn show_editor_panels(&mut self, ui: &mut egui::Ui) {
        let s = strings(self.lang);
        let avail = ui.available_size();
        // Use tabbed mobile panes on phones (both Portrait and Landscape) and folded screens;
        // use 3-column workspace on Desktop, large Tablets, and unfolded Foldables.
        let is_compact = avail.x < 960.0 || avail.y < 460.0;

        if is_compact {
            // Smartphone / Folded Cover layout: adaptive sub-pane bar
            egui::Panel::bottom("mobile_nav_bar").show(ui, |ui| {
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.selectable_value(&mut self.mobile_pane, MobilePane::Strips, s.strips_heading);
                    ui.selectable_value(&mut self.mobile_pane, MobilePane::Cad, s.tab_editor);
                    let insp_label = if self.selected_blocks.is_empty() {
                        s.inspector_title.to_string()
                    } else {
                        format!("{} ({})", s.inspector_title, self.selected_blocks.len())
                    };
                    ui.selectable_value(&mut self.mobile_pane, MobilePane::Inspector, insp_label);
                });
                ui.add_space(4.0);
            });

            egui::CentralPanel::default().show(ui, |ui| match self.mobile_pane {
                MobilePane::Strips => {
                    self.show_strips_sidebar_contents(ui);
                }
                MobilePane::Cad => {
                    self.show_cad_workspace_contents(ui);
                }
                MobilePane::Inspector => {
                    self.show_inspector_contents(ui);
                }
            });
            return;
        }

        // Desktop / Tablet / Unfolded Foldable: full 3-column workspace
        egui::Panel::left("left_sidebar")
            .default_size(255.0)
            .min_size(210.0)
            .show(ui, |ui| {
                self.show_strips_sidebar_contents(ui);
            });

        egui::Panel::right("right_inspector")
            .default_size(365.0)
            .min_size(320.0)
            .show(ui, |ui| {
                self.show_inspector_contents(ui);
            });

        egui::CentralPanel::default().show(ui, |ui| {
            self.show_cad_workspace_contents(ui);
        });
    }

    fn show_strips_sidebar_contents(&mut self, ui: &mut egui::Ui) {
        let s = strings(self.lang);
        ui.add_space(4.0);
        ui.label(egui::RichText::new(s.project_name).strong());
        ui.text_edit_singleline(&mut self.project.project_name);
        ui.separator();

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(s.strips_heading).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(s.btn_new_strip).clicked() {
                    self.new_strip_modal
                        .open_for_next_strip(self.project.strips.len());
                }
            });
        });
        ui.add_space(4.0);

        let mut delete_idx: Option<usize> = None;
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (idx, strip) in self.project.strips.iter().enumerate() {
                let is_active = idx == self.active_strip;
                let resp = ui.selectable_label(
                    is_active,
                    format!(
                        "{}  ({} {} / {} {})",
                        strip.name,
                        strip.blocks.len(),
                        s.blocks_unit,
                        strip.printable_label_count(),
                        s.labels_unit
                    ),
                );
                if resp.clicked() {
                    self.active_strip = idx;
                    self.selected_blocks.clear();
                    self.mobile_pane = MobilePane::Cad;
                }
                if is_active && !strip.description.is_empty() {
                    ui.label(
                        egui::RichText::new(&strip.description).color(secondary_text_color(ui)),
                    );
                }
            }
        });

        if !self.project.strips.is_empty() {
            ui.add_space(8.0);
            if ui.button(s.btn_delete_strip).clicked() {
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
        }
    }

    fn show_inspector_contents(&mut self, ui: &mut egui::Ui) {
        let s = strings(self.lang);
        egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(4.0);
                    ui.heading(s.inspector_title);
                    ui.separator();

                    let has_strip = self.active_strip < self.project.strips.len();
                    if has_strip {
                        // Selection & Mixed-strip reassignment
                        if self.selected_blocks.is_empty() {
                            ui.label(
                                egui::RichText::new(s.no_selection_hint)
                                    .color(secondary_text_color(ui)),
                            );
                        } else {
                            ui.label(
                                egui::RichText::new(format!(
                                    "{}: {}",
                                    s.selected_count,
                                    self.selected_blocks.len()
                                ))
                                .strong(),
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
                                ui.add_space(4.0);

                                if ui.button(s.btn_apply_type).clicked()
                                    && let Some(spec) =
                                        self.catalog.find_by_id(&self.inspector_spec_id)
                                {
                                    let indices: Vec<usize> =
                                        self.selected_blocks.iter().copied().collect();
                                    self.project.strips[self.active_strip]
                                        .reassign_blocks(&indices, spec);
                                }

                                ui.horizontal(|ui| {
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

                                if ui.button(s.btn_delete_selected).clicked() {
                                    let indices: Vec<usize> =
                                        self.selected_blocks.iter().copied().collect();
                                    self.project.strips[self.active_strip]
                                        .delete_blocks(&indices);
                                    self.selected_blocks.clear();
                                }
                            });

                            // Single block fine-tuning (Tier order & custom overrides)
                            if self.selected_blocks.len() == 1 {
                                let idx = *self.selected_blocks.iter().next().unwrap();
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
                                        ui.label(format!(
                                            "{} — {}",
                                            block.manufacturer, block.spec_name
                                        ));

                                        if block.tiers.len() > 1 {
                                            ui.horizontal(|ui| {
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
                                        for (t_idx, tier) in
                                            block.tiers.iter_mut().enumerate().rev()
                                        {
                                            let tier_title = match (total_tiers, t_idx) {
                                                (1, 0) => s.tier_lower,
                                                (2, 0) => s.tier_lower,
                                                (2, 1) => s.tier_upper,
                                                (3, 0) => s.tier_lower,
                                                (3, 1) => s.tier_middle,
                                                (3, 2) => s.tier_upper,
                                                _ => s.tier_middle,
                                            };

                                            ui.horizontal(|ui| {
                                                ui.label(format!(
                                                    "{}: [{}]",
                                                    tier_title, tier.cad_display_label
                                                ));
                                            });

                                            let mut text = tier
                                                .custom_override
                                                .clone()
                                                .unwrap_or_default();
                                            ui.horizontal(|ui| {
                                                let edit = egui::TextEdit::singleline(&mut text)
                                                    .hint_text(s.custom_override_hint)
                                                    .desired_width(150.0);
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
                        }
                    }

                    ui.add_space(10.0);
                    ui.separator();
                    ui.label(egui::RichText::new(s.accessories_heading).strong());
                    ui.add_space(4.0);

                    if !has_strip || self.selected_blocks.is_empty() {
                        ui.label(
                            egui::RichText::new(s.acc_none_selected)
                                .color(secondary_text_color(ui)),
                        );
                    } else {
                        let strip = &self.project.strips[self.active_strip];
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
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "{count}× {}",
                                                spec.name
                                            ))
                                            .strong(),
                                        );
                                    });
                                    ui.add_space(4.0);
                                    let sec = secondary_text_color(ui);
                                    egui::Grid::new(format!("acc_grid_{spec_id}"))
                                        .num_columns(2)
                                        .spacing([10.0, 5.0])
                                        .show(ui, |ui| {
                                            ui.label(egui::RichText::new(s.acc_end_plate).color(sec));
                                            ui.label(egui::RichText::new(&acc.end_plate).strong());
                                            ui.end_row();

                                            ui.label(egui::RichText::new(s.acc_jumpers).color(sec));
                                            ui.label(&acc.jumpers);
                                            ui.end_row();

                                            ui.label(egui::RichText::new(s.acc_markers).color(sec));
                                            ui.label(&acc.markers);
                                            ui.end_row();

                                            ui.label(egui::RichText::new(s.acc_end_bracket).color(sec));
                                            ui.label(&acc.end_bracket);
                                            ui.end_row();

                                            if let Some(extra) = &acc.extra {
                                                ui.label(egui::RichText::new(s.acc_extra).color(sec));
                                                ui.label(extra);
                                                ui.end_row();
                                            }
                                        });
                                });
                            }
                        }
                    }

                    ui.add_space(10.0);
                    ui.separator();
                    ui.label(egui::RichText::new(s.export_settings_heading).strong());
                    ui.add_space(4.0);

                    ui.label(s.print_order_label);
                    let order_text = match self.export_options.print_order {
                        PrintOrder::ByBlockBottomToTop => s.print_order_block_btt,
                        PrintOrder::ByBlockTopToBottom => s.print_order_block_ttb,
                        PrintOrder::ByTierStrip => s.print_order_tier_strip,
                    };
                    egui::ComboBox::from_id_salt("print_order_combo")
                        .width(240.0)
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

    fn show_cad_workspace_contents(&mut self, ui: &mut egui::Ui) {
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
            let strip = &mut self.project.strips[self.active_strip];
            ui.label(s.strip_start_number);
            if ui
                .add(egui::DragValue::new(&mut strip.start_number).range(1..=99999))
                .changed()
            {
                recalc_needed = true;
            }

            ui.separator();
            ui.label(s.zoom_label);
            ui.add_sized(
                [80.0, 20.0],
                egui::Slider::new(&mut self.canvas_state.zoom, 0.5..=2.2).show_value(false),
            );
            if ui.button(s.btn_fit_zoom).clicked() {
                self.canvas_state.zoom = 1.0;
            }

            ui.separator();
            if ui.button(s.btn_select_all).clicked() {
                self.selected_blocks = (0..strip.blocks.len()).collect();
            }
            if ui.button(s.btn_clear_selection).clicked() {
                self.selected_blocks.clear();
            }

            ui.separator();
            if ui.button(s.btn_append_block).clicked() {
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
        });

        if recalc_needed {
            self.project.strips[self.active_strip].recalculate_labels();
        }
        if let Some(spec_id) = append_spec_id
            && let Some(spec) = self.catalog.find_by_id(&spec_id)
        {
            self.project.strips[self.active_strip].append_blocks(1, spec);
        }

        ui.separator();

        let strip = &mut self.project.strips[self.active_strip];
        show_cad_canvas(
            ui,
            strip,
            &mut self.selected_blocks,
            &mut self.canvas_state,
            s,
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
