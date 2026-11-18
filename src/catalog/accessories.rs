use serde::{Deserialize, Serialize};

use crate::catalog::TerminalSpec;

/// Matching manufacturer accessories for a terminal block specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TerminalAccessories {
    /// End cover / partition plate (Enddeckel / Abschlussplatte).
    pub end_plate: String,
    /// Cross-connectors / jumpers (Brücken / Querverbinder).
    pub jumpers: String,
    /// Compatible marker strips / tags (Beschriftungsmaterial).
    pub markers: String,
    /// End clamp / bracket (Endklammer / Endhalter).
    pub end_bracket: String,
    /// Optional extra accessory (e.g. N-busbar, fuse holder, test plug).
    pub extra: Option<String>,
}

/// Resolves real-world accessories (end plates, jumpers, markers, end brackets)
/// for any built-in or custom `TerminalSpec`.
pub fn resolve_accessories(spec: &TerminalSpec) -> TerminalAccessories {
    match spec.manufacturer.as_str() {
        "WAGO" => resolve_wago(spec),
        "Phoenix Contact" => resolve_phoenix(spec),
        "Weidmüller" => resolve_weidmuller(spec),
        _ => TerminalAccessories {
            end_plate: format!("Endplatte ({:.1} mm)", f32::from(spec.width_tenths_mm) / 10.0),
            jumpers: format!("Querverbinder Raster {:.1} mm", f32::from(spec.width_tenths_mm) / 10.0),
            markers: "Beschriftungsstreifen".to_string(),
            end_bracket: "TS35 Endklammer".to_string(),
            extra: None,
        },
    }
}

