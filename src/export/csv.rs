use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use crate::export::{build_print_rows, ExportOptions};
use crate::model::Project;

/// Escapes a CSV field and wraps it in double quotes so every field is treated as text.
fn csv_quote(value: &str) -> String {
    let escaped = value.replace('"', "\"\"");
    format!("\"{escaped}\"")
}

/// Exports the ordered print sequence to a UTF-8 BOM CSV file with all fields quoted as text.
pub fn export_to_csv(
    path: &Path,
    project: &Project,
    active_strip_index: usize,
    options: &ExportOptions,
    german_headers: bool,
) -> io::Result<usize> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // UTF-8 BOM for seamless Excel import on Windows.
    writer.write_all(&[0xEF, 0xBB, 0xBF])?;

    let sep = options.csv_delimiter;
    let headers = if german_headers {
        [
            "Lfd. Nr.",
            "Klemmleiste",
            "Klemme Pos.",
            "Ebene",
            "Beschriftung",
            "Vollbezeichnung",
            "Hersteller",
            "Klemmentyp",
        ]
    } else {
        [
            "Seq.",
            "Strip",
            "Block Pos.",
            "Tier",
            "Label",
            "Full Designation",
            "Manufacturer",
            "Terminal Type",
        ]
    };

    let header_line = headers
        .iter()
        .map(|h| csv_quote(h))
        .collect::<Vec<_>>()
        .join(&sep.to_string());
    writeln!(writer, "{header_line}")?;

    let rows = build_print_rows(project, active_strip_index, options);
    for r in &rows {
        let fields = [
            csv_quote(&r.sequence_no),
            csv_quote(&r.strip_name),
            csv_quote(&r.block_position),
            csv_quote(&r.tier_level),
            csv_quote(&r.label),
            csv_quote(&r.combined_designation),
            csv_quote(&r.manufacturer),
            csv_quote(&r.terminal_type),
        ];
        writeln!(writer, "{}", fields.join(&sep.to_string()))?;
    }

    writer.flush()?;
    Ok(rows.len())
}
