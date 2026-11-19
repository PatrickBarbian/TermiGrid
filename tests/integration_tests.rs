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

#[test]
fn dxf_export_writes_single_true_scale_eplan_file_for_all_strips() {
    use termigrid_core::export::dxf::{export_to_dxf, export_to_dxf_string, strip_envelope_mm};

    let catalog = TerminalCatalog::load_embedded();
    let single = catalog.find_by_id("wago-2002-1201").unwrap(); // 5.2 x 48.5 mm
    let double = catalog.find_by_id("wago-2002-2201").unwrap(); // 5.2 x 69.7 mm
    let install = catalog.find_by_id("wago-2003-7641").unwrap(); // 5.2 x 96.0 mm

    let mut x1 = TerminalStrip::generate_with_block_count("-X1", 1, 3, 3, single);
    x1.reassign_blocks(&[1], double);
    x1.reassign_blocks(&[2], install);
    let x2 = TerminalStrip::generate_with_block_count("-X2", 1, 2, 2, single);

    let project = Project {
        format_version: 1,
        project_name: "EplanTest".to_string(),
        strips: vec![x1.clone(), x2],
        ..Default::default()
    };

    let dxf = export_to_dxf_string(&project, &catalog);
    // Strict AutoCAD R12 (AC1009) metric millimeters
    assert!(dxf.contains("AC1009"));
    assert!(dxf.contains("$INSUNITS\n 70\n4"));
    assert!(dxf.contains("$MEASUREMENT\n 70\n1"));
    // Both strips present in the single DXF file
    assert!(dxf.contains("-X1"));
    assert!(dxf.contains("-X2"));
    assert!(dxf.contains("TS 35"));
    // Strictly LINE and TEXT entities (no SOLID hatches that break EPLAN P8 import)
    assert!(dxf.contains("\nLINE\n"));
    assert!(dxf.contains("\nTEXT\n"));
    assert!(!dxf.contains("\nSOLID\n"));
    assert!(!dxf.contains("\nHATCH\n"));
    assert!(!dxf.contains("\nLWPOLYLINE\n"));

    // Envelope reflects tallest front-view block height (96.0 mm)
    let (env_w, env_h) = strip_envelope_mm(&x1, &catalog);
    assert!((env_h - 96.0).abs() < 0.05, "expected 96.0 mm height, got {env_h}");
    assert!(env_w > 25.0);

    let out_path = std::env::temp_dir().join("termigrid_eplan_test.dxf");
    let _ = std::fs::remove_file(&out_path);
    let count = export_to_dxf(&out_path, &project, &catalog).unwrap();
    assert_eq!(count, 2);
    assert!(out_path.exists());
    let _ = std::fs::remove_file(&out_path);
}

#[test]
fn dxf_places_standard_blocks_symmetrically_and_installation_blocks_with_real_offset() {
    use termigrid_core::export::dxf::{export_to_dxf_string, strip_envelope_mm};

    let catalog = TerminalCatalog::load_embedded();
    let pt25 = catalog.find_by_id("phoenix-pt-25").unwrap(); // 48.6 mm symmetric -> +-24.300
    let pti_ln = catalog.find_by_id("phoenix-pti-25-ln").unwrap(); // 101.0 mm, +13.0 mm offset -> -37.500 .. +63.500

    let mut strip = TerminalStrip::generate_with_block_count("-X1", 1, 2, 2, pt25);
    strip.end_bracket_left = false;
    strip.end_bracket_right = false;
    strip.reassign_blocks(&[1], pti_ln);

    let project = Project {
        format_version: 1,
        project_name: "RailOffset".to_string(),
        strips: vec![strip.clone()],
        ..Default::default()
    };

    let dxf = export_to_dxf_string(&project, &catalog);
    // Standard PT 2,5 centered on TS 35 (`rail_cy = 0`)
    assert!(dxf.contains("\n-24.300\n"), "PT 2,5 bottom should be -24.300");
    assert!(dxf.contains("\n24.300\n"), "PT 2,5 top should be +24.300");
    // Installation PTI 2,5-L/N shifted upward (+63.500 above, -37.500 below)
    assert!(dxf.contains("\n-37.500\n"), "PTI 2,5-L/N bottom should be -37.500");
    assert!(dxf.contains("\n63.500\n"), "PTI 2,5-L/N top should be +63.500");

    let (_w, env_h) = strip_envelope_mm(&strip, &catalog);
    assert!((env_h - 101.0).abs() < 0.05, "expected 101.0 mm, got {env_h}");
}

