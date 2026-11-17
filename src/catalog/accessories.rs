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
        let end_plate = if spec.levels >= 2 {
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
            extra: Some("210-133 (N-Sammelschiene 10x3 Cu) + 2009-304 (Sammelschienenhalter)".to_string()),
        };
    }

    if id.starts_with("wago-2005") {
        return TerminalAccessories {
            end_plate: "2005-7692 (orange) / 2005-7691 (grau)".to_string(),
            jumpers: "2004-402 (2-fach) .. 2004-410 (10-fach)".to_string(),
            markers: markers_strip,
            end_bracket,
            extra: Some("210-133 (N-Sammelschiene 10x3 Cu)".to_string()),
        };
    }

    if id.starts_with("wago-2002") || id.starts_with("wago-2202") || id.starts_with("wago-2102") {
        let end_plate = if spec.levels == 3 {
            "2002-3291 (3-Etagen)"
        } else if spec.levels == 2 {
            "2002-2291 (2-Etagen)"
        } else if id.contains("-161") || id.contains("-181") {
            "2002-1691 (Sicherung/Trenn)"
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
        return TerminalAccessories {
            end_plate: "2010-1291 (2-Leiter) / 2010-1391 (3-Leiter)".to_string(),
            jumpers: "2010-402 (2-fach) .. 2010-405 (5-fach)".to_string(),
            markers: markers_strip,
            end_bracket,
            extra: None,
        };
    }

    if id.starts_with("wago-2016") || id.starts_with("wago-2216") || id.starts_with("wago-2116") {
        return TerminalAccessories {
            end_plate: "2016-1291 (2-Leiter) / 2016-1391 (3-Leiter)".to_string(),
            jumpers: "2016-402 (2-fach) .. 2016-405 (5-fach)".to_string(),
            markers: markers_strip,
            end_bracket,
            extra: None,
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

    TerminalAccessories {
        end_plate: "280-308 (Abschlussplatte)".to_string(),
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
        } else if spec.levels >= 2 {
            "D-PTTB 1,5/S (3208579)"
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

    if id.contains("pti-25") || id.contains("sti-25") {
        let end_plate = if id.contains("sti") {
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
            extra: Some("NLS-CU 3/10 SN 1000MM (0402174 N-Sammelschiene)".to_string()),
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
        let end_plate = if spec.levels >= 2 {
            "D-PTTB 4 (3211867)"
        } else if id.contains("quattro") {
            "D-PT 4-QUATTRO (3208977)"
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

    if id.contains("pt-6") {
        return TerminalAccessories {
            end_plate: "D-PT 6 (3212044)".to_string(),
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

    if id.contains("pt-35") || id.contains("ptpower") || id.contains("ut-16") || id.contains("ut-35")
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
    let end_plate = if spec.levels == 3 {
        "D-PT 2,5-3L (3211647)"
    } else if spec.levels == 2 {
        if id.contains("pttbs") {
            "D-PTTBS 2,5 (3209604)"
        } else if id.contains("sttb") {
            "D-STTB 2,5 (3030459)"
        } else {
            "D-PTTB 2,5 (3211634)"
        }
    } else if id.contains("quattro") {
        "D-ST 2,5-QUATTRO (3030514)"
    } else if id.contains("twin") {
        "D-ST 2,5-TWIN (3030488)"
    } else if id.contains("-mt") || id.contains("-tg") || id.contains("hesi") {
        "D-ST 2,5-MT / D-PT 2,5-TWIN (3030488)"
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
        let (end_plate, jumpers, markers) = if id.contains("aitb-4") {
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
            extra: Some("SSch 10x3x1000 CU (0571300000 N-Sammelschiene)".to_string()),
        };
    }

    if id.contains("a2c-15") || id.contains("a3c-15") || id.contains("a4c-15") {
        return TerminalAccessories {
            end_plate: "AEP 2C 1.5 (1552600000)".to_string(),
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
    {
        let end_plate = if spec.levels == 3 {
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
        let end_plate = if spec.levels == 2 {
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
            ("AEP 2C 35", "ZQV 35N/2", "DEK 5/8")
        } else if id.contains("a2c-16") {
            ("AEP 2C 16 (2490430000)", "ZQV 16N/2 (2497290000)", "DEK 5/8")
        } else if id.contains("a2c-10") {
            ("AEP 2C 10 (2490360000)", "ZQV 10N/2 (2497250000)", "DEK 5/8")
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
        } else {
            "ZAP/TW 1 (1608740000)"
        };
        return TerminalAccessories {
            end_plate: end_plate.to_string(),
            jumpers: "ZQV 2.5/2 (1608860000) .. ZQV 2.5/10".to_string(),
            markers: "DEK 5/5 / WS 12/5".to_string(),
            end_bracket,
            extra: None,
        };
    }

    // W-Series (WDU / WPE / WDK / DLI / WTR / WSI)
    let end_plate = if id.contains("dli") {
        "WAP DLI 2.5 (1783550000)"
    } else if id.contains("wdk") {
        "WAP WDK2.5 (1059100000)"
    } else if id.contains("wdu-16") || id.contains("wdu-35") || id.contains("wpe-16") {
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
