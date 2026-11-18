use std::io::Read;

use termigrid_core::catalog::TerminalCatalog;
use termigrid_core::export::{
    build_print_rows, build_tier_columns, csv::export_to_csv, xlsx::export_to_xlsx, ExportOptions,
    PrintOrder,
};
use termigrid_core::model::{Project, TerminalStrip};

#[test]
fn project_json_roundtrip_preserves_and_normalizes() {
    let catalog = TerminalCatalog::load_embedded();
    let double_ll = catalog.find_by_id("phoenix-pttb-25").unwrap();
    let triple_peln = catalog.find_by_id("weidmuller-dli-25").unwrap();

    let mut strip = TerminalStrip::generate_with_block_count("-X10", 1, 10, 4, double_ll);
    strip.reassign_blocks(&[0, 1], triple_peln);

    let project = Project {
        format_version: 1,
        project_name: "TestCabinet".to_string(),
        strips: vec![strip],
        ..Default::default()
    };

    let json = serde_json::to_string_pretty(&project).unwrap();
    let mut loaded: Project = serde_json::from_str(&json).unwrap();
    loaded.normalize();

    assert_eq!(loaded, project);
    assert_eq!(loaded.strips[0].blocks[0].tiers[1].computed_label, "1");
    assert_eq!(loaded.strips[0].blocks[0].tiers[2].computed_label, "N");
    assert_eq!(loaded.strips[0].blocks[2].tiers[0].computed_label, "3");
    assert_eq!(loaded.strips[0].blocks[2].tiers[1].computed_label, "4");
}

#[test]
fn print_order_by_tier_strip_groups_levels_for_continuous_marking() {
    let catalog = TerminalCatalog::load_embedded();
    let double_ll = catalog.find_by_id("wago-2002-2201").unwrap();
    let strip = TerminalStrip::generate_from_range("-X1", 1, 8, double_ll);

    let project = Project {
        format_version: 1,
        project_name: "Test".to_string(),
        strips: vec![strip],
        ..Default::default()
    };

    let opts = ExportOptions {
        print_order: PrintOrder::ByTierStrip,
        include_pe_labels: false,
        all_strips: false,
        csv_delimiter: ';',
    };
    let rows = build_print_rows(&project, 0, &opts);
    let labels: Vec<&str> = rows.iter().map(|r| r.label.as_str()).collect();
    assert_eq!(labels, vec!["1", "3", "5", "7", "2", "4", "6", "8"]);
}

#[test]
fn mixed_pe_installation_export_groups_aligned_levels_contiguously() {
    let catalog = TerminalCatalog::load_embedded();
    let single = catalog.find_by_id("wago-2002-1201").unwrap();
    let double_ll = catalog.find_by_id("wago-2002-2201").unwrap();
    let install_peln = catalog.find_by_id("wago-2003-7641").unwrap();

    // 2x 2003-7641 (PE/L/N), 2x 2002-2201 (L/L), 2x 2002-1201 (L)
    let mut strip = TerminalStrip::generate_with_block_count("-X1", 1, 10, 6, single);
    strip.reassign_blocks(&[0, 1], install_peln);
    strip.reassign_blocks(&[2, 3], double_ll);

    let project = Project {
        format_version: 1,
        project_name: "MixedPeTest".to_string(),
        strips: vec![strip],
        ..Default::default()
    };

    // Default export is ByTierStrip: all E1 labels first, then all E2 labels
    let rows = build_print_rows(&project, 0, &ExportOptions::default());
    let labels: Vec<&str> = rows.iter().map(|r| r.label.as_str()).collect();
    let tiers: Vec<&str> = rows.iter().map(|r| r.tier_level.as_str()).collect();
    assert_eq!(
        labels,
        vec!["1", "2", "3", "5", "7", "8", "N", "N", "4", "6"]
    );
    assert_eq!(
        tiers,
        vec!["E1", "E1", "E1", "E1", "E1", "E1", "E2", "E2", "E2", "E2"]
    );

    let cols = build_tier_columns(&project, 0, &ExportOptions::default(), true);
    assert_eq!(cols.len(), 2);
    assert_eq!(cols[0].0, "Ebene 1");
    assert_eq!(cols[0].1, vec!["1", "2", "3", "5", "7", "8"]);
    assert_eq!(cols[1].0, "Ebene 2");
    assert_eq!(cols[1].1, vec!["N", "N", "4", "6"]);
}

