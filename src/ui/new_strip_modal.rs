use egui::{Context, Ui};

use crate::catalog::accessories::{
    all_end_brackets, find_end_bracket, DEFAULT_END_BRACKET_ID,
};
use crate::catalog::TerminalCatalog;
use crate::i18n::{Language, Strings};
use crate::model::TerminalStrip;
use crate::ui::icons::{
    compact_mfg_badge, draw_icon, icon_button, icon_only_button, primary_icon_button,
    show_responsive_modal, UiIcon,
};
use crate::ui::logos::{logo_filter_button, show_logo_badge, BrandLogos};
use crate::ui::product_media::{
    show_end_bracket_preview_card, show_product_preview_card, ProductMediaCache,
};
use crate::ui::theme::secondary_text_color;

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
    let mut open = state.open;

    show_responsive_modal(
        ctx,
        "new_strip_modal",
        UiIcon::Add,
        strings.modal_new_strip_title,
        580.0,
        &mut open,
        state,
        |ui, state| {
            ui.horizontal_wrapped(|ui| {
                ui.radio_value(&mut state.mode, NewStripMode::ByRange, strings.gen_mode_range);
                ui.radio_value(&mut state.mode, NewStripMode::ByCount, strings.gen_mode_count);
            });
            ui.add_space(4.0);

            let input_w = (ui.available_width() * 0.52).clamp(110.0, 280.0);
            egui::Grid::new("new_strip_grid")
                .num_columns(2)
                .spacing([12.0, 8.0])
                .show(ui, |ui| {
                    ui.label(strings.strip_designation);
                    ui.add(egui::TextEdit::singleline(&mut state.name).desired_width(input_w));
                    ui.end_row();

                    ui.label(strings.strip_description);
                    ui.add(
                        egui::TextEdit::singleline(&mut state.description).desired_width(input_w),
                    );
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
                None,
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
                ui.add(
                    egui::Label::new(
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
                    )
                    .wrap(),
                );
            }
        },
        |ui, state, close_requested| {
            if primary_icon_button(ui, UiIcon::Add, strings.btn_create_strip).clicked()
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
                *close_requested = true;
            }

            if icon_button(ui, UiIcon::Close, strings.btn_cancel).clicked() {
                *close_requested = true;
            }
        },
    );

    state.open = open;
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

