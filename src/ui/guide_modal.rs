use egui::Ui;

use crate::catalog::accessories::{all_end_brackets, TerminalAccessories};
use crate::catalog::{TerminalCatalog, TerminalSpec, TierSpec};
use crate::i18n::{tr, Language, Strings};
use crate::model::{NumberingOrder, TierBehavior, TierColor};
use crate::ui::guide_content::{
    all_guide_articles, build_ai_terminal_prompt, search_guide_articles, GuideCategory,
    GuideCodeKind, GuideSearchState,
};
use crate::ui::icons::{
    danger_icon_button, draw_icon, ghost_icon_button, icon_button, icon_only_button,
    primary_icon_button, show_responsive_modal, success_icon_button, UiIcon,
};
use crate::ui::logos::{logo_filter_button, show_logo_badge, BrandLogos};
use crate::ui::theme::{pill_badge, recessed_card, secondary_text_color, section_card, tokens};
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
    pub json_import_buf: String,
    pub json_import_error: Option<String>,
    pub json_example_copied: bool,
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
            json_import_buf: String::new(),
            json_import_error: None,
            json_example_copied: false,
        }
    }
}

/// Generates a complete, schema-exact example terminal JSON serialized dynamically from a real
/// `TerminalSpec` struct (`wago-2002-3217` 3-level terminal) including accessories and dimensions.
pub fn build_example_terminal_json(catalog: &TerminalCatalog) -> String {
    let base = catalog
        .find_by_id("wago-2002-3217")
        .or_else(|| catalog.find_by_id("wago-2002-3209"))
        .or_else(|| catalog.all().first());

    if let Some(spec) = base {
        let mut example = spec.clone();
        example.id = "custom-beispiel-2002-3217".to_string();
        example.accessories = Some(catalog.accessories_for(spec));
        example.jumper_slots_per_tier = Some(spec.effective_jumper_slots());
        example.max_jumper_poles = Some(spec.effective_max_jumper_poles());
        example.article_number = spec.effective_article_number().map(str::to_string);
        example.product_url = Some(spec.effective_product_url());
        example.image_url = spec.effective_image_url();
        example.height_tenths_mm = Some(spec.effective_height_tenths_mm());
        example.rail_offset_tenths_mm = Some(spec.effective_rail_offset_tenths_mm());
        example.user_modified = false;
        if let Ok(pretty) = serde_json::to_string_pretty(&example) {
            return pretty;
        }
    }
    String::from("{}")
}

/// Parses either a single `TerminalSpec` JSON object `{ ... }` or an array `[{ ... }, ...]`
/// cleanly, stripping optional UTF-8 BOM or markdown code fences and reporting the exact schema
/// error for whichever JSON shape was provided.
pub fn parse_terminal_specs_json(raw: &str) -> Result<Vec<TerminalSpec>, String> {
    let mut trimmed = raw.trim().trim_start_matches('\u{feff}').trim();
    if let Some(rest) = trimmed.strip_prefix("```") {
        let without_lang = rest
            .strip_prefix("json")
            .or_else(|| rest.strip_prefix("JSON"))
            .unwrap_or(rest)
            .trim_start_matches(['\r', '\n']);
        if let Some(end_idx) = without_lang.rfind("```") {
            trimmed = without_lang[..end_idx].trim();
        }
    }

    if trimmed.is_empty() {
        return Err("Empty JSON input".to_string());
    }

    let mut specs = if trimmed.starts_with('{') {
        let single = serde_json::from_str::<TerminalSpec>(trimmed).map_err(|e| e.to_string())?;
        vec![single]
    } else if trimmed.starts_with('[') {
        serde_json::from_str::<Vec<TerminalSpec>>(trimmed).map_err(|e| e.to_string())?
    } else {
        match serde_json::from_str::<Vec<TerminalSpec>>(trimmed) {
            Ok(arr) => arr,
            Err(e_arr) => serde_json::from_str::<TerminalSpec>(trimmed)
                .map(|single| vec![single])
                .map_err(|e_obj| format!("{e_obj} / {e_arr}"))?,
        }
    };

    if specs.is_empty() {
        return Err("Empty JSON array".to_string());
    }

    for (idx, spec) in specs.iter_mut().enumerate() {
        if spec.name.trim().is_empty() {
            return Err(format!("Entry {}: missing non-empty \"name\"", idx + 1));
        }
        if spec.levels == 0 {
            spec.levels = spec.tiers.len().max(1);
        }
        if spec.id.trim().is_empty() {
            let slug: String = spec
                .name
                .to_lowercase()
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
                .collect();
            spec.id = format!("custom-{}", slug.trim_matches('-'));
        }
    }

    Ok(specs)
}

