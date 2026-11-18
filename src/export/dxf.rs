use std::fmt::Write as _;
use std::path::Path;

use crate::catalog::accessories::{calculate_end_plates, find_end_bracket};
use crate::catalog::TerminalCatalog;
use crate::model::{Project, TerminalStrip};

/// Generates an EPLAN Electric P8–compatible ASCII DXF (`AC1009` / AutoCAD R12) in 1:1 millimeters
/// containing **all terminal strips** of `project` in a single drawing file (`Aufbaubild`).
///
/// Uses strictly `LINE` and `TEXT` entities with `$INSUNITS = 4` (millimeters) and `$MEASUREMENT = 1`
/// (metric) to ensure clean import in EPLAN Electric P8, AutoCAD, LibreCAD, and QCAD without solid fills
/// or unsupported polyline/MTEXT entities.
pub fn export_to_dxf_string(project: &Project, catalog: &TerminalCatalog) -> String {
    let mut entities = String::new();
    let mut bounds = Bounds::default();

    let mut rail_cy: f64 = 0.0;
    let strip_spacing_mm = 78.0;

    for strip in &project.strips {
        let eb = find_end_bracket(&strip.end_bracket_id);
        let eb_w = f64::from(eb.width_tenths_mm) / 10.0;
        let eb_h = f64::from(eb.height_tenths_mm) / 10.0;
        let end_plates = calculate_end_plates(strip, catalog);

        let block_heights: Vec<f64> = strip
            .blocks
            .iter()
            .map(|b| {
                catalog
                    .find_by_id(&b.spec_id)
                    .map(|sp| f64::from(sp.effective_height_tenths_mm()) / 10.0)
                    .unwrap_or_else(|| match b.tiers.len() {
                        1 => 48.5,
                        2 => 69.7,
                        _ => 93.3,
                    })
            })
            .collect();

        let max_block_h = block_heights.iter().copied().fold(0.0_f64, f64::max);
        let max_h = if strip.blocks.is_empty() {
            35.0
        } else {
            let with_eb = if strip.end_bracket_count() > 0 {
                max_block_h.max(eb_h)
            } else {
                max_block_h
            };
            with_eb.max(35.0)
        };

        // Compute total strip width in exact millimeters
        let mut cursor_x = 0.0_f64;
        let mut strip_total_mm = 0.0_f64;

        if !strip.blocks.is_empty() && strip.end_bracket_left {
            strip_total_mm += eb_w;
        }
        for (idx, b) in strip.blocks.iter().enumerate() {
            strip_total_mm += f64::from(b.width_tenths_mm) / 10.0;
            if let Some(p) = end_plates.iter().find(|p| p.after_block_idx == idx) {
                strip_total_mm += f64::from(p.width_tenths_mm) / 10.0;
            }
        }
        if !strip.blocks.is_empty() && strip.end_bracket_right {
            strip_total_mm += eb_w;
        }
        if strip.blocks.is_empty() {
            strip_total_mm = 60.0;
        }

        // 1. TS 35 DIN rail (35.0 mm high, 15 mm overhang left & right)
        let rail_overhang = 15.0;
        let rail_x0 = -rail_overhang;
        let rail_x1 = strip_total_mm + rail_overhang;
        draw_rect(
            &mut entities,
            &mut bounds,
            "TERMI_RAIL",
            rail_x0,
            rail_cy - 17.5,
            rail_x1,
            rail_cy + 17.5,
        );
        // Inner TS 35 flange / web lines (27 mm inner opening / 7.5 mm lip reference)
        draw_line(
            &mut entities,
            &mut bounds,
            "TERMI_RAIL",
            rail_x0,
            rail_cy + 13.5,
            rail_x1,
            rail_cy + 13.5,
        );
        draw_line(
            &mut entities,
            &mut bounds,
            "TERMI_RAIL",
            rail_x0,
            rail_cy - 13.5,
            rail_x1,
            rail_cy - 13.5,
        );
        draw_text(
            &mut entities,
            &mut bounds,
            "TERMI_RAIL",
            rail_x0 + 1.5,
            rail_cy + 14.5,
            2.2,
            0.0,
            "TS 35",
        );

        // 2. Left end bracket
        if !strip.blocks.is_empty() && strip.end_bracket_left {
            draw_end_bracket_dxf(
                &mut entities,
                &mut bounds,
                cursor_x,
                rail_cy,
                eb_w,
                eb_h,
                max_h,
            );
            cursor_x += eb_w;
        }

        // 3. Terminal blocks + end/partition plates in true 1:1 mm scale
        let pos_label_y = rail_cy - max_h * 0.5 - 5.2;
        let type_label_y = rail_cy - max_h * 0.5 - 10.5;

        for (idx, block) in strip.blocks.iter().enumerate() {
            let bw = f64::from(block.width_tenths_mm) / 10.0;
            let bh = block_heights[idx];
            let y0 = rail_cy - bh * 0.5;
            let y1 = rail_cy + bh * 0.5;

            let is_all_pe = !block.tiers.is_empty()
                && block.tiers.iter().all(|t| t.behavior.is_pe());
            let block_layer = if is_all_pe { "TERMI_PE" } else { "TERMI_BLOCK" };

            draw_rect(
                &mut entities,
                &mut bounds,
                block_layer,
                cursor_x,
                y0,
                cursor_x + bw,
                y1,
            );

            // Order tiers visually from top (highest visual_row) to bottom (lowest visual_row / PE)
            let mut ordered_tiers: Vec<(usize, isize)> = (0..block.tiers.len())
                .map(|t_idx| (t_idx, block.tier_visual_row(t_idx)))
                .collect();
            ordered_tiers.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));

            let n_tiers = ordered_tiers.len().max(1);
            let band_h = bh / (n_tiers as f64);

            for (band_pos, &(t_idx, _vrow)) in ordered_tiers.iter().enumerate() {
                let band_top = y1 - (band_pos as f64) * band_h;
                let band_bot = band_top - band_h;
                let band_cy = (band_top + band_bot) * 0.5;

                if band_pos > 0 {
                    draw_line(
                        &mut entities,
                        &mut bounds,
                        block_layer,
                        cursor_x,
                        band_top,
                        cursor_x + bw,
                        band_top,
                    );
                }

                let tier = &block.tiers[t_idx];
                let lbl = if !tier.computed_label.is_empty() {
                    tier.computed_label.as_str()
                } else if tier.behavior.is_pe() {
                    "PE"
                } else {
                    ""
                };

                if !lbl.is_empty() {
                    let lbl_layer = if tier.behavior.is_pe() {
                        "TERMI_PE"
                    } else {
                        "TERMI_LABEL"
                    };
                    let rotate_vertical = bw < 7.5 && lbl.len() > 2;
                    let text_h = if rotate_vertical {
                        (bw * 0.52).clamp(1.6, 2.8)
                    } else {
                        let max_char_w = (bw - 0.8) / (lbl.len().max(1) as f64 * 0.82);
                        max_char_w.min(band_h * 0.45).clamp(1.5, 3.0)
                    };
                    let tx = cursor_x + bw * 0.5 + if rotate_vertical { text_h * 0.4 } else { -(lbl.len() as f64 * text_h * 0.35) };
                    let ty = band_cy - if rotate_vertical { lbl.len() as f64 * text_h * 0.35 } else { text_h * 0.5 };
                    draw_text(
                        &mut entities,
                        &mut bounds,
                        lbl_layer,
                        tx,
                        ty,
                        text_h,
                        if rotate_vertical { 90.0 } else { 0.0 },
                        lbl,
                    );
                }
            }

            // Position index below block
            let pos_str = format!("{}", idx + 1);
            let pos_h = (bw * 0.42).clamp(1.5, 2.4);
            draw_text(
                &mut entities,
                &mut bounds,
                "TERMI_LABEL",
                cursor_x + bw * 0.5 - (pos_str.len() as f64 * pos_h * 0.35),
                pos_label_y,
                pos_h,
                0.0,
                &pos_str,
            );

            // Vertical (90° CCW) short type designation below position index
            let short_type = block.short_type_name();
            let type_h = (bw * 0.42).clamp(1.6, 2.3);
            draw_text(
                &mut entities,
                &mut bounds,
                "TERMI_LABEL",
                cursor_x + bw * 0.5 + type_h * 0.4,
                type_label_y - (short_type.len() as f64 * type_h * 0.72),
                type_h,
                90.0,
                short_type,
            );

            cursor_x += bw;

            // End / partition plate right after block `idx`
            if let Some(plate) = end_plates.iter().find(|p| p.after_block_idx == idx) {
                let pw = f64::from(plate.width_tenths_mm) / 10.0;
                let next_h = block_heights.get(idx + 1).copied().unwrap_or(bh);
                let ph = bh.max(next_h);
                let py0 = rail_cy - ph * 0.5;
                let py1 = rail_cy + ph * 0.5;
                draw_rect(
                    &mut entities,
                    &mut bounds,
                    "TERMI_PLATE",
                    cursor_x,
                    py0,
                    cursor_x + pw,
                    py1,
                );
                // Diagonal identifier stroke across end plate
                draw_line(
                    &mut entities,
                    &mut bounds,
                    "TERMI_PLATE",
                    cursor_x,
                    py0,
                    cursor_x + pw,
                    py1,
                );
                cursor_x += pw;
            }
        }

        // 4. Right end bracket
        if !strip.blocks.is_empty() && strip.end_bracket_right {
            draw_end_bracket_dxf(
                &mut entities,
                &mut bounds,
                cursor_x,
                rail_cy,
                eb_w,
                eb_h,
                max_h,
            );
            cursor_x += eb_w;
        }

        // 5. Overall width & max height dimension lines (`TERMI_DIM`) + strip header
        let dim_x_end = if strip.blocks.is_empty() {
            strip_total_mm
        } else {
            cursor_x
        };
        let top_edge = rail_cy + max_h * 0.5;
        let bot_edge = rail_cy - max_h * 0.5;
        let dim_y = top_edge + 12.0;

        // Horizontal total-width dimension
        draw_line(&mut entities, &mut bounds, "TERMI_DIM", 0.0, top_edge + 1.5, 0.0, dim_y + 2.5);
        draw_line(&mut entities, &mut bounds, "TERMI_DIM", dim_x_end, top_edge + 1.5, dim_x_end, dim_y + 2.5);
        draw_line(&mut entities, &mut bounds, "TERMI_DIM", 0.0, dim_y, dim_x_end, dim_y);
        draw_dim_tick(&mut entities, &mut bounds, 0.0, dim_y);
        draw_dim_tick(&mut entities, &mut bounds, dim_x_end, dim_y);
        let w_text = format!("{strip_total_mm:.1} mm");
        draw_text(
            &mut entities,
            &mut bounds,
            "TERMI_DIM",
            dim_x_end * 0.5 - (w_text.len() as f64 * 1.1),
            dim_y + 1.6,
            3.0,
            0.0,
            &w_text,
        );

        // Vertical max-height dimension on the right
        let dim_x = dim_x_end + rail_overhang + 8.0;
        draw_line(&mut entities, &mut bounds, "TERMI_DIM", dim_x_end + 1.5, bot_edge, dim_x + 2.5, bot_edge);
        draw_line(&mut entities, &mut bounds, "TERMI_DIM", dim_x_end + 1.5, top_edge, dim_x + 2.5, top_edge);
        draw_line(&mut entities, &mut bounds, "TERMI_DIM", dim_x, bot_edge, dim_x, top_edge);
        draw_dim_tick(&mut entities, &mut bounds, dim_x, bot_edge);
        draw_dim_tick(&mut entities, &mut bounds, dim_x, top_edge);
        let h_text = format!("{max_h:.1} mm");
        draw_text(
            &mut entities,
            &mut bounds,
            "TERMI_DIM",
            dim_x - 1.8,
            rail_cy - (h_text.len() as f64 * 1.1),
            3.0,
            90.0,
            &h_text,
        );

        // Strip header above width dimension line
        let header = format!(
            "{} | {} Klemmen | {:.1} x {:.1} mm | TS 35",
            strip.name,
            strip.blocks.len(),
            strip_total_mm,
            max_h
        );
        draw_text(
            &mut entities,
            &mut bounds,
            "TERMI_LABEL",
            0.0,
            dim_y + 8.0,
            4.2,
            0.0,
            &header,
        );

        // Advance downward for next terminal strip in the project
        let strip_step = max_h + strip_spacing_mm;
        rail_cy -= strip_step;
    }

    // 6. Project title / legend block (`Schriftfeld`) below all strips
    let title_top = bounds.min_y - 14.0;
    let title_bot = title_top - 22.0;
    let title_right = bounds.max_x.max(160.0);
    draw_rect(
        &mut entities,
        &mut bounds,
        "TERMI_DIM",
        -15.0,
        title_bot,
        title_right,
        title_top,
    );
    draw_text(
        &mut entities,
        &mut bounds,
        "TERMI_LABEL",
        -11.0,
        title_top - 7.5,
        4.0,
        0.0,
        &format!(
            "TermiGrid Aufbaubild (1:1 mm) - Projekt: {} | Leisten: {}",
            project.project_name,
            project.strips.len()
        ),
    );
    draw_text(
        &mut entities,
        &mut bounds,
        "TERMI_LABEL",
        -11.0,
        title_top - 15.5,
        3.0,
        0.0,
        "Massstab 1:1 (1 Einheit = 1 mm) | Hutschiene TS 35 (35.0 mm) | EPLAN Electric P8 Import",
    );

    build_dxf_document(&entities, bounds)
}