/// Reusable manufacturer-filtered & globally searchable terminal spec picker with brand badges.
#[allow(clippy::too_many_arguments)]
pub fn show_spec_picker(
    ui: &mut Ui,
    catalog: &TerminalCatalog,
    logos: &BrandLogos,
    manufacturer_filter: &mut Option<String>,
    search_query: &mut String,
    selected_spec_id: &mut String,
    reference_spec: Option<&crate::catalog::TerminalSpec>,
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
    let has_query = !search_query.trim().is_empty();

    ui.horizontal(|ui| {
        let (sr, _) = ui.allocate_exact_size(egui::Vec2::splat(16.0), egui::Sense::hover());
        draw_icon(ui.painter(), sr, UiIcon::Search, secondary_text_color(ui));
        let field_w = (input_w - if has_query { 54.0 } else { 24.0 }).clamp(80.0, 320.0);
        ui.add(
            egui::TextEdit::singleline(search_query)
                .hint_text(search_hint)
                .desired_width(field_w),
        );
        if has_query && icon_only_button(ui, UiIcon::Clear, false, "Suche leeren").clicked() {
            search_query.clear();
        }
    });

    let has_query = !search_query.trim().is_empty();
    let filtered = catalog.filter_specs_for_reference(
        manufacturer_filter.as_deref(),
        search_query,
        reference_spec,
    );

    if has_query && manufacturer_filter.is_some() {
        ui.label(
            egui::RichText::new(format!(
                "Globale Suche über alle Hersteller ({} Treffer)",
                filtered.len()
            ))
            .color(egui::Color32::from_rgb(14, 165, 233))
            .small(),
        );
    }

    if !filtered.iter().any(|s| &s.id == selected_spec_id)
        && let Some(first) = filtered.first()
    {
        *selected_spec_id = first.id.clone();
    }

    if has_query {
        if filtered.is_empty() {
            ui.label(
                egui::RichText::new("Keine passenden Klemmen gefunden.")
                    .color(secondary_text_color(ui))
                    .small(),
            );
        } else {
            egui::Frame::group(ui.style())
                .inner_margin(egui::Margin::symmetric(6, 4))
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt(format!("{spec_label}_quick_hits"))
                        .max_height(132.0)
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            for spec in filtered.iter().take(50) {
                                let is_sel = *selected_spec_id == spec.id;
                                let art_tag = spec
                                    .effective_article_number()
                                    .map(|a| format!(" · Art. {a}"))
                                    .unwrap_or_default();
                                let row_text = format!(
                                    "{} [{}]{}",
                                    spec.name,
                                    spec.pattern_summary(),
                                    art_tag
                                );
                                ui.horizontal(|ui| {
                                    compact_mfg_badge(ui, &spec.manufacturer);
                                    ui.selectable_value(selected_spec_id, spec.id.clone(), row_text);
                                    if is_sel {
                                        let (cr, _) = ui.allocate_exact_size(
                                            egui::Vec2::splat(12.0),
                                            egui::Sense::hover(),
                                        );
                                        draw_icon(
                                            ui.painter(),
                                            cr,
                                            UiIcon::Check,
                                            egui::Color32::from_rgb(22, 163, 74),
                                        );
                                    }
                                });
                            }
                            if filtered.len() > 50 {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "… {} weitere Treffer (Suchbegriff eingrenzen)",
                                        filtered.len() - 50
                                    ))
                                    .color(secondary_text_color(ui))
                                    .small(),
                                );
                            }
                        });
                });
        }
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
            let mut in_compat_section = false;
            let mut in_other_section = false;
            for spec in filtered {
                if !has_query && let Some(ref_spec) = reference_spec {
                    let is_compat = spec.compatibility_rank(ref_spec).0 <= 2;
                    if is_compat && !in_compat_section {
                        in_compat_section = true;
                        ui.label(
                            egui::RichText::new("Passende Typen (gleiche Serie / Profil)")
                                .color(egui::Color32::from_rgb(14, 165, 233))
                                .small()
                                .strong(),
                        );
                    } else if !is_compat && !in_other_section {
                        in_other_section = true;
                        if in_compat_section {
                            ui.separator();
                        }
                        ui.label(
                            egui::RichText::new("Weitere Typen")
                                .color(secondary_text_color(ui))
                                .small(),
                        );
                    }
                }
                let art_tag = spec
                    .effective_article_number()
                    .map(|a| format!(" · {a}"))
                    .unwrap_or_default();
                let text = format!("{} [{}]{}", spec.name, spec.pattern_summary(), art_tag);
                ui.horizontal(|ui| {
                    compact_mfg_badge(ui, &spec.manufacturer);
                    ui.selectable_value(selected_spec_id, spec.id.clone(), text);
                });
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
    InsertSequence {
        strip_idx: usize,
        at_index: usize,
        spec_ids: Vec<String>,
        select_inserted: bool,
    },
}

/// State of the interactive prompt for NT / 2003 / 2203 / 2005 / 2205 / PTI / AITB installation terminals.
#[derive(Debug, Clone)]
pub struct InstallPromptDialogState {
    pub open: bool,
    pub action: Option<PendingInstallAction>,
    pub base_spec_id: String,
    pub use_nt_variant: bool,
    pub supply_placement: InstallSupplyPlacement,
    pub supply_spec_id: String,
    pub strip_already_has_supply: bool,
    pub add_outer_busbar_carriers: bool,
    pub outer_carrier_spec_id: String,
    pub add_intermediate_busbar_supports: bool,
    pub intermediate_support_spec_id: String,
    pub strip_already_has_carriers: bool,
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
            add_outer_busbar_carriers: true,
            outer_carrier_spec_id: "wago-2009-305".to_string(),
            add_intermediate_busbar_supports: true,
            intermediate_support_spec_id: "wago-2009-304".to_string(),
            strip_already_has_carriers: false,
        }
    }
}

