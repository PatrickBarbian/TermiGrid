use egui::{Context, Ui};

use crate::catalog::accessories::{
    all_end_brackets, find_end_bracket, DEFAULT_END_BRACKET_ID,
};
use crate::catalog::TerminalCatalog;
use crate::i18n::{Language, Strings};
use crate::model::TerminalStrip;
use crate::ui::logos::{logo_filter_button, show_logo_badge, BrandLogos};
use crate::ui::product_media::{
    show_end_bracket_preview_card, show_product_preview_card, ProductMediaCache,
};
use crate::ui::theme::{primary_button, secondary_text_color};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewStripMode {
    ByRange,
    ByCount,
}

#[derive(Debug, Clone)]
pub struct NewStripDialogState {
    pub open: bool,
    pub name: String,
    pub description: String,
    pub start_number: u32,
    pub end_number: u32,
    pub block_count: usize,
    pub mode: NewStripMode,
    pub manufacturer_filter: Option<String>,
    pub search_query: String,
    pub selected_spec_id: String,
    pub end_bracket_id: String,
    pub end_bracket_left: bool,
    pub end_bracket_right: bool,
}

impl Default for NewStripDialogState {
    fn default() -> Self {
        Self {
            open: false,
            name: "-X1".to_string(),
            description: String::new(),
            start_number: 1,
            end_number: 50,
            block_count: 25,
            mode: NewStripMode::ByRange,
            manufacturer_filter: None,
            search_query: String::new(),
            selected_spec_id: "wago-2002-1201".to_string(),
            end_bracket_id: DEFAULT_END_BRACKET_ID.to_string(),
            end_bracket_left: true,
            end_bracket_right: true,
        }
    }
}

impl NewStripDialogState {
    pub fn open_for_next_strip(&mut self, existing_count: usize) {
        self.open = true;
        self.name = format!("-X{}", existing_count + 1);
        self.description.clear();
        self.start_number = 1;
        self.end_number = 50;
        self.block_count = 25;
        self.end_bracket_left = true;
        self.end_bracket_right = true;
    }
}

