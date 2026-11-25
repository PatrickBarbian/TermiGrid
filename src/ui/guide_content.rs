use crate::catalog::TerminalCatalog;
use crate::i18n::Language;
use crate::ui::guide_modal::build_example_terminal_json;

/// Categories for filtering the integrated TermiGrid manual.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GuideCategory {
    #[default]
    All,
    QuickStart,
    CadEditor,
    PromptAndLibrary,
    BridgingAndAccessories,
    LocalTerminalsJson,
    ExportAndEplan,
    ShortcutsAndUpdates,
}

impl GuideCategory {
    pub const ALL: [Self; 8] = [
        Self::All,
        Self::QuickStart,
        Self::CadEditor,
        Self::PromptAndLibrary,
        Self::BridgingAndAccessories,
        Self::LocalTerminalsJson,
        Self::ExportAndEplan,
        Self::ShortcutsAndUpdates,
    ];

    pub fn label(self, lang: Language) -> &'static str {
        let de = lang == Language::De;
        match self {
            Self::All => if de { "Alle Themen" } else { "All Topics" },
            Self::QuickStart => if de { "Schnellstart & UI" } else { "Quick Start & UI" },
            Self::CadEditor => if de { "CAD & Nummerierung" } else { "CAD & Numbering" },
            Self::PromptAndLibrary => if de { "Assistent & Bibliothek" } else { "Prompt & Library" },
            Self::BridgingAndAccessories => if de { "Brücken & Zubehör" } else { "Bridges & Accessories" },
            Self::LocalTerminalsJson => if de { "Lokale Klemmen & JSON" } else { "Local Terminals & JSON" },
            Self::ExportAndEplan => "Export & EPLAN P8",
            Self::ShortcutsAndUpdates => if de { "Tastenkürzel & Updates" } else { "Shortcuts & Updates" },
        }
    }
}

/// Optional copyable code/prompt block attached to a manual article.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuideCodeKind {
    Static {
        title_de: &'static str,
        title_en: &'static str,
        body_de: &'static str,
        body_en: &'static str,
    },
    AiTerminalPrompt,
    ExampleTerminalJson,
}

/// One searchable handbook entry covering a complete TermiGrid feature area.
#[derive(Debug, Clone, Copy)]
pub struct GuideArticle {
    pub id: &'static str,
    pub category: GuideCategory,
    pub title_de: &'static str,
    pub title_en: &'static str,
    pub summary_de: &'static str,
    pub summary_en: &'static str,
    pub keywords: &'static [&'static str],
    pub steps_de: &'static [(&'static str, &'static str)],
    pub steps_en: &'static [(&'static str, &'static str)],
    pub code: Option<GuideCodeKind>,
}

impl GuideArticle {
    pub fn title(&self, lang: Language) -> &'static str {
        if lang == Language::De {
            self.title_de
        } else {
            self.title_en
        }
    }

    pub fn summary(&self, lang: Language) -> &'static str {
        if lang == Language::De {
            self.summary_de
        } else {
            self.summary_en
        }
    }

    pub fn steps(&self, lang: Language) -> &'static [(&'static str, &'static str)] {
        if lang == Language::De {
            self.steps_de
        } else {
            self.steps_en
        }
    }
}

/// Persistent UI state for the searchable handbook tab.
#[derive(Debug, Clone, Default)]
pub struct GuideSearchState {
    pub query: String,
    pub category: GuideCategory,
    pub copied_feedback_id: Option<String>,
}

/// Returns all handbook articles covering every feature of TermiGrid.
pub fn all_guide_articles() -> &'static [GuideArticle] {
    GUIDE_ARTICLES
}

/// Filters and ranks handbook articles by category and multi-token search query.
pub fn search_guide_articles(
    lang: Language,
    category: GuideCategory,
    query: &str,
) -> Vec<&'static GuideArticle> {
    let tokens: Vec<String> = query
        .split_whitespace()
        .map(|t| t.trim().to_lowercase())
        .filter(|t| !t.is_empty())
        .collect();

    let mut scored: Vec<(u32, usize, &'static GuideArticle)> = Vec::new();
    for (idx, art) in GUIDE_ARTICLES.iter().enumerate() {
        if category != GuideCategory::All && art.category != category {
            continue;
        }
        if tokens.is_empty() {
            scored.push((100, idx, art));
            continue;
        }

        let title_l = art.title(lang).to_lowercase();
        let sum_l = art.summary(lang).to_lowercase();
        let kw_l: Vec<String> = art.keywords.iter().map(|k| k.to_lowercase()).collect();
        let steps_l: Vec<(String, String)> = art
            .steps(lang)
            .iter()
            .map(|(h, b)| (h.to_lowercase(), b.to_lowercase()))
            .collect();

        let mut total_score = 0u32;
        let mut all_match = true;
        for tok in &tokens {
            let mut tok_score = 0u32;
            if title_l.contains(tok) {
                tok_score += 100;
            }
            if kw_l.iter().any(|k| k.contains(tok)) {
                tok_score += 60;
            }
            if sum_l.contains(tok) {
                tok_score += 40;
            }
            for (sh, sb) in &steps_l {
                if sh.contains(tok) {
                    tok_score += 30;
                } else if sb.contains(tok) {
                    tok_score += 15;
                }
            }
            if tok_score == 0 {
                all_match = false;
                break;
            }
            total_score += tok_score;
        }

        if all_match {
            scored.push((total_score, idx, art));
        }
    }

    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    scored.into_iter().map(|(_, _, art)| art).collect()
}