impl InstallPromptDialogState {
    pub fn open_for_action(
        &mut self,
        action: PendingInstallAction,
        spec: &crate::catalog::TerminalSpec,
        strip_already_has_supply: bool,
        strip_already_has_carriers: bool,
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
        self.strip_already_has_carriers = strip_already_has_carriers;

        if let Some(outer_id) = spec.recommended_outer_busbar_carrier_spec_id() {
            self.outer_carrier_spec_id = outer_id.to_string();
            self.add_outer_busbar_carriers = !strip_already_has_carriers;
        } else {
            self.outer_carrier_spec_id.clear();
            self.add_outer_busbar_carriers = false;
        }
        if let Some(support_id) = spec.recommended_intermediate_busbar_support_spec_id() {
            self.intermediate_support_spec_id = support_id.to_string();
            self.add_intermediate_busbar_supports = true;
        } else {
            self.intermediate_support_spec_id.clear();
            self.add_intermediate_busbar_supports = false;
        }
    }
}

/// Confirmed choice from the NT, N-Einspeiseklemme & Sammelschienenträger modal dialog.
#[derive(Debug, Clone)]
pub struct InstallPromptResolution {
    pub action: PendingInstallAction,
    pub effective_spec_id: String,
    pub supply_placement: InstallSupplyPlacement,
    pub supply_spec_id: String,
    pub outer_carrier_spec_id: Option<String>,
    pub intermediate_support_spec_id: Option<String>,
}

