use egui::Ui;

use crate::catalog::accessories::all_end_brackets;
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
    pub jumper_slots: u8,
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
            jumper_slots: 2,
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

        // About, Authors, Open-Source Licenses, Update Checker, GitHub & BuyMeACoffee card
        egui::Frame::group(ui.style())
            .inner_margin(egui::Margin::same(12))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width().min(860.0));
                show_about_license_contents(ui, updater, strings, false);
            });
    });
}

/// Renders the dedicated Info & Open-Source Licenses modal dialog (`ℹ Info`), available in all builds
/// (Windows, macOS, Android).
pub fn show_about_modal(
    ctx: &egui::Context,
    open: &mut bool,
    updater: &UpdateChecker,
    strings: &Strings,
) {
    if !*open {
        return;
    }

    let screen_size = ctx.content_rect().size();
    let max_w = (screen_size.x - 24.0).max(280.0);
    let max_h = (screen_size.y - 40.0).max(240.0);
    let target_w = 560.0_f32.min(max_w);

    let mut close_clicked = false;
    egui::Window::new("ℹ Über TermiGrid – Info & Lizenzen")
        .open(open)
        .collapsible(false)
        .resizable(true)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .default_width(target_w)
        .min_width(300.0_f32.min(max_w))
        .max_width(max_w)
        .max_height(max_h)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                show_about_license_contents(ui, updater, strings, true);
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if crate::ui::theme::primary_button(ui, "Schließen / Close").clicked() {
                        close_clicked = true;
                    }
                });
            });
        });

    if close_clicked {
        *open = false;
    }
}