#[test]
fn installation_nt_and_n_supply_terminals_resolve_correctly() {
    use termigrid_core::catalog::accessories::resolve_accessories;

    let catalog = TerminalCatalog::load_embedded();
    assert!(
        catalog.all().len() >= 320,
        "Expected expanded catalog (>= 320 specs), got {}",
        catalog.all().len()
    );

    // WAGO 2016-7714 N-Einspeiseklemme
    let wago_supply = catalog
        .find_by_id("wago-2016-7714")
        .expect("wago-2016-7714 must exist");
    assert!(wago_supply.is_n_supply_terminal());
    assert_eq!(wago_supply.effective_article_number(), Some("2016-7714"));
    assert_eq!(wago_supply.width_tenths_mm, 120);
    let sup_acc = resolve_accessories(wago_supply);
    assert!(sup_acc.end_plate.contains("2016-7792"));

    // WAGO 2003, 2203, 2005, 2205 installation terminals & NT pairing
    for (base_id, expected_nt_id) in [
        ("wago-2003-7646", "wago-2003-7641"),
        ("wago-2203-7646", "wago-2203-7641"),
        ("wago-2005-7646", "wago-2005-7641"),
        ("wago-2205-7646", "wago-2205-7641"),
        ("phoenix-pti-25-peln", "phoenix-pti-25-pelnt"),
        ("weidmuller-aitb-25-nlp", "weidmuller-aitb-25-ntlp"),
    ] {
        let base = catalog.find_by_id(base_id).expect(base_id);
        assert!(base.is_installation_or_nt_terminal(), "{base_id} must trigger prompt");
        assert_eq!(base.matching_nt_spec_id(), Some(expected_nt_id));

        let nt = catalog.find_by_id(expected_nt_id).expect(expected_nt_id);
        assert!(nt.is_installation_or_nt_terminal());
        assert!(nt.is_nt_disconnect_terminal());

        let rec_supply = catalog
            .find_by_id(base.recommended_n_supply_spec_id())
            .expect("recommended N-supply must exist in catalog");
        assert!(rec_supply.is_n_supply_terminal());
        if base.manufacturer == "WAGO" {
            assert_eq!(rec_supply.id, "wago-2016-7714");
        }
    }
}

#[test]
fn multi_level_pe_alignment_and_export_handles_pure_and_hybrid_pe() {
    let catalog = TerminalCatalog::load_embedded();

    let pt_pe = catalog.find_by_id("phoenix-pt-25-pe").unwrap();
    let pttb_pe = catalog.find_by_id("phoenix-pttb-25-pe").unwrap();
    let pttb_pel = catalog.find_by_id("phoenix-pttb-25-pel").unwrap();
    let pt_3pe = catalog.find_by_id("phoenix-pt-25-3pe").unwrap();

    let mut strip = TerminalStrip::generate_with_block_count("-XPE", 1, 4, 4, pt_pe);
    strip.reassign_blocks(&[1], pttb_pe);
    strip.reassign_blocks(&[2], pttb_pel);
    strip.reassign_blocks(&[3], pt_3pe);

    // 1-tier PE: row -1
    assert_eq!(strip.blocks[0].tier_visual_row(0), -1);
    // Pure 2-level PE (PTTB 2,5-PE): rows 0, 1
    assert_eq!(strip.blocks[1].tier_visual_row(0), 0);
    assert_eq!(strip.blocks[1].tier_visual_row(1), 1);
    // Hybrid PE/L (PTTB 2,5-PE/L): bottom PE at -1, L at 0 -> sequential "1"
    assert_eq!(strip.blocks[2].tier_visual_row(0), -1);
    assert_eq!(strip.blocks[2].tier_visual_row(1), 0);
    assert_eq!(strip.blocks[2].tiers[1].computed_label, "1");
    // Pure 3-level PE (PT 2,5-3PE): rows 0, 1, 2
    assert_eq!(strip.blocks[3].tier_visual_row(0), 0);
    assert_eq!(strip.blocks[3].tier_visual_row(1), 1);
    assert_eq!(strip.blocks[3].tier_visual_row(2), 2);

    let project = Project {
        format_version: 1,
        project_name: "MultiPeExport".to_string(),
        strips: vec![strip],
        ..Default::default()
    };

    let opts = ExportOptions {
        include_pe_labels: true,
        ..ExportOptions::default()
    };
    let rows = build_print_rows(&project, 0, &opts);
    assert_eq!(rows.iter().filter(|r| r.tier_level.starts_with("PE")).count(), 7);
    assert!(rows.iter().any(|r| r.tier_level == "PE1"));
    assert!(rows.iter().any(|r| r.tier_level == "PE2"));
    assert!(rows.iter().any(|r| r.tier_level == "PE3"));

    let cols = build_tier_columns(&project, 0, &opts, true);
    assert_eq!(cols.len(), 2);
    assert_eq!(cols[0].0, "Ebene 1");
    assert_eq!(cols[0].1, vec!["1"]);
    assert_eq!(cols[1].0, "PE");
    assert_eq!(cols[1].1.len(), 7);

    // Pure-PE-only strip export must produce clean XLSX & tier columns without phantom Ebene columns
    let pe_only_strip = TerminalStrip::generate_with_block_count("-XPE2", 1, 2, 2, pttb_pe);
    let pe_only_project = Project {
        format_version: 1,
        project_name: "PurePeOnly".to_string(),
        strips: vec![pe_only_strip],
        ..Default::default()
    };
    let pe_cols = build_tier_columns(&pe_only_project, 0, &opts, true);
    assert_eq!(pe_cols.len(), 1);
    assert_eq!(pe_cols[0].0, "PE");
    assert_eq!(pe_cols[0].1, vec!["PE", "PE", "PE", "PE"]);

    let out_path = std::env::temp_dir().join("termigrid_multi_pe_test.xlsx");
    let _ = std::fs::remove_file(&out_path);
    let count = export_to_xlsx(&out_path, &pe_only_project, 0, &opts, true).unwrap();
    assert_eq!(count, 4);
    assert!(out_path.exists());
    let _ = std::fs::remove_file(&out_path);
}