/// Renders the interactive confirmation dialog asking whether an NT disconnect terminal,
/// matching N-supply terminal (`2016-7714`, `2216-7714`, `PTI 16-NLS-FI BU`, `AITB 16 BB NDT BL`),
/// and N-busbar carriers (`2009-305`, `2009-304` every 200 mm, Phoenix `AB-*` Auflagebock, Weidmüller `AEB ITB`)
/// should be placed when using WAGO `2003`/`2203`/`2005`/`2205`, Phoenix `PTI`/`UTI`/`STI`, or NT terminals.
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

    let mut resolution: Option<InstallPromptResolution> = None;
    let mut open = state.open;

    show_responsive_modal(
        ctx,
        "install_supply_prompt_modal",
        UiIcon::Bolt,
        "Installations- / NT-Klemme, N-Einspeisung & Sammelschienenträger",
        600.0,
        &mut open,
        state,
        |ui, state| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(
                        "Für Installationsklemmen (Serie 2003 / 2203 / 2005 / 2205 / PTI / UTI / STI / AITB) und NT-Klemmen bitte Ausführung, N-Einspeiseklemme und Sammelschienenträger (Auflagebock) wählen:",
                    )
                    .color(secondary_text_color(ui)),
                )
                .wrap(),
            );
            ui.add_space(6.0);

            // 1. NT-Klemmen-Auswahl (N-Trennschlitten vs. Standard)
            ui.group(|ui| {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new("1. Ausführung der Installationsklemme (NT / Standard)")
                            .strong(),
                    )
                    .wrap(),
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
                    ui.horizontal_wrapped(|ui| {
                        let (cr, _) =
                            ui.allocate_exact_size(egui::Vec2::splat(14.0), egui::Sense::hover());
                        draw_icon(
                            ui.painter(),
                            cr,
                            UiIcon::Check,
                            egui::Color32::from_rgb(16, 185, 129),
                        );
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(format!(
                                    "NT-Trennklemme mit N-Trennschlitten ausgewählt: {} {}",
                                    base_spec.manufacturer, base_spec.name
                                ))
                                .color(egui::Color32::from_rgb(16, 185, 129))
                                .strong(),
                            )
                            .wrap(),
                        );
                    });
                } else {
                    ui.add(
                        egui::Label::new(format!(
                            "Ausgewählt: {} {} [{}]",
                            base_spec.manufacturer,
                            base_spec.name,
                            base_spec.pattern_summary()
                        ))
                        .wrap(),
                    );
                }

                let preview_spec = if state.use_nt_variant
                    && let Some(nt_spec) = matching_nt
                {
                    nt_spec
                } else {
                    base_spec
                };
                ui.add_space(4.0);
                show_product_preview_card(
                    ui,
                    preview_spec,
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
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(
                            "2. Passende N-Einspeiseklemme (z. B. WAGO 2016-7714) setzen?",
                        )
                        .strong(),
                    )
                    .wrap(),
                );
                if state.strip_already_has_supply {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(
                                "Hinweis: In dieser Klemmleiste befindet sich bereits eine N-Einspeiseklemme.",
                            )
                            .color(secondary_text_color(ui))
                            .small(),
                        )
                        .wrap(),
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
                    let active_spec = if state.use_nt_variant
                        && let Some(nt_spec) = matching_nt
                    {
                        nt_spec
                    } else {
                        base_spec
                    };
                    let compat_ids = active_spec.compatible_n_supply_spec_ids();
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
                    let combo_w = (ui.available_width() - 12.0).clamp(140.0, 440.0);
                    egui::ComboBox::from_id_salt("install_supply_spec_combo")
                        .width(combo_w)
                        .truncate()
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

            // 3. Sammelschienenträger / Auflagebock (N-Sammelschiene 10×3 mm)
            let active_spec = if state.use_nt_variant
                && let Some(nt_spec) = matching_nt
            {
                nt_spec
            } else {
                base_spec
            };
            let compat_carriers = active_spec.compatible_busbar_carrier_spec_ids();
            if !compat_carriers.is_empty() {
                ui.add_space(6.0);
                ui.group(|ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(
                                "3. Sammelschienenträger / Auflagebock (N-Sammelschiene 10×3 mm)",
                            )
                            .strong(),
                        )
                        .wrap(),
                    );
                    if state.strip_already_has_carriers {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(
                                    "Hinweis: In dieser Klemmleiste sind bereits Sammelschienenträger / Auflageböcke platziert.",
                                )
                                .color(secondary_text_color(ui))
                                .small(),
                            )
                            .wrap(),
                        );
                    }
                    ui.add_space(2.0);

                    if !compat_carriers.contains(&state.outer_carrier_spec_id.as_str())
                        && let Some(rec) = active_spec.recommended_outer_busbar_carrier_spec_id()
                    {
                        state.outer_carrier_spec_id = rec.to_string();
                    }
                    if !compat_carriers.contains(&state.intermediate_support_spec_id.as_str())
                        && let Some(rec) = active_spec.recommended_intermediate_busbar_support_spec_id()
                    {
                        state.intermediate_support_spec_id = rec.to_string();
                    }

                    ui.checkbox(
                        &mut state.add_outer_busbar_carriers,
                        "Äußere Sammelschienenträger / Auflageböcke vor N-Einspeiseklemme & nach letzter Klemme setzen",
                    );

                    if state.add_outer_busbar_carriers {
                        let selected_outer = catalog.find_by_id(&state.outer_carrier_spec_id);
                        let selected_outer_text = selected_outer
                            .map(|s| {
                                format!(
                                    "{} {} (Art. {}, {:.1} mm)",
                                    s.manufacturer,
                                    s.name,
                                    s.effective_article_number().unwrap_or("—"),
                                    f32::from(s.width_tenths_mm) / 10.0
                                )
                            })
                            .unwrap_or_else(|| state.outer_carrier_spec_id.clone());

                        let combo_w = (ui.available_width() - 12.0).clamp(140.0, 440.0);
                        egui::ComboBox::from_id_salt("install_outer_carrier_combo")
                            .width(combo_w)
                            .truncate()
                            .selected_text(selected_outer_text)
                            .show_ui(ui, |ui| {
                                for &cid in compat_carriers {
                                    if let Some(cspec) = catalog.find_by_id(cid) {
                                        let label = format!(
                                            "{} {} (Art. {}, {:.1} mm)",
                                            cspec.manufacturer,
                                            cspec.name,
                                            cspec.effective_article_number().unwrap_or("—"),
                                            f32::from(cspec.width_tenths_mm) / 10.0
                                        );
                                        ui.selectable_value(
                                            &mut state.outer_carrier_spec_id,
                                            cspec.id.clone(),
                                            label,
                                        );
                                    }
                                }
                            });

                        if let Some(car_spec) = catalog.find_by_id(&state.outer_carrier_spec_id) {
                            ui.add_space(4.0);
                            show_product_preview_card(
                                ui,
                                car_spec,
                                logos,
                                media_cache,
                                strings.open_product_page,
                                strings.bridge_chambers_unit,
                                strings.height_label,
                            );
                        }
                    }

                    ui.add_space(4.0);
                    ui.checkbox(
                        &mut state.add_intermediate_busbar_supports,
                        "Bei langen Klemmenreihen automatisch alle 200 mm Zwischenstütze einfügen",
                    );

                    if state.add_intermediate_busbar_supports {
                        let selected_mid = catalog.find_by_id(&state.intermediate_support_spec_id);
                        let selected_mid_text = selected_mid
                            .map(|s| {
                                format!(
                                    "{} {} (Art. {}, {:.1} mm)",
                                    s.manufacturer,
                                    s.name,
                                    s.effective_article_number().unwrap_or("—"),
                                    f32::from(s.width_tenths_mm) / 10.0
                                )
                            })
                            .unwrap_or_else(|| state.intermediate_support_spec_id.clone());

                        let combo_w = (ui.available_width() - 12.0).clamp(140.0, 440.0);
                        egui::ComboBox::from_id_salt("install_mid_support_combo")
                            .width(combo_w)
                            .truncate()
                            .selected_text(selected_mid_text)
                            .show_ui(ui, |ui| {
                                for &cid in compat_carriers {
                                    if let Some(cspec) = catalog.find_by_id(cid) {
                                        let label = format!(
                                            "{} {} (Art. {}, {:.1} mm)",
                                            cspec.manufacturer,
                                            cspec.name,
                                            cspec.effective_article_number().unwrap_or("—"),
                                            f32::from(cspec.width_tenths_mm) / 10.0
                                        );
                                        ui.selectable_value(
                                            &mut state.intermediate_support_spec_id,
                                            cspec.id.clone(),
                                            label,
                                        );
                                    }
                                }
                            });

                        if let Some(mid_spec) = catalog.find_by_id(&state.intermediate_support_spec_id)
                            && (!state.add_outer_busbar_carriers
                                || mid_spec.id != state.outer_carrier_spec_id)
                        {
                            ui.add_space(4.0);
                            show_product_preview_card(
                                ui,
                                mid_spec,
                                logos,
                                media_cache,
                                strings.open_product_page,
                                strings.bridge_chambers_unit,
                                strings.height_label,
                            );
                        }
                    }
                });
            }
        },
        |ui, state, close_requested| {
            let final_spec = if state.use_nt_variant
                && let Some(nt_spec) = matching_nt
            {
                nt_spec
            } else {
                base_spec
            };
            if primary_icon_button(ui, UiIcon::Check, "Übernehmen & Einfügen").clicked()
                && let Some(action) = state.action.take()
            {
                resolution = Some(InstallPromptResolution {
                    action,
                    effective_spec_id: final_spec.id.clone(),
                    supply_placement: state.supply_placement,
                    supply_spec_id: state.supply_spec_id.clone(),
                    outer_carrier_spec_id: state
                        .add_outer_busbar_carriers
                        .then(|| state.outer_carrier_spec_id.clone())
                        .filter(|s| !s.is_empty()),
                    intermediate_support_spec_id: state
                        .add_intermediate_busbar_supports
                        .then(|| state.intermediate_support_spec_id.clone())
                        .filter(|s| !s.is_empty()),
                });
                *close_requested = true;
            }
            if icon_button(ui, UiIcon::Close, strings.btn_cancel).clicked() {
                *close_requested = true;
            }
        },
    );

    state.open = open;
    if !state.open {
        state.action = None;
    }

    resolution
}