/// Renders the "New Terminal Strip" modal window. Returns `Some(TerminalStrip)` when confirmed.
#[allow(clippy::too_many_arguments)]
pub fn show_new_strip_modal(
    ctx: &Context,
    state: &mut NewStripDialogState,
    catalog: &TerminalCatalog,
    logos: &BrandLogos,
    media_cache: &ProductMediaCache,
    strings: &Strings,
    lang: Language,
) -> Option<TerminalStrip> {
    if !state.open {
        return None;
    }

    let mut created: Option<TerminalStrip> = None;
    let mut should_close = false;

    let screen_size = ctx.content_rect().size();
    let max_w = (screen_size.x - 24.0).max(280.0);
    let max_h = (screen_size.y - 40.0).max(220.0);
    let target_w = 560.0_f32.min(max_w);
    let needs_scroll = screen_size.y < 900.0;

    egui::Window::new(strings.modal_new_strip_title)
        .collapsible(false)
        .resizable(true)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .default_width(target_w)
        .min_width(480.0_f32.min(max_w))
        .max_width(max_w)
        .max_height(max_h)
        .show(ctx, |ui| {
            let mut render_body = |ui: &mut Ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.radio_value(&mut state.mode, NewStripMode::ByRange, strings.gen_mode_range);
                    ui.radio_value(&mut state.mode, NewStripMode::ByCount, strings.gen_mode_count);
                });
                ui.add_space(4.0);

                egui::Grid::new("new_strip_grid")
                    .num_columns(2)
                    .spacing([12.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(strings.strip_designation);
                        ui.text_edit_singleline(&mut state.name);
                        ui.end_row();

                        ui.label(strings.strip_description);
                        ui.text_edit_singleline(&mut state.description);
                        ui.end_row();

                        ui.label(strings.start_number);
                        ui.add(egui::DragValue::new(&mut state.start_number).range(1..=99999));
                        ui.end_row();

                        match state.mode {
                            NewStripMode::ByRange => {
                                ui.label(strings.end_number);
                                if state.end_number < state.start_number {
                                    state.end_number = state.start_number;
                                }
                                ui.add(
                                    egui::DragValue::new(&mut state.end_number)
                                        .range(state.start_number..=99999),
                                );
                                ui.end_row();
                            }
                            NewStripMode::ByCount => {
                                ui.label(strings.block_count_label);
                                ui.add(egui::DragValue::new(&mut state.block_count).range(1..=500));
                                ui.end_row();
                            }
                        }
                    });

                ui.separator();

                // Manufacturer filter + search + terminal spec selector + live preview
                show_spec_picker(
                    ui,
                    catalog,
                    logos,
                    &mut state.manufacturer_filter,
                    &mut state.search_query,
                    &mut state.selected_spec_id,
                    strings.filter_manufacturer,
                    strings.default_terminal_type,
                    strings.all_manufacturers,
                    strings.search_hint,
                    lang,
                );
                if let Some(spec) = catalog.find_by_id(&state.selected_spec_id) {
                    ui.add_space(4.0);
                    show_product_preview_card(
                        ui,
                        spec,
                        logos,
                        media_cache,
                        strings.open_product_page,
                        strings.bridge_chambers_unit,
                        strings.height_label,
                    );
                }

                ui.separator();

                // End bracket selector + left/right checkboxes + live preview
                show_end_bracket_picker(
                    ui,
                    logos,
                    &mut state.end_bracket_id,
                    &mut state.end_bracket_left,
                    &mut state.end_bracket_right,
                    strings,
                    lang,
                    "new_strip_end_bracket",
                );
                if state.end_bracket_left || state.end_bracket_right {
                    ui.add_space(4.0);
                    let eb_spec = find_end_bracket(&state.end_bracket_id);
                    show_end_bracket_preview_card(
                        ui,
                        eb_spec,
                        logos,
                        media_cache,
                        strings.open_product_page,
                        strings.height_label,
                        lang,
                    );
                }

                ui.separator();

                // Live preview of resulting block count + width + Bauhöhe incl. end brackets
                if let Some(spec) = catalog.find_by_id(&state.selected_spec_id) {
                    let preview_blocks = match state.mode {
                        NewStripMode::ByRange => {
                            let total = state.end_number.saturating_sub(state.start_number) + 1;
                            let seq = spec.sequential_tier_count();
                            if seq == 0 {
                                total as usize
                            } else {
                                (total as usize).div_ceil(seq)
                            }
                        }
                        NewStripMode::ByCount => state.block_count,
                    };
                    let eb_spec = find_end_bracket(&state.end_bracket_id);
                    let eb_count =
                        usize::from(state.end_bracket_left) + usize::from(state.end_bracket_right);
                    let blocks_mm =
                        (f32::from(spec.width_tenths_mm) / 10.0) * (preview_blocks as f32);
                    let brackets_mm =
                        (f32::from(eb_spec.width_tenths_mm) / 10.0) * (eb_count as f32);
                    ui.label(
                        egui::RichText::new(format!(
                            "-> {} {} ({}: {})  |  ~{:.1} mm | {}: {:.1} mm",
                            preview_blocks,
                            strings.blocks_unit,
                            spec.levels,
                            spec.pattern_summary(),
                            blocks_mm + brackets_mm,
                            strings.height_label,
                            f32::from(spec.effective_height_tenths_mm()) / 10.0
                        ))
                        .color(secondary_text_color(ui)),
                    );
                }

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if primary_button(ui, strings.btn_create_strip).clicked()
                        && let Some(spec) = catalog.find_by_id(&state.selected_spec_id)
                    {
                        let clean_name = if state.name.trim().is_empty() {
                            "-X1".to_string()
                        } else {
                            state.name.trim().to_string()
                        };
                        let mut strip = match state.mode {
                            NewStripMode::ByRange => TerminalStrip::generate_from_range(
                                clean_name,
                                state.start_number,
                                state.end_number,
                                spec,
                            ),
                            NewStripMode::ByCount => TerminalStrip::generate_with_block_count(
                                clean_name,
                                state.start_number,
                                state.start_number + state.block_count as u32,
                                state.block_count,
                                spec,
                            ),
                        };
                        strip.description = state.description.trim().to_string();
                        strip.end_bracket_id = state.end_bracket_id.clone();
                        strip.end_bracket_left = state.end_bracket_left;
                        strip.end_bracket_right = state.end_bracket_right;
                        created = Some(strip);
                        should_close = true;
                    }

                    if ui.button(strings.btn_cancel).clicked() {
                        should_close = true;
                    }
                });
            };

            if needs_scroll {
                egui::ScrollArea::vertical().show(ui, render_body);
            } else {
                render_body(ui);
            }
        });

    if should_close {
        state.open = false;
    }

    created
}