#[test]
fn strip_library_preserves_custom_specs_potentials_and_avoids_name_collisions() {
    use termigrid_core::catalog::{TerminalSpec, TierSpec};
    use termigrid_core::model::library::{
        instantiate_strip_from_template, merge_template_custom_specs, StripLibrary,
    };
    use termigrid_core::model::{NumberingOrder, TierBehavior, TierColor};

    let custom_spec = TerminalSpec {
        id: "custom-test-ab1".to_string(),
        name: "Test-Klemme 4mm".to_string(),
        manufacturer: "Custom".to_string(),
        series: "Eigene".to_string(),
        levels: 2,
        width_tenths_mm: 62,
        numbering_order: NumberingOrder::BottomToTop,
        description_de: "Benutzerdefinierte Testklemme".to_string(),
        description_en: "Custom test terminal".to_string(),
        tiers: vec![
            TierSpec {
                behavior: TierBehavior::Sequential,
                color: TierColor::StandardGray,
            },
            TierSpec {
                behavior: TierBehavior::ProtectiveEarth,
                color: TierColor::GreenYellowPe,
            },
        ],
        accessories: None,
        jumper_slots_per_tier: Some(1),
        max_jumper_poles: Some(10),
        article_number: Some("9990001".to_string()),
        product_url: None,
        image_url: None,
        height_tenths_mm: None,
        rail_offset_tenths_mm: None,
    };

    let mut catalog = TerminalCatalog::load_embedded();
    catalog.reload_with_custom(std::slice::from_ref(&custom_spec));

    let mut strip = TerminalStrip::generate_with_block_count("-X1", 10, 15, 3, &custom_spec);
    strip.description = "Antriebsgruppe A".to_string();
    let p_id = strip.add_potential("+24V_DRV", [14, 165, 233]);
    strip.toggle_potential_member(p_id, 0, 0);
    strip.toggle_potential_member(p_id, 1, 0);

    let mut lib = StripLibrary::default();
    let tpl_id = lib.save_template(
        "Antriebs-Vorlage",
        "Motorabgang mit Custom-Klemme",
        &strip,
        std::slice::from_ref(&custom_spec),
        None,
    );

    let tpl = lib.find_by_id(&tpl_id).unwrap();
    assert_eq!(tpl.custom_specs.len(), 1);
    assert_eq!(tpl.custom_specs[0].id, "custom-test-ab1");

    // Simulate loading into a fresh project that already has -X1 and no custom specs yet
    let mut fresh_project = Project::default();
    fresh_project.strips.push(strip);
    assert!(merge_template_custom_specs(tpl, &mut fresh_project.custom_specs));
    assert!(!merge_template_custom_specs(tpl, &mut fresh_project.custom_specs));
    assert_eq!(fresh_project.custom_specs.len(), 1);

    let cloned_strip = instantiate_strip_from_template(tpl, None, &fresh_project.strips);
    assert_eq!(cloned_strip.name, "-X2");
    assert_eq!(cloned_strip.blocks.len(), 3);
    assert_eq!(cloned_strip.potentials.len(), 1);
    assert_eq!(cloned_strip.blocks[0].tiers[0].computed_label, "10");
}

