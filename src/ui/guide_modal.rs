use egui::Ui;

use crate::catalog::{TerminalCatalog, TerminalSpec, TierSpec};
use crate::i18n::{Language, Strings};
use crate::model::{NumberingOrder, TierBehavior, TierColor};
use crate::ui::logos::{logo_filter_button, show_logo_badge, BrandLogos};
use crate::ui::theme::secondary_text_color;
use crate::ui::updater::{
    open_external_url, UpdateChecker, UpdateStatus, BUYMEACOFFEE_URL, CURRENT_VERSION,
    GITHUB_REPO_URL,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CustomTierKind {
    #[default]
    Sequential,
    ProtectiveEarth,
    Fixed,
}

#[derive(Debug, Clone)]
pub struct CustomSpecDraft {
    pub manufacturer: String,
    pub name: String,
    pub width_mm: f32,
    pub levels: usize,
    pub tier_kinds: [CustomTierKind; 4],
    pub tier_fixed: [String; 4],
}

impl Default for CustomSpecDraft {
    fn default() -> Self {
        Self {
            manufacturer: "WAGO".to_string(),
            name: "2002-XXXX".to_string(),
            width_mm: 5.2,
            levels: 2,
            tier_kinds: [
                CustomTierKind::Sequential,
                CustomTierKind::Sequential,
                CustomTierKind::Sequential,
                CustomTierKind::Sequential,
            ],
            tier_fixed: [
                "N".to_string(),
                "N".to_string(),
                "N".to_string(),
                "N".to_string(),
            ],
        }
    }
}

/// Renders the dedicated "Anleitung" (Instructions) tab explaining selection, multi-level logic,
/// export, updates, and project links.
pub fn show_guide_tab(ui: &mut Ui, updater: &UpdateChecker, strings: &Strings) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(6.0);
        ui.heading(strings.guide_title);
        ui.add_space(8.0);

        for (title, body) in strings.guide_sections {
            egui::Frame::group(ui.style())
                .inner_margin(egui::Margin::same(12))
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width().min(860.0));
                    ui.label(egui::RichText::new(*title).strong().size(16.0));
                    ui.add_space(4.0);
                    ui.label(*body);
                });
            ui.add_space(8.0);
        }

        // About, VibeCoded notice, Update Checker, GitHub & BuyMeACoffee card
        egui::Frame::group(ui.style())
            .inner_margin(egui::Margin::same(12))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width().min(860.0));
                ui.label(
                    egui::RichText::new(format!("TermiGrid v{CURRENT_VERSION}"))
                        .strong()
                        .size(16.0),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(strings.about_vibecoded)
                        .color(secondary_text_color(ui)),
                );
                ui.add_space(8.0);

                ui.horizontal_wrapped(|ui| {
                    if ui.button(strings.btn_github).clicked() {
                        open_external_url(ui.ctx(), GITHUB_REPO_URL);
                    }
                    if ui.button(strings.btn_coffee).clicked() {
                        open_external_url(ui.ctx(), BUYMEACOFFEE_URL);
                    }
                    if ui.button(strings.btn_check_update).clicked() {
                        updater.check_async(ui.ctx().clone());
                    }

                    match updater.status() {
                        UpdateStatus::Idle => {}
                        UpdateStatus::Checking => {
                            ui.label(strings.update_checking);
                        }
                        UpdateStatus::UpToDate(tag) => {
                            ui.label(format!("✓ {} ({tag})", strings.update_up_to_date));
                        }
                        UpdateStatus::UpdateAvailable {
                            latest_tag,
                            release_url,
                        } => {
                            ui.label(
                                egui::RichText::new(format!(
                                    "⬆ {}: {latest_tag}",
                                    strings.update_available
                                ))
                                .strong(),
                            );
                            if ui.button(strings.update_open_release).clicked() {
                                open_external_url(ui.ctx(), &release_url);
                            }
                        }
                        UpdateStatus::Error(err) => {
                            ui.label(format!("{}: {err}", strings.update_error));
                        }
                    }
                });
            });
    });
}