/// Writes the 1:1 EPLAN DXF containing all strips of `project` to `path`, returning strip count.
pub fn export_to_dxf(
    path: &Path,
    project: &Project,
    catalog: &TerminalCatalog,
) -> Result<usize, String> {
    let content = export_to_dxf_string(project, catalog);
    std::fs::write(path, content.as_bytes()).map_err(|e| format!("DXF write failed: {e}"))?;
    Ok(project.strips.len())
}

/// Computes `(total_width_mm, max_front_height_mm)` of a single strip for EPLAN / CAD layout display.
pub fn strip_envelope_mm(strip: &TerminalStrip, catalog: &TerminalCatalog) -> (f64, f64) {
    let eb = find_end_bracket(&strip.end_bracket_id);
    let eb_w = f64::from(eb.width_tenths_mm) / 10.0;
    let eb_h = f64::from(eb.height_tenths_mm) / 10.0;
    let plates_w: f64 = calculate_end_plates(strip, catalog)
        .iter()
        .map(|p| f64::from(p.width_tenths_mm) / 10.0)
        .sum();
    let total_w = f64::from(strip.total_width_mm()) + plates_w + f64::from(strip.end_brackets_width_mm());
    let _ = eb_w;
    let max_block_h = strip
        .blocks
        .iter()
        .filter_map(|b| catalog.find_by_id(&b.spec_id))
        .map(|sp| f64::from(sp.effective_height_tenths_mm()) / 10.0)
        .fold(0.0_f64, f64::max);
    let max_h = if strip.blocks.is_empty() {
        35.0
    } else if strip.end_bracket_count() > 0 {
        max_block_h.max(eb_h).max(35.0)
    } else {
        max_block_h.max(35.0)
    };
    (total_w, max_h)
}

