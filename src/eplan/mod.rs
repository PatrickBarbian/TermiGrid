pub mod config;
pub mod csv_import;
pub mod roundtrip_export;

use std::ffi::OsString;
use std::path::PathBuf;

pub use config::{default_config_path, normalize_article_key, AppConfig, EplanConfig};
pub use csv_import::{
    import_eplan_csv_path, import_eplan_csv_to_project, parse_eplan_csv_str, EplanCsvImportReport,
};
pub use roundtrip_export::{
    build_eplan_accessory_rows, export_eplan_roundtrip_package, generate_eplan_bridge_cs_script,
    render_eplan_accessory_csv, render_eplan_accessory_xml, EplanAccessoryRow,
    EplanRoundtripReport,
};

/// Command-line startup arguments supported by the desktop executable.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StartupArgs {
    /// Optional EPLAN CSV file path passed via `--import-eplan-csv <path>` or `--import-eplan-csv=<path>`.
    pub import_eplan_csv: Option<PathBuf>,
}

impl StartupArgs {
    /// Parses CLI arguments from the current process environment.
    pub fn from_env() -> Self {
        Self::from_args(std::env::args_os())
    }

    /// Parses CLI arguments from an arbitrary iterator of OS strings (skipping argv[0]).
    pub fn from_args<I>(args: I) -> Self
    where
        I: IntoIterator<Item = OsString>,
    {
        let mut parsed = Self::default();
        let mut iter = args.into_iter().skip(1);
        while let Some(arg) = iter.next() {
            let s = arg.to_string_lossy();
            if s == "--import-eplan-csv" {
                if let Some(next_val) = iter.next()
                    && !next_val.is_empty()
                {
                    parsed.import_eplan_csv = Some(PathBuf::from(next_val));
                }
            } else if let Some(rest) = s.strip_prefix("--import-eplan-csv=")
                && !rest.trim().is_empty()
            {
                parsed.import_eplan_csv = Some(PathBuf::from(rest));
            }
        }
        parsed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::TerminalCatalog;
    use crate::model::{Project, TerminalStrip};

    #[test]
    fn vendor_prefix_stripping_and_reattaching_work_for_all_brands() {
        let cfg = EplanConfig::default();

        assert_eq!(
            cfg.strip_vendor_prefix("PXC.3210567"),
            (Some("Phoenix Contact"), "3210567".to_string())
        );
        assert_eq!(
            cfg.strip_vendor_prefix("WAG.2002-3217"),
            (Some("WAGO"), "2002-3217".to_string())
        );
        assert_eq!(
            cfg.strip_vendor_prefix("WDM.1050000000"),
            (Some("Weidmüller"), "1050000000".to_string())
        );
        // Case-insensitive and alternate prefix fallback
        assert_eq!(
            cfg.strip_vendor_prefix("pxc.3212044"),
            (Some("Phoenix Contact"), "3212044".to_string())
        );
        assert_eq!(
            cfg.strip_vendor_prefix("WAGO.2002-1201"),
            (Some("WAGO"), "2002-1201".to_string())
        );

        // Re-attaching
        assert_eq!(
            cfg.attach_vendor_prefix("Phoenix Contact", "3212044"),
            "PXC.3212044"
        );
        assert_eq!(
            cfg.attach_vendor_prefix("WAGO", "2002-1291"),
            "WAG.2002-1291"
        );
        assert_eq!(
            cfg.attach_vendor_prefix("Weidmüller", "1050000000"),
            "WDM.1050000000"
        );
        // No double prefixing
        assert_eq!(
            cfg.attach_vendor_prefix("Phoenix Contact", "PXC.3212044"),
            "PXC.3212044"
        );

        // Custom user prefixes
        let custom_cfg = EplanConfig {
            prefix_phoenix: "PHX:".to_string(),
            prefix_wago: "WG.".to_string(),
            prefix_weidmuller: "WEI.".to_string(),
            ..Default::default()
        };
        assert_eq!(
            custom_cfg.strip_vendor_prefix("PHX:3212044"),
            (Some("Phoenix Contact"), "3212044".to_string())
        );
        assert_eq!(
            custom_cfg.attach_vendor_prefix("WAGO", "2002-1291"),
            "WG.2002-1291"
        );
    }

    #[test]
    fn parses_eplan_csv_with_bom_prefixes_levels_and_preserves_order() {
        let catalog = TerminalCatalog::load_embedded();
        let cfg = EplanConfig::default();

        // Includes UTF-8 BOM, multi-level PTTB 2,5 (Etage 1 & 2), WAGO 2002-3227,
        // empty Menge defaulting to 1, and explicit numbering preserved.
        let csv = "\u{feff}BMK;Klemmennummer;Etage;Artikelnummer;Typnummer;Menge\n\
                   =NSV+A01-X1;10;1;PXC.3210567;PTTB 2,5;\n\
                   =NSV+A01-X1;11;2;PXC.3210567;PTTB 2,5;1\n\
                   =NSV+A01-X1;PE;;PXC.3210596;PTTB 2,5-PE;1\n\
                   =NSV+A01-X2;PE;1;WAG.2002-3227;2002-3227;1\n\
                   =NSV+A01-X2;L1;2;WAG.2002-3227;2002-3227;1\n\
                   =NSV+A01-X2;L2;3;WAG.2002-3227;2002-3227;1\n";

        let report = parse_eplan_csv_str(csv, &catalog, &cfg).expect("parse CSV");
        assert_eq!(report.strips.len(), 2);

        let x1 = &report.strips[0];
        assert_eq!(x1.name, "=NSV+A01-X1");
        assert_eq!(x1.blocks.len(), 2);
        assert_eq!(x1.blocks[0].spec_id, "phoenix-pttb-25");
        assert_eq!(x1.blocks[0].tiers[0].computed_label, "10");
        assert_eq!(x1.blocks[0].tiers[1].computed_label, "11");
        assert_eq!(x1.blocks[1].spec_id, "phoenix-pttb-25-pe");
        assert_eq!(x1.end_bracket_id, "phoenix-clipfix-35-5");

        let x2 = &report.strips[1];
        assert_eq!(x2.name, "=NSV+A01-X2");
        assert_eq!(x2.blocks.len(), 1);
        assert_eq!(x2.blocks[0].spec_id, "wago-2002-3227");
        assert_eq!(x2.blocks[0].tiers[0].computed_label, "PE");
        assert_eq!(x2.blocks[0].tiers[1].computed_label, "L1");
        assert_eq!(x2.blocks[0].tiers[2].computed_label, "L2");
        // Strict brand match for WAGO strip end bracket
        assert!(x2.end_bracket_id.starts_with("wago-"));
    }

    #[test]
    fn parses_english_eplan_csv_headers_and_filters_preexisting_accessories() {
        let catalog = TerminalCatalog::load_embedded();
        let mut cfg = EplanConfig::default();

        let csv = "Device Tag;TerminalNo;Level;ArticleNo;TypeNo;Qty\n\
                   =ANL+ORT-X5;1;;WAG.2002-1201;2002-1201;2\n\
                   =ANL+ORT-X5;;;WAG.249-117;249-117;2\n";

        let report = parse_eplan_csv_str(csv, &catalog, &cfg).expect("parse English CSV");
        assert_eq!(report.strips.len(), 1);
        let strip = &report.strips[0];
        assert_eq!(strip.name, "=ANL+ORT-X5");
        assert_eq!(strip.blocks.len(), 2);
        assert_eq!(strip.blocks[0].tiers[0].computed_label, "1");
        assert_eq!(strip.blocks[1].tiers[0].computed_label, "2");
        assert_eq!(strip.end_bracket_id, "wago-249-117");
        assert_eq!(report.total_preexisting_accessories, 2);

        let project = Project {
            format_version: 1,
            project_name: "Test".to_string(),
            strips: report.strips.clone(),
            custom_specs: Vec::new(),
        };

        // When export_added_accessories_only == true, pre-existing 2x 249-117 are skipped,
        // leaving only the newly calculated end plate WAG.2002-1291!
        cfg.export_added_accessories_only = true;
        let added_rows = build_eplan_accessory_rows(&project, &catalog, &cfg);
        assert_eq!(added_rows.len(), 1);
        assert_eq!(added_rows[0].article_number, "WAG.2002-1291");
        assert_eq!(added_rows[0].quantity, 1);

        // When export_added_accessories_only == false, both WAG.2002-1291 and WAG.249-117 are exported
        cfg.export_added_accessories_only = false;
        let all_rows = build_eplan_accessory_rows(&project, &catalog, &cfg);
        assert_eq!(all_rows.len(), 2);
        assert!(all_rows.iter().any(|r| r.article_number == "WAG.2002-1291" && r.quantity == 1));
        assert!(all_rows.iter().any(|r| r.article_number == "WAG.249-117" && r.quantity == 2));
    }

    #[test]
    fn roundtrip_package_writes_csv_xml_and_sanitized_dxf_with_strict_brands() {
        let catalog = TerminalCatalog::load_embedded();
        let cfg = EplanConfig::default();

        // Deliberately assign wrong-brand Phoenix end bracket to a WAGO strip + normal PT 6 strip
        let pt6 = catalog.find_by_id("phoenix-pt-6").unwrap();
        let wago2 = catalog.find_by_id("wago-2002-1201").unwrap();

        let s1 = TerminalStrip::generate_with_block_count("=NSV+A01-X1", 1, 3, 3, pt6);
        let s2 = TerminalStrip::generate_with_block_count("=NSV+A01-X2", 1, 2, 2, wago2);

        let mut project = Project {
            format_version: 1,
            project_name: "RoundtripTest".to_string(),
            strips: vec![s1, s2],
            custom_specs: Vec::new(),
        };

        let out_dir = std::env::temp_dir().join(format!(
            "termigrid_eplan_roundtrip_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&out_dir);

        let report = export_eplan_roundtrip_package(&out_dir, &mut project, &catalog, &cfg)
            .expect("export roundtrip");

        assert!(report.csv_path.exists());
        assert!(report.xml_path.exists());
        assert_eq!(report.dxf_files.len(), 2);
        assert_eq!(
            report.dxf_files[0].file_name().and_then(|s| s.to_str()),
            Some("NSV_A01_X1.dxf")
        );
        assert_eq!(
            report.dxf_files[1].file_name().and_then(|s| s.to_str()),
            Some("NSV_A01_X2.dxf")
        );

        let csv_txt = std::fs::read_to_string(&report.csv_path).unwrap();
        assert!(csv_txt.starts_with('\u{feff}'));
        assert!(csv_txt.contains("BMK;Artikelnummer;Menge;Bezeichnung"));
        assert!(csv_txt.contains("=NSV+A01-X1;PXC.3212044;1;"));
        assert!(csv_txt.contains("=NSV+A01-X1;PXC.3022276;2;"));
        // WAGO strip MUST get WAGO end bracket (WAG.249-117), NEVER Phoenix PXC.3022276!
        assert!(csv_txt.contains("=NSV+A01-X2;WAG.2002-1291;1;"));
        assert!(csv_txt.contains("=NSV+A01-X2;WAG.249-117;2;"));
        assert!(!csv_txt.contains("=NSV+A01-X2;PXC."));

        let xml_txt = std::fs::read_to_string(&report.xml_path).unwrap();
        assert!(xml_txt.contains("<EplanDeviceList"));
        assert!(xml_txt.contains("PXC.3212044"));
        assert!(xml_txt.contains("WAG.249-117"));

        let _ = std::fs::remove_dir_all(&out_dir);
    }

    #[test]
    fn cli_args_and_config_roundtrip_work() {
        let args = StartupArgs::from_args([
            OsString::from("TermiGrid.exe"),
            OsString::from("--import-eplan-csv"),
            OsString::from(r"C:\EPLAN\Export\klemmen.csv"),
        ]);
        assert_eq!(
            args.import_eplan_csv,
            Some(PathBuf::from(r"C:\EPLAN\Export\klemmen.csv"))
        );

        let args_eq = StartupArgs::from_args([
            OsString::from("TermiGrid.exe"),
            OsString::from("--import-eplan-csv=/tmp/export.csv"),
        ]);
        assert_eq!(
            args_eq.import_eplan_csv,
            Some(PathBuf::from("/tmp/export.csv"))
        );

        let cfg_path = std::env::temp_dir().join(format!(
            "termigrid_eplan_cfg_{}.json",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&cfg_path);
        let mut cfg = AppConfig::default();
        cfg.eplan.prefix_phoenix = "PHX.".to_string();
        cfg.eplan.export_added_accessories_only = false;
        cfg.save_to_path(&cfg_path).unwrap();
        let loaded = AppConfig::load_from_path(&cfg_path).unwrap();
        assert_eq!(loaded, cfg);
        let _ = std::fs::remove_file(&cfg_path);

        let script = generate_eplan_bridge_cs_script(
            Some(std::path::Path::new(r"C:\Apps\TermiGrid.exe")),
            None,
        );
        assert!(script.contains(r#"TermiGridExePath = @"C:\Apps\TermiGrid.exe""#));
        assert!(script.contains("--import-eplan-csv"));
    }
}