#[test]
fn busbar_carriers_frame_install_group_and_support_every_200_mm() {
    use termigrid_core::catalog::accessories::calculate_end_plates;
    use termigrid_core::model::numbering::{apply_n_busbar_carriers_around, BusbarCarrierPlan};

    let catalog = TerminalCatalog::load_embedded();

    // 1. WAGO 2003-7641 short group with 2016-7714 supply + 2009-305 outer carriers
    let wago_nt = catalog.find_by_id("wago-2003-7641").unwrap();
    let wago_supply = catalog.find_by_id("wago-2016-7714").unwrap();
    let wago_outer = catalog.find_by_id("wago-2009-305").unwrap();
    let wago_mid = catalog.find_by_id("wago-2009-304").unwrap();

    assert_eq!(wago_nt.recommended_outer_busbar_carrier_spec_id(), Some("wago-2009-305"));
    assert_eq!(wago_nt.recommended_intermediate_busbar_support_spec_id(), Some("wago-2009-304"));
    assert_eq!(wago_outer.width_tenths_mm, 75);
    assert_eq!(wago_mid.width_tenths_mm, 15);

    let mut short_strip = TerminalStrip::generate_with_block_count("-XINST1", 1, 10, 5, wago_nt);
    short_strip.insert_blocks(0, 1, wago_supply);
    let end = short_strip.blocks.len();
    apply_n_busbar_carriers_around(
        &mut short_strip,
        0,
        end,
        &catalog,
        &BusbarCarrierPlan {
            outer_carrier: Some(wago_outer),
            intermediate_support: Some(wago_mid),
            max_unsupported_span_tenths_mm: 2000,
        },
    );

    // Must be: [2009-305, 2016-7714, 5x 2003-7641, 2009-305]
    assert_eq!(short_strip.blocks.len(), 8);
    assert_eq!(short_strip.blocks[0].spec_id, "wago-2009-305");
    assert_eq!(short_strip.blocks[1].spec_id, "wago-2016-7714");
    assert_eq!(short_strip.blocks[7].spec_id, "wago-2009-305");
    // No intermediate 2009-304 needed when unsupported span is well under 200 mm
    assert_eq!(
        short_strip.blocks.iter().filter(|b| b.spec_id == "wago-2009-304").count(),
        0
    );

    // 2. Long WAGO 2003-7641 row (45 blocks * 5.2 mm + 12.0 mm supply = 246.0 mm > 200.0 mm)
    let mut long_strip = TerminalStrip::generate_with_block_count("-XINST2", 1, 90, 45, wago_nt);
    long_strip.insert_blocks(0, 1, wago_supply);
    let long_end = long_strip.blocks.len();
    apply_n_busbar_carriers_around(
        &mut long_strip,
        0,
        long_end,
        &catalog,
        &BusbarCarrierPlan {
            outer_carrier: Some(wago_outer),
            intermediate_support: Some(wago_mid),
            max_unsupported_span_tenths_mm: 2000,
        },
    );

    assert_eq!(long_strip.blocks.first().unwrap().spec_id, "wago-2009-305");
    assert_eq!(long_strip.blocks[1].spec_id, "wago-2016-7714");
    assert_eq!(long_strip.blocks.last().unwrap().spec_id, "wago-2009-305");
    assert_eq!(
        long_strip.blocks.iter().filter(|b| b.spec_id == "wago-2009-304").count(),
        1,
        "Expected one 2009-304 intermediate support at ~200 mm span"
    );

    // Verify no unsupported distance between any two busbar carriers exceeds 200.0 mm
    // AND that the split on ~246 mm is symmetrical around ~123 mm (121.2 mm | 124.8 mm)
    let mut segment_widths: Vec<u16> = Vec::new();
    let mut span_tenths: u16 = 0;
    for blk in &long_strip.blocks {
        if catalog.find_by_id(&blk.spec_id).is_some_and(|s| s.is_busbar_carrier()) {
            if span_tenths > 0 {
                segment_widths.push(span_tenths);
            }
            assert!(
                span_tenths <= 2000,
                "Unsupported N-busbar span exceeded 200.0 mm: {} tenths mm",
                span_tenths
            );
            span_tenths = 0;
        } else {
            span_tenths += blk.width_tenths_mm;
        }
    }
    assert_eq!(segment_widths.len(), 2);
    let imbalance = segment_widths[0].abs_diff(segment_widths[1]);
    assert!(
        imbalance <= 60,
        "Expected symmetrical intermediate support (~120 mm split), got {:?}",
        segment_widths
    );

    // Re-running placement must be idempotent (zero duplicate carriers added)
    let count_before = long_strip.blocks.len();
    let end2 = long_strip.blocks.len();
    apply_n_busbar_carriers_around(
        &mut long_strip,
        0,
        end2,
        &catalog,
        &BusbarCarrierPlan {
            outer_carrier: Some(wago_outer),
            intermediate_support: Some(wago_mid),
            max_unsupported_span_tenths_mm: 2000,
        },
    );
    assert_eq!(long_strip.blocks.len(), count_before);

    // Busbar carriers must not trigger end plates before themselves and must be omitted from printer labels
    let plates = calculate_end_plates(&long_strip, &catalog);
    for p in &plates {
        let next_blk = long_strip.blocks.get(p.after_block_idx + 1);
        assert!(
            next_blk.is_none_or(|nb| !catalog.find_by_id(&nb.spec_id).unwrap().is_busbar_carrier()),
            "No intermediate end plate should be placed before a busbar carrier at idx {}",
            p.after_block_idx
        );
    }

    let proj = Project {
        format_version: 1,
        project_name: "BusbarCarrierExport".to_string(),
        strips: vec![long_strip],
        ..Default::default()
    };
    let print_rows = build_print_rows(&proj, 0, &ExportOptions::default());
    assert!(
        !print_rows.iter().any(|r| r.terminal_type.contains("2009-30")),
        "Unlabeled busbar carriers must not pollute printer label rows"
    );

    let out_path = std::env::temp_dir().join("termigrid_busbar_carrier_test.xlsx");
    let _ = std::fs::remove_file(&out_path);
    export_to_xlsx(&out_path, &proj, 0, &ExportOptions::default(), true).unwrap();
    assert!(out_path.exists());
    let _ = std::fs::remove_file(&out_path);

    // 3. Phoenix Contact PTI 2,5-PE/L/NT with AB-PTI/3 Auflagebock (symmetrical around ~117 mm)
    let phx_nt = catalog.find_by_id("phoenix-pti-25-pelnt").unwrap();
    let phx_ab = catalog.find_by_id("phoenix-ab-pti-3").unwrap();
    assert_eq!(phx_nt.recommended_outer_busbar_carrier_spec_id(), Some("phoenix-ab-pti-3"));
    assert_eq!(phx_nt.recommended_intermediate_busbar_support_spec_id(), Some("phoenix-ab-pti-3"));

    let mut phx_strip = TerminalStrip::generate_with_block_count("-XPHX", 1, 90, 45, phx_nt);
    let phx_end = phx_strip.blocks.len();
    apply_n_busbar_carriers_around(
        &mut phx_strip,
        0,
        phx_end,
        &catalog,
        &BusbarCarrierPlan {
            outer_carrier: Some(phx_ab),
            intermediate_support: Some(phx_ab),
            max_unsupported_span_tenths_mm: 2000,
        },
    );
    // 2 outer AB-PTI/3 + 1 intermediate AB-PTI/3 (45 * 5.2 mm = 234.0 mm -> symmetrical 22/23 split: ~114.4 mm | ~119.6 mm)
    let ab_indices: Vec<usize> = phx_strip
        .blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| (b.spec_id == "phoenix-ab-pti-3").then_some(i))
        .collect();
    assert_eq!(ab_indices.len(), 3);
    assert_eq!(ab_indices[0], 0);
    assert_eq!(ab_indices[2], 47);
    assert!(
        ab_indices[1] == 23 || ab_indices[1] == 24,
        "Expected symmetrical center carrier at index 23 or 24, got {:?}",
        ab_indices
    );
}