/// Generates a complete, copy-ready LLM prompt (for ChatGPT / Claude / Gemini) that produces
/// 100% schema-valid TermiGrid `TerminalSpec` JSON for any manufacturer datasheet or catalog page.
pub fn build_ai_terminal_prompt(lang: Language, catalog: &TerminalCatalog) -> String {
    let example_json = build_example_terminal_json(catalog);
    if lang == Language::De {
        format!(
            r#"Erstelle mir für das Programm „TermiGrid“ ein gültiges JSON-Array `[ ... ]` (oder einzelnes JSON-Objekt `{{ ... }}`) mit Reihenklemmen-Spezifikationen für folgende Klemme(n):
[HIER HERSTELLER, ARTIKELNUMMER(N) / TYPENBEZEICHNUNG ODER DATENBLATT EINFÜGEN]

Halte dich EXAKT an dieses Schema und diese Regeln:

1. Pflichtfelder & Einheiten:
- "id": Eindeutige Kleinbuchstaben-ID mit Bindestrichen, z. B. "custom-phoenix-3210567" oder "custom-weidmuller-1173890000".
- "name": "<Kurztyp> (<Kurzbeschreibung>)", z. B. "PTTB 2,5 (2-Etagen L/L)". Der Teil vor " (" wird im CAD und Export als Klemmentyp angezeigt.
- "manufacturer": Exakt einer von: "WAGO", "Phoenix Contact" oder "Weidmüller".
- "series": Baureihe (z. B. "TOPJOB S 2002", "PT 2,5", "A-Reihe", "P-Reihe PPV").
- "levels": Anzahl der Beschriftungsebenen (1 bis 4). WICHTIG: Bei Potential-/Rangierverteilern mit nur einem gemeinsamen Schild pro Klemme (z. B. PPV/PRV) auf 1 setzen!
- "width_tenths_mm": Baubreite in Zehntel-Millimetern (z. B. 5.2 mm -> 52, 8.2 mm -> 82).
- "height_tenths_mm": Bauhöhe / Länge quer zur Hutschiene in 0,1 mm (z. B. 70 mm -> 700).
- "rail_offset_tenths_mm": Vertikaler Versatz zur Hutschienenmitte in 0,1 mm (meist 0; bei asymmetrischen Installationsklemmen mit tiefem PE-Fuß z. B. 130).
- "numbering_order": "BottomToTop" (Standard) oder "TopToBottom".
- "description_de" / "description_en": Technische Kurzbeschreibung (Querschnitt, Anschlussart, Maße, Farbe).

2. "tiers" Array (Länge MUSS exakt gleich "levels" sein, geordnet von Unten nach Oben):
Jedes Element hat "behavior" und "color":
- "behavior":
  * "Sequential" -> fortlaufende Nummerierung (1, 2, 3...)
  * "ProtectiveEarth" -> Schutzleiter (PE)
  * {{"Fixed": "N"}} -> Fester Text (z. B. "N", "+", "-")
  * "Unlabeled" -> Leeres Schild (kein Text)
- "color": "StandardGray" | "BlueNeutral" | "GreenYellowPe" | "RedPhase" | "OrangeAux"

3. Brücken & Zubehör (optional, aber empfohlen):
- "jumper_slots_per_tier": 0, 1 oder 2 (Anzahl Brückerschächte pro Ebene).
- "max_jumper_poles": Maximale Polzahl einer Kammbrücke (z. B. 10, oder 0 wenn nicht brückbar).
- "article_number": Hersteller-Artikelnummer als String (z. B. "3210567").
- "product_url": Direkte Hersteller-Produktseite (https://...).
- "image_url": Direkte Produktbild-URL (oder "" falls unbekannt).
- "accessories": Objekt mit den Strings "end_plate", "jumpers", "markers", "end_bracket" und optional "extra": null oder "...".

4. Gültiges Referenz-JSON aus TermiGrid als Vorlage:
```json
[
{example_json}
]
```

Gib mir ausschließlich das fertige, syntaktisch gültige JSON (ohne Platzhalter wie "..." und ohne Kommentare) zurück, damit ich es direkt in TermiGrid unter „Klemmen-Datenbank → JSON importieren“ einfügen kann."#
        )
    } else {
        format!(
            r#"Create a valid JSON array `[ ... ]` (or single JSON object `{{ ... }}`) of terminal block specifications for "TermiGrid" for the following terminal(s):
[INSERT MANUFACTURER, PART NUMBER(S) / TYPE, OR DATASHEET TEXT HERE]

Follow this exact schema and rules:

1. Required fields & units:
- "id": Unique lowercase slug, e.g. "custom-phoenix-3210567".
- "name": "<ShortType> (<ShortDescription>)", e.g. "PTTB 2,5 (2-Level L/L)". Everything before " (" is shown as the compact type name in CAD & exports.
- "manufacturer": Exact string: "WAGO", "Phoenix Contact", or "Weidmüller".
- "series": Product family (e.g. "TOPJOB S 2002", "PT 2,5", "A-Series").
- "levels": Number of printable marker levels (1 to 4). NOTE: For multi-connection distribution blocks that carry only 1 shared marker per block (e.g. PPV/PRV), set "levels": 1.
- "width_tenths_mm": Pitch width in 0.1 mm (e.g. 5.2 mm -> 52).
- "height_tenths_mm": Front-view height across the DIN rail in 0.1 mm (e.g. 70 mm -> 700).
- "rail_offset_tenths_mm": Vertical offset from DIN-rail center in 0.1 mm (usually 0; e.g. 130 for asymmetric installation blocks).
- "numbering_order": "BottomToTop" or "TopToBottom".
- "description_de" / "description_en": Short technical summary.

2. "tiers" array (length MUST equal "levels", ordered bottom to top):
- "behavior": "Sequential" | "ProtectiveEarth" | {{"Fixed": "N"}} | "Unlabeled"
- "color": "StandardGray" | "BlueNeutral" | "GreenYellowPe" | "RedPhase" | "OrangeAux"

3. Jumpers & accessories:
- "jumper_slots_per_tier": 0, 1, or 2.
- "max_jumper_poles": Max comb jumper poles (e.g. 10, or 0 if none).
- "article_number": Manufacturer order/part number as string.
- "product_url" / "image_url": Direct manufacturer URLs (or "" for image_url if unknown).
- "accessories": {{"end_plate": "...", "jumpers": "...", "markers": "...", "end_bracket": "...", "extra": null}}.

4. Valid reference JSON from TermiGrid:
```json
[
{example_json}
]
```

Return ONLY valid JSON (no comments, no ellipsis) so I can paste it directly into TermiGrid under "Terminal Database → Import JSON"."#
        )
    }
}

static GUIDE_ARTICLES: &[GuideArticle] = &[
    GuideArticle {
        id: "quickstart-pipeline",
        category: GuideCategory::QuickStart,
        title_de: "1. Projektstart, Oberfläche & geführter Ablauf",
        title_en: "1. Getting Started, Workspace & Guided Workflow",
        summary_de: "Wie Sie ein neues Projekt anlegen, bestehende .termi-Dateien oder EPLAN-PDFs öffnen und die Oberfläche anpassen.",
        summary_en: "How to start a new project, open .termi files or EPLAN PDFs, and customize the workspace.",
        keywords: &[
            "Start", "Neu", "Öffnen", "Speichern", "termi", "json", "Theme", "Hell", "Dunkel",
            "System", "Sprache", "Seitenleiste", "Tour", "Onboarding", "Welcome", "Willkommen",
        ],
        steps_de: &[
            (
                "Startbildschirm (4 Möglichkeiten)",
                "• Neues Projekt erstellen: Öffnet den Dialog „Neue Klemmleiste“ für die erste Leiste.\n• Bestehendes Projekt laden (.termi / .json): Lädt gespeicherte TermiGrid-Projekte inkl. aller Potentiale und eigenen Klemmen.\n• EPLAN PDF-Stückliste: Analysiert EPLAN-Artikelstücklisten automatisch (auch per Drag & Drop ins Fenster!).\n• Beispielprojekt (-X1): Legt eine sofort editierbare Demo-Klemmleiste an.",
            ),
            (
                "Ansicht, Sprache & Design",
                "• Oben rechts wechseln Sie zwischen 7 Sprachen (DE, EN, FR, IT, ES, PL, NL) und dem Farbschema (System, Hell, Dunkel).\n• Mit den beiden Sidebar-Icons blenden Sie die linke Leisten-Übersicht und den rechten Eigenschaften-Inspektor ein oder aus.\n• Über das Hamburger-Menü (oben links) starten Sie jederzeit die Basis-Tour oder Profi-Tour.",
            ),
        ],
        steps_en: &[
            (
                "Welcome Screen (4 Start Options)",
                "• Create New Project: Opens the New Strip wizard immediately.\n• Open Existing Project (.termi / .json): Loads saved projects including potentials and custom specs.\n• Import EPLAN PDF BOM: Parses EPLAN parts lists automatically (also via drag & drop).\n• Sample Project (-X1): Opens an editable demo strip.",
            ),
            (
                "Layout, Language & Theme",
                "• Top-right controls switch between 7 languages and System / Light / Dark themes.\n• Sidebar toggle buttons show or hide the left strip list and right inspector.\n• The top-left Menu launches interactive Basic and Pro guided tours anytime.",
            ),
        ],
        code: None,
    },
    GuideArticle {
        id: "strips-and-numbering",
        category: GuideCategory::CadEditor,
        title_de: "2. Klemmleisten anlegen, Nummernkreise & N-/PE-Modi",
        title_en: "2. Creating Strips, Numbering Sequences & N/PE Modes",
        summary_de: "Erzeugung nach Nummernkreis oder Klemmenanzahl, ebenenweise vs. klemmenweise Nummerierung sowie N- und PE-Zählweisen.",
        summary_en: "Generating strips by number range or block count, per-block vs. per-level sequencing, and N/PE labeling modes.",
        keywords: &[
            "Klemmleiste", "Neue Leiste", "Startnummer", "Endnummer", "Nummernkreis", "Klemmenweise",
            "Ebenenweise", "ByBlock", "ByLevel", "N-Modus", "PE-Modus", "1N", "1PE", "Neutral",
            "Schutzleiter", "BMK", "-X1",
        ],
        steps_de: &[
            (
                "Leiste anlegen (+ Neue Leiste)",
                "• Nach Nummernkreis (Start..Ende): z. B. 1 bis 50. TermiGrid berechnet bei Mehrstockklemmen automatisch die benötigte Anzahl physischer Blöcke.\n• Nach Klemmenanzahl: Legt exakt N Klemmenblöcke an.",
            ),
            (
                "Sequenzmodus (Klemmenweise vs. Ebenenweise)",
                "• Klemmenweise (Standard): Klemme 1 erhält unten 1 und oben 2; Klemme 2 erhält 3 und 4.\n• Ebenenweise (Streifendruck): Erst wird Ebene 1 komplett durchnummeriert (1, 2, 3…), danach Ebene 2.",
            ),
            (
                "N-Modus & PE-Modus",
                "• Fest 'N' / Fest 'PE': Klassische Beschriftung ohne Zählerverbrauch.\n• Mitgezählt (1, 2, ...) oder (1N / 1PE): N- bzw. PE-Klemmen zählen in der Hauptnummerierung mit.\n• Eigener Zähler (1N, 2N... / 1PE, 2PE...): Separater Zähler nur für N bzw. PE.",
            ),
        ],
        steps_en: &[
            (
                "Creating a Strip (+ New Strip)",
                "• By Number Range (Start..End): Automatically calculates the exact number of physical blocks required for multi-level terminals.\n• By Block Count: Creates an exact number of physical terminal blocks.",
            ),
            (
                "Sequence Mode (Per Block vs. Per Level)",
                "• Per Block (default): Block 1 gets 1 & 2, Block 2 gets 3 & 4.\n• Per Level: Numbers all Tier 1 labels continuously across the strip, then continues on Tier 2.",
            ),
            (
                "N & PE Numbering Modes",
                "Choose between fixed 'N'/'PE', sharing the main counter (1, 2... or 1N/1PE), or a dedicated N/PE counter.",
            ),
        ],
        code: None,
    },
    GuideArticle {
        id: "cad-editing-blank-markers",
        category: GuideCategory::CadEditor,
        title_de: "3. CAD-Editor: Auswahl, Verschieben, Doppelklick & Leeres Schild",
        title_en: "3. CAD Editor: Selection, Moving, Inline Edit & Blank Markers",
        summary_de: "Wie Sie Klemmen markieren, umhängen, duplizieren, manuell beschriften oder gezielt leere Beschriftungsschilder erzeugen.",
        summary_en: "How to select, reorder, duplicate, manually override labels, and create intentionally blank marker cells.",
        keywords: &[
            "CAD", "Auswahl", "Rahmenauswahl", "Shift", "Strg", "Doppelklick", "Leeres Schild",
            "leer", "Reserve", "Blank", "Auto", "Präfix", "Suffix", "Sonderzeichen", "Verschieben",
            "Duplizieren", "Typ-Anzeige", "Grundtypen", "gruppieren", "Zoom", "Pan",
        ],
        steps_de: &[
            (
                "Auswahl & Navigation im CAD",
                "• Klick: Einzelne Klemme (oder Endhalter EH) auswählen.\n• Shift+Klick / Strg+Klick / Ziehen: Bereichs-, Mehrfach- oder Rahmenauswahl.\n• Strg+Mausrad (oder Pinch): Zoomen. Mittlere Maustaste (Mausrad drücken) + Ziehen: Ansicht verschieben.",
            ),
            (
                "Direktbearbeitung & Leeres Schild",
                "• Doppelklick auf eine Ebene öffnet das Textfeld direkt auf der Klemme (Enter = Übernehmen, Esc = Abbrechen).\n• Leeres Schild: Lassen Sie das Textfeld einfach leer (oder geben Sie ein Leerzeichen ein) und bestätigen mit Enter — die Klemme erhält ein explizit leeres Schild (auch im Excel-/CSV-Export bleibt diese Zelle als leeres Schild erhalten!).\n• Mit „Auto“ im rechten Inspektor kehrt die Ebene zur automatischen Nummerierung zurück.",
            ),
            (
                "Typen ändern, Verschieben & Präfix/Suffix",
                "• Im rechten Inspektor weisen Sie der Auswahl einen anderen Klemmentyp zu, fügen Klemmen davor/dahinter ein oder wenden Präfixe/Suffixe (z. B. „+“ oder „A“) auf alle oder eine bestimmte Ebene an.\n• Mit Alt+Pfeil Links/Rechts verschieben Sie markierte Klemmen; mit Strg+D duplizieren Sie sie.\n• „Grundtypen gruppieren“ sortiert z. B. L, N und PE innerhalb gleicher Bauform sauber zusammen.",
            ),
        ],
        steps_en: &[
            (
                "Selection & Navigation",
                "• Click to select a block or end bracket (EH). Shift+Click, Ctrl+Click, or drag for multi-selection.\n• Ctrl+Wheel zooms; Middle-drag pans the canvas.",
            ),
            (
                "Inline Editing & Blank Markers",
                "• Double-click any label on the CAD canvas to edit inline (Enter = apply, Esc = cancel).\n• Blank Marker: Clear the text (empty) and press Enter to emit an intentional blank marker cell in Excel/CSV exports.\n• Click 'Auto' in the right inspector to restore dynamic numbering.",
            ),
            (
                "Reassigning, Reordering & Batch Affixes",
                "• Reassign types, insert before/after, or apply prefixes/suffixes in the right inspector.\n• Alt+Left/Right moves selected blocks; Ctrl+D duplicates them.",
            ),
        ],
        code: None,
    },
    GuideArticle {
        id: "multilevel-pe-nt",
        category: GuideCategory::CadEditor,
        title_de: "4. Mehrstock-, PE- & Installationsklemmen (2002-32xx, PTI, NT)",
        title_en: "4. Multi-Level, PE & Installation Terminals (2002-32xx, PTI, NT)",
        summary_de: "Warum 2002-3227 PE in Ebene 1 hat, wie Installationsklemmen auf die N-Sammelschiene ausgerichtet werden und wie der NT-Assistent arbeitet.",
        summary_en: "How 2002-3227 places PE on E1, how installation blocks align with N busbars, and how the NT assistant works.",
        keywords: &[
            "2002-3227", "2002-3217", "2003", "2203", "PTI", "AITB", "PE", "E1", "E2", "E3",
            "Installationsklemme", "NT", "N-Trennklemme", "Sammelschiene", "2016-7714", "2009-305",
            "2009-304", "200 mm", "PPV", "PRV", "Etagen",
        ],
        steps_de: &[
            (
                "Standard-Dreistockklemmen (z. B. WAGO 2002-3227, PT 2,5-PE/L/L)",
                "Bei klassischen 3-Etagen-Klemmen sitzt die unterste Ebene (auch wenn sie PE ist, wie bei 2002-3227) exakt auf E1, die mittlere auf E2 und die obere auf E3. PE verbraucht dabei keine fortlaufende Nummer.",
            ),
            (
                "Asymmetrische Installationsklemmen & N-Sammelschiene",
                "Installationsklemmen (z. B. 2003-7641, PTI 2,5-PE/L/NT, AITB) besitzen einen tiefen PE-Fuß unterhalb der Hauptebene und eine asymmetrische Bauhöhe (echter Rail-Offset im CAD & DXF).\n• Bei NT-Klemmen bietet TermiGrid automatisch die passende N-Einspeiseklemme (z. B. 2016-7714) sowie Sammelschienenträger (2009-305 außen + 2009-304 alle 200 mm) an.\n• Mit dem Button „N → NT“ / „NT → N“ wandeln Sie Blöcke direkt um.",
            ),
            (
                "Potential- & Rangierverteiler (z. B. Weidmüller PPV / PRV)",
                "Verteilerklemmen mit 4, 8 oder 16 Anschlusspunkten, die nur ein gemeinsames Beschriftungsschild tragen, sind mit 1 Beschriftungsebene hinterlegt — so wird genau ein Schild pro Klemme erzeugt.",
            ),
        ],
        steps_en: &[
            (
                "Standard 3-Level Terminals (e.g. WAGO 2002-3227)",
                "Standard 3-deck blocks map their physical decks directly to E1, E2, E3 (E1: PE, E2: L, E3: L on 2002-3227) while skipping PE in sequence numbering.",
            ),
            (
                "Installation Terminals & N-Busbar",
                "Asymmetric installation terminals (2003-*, PTI, AITB) render with their real rail offset and automatically offer matching N-supply feeders and busbar carriers every 200 mm.",
            ),
            (
                "Potential Distributors (PPV / PRV)",
                "Multi-port potential distributors carrying a single shared marker per block are modeled with 1 printable label level.",
            ),
        ],
        code: None,
    },
    GuideArticle {
        id: "quick-prompt-assistant",
        category: GuideCategory::PromptAndLibrary,
        title_de: "5. Intelligenter Schnell-Assistent (Quick-Prompt ohne KI)",
        title_en: "5. Quick-Prompt Terminal Assistant (Local, No LLM)",
        summary_de: "Ganze Abgänge, Drehstromkreise, Einspeisungen und gebrückte Blöcke per Klartext-Eingabe blitzschnell erzeugen.",
        summary_en: "Generate complete feeders, 3-phase circuits, power infeeds, and bridged blocks from plain text locally.",
        keywords: &[
            "Quick-Prompt", "Schnell-Assistent", "Textfeld", "Drehstrom", "Abgang", "1-pol",
            "3-pol", "4-pol", "PTI", "Motor", "Sicherung", "Reserve", "gebrückt", "+24V",
            "URTK", "Einspeisung", "Vorschau",
        ],
        steps_de: &[
            (
                "So öffnen und nutzen Sie den Schnell-Assistenten",
                "1. Klicken Sie in der CAD-Toolbar auf „+ Schnell-Assistent“.\n2. Wählen Sie oben per Klick Ihren Standard-Hersteller (Phoenix, WAGO, Weidmüller) und die Standard-Bauform (Install, Durchgang, Doppelstock).\n3. Tippen Sie Ihren Wunsch ein (oder klicken Sie auf eines der Beispiel-Chips). Die Live-Vorschau zeigt sofort alle erkannten Klemmen inkl. Brücken und NT-Trägern.\n4. „Anhängen“ fügt ans Leistenende an, „Ersetzen“ tauscht die Leiste aus.",
            ),
            (
                "Unterstützte Befehle & Beispiele",
                "• Drehstrom-Abgänge: „4x Drehstrom Abgang 2,5mm² Installationsklemmen“ (erzeugt bei PTI automatisch je 1× L/L + 1× PE/L/N!).\n• Polzahl-Angaben: „4x 3-pol PTI 2,5“, „6x 1-pol PTI 2,5“, „2x 4-pol 4mm²“.\n• Einspeisung & Sonderklemmen: „1x 16mm² Einspeisung“, „2x Motor 4mm²“, „4x URTK/SP“, „2x Sicherung“.\n• Brücken & Reserve: „6x +24V QUATTRO gebrückt“, „2x Reserve leer“.",
            ),
        ],
        steps_en: &[
            (
                "Using the Quick-Prompt Bar",
                "Open '+ Quick-Prompt' in the CAD toolbar, pick default brand & style chips, type your circuit description, inspect the live preview, and click Append or Replace.",
            ),
            (
                "Supported Clauses & Examples",
                "• 3-phase feeders: '4x Drehstrom Abgang 2,5mm² Installationsklemmen'\n• Pole counts: '4x 3-pol PTI 2,5', '6x 1-pol PTI 2,5'\n• Bridged & special: '6x +24V QUATTRO gebrückt', '2x Motor 4mm²', '4x URTK/SP', '2x Reserve leer'.",
            ),
        ],
        code: Some(GuideCodeKind::Static {
            title_de: "Beispiel-Eingabe für eine komplette Verteilung",
            title_en: "Example input for a complete distribution strip",
            body_de: "1x 16mm² Einspeisung + 4x Drehstrom Abgang 2,5mm² Installationsklemmen + 6x 1-pol PTI 2,5 + 6x +24V QUATTRO gebrückt + 2x Reserve leer",
            body_en: "1x 16mm² Einspeisung + 4x Drehstrom Abgang 2,5mm² Installationsklemmen + 6x 1-pol PTI 2,5 + 6x +24V QUATTRO gebrückt + 2x Reserve leer",
        }),
    },
    GuideArticle {
        id: "strip-library",
        category: GuideCategory::PromptAndLibrary,
        title_de: "6. Persönliche Klemmleisten-Bibliothek",
        title_en: "6. Personal Terminal Strip Library",
        summary_de: "Fertige Klemmleisten (inkl. Potentiale, Endhalter & eigenen Klemmen) dauerhaft auf dem PC speichern und projektübergreifend einfügen.",
        summary_en: "Save complete terminal strip templates permanently on your PC and reuse them across any project.",
        keywords: &[
            "Bibliothek", "Vorlage", "strip_library.json", "Speichern", "Einfügen", "Umbenennen",
            "Template", "Projektübergreifend",
        ],
        steps_de: &[
            (
                "Vorlage speichern & verwalten",
                "• Über den Reiter „Bibliothek“ in der linken Seitenleiste (oder den Bibliotheks-Dialog) speichern Sie die aktive Leiste als benannte Vorlage.\n• Gespeichert werden alle Klemmen, Nummerierungsmodi, Potentiale, Endhalter sowie eventuell verwendete eigene Klemmen-Definitionen in `strip_library.json`.\n• Beim Einfügen in ein Projekt werden Namenskollisionen (z. B. `-X1` → `-X2`) automatisch vermieden.",
            ),
        ],
        steps_en: &[
            (
                "Saving & Reusing Strip Templates",
                "Use the Library tab in the left sidebar to save any strip (including potentials, end brackets, and custom specs) into `strip_library.json` and insert it into any project with automatic name collision avoidance.",
            ),
        ],
        code: None,
    },
    GuideArticle {
        id: "bridging-mode",
        category: GuideCategory::BridgingAndAccessories,
        title_de: "7. Brücken-Modus (Kammbrücken, Pin-Kürzung & Drahtbrücken)",
        title_en: "7. Bridge Mode (Comb Jumpers, Cut Pins & Wire Bridges)",
        summary_de: "Potentiale anlegen, per Klick & Ziehen brücken und Kammbrücken inkl. 2-Kammer-Kaskade und herausgebrochenen Pins berechnen lassen.",
        summary_en: "Assign electrical potentials by clicking or dragging and auto-calculate comb jumpers, cut pins, and wire bridges.",
        keywords: &[
            "Brücken", "Kammbrücke", "Drahtbrücke", "Potential", "+24V", "GND", "Kaskade",
            "Pin entfernen", "Brückerschacht", "Doppelbrückenschacht", "Ziehen", "Bridge",
        ],
        steps_de: &[
            (
                "Potentiale direkt im CAD zuweisen",
                "1. Wechseln Sie oben auf „Brücken-Modus“.\n2. Über dem CAD erscheint die Potential-Schnellleiste (z. B. +24V, GND). Legen Sie bei Bedarf neue Potentiale an.\n3. Klicken Sie einzelne Ebenen an oder halten Sie die linke Maustaste gedrückt und ziehen Sie über mehrere Klemmen hinweg, um sie in einem Zug zu brücken.",
            ),
            (
                "Automatische Kammbrücken-Logik",
                "• TermiGrid berücksichtigt Rastermaß, Ebenenlage, max. Polzahl und Endplatten-Barrieren.\n• Doppelbrückenschächte (2 Kammern) kaskadieren automatisch über lange Strecken.\n• Übersprungene Klemmen werden erkannt: Im CAD erscheint ein rotes × und in der Excel-Stückliste die exakte Pin-Kürzungsanweisung („PIN ENTFERNEN BEI: #...“).\n• Bei Ebenenwechsel oder Endplatten schlägt TermiGrid automatisch eine Drahtbrücke vor.",
            ),
        ],
        steps_en: &[
            (
                "Assigning Potentials in CAD",
                "Switch to Bridge Mode, pick or add a potential in the inline bar above the canvas, and click or left-drag across terminal tiers.",
            ),
            (
                "Automatic Jumper Calculation",
                "Respects pitch, tier level, max poles, and end-plate barriers; supports 2-slot cascading, skipped-pin cut markers (red ×), and wire-bridge fallbacks.",
            ),
        ],
        code: None,
    },
    GuideArticle {
        id: "endplates-and-brackets",
        category: GuideCategory::BridgingAndAccessories,
        title_de: "8. Endplatten, Zwischenplatten & TS-35-Endhalter",
        title_en: "8. End Plates, Partition Plates & TS 35 End Brackets",
        summary_de: "Automatische Erkennung von Profilwechseln, geschlossenen Gehäusen und herstellerspezifischen Endhaltern.",
        summary_en: "Automatic contour-transition partition plates, closed-housing detection, and brand-matched TS 35 end brackets.",
        keywords: &[
            "Endplatte", "Abschlussplatte", "Zwischenplatte", "Endhalter", "Endklammer", "EH",
            "CLIPFIX", "WEW", "249-117", "TS 35", "Bearbeitungsmodus", "Endplatten generieren",
            "Geschlossenes Gehäuse",
        ],
        steps_de: &[
            (
                "Automatische Endplatten-Generierung",
                "Sobald Sie Klemmen im CAD bearbeiten, wechselt die Leiste automatisch in den Bearbeitungsmodus. Beim Klick auf „Endplatten generieren“, beim Wechsel in „e-Planer (1:1)“ oder bei jedem Export werden alle Abschluss- und Zwischenplatten automatisch berechnet und eingeblendet.",
            ),
            (
                "Reale Baubreiten & Endhalter (EH)",
                "• Zwischenplatten werden nur bei echten Profil-/Herstellerwechseln gesetzt; geschlossene Gehäuse (z. B. USLKG, -HESI, PTPOWER, 285-*) erhalten 0 mm.\n• Endhalter links/rechts lassen sich im Inspektor oder per Klick direkt auf „EH“ im CAD konfigurieren (26 verifizierte Modelle von Phoenix Contact, Weidmüller & WAGO).",
            ),
        ],
        steps_en: &[
            (
                "Automatic End-Plate Workflow",
                "Editing automatically enters edit mode; switching to e-Planer, exporting, or clicking Generate End Plates updates all contour transitions and strip-end covers automatically.",
            ),
            (
                "Real Thickness & End Brackets",
                "Closed-housing terminals report 0 mm end plate width. Click any 'EH' bracket in the CAD canvas or use the inspector to configure left/right TS 35 end brackets.",
            ),
        ],
        code: None,
    },
    GuideArticle {
        id: "local-terminals-manual-and-json",
        category: GuideCategory::LocalTerminalsJson,
        title_de: "9. Eigene Klemmen lokal einbinden (Editor & JSON-Import)",
        title_en: "9. Adding Local Terminals (Visual Editor & JSON Import)",
        summary_de: "Schritt-für-Schritt-Anleitung, wie Sie neue oder angepasste Klemmen dauerhaft lokal auf Ihrem PC einbinden und zwischen Projekten teilen.",
        summary_en: "Step-by-step guide to adding custom terminals permanently on your PC via the visual editor or JSON import.",
        keywords: &[
            "lokal", "eigene Klemme", "einbinden", "hinzufügen", "Klemmen-Datenbank", "custom_specs.json",
            "JSON", "Import", "Bearbeiten", "Stift", "Werkszustand", "Schema", "width_tenths_mm",
            "height_tenths_mm", "AppData",
        ],
        steps_de: &[
            (
                "Weg 1: Grafischer Klemmen-Editor (schnell & einfach)",
                "1. Öffnen Sie oben den Tab „Klemmen-Datenbank“.\n2. Klicken Sie auf „+ Eigene Klemme“ (oder bei einer vorhandenen Klemme auf das Stift-Icon, um eine Variante abzuleiten).\n3. Vergeben Sie Hersteller, Typname, Artikelnummer, Ebenen (1–4), Belegung (Sequentiell / PE / Fixtext), Farbe, Breite (mm), Bauhöhe (mm) und Brückerkammern.\n4. Nach „Speichern“ steht die Klemme sofort in allen Projekten zur Verfügung.",
            ),
            (
                "Weg 2: JSON-Datei oder JSON-Text direkt importieren",
                "1. Öffnen Sie „Klemmen-Datenbank“ → Bereich „JSON-Import / Export“.\n2. Fügen Sie ein einzelnes JSON-Objekt `{ ... }` oder ein komplettes Array `[{ ... }, { ... }]` ein (Markdown-Codeblöcke ```json und UTF-8-BOM werden automatisch bereinigt).\n3. Klicken Sie auf „JSON importieren“ — TermiGrid prüft das Schema, meldet eventuelle Feld-Fehler präzise und übernimmt alle Klemmen sofort ins laufende Programm.",
            ),
            (
                "Wo speichert TermiGrid lokale Klemmen?",
                "• PC-weit dauerhaft: In `custom_specs.json` im lokalen AppData-/Konfigurationsordner (Windows: `%APPDATA%\\TermiGrid\\custom_specs.json`, macOS: `~/Library/Application Support/TermiGrid/custom_specs.json`, Linux: `~/.local/share/TermiGrid/custom_specs.json`).\n• Projektbezogen portabel: Zusätzlich werden verwendete eigene Klemmen direkt in jede `.termi`-Projektdatei und in Bibliotheksvorlagen eingebettet — Kollegen können Ihr Projekt ohne Zusatzdateien öffnen!",
            ),
        ],
        steps_en: &[
            (
                "Method 1: Visual Terminal Editor",
                "Open the 'Terminal Database' tab, click '+ Add Custom Terminal' (or the Edit icon on any existing block), configure geometry, tiers, colors, and jumper slots, and click Save.",
            ),
            (
                "Method 2: JSON Import (Single Object or Array)",
                "Open 'Terminal Database' → JSON Import, paste `{ ... }` or `[{ ... }]` (code fences & BOM are stripped automatically), and click Import JSON.",
            ),
            (
                "Persistence Locations",
                "PC-wide custom terminals are saved in `custom_specs.json` inside your OS AppData folder and are also embedded inside `.termi` project files for portability.",
            ),
        ],
        code: Some(GuideCodeKind::ExampleTerminalJson),
    },
    GuideArticle {
        id: "local-terminals-ai-prompt",
        category: GuideCategory::LocalTerminalsJson,
        title_de: "10. Beispiel-KI-Prompt: Passendes Klemmen-JSON automatisch erstellen lassen",
        title_en: "10. Example AI Prompt: Auto-Generate Valid Terminal JSON",
        summary_de: "Fertige Prompt-Vorlage zum Kopieren für ChatGPT, Claude oder Gemini — erzeugt aus jedem Datenblatt sofort ein fehlerfreies TermiGrid-JSON.",
        summary_en: "Copy-ready LLM prompt template for ChatGPT, Claude, or Gemini to turn any datasheet into valid TermiGrid JSON.",
        keywords: &[
            "Prompt", "KI", "LLM", "ChatGPT", "Claude", "Gemini", "JSON", "Vorlage", "Beispiel",
            "Datenblatt", "Schema", "Kopie", "Kopieren", "custom", "erstellen",
        ],
        steps_de: &[
            (
                "So funktioniert es in 3 Schritten",
                "1. Klicken Sie unten auf „Prompt kopieren“.\n2. Fügen Sie den Prompt in ChatGPT, Claude oder Gemini ein und ersetzen Sie den Platzhalter `[HIER HERSTELLER...]` durch Artikelnummer, Typbezeichnung oder kopierten Datenblatt-Text (gerne auch mehrere Klemmen auf einmal!).\n3. Kopieren Sie das zurückgegebene JSON und fügen Sie es in TermiGrid unter „Klemmen-Datenbank → JSON importieren“ ein.",
            ),
            (
                "Wichtige Hinweise für perfekte Ergebnisse",
                "• Einheiten sind immer Zehntel-Millimeter (`5,2 mm` = `52`, `70 mm` = `700`).\n• Die Liste `tiers` muss exakt so viele Einträge haben wie `levels` (Reihenfolge: Unten → Oben).\n• Bei Verteilern mit mehreren Anschlüssen aber nur 1 Schild pro Klemme (z. B. PPV/PRV) immer `\"levels\": 1` angeben.",
            ),
        ],
        steps_en: &[
            (
                "3-Step Workflow",
                "1. Click 'Copy Prompt' below.\n2. Paste into ChatGPT / Claude / Gemini and insert the part number(s) or datasheet text.\n3. Paste the generated JSON into TermiGrid under Terminal Database → Import JSON.",
            ),
            (
                "Key Schema Rules",
                "Dimensions use 0.1 mm integers (`width_tenths_mm`, `height_tenths_mm`), `tiers.len()` must match `levels` (bottom-to-top), and single-label distribution blocks use `levels: 1`.",
            ),
        ],
        code: Some(GuideCodeKind::AiTerminalPrompt),
    },
    GuideArticle {
        id: "export-excel-csv-dxf",
        category: GuideCategory::ExportAndEplan,
        title_de: "11. Excel (.xlsx), CSV & e-Planer 1:1 DXF-Export",
        title_en: "11. Excel (.xlsx), CSV & e-Planer 1:1 DXF Export",
        summary_de: "Druckdaten für WAGO/Phoenix/Weidmüller-Beschriftungsdrucker, 4 Excel-Tabellenblätter und maßstabsgetreues 1:1-Aufbaubild (DXF).",
        summary_en: "Printer-ready Excel & CSV exports (4 worksheets, strict text format) and 1:1 millimeter DXF elevation drawings.",
        keywords: &[
            "Excel", "xlsx", "CSV", "DXF", "e-Planer", "1:1", "Druckdaten", "Stückliste",
            "Druckstreifen", "Matrix", "Textformat", "Smart Designer", "Project Complete",
            "M-Print", "AutoCAD", "Hutschiene",
        ],
        steps_de: &[
            (
                "Excel-Arbeitsmappe (.xlsx) — 4 Blätter",
                "• Druckdaten (Flach): Reine Textzellen (`@`) für Beschriftungsdrucker (WAGO Smart Designer, Phoenix Project Complete, Weidmüller M-Print PRO).\n• Etagen-Druckstreifen: Durchgehende Streifen je Ebene (E1..E4) pro Leiste inkl. benötigtem Schildermaterial.\n• Bestückungsplan (Matrix): Übersichtliche Tabelle aller Klemmen mit Etagen-Spalten.\n• Stückliste & Montage: Aggregierte Klemmen, End-/Zwischenplatten, Kammbrücken inkl. Pin-Kürzung, Endhalter und exakte Hutschienenlänge in mm.",
            ),
            (
                "e-Planer (1:1 mm) & DXF-Export",
                "Im Modus „e-Planer (1:1)“ sehen Sie das maßstabsgetreue Aufbaubild auf TS 35 mit Bauhöhen und Bemaßung. Über „DXF“ exportieren Sie wahlweise die aktive Leiste oder alle Leisten einzeln in einen Ordner (`AC1009` AutoCAD R12, reine LINE & TEXT Entities).",
            ),
        ],
        steps_en: &[
            (
                "Excel Workbook (.xlsx) — 4 Sheets",
                "Exports Flat Print Data (`@` strict text), Per-Tier Print Strips, Tier Matrix, and full BOM & Mounting Plan (including rail length, cut-pin instructions, end plates, and end brackets).",
            ),
            (
                "e-Planer (1:1 mm) & DXF",
                "True-to-scale TS 35 front elevation with real heights and offsets; exports single or per-strip AutoCAD R12 (`AC1009`) DXF files.",
            ),
        ],
        code: None,
    },
    GuideArticle {
        id: "eplan-p8-roundtrip",
        category: GuideCategory::ExportAndEplan,
        title_de: "12. EPLAN Electric P8 Roundtrip-Schnittstelle",
        title_en: "12. EPLAN Electric P8 Roundtrip Integration",
        summary_de: "Direkter Datenaustausch mit EPLAN P8: CLI-Aufruf, CSV-Import mit Präfix-Erkennung, Zubehör-Rückimport (CSV/XML) & DXF-Paket.",
        summary_en: "Seamless EPLAN P8 roundtrip: CLI CSV launch, vendor prefix stripping/re-attaching, accessory CSV/XML + DXF package, and C# bridge.",
        keywords: &[
            "EPLAN", "P8", "Roundtrip", "CLI", "--import-eplan-csv", "PXC.", "WAG.", "WDM.",
            "Rückimport", "_EPLAN_Zubehoer_Import", "xml", "csv", "C#", "Bridge", "config.json",
            "EPLAN-Schnittstelle",
        ],
        steps_de: &[
            (
                "Wo finde ich die EPLAN-Einstellungen?",
                "Um die normale Werkstatt-Oberfläche schlank zu halten, liegen alle EPLAN-Funktionen dezent unter `Menü (oben links) → EPLAN-Schnittstelle...` (oder `Strg+K` → „EPLAN“).",
            ),
            (
                "Import aus EPLAN (manuell oder per CLI)",
                "• Unterstützt Semikolon-getrennte EPLAN-CSVs (`BMK;Klemmennummer;Etage;Artikelnummer;Typnummer;Menge`, UTF-8 mit/ohne BOM).\n• Konfigurierbare Hersteller-Präfixe (`PXC.`, `WAG.`, `WDM.`) werden beim Abgleich entfernt und die exakte Klemmenreihenfolge & Nummerierung aus EPLAN übernommen.\n• Über „EPLAN C#-Skript exportieren“ erhalten Sie `TermiGrid_EPLAN_Bridge.cs` mit automatisch vorausgefülltem `.exe`-Pfad.",
            ),
            (
                "1-Klick-Rückimport-Paket (TermiGrid → EPLAN)",
                "„EPLAN Rückimport-Paket erstellen“ berechnet fehlende End-/Zwischenplatten und strikt markenreine Endhalter, hängt die konfigurierten EPLAN-Präfixe wieder an und schreibt:\n• `_EPLAN_Zubehoer_Import.csv` & `.xml` (optional nur das neu ergänzte Zubehör)\n• `/DXF/<Sanitized_BMK>.dxf` (z. B. `NSV_A01_X1.dxf`) je Klemmleiste.",
            ),
        ],
        steps_en: &[
            (
                "Location (Unobtrusive UI)",
                "All EPLAN settings and roundtrip actions live inside `Menu (top-left) → EPLAN Interface...` (or `Ctrl+K`).",
            ),
            (
                "Importing EPLAN CSV & CLI",
                "Supports `--import-eplan-csv \"path/to/export.csv\"` or manual import, stripping `PXC.` / `WAG.` / `WDM.` prefixes while preserving exact terminal order and numbering.",
            ),
            (
                "1-Click Return Package",
                "Generates `_EPLAN_Zubehoer_Import.csv`, `.xml`, and per-strip `/DXF/<BMK>.dxf` files with vendor prefixes re-attached and strict brand matching.",
            ),
        ],
        code: Some(GuideCodeKind::Static {
            title_de: "CLI-Aufruf (z. B. aus EPLAN C#-Skript)",
            title_en: "CLI invocation (e.g. from EPLAN C# script)",
            body_de: "TermiGrid.exe --import-eplan-csv \"C:\\EPLAN\\Export\\Klemmleisten.csv\"",
            body_en: "TermiGrid.exe --import-eplan-csv \"C:\\EPLAN\\Export\\Terminals.csv\"",
        }),
    },
    GuideArticle {
        id: "shortcuts-and-updates",
        category: GuideCategory::ShortcutsAndUpdates,
        title_de: "13. Tastenkürzel (Shortcuts), Live-Datenbank- & App-Updates",
        title_en: "13. Keyboard Shortcuts, Live Database & App Updates",
        summary_de: "Alle Tastenkürzel für schnelles Arbeiten sowie automatische Live-Updates der Klemmen-Datenbank ohne Programm-Neustart.",
        summary_en: "All keyboard shortcuts for fast editing plus background GitHub terminal database hot-reload and app updates.",
        keywords: &[
            "Shortcut", "Tastenkürzel", "Strg+K", "Strg+Z", "Strg+Y", "Strg+D", "Strg+A", "Alt",
            "Entf", "Update", "Datenbank-Update", "GitHub", "Hot-Reload", "Offline", "Cache",
        ],
        steps_de: &[
            (
                "Wichtige Tastenkürzel im CAD-Editor",
                "• Strg+K: Befehlspalette & Klemmen-Schnellsuche öffnen\n• Strg+Z / Strg+Y: Rückgängig / Wiederholen (Undo/Redo)\n• Strg+D: Ausgewählte Klemmen duplizieren\n• Strg+A: Alle Klemmen der aktiven Leiste auswählen\n• Alt+Pfeil Links / Rechts: Markierte Klemmen verschieben\n• Entf / Backspace: Auswahl löschen (mit Rückgängig-Toast)\n• Doppelklick auf Zahl: Direkt bearbeiten (Enter = Übernehmen, Leer = Leeres Schild, Esc = Abbrechen)\n• Strg+Mausrad: Zoom | Mausrad drücken + Ziehen: Verschieben",
            ),
            (
                "Live-Datenbank-Updates & App-Updates",
                "• TermiGrid prüft beim Start asynchron (offline-sicher!) auf neue Klemmen in der GitHub-Datenbank (`terminal_specs.json`).\n• Bei neuen Klemmen erscheint oben der grüne Button „Datenbank-Update verfügbar“ — ein Klick lädt die aktuelle Datenbank herunter und aktiviert sie sofort im laufenden Betrieb (Hot-Reload ohne Neustart).\n• Produktbilder werden lokal im `photo_cache` zwischengespeichert und funktionieren danach auch offline.",
            ),
        ],
        steps_en: &[
            (
                "CAD Editor Shortcuts",
                "• Ctrl+K: Command Palette & Quick Terminal Search\n• Ctrl+Z / Ctrl+Y: Undo / Redo\n• Ctrl+D: Duplicate selected blocks\n• Ctrl+A: Select all blocks\n• Alt+Left / Right: Move selected blocks\n• Del: Delete selection\n• Ctrl+Wheel: Zoom | Middle-drag: Pan",
            ),
            (
                "Live Database Hot-Reload & App Updates",
                "TermiGrid checks GitHub asynchronously at startup for updated `terminal_specs.json` catalogs and offers a 1-click hot-reload button without restarting.",
            ),
        ],
        code: None,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guide_search_finds_local_terminals_prompt_and_eplan_with_ranking() {
        let catalog = TerminalCatalog::load_embedded();

        // Empty query returns all 13 comprehensive articles
        let all = search_guide_articles(Language::De, GuideCategory::All, "");
        assert_eq!(all.len(), 13);

        // Searching for "KI Prompt JSON" ranks the AI prompt article first
        let prompt_hits =
            search_guide_articles(Language::De, GuideCategory::All, "Prompt JSON");
        assert!(!prompt_hits.is_empty());
        assert_eq!(prompt_hits[0].id, "local-terminals-ai-prompt");

        // Searching for "custom_specs.json" finds the local terminals guide
        let local_hits =
            search_guide_articles(Language::De, GuideCategory::All, "custom_specs.json");
        assert_eq!(local_hits.len(), 1);
        assert_eq!(local_hits[0].id, "local-terminals-manual-and-json");

        // Searching for "Leeres Schild" finds inline editing guide
        let blank_hits =
            search_guide_articles(Language::De, GuideCategory::All, "Leeres Schild");
        assert!(blank_hits.iter().any(|a| a.id == "cad-editing-blank-markers"));

        // Category filter restricts results cleanly
        let eplan_cat = search_guide_articles(
            Language::De,
            GuideCategory::ExportAndEplan,
            "PXC.",
        );
        assert_eq!(eplan_cat.len(), 1);
        assert_eq!(eplan_cat[0].id, "eplan-p8-roundtrip");

        // Generated AI prompt contains schema fields and real example JSON
        let prompt_de = build_ai_terminal_prompt(Language::De, &catalog);
        assert!(prompt_de.contains("width_tenths_mm"));
        assert!(prompt_de.contains("height_tenths_mm"));
        assert!(prompt_de.contains("custom-beispiel-2002-3217"));
        assert!(prompt_de.contains("ProtectiveEarth"));

        let prompt_en = build_ai_terminal_prompt(Language::En, &catalog);
        assert!(prompt_en.contains("Terminal Database"));
        assert!(prompt_en.contains("custom-beispiel-2002-3217"));
    }
}
