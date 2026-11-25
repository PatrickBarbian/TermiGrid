use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use crate::catalog::accessories::{
    calculate_end_plates, dominant_strip_manufacturer, extract_accessory_article_number,
    extract_comb_jumper_article_number, strict_end_bracket_for_strip,
};
use crate::catalog::TerminalCatalog;
use crate::export::dxf::export_all_strips_to_eplan_dxf_dir;
use crate::eplan::config::{normalize_article_key, EplanConfig};
use crate::model::bridging::calculate_bridges;
use crate::model::Project;

/// Single aggregated row in `_EPLAN_Zubehoer_Import.csv` and `_EPLAN_Zubehoer_Import.xml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EplanAccessoryRow {
    pub bmk: String,
    pub article_number: String,
    pub clean_article: String,
    pub manufacturer: String,
    pub quantity: usize,
    pub designation: String,
}

/// Result summary after generating a complete EPLAN roundtrip return-import package.
#[derive(Debug, Clone)]
pub struct EplanRoundtripReport {
    pub output_dir: PathBuf,
    pub csv_path: PathBuf,
    pub xml_path: PathBuf,
    pub dxf_dir: PathBuf,
    pub dxf_files: Vec<PathBuf>,
    pub accessory_rows: Vec<EplanAccessoryRow>,
    pub total_accessory_units: usize,
}

#[derive(Default)]
struct AggregatedAccessory {
    manufacturer: String,
    clean_article: String,
    quantity: usize,
    designations: Vec<String>,
}

