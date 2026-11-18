use egui::Context;

use crate::catalog::TerminalCatalog;
use crate::i18n::Strings;
use crate::model::library::{StripLibrary, StripTemplate};
use crate::model::TerminalStrip;
use crate::ui::icons::{
    compact_mfg_badge, danger_icon_button, draw_icon, icon_button, icon_only_button,
    primary_icon_button, show_responsive_modal, success_icon_button, UiIcon,
};
use crate::ui::theme::secondary_text_color;

/// Dialog state for saving an existing terminal strip into the personal library.
#[derive(Debug, Clone, Default)]
pub struct SaveStripLibraryDialogState {
    pub open: bool,
    pub strip_idx: usize,
    pub template_name: String,
    pub description: String,
    pub overwrite_existing: bool,
}

impl SaveStripLibraryDialogState {
    pub fn open_for_strip(&mut self, strip_idx: usize, strip: &TerminalStrip) {
        self.open = true;
        self.strip_idx = strip_idx;
        let desc_trimmed = strip.description.trim();
        self.template_name = if !desc_trimmed.is_empty() {
            desc_trimmed.to_string()
        } else {
            format!("{} ({} Klemmen)", strip.name, strip.blocks.len())
        };
        self.description = strip.description.clone();
        self.overwrite_existing = false;
    }
}

/// Confirmed result from the "Save Strip to Library" modal.
#[derive(Debug, Clone)]
pub struct SaveStripLibraryConfirm {
    pub strip_idx: usize,
    pub template_name: String,
    pub description: String,
    pub overwrite_id: Option<String>,
}

/// Renders the modal for naming and saving a terminal strip into the personal library.
pub fn show_save_strip_to_library_modal(
    ctx: &Context,
    state: &mut SaveStripLibraryDialogState,
    strip: Option<&TerminalStrip>,
    library: &StripLibrary,
    strings: &Strings,
) -> Option<SaveStripLibraryConfirm> {
    if !state.open {
        return None;
    }
    let Some(strip) = strip else {
        state.open = false;
        return None;
    };

    struct SaveModalData<'a> {
        state: &'a mut SaveStripLibraryDialogState,
        strip: &'a TerminalStrip,
        existing_match: Option<(String, String)>,
        confirmed: Option<SaveStripLibraryConfirm>,
    }

    let existing_match = library
        .find_by_name_ignore_case(&state.template_name)
        .map(|t| (t.id.clone(), t.name.clone()));

    let mut data = SaveModalData {
        state,
        strip,
        existing_match,
        confirmed: None,
    };
    let mut open = data.state.open;

    show_responsive_modal(
        ctx,
        "save_strip_library_modal",
        UiIcon::Bookmark,
        "Klemmleiste in Bibliothek speichern",
        540.0,
        &mut open,
        &mut data,
        |ui, data| {
            let sec = secondary_text_color(ui);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(
                        "Speichert Aufbau, Klemmentypen, Brückungs-Potentiale, Nummerierungsmodus und Endhalter als wiederverwendbare Vorlage.",
                    )
                    .color(sec),
                )
                .wrap(),
            );
            ui.add_space(6.0);

            // Compact strip preview summary
            egui::Frame::group(ui.style())
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            egui::RichText::new(format!("Aktive Leiste: {}", data.strip.name))
                                .strong(),
                        );
                        ui.label(
                            egui::RichText::new(format!(
                                "· {} {} · {:.1} mm · {} Potential(e)",
                                data.strip.blocks.len(),
                                strings.blocks_unit,
                                data.strip.total_width_mm(),
                                data.strip.potentials.len(),
                            ))
                            .color(sec),
                        );
                    });
                });

            ui.add_space(8.0);
            let input_w = (ui.available_width() - 16.0).clamp(160.0, 460.0);
            ui.label(egui::RichText::new("Vorlagenname *").strong());
            ui.add(
                egui::TextEdit::singleline(&mut data.state.template_name)
                    .hint_text("z. B. Einspeisung 24V DC, Motorabgang 400V …")
                    .desired_width(input_w),
            );
            if data.state.template_name.trim().is_empty() {
                ui.label(
                    egui::RichText::new("Bitte einen Namen für die Vorlage eingeben.")
                        .color(egui::Color32::from_rgb(239, 68, 68))
                        .small(),
                );
            }

            ui.add_space(6.0);
            ui.label(egui::RichText::new("Beschreibung / Suchbegriffe (optional)").strong());
            ui.add(
                egui::TextEdit::singleline(&mut data.state.description)
                    .hint_text("z. B. Schaltschrank Hauptverteilung, mit N-Sammelschiene")
                    .desired_width(input_w),
            );

            if let Some((_, existing_name)) = &data.existing_match {
                ui.add_space(6.0);
                ui.checkbox(
                    &mut data.state.overwrite_existing,
                    format!(
                        "Vorhandene Vorlage „{existing_name}“ aktualisieren (überschreiben)"
                    ),
                );
            }
        },
        |ui, data, close_requested| {
            let can_save = !data.state.template_name.trim().is_empty();
            ui.add_enabled_ui(can_save, |ui| {
                if success_icon_button(ui, UiIcon::Bookmark, "In Bibliothek speichern").clicked() {
                    let overwrite_id = if data.state.overwrite_existing {
                        data.existing_match.as_ref().map(|(id, _)| id.clone())
                    } else {
                        None
                    };
                    data.confirmed = Some(SaveStripLibraryConfirm {
                        strip_idx: data.state.strip_idx,
                        template_name: data.state.template_name.trim().to_string(),
                        description: data.state.description.trim().to_string(),
                        overwrite_id,
                    });
                    *close_requested = true;
                }
            });
            if icon_button(ui, UiIcon::Close, strings.btn_cancel).clicked() {
                *close_requested = true;
            }
        },
    );

    data.state.open = open && data.confirmed.is_none();
    data.confirmed
}