/// Renders the searchable embedded terminal database table ("Klemmen-Datenbank") plus a form
/// to define custom terminal types stored with the project.
#[allow(clippy::too_many_arguments)]
pub fn show_database_tab(
    ui: &mut Ui,
    catalog: &mut TerminalCatalog,
    logos: &BrandLogos,
    custom_specs: &mut Vec<TerminalSpec>,
    mfg_filter: &mut Option<String>,
    search_query: &mut String,
    draft: &mut CustomSpecDraft,
    strings: &Strings,
    lang: Language,
) {
    ui.add_space(6.0);
    ui.heading(format!("{} ({})", strings.db_heading, catalog.all().len()));
    ui.add_space(6.0);

    egui::CollapsingHeader::new(strings.custom_spec_heading)
        .default_open(false)
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(strings.db_col_manufacturer);
                ui.add(egui::TextEdit::singleline(&mut draft.manufacturer).desired_width(130.0));
                ui.label(strings.db_col_name);
                ui.add(egui::TextEdit::singleline(&mut draft.name).desired_width(170.0));
                ui.label(strings.db_col_width);
                ui.add(
                    egui::DragValue::new(&mut draft.width_mm)
                        .speed(0.1)
                        .range(3.0..=30.0)
                        .suffix(" mm"),
                );
                ui.label(strings.db_col_levels);
                ui.add(egui::DragValue::new(&mut draft.levels).range(1..=4));
            });

            ui.add_space(4.0);
            for lvl in 0..draft.levels {
                ui.horizontal(|ui| {
                    ui.label(format!("E{}:", lvl + 1));
                    ui.selectable_value(
                        &mut draft.tier_kinds[lvl],
                        CustomTierKind::Sequential,
                        strings.custom_spec_tier_seq,
                    );
                    ui.selectable_value(
                        &mut draft.tier_kinds[lvl],
                        CustomTierKind::ProtectiveEarth,
                        strings.custom_spec_tier_pe,
                    );
                    ui.selectable_value(
                        &mut draft.tier_kinds[lvl],
                        CustomTierKind::Fixed,
                        strings.custom_spec_tier_fixed,
                    );
                    if draft.tier_kinds[lvl] == CustomTierKind::Fixed {
                        ui.add(
                            egui::TextEdit::singleline(&mut draft.tier_fixed[lvl])
                                .desired_width(70.0),
                        );
                    }
                });
            }

            if ui.button(strings.custom_spec_add).clicked() && !draft.name.trim().is_empty() {
                let tiers: Vec<TierSpec> = (0..draft.levels)
                    .map(|lvl| match draft.tier_kinds[lvl] {
                        CustomTierKind::Sequential => TierSpec {
                            behavior: TierBehavior::Sequential,
                            color: TierColor::StandardGray,
                        },
                        CustomTierKind::ProtectiveEarth => TierSpec {
                            behavior: TierBehavior::ProtectiveEarth,
                            color: TierColor::GreenYellowPe,
                        },
                        CustomTierKind::Fixed => {
                            let text = draft.tier_fixed[lvl].trim().to_string();
                            let color = if text.eq_ignore_ascii_case("N") {
                                TierColor::BlueNeutral
                            } else {
                                TierColor::StandardGray
                            };
                            TierSpec {
                                behavior: TierBehavior::Fixed(text),
                                color,
                            }
                        }
                    })
                    .collect();

                let slug: String = format!("{}-{}", draft.manufacturer, draft.name)
                    .to_lowercase()
                    .chars()
                    .map(|c| if c.is_alphanumeric() { c } else { '-' })
                    .collect();
                let id = format!("custom-{slug}-{}", custom_specs.len() + 1);
                let mfg = if draft.manufacturer.trim().is_empty() {
                    "Custom".to_string()
                } else {
                    draft.manufacturer.trim().to_string()
                };

                let spec = TerminalSpec {
                    id,
                    name: draft.name.trim().to_string(),
                    manufacturer: mfg,
                    series: "Custom".to_string(),
                    levels: draft.levels,
                    width_tenths_mm: (draft.width_mm * 10.0).round() as u16,
                    numbering_order: NumberingOrder::BottomToTop,
                    description_de: "Benutzerdefinierte Klemme".to_string(),
                    description_en: "User-defined terminal block".to_string(),
                    tiers,
                    accessories: None,
                };

                custom_specs.push(spec.clone());
                catalog.add_custom_spec(spec);
            }
        });

    ui.add_space(6.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(strings.filter_manufacturer);
        if ui
            .selectable_label(mfg_filter.is_none(), strings.all_manufacturers)
            .clicked()
        {
            *mfg_filter = None;
        }
        for mfg in catalog.manufacturers() {
            let is_sel = mfg_filter.as_deref() == Some(mfg);
            if logo_filter_button(ui, logos, mfg, is_sel, 24.0).clicked() {
                *mfg_filter = if is_sel { None } else { Some(mfg.to_string()) };
            }
        }

        ui.add_space(8.0);
        ui.add(
            egui::TextEdit::singleline(search_query)
                .hint_text(strings.search_hint)
                .desired_width(240.0),
        );
    });
    ui.add_space(8.0);

    let q = search_query.trim().to_lowercase();
    egui::ScrollArea::both().show(ui, |ui| {
        egui::Grid::new("terminal_db_grid")
            .striped(true)
            .spacing([14.0, 6.0])
            .show(ui, |ui| {
                ui.strong(strings.db_col_manufacturer);
                ui.strong(strings.db_col_name);
                ui.strong(strings.db_col_levels);
                ui.strong(strings.db_col_pattern);
                ui.strong(strings.db_col_width);
                ui.strong(strings.acc_end_plate);
                ui.strong(strings.db_col_desc);
                ui.end_row();

                for spec in catalog.all() {
                    let mfg_ok = mfg_filter
                        .as_deref()
                        .is_none_or(|m| spec.manufacturer == m);
                    let q_ok = q.is_empty()
                        || spec.name.to_lowercase().contains(&q)
                        || spec.manufacturer.to_lowercase().contains(&q)
                        || spec.series.to_lowercase().contains(&q)
                        || spec.pattern_summary().to_lowercase().contains(&q)
                        || spec.description_de.to_lowercase().contains(&q)
                        || spec.description_en.to_lowercase().contains(&q);
                    if !mfg_ok || !q_ok {
                        continue;
                    }

                    show_logo_badge(ui, logos, &spec.manufacturer, 18.0);
                    ui.label(&spec.name);
                    ui.label(spec.levels.to_string());
                    ui.monospace(spec.pattern_summary());
                    ui.label(format!("{:.1} mm", f32::from(spec.width_tenths_mm) / 10.0));
                    let acc = catalog.accessories_for(spec);
                    ui.label(acc.end_plate);
                    let desc = match lang {
                        Language::De => &spec.description_de,
                        _ => &spec.description_en,
                    };
                    ui.label(desc);
                    ui.end_row();
                }
            });
    });
}