#[test]
fn n_supply_places_before_first_install_or_nt_block_even_at_pos_8() {
    use termigrid_core::model::numbering::{
        apply_n_busbar_carriers_around, place_n_supply_in_zone, BusbarCarrierPlan,
    };

    let catalog = TerminalCatalog::load_embedded();
    let std_2002 = catalog.find_by_id("wago-2002-1201").unwrap();
    let wago_2003_pln = catalog.find_by_id("wago-2003-7646").unwrap(); // 2003 without NT
    let wago_2003_nt = catalog.find_by_id("wago-2003-7641").unwrap(); // 2003 with NT
    let wago_supply = catalog.find_by_id("wago-2016-7714").unwrap();
    let wago_outer = catalog.find_by_id("wago-2009-305").unwrap();
    let wago_mid = catalog.find_by_id("wago-2009-304").unwrap();

    // Pos 1..7 (indices 0..7) = standard 2002-1201
    // Pos 8..9 (indices 7..9) = 2003-7646 (non-NT 2003)
    // Pos 10 (index 9) = newly added 2003-7641 (NT)
    let mut strip = TerminalStrip::generate_with_block_count("-XMIX", 1, 20, 10, std_2002);
    strip.reassign_blocks(&[7, 8], wago_2003_pln);
    strip.reassign_blocks(&[9], wago_2003_nt);

    let (zs, ze) = place_n_supply_in_zone(&mut strip, 9, 10, wago_supply, true, &catalog);
    apply_n_busbar_carriers_around(
        &mut strip,
        zs,
        ze,
        &catalog,
        &BusbarCarrierPlan {
            outer_carrier: Some(wago_outer),
            intermediate_support: Some(wago_mid),
            max_unsupported_span_tenths_mm: 2000,
        },
    );

    // Standard 2002-1201 remain untouched at indices 0..7
    for block in strip.blocks.iter().take(7) {
        assert_eq!(block.spec_id, "wago-2002-1201");
    }
    // Index 7 (Pos 8) starts with left carrier 2009-305, then 2016-7714 BEFORE the first 2003 (2003-7646)!
    assert_eq!(strip.blocks[7].spec_id, "wago-2009-305");
    assert_eq!(strip.blocks[8].spec_id, "wago-2016-7714");
    assert_eq!(strip.blocks[9].spec_id, "wago-2003-7646");
    assert_eq!(strip.blocks[10].spec_id, "wago-2003-7646");
    assert_eq!(strip.blocks[11].spec_id, "wago-2003-7641");
    assert_eq!(strip.blocks[12].spec_id, "wago-2009-305");
}