/// Calculates and aggregates EPLAN accessory rows across all non-empty strips in `project`,
/// enforcing strict manufacturer matching per strip and applying `config.export_added_accessories_only`.
pub fn build_eplan_accessory_rows(
    project: &Project,
    catalog: &TerminalCatalog,
    config: &EplanConfig,
) -> Vec<EplanAccessoryRow> {
    let mut rows = Vec::new();

    for strip in &project.strips {
        if strip.blocks.is_empty() {
            continue;
        }

        let dominant_mfg = dominant_strip_manufacturer(&strip.blocks);
        let mut per_strip: BTreeMap<(String, String), AggregatedAccessory> = BTreeMap::new();

        // 1. Automatically calculated end plates & partition plates (strictly block's manufacturer)
        let plates = calculate_end_plates(strip, catalog);
        let block_len = strip.blocks.len();
        for plate in &plates {
            if let Some(ref dom) = dominant_mfg
                && !plate.manufacturer.eq_ignore_ascii_case(dom)
            {
                // Skip foreign plates only if strip has mixed brands; actually each plate
                // already matches its own terminal block's manufacturer cleanly.
            }
            let Some(clean_art) =
                extract_accessory_article_number(&plate.manufacturer, &plate.end_plate)
            else {
                continue;
            };
            let norm_key = normalize_article_key(&clean_art);
            if norm_key.is_empty() {
                continue;
            }
            let is_strip_end = plate.after_block_idx + 1 >= block_len;
            let short_name = plate
                .end_plate
                .split(" (")
                .next()
                .unwrap_or(&plate.end_plate)
                .trim();
            let label = if is_strip_end {
                format!("Endplatte {short_name}")
            } else {
                format!("Zwischenplatte {short_name}")
            };

            let entry = per_strip
                .entry((plate.manufacturer.clone(), norm_key))
                .or_insert_with(|| AggregatedAccessory {
                    manufacturer: plate.manufacturer.clone(),
                    clean_article: clean_art,
                    quantity: 0,
                    designations: Vec::new(),
                });
            entry.quantity += 1;
            if !entry.designations.contains(&label) {
                entry.designations.push(label);
            }
        }

        // 2. DIN-rail end brackets (strictly matched to strip's terminal manufacturer)
        let bracket_qty = strip.end_bracket_count();
        if bracket_qty > 0 {
            let eb = strict_end_bracket_for_strip(strip);
            let norm_key = normalize_article_key(eb.article_number);
            if !norm_key.is_empty() {
                let label = format!("Endhalter {}", eb.name);
                let entry = per_strip
                    .entry((eb.manufacturer.to_string(), norm_key))
                    .or_insert_with(|| AggregatedAccessory {
                        manufacturer: eb.manufacturer.to_string(),
                        clean_article: eb.article_number.to_string(),
                        quantity: 0,
                        designations: Vec::new(),
                    });
                entry.quantity += bracket_qty;
                if !entry.designations.contains(&label) {
                    entry.designations.push(label);
                }
            }
        }

        // 3. Comb jumpers (calculated from Bridge Mode potentials)
        let bridges = calculate_bridges(strip, catalog);
        for seg in &bridges.comb_segments {
            let Some(blk) = strip.blocks.get(seg.start_block_idx) else {
                continue;
            };
            if let Some(clean_art) =
                extract_comb_jumper_article_number(&blk.manufacturer, &seg.part_hint)
            {
                let norm_key = normalize_article_key(&clean_art);
                if !norm_key.is_empty() {
                    let short_hint = seg
                        .part_hint
                        .split(" (")
                        .next()
                        .unwrap_or(&seg.part_hint)
                        .trim();
                    let label = format!("Kammbrücke {short_hint}");
                    let entry = per_strip
                        .entry((blk.manufacturer.clone(), norm_key))
                        .or_insert_with(|| AggregatedAccessory {
                            manufacturer: blk.manufacturer.clone(),
                            clean_article: clean_art,
                            quantity: 0,
                            designations: Vec::new(),
                        });
                    entry.quantity += 1;
                    if !entry.designations.contains(&label) {
                        entry.designations.push(label);
                    }
                }
            }
        }

        // 4. Busbar carriers (only when exporting all accessories, since carriers sit on the DIN rail)
        if !config.export_added_accessories_only {
            for blk in &strip.blocks {
                if let Some(sp) = catalog.find_by_id(&blk.spec_id)
                    && sp.is_busbar_carrier()
                    && let Some(clean_art) = sp.effective_article_number()
                {
                    let norm_key = normalize_article_key(clean_art);
                    if !norm_key.is_empty() {
                        let label = format!("Sammelschienenträger {}", blk.short_type_name());
                        let entry = per_strip
                            .entry((blk.manufacturer.clone(), norm_key))
                            .or_insert_with(|| AggregatedAccessory {
                                manufacturer: blk.manufacturer.clone(),
                                clean_article: clean_art.to_string(),
                                quantity: 0,
                                designations: Vec::new(),
                            });
                        entry.quantity += 1;
                        if !entry.designations.contains(&label) {
                            entry.designations.push(label);
                        }
                    }
                }
            }
        }

        // Subtract pre-existing EPLAN accessories if `export_added_accessories_only == true`
        for ((_mfg, norm_key), agg) in per_strip {
            let preexisting = if config.export_added_accessories_only {
                strip
                    .eplan_preexisting_accessories
                    .get(&norm_key)
                    .copied()
                    .unwrap_or(0)
            } else {
                0
            };
            let net_qty = agg.quantity.saturating_sub(preexisting);
            if net_qty == 0 {
                continue;
            }

            let eplan_article = config.attach_vendor_prefix(&agg.manufacturer, &agg.clean_article);
            let designation = combine_designations(&agg.designations);
            rows.push(EplanAccessoryRow {
                bmk: strip.name.clone(),
                article_number: eplan_article,
                clean_article: agg.clean_article,
                manufacturer: agg.manufacturer,
                quantity: net_qty,
                designation,
            });
        }
    }

    rows
}

fn combine_designations(parts: &[String]) -> String {
    if parts.is_empty() {
        return String::new();
    }
    if parts.len() == 1 {
        return parts[0].clone();
    }
    let has_end = parts.iter().any(|p| p.starts_with("Endplatte "));
    let has_part = parts.iter().any(|p| p.starts_with("Zwischenplatte "));
    if has_end && has_part && parts.len() == 2 {
        let suffix = parts[0]
            .strip_prefix("Endplatte ")
            .or_else(|| parts[0].strip_prefix("Zwischenplatte "))
            .unwrap_or(&parts[0]);
        return format!("End-/Zwischenplatte {suffix}");
    }
    parts.join(" + ")
}

