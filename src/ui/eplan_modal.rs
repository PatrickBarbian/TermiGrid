use std::path::Path;

use egui::Context;

use crate::eplan::EplanConfig;
use crate::i18n::{tr, Language, Strings};
use crate::ui::icons::{
    icon_button, icon_only_button, primary_icon_button, show_responsive_modal,
    subtle_success_icon_button, UiIcon,
};
use crate::ui::theme::{secondary_text_color, section_card, tokens};

/// Interactive state for the `EPLAN-Schnittstelle...` modal dialog.
#[derive(Debug, Clone, Default)]
pub struct EplanModalState {
    pub open: bool,
    pub draft: EplanConfig,
}

impl EplanModalState {
    pub fn open_with(&mut self, cfg: &EplanConfig) {
        self.draft = cfg.clone();
        self.open = true;
    }
}

/// Actions triggered from inside the `EPLAN-Schnittstelle...` modal.
#[derive(Debug, Clone)]
pub enum EplanModalAction {
    SaveConfig(EplanConfig),
    ImportCsv(EplanConfig),
    ExportPackage(EplanConfig),
    ExportCSharpScript(EplanConfig),
    PickDefaultDir,
}

struct ModalData<'a> {
    draft: &'a mut EplanConfig,
    has_strips: bool,
    last_csv_dir: Option<&'a Path>,
    lang: Language,
    cancel_label: &'a str,
    action: Option<EplanModalAction>,
}