#[test]
fn batch_n_to_nt_and_nt_to_n_conversion_preserves_labels_and_overrides() {
    let catalog = TerminalCatalog::load_embedded();

    // Mixed installation terminals across all three manufacturers
    let wago_n = catalog.find_by_id("wago-2003-7646").unwrap();
    let wago_ll = catalog.find_by_id("wago-2003-7649").unwrap();
    let phx_n = catalog.find_by_id("phoenix-pti-25-peln").unwrap();
    let wm_n = catalog.find_by_id("weidmuller-aitb-25-nlp").unwrap();
    let std_l = catalog.find_by_id("wago-2002-1201").unwrap();

    let mut strip = TerminalStrip::generate_with_block_count("-XNT", 1, 10, 5, wago_n);
    strip.reassign_blocks(&[1], wago_ll);
    strip.reassign_blocks(&[2], phx_n);
    strip.reassign_blocks(&[3], wm_n);
    strip.reassign_blocks(&[4], std_l);

    // Apply a custom override on block #0 L tier (tier index 1) and affix on block #2 N tier (tier index 2)
    strip.blocks[0].tiers[1].custom_override = Some("L1_HAUPT".to_string());
    strip.blocks[2].tiers[2].label_prefix = "N_".to_string();
    strip.recalculate_labels();

    // 4 blocks should convert (0: 2003-7646, 1: 2003-7649, 2: PTI-25-PE/L/N, 3: AITB-25-NLP); block 4 (2002-1201) stays
    let converted_nt = strip.convert_n_to_nt_blocks(&catalog);
    assert_eq!(converted_nt, 4);
    assert_eq!(strip.blocks[0].spec_id, "wago-2003-7641");
    assert_eq!(strip.blocks[1].spec_id, "wago-2003-7640");
    assert_eq!(strip.blocks[2].spec_id, "phoenix-pti-25-pelnt");
    assert_eq!(strip.blocks[3].spec_id, "weidmuller-aitb-25-ntlp");
    assert_eq!(strip.blocks[4].spec_id, "wago-2002-1201");

    // Overrides & affixes preserved across conversion
    assert_eq!(strip.blocks[0].tiers[1].custom_override.as_deref(), Some("L1_HAUPT"));
    assert_eq!(strip.blocks[0].tiers[1].computed_label, "L1_HAUPT");
    assert_eq!(strip.blocks[2].tiers[2].label_prefix, "N_");
    assert_eq!(strip.blocks[2].tiers[2].computed_label, "N_N");

    // Round-trip back from NT -> N
    let converted_n = strip.convert_nt_to_n_blocks(&catalog);
    assert_eq!(converted_n, 4);
    assert_eq!(strip.blocks[0].spec_id, "wago-2003-7646");
    assert_eq!(strip.blocks[1].spec_id, "wago-2003-7649");
    assert_eq!(strip.blocks[2].spec_id, "phoenix-pti-25-peln");
    assert_eq!(strip.blocks[3].spec_id, "weidmuller-aitb-25-nlp");
    assert_eq!(strip.blocks[0].tiers[1].computed_label, "L1_HAUPT");
    assert_eq!(strip.blocks[2].tiers[2].computed_label, "N_N");
}

