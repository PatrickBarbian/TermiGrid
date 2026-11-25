use egui::{Color32, Context, CornerRadius, Frame, Margin, Pos2, Rect, Stroke, Vec2};

use crate::catalog::accessories::find_end_bracket;
use crate::catalog::TerminalCatalog;
use crate::i18n::{tr, Strings};
use crate::import::planner::{PdfBomImportPlan, StripImportPlan};
use crate::ui::icons::{
    compact_mfg_badge, icon_button, primary_icon_button, show_responsive_modal, UiIcon,
};
use crate::ui::theme::secondary_text_color;

/// Per-strip interactive selection inside the PDF BOM import dialog.
#[derive(Debug, Clone)]
pub struct StripImportChoice {
    pub selected: bool,
    pub selected_variant: usize,
    pub include_end_clamps: bool,
    /// Per-unknown-article mapping to a catalog `spec_id` (`None` = skip/unmapped).
    pub unknown_mappings: Vec<Option<String>>,
    /// Per-unknown-article search filter text in the mapping dropdown.
    pub unknown_searches: Vec<String>,
}

/// Confirmed result returned when the user clicks `"Ausgewählte Leisten übernehmen"`.
#[derive(Debug, Clone)]
pub struct SelectedStripImport {
    pub plan: StripImportPlan,
    pub variant_index: usize,
    pub include_end_clamps: bool,
    pub unknown_mappings: Vec<Option<String>>,
}

/// Interactive state for the PDF BOM import modal.
#[derive(Debug, Clone, Default)]
pub struct PdfImportDialogState {
    pub open: bool,
    pub source_label: String,
    pub error_message: Option<String>,
    pub plan: Option<PdfBomImportPlan>,
    pub choices: Vec<StripImportChoice>,
}

impl PdfImportDialogState {
    pub fn open_with_plan(&mut self, source_label: String, plan: PdfBomImportPlan) {
        self.choices = plan
            .strips
            .iter()
            .map(|s| {
                let unk_len = s.unknown_articles.len();
                StripImportChoice {
                    selected: s.total_terminal_count > 0 && !s.variants.is_empty(),
                    selected_variant: 0,
                    include_end_clamps: s.end_parts.has_pdf_end_clamps
                        || s.end_parts.auto_complement_brackets,
                    unknown_mappings: vec![None; unk_len],
                    unknown_searches: vec![String::new(); unk_len],
                }
            })
            .collect();
        self.source_label = source_label;
        self.error_message = None;
        self.plan = Some(plan);
        self.open = true;
    }

    pub fn open_with_error(&mut self, source_label: String, err: String) {
        self.source_label = source_label;
        self.error_message = Some(err);
        self.plan = None;
        self.choices.clear();
        self.open = true;
    }
}

fn strip_has_importable_blocks(strip: &StripImportPlan, choice: &StripImportChoice) -> bool {
    (strip.total_terminal_count > 0 && !strip.variants.is_empty())
        || choice.unknown_mappings.iter().any(|m| m.is_some())
}

fn draw_pill_badge(ui: &mut egui::Ui, text: &str, bg: Color32, fg: Color32) {
    Frame::NONE
        .fill(bg)
        .corner_radius(CornerRadius::same(4))
        .inner_margin(Margin::symmetric(6, 2))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(text).color(fg).strong().size(11.0));
        });
}

fn draw_strip_sequence_preview(
    ui: &mut egui::Ui,
    spec_ids: &[String],
    catalog: &TerminalCatalog,
) {
    if spec_ids.is_empty() {
        return;
    }
    let avail_w = (ui.available_width() - 8.0).clamp(120.0, 620.0);
    let h = 14.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(avail_w, h), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(2), ui.visuals().faint_bg_color);

    let count = spec_ids.len() as f32;
    let step = (avail_w / count).max(1.5);
    let mut x = rect.min.x;

    for id in spec_ids {
        let block_w = step.min(rect.max.x - x).max(1.0);
        if block_w <= 0.2 || x >= rect.max.x {
            break;
        }
        let color = if let Some(spec) = catalog.find_by_id(id) {
            if spec.is_nt_disconnect_terminal() {
                Color32::from_rgb(14, 116, 144)
            } else if !spec.tiers.is_empty() && spec.tiers.iter().all(|t| t.behavior.is_pe()) {
                Color32::from_rgb(34, 197, 94)
            } else if !spec.tiers.is_empty() && spec.tiers.iter().all(|t| t.behavior.is_neutral()) {
                Color32::from_rgb(37, 99, 235)
            } else {
                Color32::from_rgb(120, 125, 134)
            }
        } else {
            Color32::GRAY
        };

        let r = Rect::from_min_max(Pos2::new(x, rect.min.y + 1.0), Pos2::new(x + block_w, rect.max.y - 1.0));
        painter.rect_filled(r, CornerRadius::ZERO, color);
        if step >= 4.0 {
            painter.line_segment(
                [Pos2::new(r.max.x, rect.min.y + 1.0), Pos2::new(r.max.x, rect.max.y - 1.0)],
                Stroke::new(0.6, ui.visuals().window_fill),
            );
        }
        x += step;
    }
}