#[test]
fn xlsx_export_formats_all_cells_strictly_as_text() {
    let catalog = TerminalCatalog::load_embedded();
    let double_ll = catalog.find_by_id("wago-2002-2201").unwrap();
    let strip = TerminalStrip::generate_from_range("-X1", 1, 4, double_ll);

    let project = Project {
        format_version: 1,
        project_name: "XlsxTextTest".to_string(),
        strips: vec![strip],
        ..Default::default()
    };

    let temp_dir = std::env::temp_dir();
    let out_path = temp_dir.join("termigrid_text_format_test.xlsx");
    let _ = std::fs::remove_file(&out_path);

    let count = export_to_xlsx(&out_path, &project, 0, &ExportOptions::default(), true).unwrap();
    assert_eq!(count, 4);
    assert!(out_path.exists());

    let file = std::fs::File::open(&out_path).unwrap();
    let mut bytes = Vec::new();
    std::io::BufReader::new(file).read_to_end(&mut bytes).unwrap();
    assert_eq!(&bytes[0..2], b"PK");

    let _ = std::fs::remove_file(&out_path);
}

#[test]
fn csv_export_writes_utf8_bom_and_quoted_text() {
    let catalog = TerminalCatalog::load_embedded();
    let double_ln = catalog.find_by_id("wago-2002-2204").unwrap();
    let strip = TerminalStrip::generate_with_block_count("-X1", 1, 2, 2, double_ln);

    let project = Project {
        format_version: 1,
        project_name: "CsvTest".to_string(),
        strips: vec![strip],
        ..Default::default()
    };

    let out_path = std::env::temp_dir().join("termigrid_csv_test.csv");
    let _ = std::fs::remove_file(&out_path);

    let count = export_to_csv(&out_path, &project, 0, &ExportOptions::default(), true).unwrap();
    assert_eq!(count, 4);

    let content = std::fs::read(&out_path).unwrap();
    assert!(content.starts_with(&[0xEF, 0xBB, 0xBF]));
    let text = std::str::from_utf8(&content[3..]).unwrap();
    assert!(text.contains("\"-X1\";\"1\";\"E1\";\"1\""));
    assert!(text.contains("\"-X1\";\"1\";\"E2\";\"N\""));

    let _ = std::fs::remove_file(&out_path);
}

#[test]
fn accessories_resolve_for_all_three_manufacturers() {
    let catalog = TerminalCatalog::load_embedded();

    let wago = catalog.find_by_id("wago-2003-7641").unwrap();
    let wago_acc = catalog.accessories_for(wago);
    assert!(wago_acc.end_plate.contains("2003-7692"));
    assert!(wago_acc.jumpers.contains("2002-402"));

    let phoenix = catalog.find_by_id("phoenix-pti-25-peln").unwrap();
    let phx_acc = catalog.accessories_for(phoenix);
    assert!(phx_acc.end_plate.contains("D-PTI/3"));
    assert!(phx_acc.jumpers.contains("FBS 2-5"));

    let weid = catalog.find_by_id("weidmuller-aitb-25-nlp").unwrap();
    let weid_acc = catalog.accessories_for(weid);
    assert!(weid_acc.end_plate.contains("AEP ITB 2.5"));
    assert!(weid_acc.jumpers.contains("ZQV 2.5N/2"));
}