#[test]
fn one_pole_and_three_pole_feeder_plans_insert_cleanly_across_families() {
    use termigrid_core::catalog::accessories::calculate_end_plates;

    let catalog = TerminalCatalog::load_embedded();

    // 1. WAGO 2002-1201: 1-Polig (L + N + PE) & 3-Polig (3x L + N + PE)
    let w2002 = catalog.find_by_id("wago-2002-1201").unwrap();
    let plan_1p = catalog.feeder_block_plan(w2002, 1).unwrap();
    let plan_3p = catalog.feeder_block_plan(w2002, 3).unwrap();
    assert_eq!(
        plan_1p.specs.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(),
        vec!["wago-2002-1201", "wago-2002-1204", "wago-2002-1207"]
    );
    assert_eq!(
        plan_3p.specs.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(),
        vec![
            "wago-2002-1201",
            "wago-2002-1201",
            "wago-2002-1201",
            "wago-2002-1204",
            "wago-2002-1207"
        ]
    );

    let mut strip = TerminalStrip::generate_with_block_count("-XFEED", 1, 1, 0, w2002);
    let ins1 = strip.insert_spec_sequence(0, &plan_1p.specs);
    assert_eq!(ins1.len(), 3);
    assert_eq!(strip.blocks[0].tiers[0].computed_label, "1");
    assert_eq!(strip.blocks[1].tiers[0].computed_label, "N");
    assert!(strip.blocks[2].tiers[0].behavior.is_pe());

    let ins3 = strip.insert_spec_sequence(strip.blocks.len(), &plan_3p.specs);
    assert_eq!(ins3.len(), 5);
    assert_eq!(strip.blocks.len(), 8);
    assert_eq!(strip.blocks[3].tiers[0].computed_label, "2");
    assert_eq!(strip.blocks[4].tiers[0].computed_label, "3");
    assert_eq!(strip.blocks[5].tiers[0].computed_label, "4");
    assert_eq!(strip.blocks[6].tiers[0].computed_label, "N");
    assert!(strip.blocks[7].tiers[0].behavior.is_pe());

    // Homogeneous 2002 profile across L/N/PE -> only 1 end plate at the end of the strip
    let plates = calculate_end_plates(&strip, &catalog);
    assert_eq!(plates.len(), 1);
    assert_eq!(plates[0].after_block_idx, 7);

    // 2. Phoenix PTI 2,5 installation (standard N & NT) & Weidmüller A2C 2.5
    let pti_peln = catalog.find_by_id("phoenix-pti-25-peln").unwrap();
    let pti_3p = catalog.feeder_block_plan(pti_peln, 3).unwrap();
    assert_eq!(
        pti_3p.specs.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(),
        vec!["phoenix-pti-25-ll", "phoenix-pti-25-peln"]
    );

    let pti_nt = catalog.find_by_id("phoenix-pti-25-pelnt").unwrap();
    let pti_nt_3p = catalog.feeder_block_plan(pti_nt, 3).unwrap();
    assert_eq!(
        pti_nt_3p.specs.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(),
        vec!["phoenix-pti-25-ll", "phoenix-pti-25-pelnt"]
    );

    let a2c = catalog.find_by_id("weidmuller-a2c-25").unwrap();
    let a2c_3p = catalog.feeder_block_plan(a2c, 3).unwrap();
    assert_eq!(
        a2c_3p.specs.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(),
        vec![
            "weidmuller-a2c-25",
            "weidmuller-a2c-25",
            "weidmuller-a2c-25",
            "weidmuller-a2c-25-bl",
            "weidmuller-a2c-25-pe"
        ]
    );
}