/// Reusable TS 35 DIN-rail end bracket (`Endhalter`) ComboBox selector with left/right placement checkboxes.
#[allow(clippy::too_many_arguments)]
pub fn show_end_bracket_picker(
    ui: &mut Ui,
    logos: &BrandLogos,
    end_bracket_id: &mut String,
    end_bracket_left: &mut bool,
    end_bracket_right: &mut bool,
    strings: &Strings,
    lang: Language,
    id_salt: &str,
) {
    let selected = find_end_bracket(end_bracket_id);
    if end_bracket_id != selected.id {
        *end_bracket_id = selected.id.to_string();
    }

    ui.label(egui::RichText::new(strings.end_bracket_heading).strong());
    ui.add_space(2.0);

    let selected_text = format!(
        "{} {} (Art. {}, {:.1} mm)",
        selected.manufacturer,
        selected.name,
        selected.article_number,
        f32::from(selected.width_tenths_mm) / 10.0
    );

    let avail_w = ui.available_width();
    let combo_w = (avail_w - 16.0).clamp(100.0, 360.0);

    egui::ComboBox::from_id_salt(id_salt)
        .width(combo_w)
        .truncate()
        .selected_text(selected_text)
        .show_ui(ui, |ui| {
            for eb in all_end_brackets() {
                let text = format!(
                    "{} {} (Art. {}, {:.1} mm)",
                    eb.manufacturer,
                    eb.name,
                    eb.article_number,
                    f32::from(eb.width_tenths_mm) / 10.0
                );
                if ui
                    .selectable_label(selected.id == eb.id, text)
                    .clicked()
                {
                    *end_bracket_id = eb.id.to_string();
                }
            }
        });

    let active = find_end_bracket(end_bracket_id);
    let mounting = match lang {
        Language::De => active.mounting_de,
        _ => active.mounting_en,
    };
    ui.horizontal_wrapped(|ui| {
        show_logo_badge(ui, logos, active.manufacturer, 16.0);
        ui.add(
            egui::Label::new(
                egui::RichText::new(format!("{mounting} | Art. {}", active.article_number))
                    .color(secondary_text_color(ui))
                    .small(),
            )
            .wrap(),
        );
    });

    ui.horizontal_wrapped(|ui| {
        ui.checkbox(end_bracket_left, strings.end_bracket_left);
        ui.checkbox(end_bracket_right, strings.end_bracket_right);
    });
}