#[test]
fn end_brackets_database_covers_all_three_manufacturers_and_defaults_to_clipfix_35_5() {
    use std::collections::BTreeSet;
    use termigrid_core::catalog::accessories::{
        all_end_brackets, find_end_bracket, DEFAULT_END_BRACKET_ID,
    };

    let all = all_end_brackets();
    assert!(all.len() >= 18);

    let default_eb = find_end_bracket("");
    assert_eq!(default_eb.id, DEFAULT_END_BRACKET_ID);
    assert_eq!(default_eb.manufacturer, "Phoenix Contact");
    assert_eq!(default_eb.name, "CLIPFIX 35-5");
    assert_eq!(default_eb.article_number, "3022276");
    assert_eq!(default_eb.width_tenths_mm, 52);

    let mut mfgs = BTreeSet::new();
    let mut ids = BTreeSet::new();
    for eb in all {
        assert!(ids.insert(eb.id), "duplicate end bracket id: {}", eb.id);
        assert!(!eb.article_number.is_empty());
        assert!(eb.width_tenths_mm >= 40);
        mfgs.insert(eb.manufacturer);
    }
    assert!(mfgs.contains("Phoenix Contact"));
    assert!(mfgs.contains("Weidmüller"));
    assert!(mfgs.contains("WAGO"));
}

#[test]
fn strip_end_bracket_defaults_toggles_and_legacy_json_deserialization() {
    let catalog = TerminalCatalog::load_embedded();
    let single = catalog.find_by_id("wago-2002-1201").unwrap();
    let mut strip = TerminalStrip::generate_with_block_count("-X1", 1, 4, 4, single);

    assert_eq!(strip.end_bracket_id, "phoenix-clipfix-35-5");
    assert!(strip.end_bracket_left);
    assert!(strip.end_bracket_right);
    assert_eq!(strip.end_bracket_count(), 2);
    assert!((strip.end_brackets_width_mm() - 10.4).abs() < 0.01);

    // Disable one side when joining strips flush
    strip.end_bracket_right = false;
    assert_eq!(strip.end_bracket_count(), 1);
    assert!((strip.end_brackets_width_mm() - 5.2).abs() < 0.01);

    // Disable both sides
    strip.end_bracket_left = false;
    assert_eq!(strip.end_bracket_count(), 0);
    assert!((strip.end_brackets_width_mm() - 0.0).abs() < 0.01);

    // Legacy JSON without end_bracket_* fields must deserialize cleanly to CLIPFIX 35-5 (both sides enabled)
    let legacy_json = r#"{
        "format_version": 1,
        "project_name": "Legacy",
        "strips": [
            {
                "name": "-X1",
                "description": "",
                "start_number": 1,
                "target_end_number": 2,
                "default_spec_id": "wago-2002-1201",
                "blocks": [],
                "next_block_id": 1
            }
        ]
    }"#;
    let loaded: Project = serde_json::from_str(legacy_json).unwrap();
    assert_eq!(loaded.strips[0].end_bracket_id, "phoenix-clipfix-35-5");
    assert!(loaded.strips[0].end_bracket_left);
    assert!(loaded.strips[0].end_bracket_right);
}

#[test]
fn xlsx_export_succeeds_with_0_1_and_2_end_brackets() {
    let catalog = TerminalCatalog::load_embedded();
    let single = catalog.find_by_id("wago-2002-1201").unwrap();

    for (left, right, eb_id, suffix) in [
        (true, true, "phoenix-clipfix-35-5", "both"),
        (true, false, "weidmuller-ew-35", "left_only"),
        (false, false, "wago-249-116", "none"),
    ] {
        let mut strip = TerminalStrip::generate_with_block_count("-X1", 1, 3, 3, single);
        strip.end_bracket_id = eb_id.to_string();
        strip.end_bracket_left = left;
        strip.end_bracket_right = right;

        let project = Project {
            format_version: 1,
            project_name: "EndBracketXlsx".to_string(),
            strips: vec![strip],
            ..Default::default()
        };

        let out_path = std::env::temp_dir().join(format!("termigrid_eb_{suffix}.xlsx"));
        let _ = std::fs::remove_file(&out_path);
        let count = export_to_xlsx(&out_path, &project, 0, &ExportOptions::default(), true).unwrap();
        assert_eq!(count, 3);
        assert!(out_path.exists());
        let _ = std::fs::remove_file(&out_path);
    }
}