/// Renders the searchable "Anleitung" (Instructions) tab covering every feature of TermiGrid,
/// including local terminal integration and a copyable AI JSON prompt generator.
pub fn show_guide_tab(
    ui: &mut Ui,
    catalog: &TerminalCatalog,
    guide_state: &mut GuideSearchState,
    updater: &UpdateChecker,
    strings: &Strings,
) {
    let lang = strings.lang;
    let de = lang == Language::De;
    let t = tokens(ui);
    let max_w = ui.available_width().min(960.0);
    let total_articles = all_guide_articles().len();
    let matches = search_guide_articles(lang, guide_state.category, &guide_state.query);

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            ui.heading(strings.guide_title);
            pill_badge(
                ui,
                &format!("{} / {total_articles}", matches.len()),
                t.text_secondary,
                t.bg_recessed,
            );
        });
        ui.add_space(6.0);

        // Search & Filter Bar
        section_card(ui, |ui| {
            ui.set_max_width(max_w);
            ui.horizontal_wrapped(|ui| {
                let (sr, _) = ui.allocate_exact_size(egui::Vec2::splat(16.0), egui::Sense::hover());
                draw_icon(ui.painter(), sr, UiIcon::Search, t.text_muted);

                let hint = if de {
                    "Anleitung durchsuchen (z.B. JSON, KI-Prompt, EPLAN, Leeres Schild, Brücken, Shortcut)..."
                } else {
                    "Search manual (e.g. JSON, AI prompt, EPLAN, blank marker, bridges, shortcut)..."
                };
                let search_w = (ui.available_width() - 140.0).clamp(180.0, 520.0);
                let dark = ui.ctx().theme() == egui::Theme::Dark;
                let input_bg = if dark {
                    t.bg_recessed
                } else {
                    egui::Color32::WHITE
                };
                egui::Frame::NONE
                    .fill(input_bg)
                    .stroke(egui::Stroke::new(1.0, t.border_strong))
                    .corner_radius(egui::CornerRadius::same(6))
                    .inner_margin(egui::Margin::symmetric(8, 4))
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut guide_state.query)
                                .frame(egui::Frame::NONE)
                                .hint_text(hint)
                                .desired_width(search_w),
                        );
                    });

                if !guide_state.query.is_empty()
                    && icon_button(ui, UiIcon::Clear, tr::clear_search(lang)).clicked()
                {
                    guide_state.query.clear();
                }
            });

            ui.add_space(6.0);
            // Category filter chips
            ui.horizontal_wrapped(|ui| {
                for cat in GuideCategory::ALL {
                    let active = guide_state.category == cat;
                    if ui.selectable_label(active, cat.label(lang)).clicked() {
                        guide_state.category = cat;
                    }
                }
            });

            ui.add_space(4.0);
            // Quick-search tag pills
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new(if de { "Schnellsuche:" } else { "Quick tags:" })
                        .size(11.5)
                        .color(t.text_muted),
                );
                for tag in [
                    "KI-Prompt",
                    "JSON",
                    "custom_specs.json",
                    "Leeres Schild",
                    "Quick-Prompt",
                    "Brücken",
                    "EPLAN",
                    "Shortcut",
                    "Update",
                ] {
                    if ui
                        .small_button(tag)
                        .clicked()
                    {
                        guide_state.category = GuideCategory::All;
                        guide_state.query = tag.to_string();
                    }
                }
            });
        });

        ui.add_space(8.0);

        if matches.is_empty() {
            section_card(ui, |ui| {
                ui.set_max_width(max_w);
                ui.label(
                    egui::RichText::new(if de {
                        "Keine passenden Themen gefunden. Leeren Sie die Suche oder wählen Sie „Alle Themen“."
                    } else {
                        "No matching topics found. Clear the search or select 'All Topics'."
                    })
                    .color(t.text_secondary),
                );
            });
            ui.add_space(8.0);
        }

        for article in matches {
            section_card(ui, |ui| {
                ui.set_max_width(max_w);
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        egui::RichText::new(article.title(lang))
                            .strong()
                            .size(15.5)
                            .color(t.text_primary),
                    );
                    pill_badge(
                        ui,
                        article.category.label(lang),
                        t.accent_text,
                        t.accent_soft,
                    );
                });
                ui.add_space(3.0);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(article.summary(lang))
                            .size(12.5)
                            .color(t.text_secondary),
                    )
                    .wrap(),
                );
                ui.add_space(6.0);

                for (sub_title, body) in article.steps(lang) {
                    ui.label(
                        egui::RichText::new(*sub_title)
                            .strong()
                            .size(13.0)
                            .color(t.text_primary),
                    );
                    ui.add(egui::Label::new(*body).wrap());
                    ui.add_space(5.0);
                }

                if let Some(code_kind) = article.code {
                    let (code_title, code_text): (&str, String) = match code_kind {
                        GuideCodeKind::Static {
                            title_de,
                            title_en,
                            body_de,
                            body_en,
                        } => (
                            if de { title_de } else { title_en },
                            if de { body_de } else { body_en }.to_string(),
                        ),
                        GuideCodeKind::AiTerminalPrompt => (
                            if de {
                                "Kopierfertiger KI-Prompt (ChatGPT / Claude / Gemini)"
                            } else {
                                "Copy-ready AI Prompt (ChatGPT / Claude / Gemini)"
                            },
                            build_ai_terminal_prompt(lang, catalog),
                        ),
                        GuideCodeKind::ExampleTerminalJson => (
                            if de {
                                "Vollständiges Beispiel-JSON einer Klemme (als Array-Vorlage)"
                            } else {
                                "Complete Example Terminal JSON (Array Template)"
                            },
                            format!("[\n{}\n]", build_example_terminal_json(catalog)),
                        ),
                    };

                    recessed_card(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(code_title)
                                    .strong()
                                    .size(12.0)
                                    .color(t.text_primary),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let is_copied = guide_state
                                        .copied_feedback_id
                                        .as_deref()
                                        == Some(article.id);
                                    let btn_lbl = if is_copied {
                                        if de { "Kopiert!" } else { "Copied!" }
                                    } else if de {
                                        "Kopieren"
                                    } else {
                                        "Copy"
                                    };
                                    let clicked = if is_copied {
                                        success_icon_button(ui, UiIcon::Check, btn_lbl).clicked()
                                    } else {
                                        primary_icon_button(ui, UiIcon::Duplicate, btn_lbl)
                                            .clicked()
                                    };
                                    if clicked {
                                        ui.ctx().copy_text(code_text.clone());
                                        guide_state.copied_feedback_id =
                                            Some(article.id.to_string());
                                    }
                                },
                            );
                        });
                        ui.add_space(4.0);
                        let mut preview = code_text;
                        egui::ScrollArea::vertical()
                            .id_salt(format!("guide_code_{}", article.id))
                            .max_height(210.0)
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut preview)
                                        .font(egui::TextStyle::Monospace)
                                        .desired_width(ui.available_width())
                                        .desired_rows(8)
                                        .interactive(true),
                                );
                            });
                    });
                }
            });
            ui.add_space(8.0);
        }

        // About, Authors, Open-Source Licenses, Update Checker, GitHub & BuyMeACoffee card
        section_card(ui, |ui| {
            ui.set_max_width(max_w);
            show_about_license_contents(ui, updater, strings, false);
        });
    });
}