fn draw_end_bracket_dxf(
    out: &mut String,
    bounds: &mut Bounds,
    x0: f64,
    rail_cy: f64,
    w: f64,
    h: f64,
    max_h: f64,
) {
    let y0 = rail_cy - h * 0.5;
    let y1 = rail_cy + h * 0.5;
    draw_rect(out, bounds, "TERMI_BRACKET", x0, y0, x0 + w, y1);
    // Clamping foot detail lines
    draw_line(out, bounds, "TERMI_BRACKET", x0, y0 + 2.5, x0 + w, y0 + 2.5);
    draw_line(out, bounds, "TERMI_BRACKET", x0, y1 - 2.5, x0 + w, y1 - 2.5);
    draw_text(
        out,
        bounds,
        "TERMI_LABEL",
        x0 + w * 0.5 - 1.5,
        rail_cy - 1.1,
        2.0,
        0.0,
        "EH",
    );
    draw_text(
        out,
        bounds,
        "TERMI_LABEL",
        x0 + w * 0.5 - 1.8,
        rail_cy - max_h * 0.5 - 5.2,
        1.6,
        0.0,
        &format!("{w:.1}"),
    );
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    initialized: bool,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            initialized: false,
            min_x: 0.0,
            min_y: 0.0,
            max_x: 100.0,
            max_y: 50.0,
        }
    }
}

