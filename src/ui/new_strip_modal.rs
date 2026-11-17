use egui::{Context, Ui};

use crate::catalog::TerminalCatalog;
use crate::i18n::{Language, Strings};
use crate::model::TerminalStrip;
use crate::ui::logos::{logo_filter_button, show_logo_badge, BrandLogos};
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
    }
}

/// Renders the "New Terminal Strip" modal window. Returns `Some(TerminalStrip)` when confirmed.
pub fn show_new_strip_modal(
    ctx: &Context,
    state: &mut NewStripDialogState,
    catalog: &TerminalCatalog,
    logos: &BrandLogos,
    strings: &Strings,
    lang: Language,
) -> Option<TerminalStrip> {
    if !state.open {
        return None;
    }

    let mut created: Option<TerminalStrip> = None;
    let mut should_close = false;

    let screen_size = ctx.content_rect().size();
    let max_w = (screen_size.x - 24.0).max(260.0);
    let max_h = (screen_size.y - 48.0).max(220.0);

    egui::Window::new(strings.modal_new_strip_title)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .min_width(440.0_f32.min(max_w))
        .max_width(max_w)
        .max_height(max_h)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
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
                });

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.radio_value(&mut state.mode, NewStripMode::ByRange, strings.gen_mode_range);
                ui.radio_value(&mut state.mode, NewStripMode::ByCount, strings.gen_mode_count);
            });

            ui.horizontal(|ui| {
                match state.mode {
                    NewStripMode::ByRange => {
                        ui.label(strings.end_number);
                        ui.add(
                            egui::DragValue::new(&mut state.end_number)
                                .range(state.start_number..=99999),
                        );
                        if state.end_number < state.start_number {
                            state.end_number = state.start_number;
                        }
                    }
                    NewStripMode::ByCount => {
                        ui.label(strings.block_count_label);
                        ui.add(egui::DragValue::new(&mut state.block_count).range(1..=500));
                    }
                }
            });

            ui.separator();

            // Manufacturer filter + search + terminal spec selector
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

            ui.separator();

            // Live preview of resulting block count
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
                ui.label(
                    egui::RichText::new(format!(
                        "→ {} {} ({}: {})",
                        preview_blocks,
                        strings.blocks_unit,
                        spec.levels,
                        spec.pattern_summary()
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
                    created = Some(strip);
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
    }

    created
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

    ui.add(
        egui::TextEdit::singleline(search_query)
            .hint_text(search_hint)
            .desired_width((ui.available_width() - 8.0).max(180.0)),
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
        .width((ui.available_width() - 8.0).max(200.0))
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
            ui.label(egui::RichText::new(desc).color(secondary_text_color(ui)));
        });
    }
}