/// Renders the dedicated Info & Open-Source Licenses modal dialog, available in all builds
/// (Windows, macOS, Android) with responsive viewport clamping and sticky header/footer.
pub fn show_about_modal(
    ctx: &egui::Context,
    open: &mut bool,
    updater: &UpdateChecker,
    strings: &Strings,
) {
    let mut unit = ();
    show_responsive_modal(
        ctx,
        "about_termi_modal",
        UiIcon::Info,
        tr::about_guide_title(strings.lang),
        580.0,
        open,
        &mut unit,
        |ui, ()| {
            show_about_license_contents(ui, updater, strings, true);
        },
        |ui, (), close_requested| {
            if primary_icon_button(ui, UiIcon::Close, tr::close(strings.lang)).clicked() {
                *close_requested = true;
            }
        },
    );
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

    section_card(ui, |ui| {
        ui.label(egui::RichText::new("Entwicklung & Autoren").strong());
        ui.add_space(2.0);
        ui.add(
            egui::Label::new(tr::about_author_line(strings.lang)).wrap(),
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
        if icon_button(ui, UiIcon::GitHub, strings.btn_github).clicked() {
            open_external_url(ui.ctx(), GITHUB_REPO_URL);
        }
        if icon_button(ui, UiIcon::Coffee, strings.btn_coffee).clicked() {
            open_external_url(ui.ctx(), BUYMEACOFFEE_URL);
        }
        if icon_button(ui, UiIcon::RefreshCheck, strings.btn_check_update).clicked() {
            updater.check_async(ui.ctx().clone());
        }

        match updater.status() {
            UpdateStatus::Idle => {}
            UpdateStatus::Checking => {
                ui.label(strings.update_checking);
            }
            UpdateStatus::UpToDate(tag) => {
                ui.horizontal(|ui| {
                    let (cr, _) =
                        ui.allocate_exact_size(egui::Vec2::splat(14.0), egui::Sense::hover());
                    draw_icon(
                        ui.painter(),
                        cr,
                        UiIcon::Check,
                        egui::Color32::from_rgb(22, 163, 74),
                    );
                    ui.label(format!("{} ({tag})", strings.update_up_to_date));
                });
            }
            UpdateStatus::UpdateAvailable {
                latest_tag,
                release_url,
            } => {
                ui.label(
                    egui::RichText::new(format!("{}: {latest_tag}", strings.update_available))
                        .strong(),
                );
                if primary_icon_button(ui, UiIcon::DownloadUpdate, strings.update_open_release)
                    .clicked()
                {
                    open_external_url(ui.ctx(), &release_url);
                }
            }
            UpdateStatus::Error(err) => {
                ui.add(egui::Label::new(format!("{}: {err}", strings.update_error)).wrap());
            }
        }
    });

    ui.add_space(8.0);
    section_card(ui, |ui| {
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
            egui::RichText::new(tr::about_brand_notice(strings.lang))
                .color(sec)
                .small(),
        )
        .wrap(),
    );
}

/// Action emitted by the Terminal Database tab or the manual Spec Editor modal.
#[derive(Debug, Clone)]
pub enum DatabaseAction {
    /// Save a newly created or manually edited specification (persists PC-wide & in active project).
    SaveSpec(Box<TerminalSpec>),
    /// Batch-import multiple terminal specifications from a pasted JSON array/object.
    ImportSpecsBatch(Vec<TerminalSpec>),
    /// Reset an edited built-in spec to factory defaults or delete a user-created custom spec.
    ResetOrDeleteSpec { id: String, is_builtin: bool },
}

/// Modal editor state for manually modifying any terminal block specification at runtime.
#[derive(Debug, Clone, Default)]
pub struct SpecEditorModalState {
    pub open: bool,
    pub spec_id: String,
    pub is_builtin: bool,
    pub was_modified: bool,
    pub manufacturer: String,
    pub name: String,
    pub series: String,
    pub article_number: String,
    pub levels: usize,
    pub width_mm: f32,
    pub height_mm: f32,
    pub rail_offset_mm: f32,
    pub jumper_slots: u8,
    pub max_jumper_poles: u8,
    pub numbering_order: NumberingOrder,
    pub description_de: String,
    pub description_en: String,
    pub product_url: String,
    pub image_url: String,
    pub tier_kinds: [CustomTierKind; 4],
    pub tier_fixed: [String; 4],
    pub tier_colors: [TierColor; 4],
    pub preserved_accessories: Option<TerminalAccessories>,
}

impl SpecEditorModalState {
    /// Opens the editor pre-populated with all current effective properties of `spec`.
    pub fn open_for(&mut self, spec: &TerminalSpec, is_builtin: bool, was_modified: bool) {
        let mut tier_kinds = [CustomTierKind::Sequential; 4];
        let mut tier_fixed = [String::new(), String::new(), String::new(), String::new()];
        let mut tier_colors = [TierColor::StandardGray; 4];

        for (i, tier) in spec.tiers.iter().take(4).enumerate() {
            tier_colors[i] = tier.color;
            match &tier.behavior {
                TierBehavior::Sequential => {
                    tier_kinds[i] = CustomTierKind::Sequential;
                }
                TierBehavior::ProtectiveEarth => {
                    tier_kinds[i] = CustomTierKind::ProtectiveEarth;
                }
                TierBehavior::Fixed(txt) => {
                    tier_kinds[i] = CustomTierKind::Fixed;
                    tier_fixed[i] = txt.clone();
                }
                TierBehavior::Unlabeled => {
                    tier_kinds[i] = CustomTierKind::Fixed;
                    tier_fixed[i] = String::new();
                }
            }
        }

        self.open = true;
        self.spec_id = spec.id.clone();
        self.is_builtin = is_builtin;
        self.was_modified = was_modified;
        self.manufacturer = spec.manufacturer.clone();
        self.name = spec.name.clone();
        self.series = spec.series.clone();
        self.article_number = spec
            .effective_article_number()
            .unwrap_or_default()
            .to_string();
        self.levels = spec.levels.clamp(1, 4);
        self.width_mm = f32::from(spec.width_tenths_mm) / 10.0;
        self.height_mm = f32::from(spec.effective_height_tenths_mm()) / 10.0;
        self.rail_offset_mm = f32::from(spec.effective_rail_offset_tenths_mm()) / 10.0;
        self.jumper_slots = spec.effective_jumper_slots();
        self.max_jumper_poles = spec.effective_max_jumper_poles();
        self.numbering_order = spec.numbering_order;
        self.description_de = spec.description_de.clone();
        self.description_en = spec.description_en.clone();
        self.product_url = spec.effective_product_url();
        self.image_url = spec.effective_image_url().unwrap_or_default();
        self.tier_kinds = tier_kinds;
        self.tier_fixed = tier_fixed;
        self.tier_colors = tier_colors;
        self.preserved_accessories = spec.accessories.clone();
    }

    fn build_spec(&self) -> TerminalSpec {
        let levels = self.levels.clamp(1, 4);
        let tiers: Vec<TierSpec> = (0..levels)
            .map(|lvl| {
                let behavior = match self.tier_kinds[lvl] {
                    CustomTierKind::Sequential => TierBehavior::Sequential,
                    CustomTierKind::ProtectiveEarth => TierBehavior::ProtectiveEarth,
                    CustomTierKind::Fixed => {
                        TierBehavior::Fixed(self.tier_fixed[lvl].trim().to_string())
                    }
                };
                TierSpec {
                    behavior,
                    color: self.tier_colors[lvl],
                }
            })
            .collect();

        let trim_opt = |s: &str| {
            let t = s.trim();
            if t.is_empty() {
                None
            } else {
                Some(t.to_string())
            }
        };

        let mfg = if self.manufacturer.trim().is_empty() {
            "Custom".to_string()
        } else {
            self.manufacturer.trim().to_string()
        };
        let height_tenths = (self.height_mm.clamp(10.0, 250.0) * 10.0).round() as u16;
        let offset_tenths = (self.rail_offset_mm.clamp(-50.0, 50.0) * 10.0).round() as i16;

        TerminalSpec {
            id: self.spec_id.clone(),
            name: self.name.trim().to_string(),
            manufacturer: mfg,
            series: if self.series.trim().is_empty() {
                "Custom".to_string()
            } else {
                self.series.trim().to_string()
            },
            levels,
            width_tenths_mm: (self.width_mm.clamp(2.0, 100.0) * 10.0).round() as u16,
            numbering_order: self.numbering_order,
            description_de: self.description_de.trim().to_string(),
            description_en: self.description_en.trim().to_string(),
            tiers,
            accessories: self.preserved_accessories.clone(),
            jumper_slots_per_tier: Some(self.jumper_slots.min(2)),
            max_jumper_poles: Some(self.max_jumper_poles),
            article_number: trim_opt(&self.article_number),
            product_url: trim_opt(&self.product_url),
            image_url: trim_opt(&self.image_url),
            height_tenths_mm: Some(height_tenths),
            rail_offset_tenths_mm: Some(offset_tenths),
            user_modified: self.is_builtin,
        }
    }
}

/// Renders the searchable terminal database table ("Klemmen-Datenbank") with manual editing support.
#[allow(clippy::too_many_arguments)]
pub fn show_database_tab(
    ui: &mut Ui,
    catalog: &TerminalCatalog,
    logos: &BrandLogos,
    custom_specs: &[TerminalSpec],
    mfg_filter: &mut Option<String>,
    search_query: &mut String,
    draft: &mut CustomSpecDraft,
    editor_modal: &mut SpecEditorModalState,
    strings: &Strings,
    lang: Language,
) -> Option<DatabaseAction> {
    let mut action = None;
    let tok = tokens(ui);

    ui.add_space(6.0);
    ui.horizontal_wrapped(|ui| {
        ui.heading(format!("{} ({})", strings.db_heading, catalog.all().len()));
        let edited_count = catalog.all().iter().filter(|s| s.user_modified).count();
        let custom_count = catalog
            .all()
            .iter()
            .filter(|s| !s.user_modified && s.id.starts_with("custom-"))
            .count();
        if edited_count > 0 || custom_count > 0 {
            let badge_txt = tr::db_custom_badge(lang, edited_count, custom_count);
            pill_badge(ui, &badge_txt, tok.accent_text, tok.accent_soft);
        }
        ui.label(
            egui::RichText::new(tr::db_edit_hint(lang))
                .color(secondary_text_color(ui))
                .small(),
        );
    });
    ui.add_space(6.0);

    egui::CollapsingHeader::new(strings.custom_spec_heading)
        .default_open(false)
        .show(ui, |ui| {
            let form_w = ui.available_width();
            ui.horizontal_wrapped(|ui| {
                ui.label(strings.db_col_manufacturer);
                ui.add(
                    egui::TextEdit::singleline(&mut draft.manufacturer)
                        .desired_width((form_w * 0.22).clamp(90.0, 140.0)),
                );
                ui.label(strings.db_col_name);
                ui.add(
                    egui::TextEdit::singleline(&mut draft.name)
                        .desired_width((form_w * 0.28).clamp(110.0, 180.0)),
                );
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
                ui.horizontal_wrapped(|ui| {
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

            if success_icon_button(ui, UiIcon::Add, strings.custom_spec_add).clicked()
                && !draft.name.trim().is_empty()
            {
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
                let mut seq = custom_specs.len() + 1;
                let mut id = format!("custom-{slug}-{seq}");
                while catalog.find_by_id(&id).is_some()
                    || custom_specs.iter().any(|s| s.id == id)
                {
                    seq += 1;
                    id = format!("custom-{slug}-{seq}");
                }
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
                    user_modified: false,
                };

                action = Some(DatabaseAction::SaveSpec(Box::new(spec)));
            }
        });

    ui.add_space(4.0);
    egui::CollapsingHeader::new(tr::json_batch_heading(lang))
        .default_open(false)
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(tr::json_batch_hint(lang))
                    .color(secondary_text_color(ui))
                    .small(),
            );
            ui.add_space(4.0);
            egui::Frame::NONE
                .fill(tok.bg_recessed)
                .stroke(egui::Stroke::new(1.0, tok.border_hairline))
                .corner_radius(egui::CornerRadius::same(6))
                .inner_margin(egui::Margin::same(6))
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("json_batch_import_scroll")
                        .max_height(260.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let resp = ui.add(
                                egui::TextEdit::multiline(&mut draft.json_import_buf)
                                    .font(egui::TextStyle::Monospace)
                                    .desired_rows(10)
                                    .desired_width(f32::INFINITY)
                                    .frame(egui::Frame::NONE)
                                    .hint_text("{\n  \"id\": \"custom-beispiel-2002-3217\",\n  \"name\": \"2002-3217 (2,5mm² Dreistockklemme PE/N/L)\",\n  \"manufacturer\": \"WAGO\",\n  ...\n}"),
                            );
                            if resp.changed() {
                                draft.json_import_error = None;
                            }
                        });
                });
            if let Some(err) = &draft.json_import_error {
                ui.add_space(2.0);
                ui.label(
                    egui::RichText::new(err)
                        .color(egui::Color32::from_rgb(239, 68, 68))
                        .small(),
                );
            }
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                let can_import = !draft.json_import_buf.trim().is_empty();
                if ui
                    .add_enabled_ui(can_import, |ui| {
                        success_icon_button(ui, UiIcon::Add, tr::json_batch_btn(lang))
                    })
                    .inner
                    .clicked()
                {
                    match parse_terminal_specs_json(&draft.json_import_buf) {
                        Ok(specs) => {
                            draft.json_import_error = None;
                            draft.json_import_buf.clear();
                            draft.json_example_copied = false;
                            action = Some(DatabaseAction::ImportSpecsBatch(specs));
                        }
                        Err(err) => {
                            draft.json_import_error = Some(tr::json_batch_err(lang, &err));
                        }
                    }
                }

                ui.separator();

                if icon_button(ui, UiIcon::Duplicate, tr::json_example_copy_btn(lang)).clicked() {
                    let example_json = build_example_terminal_json(catalog);
                    ui.ctx().copy_text(example_json);
                    draft.json_example_copied = true;
                }
                if ghost_icon_button(ui, UiIcon::Apply, tr::json_example_insert_btn(lang)).clicked()
                {
                    draft.json_import_buf = build_example_terminal_json(catalog);
                    draft.json_import_error = None;
                    draft.json_example_copied = false;
                }
                if draft.json_example_copied {
                    ui.label(
                        egui::RichText::new(tr::json_example_copied_hint(lang))
                            .color(tok.success)
                            .small()
                            .strong(),
                    );
                }
            });

            ui.add_space(4.0);
            egui::CollapsingHeader::new(tr::json_example_preview_heading(lang))
                .id_salt("json_example_preview_collapse")
                .default_open(false)
                .show(ui, |ui| {
                    let mut preview = build_example_terminal_json(catalog);
                    egui::Frame::NONE
                        .fill(tok.bg_recessed)
                        .stroke(egui::Stroke::new(1.0, tok.border_hairline))
                        .corner_radius(egui::CornerRadius::same(6))
                        .inner_margin(egui::Margin::same(6))
                        .show(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .id_salt("json_example_preview_scroll")
                                .max_height(220.0)
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    ui.add(
                                        egui::TextEdit::multiline(&mut preview)
                                            .font(egui::TextStyle::Monospace)
                                            .desired_width(f32::INFINITY)
                                            .frame(egui::Frame::NONE)
                                            .interactive(true),
                                    );
                                });
                        });
                });
        });

    ui.add_space(6.0);
    let q = search_query.trim().to_lowercase();
    let global_active = !q.is_empty();

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
        let search_w = (ui.available_width() - 56.0).clamp(130.0, 260.0);
        let (sr, _) = ui.allocate_exact_size(egui::Vec2::splat(16.0), egui::Sense::hover());
        draw_icon(ui.painter(), sr, UiIcon::Search, secondary_text_color(ui));
        ui.add(
            egui::TextEdit::singleline(search_query)
                .hint_text(strings.search_hint)
                .desired_width(search_w),
        );
        if !search_query.is_empty()
            && icon_only_button(ui, UiIcon::Clear, false, tr::clear_search(lang)).clicked()
        {
            search_query.clear();
        }
        if global_active && mfg_filter.is_some() {
            ui.label(
                egui::RichText::new(tr::db_global_search(lang))
                    .color(egui::Color32::from_rgb(14, 165, 233))
                    .small()
                    .strong(),
            );
        }
    });
    ui.add_space(8.0);

    let all_eb = all_end_brackets();
    egui::CollapsingHeader::new(format!("{} ({})", strings.end_bracket_db_heading, all_eb.len()))
        .default_open(false)
        .show(ui, |ui| {
            egui::ScrollArea::horizontal()
                .id_salt("end_bracket_db_hscroll")
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
                                let mfg_ok = global_active
                                    || mfg_filter
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
                                ui.label(format!(
                                    "{:.1} mm",
                                    f32::from(eb.width_tenths_mm) / 10.0
                                ));
                                ui.label(format!(
                                    "{:.1} mm",
                                    f32::from(eb.height_tenths_mm) / 10.0
                                ));
                                let desc = match lang {
                                    Language::De => eb.mounting_de,
                                    _ => eb.mounting_en,
                                };
                                ui.label(desc);
                                if icon_button(ui, UiIcon::ExternalLink, "Web").clicked() {
                                    open_external_url(ui.ctx(), eb.product_url);
                                }
                                ui.end_row();
                            }
                        });
                });
        });
    ui.add_space(4.0);

    let filtered_specs = catalog.filter_specs(mfg_filter.as_deref(), search_query);
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
                ui.strong(tr::actions_col(lang));
                ui.end_row();

                for spec in filtered_specs {
                    show_logo_badge(ui, logos, &spec.manufacturer, 18.0);
                    ui.horizontal(|ui| {
                        ui.label(&spec.name);
                        if spec.user_modified {
                            pill_badge(
                                ui,
                                tr::badge_modified(lang),
                                tok.accent_text,
                                tok.accent_soft,
                            );
                        } else if spec.id.starts_with("custom-") {
                            pill_badge(ui, tr::badge_custom(lang), tok.success, tok.success_soft);
                        }
                    });
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
                    ui.horizontal(|ui| {
                        if icon_button(ui, UiIcon::Edit, tr::edit(lang)).clicked() {
                            let is_builtin =
                                TerminalCatalog::embedded_default_by_id(&spec.id).is_some();
                            let was_modified = spec.user_modified
                                || custom_specs.iter().any(|c| c.id == spec.id);
                            editor_modal.open_for(spec, is_builtin, was_modified);
                        }
                        if icon_only_button(
                            ui,
                            UiIcon::ExternalLink,
                            false,
                            tr::open_in_browser_tip(lang),
                        )
                        .clicked()
                        {
                            open_external_url(ui.ctx(), &spec.effective_product_url());
                        }
                    });
                    ui.end_row();
                }
            });
    });

    action
}