impl Bounds {
    fn include(&mut self, x: f64, y: f64) {
        if !self.initialized {
            self.initialized = true;
            self.min_x = x;
            self.max_x = x;
            self.min_y = y;
            self.max_y = y;
        } else {
            self.min_x = self.min_x.min(x);
            self.max_x = self.max_x.max(x);
            self.min_y = self.min_y.min(y);
            self.max_y = self.max_y.max(y);
        }
    }
}

fn draw_rect(
    out: &mut String,
    bounds: &mut Bounds,
    layer: &str,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
) {
    draw_line(out, bounds, layer, x0, y0, x1, y0);
    draw_line(out, bounds, layer, x1, y0, x1, y1);
    draw_line(out, bounds, layer, x1, y1, x0, y1);
    draw_line(out, bounds, layer, x0, y1, x0, y0);
}

fn draw_dim_tick(out: &mut String, bounds: &mut Bounds, x: f64, y: f64) {
    draw_line(out, bounds, "TERMI_DIM", x - 1.4, y - 1.4, x + 1.4, y + 1.4);
}

fn draw_line(
    out: &mut String,
    bounds: &mut Bounds,
    layer: &str,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
) {
    bounds.include(x0, y0);
    bounds.include(x1, y1);
    let _ = writeln!(
        out,
        "  0\nLINE\n  8\n{layer}\n 10\n{x0:.3}\n 20\n{y0:.3}\n 30\n0.0\n 11\n{x1:.3}\n 21\n{y1:.3}\n 31\n0.0"
    );
}