/// Reusable manufacturer-filtered & searchable terminal spec ComboBox picker with brand logos.
#[allow(clippy::too_many_arguments)]
pub fn show_spec_picker(
    ui: &mut Ui,
    catalog: &TerminalCatalog,
    logos: &BrandLogos,
    manufacturer_filter: &mut Option<String>,
    search_query: &mut String,
    selected_spec_id: &mut String,
    mfg_label: &str,
    spec_label: &str,
    all_label: &str,
    search_hint: &str,
    lang: Language,
) {
    ui.label(mfg_label);
    ui.horizontal_wrapped(|ui| {
        if ui
            .selectable_label(manufacturer_filter.is_none(), all_label)
            .clicked()
        {
            *manufacturer_filter = None;
        }
        for mfg in catalog.manufacturers() {
            let is_sel = manufacturer_filter.as_deref() == Some(mfg);
            if logo_filter_button(ui, logos, mfg, is_sel, 22.0).clicked() {
                *manufacturer_filter = if is_sel { None } else { Some(mfg.to_string()) };
            }
        }
    });
    ui.add_space(3.0);

    let avail_w = ui.available_width();
    let input_w = (avail_w - 16.0).clamp(100.0, 360.0);
    ui.add(
        egui::TextEdit::singleline(search_query)
            .hint_text(search_hint)
            .desired_width(input_w),
    );

    let q = search_query.trim().to_lowercase();
    let filtered: Vec<_> = catalog
        .all()
        .iter()
        .filter(|s| {
            let mfg_ok = manufacturer_filter
                .as_deref()
                .is_none_or(|m| s.manufacturer == m);
            let q_ok = q.is_empty()
                || s.name.to_lowercase().contains(&q)
                || s.manufacturer.to_lowercase().contains(&q)
                || s.series.to_lowercase().contains(&q)
                || s.pattern_summary().to_lowercase().contains(&q);
            mfg_ok && q_ok
        })
        .collect();

    if !filtered.iter().any(|s| &s.id == selected_spec_id)
        && let Some(first) = filtered.first()
    {
        *selected_spec_id = first.id.clone();
    }

    let selected_spec = catalog.find_by_id(selected_spec_id);
    let selected_text = selected_spec
        .map(|s| format!("{} {} [{}]", s.manufacturer, s.name, s.pattern_summary()))
        .unwrap_or_else(|| "—".to_string());

    ui.label(spec_label);
    egui::ComboBox::from_id_salt(format!("{spec_label}_spec"))
        .width(input_w)
        .truncate()
        .selected_text(selected_text)
        .show_ui(ui, |ui| {
            for spec in filtered {
                let text = format!(
                    "{} {} [{}]",
                    spec.manufacturer,
                    spec.name,
                    spec.pattern_summary()
                );
                ui.selectable_value(selected_spec_id, spec.id.clone(), text);
            }
        });

    if let Some(spec) = selected_spec {
        let desc = match lang {
            Language::De => &spec.description_de,
            _ => &spec.description_en,
        };
        ui.horizontal_wrapped(|ui| {
            show_logo_badge(ui, logos, &spec.manufacturer, 18.0);
            ui.add(
                egui::Label::new(egui::RichText::new(desc).color(secondary_text_color(ui)))
                    .wrap(),
            );
        });
    }
}

/// Where to place the matching N-busbar supply terminal (`N-Einspeiseklemme`, e.g. WAGO `2016-7714`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallSupplyPlacement {
    BeforeGroup,
    AfterGroup,
    None,
}

/// Action deferred while the user answers the NT / N-Einspeiseklemme prompt.
#[derive(Debug, Clone)]
pub enum PendingInstallAction {
    CreateStrip(TerminalStrip),
    ReassignSelection {
        strip_idx: usize,
        indices: Vec<usize>,
        spec_id: String,
    },
    InsertBlocks {
        strip_idx: usize,
        at_index: usize,
        count: usize,
        spec_id: String,
        select_inserted: bool,
    },
}

/// State of the interactive prompt for NT / 2003 / 2203 / 2005 / 2205 installation terminals.
#[derive(Debug, Clone)]
pub struct InstallPromptDialogState {
    pub open: bool,
    pub action: Option<PendingInstallAction>,
    pub base_spec_id: String,
    pub use_nt_variant: bool,
    pub supply_placement: InstallSupplyPlacement,
    pub supply_spec_id: String,
    pub strip_already_has_supply: bool,
}

impl Default for InstallPromptDialogState {
    fn default() -> Self {
        Self {
            open: false,
            action: None,
            base_spec_id: "wago-2003-7646".to_string(),
            use_nt_variant: false,
            supply_placement: InstallSupplyPlacement::BeforeGroup,
            supply_spec_id: "wago-2016-7714".to_string(),
            strip_already_has_supply: false,
        }
    }
}

impl InstallPromptDialogState {
    pub fn open_for_action(
        &mut self,
        action: PendingInstallAction,
        spec: &crate::catalog::TerminalSpec,
        strip_already_has_supply: bool,
    ) {
        self.open = true;
        self.action = Some(action);
        self.base_spec_id = spec.id.clone();
        self.use_nt_variant = spec.is_nt_disconnect_terminal();
        self.supply_placement = if strip_already_has_supply {
            InstallSupplyPlacement::None
        } else {
            InstallSupplyPlacement::BeforeGroup
        };
        self.supply_spec_id = spec.recommended_n_supply_spec_id().to_string();
        self.strip_already_has_supply = strip_already_has_supply;
    }
}