/// Renders the tucked-away `Menü → EPLAN-Schnittstelle...` settings & roundtrip modal.
pub fn show_eplan_interface_modal(
    ctx: &Context,
    state: &mut EplanModalState,
    has_strips: bool,
    last_csv_dir: Option<&Path>,
    s: &Strings,
) -> Option<EplanModalAction> {
    if !state.open {
        return None;
    }

    let lang = s.lang;
    let mut open = state.open;
    let mut data = ModalData {
        draft: &mut state.draft,
        has_strips,
        last_csv_dir,
        lang,
        cancel_label: s.btn_cancel,
        action: None,
    };

    show_responsive_modal(
        ctx,
        "eplan_interface_modal",
        UiIcon::Settings,
        tr::eplan_modal_title(lang),
        560.0,
        &mut open,
        &mut data,
        |ui, data| {
            let t = tokens(ui);
            let lang = data.lang;

            ui.label(
                egui::RichText::new(tr::eplan_modal_subtitle(lang))
                    .size(12.5)
                    .color(t.text_secondary),
            );
            ui.add_space(10.0);

            // Card 1: Vendor Prefixes
            section_card(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(
                    egui::RichText::new(tr::eplan_prefixes_heading(lang))
                        .size(13.0)
                        .strong()
                        .color(t.text_primary),
                );
                ui.add_space(6.0);
                egui::Grid::new("eplan_prefixes_grid")
                    .num_columns(2)
                    .spacing([14.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Phoenix Contact")
                                .size(12.5)
                                .color(t.text_primary),
                        );
                        ui.add(
                            egui::TextEdit::singleline(&mut data.draft.prefix_phoenix)
                                .hint_text("PXC.")
                                .desired_width(130.0),
                        );
                        ui.end_row();

                        ui.label(
                            egui::RichText::new("WAGO")
                                .size(12.5)
                                .color(t.text_primary),
                        );
                        ui.add(
                            egui::TextEdit::singleline(&mut data.draft.prefix_wago)
                                .hint_text("WAG.")
                                .desired_width(130.0),
                        );
                        ui.end_row();

                        ui.label(
                            egui::RichText::new("Weidmüller")
                                .size(12.5)
                                .color(t.text_primary),
                        );
                        ui.add(
                            egui::TextEdit::singleline(&mut data.draft.prefix_weidmuller)
                                .hint_text("WDM.")
                                .desired_width(130.0),
                        );
                        ui.end_row();
                    });
            });

            ui.add_space(8.0);

            // Card 2: CSV & Roundtrip Options
            section_card(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(
                    egui::RichText::new(tr::eplan_options_heading(lang))
                        .size(13.0)
                        .strong()
                        .color(t.text_primary),
                );
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(tr::eplan_delimiter_label(lang))
                            .size(12.5)
                            .color(t.text_primary),
                    );
                    let delim_label = match data.draft.csv_delimiter {
                        ';' => ";  (Semikolon / EPLAN DE)",
                        ',' => ",  (Komma / International)",
                        '\t' => "Tab  (TSV)",
                        _ => ";",
                    };
                    egui::ComboBox::from_id_salt("eplan_csv_delimiter_combo")
                        .width(210.0)
                        .selected_text(delim_label)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut data.draft.csv_delimiter,
                                ';',
                                ";  (Semikolon / EPLAN DE)",
                            );
                            ui.selectable_value(
                                &mut data.draft.csv_delimiter,
                                ',',
                                ",  (Komma / International)",
                            );
                            ui.selectable_value(
                                &mut data.draft.csv_delimiter,
                                '\t',
                                "Tab  (TSV)",
                            );
                        });
                });

                ui.add_space(6.0);
                ui.checkbox(
                    &mut data.draft.export_added_accessories_only,
                    tr::eplan_only_added_checkbox(lang),
                );

                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(tr::eplan_default_dir_label(lang))
                        .size(12.0)
                        .color(secondary_text_color(ui)),
                );
                ui.horizontal_wrapped(|ui| {
                    let dir_display = if let Some(ref dir) = data.draft.default_export_dir {
                        dir.display().to_string()
                    } else if let Some(csv_dir) = data.last_csv_dir {
                        format!("{} (CSV)", csv_dir.display())
                    } else {
                        tr::eplan_default_dir_auto(lang).to_string()
                    };
                    ui.label(
                        egui::RichText::new(dir_display)
                            .monospace()
                            .size(11.5)
                            .color(t.text_secondary),
                    );
                });
                ui.horizontal(|ui| {
                    if icon_button(ui, UiIcon::FolderOpen, tr::eplan_choose_folder(lang)).clicked()
                    {
                        data.action = Some(EplanModalAction::PickDefaultDir);
                    }
                    if data.draft.default_export_dir.is_some()
                        && icon_only_button(
                            ui,
                            UiIcon::Clear,
                            false,
                            tr::reset_factory(lang),
                        )
                        .clicked()
                    {
                        data.draft.default_export_dir = None;
                    }
                });
            });

            ui.add_space(8.0);

            // Card 3: Direct Roundtrip Actions & C# Script
            section_card(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(
                    egui::RichText::new(tr::eplan_actions_heading(lang))
                        .size(13.0)
                        .strong()
                        .color(t.text_primary),
                );
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    if icon_button(ui, UiIcon::ExportCsv, tr::eplan_import_csv_btn(lang)).clicked()
                    {
                        data.action = Some(EplanModalAction::ImportCsv(data.draft.clone()));
                    }

                    let pkg_resp = ui
                        .add_enabled_ui(data.has_strips, |ui| {
                            subtle_success_icon_button(
                                ui,
                                UiIcon::ExportDxf,
                                tr::eplan_export_package_btn(lang),
                            )
                        })
                        .inner;
                    if pkg_resp.clicked() {
                        data.action = Some(EplanModalAction::ExportPackage(data.draft.clone()));
                    }

                    if icon_button(ui, UiIcon::NewFile, tr::eplan_export_cs_btn(lang)).clicked() {
                        data.action =
                            Some(EplanModalAction::ExportCSharpScript(data.draft.clone()));
                    }
                });
            });
        },
        |ui, data, close_requested| {
            let lang = data.lang;
            if primary_icon_button(ui, UiIcon::Save, tr::eplan_save_close_btn(lang)).clicked() {
                data.action = Some(EplanModalAction::SaveConfig(data.draft.clone()));
                *close_requested = true;
            }
            if icon_button(ui, UiIcon::Close, data.cancel_label).clicked() {
                *close_requested = true;
            }
        },
    );

    if matches!(data.action, Some(EplanModalAction::ImportCsv(_))) {
        open = false;
    }
    state.open = open;
    data.action
}