/// Renders `_EPLAN_Zubehoer_Import.csv` (UTF-8 with BOM, configured delimiter).
pub fn render_eplan_accessory_csv(rows: &[EplanAccessoryRow], delimiter: char) -> String {
    let mut out = String::from("\u{feff}");
    let sep = if delimiter == '\0' { ';' } else { delimiter };
    let _ = writeln!(
        out,
        "BMK{sep}Artikelnummer{sep}Menge{sep}Bezeichnung"
    );
    for row in rows {
        let bmk = csv_escape(&row.bmk, sep);
        let art = csv_escape(&row.article_number, sep);
        let qty = row.quantity;
        let desc = csv_escape(&row.designation, sep);
        let _ = writeln!(out, "{bmk}{sep}{art}{sep}{qty}{sep}{desc}");
    }
    out
}

/// Renders `_EPLAN_Zubehoer_Import.xml` in UTF-8 EPLAN Device List XML format.
pub fn render_eplan_accessory_xml(rows: &[EplanAccessoryRow]) -> String {
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    out.push_str("<EplanDeviceList generator=\"TermiGrid\">\n");
    for row in rows {
        let bmk = xml_escape(&row.bmk);
        let art = xml_escape(&row.article_number);
        let qty = row.quantity;
        let desc = xml_escape(&row.designation);
        let mfg = xml_escape(&row.manufacturer);
        let _ = writeln!(
            out,
            "  <Device BMK=\"{bmk}\" Artikelnummer=\"{art}\" Menge=\"{qty}\" Bezeichnung=\"{desc}\" Hersteller=\"{mfg}\">"
        );
        let _ = writeln!(out, "    <BMK>{bmk}</BMK>");
        let _ = writeln!(out, "    <Artikelnummer>{art}</Artikelnummer>");
        let _ = writeln!(out, "    <Menge>{qty}</Menge>");
        let _ = writeln!(out, "    <Bezeichnung>{desc}</Bezeichnung>");
        let _ = writeln!(out, "    <Hersteller>{mfg}</Hersteller>");
        out.push_str("  </Device>\n");
    }
    out.push_str("</EplanDeviceList>\n");
    out
}

/// Executes the 1-click EPLAN roundtrip package export into `output_dir`:
/// 1. Prepares all strips (updates end plates, labels, and strictly matched end brackets).
/// 2. Writes `_EPLAN_Zubehoer_Import.csv` and `_EPLAN_Zubehoer_Import.xml`.
/// 3. Exports each terminal strip as `<Sanitized_BMK>.dxf` inside `output_dir/DXF/`.
pub fn export_eplan_roundtrip_package(
    output_dir: &Path,
    project: &mut Project,
    catalog: &TerminalCatalog,
    config: &EplanConfig,
) -> Result<EplanRoundtripReport, String> {
    if project.strips.is_empty() {
        return Err("Keine Klemmleisten im Projekt vorhanden.".to_string());
    }

    // 1. Run accessory calculation & enforce strict manufacturer matching across all strips
    for strip in &mut project.strips {
        if strip.blocks.is_empty() {
            continue;
        }
        let strict_eb = strict_end_bracket_for_strip(strip);
        strip.end_bracket_id = strict_eb.id.to_string();
        strip.end_plates_generated = true;
        strip.recalculate_labels();
    }

    std::fs::create_dir_all(output_dir)
        .map_err(|e| format!("Exportordner konnte nicht erstellt werden: {e}"))?;

    let accessory_rows = build_eplan_accessory_rows(project, catalog, config);
    let total_accessory_units = accessory_rows.iter().map(|r| r.quantity).sum();

    // 2. Write _EPLAN_Zubehoer_Import.csv and .xml
    let csv_path = output_dir.join("_EPLAN_Zubehoer_Import.csv");
    let csv_content = render_eplan_accessory_csv(&accessory_rows, config.csv_delimiter);
    std::fs::write(&csv_path, csv_content.as_bytes())
        .map_err(|e| format!("Fehler beim Schreiben von _EPLAN_Zubehoer_Import.csv: {e}"))?;

    let xml_path = output_dir.join("_EPLAN_Zubehoer_Import.xml");
    let xml_content = render_eplan_accessory_xml(&accessory_rows);
    std::fs::write(&xml_path, xml_content.as_bytes())
        .map_err(|e| format!("Fehler beim Schreiben von _EPLAN_Zubehoer_Import.xml: {e}"))?;

    // 3. Export individual 1:1 DXF files into /DXF/<Sanitized_BMK>.dxf
    let dxf_dir = output_dir.join("DXF");
    let dxf_files = export_all_strips_to_eplan_dxf_dir(&dxf_dir, project, catalog)?;

    Ok(EplanRoundtripReport {
        output_dir: output_dir.to_path_buf(),
        csv_path,
        xml_path,
        dxf_dir,
        dxf_files,
        accessory_rows,
        total_accessory_units,
    })
}