/// Confirmed choice from the NT & N-Einspeiseklemme modal dialog.
#[derive(Debug, Clone)]
pub struct InstallPromptResolution {
    pub action: PendingInstallAction,
    pub effective_spec_id: String,
    pub supply_placement: InstallSupplyPlacement,
    pub supply_spec_id: String,
}

/// Renders the interactive confirmation dialog asking whether an NT disconnect terminal
/// and/or a matching N-supply terminal (`2016-7714`, `2216-7714`, `PTI 16-NLS-FI BU`, `AITB 16 BB NDT BL`)
/// should be placed when using WAGO `2003` / `2203` / `2005` / `2205` or NT installation terminals.
#[allow(clippy::too_many_arguments)]
pub fn show_install_supply_prompt_modal(
    ctx: &Context,
    state: &mut InstallPromptDialogState,
    catalog: &TerminalCatalog,
    logos: &BrandLogos,
    media_cache: &ProductMediaCache,
    strings: &Strings,
) -> Option<InstallPromptResolution> {
    if !state.open || state.action.is_none() {
        return None;
    }

    let Some(base_spec) = catalog.find_by_id(&state.base_spec_id) else {
        state.open = false;
        state.action = None;
        return None;
    };

    let matching_nt = base_spec
        .matching_nt_spec_id()
        .and_then(|id| catalog.find_by_id(id));

    let effective_spec = if state.use_nt_variant
        && let Some(nt_spec) = matching_nt
    {
        nt_spec
    } else {
        base_spec
    };

    let mut resolution: Option<InstallPromptResolution> = None;
    let mut should_close = false;

    let screen_size = ctx.content_rect().size();
    let max_w = (screen_size.x - 24.0).max(280.0);
    let max_h = (screen_size.y - 40.0).max(240.0);
    let target_w = 560.0_f32.min(max_w);

    egui::Window::new("⚡ Installations- / NT-Klemme & N-Einspeiseklemme")
        .collapsible(false)
        .resizable(true)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .default_width(target_w)
        .min_width(460.0_f32.min(max_w))
        .max_width(max_w)
        .max_height(max_h)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(
                    egui::RichText::new(
                        "Für Installationsklemmen (Serie 2003 / 2203 / 2005 / 2205 / PTI / AITB) und NT-Klemmen bitte Ausführung und N-Einspeiseklemme wählen:",
                    )
                    .color(secondary_text_color(ui)),
                );
                ui.add_space(6.0);

                // 1. NT-Klemmen-Auswahl (N-Trennschlitten vs. Standard)
                ui.group(|ui| {
                    ui.label(
                        egui::RichText::new("1. Ausführung der Installationsklemme (NT / Standard)")
                            .strong(),
                    );
                    ui.add_space(2.0);

                    if let Some(nt_spec) = matching_nt {
                        ui.radio_value(
                            &mut state.use_nt_variant,
                            false,
                            format!(
                                "Gewählte Klemme beibehalten: {} {} [{}]",
                                base_spec.manufacturer,
                                base_spec.name,
                                base_spec.pattern_summary()
                            ),
                        );
                        ui.radio_value(
                            &mut state.use_nt_variant,
                            true,
                            format!(
                                "Als NT-Trennklemme (mit N-Trennschlitten) setzen: {} {} [{}]",
                                nt_spec.manufacturer,
                                nt_spec.name,
                                nt_spec.pattern_summary()
                            ),
                        );
                    } else if base_spec.is_nt_disconnect_terminal() {
                        ui.label(
                            egui::RichText::new(format!(
                                "✔ NT-Trennklemme mit N-Trennschlitten ausgewählt: {} {}",
                                base_spec.manufacturer, base_spec.name
                            ))
                            .color(egui::Color32::from_rgb(16, 185, 129))
                            .strong(),
                        );
                    } else {
                        ui.label(format!(
                            "Ausgewählt: {} {} [{}]",
                            base_spec.manufacturer,
                            base_spec.name,
                            base_spec.pattern_summary()
                        ));
                    }

                    ui.add_space(4.0);
                    show_product_preview_card(
                        ui,
                        effective_spec,
                        logos,
                        media_cache,
                        strings.open_product_page,
                        strings.bridge_chambers_unit,
                        strings.height_label,
                    );
                });

                ui.add_space(6.0);

                // 2. Passende N-Sammelschienen-Einspeiseklemme (z. B. WAGO 2016-7714)
                ui.group(|ui| {
                    ui.label(
                        egui::RichText::new(
                            "2. Passende N-Einspeiseklemme (z. B. WAGO 2016-7714) setzen?",
                        )
                        .strong(),
                    );
                    if state.strip_already_has_supply {
                        ui.label(
                            egui::RichText::new(
                                "ℹ Hinweis: In dieser Klemmleiste befindet sich bereits eine N-Einspeiseklemme.",
                            )
                            .color(secondary_text_color(ui))
                            .small(),
                        );
                    }
                    ui.add_space(2.0);

                    ui.radio_value(
                        &mut state.supply_placement,
                        InstallSupplyPlacement::BeforeGroup,
                        "Ja – Vor den Installationsklemmen einfügen (links einspeisen)",
                    );
                    ui.radio_value(
                        &mut state.supply_placement,
                        InstallSupplyPlacement::AfterGroup,
                        "Ja – Nach den Installationsklemmen einfügen (rechts einspeisen)",
                    );
                    ui.radio_value(
                        &mut state.supply_placement,
                        InstallSupplyPlacement::None,
                        "Nein – Keine N-Einspeiseklemme hinzufügen",
                    );

                    if state.supply_placement != InstallSupplyPlacement::None {
                        ui.add_space(4.0);
                        let compat_ids = effective_spec.compatible_n_supply_spec_ids();
                        if !compat_ids.contains(&state.supply_spec_id.as_str())
                            && let Some(&first_id) = compat_ids.first()
                        {
                            state.supply_spec_id = first_id.to_string();
                        }

                        let selected_supply = catalog.find_by_id(&state.supply_spec_id);
                        let selected_supply_text = selected_supply
                            .map(|s| {
                                format!(
                                    "{} {} (Art. {}, {:.1} mm)",
                                    s.manufacturer,
                                    s.name,
                                    s.effective_article_number().unwrap_or("—"),
                                    f32::from(s.width_tenths_mm) / 10.0
                                )
                            })
                            .unwrap_or_else(|| state.supply_spec_id.clone());

                        ui.label("Auswahl N-Einspeiseklemme:");
                        egui::ComboBox::from_id_salt("install_supply_spec_combo")
                            .width((ui.available_width() - 8.0).max(200.0))
                            .selected_text(selected_supply_text)
                            .show_ui(ui, |ui| {
                                for &cid in compat_ids {
                                    if let Some(cspec) = catalog.find_by_id(cid) {
                                        let label = format!(
                                            "{} {} (Art. {}, {:.1} mm)",
                                            cspec.manufacturer,
                                            cspec.name,
                                            cspec.effective_article_number().unwrap_or("—"),
                                            f32::from(cspec.width_tenths_mm) / 10.0
                                        );
                                        ui.selectable_value(
                                            &mut state.supply_spec_id,
                                            cspec.id.clone(),
                                            label,
                                        );
                                    }
                                }
                            });

                        if let Some(sup_spec) = catalog.find_by_id(&state.supply_spec_id) {
                            ui.add_space(4.0);
                            show_product_preview_card(
                                ui,
                                sup_spec,
                                logos,
                                media_cache,
                                strings.open_product_page,
                                strings.bridge_chambers_unit,
                                strings.height_label,
                            );
                        }
                    }
                });

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if primary_button(ui, "✔ Übernehmen & Einfügen").clicked()
                        && let Some(action) = state.action.take()
                    {
                        resolution = Some(InstallPromptResolution {
                            action,
                            effective_spec_id: effective_spec.id.clone(),
                            supply_placement: state.supply_placement,
                            supply_spec_id: state.supply_spec_id.clone(),
                        });
                        should_close = true;
                    }
                    if ui.button(strings.btn_cancel).clicked() {
                        should_close = true;
                    }
                });
            });
        });

    if should_close {
        state.open = false;
        state.action = None;
    }

    resolution
}