fn show_about_license_contents(
    ui: &mut Ui,
    updater: &UpdateChecker,
    strings: &Strings,
    compact: bool,
) {
    let sec = secondary_text_color(ui);

    ui.label(
        egui::RichText::new(format!("TermiGrid v{CURRENT_VERSION}"))
            .strong()
            .size(18.0),
    );
    ui.add(
        egui::Label::new(egui::RichText::new(strings.app_subtitle).color(sec)).wrap(),
    );
    ui.add_space(6.0);

    ui.group(|ui| {
        ui.label(egui::RichText::new("Entwicklung & Autoren").strong());
        ui.add_space(2.0);
        ui.add(
            egui::Label::new(
                "• Patrick Barbian — Projektinhaber, Elektrotechnik-Konzept & Entwicklung",
            )
            .wrap(),
        );
        ui.add(
            egui::Label::new(
                "• Antigravity (Google Deepmind / Gemini) — KI-Pair-Programming & Architektur",
            )
            .wrap(),
        );
        ui.add(
            egui::Label::new(egui::RichText::new(strings.about_vibecoded).color(sec).small())
                .wrap(),
        );
    });

    ui.add_space(6.0);
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
                    egui::RichText::new(format!("{}: {latest_tag}", strings.update_available))
                        .strong(),
                );
                if ui.button(strings.update_open_release).clicked() {
                    open_external_url(ui.ctx(), &release_url);
                }
            }
            UpdateStatus::Error(err) => {
                ui.add(egui::Label::new(format!("{}: {err}", strings.update_error)).wrap());
            }
        }
    });

    ui.add_space(8.0);
    ui.group(|ui| {
        ui.label(
            egui::RichText::new("Verwendete Open-Source-Komponenten & Lizenzen").strong(),
        );
        ui.add_space(2.0);
        let libs: [(&str, &str, &str); 9] = [
            ("TermiGrid", "MIT License", "© Patrick Barbian"),
            (
                "egui / eframe (0.37)",
                "MIT OR Apache-2.0",
                "Emil Ernerfeldt & egui contributors",
            ),
            (
                "serde / serde_json (1.0)",
                "MIT OR Apache-2.0",
                "Erick Tryzelaar, David Tolnay",
            ),
            (
                "rust_xlsxwriter (0.102)",
                "MIT OR Apache-2.0",
                "John McNamara",
            ),
            (
                "image (0.25)",
                "MIT OR Apache-2.0",
                "The image-rs Developers",
            ),
            ("rfd (0.18, Desktop)", "MIT", "PolyMeilex (Native File Dialogs)"),
            ("winit (0.30, Android)", "Apache-2.0", "The winit contributors"),
            ("jni (0.22, Android)", "MIT OR Apache-2.0", "jni-rs contributors"),
            ("winresource (0.1, Windows)", "MIT", "Maxime Chambreuil"),
        ];
        for (name, license, author) in libs {
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new(format!("• {name}")).strong().small());
                ui.label(
                    egui::RichText::new(format!("[{license}]"))
                        .color(egui::Color32::from_rgb(56, 189, 248))
                        .small(),
                );
                ui.add(
                    egui::Label::new(egui::RichText::new(author).color(sec).small()).wrap(),
                );
            });
        }

        ui.add_space(4.0);
        egui::CollapsingHeader::new("MIT-Lizenztext (TermiGrid)")
            .default_open(!compact)
            .show(ui, |ui| {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(
                            "Copyright (c) 2026 Patrick Barbian\n\n\
Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:\n\n\
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.\n\n\
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.",
                        )
                        .color(sec)
                        .small(),
                    )
                    .wrap(),
                );
            });
    });

    ui.add_space(6.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new(
                "Marken-Hinweis: WAGO (TOPJOB S), Phoenix Contact (CLIPLINE complete / PT / PTI) und Weidmüller (Klippon Connect) sind eingetragene Marken der jeweiligen Hersteller.",
            )
            .color(sec)
            .small(),
        )
        .wrap(),
    );
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
                ui.label(strings.custom_spec_chambers);
                ui.selectable_value(&mut draft.jumper_slots, 1, "1×");
                ui.selectable_value(&mut draft.jumper_slots, 2, "2×");
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
                    jumper_slots_per_tier: Some(draft.jumper_slots),
                    max_jumper_poles: Some(10),
                    article_number: None,
                    product_url: None,
                    image_url: None,
                    height_tenths_mm: None,
                    rail_offset_tenths_mm: None,
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
    let all_eb = all_end_brackets();
    egui::CollapsingHeader::new(format!("{} ({})", strings.end_bracket_db_heading, all_eb.len()))
        .default_open(false)
        .show(ui, |ui| {
            egui::Grid::new("end_bracket_db_grid")
                .striped(true)
                .spacing([14.0, 6.0])
                .show(ui, |ui| {
                    ui.strong(strings.db_col_manufacturer);
                    ui.strong(strings.db_col_name);
                    ui.strong("Art.-Nr.");
                    ui.strong(strings.db_col_width);
                    ui.strong(strings.db_col_height);
                    ui.strong(strings.db_col_desc);
                    ui.strong("");
                    ui.end_row();

                    for eb in all_eb {
                        let mfg_ok = mfg_filter
                            .as_deref()
                            .is_none_or(|m| eb.manufacturer == m);
                        let q_ok = q.is_empty()
                            || eb.name.to_lowercase().contains(&q)
                            || eb.manufacturer.to_lowercase().contains(&q)
                            || eb.article_number.to_lowercase().contains(&q)
                            || eb.mounting_de.to_lowercase().contains(&q)
                            || eb.mounting_en.to_lowercase().contains(&q);
                        if !mfg_ok || !q_ok {
                            continue;
                        }
                        show_logo_badge(ui, logos, eb.manufacturer, 18.0);
                        ui.label(eb.name);
                        ui.monospace(eb.article_number);
                        ui.label(format!("{:.1} mm", f32::from(eb.width_tenths_mm) / 10.0));
                        ui.label(format!("{:.1} mm", f32::from(eb.height_tenths_mm) / 10.0));
                        let desc = match lang {
                            Language::De => eb.mounting_de,
                            _ => eb.mounting_en,
                        };
                        ui.label(desc);
                        if ui.small_button("Web ↗").clicked() {
                            open_external_url(ui.ctx(), eb.product_url);
                        }
                        ui.end_row();
                    }
                });
        });
    ui.add_space(4.0);

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
                ui.strong(strings.db_col_height);
                ui.strong(strings.acc_end_plate);
                ui.strong(strings.custom_spec_chambers);
                ui.strong(strings.db_col_desc);
                ui.strong("");
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
                        || spec.description_en.to_lowercase().contains(&q)
                        || spec
                            .effective_article_number()
                            .is_some_and(|a| a.to_lowercase().contains(&q));
                    if !mfg_ok || !q_ok {
                        continue;
                    }

                    show_logo_badge(ui, logos, &spec.manufacturer, 18.0);
                    ui.label(&spec.name);
                    ui.label(spec.levels.to_string());
                    ui.monospace(spec.pattern_summary());
                    ui.label(format!("{:.1} mm", f32::from(spec.width_tenths_mm) / 10.0));
                    ui.label(format!(
                        "{:.1} mm",
                        f32::from(spec.effective_height_tenths_mm()) / 10.0
                    ));
                    let acc = catalog.accessories_for(spec);
                    ui.label(acc.end_plate);
                    ui.label(format!("{}×", spec.effective_jumper_slots()));
                    let desc = match lang {
                        Language::De => &spec.description_de,
                        _ => &spec.description_en,
                    };
                    ui.label(desc);
                    if ui.small_button("Web ↗").clicked() {
                        open_external_url(ui.ctx(), &spec.effective_product_url());
                    }
                    ui.end_row();
                }
            });
    });
}