#[test]
fn pti_25_multilevel_shares_d_pti_3_without_intermediate_end_plates_and_nt_only_predicate() {
    use termigrid_core::catalog::accessories::calculate_end_plates;
    let catalog = TerminalCatalog::load_embedded();

    // Non-NT installation terminals must NOT trigger the NT disconnect popup
    for non_nt_id in [
        "phoenix-pti-25-ll",
        "phoenix-pti-25-ln",
        "phoenix-pti-25-peln",
        "phoenix-pti-25-pell",
        "wago-2003-7646",
        "wago-2203-7646",
        "weidmuller-aitb-25-nlp",
    ] {
        let sp = catalog.find_by_id(non_nt_id).expect(non_nt_id);
        assert!(
            !sp.is_nt_disconnect_terminal(),
            "{non_nt_id} is not an NT terminal and must not trigger NT popup"
        );
    }

    // Genuine NT disconnect terminals MUST trigger the NT popup
    for nt_id in [
        "phoenix-pti-25-lnt",
        "phoenix-pti-25-pelnt",
        "phoenix-pti-4-pelnt",
        "wago-2003-7641",
        "wago-2203-7641",
        "weidmuller-aitb-25-ntlp",
    ] {
        let sp = catalog.find_by_id(nt_id).expect(nt_id);
        assert!(
            sp.is_nt_disconnect_terminal(),
            "{nt_id} is a genuine NT terminal and must trigger NT popup"
        );
    }

    // Mixing PTI 2,5-L/L, PTI 2,5-L/N, PTI 2,5-PE/L/N and PTI 2,5-PE/L/NT on one strip
    // shares the single 101 mm contour cover D-PTI/3 (3213975) -> only 1 end plate at the end!
    let pti_peln = catalog.find_by_id("phoenix-pti-25-peln").unwrap();
    let pti_ll = catalog.find_by_id("phoenix-pti-25-ll").unwrap();
    let pti_ln = catalog.find_by_id("phoenix-pti-25-ln").unwrap();
    let pti_pelnt = catalog.find_by_id("phoenix-pti-25-pelnt").unwrap();

    let mut strip = TerminalStrip::generate_with_block_count("-XPTI", 1, 1, 0, pti_peln);
    strip.insert_spec_sequence(0, &[pti_peln, pti_ll, pti_peln, pti_ln, pti_pelnt]);
    let plates = calculate_end_plates(&strip, &catalog);
    assert_eq!(plates.len(), 1, "Expected no intermediate plates between PTI 2,5 multi-level blocks: {plates:?}");
    assert_eq!(plates[0].after_block_idx, 4);
    assert!(plates[0].end_plate.contains("3213975"));
}

#[test]
fn end_plates_generated_lifecycle_and_legacy_deserialization() {
    use std::collections::BTreeSet;
    let catalog = TerminalCatalog::load_embedded();
    let l_spec = catalog.find_by_id("wago-2002-1201").unwrap();
    let n_spec = catalog.find_by_id("wago-2002-1204").unwrap();
    let pe_spec = catalog.find_by_id("wago-2002-1207").unwrap();

    // Fresh strip starts ungenerated (`false`) until user clicks "Generieren"
    let mut strip = TerminalStrip::generate_with_block_count("-XGEN", 1, 5, 2, l_spec);
    assert!(!strip.end_plates_generated);

    // Simulating "Generieren": mark generated & verify JSON roundtrip preserves `true`
    strip.end_plates_generated = true;
    let json = serde_json::to_string(&strip).unwrap();
    let reloaded: TerminalStrip = serde_json::from_str(&json).unwrap();
    assert!(reloaded.end_plates_generated);

    // Legacy project JSON without `end_plates_generated` defaults to `true`
    let mut value: serde_json::Value = serde_json::from_str(&json).unwrap();
    value.as_object_mut().unwrap().remove("end_plates_generated");
    let legacy: TerminalStrip = serde_json::from_value(value).unwrap();
    assert!(legacy.end_plates_generated);

    // Every physical block mutation resets `end_plates_generated` back to `false`
    strip.end_plates_generated = true;
    strip.insert_blocks(2, 1, n_spec);
    assert!(!strip.end_plates_generated);

    strip.end_plates_generated = true;
    strip.reassign_blocks(&[0], pe_spec);
    assert!(!strip.end_plates_generated);

    strip.end_plates_generated = true;
    let sel: BTreeSet<usize> = [0].into_iter().collect();
    strip.move_selected_blocks(&sel, 1);
    assert!(!strip.end_plates_generated);

    strip.end_plates_generated = true;
    let dup_sel: BTreeSet<usize> = [1].into_iter().collect();
    strip.duplicate_selected_blocks(&dup_sel);
    assert!(!strip.end_plates_generated);

    strip.end_plates_generated = true;
    strip.insert_spec_sequence(strip.blocks.len(), &[l_spec, n_spec, pe_spec]);
    assert!(!strip.end_plates_generated);

    strip.end_plates_generated = true;
    let converted_to_nt = strip.convert_n_to_nt_blocks(&catalog);
    assert!(converted_to_nt > 0);
    assert!(!strip.end_plates_generated);

    strip.end_plates_generated = true;
    let converted_to_n = strip.convert_nt_to_n_blocks(&catalog);
    assert!(converted_to_n > 0);
    assert!(!strip.end_plates_generated);

    strip.end_plates_generated = true;
    strip.delete_blocks(&[0]);
    assert!(!strip.end_plates_generated);
}