#[allow(clippy::too_many_arguments)]
fn draw_text(
    out: &mut String,
    bounds: &mut Bounds,
    layer: &str,
    x: f64,
    y: f64,
    height: f64,
    rotation_deg: f64,
    text: &str,
) {
    let clean = sanitize_dxf_text(text);
    bounds.include(x, y);
    let _ = writeln!(
        out,
        "  0\nTEXT\n  8\n{layer}\n 10\n{x:.3}\n 20\n{y:.3}\n 30\n0.0\n 40\n{height:.3}\n  1\n{clean}\n 50\n{rotation_deg:.1}\n  7\nSTANDARD"
    );
}

fn sanitize_dxf_text(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\n' | '\r' => ' ',
            '²' => '2',
            '³' => '3',
            _ => c,
        })
        .collect()
}

fn build_dxf_document(entities: &str, bounds: Bounds) -> String {
    let mut doc = String::with_capacity(entities.len() + 2048);
    let layers = [
        ("0", 7),
        ("TERMI_RAIL", 8),
        ("TERMI_BRACKET", 9),
        ("TERMI_BLOCK", 7),
        ("TERMI_PE", 3),
        ("TERMI_PLATE", 30),
        ("TERMI_LABEL", 7),
        ("TERMI_DIM", 1),
    ];

    let _ = writeln!(
        doc,
        "  0\nSECTION\n  2\nHEADER\n  9\n$ACADVER\n  1\nAC1009\n  9\n$MEASUREMENT\n 70\n1\n  9\n$INSUNITS\n 70\n4\n  9\n$EXTMIN\n 10\n{:.3}\n 20\n{:.3}\n 30\n0.0\n  9\n$EXTMAX\n 10\n{:.3}\n 20\n{:.3}\n 30\n0.0\n  0\nENDSEC",
        bounds.min_x, bounds.min_y, bounds.max_x, bounds.max_y
    );

    let _ = writeln!(
        doc,
        "  0\nSECTION\n  2\nTABLES\n  0\nTABLE\n  2\nLTYPE\n 70\n1\n  0\nLTYPE\n  2\nCONTINUOUS\n 70\n0\n  3\nSolid line\n 72\n65\n 73\n0\n 40\n0.0\n  0\nENDTAB\n  0\nTABLE\n  2\nLAYER\n 70\n{}",
        layers.len()
    );
    for (name, color) in layers {
        let _ = writeln!(
            doc,
            "  0\nLAYER\n  2\n{name}\n 70\n0\n 62\n{color}\n  6\nCONTINUOUS"
        );
    }
    let _ = writeln!(
        doc,
        "  0\nENDTAB\n  0\nTABLE\n  2\nSTYLE\n 70\n1\n  0\nSTYLE\n  2\nSTANDARD\n 70\n0\n 40\n0.0\n 41\n1.0\n 50\n0.0\n 71\n0\n 42\n2.5\n  3\ntxt\n  4\n\n  0\nENDTAB\n  0\nENDSEC"
    );

    let _ = writeln!(doc, "  0\nSECTION\n  2\nENTITIES");
    doc.push_str(entities);
    let _ = writeln!(doc, "  0\nENDSEC\n  0\nEOF");
    doc
}
