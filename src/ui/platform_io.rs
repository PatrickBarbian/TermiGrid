use std::path::PathBuf;

/// Prompts for a project save path on desktop, or returns a portable Android Downloads path.
pub fn pick_save_project_path(default_filename: &str) -> Option<PathBuf> {
    #[cfg(not(target_os = "android"))]
    {
        rfd::FileDialog::new()
            .set_file_name(default_filename)
            .add_filter("TermiGrid Project", &["termi", "json"])
            .save_file()
    }
    #[cfg(target_os = "android")]
    {
        let dir = android_export_dir();
        let _ = std::fs::create_dir_all(&dir);
        Some(dir.join(default_filename))
    }
}

/// Prompts for a project file to open on desktop, or opens the latest `.termi`/`.json` on Android.
pub fn pick_open_project_path() -> Option<PathBuf> {
    #[cfg(not(target_os = "android"))]
    {
        rfd::FileDialog::new()
            .add_filter("TermiGrid Project", &["termi", "json"])
            .pick_file()
    }
    #[cfg(target_os = "android")]
    {
        let dir = android_export_dir();
        let entries = std::fs::read_dir(&dir).ok()?;
        entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .find(|p| {
                p.extension()
                    .and_then(|s| s.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("termi") || ext.eq_ignore_ascii_case("json"))
            })
    }
}

/// Prompts for an `.xlsx` export path on desktop, or returns an Android Downloads path.
pub fn pick_export_xlsx_path(default_filename: &str) -> Option<PathBuf> {
    #[cfg(not(target_os = "android"))]
    {
        rfd::FileDialog::new()
            .set_file_name(default_filename)
            .add_filter("Excel Workbook", &["xlsx"])
            .save_file()
    }
    #[cfg(target_os = "android")]
    {
        let dir = android_export_dir();
        let _ = std::fs::create_dir_all(&dir);
        Some(dir.join(default_filename))
    }
}

/// Prompts for a `.csv` export path on desktop, or returns an Android Downloads path.
pub fn pick_export_csv_path(default_filename: &str) -> Option<PathBuf> {
    #[cfg(not(target_os = "android"))]
    {
        rfd::FileDialog::new()
            .set_file_name(default_filename)
            .add_filter("CSV File", &["csv"])
            .save_file()
    }
    #[cfg(target_os = "android")]
    {
        let dir = android_export_dir();
        let _ = std::fs::create_dir_all(&dir);
        Some(dir.join(default_filename))
    }
}

/// Prompts for a `.dxf` EPLAN CAD export path on desktop, or returns an Android Downloads path.
pub fn pick_export_dxf_path(default_filename: &str) -> Option<PathBuf> {
    #[cfg(not(target_os = "android"))]
    {
        rfd::FileDialog::new()
            .set_file_name(default_filename)
            .add_filter("AutoCAD / EPLAN DXF (1:1 mm)", &["dxf"])
            .save_file()
    }
    #[cfg(target_os = "android")]
    {
        let dir = android_export_dir();
        let _ = std::fs::create_dir_all(&dir);
        Some(dir.join(default_filename))
    }
}

/// Prompts for an EPLAN PDF parts list to import on desktop, or picks the newest `.pdf` in Android Downloads.
pub fn pick_import_pdf_path() -> Option<PathBuf> {
    #[cfg(not(target_os = "android"))]
    {
        rfd::FileDialog::new()
            .add_filter("PDF-Stückliste", &["pdf"])
            .pick_file()
    }
    #[cfg(target_os = "android")]
    {
        let dir = android_export_dir();
        let entries = std::fs::read_dir(&dir).ok()?;
        entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .find(|p| {
                p.extension()
                    .and_then(|s| s.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("pdf"))
            })
    }
}

#[cfg(target_os = "android")]
fn android_export_dir() -> PathBuf {
    PathBuf::from("/sdcard/Download/TermiGrid")
}