/// Renders the intelligent EPLAN PDF BOM import modal.
/// Returns `Some(Vec<SelectedStripImport>)` when the user confirms importing the selected strips.
pub fn show_pdf_import_modal(
    ctx: &Context,
    state: &mut PdfImportDialogState,
    catalog: &TerminalCatalog,
    strings: &Strings,
) -> Option<Vec<SelectedStripImport>> {
    if !state.open {
        return None;
    }

    let mut confirmed: Option<Vec<SelectedStripImport>> = None;
    let mut open = state.open;
    let lang = strings.lang;
    let title = if state.source_label.is_empty() {
        tr::pdf_import_title(lang).to_string()
    } else {
        tr::pdf_import_file_title(lang, &state.source_label)
    };

    show_responsive_modal(
        ctx,
        "pdf_bom_import_modal",
        UiIcon::ImportPdf,
        &title,
        780.0,
        &mut open,
        state,
        |ui, state| {
            if let Some(err) = &state.error_message {
                Frame::group(ui.style())
                    .fill(Color32::from_rgba_unmultiplied(220, 38, 38, 28))
                    .show(ui, |ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(err)
                                    .color(Color32::from_rgb(239, 68, 68))
                                    .strong(),
                            )
                            .wrap(),
                        );
                    });
                return;
            }

            let Some(plan) = state.plan.as_ref() else {
                ui.label(tr::pdf_no_data(lang));
                return;
            };

            let recognized_total: usize =
                plan.strips.iter().map(|s| s.total_terminal_count).sum();
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new(tr::pdf_summary(
                        lang,
                        plan.strips.len(),
                        recognized_total,
                        plan.skipped_foreign.len(),
                        plan.ignored_non_x_rows.len(),
                    ))
                    .color(secondary_text_color(ui)),
                );
            });
            ui.add_space(4.0);

            if !plan.strips.is_empty() {
                ui.horizontal_wrapped(|ui| {
                    if icon_button(ui, UiIcon::SelectAll, tr::pdf_all_selectable(lang)).clicked() {
                        for (s, c) in plan.strips.iter().zip(state.choices.iter_mut()) {
                            c.selected = strip_has_importable_blocks(s, c);
                        }
                    }
                    if icon_button(ui, UiIcon::SelectNone, tr::select_none(strings.lang)).clicked() {
                        for c in &mut state.choices {
                            c.selected = false;
                        }
                    }
                });
                ui.add_space(4.0);
            }

            if plan.strips.is_empty() {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(tr::pdf_none_found(lang))
                            .color(secondary_text_color(ui)),
                    )
                    .wrap(),
                );
            }

            for (idx, (strip, choice)) in plan
                .strips
                .iter()
                .zip(state.choices.iter_mut())
                .enumerate()
            {
                // Ensure mapping vectors stay sized to unknown_articles
                if choice.unknown_mappings.len() != strip.unknown_articles.len() {
                    choice.unknown_mappings.resize(strip.unknown_articles.len(), None);
                }
                if choice.unknown_searches.len() != strip.unknown_articles.len() {
                    choice
                        .unknown_searches
                        .resize(strip.unknown_articles.len(), String::new());
                }

                let mapped_unknown_count: usize = strip
                    .unknown_articles
                    .iter()
                    .zip(choice.unknown_mappings.iter())
                    .filter_map(|(u, m)| m.as_ref().map(|_| u.quantity))
                    .sum();
                let effective_total = strip.total_terminal_count + mapped_unknown_count;
                let can_import = strip_has_importable_blocks(strip, choice);
                let card_stroke = if choice.selected && can_import {
                    Stroke::new(1.2, Color32::from_rgb(2, 132, 199))
                } else {
                    ui.visuals().widgets.noninteractive.bg_stroke
                };

                Frame::group(ui.style())
                    .stroke(card_stroke)
                    .inner_margin(Margin::same(10))
                    .show(ui, |ui| {
                        // Header row: Checkbox + BMK + Mfg badge(s) + Counts + Status Badges
                        ui.horizontal_wrapped(|ui| {
                            if can_import {
                                ui.checkbox(&mut choice.selected, "");
                            } else {
                                let mut dummy = false;
                                ui.add_enabled(false, egui::Checkbox::new(&mut dummy, ""));
                            }
                            ui.label(egui::RichText::new(&strip.bmk).strong().size(15.5));
                            if strip.manufacturers.is_empty() {
                                compact_mfg_badge(ui, &strip.dominant_manufacturer);
                            } else {
                                for mfg in &strip.manufacturers {
                                    compact_mfg_badge(ui, mfg);
                                }
                            }

                            let page_str = if strip.pages.len() <= 1 {
                                format!("P. {}", strip.pages.first().copied().unwrap_or(1))
                            } else {
                                format!(
                                    "P. {}-{}",
                                    strip.pages.first().copied().unwrap_or(1),
                                    strip.pages.last().copied().unwrap_or(1)
                                )
                            };
                            ui.label(
                                egui::RichText::new(format!(
                                    "({effective_total} {} | {page_str})",
                                    strings.blocks_unit
                                ))
                                .color(secondary_text_color(ui)),
                            );

                            if strip.has_nt_or_install {
                                draw_pill_badge(
                                    ui,
                                    tr::pdf_nt_badge(lang),
                                    Color32::from_rgb(14, 116, 144),
                                    Color32::WHITE,
                                );
                            }
                            if strip.end_parts.has_pdf_end_clamps
                                || strip.end_parts.has_pdf_end_plates
                            {
                                draw_pill_badge(
                                    ui,
                                    "PDF End Parts",
                                    Color32::from_rgb(15, 118, 110),
                                    Color32::WHITE,
                                );
                            } else if strip.end_parts.auto_complement_brackets
                                && choice.include_end_clamps
                            {
                                draw_pill_badge(
                                    ui,
                                    tr::pdf_end_parts_badge(lang),
                                    Color32::from_rgb(22, 163, 74),
                                    Color32::WHITE,
                                );
                            }
                            if !strip.ignored_jumpers.is_empty() {
                                draw_pill_badge(
                                    ui,
                                    tr::pdf_jumpers_ignored(lang),
                                    Color32::from_rgb(100, 116, 139),
                                    Color32::WHITE,
                                );
                            }
                            let unmapped_count = choice
                                .unknown_mappings
                                .iter()
                                .filter(|m| m.is_none())
                                .count();
                            if unmapped_count > 0 {
                                draw_pill_badge(
                                    ui,
                                    "Heuristik / Nicht in DB",
                                    Color32::from_rgb(217, 119, 6),
                                    Color32::WHITE,
                                );
                            } else if !strip.unknown_articles.is_empty() {
                                draw_pill_badge(
                                    ui,
                                    "Zugeordnet",
                                    Color32::from_rgb(22, 163, 74),
                                    Color32::WHITE,
                                );
                            }
                        });

                        // Recognized terminals summary
                        if !strip.terminals.is_empty() {
                            let summary = strip
                                .terminals
                                .iter()
                                .map(|t| format!("{}× {}", t.quantity, t.article_number))
                                .collect::<Vec<_>>()
                                .join("  |  ");
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(summary)
                                        .small()
                                        .color(secondary_text_color(ui)),
                                )
                                .wrap(),
                            );
                        }

                        // Unknown / heuristic terminal warnings + interactive catalog mapping
                        for (u_idx, unk) in strip.unknown_articles.iter().enumerate() {
                            let mapped_id = choice.unknown_mappings[u_idx].clone();
                            let mapped_spec =
                                mapped_id.as_deref().and_then(|id| catalog.find_by_id(id));
                            let tag_prefix = if unk.heuristic_matched {
                                "⚠ Heuristik (Nicht in DB)"
                            } else {
                                "⚠ Nicht in DB"
                            };
                            let warn_color = if mapped_spec.is_some() {
                                Color32::from_rgb(22, 163, 74)
                            } else {
                                Color32::from_rgb(217, 119, 6)
                            };
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(format!(
                                        "{tag_prefix}: {}× {} {} ({})",
                                        unk.quantity,
                                        unk.article_number,
                                        unk.type_number,
                                        unk.description
                                    ))
                                    .small()
                                    .color(warn_color),
                                )
                                .wrap(),
                            );

                            ui.push_id(("unk_map", idx, u_idx), |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    let current_label = if let Some(spec) = mapped_spec {
                                        format!("Zugeordnet: {} ({})", spec.name, spec.manufacturer)
                                    } else {
                                        "Zuordnen / bestätigen…".to_string()
                                    };
                                    egui::ComboBox::from_id_salt(("unk_combo", idx, u_idx))
                                        .selected_text(current_label)
                                        .width(280.0)
                                        .show_ui(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("Suche:");
                                                ui.text_edit_singleline(
                                                    &mut choice.unknown_searches[u_idx],
                                                );
                                            });
                                            ui.separator();
                                            if ui
                                                .selectable_label(
                                                    choice.unknown_mappings[u_idx].is_none(),
                                                    "— Nicht übernehmen (überspringen) —",
                                                )
                                                .clicked()
                                            {
                                                choice.unknown_mappings[u_idx] = None;
                                            }
                                            let q = choice.unknown_searches[u_idx].trim();
                                            let mfg_opt = (unk.manufacturer != "Unbekannt")
                                                .then_some(unk.manufacturer.as_str());
                                            for spec in catalog.filter_specs(mfg_opt, q).into_iter().take(40) {
                                                let selected = choice.unknown_mappings[u_idx]
                                                    .as_deref()
                                                    == Some(spec.id.as_str());
                                                let item_lbl = format!(
                                                    "{} | {} ({})",
                                                    spec.manufacturer,
                                                    spec.name,
                                                    spec.effective_article_number().unwrap_or("-")
                                                );
                                                if ui.selectable_label(selected, item_lbl).clicked() {
                                                    choice.unknown_mappings[u_idx] =
                                                        Some(spec.id.clone());
                                                    choice.selected = true;
                                                }
                                            }
                                        });

                                    if mapped_spec.is_none()
                                        && let Some(sug_id) = &unk.suggested_spec_id
                                        && let Some(sug_spec) = catalog.find_by_id(sug_id)
                                        && ui
                                            .small_button(format!(
                                                "✓ Als {} übernehmen",
                                                sug_spec.name.split(" (").next().unwrap_or(&sug_spec.name)
                                            ))
                                            .clicked()
                                    {
                                        choice.unknown_mappings[u_idx] = Some(sug_spec.id.clone());
                                        choice.selected = true;
                                    }
                                });
                            });
                        }

                        // Ignored jumpers detail
                        if !strip.ignored_jumpers.is_empty() {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(format!(
                                        "Brücker aus PDF ignoriert: {}",
                                        strip.ignored_jumpers.join(", ")
                                    ))
                                    .small()
                                    .color(secondary_text_color(ui)),
                                )
                                .wrap(),
                            );
                        }

                        // End clamps toggle / PDF summary
                        if can_import {
                            let eb = find_end_bracket(&strip.end_parts.end_bracket_id);
                            if strip.end_parts.has_pdf_end_clamps {
                                let plates_note = if strip.end_parts.has_pdf_end_plates {
                                    format!(
                                        " + {} Abschlussplatte(n) aus PDF",
                                        strip.end_parts.pdf_end_plate_count
                                    )
                                } else {
                                    String::new()
                                };
                                ui.checkbox(
                                    &mut choice.include_end_clamps,
                                    format!(
                                        "Endhalter aus PDF übernehmen ({}× {}{} – keine Duplikate)",
                                        strip.end_parts.pdf_end_clamp_count,
                                        eb.name,
                                        plates_note
                                    ),
                                );
                            } else {
                                ui.checkbox(
                                    &mut choice.include_end_clamps,
                                    format!(
                                        "Endhalter automatisch ergänzen (2× {} {}, links & rechts)",
                                        eb.manufacturer, eb.name
                                    ),
                                );
                            }
                        }

                        // Multiple ordering variants radio picker (shown ONLY when > 1 unique variant!)
                        if strip.variants.len() > 1 {
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new("Anordnungsvariante (Stückzahlen identisch):")
                                    .small()
                                    .strong(),
                            );
                            for (v_idx, var) in strip.variants.iter().enumerate() {
                                ui.push_id((idx, v_idx), |ui| {
                                    ui.radio_value(
                                        &mut choice.selected_variant,
                                        v_idx,
                                        &var.label,
                                    );
                                });
                            }
                            if let Some(active_var) = strip.variants.get(choice.selected_variant) {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(&active_var.description)
                                            .small()
                                            .color(secondary_text_color(ui)),
                                    )
                                    .wrap(),
                                );
                                draw_strip_sequence_preview(
                                    ui,
                                    &active_var.block_spec_ids,
                                    catalog,
                                );
                            }
                        } else if let Some(only_var) = strip.variants.first() {
                            draw_strip_sequence_preview(ui, &only_var.block_spec_ids, catalog);
                        }
                    });
                ui.add_space(6.0);
            }

            // Collapsible Skipped Foreign & Ignored Rows section
            if !plan.skipped_foreign.is_empty() || !plan.ignored_non_x_rows.is_empty() {
                egui::CollapsingHeader::new(format!(
                    "Übersprungen & ignoriertes Zubehör ({} Fremdhersteller, {} Nicht-Klemmen)",
                    plan.skipped_foreign.len(),
                    plan.ignored_non_x_rows.len()
                ))
                .default_open(false)
                .show(ui, |ui| {
                    for f in &plan.skipped_foreign {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(format!(
                                    "• {} | {}× {} [Hersteller: {}] – {}",
                                    f.bmk, f.quantity, f.article_number, f.manufacturer, f.description
                                ))
                                .small()
                                .color(secondary_text_color(ui)),
                            )
                            .wrap(),
                        );
                    }
                    if !plan.ignored_non_x_rows.is_empty() {
                        let sample: Vec<String> = plan
                            .ignored_non_x_rows
                            .iter()
                            .take(15)
                            .map(|r| format!("{} ({})", r.bmk, r.article_number))
                            .collect();
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(format!(
                                    "Gefiltert (Nicht-Klemmen): {}{}",
                                    sample.join(", "),
                                    if plan.ignored_non_x_rows.len() > 15 {
                                        " …"
                                    } else {
                                        ""
                                    }
                                ))
                                .small()
                                .color(secondary_text_color(ui)),
                            )
                            .wrap(),
                        );
                    }
                });
            }
        },
        |ui, state, close_requested| {
            let selected_count = state
                .plan
                .as_ref()
                .map(|p| {
                    p.strips
                        .iter()
                        .zip(state.choices.iter())
                        .filter(|(s, c)| c.selected && strip_has_importable_blocks(s, c))
                        .count()
                })
                .unwrap_or(0);

            let confirm_label = if selected_count > 0 {
                format!("Ausgewählte Leisten übernehmen ({selected_count})")
            } else {
                "Ausgewählte Leisten übernehmen".to_string()
            };

            let clicked = ui
                .add_enabled(selected_count > 0, |ui: &mut egui::Ui| {
                    primary_icon_button(ui, UiIcon::Check, &confirm_label)
                })
                .clicked();

            if clicked && let Some(plan) = state.plan.as_ref() {
                let picked: Vec<SelectedStripImport> = plan
                    .strips
                    .iter()
                    .zip(state.choices.iter())
                    .filter(|(s, c)| c.selected && strip_has_importable_blocks(s, c))
                    .map(|(s, c)| SelectedStripImport {
                        plan: s.clone(),
                        variant_index: c.selected_variant,
                        include_end_clamps: c.include_end_clamps,
                        unknown_mappings: c.unknown_mappings.clone(),
                    })
                    .collect();
                confirmed = Some(picked);
                *close_requested = true;
            }

            if icon_button(ui, UiIcon::Close, strings.btn_cancel).clicked() {
                *close_requested = true;
            }
        },
    );

    state.open = open;
    confirmed
}