/// Dialog state for browsing, searching, renaming, deleting, and inserting personal strip templates.
#[derive(Debug, Clone, Default)]
pub struct StripLibraryDialogState {
    pub open: bool,
    pub search_query: String,
    pub target_designation: String,
    pub editing_id: Option<String>,
    pub edit_name: String,
    pub edit_description: String,
    pub confirm_delete_id: Option<String>,
}

impl StripLibraryDialogState {
    pub fn open_browser(&mut self) {
        self.open = true;
        self.editing_id = None;
        self.confirm_delete_id = None;
        self.target_designation.clear();
    }
}

/// Action chosen by the user inside the personal strip library browser.
#[derive(Debug, Clone)]
pub enum StripLibraryAction {
    InsertTemplate {
        template_id: String,
        target_designation: Option<String>,
    },
    SaveActiveStrip,
}

/// Renders the responsive "Personal Terminal Strip Library" modal with live search,
/// inline rename/edit, delete confirmation, and 1-click insertion into the active project.
#[allow(clippy::too_many_arguments)]
pub fn show_strip_library_modal(
    ctx: &Context,
    state: &mut StripLibraryDialogState,
    library: &mut StripLibrary,
    catalog: &TerminalCatalog,
    has_active_project: bool,
    can_save_active_strip: bool,
    strings: &Strings,
    status_message: &mut String,
) -> Option<StripLibraryAction> {
    if !state.open {
        return None;
    }

    struct BrowserData<'a> {
        state: &'a mut StripLibraryDialogState,
        library: &'a mut StripLibrary,
        catalog: &'a TerminalCatalog,
        has_active_project: bool,
        can_save_active_strip: bool,
        strings: &'a Strings,
        status_message: &'a mut String,
        action: Option<StripLibraryAction>,
    }

    let mut data = BrowserData {
        state,
        library,
        catalog,
        has_active_project,
        can_save_active_strip,
        strings,
        status_message,
        action: None,
    };
    let mut open = data.state.open;

    show_responsive_modal(
        ctx,
        "strip_library_browser_modal",
        UiIcon::Library,
        "Persönliche Klemmleisten-Bibliothek",
        680.0,
        &mut open,
        &mut data,
        |ui, data| {
            let sec = secondary_text_color(ui);
            let total_count = data.library.templates.len();

            // Search bar + hit counter
            ui.horizontal(|ui| {
                let (sr, _) = ui.allocate_exact_size(egui::Vec2::splat(16.0), egui::Sense::hover());
                draw_icon(ui.painter(), sr, UiIcon::Search, sec);

                let clear_reserve = if data.state.search_query.is_empty() {
                    68.0
                } else {
                    102.0
                };
                let search_w = (ui.available_width() - clear_reserve).max(120.0);
                ui.add(
                    egui::TextEdit::singleline(&mut data.state.search_query)
                        .hint_text("Nach Name, Beschreibung, Klemmentyp oder Art.-Nr. suchen …")
                        .desired_width(search_w),
                );
                if !data.state.search_query.is_empty()
                    && icon_only_button(ui, UiIcon::Clear, false, "Suche leeren").clicked()
                {
                    data.state.search_query.clear();
                }
                let hits = data
                    .library
                    .filter(&data.state.search_query, data.catalog)
                    .len();
                ui.label(
                    egui::RichText::new(format!("{hits}/{total_count}"))
                        .color(sec)
                        .small(),
                );
            });

            if total_count > 0 {
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        egui::RichText::new("Ziel-Bezeichnung beim Einfügen (optional):")
                            .color(sec)
                            .small(),
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut data.state.target_designation)
                            .hint_text("Auto (-X#)")
                            .desired_width(120.0),
                    );
                });
            }

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            if total_count == 0 {
                egui::Frame::group(ui.style())
                    .inner_margin(egui::Margin::same(14))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Noch keine eigenen Klemmleisten gespeichert.")
                                .strong(),
                        );
                        ui.add_space(4.0);
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(
                                    "Konfiguriere eine Klemmleiste im Editor und klicke links unter „Klemmleisten“ auf „In Bibliothek“, um sie hier dauerhaft mit eigenem Namen abzulegen.",
                                )
                                .color(sec),
                            )
                            .wrap(),
                        );
                    });
                return;
            }

            let matching: Vec<StripTemplate> = data
                .library
                .filter(&data.state.search_query, data.catalog)
                .into_iter()
                .cloned()
                .collect();

            if matching.is_empty() {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!(
                            "Keine Vorlagen gefunden für „{}“.",
                            data.state.search_query.trim()
                        ))
                        .color(sec),
                    )
                    .wrap(),
                );
                return;
            }

            let mut pending_rename: Option<(String, String, String)> = None;
            let mut pending_delete: Option<String> = None;

            for tpl in &matching {
                let mfgs = tpl.manufacturers_summary();
                let block_summary = tpl.block_type_summary(4);
                let is_editing = data.state.editing_id.as_deref() == Some(&tpl.id);
                let is_confirm_delete = data.state.confirm_delete_id.as_deref() == Some(&tpl.id);

                egui::Frame::group(ui.style())
                    .inner_margin(egui::Margin::same(10))
                    .show(ui, |ui| {
                        ui.set_width((ui.available_width() - 4.0).max(160.0));

                        // Header row: manufacturer badges + template name + dimensions summary
                        ui.horizontal_wrapped(|ui| {
                            for mfg in &mfgs {
                                compact_mfg_badge(ui, mfg);
                            }
                            ui.label(egui::RichText::new(&tpl.name).strong().size(15.0));
                        });

                        let pot_info = if tpl.strip.potentials.is_empty() {
                            String::new()
                        } else {
                            format!(" · {} Potential(e)", tpl.strip.potentials.len())
                        };
                        ui.label(
                            egui::RichText::new(format!(
                                "{} {} · {:.1} mm · Orig: {}{}",
                                tpl.strip.blocks.len(),
                                data.strings.blocks_unit,
                                tpl.strip.total_width_mm(),
                                tpl.strip.name,
                                pot_info
                            ))
                            .color(sec)
                            .small(),
                        );

                        if is_editing {
                            ui.add_space(4.0);
                            let edit_w = (ui.available_width() - 16.0).clamp(140.0, 460.0);
                            ui.label(egui::RichText::new("Vorlagenname:").small().strong());
                            ui.add(
                                egui::TextEdit::singleline(&mut data.state.edit_name)
                                    .desired_width(edit_w),
                            );
                            ui.label(egui::RichText::new("Beschreibung:").small().strong());
                            ui.add(
                                egui::TextEdit::singleline(&mut data.state.edit_description)
                                    .desired_width(edit_w),
                            );
                            ui.add_space(4.0);
                            ui.horizontal_wrapped(|ui| {
                                let valid = !data.state.edit_name.trim().is_empty();
                                ui.add_enabled_ui(valid, |ui| {
                                    if success_icon_button(ui, UiIcon::Check, "Speichern").clicked()
                                    {
                                        pending_rename = Some((
                                            tpl.id.clone(),
                                            data.state.edit_name.clone(),
                                            data.state.edit_description.clone(),
                                        ));
                                    }
                                });
                                if icon_button(ui, UiIcon::Close, data.strings.btn_cancel).clicked()
                                {
                                    data.state.editing_id = None;
                                }
                            });
                        } else {
                            if !tpl.description.is_empty() {
                                ui.add_space(2.0);
                                ui.add(egui::Label::new(&tpl.description).wrap());
                            }
                            if !block_summary.is_empty() {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(block_summary).color(sec).small(),
                                    )
                                    .wrap(),
                                );
                            }

                            ui.add_space(6.0);
                            ui.horizontal_wrapped(|ui| {
                                let insert_label = if data.has_active_project {
                                    "In Projekt einfügen"
                                } else {
                                    "Projekt damit starten"
                                };
                                if primary_icon_button(ui, UiIcon::Add, insert_label).clicked() {
                                    let target = {
                                        let t = data.state.target_designation.trim();
                                        if t.is_empty() {
                                            None
                                        } else {
                                            Some(t.to_string())
                                        }
                                    };
                                    data.action = Some(StripLibraryAction::InsertTemplate {
                                        template_id: tpl.id.clone(),
                                        target_designation: target,
                                    });
                                }

                                if icon_button(ui, UiIcon::Edit, "Umbenennen").clicked() {
                                    data.state.editing_id = Some(tpl.id.clone());
                                    data.state.edit_name = tpl.name.clone();
                                    data.state.edit_description = tpl.description.clone();
                                    data.state.confirm_delete_id = None;
                                }

                                if is_confirm_delete {
                                    if danger_icon_button(
                                        ui,
                                        UiIcon::Delete,
                                        "Wirklich löschen?",
                                    )
                                    .clicked()
                                    {
                                        pending_delete = Some(tpl.id.clone());
                                    }
                                    if icon_button(ui, UiIcon::Close, data.strings.btn_cancel)
                                        .clicked()
                                    {
                                        data.state.confirm_delete_id = None;
                                    }
                                } else if icon_button(ui, UiIcon::Delete, "Löschen").clicked() {
                                    data.state.confirm_delete_id = Some(tpl.id.clone());
                                }
                            });
                        }
                    });
                ui.add_space(6.0);
            }

            if let Some((id, new_name, new_desc)) = pending_rename {
                if data.library.rename_template(&id, new_name, new_desc) {
                    match data.library.save_default() {
                        Ok(_) => {
                            *data.status_message = "Bibliothek-Vorlage aktualisiert.".to_string();
                        }
                        Err(e) => {
                            *data.status_message =
                                format!("{}: {e}", data.strings.status_error);
                        }
                    }
                }
                data.state.editing_id = None;
            }

            if let Some(id) = pending_delete {
                if data.library.delete_template(&id) {
                    match data.library.save_default() {
                        Ok(_) => {
                            *data.status_message = "Vorlage aus Bibliothek entfernt.".to_string();
                        }
                        Err(e) => {
                            *data.status_message =
                                format!("{}: {e}", data.strings.status_error);
                        }
                    }
                }
                data.state.confirm_delete_id = None;
            }
        },
        |ui, data, close_requested| {
            if data.can_save_active_strip
                && success_icon_button(ui, UiIcon::Bookmark, "Aktive Leiste speichern").clicked()
            {
                data.action = Some(StripLibraryAction::SaveActiveStrip);
                *close_requested = true;
            }
            if icon_button(ui, UiIcon::Close, "Schließen").clicked() {
                *close_requested = true;
            }
        },
    );

    if data.action.is_some() {
        open = false;
    }
    data.state.open = open;
    data.action
}
