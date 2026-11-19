pub mod pdf_parser;
pub mod planner;
pub mod rules;

use std::path::Path;

use crate::catalog::TerminalCatalog;
use crate::import::pdf_parser::{extract_pdf_text_spans, parse_eplan_bom_tables, RawBomRow};
use crate::import::planner::{build_import_plan, PdfBomImportPlan};
use crate::import::rules::CompiledImportRules;

/// Imports and plans terminal strips from an EPLAN PDF parts list file on disk.
pub fn import_pdf_path(path: &Path, catalog: &TerminalCatalog) -> Result<PdfBomImportPlan, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("PDF-Datei konnte nicht gelesen werden: {e}"))?;
    import_pdf_bytes(&bytes, catalog)
}

/// Imports and plans terminal strips from in-memory PDF bytes using embedded rules.
pub fn import_pdf_bytes(
    pdf_bytes: &[u8],
    catalog: &TerminalCatalog,
) -> Result<PdfBomImportPlan, String> {
    let rules = CompiledImportRules::load_default();
    let spans = extract_pdf_text_spans(pdf_bytes)?;
    let rows = parse_eplan_bom_tables(&spans, &rules);
    Ok(build_import_plan(&rows, &rules, catalog))
}

/// Plans terminal strips directly from pre-parsed BOM rows (useful for testing and alternate tabular sources).
pub fn plan_from_bom_rows(raw_rows: &[RawBomRow], catalog: &TerminalCatalog) -> PdfBomImportPlan {
    let rules = CompiledImportRules::load_default();
    build_import_plan(raw_rows, &rules, catalog)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import::pdf_parser::RawBomRow;
    use crate::import::planner::instantiate_imported_strip;
    use crate::model::numbering::{
        apply_n_busbar_carriers_around, place_n_supply_in_zone, BusbarCarrierPlan,
    };
    use std::collections::BTreeMap;

    fn hb_hixberg_sample_rows() -> Vec<RawBomRow> {
        let r = |page, bmk: &str, qty, desc: &str, typ: &str, sup: &str, art: &str| RawBomRow {
            page,
            bmk: bmk.to_string(),
            quantity: qty,
            description: desc.to_string(),
            type_number: typ.to_string(),
            supplier: sup.to_string(),
            article_number: art.to_string(),
        };
        vec![
            // Distractor rows (must be completely ignored via strict BMK filter)
            r(1, "-Q2.18", 1, "LS-Schalter B16", "PXK-B161N//003-A", "ETN", "ETN.236948"),
            r(1, "-Q2.24", 1, "LS-Schalter B16", "PXK-B161N//003-A", "ETN", "ETN.236948"),
            r(1, "-Q4", 1, "Hauptschalter", "ERC218", "HAG", "HAG.ERC218"),
            r(1, "-1T1", 1, "Stromwandler", "13049", "MBS", "MBS.13049"),
            r(1, "-2T3", 1, "Stromwandler", "13049", "MBS", "MBS.13049"),
            r(1, "-12T1", 1, "Steuertrafo / Klemme", "879-3120", "", "WAGO.879-3120"),
            r(1, "-W1", 1, "Steuerleitung", "ÖLFLEX", "LAPP", "LAPP.00123"),
            r(1, "-W5", 1, "Mantelleitung", "NYM-J", "HELU", "HELU.4567"),
            // Valid -X... strips from HB Hixberg.pdf
            r(1, "-X0.1", 6, "Durchgangs-Reihenklemme", "285-195", "", "WAGO.285-195"),
            r(1, "-X0.1", 3, "Querverbinder/Brücker für Reihenklemme", "285-495", "", "WAGO.285-495"),
            r(1, "-X0.2", 6, "Durchgangs-Reihenklemme", "285-195", "", "WAGO.285-195"),
            r(1, "-X0.2", 3, "Querverbinder/Brücker für Reihenklemme", "285-495", "", "WAGO.285-495"),
            r(1, "-X1", 6, "Durchgangs-Reihenklemme", "285-135", "", "WAGO.285-135"),
            r(1, "-X1", 3, "Querverbinder/Brücker für Reihenklemme", "285-435", "", "WAGO.285-435"),
            r(1, "-X2", 24, "Dreistock-Installationsklemme", "2203-7646", "", "WAGO.2203-7646"),
            // -X3 split across page 1 and page 2
            r(1, "-X3", 4, "Dreistock-Installationsklemme", "2203-7646", "", "WAGO.2203-7646"),
            r(1, "-X3", 24, "Durchgangs-Reihenklemme", "2204-1201", "", "WAGO.2204-1201"),
            r(2, "-X3", 8, "Durchgangs-Reihenklemme", "2204-1204", "", "WAGO.2204-1204"),
            r(2, "-X3", 8, "Schutzleiter-Reihenklemme", "2204-1207", "", "WAGO.2204-1207"),
            // -XB1 with existing end clamps & end plates in PDF
            r(2, "-XB1", 2, "Endklammer", "249-117", "", "WAGO.249-117"),
            r(2, "-XB1", 10, "3-Leiter-Dreistockklemme", "2202-3201", "", "WAGO.2202-3201"),
            r(2, "-XB1", 18, "Doppelstockklemme", "2202-2701", "", "WAGO.2202-2701"),
            r(2, "-XB1", 2, "Abschlussplatte", "2002-3292", "", "WAGO.2002-3292"),
            r(2, "-XB1", 1, "Schutzleiter-Reihenklemme", "2202-1207", "", "WAGO.2202-1207"),
            // -XB2.1 & -XB2.2 with unknown WAGO 2007-8873
            r(2, "-XB2.1", 1, "Stromwandlerblock", "2007-8873", "", "WAGO.2007-8873"),
            r(2, "-XB2.2", 1, "Stromwandlerblock", "2007-8873", "", "WAGO.2007-8873"),
            // Foreign manufacturers on -XP and -Xn
            r(2, "-XP", 1, "Durchführungssystem", "KDP", "MCT", "MCT.KDP-24"),
            r(2, "-Xn", 1, "Schienenmaterial", "080220-1-3", "", "HORA.080220-1-3"),
        ]
    }

    #[test]
    fn strict_bmk_filter_and_hb_hixberg_classification() {
        let catalog = TerminalCatalog::load_embedded();
        let rows = hb_hixberg_sample_rows();
        let plan = plan_from_bom_rows(&rows, &catalog);

        // Distractors: -Q2.18, -Q2.24, -Q4, -1T1, -2T3, -12T1, -W1, -W5 => 8 ignored rows
        assert_eq!(plan.ignored_non_x_rows.len(), 8);
        assert!(plan.ignored_non_x_rows.iter().any(|r| r.bmk == "-12T1"));

        // Foreign strips skipped cleanly: -XP (MCT) and -Xn (HORA)
        assert_eq!(plan.skipped_foreign.len(), 2);
        assert!(plan.skipped_foreign.iter().any(|f| f.bmk == "-XP" && f.manufacturer == "MCT"));
        assert!(plan.skipped_foreign.iter().any(|f| f.bmk == "-Xn" && f.manufacturer == "HORA"));

        // Imported strips: -X0.1, -X0.2, -X1, -X2, -X3, -XB1, -XB2.1, -XB2.2 => 8 strips
        let bmks: Vec<&str> = plan.strips.iter().map(|s| s.bmk.as_str()).collect();
        assert_eq!(
            bmks,
            vec!["-X0.1", "-X0.2", "-X1", "-X2", "-X3", "-XB1", "-XB2.1", "-XB2.2"]
        );

        // -X0.1: 6x 285-195, jumper 285-495 ignored, homogeneous (1 variant), auto end clamps
        let x01 = &plan.strips[0];
        assert_eq!(x01.total_terminal_count, 6);
        assert_eq!(x01.variants.len(), 1);
        assert_eq!(x01.ignored_jumpers.len(), 1);
        assert!(x01.end_parts.auto_complement_brackets);
        assert!(!x01.has_nt_or_install);

        // -X1: 6x 285-135, jumper 285-435 ignored
        let x1 = &plan.strips[2];
        assert_eq!(x1.total_terminal_count, 6);
        assert_eq!(x1.terminals[0].spec_id, "wago-285-135");
        assert_eq!(x1.ignored_jumpers.len(), 1);

        // -X2: 24x 2203-7646 (PE/L/N, not NT), homogeneous, does NOT trigger NT popup
        let x2 = &plan.strips[3];
        assert_eq!(x2.total_terminal_count, 24);
        assert_eq!(x2.variants.len(), 1);
        assert!(!x2.has_nt_or_install);
        assert_eq!(x2.nt_anchor_spec_id, None);

        // -X3: merged across Page 1 & 2, exact 44 terminals, <=3 unique variants, closed install block 0..4 (no NT popup for 2203-7646)
        let x3 = &plan.strips[4];
        assert_eq!(x3.pages, vec![1, 2]);
        assert_eq!(x3.total_terminal_count, 44);
        assert!(!x3.has_nt_or_install);
        assert!(x3.variants.len() >= 2 && x3.variants.len() <= 3);

        // Verify quantity invariant & closed NT block across ALL variants
        let expected_x3: BTreeMap<&str, usize> = [
            ("wago-2203-7646", 4usize),
            ("wago-2204-1201", 24),
            ("wago-2204-1204", 8),
            ("wago-2204-1207", 8),
        ]
        .into_iter()
        .collect();
        for v in &x3.variants {
            let mut actual: BTreeMap<&str, usize> = BTreeMap::new();
            for id in &v.block_spec_ids {
                *actual.entry(id.as_str()).or_insert(0) += 1;
            }
            assert_eq!(actual, expected_x3);
            assert_eq!(v.nt_zone, Some((0, 4)));
            assert!(v.block_spec_ids[0..4].iter().all(|id| id == "wago-2203-7646"));
        }
        // Variant 0 is Drehstrom 3x L + N + PE after closed NT block
        assert_eq!(
            &x3.variants[0].block_spec_ids[4..9],
            &[
                "wago-2204-1201",
                "wago-2204-1201",
                "wago-2204-1201",
                "wago-2204-1204",
                "wago-2204-1207",
            ]
        );

        // -XB1: 29 terminals, end clamps & end plates came FROM PDF (no duplicate auto-complement!)
        let xb1 = &plan.strips[5];
        assert_eq!(xb1.total_terminal_count, 29);
        assert!(xb1.end_parts.has_pdf_end_clamps);
        assert!(xb1.end_parts.has_pdf_end_plates);
        assert!(!xb1.end_parts.auto_complement_brackets);
        assert_eq!(xb1.end_parts.end_bracket_id, "wago-249-117");
        assert!(xb1.end_parts.bracket_left && xb1.end_parts.bracket_right);
        assert_eq!(xb1.variants[0].block_spec_ids[0], "wago-2202-3201");
        assert_eq!(xb1.variants[0].block_spec_ids[10], "wago-2202-2701");
        assert_eq!(xb1.variants[0].block_spec_ids[28], "wago-2202-1207");

        // -XB2.1 & -XB2.2: cleanly flagged with unknown article 2007-8873 ("Nicht in DB")
        let xb21 = &plan.strips[6];
        assert_eq!(xb21.total_terminal_count, 0);
        assert_eq!(xb21.unknown_articles.len(), 1);
        assert_eq!(xb21.unknown_articles[0].article_number, "WAGO.2007-8873");
    }

    #[test]
    fn instantiate_x3_and_apply_nt_supply_with_carriers() {
        let catalog = TerminalCatalog::load_embedded();
        let rows = hb_hixberg_sample_rows();
        let plan = plan_from_bom_rows(&rows, &catalog);
        let x3_plan = plan.strips.iter().find(|s| s.bmk == "-X3").unwrap();

        let (mut strip, nt_zone) = instantiate_imported_strip(x3_plan, 0, true, &catalog).unwrap();
        let (nt_start, nt_end) = nt_zone.unwrap();
        assert_eq!((nt_start, nt_end), (0, 4));
        assert_eq!(strip.blocks.len(), 44);
        assert!(strip.end_bracket_left && strip.end_bracket_right);

        // Convert the 4 installation terminals to NT (wago-2203-7641) and place 2016-7714 + 2009-305
        let nt_spec = catalog.find_by_id("wago-2203-7641").unwrap();
        strip.reassign_blocks(&[0, 1, 2, 3], nt_spec);
        let sup = catalog.find_by_id("wago-2016-7714").unwrap();
        let (z0, z1) = place_n_supply_in_zone(&mut strip, nt_start, nt_end, sup, true, &catalog);
        let carrier = catalog.find_by_id("wago-2009-305").unwrap();
        let support = catalog.find_by_id("wago-2009-304").unwrap();
        apply_n_busbar_carriers_around(
            &mut strip,
            z0,
            z1,
            &catalog,
            &BusbarCarrierPlan {
                outer_carrier: Some(carrier),
                intermediate_support: Some(support),
                max_unsupported_span_tenths_mm: 2000,
            },
        );

        // Order inside -X3: [2009-305, 2016-7714, 4x 2203-7641, 2009-305, 40x 2204-*]
        assert_eq!(strip.blocks[0].spec_id, "wago-2009-305");
        assert_eq!(strip.blocks[1].spec_id, "wago-2016-7714");
        assert!(strip.blocks[2..6].iter().all(|b| b.spec_id == "wago-2203-7641"));
        assert_eq!(strip.blocks[6].spec_id, "wago-2009-305");
        assert_eq!(strip.blocks.len(), 47);
    }

    #[test]
    fn end_to_end_binary_pdf_parsing_with_eplan_quirks() {
        use lopdf::content::{Content, Operation};
        use lopdf::{dictionary, Document, Object, Stream};

        fn text_op(x: f32, y: f32, txt: &str) -> Vec<Operation> {
            vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), 9.into()]),
                Operation::new("Tm", vec![1.into(), 0.into(), 0.into(), 1.into(), x.into(), y.into()]),
                Operation::new("Tj", vec![Object::string_literal(txt)]),
                Operation::new("ET", vec![]),
            ]
        }

        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
            "Encoding" => "WinAnsiEncoding",
        });
        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });

        let mut add_page = |ops: Vec<Operation>| {
            let encoded = Content { operations: ops }.encode().unwrap();
            let content_id = doc.add_object(Stream::new(dictionary! {}, encoded));
            doc.add_object(dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "MediaBox" => vec![0.into(), 0.into(), 842.into(), 595.into()],
                "Resources" => resources_id,
                "Contents" => content_id,
            })
        };

        // Page 1: Header row with slipped "Typnummer" sharing Y with row 1!
        let mut ops1 = Vec::new();
        // Header row at y=540 (note Typnummer slipped down onto y=520!)
        ops1.extend(text_op(40.0, 540.0, "Betriebsmittelkennzeichen"));
        ops1.extend(text_op(160.0, 540.0, "Menge"));
        ops1.extend(text_op(210.0, 540.0, "Bezeichnung"));
        ops1.extend(text_op(480.0, 540.0, "Lieferant"));
        ops1.extend(text_op(560.0, 540.0, "Artikelnummer"));

        // Row 1 at y=520: has slipped "Typnummer" in col 3 along with -X0.1 data
        ops1.extend(text_op(380.0, 520.0, "Typnummer"));
        ops1.extend(text_op(40.0, 520.0, "-X0.1"));
        ops1.extend(text_op(160.0, 520.0, "6"));
        ops1.extend(text_op(210.0, 520.0, "Durchgangs-Reihenklemme"));
        ops1.extend(text_op(380.0, 514.0, "285-195"));
        ops1.extend(text_op(560.0, 520.0, "WAGO.285-195"));

        // Distractor -12T1 with WAGO.879-3120 (empty supplier) -> MUST be filtered out!
        ops1.extend(text_op(40.0, 495.0, "-12T1"));
        ops1.extend(text_op(160.0, 495.0, "1"));
        ops1.extend(text_op(210.0, 495.0, "Trenntrafo"));
        ops1.extend(text_op(380.0, 495.0, "879-3120"));
        ops1.extend(text_op(560.0, 495.0, "WAGO.879-3120"));

        // -X3 part 1 on Page 1 with merged child row (empty BMK on second line)
        ops1.extend(text_op(40.0, 470.0, "-X3"));
        ops1.extend(text_op(160.0, 470.0, "4"));
        ops1.extend(text_op(210.0, 470.0, "Dreistock-Installationsklemme"));
        ops1.extend(text_op(380.0, 470.0, "2203-7646"));
        ops1.extend(text_op(560.0, 470.0, "WAGO.2203-7646"));

        ops1.extend(text_op(160.0, 450.0, "24"));
        ops1.extend(text_op(210.0, 450.0, "Durchgangs-Reihenklemme"));
        ops1.extend(text_op(380.0, 450.0, "2204-1201"));
        ops1.extend(text_op(560.0, 450.0, "WAGO.2204-1201"));

        // Footer line (must be ignored)
        ops1.extend(text_op(40.0, 30.0, "Artikelstückliste: ETN.236948 - HORA.080220-1-3"));
        ops1.extend(text_op(650.0, 30.0, "Blatt 33.a von 35"));

        // Page 2: continuation of -X3 + -XB1
        let mut ops2 = Vec::new();
        ops2.extend(text_op(40.0, 520.0, "-X3"));
        ops2.extend(text_op(160.0, 520.0, "8"));
        ops2.extend(text_op(210.0, 520.0, "Durchgangs-Reihenklemme"));
        ops2.extend(text_op(380.0, 520.0, "2204-1204"));
        ops2.extend(text_op(560.0, 520.0, "WAGO.2204-1204"));

        ops2.extend(text_op(160.0, 500.0, "8"));
        ops2.extend(text_op(210.0, 500.0, "Schutzleiter-Reihenklemme"));
        ops2.extend(text_op(380.0, 500.0, "2204-1207"));
        ops2.extend(text_op(560.0, 500.0, "WAGO.2204-1207"));

        ops2.extend(text_op(40.0, 470.0, "-XP"));
        ops2.extend(text_op(160.0, 470.0, "1"));
        ops2.extend(text_op(210.0, 470.0, "Kabeldurchführung"));
        ops2.extend(text_op(380.0, 470.0, "KDP-24"));
        ops2.extend(text_op(480.0, 470.0, "MCT"));
        ops2.extend(text_op(560.0, 470.0, "MCT.KDP-24"));

        let p1 = add_page(ops1);
        let p2 = add_page(ops2);
        doc.objects.insert(
            pages_id,
            Object::Dictionary(dictionary! {
                "Type" => "Pages",
                "Kids" => vec![p1.into(), p2.into()],
                "Count" => 2,
            }),
        );
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);

        let mut pdf_bytes = Vec::new();
        doc.save_to(&mut pdf_bytes).unwrap();

        let catalog = TerminalCatalog::load_embedded();
        let plan = import_pdf_bytes(&pdf_bytes, &catalog).unwrap();

        assert_eq!(plan.ignored_non_x_rows.len(), 1);
        assert_eq!(plan.ignored_non_x_rows[0].bmk, "-12T1");
        assert_eq!(plan.skipped_foreign.len(), 1);
        assert_eq!(plan.skipped_foreign[0].bmk, "-XP");

        assert_eq!(plan.strips.len(), 2);
        assert_eq!(plan.strips[0].bmk, "-X0.1");
        assert_eq!(plan.strips[0].total_terminal_count, 6);
        assert_eq!(plan.strips[1].bmk, "-X3");
        assert_eq!(plan.strips[1].total_terminal_count, 44);
        assert_eq!(plan.strips[1].pages, vec![1, 2]);
    }

    #[test]
    fn sequential_nt_queue_triggers_only_on_genuine_nt_terminals() {
        let catalog = TerminalCatalog::load_embedded();
        let rows = hb_hixberg_sample_rows();
        let plan = plan_from_bom_rows(&rows, &catalog);

        // Non-NT 2203-7646 on -X2 and -X3 must NOT trigger the NT popup,
        // while -X2/-X3 still preserve their contiguous installation zone.
        let nt_strips: Vec<&str> = plan
            .strips
            .iter()
            .filter(|s| s.has_nt_or_install && s.total_terminal_count > 0)
            .map(|s| s.bmk.as_str())
            .collect();
        assert!(nt_strips.is_empty());

        let x2 = plan.strips.iter().find(|s| s.bmk == "-X2").unwrap();
        let (_, z2) = instantiate_imported_strip(x2, 0, true, &catalog).unwrap();
        assert_eq!(z2, Some((0, 24)));

        let x3 = plan.strips.iter().find(|s| s.bmk == "-X3").unwrap();
        let (_, z3) = instantiate_imported_strip(x3, 0, true, &catalog).unwrap();
        assert_eq!(z3, Some((0, 4)));

        // Genuine NT terminals (e.g. WAGO 2203-7641 & Phoenix PTI 2,5-PE/L/NT) MUST queue in order,
        // whereas Phoenix PTI 2,5-L/L (3213953) must NOT trigger the NT popup.
        let r = |bmk: &str, qty, desc: &str, typ: &str, art: &str| RawBomRow {
            page: 1,
            bmk: bmk.to_string(),
            quantity: qty,
            description: desc.to_string(),
            type_number: typ.to_string(),
            supplier: String::new(),
            article_number: art.to_string(),
        };
        let genuine_rows = vec![
            r("-X10", 4, "Installationsklemme L/L", "PTI 2,5-L/L", "3213953"),
            r("-X11", 6, "Dreistock-NT-Klemme", "2203-7641", "WAGO.2203-7641"),
            r("-X12", 4, "Installations-NT-Klemme", "PTI 2,5-PE/L/NT", "3213955"),
        ];
        let nt_plan = plan_from_bom_rows(&genuine_rows, &catalog);
        let queued: Vec<&str> = nt_plan
            .strips
            .iter()
            .filter(|s| s.has_nt_or_install && s.total_terminal_count > 0)
            .map(|s| s.bmk.as_str())
            .collect();
        assert_eq!(queued, vec!["-X11", "-X12"]);
    }
}