fn resolve_wago(spec: &TerminalSpec) -> TerminalAccessories {
    let id = spec.id.as_str();
    let end_bracket = "249-116 (grau) / 249-117 (schwarz)".to_string();
    let markers_strip = "2009-110 (Beschriftungsstreifen) / 2009-115 (WMB)".to_string();

    if id.starts_with("wago-2000") || id.starts_with("wago-2200") {
        let end_plate = if spec.levels >= 3 {
            "2000-3291 (3-Etagen)"
        } else if spec.levels == 2 {
            "2000-2291 (2-Etagen)"
        } else if id.contains("-140") {
            "2000-1491 (4-Leiter)"
        } else if id.contains("-130") {
            "2000-1391 (3-Leiter)"
        } else {
            "2000-1291 (2-Leiter)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "2000-402 (2-fach) .. 2000-410 (10-fach)".to_string(),
            markers: "2009-110 (Streifen) / 793-3501 (WMB 3,5mm)".to_string(),
            end_bracket,
            extra: Some("2000-433..440 (Brücker 1 auf 3..10)".to_string()),
        };
    }

    if id.starts_with("wago-2001") || id.starts_with("wago-2201") {
        let end_plate = if spec.levels >= 2 {
            "2001-2291 (2-Etagen)"
        } else if id.contains("-140") {
            "2001-1491 (4-Leiter)"
        } else if id.contains("-130") {
            "2001-1391 (3-Leiter)"
        } else {
            "2001-1291 (2-Leiter)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "2001-402 (2-fach) .. 2001-410 (10-fach)".to_string(),
            markers: "2009-110 (Streifen) / 793-4501 (WMB 4,2mm)".to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.starts_with("wago-2003") || id.starts_with("wago-2203") {
        return TerminalAccessories {
            end_plate: "2003-7692 (orange) / 2003-7691 (grau)".to_string(),
            jumpers: "2002-402 (2-fach) .. 2002-410 (10-fach)".to_string(),
            markers: markers_strip,
            end_bracket,
            extra: Some("210-133 (N-Sammelschiene 10x3 Cu) + 2016-7714 (16mm² N-Einspeiseklemme)".to_string()),
        };
    }

    if id.starts_with("wago-2005") || id.starts_with("wago-2205") {
        let end_plate = if id.starts_with("wago-2205") {
            "2205-7692 (orange) / 2205-7691 (grau)"
        } else {
            "2005-7692 (orange) / 2005-7691 (grau)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "2004-402 (2-fach) .. 2004-410 (10-fach)".to_string(),
            markers: markers_strip,
            end_bracket,
            extra: Some("210-133 (N-Sammelschiene 10x3 Cu) + 2016-7714 (16mm² N-Einspeiseklemme)".to_string()),
        };
    }

    if id.starts_with("wago-2002") || id.starts_with("wago-2202") || id.starts_with("wago-2102") {
        let end_plate = if id.contains("-261") {
            "2002-2691 (Doppelstock-Trenn)"
        } else if id.contains("-296") {
            "2002-2991 (4-Leiter Doppelstock)"
        } else if spec.levels == 3 {
            "2002-3291 (3-Etagen)"
        } else if spec.levels == 2 {
            "2002-2291 (2-Etagen)"
        } else if id.contains("-161") || id.contains("-166") || id.contains("-181") {
            "2002-1691 (Sicherung/Trenn)"
        } else if id.contains("-198") {
            "2002-1991 (4-Leiter Trenn)"
        } else if id.contains("-140") {
            "2002-1491 (4-Leiter)"
        } else if id.contains("-130") || id.contains("-171") {
            "2002-1391 (3-Leiter)"
        } else if id.starts_with("wago-2102") {
            "2102-1291 (Hebel 2-Leiter)"
        } else {
            "2002-1291 (2-Leiter)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "2002-402 (2-fach) .. 2002-410 (10-fach), Stufenbrücker 2002-472".to_string(),
            markers: markers_strip,
            end_bracket,
            extra: if spec.levels >= 2 {
                Some("2002-492 (Vertikalbrücker Etage-zu-Etage)".to_string())
            } else {
                None
            },
        };
    }

    if id.starts_with("wago-2004") || id.starts_with("wago-2204") || id.starts_with("wago-2104") {
        let end_plate = if id.contains("-140") {
            "2004-1491 (4-Leiter)"
        } else if id.contains("-130") {
            "2004-1391 (3-Leiter)"
        } else if id.starts_with("wago-2104") {
            "2104-1291 (Hebel 2-Leiter)"
        } else {
            "2004-1291 (2-Leiter)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "2004-402 (2-fach) .. 2004-410 (10-fach)".to_string(),
            markers: markers_strip,
            end_bracket,
            extra: None,
        };
    }

    if id.starts_with("wago-2006") || id.starts_with("wago-2206") || id.starts_with("wago-2106") {
        let end_plate = if id.contains("-161") || id.contains("-168") {
            "2006-1691 (Sicherung)"
        } else if id.contains("-130") {
            "2006-1391 (3-Leiter)"
        } else if id.starts_with("wago-2106") {
            "2106-1291 (Hebel 6mm²)"
        } else {
            "2006-1291 (2-Leiter 6mm²)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "2006-402 (2-fach) .. 2006-405 (5-fach)".to_string(),
            markers: markers_strip,
            end_bracket,
            extra: None,
        };
    }

    if id.starts_with("wago-2010") || id.starts_with("wago-2210") || id.starts_with("wago-2110") {
        let end_plate = if id.contains("-130") {
            "2010-1391 (3-Leiter 10mm²)"
        } else {
            "2010-1291 (2-Leiter 10mm²)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "2010-402 (2-fach) .. 2010-405 (5-fach)".to_string(),
            markers: markers_strip,
            end_bracket,
            extra: None,
        };
    }

    if id.starts_with("wago-2016") || id.starts_with("wago-2216") || id.starts_with("wago-2116") {
        let (end_plate, extra) = if id.contains("-7714") {
            (
                "2016-7792 (orange) / 2016-7791 (grau)",
                Some("210-133 (N-Sammelschiene 10x3 Cu) + 2009-305 (Sammelschienenhalter)".to_string()),
            )
        } else if id.contains("-7614") {
            (
                "2016-7692 (orange) / 2016-7691 (grau)",
                Some("210-133 (N-Sammelschiene 10x3 Cu)".to_string()),
            )
        } else if id.contains("-130") {
            ("2016-1391 (3-Leiter 16mm²)", None)
        } else {
            ("2016-1291 (2-Leiter 16mm²)", None)
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "2016-402 (2-fach) .. 2016-405 (5-fach)".to_string(),
            markers: markers_strip,
            end_bracket,
            extra,
        };
    }

    if id.starts_with("wago-2007") {
        return TerminalAccessories {
            end_plate: "2007-8891 (Trenn-/Abschlussplatte)".to_string(),
            jumpers: "2006-402 .. 2006-405 / 2007-8442".to_string(),
            markers: markers_strip,
            end_bracket,
            extra: Some("2007-8899 (Prüfsteckerleiste)".to_string()),
        };
    }

    if id.starts_with("wago-285") {
        return TerminalAccessories {
            end_plate: "Geschlossenes POWER-Gehäuse (kein Deckel)".to_string(),
            jumpers: "285-435 / 285-447 (2-fach Brücker)".to_string(),
            markers: "793-5501 (WMB 5mm)".to_string(),
            end_bracket: "249-117 (Endklammer 10mm)".to_string(),
            extra: None,
        };
    }

    if id.starts_with("wago-202") {
        return TerminalAccessories {
            end_plate: "2022-1291 (X-COM S-SYSTEM)".to_string(),
            jumpers: "2002-402 .. 2002-410".to_string(),
            markers: markers_strip,
            end_bracket,
            extra: Some("2022-101..115 (Stecker 1- bis 15-polig)".to_string()),
        };
    }

    let end_plate_280 = if id.starts_with("wago-281") {
        "281-317 (Sicherung 4mm²)"
    } else if id.contains("-560") {
        "280-320 (Sensor/Aktor)"
    } else if id.contains("-833") {
        "280-314 (4-Leiter)"
    } else if id.contains("-681") {
        "280-324 (3-Leiter)"
    } else {
        "280-308 (2-Leiter)"
    };
    TerminalAccessories {
        end_plate: end_plate_280.to_string(),
        jumpers: "280-402 (2-fach) / 280-409 (9-fach)".to_string(),
        markers: "793-5501 (WMB 5mm)".to_string(),
        end_bracket,
        extra: None,
    }
}

fn resolve_phoenix(spec: &TerminalSpec) -> TerminalAccessories {
    let id = spec.id.as_str();
    let end_bracket = "CLIPFIX 35 (3022218) / CLIPFIX 35-5 (3022276)".to_string();

    if id.contains("pt-15s") || id.contains("pttb-15s") || id.contains("ptio-15s") {
        let end_plate = if id.contains("ptio") {
            "D-PTIO 1,5/S (3244575)"
        } else if spec.levels >= 3 {
            "D-PT 1,5/S-3L (3113771)"
        } else if spec.levels == 2 {
            "D-PTTB 1,5/S (3208579)"
        } else if id.contains("-mt") {
            "D-PT 1,5/S-MT-0,8 (3210303)"
        } else if id.contains("quattro") {
            "D-PT 1,5/S-QUATTRO (3208375)"
        } else if id.contains("twin") {
            "D-PT 1,5/S-TWIN (3208184)"
        } else {
            "D-PT 1,5/S (3208142)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "FBS 2-3,5 (3213014) .. FBS 10-3,5 (3213056)".to_string(),
            markers: "ZBF 3,5 / UCT-TMF 3,5 / TMT (EX9,5)".to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("pti-25") || id.contains("sti-25") || id.contains("uti-25") {
        let end_plate = if id.contains("uti") {
            "D-UTI 2,5 (3076038)"
        } else if id.contains("sti") {
            "D-STI/3 (3038626)"
        } else if spec.levels >= 3 {
            "D-PTI/3 (3213975)"
        } else {
            "D-PTI/2B (3213976)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "FBS 2-5 (3030161) .. FBS 10-5 (3030213)".to_string(),
            markers: "ZBF 5 / UCT-TMF 5 / TMT 5 R".to_string(),
            end_bracket,
            extra: Some("NLS-CU 3/10 SN 1000MM (0402174 N-Sammelschiene) + AB-PTI".to_string()),
        };
    }

    if id.contains("pti-4") {
        return TerminalAccessories {
            end_plate: "D-PTI 4/3 (1093910)".to_string(),
            jumpers: "FBS 2-6 (3030336) .. FBS 10-6 (3030271)".to_string(),
            markers: "ZBF 6 / UCT-TMF 6".to_string(),
            end_bracket,
            extra: Some("NLS-CU 3/10 SN 1000MM (0402174 N-Sammelschiene)".to_string()),
        };
    }

    if id.contains("pti-6") {
        return TerminalAccessories {
            end_plate: "D-PTI 6/3 (1093911)".to_string(),
            jumpers: "FBS 2-8 (3030284) .. FBS 5-8 (3030307)".to_string(),
            markers: "ZBF 8 / UCT-TMF 8".to_string(),
            end_bracket,
            extra: Some("NLS-CU 3/10 SN 1000MM (0402174 N-Sammelschiene) + PTI 16-NLS-FI BU".to_string()),
        };
    }

    if id.contains("pti-16-nls") {
        return TerminalAccessories {
            end_plate: "D-PTI 16-NLS-FI (1030133)".to_string(),
            jumpers: "FBS 2-12 (3005950)".to_string(),
            markers: "ZBF 12 / UCT-TM 12".to_string(),
            end_bracket,
            extra: Some("NLS-CU 3/10 SN 1000MM (0402174 N-Sammelschiene) + AB-PTI 16-NLS".to_string()),
        };
    }

    if id.contains("ptv-25") {
        return TerminalAccessories {
            end_plate: "D-PTV 2,5/4 (1088746)".to_string(),
            jumpers: "FBS 2-5 (3030161) .. FBS 10-5 (3030213)".to_string(),
            markers: "ZBF 5 / UCT-TM 5".to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("ptv-4") {
        return TerminalAccessories {
            end_plate: "D-PTV 2,5/4 (1088746)".to_string(),
            jumpers: "FBS 2-6 (3030336) .. FBS 10-6 (3030271)".to_string(),
            markers: "ZBF 6 / UCT-TM 6".to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("ut-25") || id.contains("uttb-25") {
        let end_plate = if spec.levels >= 2 {
            "D-UTTB 2,5/4 (3047293)"
        } else {
            "D-UT 2,5/10 (3047028)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "FBS 2-5 (3030161) .. FBS 10-5 (3030213)".to_string(),
            markers: "ZB 5 / UCT-TM 5".to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("ut-4") || id.contains("uttb-4") || id.contains("ut-6") || id.contains("ut-10") {
        let end_plate = if spec.levels >= 2 {
            "D-UTTB 2,5/4 (3047293)"
        } else {
            "D-UT 2,5/10 (3047028)"
        };
        let (jumpers, markers) = if spec.width_tenths_mm >= 100 {
            ("FBS 2-10 (3005947)", "ZB 10 / UCT-TM 10")
        } else if spec.width_tenths_mm >= 80 {
            ("FBS 2-8 (3030284)", "ZB 8 / UCT-TM 8")
        } else {
            ("FBS 2-6 (3030336) .. FBS 10-6", "ZB 6 / UCT-TM 6")
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: jumpers.to_string(),
            markers: markers.to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("pt-4") || id.contains("pttb-4") {
        let end_plate = if id.contains("hesi") {
            "Geschlossenes Sicherungsgehäuse (kein Enddeckel nötig)"
        } else if spec.levels >= 2 {
            "D-PTTB 4 (3211867)"
        } else if id.contains("-mt") {
            "D-PT 4-MT (3211940)"
        } else if id.contains("quattro") {
            "D-PT 4-QUATTRO (3208979)"
        } else if id.contains("twin") {
            "D-PT 4-TWIN (3208977)"
        } else {
            "D-ST 4 (3030420)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "FBS 2-6 (3030336) .. FBS 10-6 (3030271)".to_string(),
            markers: "ZBF 6 / UCT-TM 6 / TMT 6 R".to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("pt-6") || id.contains("ptme-6") {
        let end_plate = if id.contains("ptme") {
            "D-PTME 6 (3212169)"
        } else if id.contains("-t") || id.contains("-mt") {
            "D-PT 6-MT (3212173)"
        } else {
            "D-PT 6 (3212044)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "FBS 2-8 (3030284) .. FBS 5-8 (3030307)".to_string(),
            markers: "ZBF 8 / UCT-TM 8".to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("pt-10") {
        return TerminalAccessories {
            end_plate: "D-PT 10 (3212057)".to_string(),
            jumpers: "FBS 2-10 (3005947) .. FBS 5-10".to_string(),
            markers: "ZBF 10 / UCT-TM 10".to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("pt-16") {
        return TerminalAccessories {
            end_plate: "D-PT 16 N (3212060)".to_string(),
            jumpers: "FBS 2-12 (3005950)".to_string(),
            markers: "ZBF 12 / UCT-TM 12".to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("udmtk") || id.contains("ukk-5") {
        let jumpers = if id.contains("udmtkb") || id.contains("ukk-5") {
            "FBI 10-6 (0203250) / ISSBI 10-6 (0301505)"
        } else {
            "Keine Steckbrücke (Trennklemme)"
        };
        let end_plate = if id.contains("ukk-5") {
            "Geschlossenes Doppelstock-Gehäuse (optional: D-UK 5-MTK 2716100)"
        } else {
            "Geschlossenes Gehäuse (kein Enddeckel nötig)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: jumpers.to_string(),
            markers: "ZB 6 / UCT-TM 6".to_string(),
            end_bracket,
            extra: Some("Optional: Trennplatte TP-UDMTK 5 (3100266)".to_string()),
        };
    }

    if id.contains("uk-5") {
        return TerminalAccessories {
            end_plate: "D-UK 4/10 (3003020)".to_string(),
            jumpers: "FBRI 10-5 N (2770642)".to_string(),
            markers: "ZB 5 / UCT-TM 5".to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("pt-35")
        || id.contains("ptpower")
        || id.contains("ut-16")
        || id.contains("ut-35")
        || id.contains("uslkg")
    {
        return TerminalAccessories {
            end_plate: "Geschlossenes Gehäuse (kein Enddeckel nötig)".to_string(),
            jumpers: "Einlegebrücke / Hochstrombrücke".to_string(),
            markers: "ZBF 12 / ZB 15".to_string(),
            end_bracket: "E/AL-NS 35 (1201662)".to_string(),
            extra: None,
        };
    }

    // Default PT 2,5 / PTTB 2,5 / ST 2,5 family (5.2 mm)
    let end_plate = if id.contains("hesi") {
        "Geschlossenes Sicherungsgehäuse (kein Enddeckel nötig)"
    } else if spec.levels == 3 {
        "D-PT 2,5-3L (3211647)"
    } else if spec.levels == 2 {
        if id.contains("pttbs") {
            "D-PTTBS 2,5 (3209617)"
        } else if id.contains("sttb") {
            "D-STTB 2,5 (3030459)"
        } else {
            "D-PTTB 2,5 (3211634)"
        }
    } else if id.contains("quattro") {
        "D-ST 2,5-QUATTRO (3030514)"
    } else if id.contains("twin") || id.contains("-mt") || id.contains("-tg") {
        "D-ST 2,5-TWIN (3030488)"
    } else {
        "D-ST 2,5 (3030417)"
    };

    TerminalAccessories {
        end_plate: end_plate.to_string(),
        jumpers: "FBS 2-5 (3030161) .. FBS 10-5 (3030213)".to_string(),
        markers: "ZBF 5 / UCT-TM 5 / TMT 5 R".to_string(),
        end_bracket,
        extra: if spec.levels >= 2 {
            Some("FBS-PV (Vertikalbrückung Potentialverteilung)".to_string())
        } else {
            None
        },
    }
}

fn resolve_weidmuller(spec: &TerminalSpec) -> TerminalAccessories {
    let id = spec.id.as_str();
    let end_bracket = "AEB 35 SC/1 (1991920000) / WEW 35/2 (1061200000)".to_string();

    if id.contains("aitb") {
        let (end_plate, jumpers, markers) = if id.contains("aitb-16") {
            (
                "AEP ITB 16 BB (2668980000)",
                "ZQV 16N/2 (2497290000)",
                "DEK 5/8 / WS 10/8",
            )
        } else if id.contains("aitb-4") {
            (
                "AEP ITB 4 (2669120000)",
                "ZQV 4N/2 .. ZQV 4N/10",
                "DEK 5/6 / WS 10/6",
            )
        } else if id.contains("aitb-6") {
            (
                "AEP ITB 6 (2669160000)",
                "ZQV 6N/2 .. ZQV 6N/6",
                "DEK 5/8 / WS 10/8",
            )
        } else {
            (
                "AEP ITB 2.5 (2668960000)",
                "ZQV 2.5N/2 (1527540000) .. ZQV 2.5N/10",
                "DEK 5/5 / WS 10/5",
            )
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: jumpers.to_string(),
            markers: markers.to_string(),
            end_bracket,
            extra: Some("SSch 10x3x1000 CU (0571300000 N-Sammelschiene) + AITB 16 BB NDT BL".to_string()),
        };
    }

    if id.contains("a2c-15") || id.contains("a3c-15") || id.contains("a4c-15") || id.contains("aio21") {
        let end_plate = if id.contains("aio21") {
            "AEP IO21 (1993580000)"
        } else if id.contains("a4c-15") {
            "AEP 4C 1.5 (1552640000)"
        } else if id.contains("a3c-15") {
            "AEP 3C 1.5 (1552620000)"
        } else {
            "AEP 2C 1.5 (1552600000)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "ZQV 1.5N/2 (1985410000) .. ZQV 1.5N/10".to_string(),
            markers: "DEK 5/3.5 / WS 10/3.5".to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("a2c-25")
        || id.contains("a3c-25")
        || id.contains("a4c-25")
        || id.contains("a2t-25")
        || id.contains("a3t-25")
        || id.contains("amc-25")
        || id.contains("adt-25")
    {
        let end_plate = if id.contains("amc-25") {
            "AEP MC 2.5 (2434380000)"
        } else if id.contains("adt-25") {
            "AEP DT 2.5 2C (1989880000)"
        } else if spec.levels == 3 {
            "AEP 3T 2.5 (1547700000)"
        } else if spec.levels == 2 {
            "AEP 2T 2.5 (1547690000)"
        } else if id.contains("a4c") {
            "AEP 4C 2.5 (1521530000)"
        } else if id.contains("a3c") {
            "AEP 3C 2.5 (1521510000)"
        } else {
            "AEP 2C 2.5 (1514400000)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "ZQV 2.5N/2 (1527540000) .. ZQV 2.5N/10 (1527690000)".to_string(),
            markers: "DEK 5/5 / WS 10/5 / MultiCard".to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("a2c-4")
        || id.contains("a3c-4")
        || id.contains("a4c-4")
        || id.contains("a2t-4")
        || id.contains("afs-4")
    {
        let end_plate = if id.contains("afs-4") {
            "Geschlossenes Sicherungsgehäuse (kein Enddeckel nötig)"
        } else if spec.levels == 2 {
            "AEP 2T 4 (2540030000)"
        } else if id.contains("a4c") {
            "AEP 4C 4 (2051920000)"
        } else if id.contains("a3c") {
            "AEP 3C 4 (2051890000)"
        } else {
            "AEP 2C 4 (2051680000)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "ZQV 4N/2 (1527930000) .. ZQV 4N/10".to_string(),
            markers: "DEK 5/6 / WS 10/6".to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("a2c-6") || id.contains("a2c-10") || id.contains("a2c-16") || id.contains("a2c-35")
    {
        let (end_plate, jumpers, markers) = if id.contains("a2c-35") {
            ("Geschlossenes Gehäuse (kein Enddeckel nötig)", "ZQV 35N/2", "DEK 5/8")
        } else if id.contains("a2c-16") {
            ("AEP 2C 16 (2490430000)", "ZQV 16N/2 (2497290000)", "DEK 5/8")
        } else if id.contains("a2c-10") {
            ("AEP 2C 10 (2490380000)", "ZQV 10N/2 (2497250000)", "DEK 5/8")
        } else {
            ("AEP 2C 6 (1991970000)", "ZQV 6N/2 (1985740000) .. ZQV 6N/6", "DEK 5/8")
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: jumpers.to_string(),
            markers: markers.to_string(),
            end_bracket,
            extra: None,
        };
    }

    if id.contains("zdu") || id.contains("zpe") || id.contains("zdk") || id.contains("zsi") || id.contains("zdl") {
        let end_plate = if id.contains("zdl") {
            "ZAP/TW ZDL 2.5 (1720910000)"
        } else if spec.levels >= 2 {
            "ZAP/TW ZDK2.5 (1674730000)"
        } else if id.contains("-4") {
            "ZAP/TW 4 (1632090000)"
        } else {
            "ZAP/TW 1 (1608740000)"
        };
        let jumpers = if id.contains("-4") {
            "ZQV 4/2 (1608950000) .. ZQV 4/10"
        } else {
            "ZQV 2.5/2 (1608860000) .. ZQV 2.5/10"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: jumpers.to_string(),
            markers: "DEK 5/5 / WS 12/5".to_string(),
            end_bracket,
            extra: None,
        };
    }

    // W-Series (WDU / WPE / WDK / DLI / WTR / WSI)
    let end_plate = if id.contains("wsi") || id.contains("wdu-35") {
        "Geschlossenes Gehäuse (kein Enddeckel nötig)"
    } else if id.contains("dli") {
        "WAP DLI 2.5 (1783550000)"
    } else if id.contains("wdk") {
        "WAP WDK2.5 (1059100000)"
    } else if id.contains("wdu-16") || id.contains("wpe-16") {
        "WAP 16+35 WTW 2.5-10 (1050100000)"
    } else {
        "WAP 2.5-10 (1050000000)"
    };
    let jumpers = if spec.width_tenths_mm >= 100 {
        "WQV 10/2 .. WQV 16/2"
    } else if spec.width_tenths_mm >= 79 {
        "WQV 6/2 (1052360000)"
    } else if spec.width_tenths_mm >= 60 {
        "WQV 4/2 (1051960000) / ZQV 4N/2"
    } else {
        "WQV 2.5/2 (1053660000) .. WQV 2.5/10"
    };

    TerminalAccessories {
        end_plate: end_plate.to_string(),
        jumpers: jumpers.to_string(),
        markers: "DEK 5/5 / WS 12/5".to_string(),
        end_bracket,
        extra: if id.contains("dli") {
            Some("SSch 10x3x1000 CU (N-Sammelschiene)".to_string())
        } else {
            None
        },
    }
}

/// Real physical thickness of the end/partition plate for `spec` in tenths of a millimeter (`0.1 mm`),
/// or `0` if the terminal block has a closed housing requiring no separate end plate.
pub fn end_plate_width_tenths_mm(spec: &TerminalSpec) -> u16 {
    let id = spec.id.as_str();
    match spec.manufacturer.as_str() {
        "WAGO" => {
            if id.starts_with("wago-285") {
                0
            } else if id.starts_with("wago-2000") || id.starts_with("wago-2200") {
                7 // 0.7 mm (2000-1291 / 2000-2291)
            } else if id.contains("-7714") {
                8 // 0.8 mm (2016-7792 for 2016-7714 / 2216-7714)
            } else if id.starts_with("wago-2205")
                || id.starts_with("wago-2006")
                || id.starts_with("wago-2106")
                || id.starts_with("wago-2206")
                || id.starts_with("wago-2007")
                || id.starts_with("wago-2010")
                || id.starts_with("wago-2110")
                || id.starts_with("wago-2210")
                || id.starts_with("wago-2016")
                || id.starts_with("wago-2116")
                || id.starts_with("wago-2216")
                || id.starts_with("wago-202")
            {
                10 // 1.0 mm (2205-7692 / 2006-1291 / 2010-1291 / 2016-1291)
            } else if id.starts_with("wago-28") {
                25 // 2.5 mm (280-308 / 281-317)
            } else {
                8 // 0.8 mm (TOPJOB S 2001..2005 / 2102..2104 / 2201..2204)
            }
        }
        "Phoenix Contact" => {
            if id.contains("pt-35")
                || id.contains("ptpower")
                || id.contains("ut-16")
                || id.contains("ut-35")
                || id.contains("uslkg")
                || id.contains("hesi")
                || id.contains("udmtk")
                || id.contains("ukk-5")
            {
                0
            } else if id.contains("pt-15s-mt") {
                8 // 0.8 mm (D-PT 1,5/S-MT-0,8)
            } else if id.contains("uk-") {
                15 // 1.5 mm (D-UK 4/10)
            } else {
                22 // 2.2 mm (CLIPLINE complete D-PT / D-PTTB / D-PTI / D-ST / D-UT)
            }
        }
        "Weidmüller" => {
            if id.contains("a2c-35")
                || id.contains("wdu-35")
                || id.contains("wsi")
                || id.contains("afs")
            {
                0
            } else if id.contains("wdu")
                || id.contains("wpe")
                || id.contains("wdk")
                || id.contains("dli")
                || id.contains("wtr")
                || id.contains("a2c-15")
                || id.contains("a3c-15")
                || id.contains("a4c-15")
            {
                15 // 1.5 mm (WAP 2.5-10 / WAP WDK2.5 / AEP 2C 1.5)
            } else {
                20 // 2.0 mm (AEP 2C 2.5..16 / AEP 2T/3T / AEP ITB / ZAP/TW)
            }
        }
        _ => 15,
    }
}

/// Automatically detected end/partition plate placement after block `after_block_idx`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndPlatePlacement {
    /// 0-based index of the block immediately to the left of this end plate.
    pub after_block_idx: usize,
    /// Manufacturer of the terminal block requiring this plate.
    pub manufacturer: String,
    /// End plate part designation (e.g., "2003-7692 (orange) / 2003-7691 (grau)").
    pub end_plate: String,
    /// Real physical width of the end plate in tenths of a millimeter (e.g., 8 = 0.8 mm, 22 = 2.2 mm).
    pub width_tenths_mm: u16,
    /// Minimum visual row covered by this plate (`-1` if PE, `0` if E1).
    pub min_visual_row: isize,
    /// Maximum visual row covered by this plate (`0` = E1, `1` = E2, `2` = E3).
    pub max_visual_row: isize,
}

/// Computes all required end/partition plates (`Abschlussdeckel`) along `strip`
/// at contour transitions (where adjacent blocks do not share the same end-plate profile)
/// and at the right-most end of the strip.
pub fn calculate_end_plates(
    strip: &crate::model::TerminalStrip,
    catalog: &crate::catalog::TerminalCatalog,
) -> Vec<EndPlatePlacement> {
    let mut plates = Vec::new();
    for (idx, block) in strip.blocks.iter().enumerate() {
        let Some(spec) = catalog.find_by_id(&block.spec_id) else {
            continue;
        };
        let plate_w = end_plate_width_tenths_mm(spec);
        if plate_w == 0 {
            continue;
        }
        let acc = catalog.accessories_for(spec);
        let need_plate = match strip.blocks.get(idx + 1) {
            None => true,
            Some(next_block) => {
                if let Some(next_spec) = catalog.find_by_id(&next_block.spec_id) {
                    let next_acc = catalog.accessories_for(next_spec);
                    next_acc.end_plate != acc.end_plate || next_block.manufacturer != block.manufacturer
                } else {
                    true
                }
            }
        };
        if need_plate {
            let min_row = (0..block.tiers.len())
                .map(|l| block.tier_visual_row(l))
                .min()
                .unwrap_or(0);
            let max_row = (0..block.tiers.len())
                .map(|l| block.tier_visual_row(l))
                .max()
                .unwrap_or(0);
            plates.push(EndPlatePlacement {
                after_block_idx: idx,
                manufacturer: block.manufacturer.clone(),
                end_plate: acc.end_plate,
                width_tenths_mm: plate_w,
                min_visual_row: min_row,
                max_visual_row: max_row,
            });
        }
    }
    plates
}

/// Technical specification of a TS 35 / NS 35 DIN-rail end bracket (`Endhalter` / `Endklammer`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndBracketSpec {
    pub id: &'static str,
    pub manufacturer: &'static str,
    pub name: &'static str,
    pub article_number: &'static str,
    /// Physical width on the DIN rail in tenths of a millimeter (e.g. `52` = `5.2 mm`).
    pub width_tenths_mm: u16,
    /// Front-view vertical height across the TS 35 DIN rail in tenths of a millimeter (e.g. `486` = `48.6 mm`).
    pub height_tenths_mm: u16,
    pub mounting_de: &'static str,
    pub mounting_en: &'static str,
    pub product_url: &'static str,
    pub image_url: Option<&'static str>,
}

/// Default DIN-rail end bracket ID (`Phoenix Contact CLIPFIX 35-5`).
pub const DEFAULT_END_BRACKET_ID: &str = "phoenix-clipfix-35-5";

/// Verified database of TS 35 / NS 35 DIN-rail end brackets from Phoenix Contact, Weidmüller, and WAGO.
pub static END_BRACKETS: &[EndBracketSpec] = &[
    // --- Phoenix Contact ---
    EndBracketSpec {
        id: "phoenix-clipfix-35-5",
        manufacturer: "Phoenix Contact",
        name: "CLIPFIX 35-5",
        article_number: "3022276",
        width_tenths_mm: 52,
        height_tenths_mm: 486,
        mounting_de: "Rastbar / schraubenlos (NS 35/7,5 & NS 35/15)",
        mounting_en: "Snap-on screwless (NS 35/7.5 & NS 35/15)",
        product_url: "https://www.phoenixcontact.com/en-pc/products/end-clamp-clipfix-35-5-3022276",
        image_url: Some("https://caas.phoenixcontact.com/caas/v1/stable/media/202056/full/b408?format=jpg"),
    },
    EndBracketSpec {
        id: "phoenix-clipfix-35-5-v0",
        manufacturer: "Phoenix Contact",
        name: "CLIPFIX 35-5 V0",
        article_number: "3032350",
        width_tenths_mm: 52,
        height_tenths_mm: 485,
        mounting_de: "Rastbar / schraubenlos UL 94 V0 (NS 35)",
        mounting_en: "Snap-on screwless UL 94 V0 (NS 35)",
        product_url: "https://www.phoenixcontact.com/en-pc/products/end-clamp-clipfix-35-5-v0-3032350",
        image_url: Some("https://caas.phoenixcontact.com/caas/v1/stable/media/202056/full/b408?format=jpg"),
    },
    EndBracketSpec {
        id: "phoenix-clipfix-35",
        manufacturer: "Phoenix Contact",
        name: "CLIPFIX 35",
        article_number: "3022218",
        width_tenths_mm: 95,
        height_tenths_mm: 556,
        mounting_de: "Rastbar / schraubenlos (NS 35/7,5 & NS 35/15)",
        mounting_en: "Snap-on screwless (NS 35/7.5 & NS 35/15)",
        product_url: "https://www.phoenixcontact.com/en-pc/products/end-clamp-clipfix-35-3022218",
        image_url: Some("https://caas.phoenixcontact.com/caas/v1/stable/media/11646/full/b408?format=jpg"),
    },
    EndBracketSpec {
        id: "phoenix-e-ns-35-n",
        manufacturer: "Phoenix Contact",
        name: "E/NS 35 N",
        article_number: "0800886",
        width_tenths_mm: 95,
        height_tenths_mm: 486,
        mounting_de: "Schraubmontage (NS 35/7,5 & NS 35/15)",
        mounting_en: "Screw-clamp (NS 35/7.5 & NS 35/15)",
        product_url: "https://www.phoenixcontact.com/en-pc/products/end-clamp-ens-35-n-0800886",
        image_url: Some("https://caas.phoenixcontact.com/caas/v1/stable/media/239820/full/b408?format=jpg"),
    },
    EndBracketSpec {
        id: "phoenix-e-uk",
        manufacturer: "Phoenix Contact",
        name: "E/UK",
        article_number: "1201442",
        width_tenths_mm: 95,
        height_tenths_mm: 505,
        mounting_de: "Schraubmontage (NS 32 & NS 35/7,5)",
        mounting_en: "Screw-clamp (NS 32 & NS 35/7.5)",
        product_url: "https://www.phoenixcontact.com/en-pc/products/end-clamp-e-uk-1201442",
        image_url: Some("https://caas.phoenixcontact.com/caas/v1/stable/media/223114/full/b408?format=jpg"),
    },
    EndBracketSpec {
        id: "phoenix-e-uk-1",
        manufacturer: "Phoenix Contact",
        name: "E/UK 1",
        article_number: "1201413",
        width_tenths_mm: 100,
        height_tenths_mm: 555,
        mounting_de: "Schraubmontage für 2-/3-Etagenklemmen",
        mounting_en: "Screw-clamp for 2-/3-level blocks",
        product_url: "https://www.phoenixcontact.com/en-pc/products/end-clamp-e-uk-1-1201413",
        image_url: Some("https://caas.phoenixcontact.com/caas/v1/stable/media/255612/full/b408?format=jpg"),
    },
    EndBracketSpec {
        id: "phoenix-e-al-ns-35",
        manufacturer: "Phoenix Contact",
        name: "E/AL-NS 35",
        article_number: "1201662",
        width_tenths_mm: 100,
        height_tenths_mm: 590,
        mounting_de: "Schwerlast-Alu (2 Schrauben, Hochstrom)",
        mounting_en: "Heavy-duty aluminum (2-screw, high-current)",
        product_url: "https://www.phoenixcontact.com/en-pc/products/end-clamp-e-al-ns-35-1201662",
        image_url: Some("https://caas.phoenixcontact.com/caas/v1/stable/media/113558/full/b408?format=jpg"),
    },
    // --- Weidmüller ---
    EndBracketSpec {
        id: "weidmuller-aeb-35-scl-1-v0",
        manufacturer: "Weidmüller",
        name: "AEB 35 SCL/1 V0",
        article_number: "2661280000",
        width_tenths_mm: 60,
        height_tenths_mm: 526,
        mounting_de: "Rastbar / schraubenlos A-Reihe V0",
        mounting_en: "Snap-on screwless A-Series V0",
        product_url: "https://eshop.weidmueller.com/en/aeb-35-scl1-v0/p/2661280000",
        image_url: Some("https://assets.dam.weidmueller.com/assets/api/cd27c8f0-1e41-4a89-8f46-2f68a34e52d4/DAP_Master_PP,w_800,q_80"),
    },
    EndBracketSpec {
        id: "weidmuller-aeb-35-sc-1",
        manufacturer: "Weidmüller",
        name: "AEB 35 SC/1",
        article_number: "1991920000",
        width_tenths_mm: 80,
        height_tenths_mm: 580,
        mounting_de: "Schraubmontage A-Reihe V0",
        mounting_en: "Screw-clamp A-Series V0",
        product_url: "https://eshop.weidmueller.com/en/aeb-35-sc1/p/1991920000",
        image_url: Some("https://assets.dam.weidmueller.com/assets/api/cc1fc5e7-36dc-4017-bc0e-29583aa74d4d/DAP_Master_PP,w_800,q_80"),
    },
    EndBracketSpec {
        id: "weidmuller-zew-35",
        manufacturer: "Weidmüller",
        name: "ZEW 35",
        article_number: "9540000000",
        width_tenths_mm: 60,
        height_tenths_mm: 550,
        mounting_de: "Rastbar / schraubenlos Z-Reihe",
        mounting_en: "Snap-on screwless Z-Series",
        product_url: "https://eshop.weidmueller.com/en/zew-35/p/9540000000",
        image_url: Some("https://assets.dam.weidmueller.com/assets/api/73291430-0f06-4ec3-a978-6f50c698ae20/DAP_Master_PP,w_800,q_80"),
    },
    EndBracketSpec {
        id: "weidmuller-zew-35-2",
        manufacturer: "Weidmüller",
        name: "ZEW 35/2",
        article_number: "8630740000",
        width_tenths_mm: 80,
        height_tenths_mm: 550,
        mounting_de: "Rastbar / schraubenlos Z-Reihe (breit)",
        mounting_en: "Snap-on screwless Z-Series (wide)",
        product_url: "https://eshop.weidmueller.com/en/zew-352/p/8630740000",
        image_url: Some("https://assets.dam.weidmueller.com/assets/api/19500219-f550-4003-b1e7-862dc47d98c0/DAP_Master_PP,w_800,q_80"),
    },
    EndBracketSpec {
        id: "weidmuller-wew-35-2",
        manufacturer: "Weidmüller",
        name: "WEW 35/2",
        article_number: "1061200000",
        width_tenths_mm: 80,
        height_tenths_mm: 560,
        mounting_de: "Schraubmontage W-Reihe",
        mounting_en: "Screw-clamp W-Series",
        product_url: "https://eshop.weidmueller.com/en/wew-352/p/1061200000",
        image_url: Some("https://assets.dam.weidmueller.com/assets/api/daf369fb-49b6-4b40-91b8-4a1f5bba2b45/DAP_Master_PP,w_800,q_80"),
    },
    EndBracketSpec {
        id: "weidmuller-wew-35-1",
        manufacturer: "Weidmüller",
        name: "WEW 35/1",
        article_number: "1059000000",
        width_tenths_mm: 120,
        height_tenths_mm: 560,
        mounting_de: "Schraubmontage W-Reihe (stark)",
        mounting_en: "Screw-clamp W-Series (heavy)",
        product_url: "https://eshop.weidmueller.com/en/wew-351/p/1059000000",
        image_url: Some("https://assets.dam.weidmueller.com/assets/api/593f7b0a-b315-4f68-8eb5-0e0cce1bc2ff/DAP_Master_PP,w_800,q_80"),
    },
    EndBracketSpec {
        id: "weidmuller-ew-35",
        manufacturer: "Weidmüller",
        name: "EW 35",
        article_number: "0383560000",
        width_tenths_mm: 85,
        height_tenths_mm: 460,
        mounting_de: "Schraubmontage Standard beige",
        mounting_en: "Screw-clamp standard beige",
        product_url: "https://eshop.weidmueller.com/en/ew-35/p/0383560000",
        image_url: Some("https://assets.dam.weidmueller.com/assets/api/15cc897d-7612-48d7-a1c5-c8298dfd0a53/DAP_Master_PP,w_800,q_80"),
    },
    EndBracketSpec {
        id: "weidmuller-ew-35-gr",
        manufacturer: "Weidmüller",
        name: "EW 35 GR 7032",
        article_number: "0383530000",
        width_tenths_mm: 90,
        height_tenths_mm: 460,
        mounting_de: "Schraubmontage grau V0",
        mounting_en: "Screw-clamp grey V0",
        product_url: "https://eshop.weidmueller.com/en/ew-35-gr-7032/p/0383530000",
        image_url: Some("https://assets.dam.weidmueller.com/assets/api/8c0b24de-b1d1-4e82-9c55-9cb7c8a07e69/DAP_Master_PP,w_800,q_80"),
    },
    EndBracketSpec {
        id: "weidmuller-ew-35-scha-m3",
        manufacturer: "Weidmüller",
        name: "EW 35/SCHA/M3",
        article_number: "0258660000",
        width_tenths_mm: 165,
        height_tenths_mm: 460,
        mounting_de: "Schraubmontage M3 (schwer)",
        mounting_en: "Heavy screw-clamp M3",
        product_url: "https://eshop.weidmueller.com/en/ew-35scham3/p/0258660000",
        image_url: Some("https://assets.dam.weidmueller.com/assets/api/436f023d-d252-4119-867d-924c88bc579e/DAP_Master_PP,w_800,q_80"),
    },
    EndBracketSpec {
        id: "weidmuller-mew-35-1",
        manufacturer: "Weidmüller",
        name: "MEW 35/1",
        article_number: "1805610000",
        width_tenths_mm: 125,
        height_tenths_mm: 562,
        mounting_de: "Metall-Schraubmontage (schwer)",
        mounting_en: "Metal screw-clamp (heavy)",
        product_url: "https://eshop.weidmueller.com/en/mew-351/p/1805610000",
        image_url: Some("https://assets.dam.weidmueller.com/assets/api/e8ed408e-cabd-4148-b954-2757cfd1fbf0/DAP_Master_PP,w_800,q_80"),
    },
    // --- WAGO ---
    EndBracketSpec {
        id: "wago-249-116",
        manufacturer: "WAGO",
        name: "249-116",
        article_number: "249-116",
        width_tenths_mm: 60,
        height_tenths_mm: 440,
        mounting_de: "Schraubenloser Endstopp grau (TS 35)",
        mounting_en: "Screwless end stop grey (TS 35)",
        product_url: "https://www.wago.com/global/accessories/screwless-end-stop/p/249-116",
        image_url: None,
    },
    EndBracketSpec {
        id: "wago-249-117",
        manufacturer: "WAGO",
        name: "249-117",
        article_number: "249-117",
        width_tenths_mm: 100,
        height_tenths_mm: 440,
        mounting_de: "Schraubenloser Endstopp breit (TS 35)",
        mounting_en: "Screwless end stop wide (TS 35)",
        product_url: "https://www.wago.com/gb/accessories/screwless-end-stop/p/249-117",
        image_url: None,
    },
    EndBracketSpec {
        id: "wago-249-197",
        manufacturer: "WAGO",
        name: "249-197",
        article_number: "249-197",
        width_tenths_mm: 140,
        height_tenths_mm: 600,
        mounting_de: "Schwerlast-Endstopp schraubenlos (TS 35)",
        mounting_en: "Heavy-duty screwless end stop (TS 35)",
        product_url: "https://www.wago.com/global/accessories/screwless-end-stop/p/249-197",
        image_url: None,
    },
];

/// Returns all built-in TS 35 DIN-rail end bracket specifications.
pub fn all_end_brackets() -> &'static [EndBracketSpec] {
    END_BRACKETS
}

/// Finds an end bracket by exact ID, returning `None` if not present.
pub fn end_bracket_by_id(id: &str) -> Option<&'static EndBracketSpec> {
    END_BRACKETS.iter().find(|eb| eb.id == id)
}

/// Resolves an end bracket by ID, falling back to `CLIPFIX 35-5` if empty or unknown.
pub fn find_end_bracket(id: &str) -> &'static EndBracketSpec {
    end_bracket_by_id(id)
        .or_else(|| end_bracket_by_id(DEFAULT_END_BRACKET_ID))
        .unwrap_or(&END_BRACKETS[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_end_brackets_have_valid_dimensions_and_urls() {
        assert_eq!(END_BRACKETS.len(), 20);
        for eb in all_end_brackets() {
            assert!(eb.width_tenths_mm >= 50, "width invalid for {}", eb.id);
            assert!(
                eb.height_tenths_mm >= 400 && eb.height_tenths_mm <= 700,
                "height invalid for {}: {}",
                eb.id,
                eb.height_tenths_mm
            );
            assert!(eb.product_url.starts_with("https://"), "url invalid for {}", eb.id);
            if eb.manufacturer == "Phoenix Contact" || eb.manufacturer == "Weidmüller" {
                assert!(
                    eb.image_url.is_some_and(|u| u.starts_with("https://")),
                    "missing CDN image_url for {}",
                    eb.id
                );
            }
        }
    }
}