/// Renders the modal dialog for manually editing any terminal specification without recompiling.
pub fn show_spec_editor_modal(
    ctx: &egui::Context,
    state: &mut SpecEditorModalState,
    strings: &Strings,
) -> Option<DatabaseAction> {
    let mut action: Option<DatabaseAction> = None;
    let mut open = state.open;
    let lang = strings.lang;
    let title = tr::spec_editor_title(lang, &state.name);

    show_responsive_modal(
        ctx,
        "spec_editor_modal",
        UiIcon::Edit,
        &title,
        620.0,
        &mut open,
        state,
        |ui, st| {
            let sec = secondary_text_color(ui);
            ui.label(
                egui::RichText::new(tr::spec_editor_subtitle(lang, &st.spec_id))
                    .color(sec)
                    .small(),
            );
            ui.add_space(6.0);

            section_card(ui, |ui| {
                ui.strong(tr::spec_master_data(lang));
                ui.add_space(4.0);
                egui::Grid::new("spec_edit_meta_grid")
                    .num_columns(2)
                    .spacing([12.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(tr::spec_label_name(lang));
                        ui.add(
                            egui::TextEdit::singleline(&mut st.name)
                                .desired_width(ui.available_width().max(200.0)),
                        );
                        ui.end_row();

                        ui.label(tr::spec_label_mfg(lang));
                        ui.add(
                            egui::TextEdit::singleline(&mut st.manufacturer)
                                .desired_width(ui.available_width().max(200.0)),
                        );
                        ui.end_row();

                        ui.label(tr::spec_label_series(lang));
                        ui.add(
                            egui::TextEdit::singleline(&mut st.series)
                                .desired_width(ui.available_width().max(200.0)),
                        );
                        ui.end_row();

                        ui.label(tr::spec_label_art(lang));
                        ui.add(
                            egui::TextEdit::singleline(&mut st.article_number)
                                .hint_text("3048467")
                                .desired_width(180.0),
                        );
                        ui.end_row();

                        ui.label(tr::spec_label_desc_de(lang));
                        ui.add(
                            egui::TextEdit::singleline(&mut st.description_de)
                                .desired_width(ui.available_width().max(200.0)),
                        );
                        ui.end_row();

                        ui.label(tr::spec_label_desc_en(lang));
                        ui.add(
                            egui::TextEdit::singleline(&mut st.description_en)
                                .desired_width(ui.available_width().max(200.0)),
                        );
                        ui.end_row();
                    });
            });

            ui.add_space(6.0);
            section_card(ui, |ui| {
                ui.strong(tr::spec_geo_heading(lang));
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(tr::spec_levels(lang));
                    egui::ComboBox::from_id_salt("spec_edit_levels")
                        .selected_text(format!("{}", st.levels))
                        .show_ui(ui, |ui| {
                            for l in 1..=4 {
                                ui.selectable_value(&mut st.levels, l, format!("{l}"));
                            }
                        });

                    ui.add_space(8.0);
                    ui.label(tr::spec_width(lang));
                    ui.add(
                        egui::DragValue::new(&mut st.width_mm)
                            .speed(0.1)
                            .range(2.0..=100.0)
                            .suffix(" mm"),
                    );

                    ui.add_space(8.0);
                    ui.label(tr::spec_height(lang));
                    ui.add(
                        egui::DragValue::new(&mut st.height_mm)
                            .speed(0.5)
                            .range(10.0..=250.0)
                            .suffix(" mm"),
                    );

                    ui.add_space(8.0);
                    ui.label(tr::spec_rail_offset(lang));
                    ui.add(
                        egui::DragValue::new(&mut st.rail_offset_mm)
                            .speed(0.5)
                            .range(-50.0..=50.0)
                            .suffix(" mm"),
                    );
                });

                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(tr::spec_jumpers_per_tier(lang));
                    egui::ComboBox::from_id_salt("spec_edit_jumpers")
                        .selected_text(format!("{}×", st.jumper_slots))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut st.jumper_slots, 0, tr::spec_jumper_0(lang));
                            ui.selectable_value(&mut st.jumper_slots, 1, tr::spec_jumper_1(lang));
                            ui.selectable_value(&mut st.jumper_slots, 2, tr::spec_jumper_2(lang));
                        });

                    ui.add_space(8.0);
                    ui.label(tr::spec_max_poles(lang));
                    ui.add(
                        egui::DragValue::new(&mut st.max_jumper_poles)
                            .speed(1)
                            .range(0..=30)
                            .suffix(tr::spec_poles_suffix(lang)),
                    );

                    ui.add_space(8.0);
                    ui.label(tr::spec_num_order(lang));
                    egui::ComboBox::from_id_salt("spec_edit_num_order")
                        .selected_text(match st.numbering_order {
                            NumberingOrder::BottomToTop => tr::spec_bottom_to_top(lang),
                            NumberingOrder::TopToBottom => tr::spec_top_to_bottom(lang),
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut st.numbering_order,
                                NumberingOrder::BottomToTop,
                                tr::spec_bottom_to_top(lang),
                            );
                            ui.selectable_value(
                                &mut st.numbering_order,
                                NumberingOrder::TopToBottom,
                                tr::spec_top_to_bottom(lang),
                            );
                        });
                });

                ui.add_space(6.0);
                for lvl in 0..st.levels.clamp(1, 4) {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(tr::spec_tier_n(lang, lvl + 1));
                        egui::ComboBox::from_id_salt(format!("spec_edit_tier_kind_{lvl}"))
                            .selected_text(match st.tier_kinds[lvl] {
                                CustomTierKind::Sequential => strings.custom_spec_tier_seq,
                                CustomTierKind::ProtectiveEarth => strings.custom_spec_tier_pe,
                                CustomTierKind::Fixed => strings.custom_spec_tier_fixed,
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut st.tier_kinds[lvl],
                                    CustomTierKind::Sequential,
                                    strings.custom_spec_tier_seq,
                                );
                                ui.selectable_value(
                                    &mut st.tier_kinds[lvl],
                                    CustomTierKind::ProtectiveEarth,
                                    strings.custom_spec_tier_pe,
                                );
                                ui.selectable_value(
                                    &mut st.tier_kinds[lvl],
                                    CustomTierKind::Fixed,
                                    strings.custom_spec_tier_fixed,
                                );
                            });

                        if st.tier_kinds[lvl] == CustomTierKind::Fixed {
                            ui.add(
                                egui::TextEdit::singleline(&mut st.tier_fixed[lvl])
                                    .hint_text("N / +24V / ...")
                                    .desired_width(90.0),
                            );
                        }

                        egui::ComboBox::from_id_salt(format!("spec_edit_tier_color_{lvl}"))
                            .selected_text(tr::tier_color_label(lang, st.tier_colors[lvl]))
                            .show_ui(ui, |ui| {
                                for c in [
                                    TierColor::StandardGray,
                                    TierColor::BlueNeutral,
                                    TierColor::GreenYellowPe,
                                    TierColor::RedPhase,
                                    TierColor::OrangeAux,
                                ] {
                                    ui.selectable_value(
                                        &mut st.tier_colors[lvl],
                                        c,
                                        tr::tier_color_label(lang, c),
                                    );
                                }
                            });
                    });
                }
            });

            ui.add_space(6.0);
            egui::CollapsingHeader::new(tr::spec_links_heading(lang))
                .default_open(false)
                .show(ui, |ui| {
                    egui::Grid::new("spec_edit_urls_grid")
                        .num_columns(2)
                        .spacing([12.0, 6.0])
                        .show(ui, |ui| {
                            ui.label(tr::spec_product_url(lang));
                            ui.add(
                                egui::TextEdit::singleline(&mut st.product_url)
                                    .desired_width(ui.available_width().max(220.0)),
                            );
                            ui.end_row();
                            ui.label(tr::spec_image_url(lang));
                            ui.add(
                                egui::TextEdit::singleline(&mut st.image_url)
                                    .desired_width(ui.available_width().max(220.0)),
                            );
                            ui.end_row();
                        });
                });
        },
        |ui, st, modal_open| {
            if st.is_builtin && st.was_modified {
                if danger_icon_button(ui, UiIcon::Undo, tr::reset_factory(lang)).clicked() {
                    action = Some(DatabaseAction::ResetOrDeleteSpec {
                        id: st.spec_id.clone(),
                        is_builtin: true,
                    });
                    *modal_open = false;
                }
            } else if !st.is_builtin
                && danger_icon_button(ui, UiIcon::Delete, tr::delete(lang)).clicked()
            {
                action = Some(DatabaseAction::ResetOrDeleteSpec {
                    id: st.spec_id.clone(),
                    is_builtin: false,
                });
                *modal_open = false;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let can_save = !st.name.trim().is_empty();
                ui.add_enabled_ui(can_save, |ui| {
                    if primary_icon_button(ui, UiIcon::Save, tr::save(lang)).clicked() {
                        action = Some(DatabaseAction::SaveSpec(Box::new(st.build_spec())));
                        *modal_open = false;
                    }
                });
                if ghost_icon_button(ui, UiIcon::Close, tr::cancel(lang)).clicked() {
                    *modal_open = false;
                }
            });
        },
    );

    state.open = open;
    action
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_terminal_json_serializes_complete_schema_and_round_trips() {
        let catalog = TerminalCatalog::load_embedded();
        let example = build_example_terminal_json(&catalog);
        assert!(example.contains("\"custom-beispiel-2002-3217\""));
        assert!(example.contains("\"accessories\""));
        assert!(example.contains("\"2002-3291"));
        assert!(example.contains("\"2009-115"));
        assert!(example.contains("\"height_tenths_mm\": 933"));

        let parsed = parse_terminal_specs_json(&example).expect("example JSON must parse cleanly");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].id, "custom-beispiel-2002-3217");
        assert_eq!(parsed[0].manufacturer, "WAGO");
        assert_eq!(parsed[0].levels, 3);
        assert_eq!(parsed[0].tiers.len(), 3);
        assert!(parsed[0].accessories.is_some());
    }

    #[test]
    fn parse_terminal_specs_json_accepts_single_object_array_and_fences() {
        let catalog = TerminalCatalog::load_embedded();
        let single_json = build_example_terminal_json(&catalog);

        // 1. Single `{ ... }` object
        let one = parse_terminal_specs_json(&single_json).expect("single object should parse");
        assert_eq!(one.len(), 1);

        // 2. JSON array `[{ ... }, { ... }]`
        let array_json = format!("[{}, {}]", single_json, single_json);
        let two = parse_terminal_specs_json(&array_json).expect("array should parse");
        assert_eq!(two.len(), 2);

        // 3. Markdown fenced single object with UTF-8 BOM
        let fenced = format!("\u{feff}```json\n{}\n```", single_json);
        let fenced_parsed =
            parse_terminal_specs_json(&fenced).expect("fenced single object should parse");
        assert_eq!(fenced_parsed.len(), 1);

        // 4. Invalid single `{ ... }` reports the real object error, NOT "expected a sequence"
        let broken_single = r#"{ "id": "test", "name": "Test 2,5" }"#;
        let err = parse_terminal_specs_json(broken_single).unwrap_err();
        assert!(
            !err.contains("expected a sequence"),
            "single object error must not say expected a sequence: {err}"
        );
        assert!(err.contains("missing field"), "expected missing field error: {err}");
    }
}