/// Generates a ready-to-use `TermiGrid_EPLAN_Bridge.cs` script for EPLAN Electric P8
/// with the running `TermiGrid.exe` path automatically pre-filled.
pub fn generate_eplan_bridge_cs_script(
    exe_path: Option<&Path>,
    default_export_dir: Option<&Path>,
) -> String {
    let resolved_exe = exe_path
        .map(|p| p.display().to_string())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| r"C:\Tools\TermiGrid\TermiGrid.exe".to_string())
        .replace('"', "\"\"");

    let resolved_dir = default_export_dir
        .map(|p| p.display().to_string())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_default()
        .replace('"', "\"\"");

    format!(
        r#"// TermiGrid <-> EPLAN Electric P8 Roundtrip Bridge Script
// Automatisch generiert von TermiGrid.
// In EPLAN unter Dienstprogramme -> Scripte -> Ausführen laden.

using System;
using System.Diagnostics;
using System.IO;
using System.Windows.Forms;
using Eplan.EplApi.ApplicationFramework;
using Eplan.EplApi.Scripting;

public class TermiGridEplanBridge
{{
    private const string TermiGridExePath = @"{resolved_exe}";
    private const string DefaultRoundtripDir = @"{resolved_dir}";

    [Start]
    public void RunTermiGridRoundtrip()
    {{
        try
        {{
            if (!File.Exists(TermiGridExePath))
            {{
                MessageBox.Show(
                    "TermiGrid.exe wurde nicht gefunden:\n" + TermiGridExePath,
                    "TermiGrid EPLAN Bridge",
                    MessageBoxButtons.OK,
                    MessageBoxIcon.Warning
                );
                return;
            }}

            string csvPath = SelectOrFindExportCsv();
            if (string.IsNullOrEmpty(csvPath))
            {{
                return;
            }}

            var psi = new ProcessStartInfo
            {{
                FileName = TermiGridExePath,
                Arguments = "--import-eplan-csv \"" + csvPath + "\"",
                UseShellExecute = false
            }};
            Process.Start(psi);
        }}
        catch (Exception ex)
        {{
            MessageBox.Show(
                "Fehler beim Starten von TermiGrid:\n" + ex.Message,
                "TermiGrid EPLAN Bridge",
                MessageBoxButtons.OK,
                MessageBoxIcon.Error
            );
        }}
    }}

    private static string SelectOrFindExportCsv()
    {{
        using (var dlg = new OpenFileDialog())
        {{
            dlg.Title = "EPLAN Klemmen-Export CSV für TermiGrid auswählen";
            dlg.Filter = "EPLAN CSV Export (*.csv)|*.csv|Alle Dateien (*.*)|*.*";
            if (!string.IsNullOrEmpty(DefaultRoundtripDir) && Directory.Exists(DefaultRoundtripDir))
            {{
                dlg.InitialDirectory = DefaultRoundtripDir;
            }}
            return dlg.ShowDialog() == DialogResult.OK ? dlg.FileName : null;
        }}
    }}
}}
"#
    )
}

fn csv_escape(field: &str, delimiter: char) -> String {
    if field.contains(delimiter) || field.contains('"') || field.contains('\n') || field.contains('\r') {
        let escaped = field.replace('"', "\"\"");
        format!("\"{escaped}\"")
    } else {
        field.to_string()
    }
}

fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}
