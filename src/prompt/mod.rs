use crate::catalog::{TerminalCatalog, TerminalSpec};

/// Supported manufacturer choices for the deterministic natural-language prompt engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptManufacturer {
    PhoenixContact,
    Wago,
    Weidmueller,
}

impl PromptManufacturer {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhoenixContact => "Phoenix Contact",
            Self::Wago => "WAGO",
            Self::Weidmueller => "Weidmüller",
        }
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Self::PhoenixContact => "Phoenix",
            Self::Wago => "WAGO",
            Self::Weidmueller => "Weidmüller",
        }
    }

    pub fn from_catalog_str(s: &str) -> Option<Self> {
        let low = s.to_lowercase();
        if low.contains("phoenix") || low == "pxc" {
            Some(Self::PhoenixContact)
        } else if low.contains("wago") || low == "wag" {
            Some(Self::Wago)
        } else if low.contains("weidm") || low == "wdm" {
            Some(Self::Weidmueller)
        } else {
            None
        }
    }
}

/// Physical terminal construction / deck style parsed from or inherited by a clause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptStyle {
    /// 3-level installation terminal block (`PTI`, `2003`/`2203`/`2005`, `AITB`).
    Installation,
    /// Standard 1-level feed-through terminal block (`PT`, `2002-12xx`, `A2C`, `WDU`, `UT`, `ST`).
    FeedThrough,
    /// 2-level double-deck terminal block (`PTTB`, `2002-22xx`, `A2T`, `WDK`).
    DoubleLevel,
    /// 3-level triple-deck terminal block (`PT 2,5-3L`/`PE/L/N`, `2002-32xx`, `A3T`).
    TripleLevel,
}

/// Electrical circuit or component intent parsed from a single prompt clause.
#[derive(Debug, Clone, PartialEq)]
pub enum PromptCircuitKind {
    /// 3-phase feeder with neutral and PE (`L1 + L2 + L3 + N + PE`, e.g. "Drehstrom", "3-polig", "400V").
    ThreePhaseNPe,
    /// 3-phase motor / load without neutral (`L1 + L2 + L3 + PE`, e.g. "Motor", "3L+PE", "Drehstrom ohne N").
    ThreePhasePe,
    /// 1-phase feeder with neutral and PE (`L + N + PE`, e.g. "Wechselstrom", "Licht", "Steckdose", "1-polig", "230V").
    SinglePhaseNPe,
    /// Power supply / incoming feeder (`Einspeisung`, `Zuleitung`, 3P+N+PE).
    SupplyFeeder,
    /// Fuse terminal block (`Sicherung`, `HESI`, `WSI`, `AFS`).
    Fuse,
    /// Disconnect terminal block (`Trennklemme`, `Messertrennklemme`, `-MT`, `-TG`).
    Disconnect,
    /// Current transformer test disconnect terminal (`Messwandler`, `Wandlerklemme`, `URTK`, `PTME`).
    TransformerTest,
    /// Pure PE ground terminal (`PE`, `Schutzleiterklemme`, `Erdungsklemme`).
    GroundPe,
    /// Pure Neutral terminal (`N-Klemme`, `Neutralleiter`, `blau`).
    NeutralN,
    /// Reserve / spare feed-through terminal (optionally with blank marker `Leeres Schild`).
    Reserve,
    /// Single feed-through terminal (`Durchgangsklemme`, `L-Klemme`, `24V`, `0V`).
    SingleFeedThrough,
    /// Direct match to a specific catalog specification ID (when user writes an exact type or article number like `URTK/SP`, `2002-3217`, `PTTB 2,5`).
    DirectSpec(String),
}

/// A single resolved block entry inside a `ResolvedPromptSegment`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPromptBlock {
    pub spec_id: String,
    pub spec_name: String,
    pub manufacturer: String,
    pub width_tenths_mm: u16,
    pub blank_marker: bool,
}

/// Human-readable preview & resolved block sequence for one clause of the prompt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPromptSegment {
    pub raw_clause: String,
    pub count: usize,
    pub intent_label_de: String,
    pub intent_label_en: String,
    pub blocks: Vec<ResolvedPromptBlock>,
    pub summary: String,
    pub width_tenths_mm: u32,
}

/// Complete live-preview result for a natural-language terminal prompt.
#[derive(Debug, Clone, Default)]
pub struct PromptPlan {
    pub segments: Vec<ResolvedPromptSegment>,
    pub unrecognized_clauses: Vec<String>,
    pub total_blocks: usize,
    pub total_width_tenths_mm: u32,
}

impl PromptPlan {
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn total_width_mm(&self) -> f32 {
        (self.total_width_tenths_mm as f32) / 10.0
    }

    /// Flattens all resolved blocks in sequence order: `(spec_id, blank_marker)`.
    pub fn flattened_blocks(&self) -> Vec<(String, bool)> {
        let mut out = Vec::with_capacity(self.total_blocks);
        for seg in &self.segments {
            for b in &seg.blocks {
                out.push((b.spec_id.clone(), b.blank_marker));
            }
        }
        out
    }

    /// Compact summary of all inserted terminal types for toast notifications.
    pub fn compact_summary(&self) -> String {
        let mut parts = Vec::new();
        for seg in &self.segments {
            if !seg.summary.is_empty() {
                parts.push(seg.summary.clone());
            }
        }
        parts.join(" + ")
    }
}

/// User-controlled context / default toggles in the Quick-Prompt UI bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromptContext {
    pub default_manufacturer: PromptManufacturer,
    /// When true, installation terminals default to N-disconnect (`NT`) variants (`PTI ...-PE/L/NT`, `2003-7641`, `AITB ... NTLP`).
    pub prefer_nt_for_installation: bool,
    /// Default style when a clause specifies a circuit (e.g. "4x Drehstrom 2,5mm²") without explicitly saying "Installationsklemme" or "Durchgang".
    pub default_style: PromptStyle,
}

impl Default for PromptContext {
    fn default() -> Self {
        Self {
            default_manufacturer: PromptManufacturer::PhoenixContact,
            prefer_nt_for_installation: true,
            default_style: PromptStyle::Installation,
        }
    }
}

/// Splits a free-text prompt into individual clauses around `+`, `;`, newlines, `,` (when not inside a decimal number like `2,5`), and ` und `.
pub fn split_prompt_clauses(input: &str) -> Vec<String> {
    let mut normalized = String::with_capacity(input.len() + 16);
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '+' || c == ';' || c == '\n' || c == '\r' {
            // Preserve "+24V" when '+' is followed immediately by digits and preceded by whitespace/start
            let prev_is_digit = i > 0 && chars[i - 1].is_ascii_digit();
            let next_is_24v = c == '+'
                && chars
                    .get(i + 1..i + 4)
                    .is_some_and(|s| s == ['2', '4', 'V'] || s == ['2', '4', 'v']);
            if next_is_24v && !prev_is_digit {
                normalized.push(c);
            } else {
                normalized.push('\n');
            }
            i += 1;
            continue;
        }
        // Comma is a decimal separator if surrounded by digits (`2,5` or `1,5`), otherwise a clause separator
        if c == ',' {
            let prev_digit = i > 0 && chars[i - 1].is_ascii_digit();
            let next_digit = i + 1 < chars.len() && chars[i + 1].is_ascii_digit();
            if prev_digit && next_digit {
                normalized.push(c);
            } else {
                normalized.push('\n');
            }
            i += 1;
            continue;
        }
        normalized.push(c);
        i += 1;
    }

    let mut clauses = Vec::new();
    for line in normalized.lines() {
        for sub in split_on_conjunction(line) {
            let trimmed = sub.trim();
            if !trimmed.is_empty() {
                clauses.push(trimmed.to_string());
            }
        }
    }
    clauses
}

fn split_on_conjunction(line: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let lower = line.to_lowercase();
    let mut last = 0;
    let bytes = lower.as_bytes();
    let mut i = 0;
    while i + 5 <= bytes.len() {
        if &bytes[i..i + 5] == b" und "
            && line.is_char_boundary(i)
            && line.is_char_boundary(i + 5)
        {
            out.push(&line[last..i]);
            last = i + 5;
            i += 5;
            continue;
        }
        i += 1;
    }
    out.push(&line[last..]);
    out
}

/// Parses and resolves a natural-language terminal prompt into a deterministic `PromptPlan`.
pub fn parse_and_resolve_prompt(
    input: &str,
    ctx: PromptContext,
    catalog: &TerminalCatalog,
) -> PromptPlan {
    let clauses = split_prompt_clauses(input);
    let mut plan = PromptPlan::default();

    let mut inherited_mfg = ctx.default_manufacturer;
    let mut inherited_style: Option<PromptStyle> = None;
    let mut inherited_cs_mm2: Option<f32> = None;
    let mut inherited_nt: Option<bool> = None;
    let mut inherited_screw = false;

    for raw_clause in clauses {
        if let Some(mfg_only) = parse_standalone_manufacturer_prefix(&raw_clause) {
            inherited_mfg = mfg_only;
            continue;
        }

        let parsed = parse_single_clause(
            &raw_clause,
            inherited_mfg,
            inherited_style.unwrap_or(ctx.default_style),
            inherited_cs_mm2,
            inherited_nt.unwrap_or(ctx.prefer_nt_for_installation),
            inherited_screw,
            catalog,
        );

        let Some(clause) = parsed else {
            plan.unrecognized_clauses.push(raw_clause);
            continue;
        };

        // Update inherited context for subsequent shorthand clauses (e.g. "+ 6x 1-polig")
        inherited_mfg = clause.manufacturer;
        if clause.explicit_style {
            inherited_style = Some(clause.style);
        }
        if clause.explicit_cs {
            inherited_cs_mm2 = Some(clause.cross_section_mm2);
        }
        if clause.explicit_nt {
            inherited_nt = Some(clause.use_nt);
        }
        if clause.explicit_screw {
            inherited_screw = clause.prefer_screw;
        }

        let unit_specs = resolve_unit_specs(&clause, catalog);
        if unit_specs.is_empty() {
            plan.unrecognized_clauses.push(raw_clause);
            continue;
        }

        let mut blocks = Vec::with_capacity(unit_specs.len() * clause.count);
        let mut all_spec_refs: Vec<&TerminalSpec> =
            Vec::with_capacity(unit_specs.len() * clause.count);

        for _ in 0..clause.count {
            for &sp in &unit_specs {
                blocks.push(ResolvedPromptBlock {
                    spec_id: sp.id.clone(),
                    spec_name: sp.name.clone(),
                    manufacturer: sp.manufacturer.clone(),
                    width_tenths_mm: sp.width_tenths_mm,
                    blank_marker: clause.blank_marker,
                });
                all_spec_refs.push(sp);
            }
        }

        let seg_width: u32 = blocks.iter().map(|b| u32::from(b.width_tenths_mm)).sum();
        let summary = format_specs_summary(&all_spec_refs, clause.blank_marker);

        plan.total_blocks += blocks.len();
        plan.total_width_tenths_mm += seg_width;
        plan.segments.push(ResolvedPromptSegment {
            raw_clause,
            count: clause.count,
            intent_label_de: clause.intent_label_de,
            intent_label_en: clause.intent_label_en,
            blocks,
            summary,
            width_tenths_mm: seg_width,
        });
    }

    plan
}

fn parse_standalone_manufacturer_prefix(clause: &str) -> Option<PromptManufacturer> {
    let clean = clause.trim().trim_end_matches(':').trim().to_lowercase();
    match clean.as_str() {
        "phoenix" | "phoenix contact" | "pxc" => Some(PromptManufacturer::PhoenixContact),
        "wago" | "wag" => Some(PromptManufacturer::Wago),
        "weidmüller" | "weidmueller" | "weidmuller" | "wdm" => {
            Some(PromptManufacturer::Weidmueller)
        }
        _ => None,
    }
}

#[derive(Debug, Clone)]
struct ParsedClause {
    count: usize,
    manufacturer: PromptManufacturer,
    style: PromptStyle,
    explicit_style: bool,
    cross_section_mm2: f32,
    explicit_cs: bool,
    use_nt: bool,
    explicit_nt: bool,
    prefer_screw: bool,
    explicit_screw: bool,
    blank_marker: bool,
    kind: PromptCircuitKind,
    intent_label_de: String,
    intent_label_en: String,
}

fn parse_single_clause(
    raw: &str,
    inherited_mfg: PromptManufacturer,
    default_style: PromptStyle,
    inherited_cs: Option<f32>,
    default_nt: bool,
    inherited_screw: bool,
    catalog: &TerminalCatalog,
) -> Option<ParsedClause> {
    // Strip optional leading manufacturer prefix like "WAGO: 4x ..."
    let mut working = raw.trim();
    let mut explicit_mfg: Option<PromptManufacturer> = None;
    if let Some((prefix, rest)) = working.split_once(':')
        && let Some(m) = parse_standalone_manufacturer_prefix(prefix)
    {
        explicit_mfg = Some(m);
        working = rest.trim();
    }

    // 1. Extract leading or inline quantity (e.g. "4x", "4 x", "4×", "4 Stk", "vier")
    let (count, after_count) = extract_quantity(working);
    let text_body = after_count.trim();
    if text_body.is_empty() {
        return None;
    }
    let low = text_body.to_lowercase();

    // 2. Detect blank marker request ("leer", "leeres schild", "unbeschriftet", "blank")
    let blank_marker = contains_word(&low, "leer")
        || low.contains("leeres schild")
        || low.contains("unbeschriftet")
        || contains_word(&low, "blank");

    // 3. Detect explicit manufacturer / series keywords
    let mut explicit_style: Option<PromptStyle> = None;
    let mut explicit_nt: Option<bool> = None;
    let mut explicit_screw = false;
    let mut prefer_screw = inherited_screw;

    if low.contains("schraub") || contains_word(&low, "ut") || contains_word(&low, "uti") || contains_word(&low, "wdu") || contains_word(&low, "wdk") || contains_word(&low, "urtk") {
        prefer_screw = true;
        explicit_screw = true;
    } else if low.contains("push-in") || low.contains("push in") || contains_word(&low, "pt") || contains_word(&low, "pti") || contains_word(&low, "a2c") || contains_word(&low, "aitb") || low.contains("topjob") {
        prefer_screw = false;
        explicit_screw = true;
    }

    if explicit_mfg.is_none() {
        if low.contains("phoenix")
            || contains_word(&low, "pti")
            || contains_word(&low, "pt")
            || contains_word(&low, "pttb")
            || contains_word(&low, "ut")
            || contains_word(&low, "uti")
            || contains_word(&low, "st")
            || contains_word(&low, "urtk")
            || contains_word(&low, "qtc")
            || contains_word(&low, "ptpower")
            || contains_word(&low, "ptme")
        {
            explicit_mfg = Some(PromptManufacturer::PhoenixContact);
        } else if low.contains("wago")
            || low.contains("topjob")
            || contains_word(&low, "2002")
            || contains_word(&low, "2003")
            || contains_word(&low, "2004")
            || contains_word(&low, "2005")
            || contains_word(&low, "2006")
            || contains_word(&low, "2010")
            || contains_word(&low, "2016")
            || contains_word(&low, "2202")
            || contains_word(&low, "2203")
            || contains_word(&low, "285")
        {
            explicit_mfg = Some(PromptManufacturer::Wago);
        } else if low.contains("weidm")
            || contains_word(&low, "aitb")
            || contains_word(&low, "a2c")
            || contains_word(&low, "a3c")
            || contains_word(&low, "a4c")
            || contains_word(&low, "a2t")
            || contains_word(&low, "a3t")
            || contains_word(&low, "wdu")
            || contains_word(&low, "wpe")
            || contains_word(&low, "wdk")
            || contains_word(&low, "zdu")
            || contains_word(&low, "dli")
        {
            explicit_mfg = Some(PromptManufacturer::Weidmueller);
        }
    }

    // 4. Detect terminal construction style
    if low.contains("install")
        || contains_word(&low, "inst")
        || low.contains("inst.")
        || contains_word(&low, "pti")
        || contains_word(&low, "uti")
        || contains_word(&low, "sti")
        || contains_word(&low, "aitb")
        || contains_word(&low, "dli")
        || contains_word(&low, "2003")
        || contains_word(&low, "2203")
        || contains_word(&low, "2005")
        || contains_word(&low, "2205")
        || contains_word(&low, "nt")
    {
        explicit_style = Some(PromptStyle::Installation);
    } else if low.contains("doppelstock")
        || low.contains("2-etagen")
        || low.contains("2-stock")
        || low.contains("zweistock")
        || contains_word(&low, "pttb")
        || contains_word(&low, "uttb")
        || contains_word(&low, "sttb")
        || contains_word(&low, "a2t")
        || contains_word(&low, "wdk")
        || contains_word(&low, "zdk")
    {
        explicit_style = Some(PromptStyle::DoubleLevel);
    } else if low.contains("dreistock")
        || low.contains("3-etagen")
        || low.contains("3-stock")
        || contains_word(&low, "a3t")
        || low.contains("3l")
    {
        explicit_style = Some(PromptStyle::TripleLevel);
    } else if low.contains("durchgang")
        || low.contains("1-etage")
        || low.contains("einstock")
        || contains_word(&low, "a2c")
        || contains_word(&low, "wdu")
        || contains_word(&low, "zdu")
    {
        explicit_style = Some(PromptStyle::FeedThrough);
    }

    // 5. Detect NT preference
    if low.contains("ohne nt") || low.contains("fester n") || low.contains("festem n") {
        explicit_nt = Some(false);
    } else if contains_word(&low, "nt")
        || low.contains("n-trenn")
        || low.contains("trennschlitten")
        || low.contains("pelnt")
        || low.contains("ntlp")
    {
        explicit_nt = Some(true);
    }

    // 6. Extract cross-section in mm² (e.g. "2,5mm²", "16 mm2", "4qmm")
    let extracted_cs = extract_cross_section_mm2(&low);
    let cross_section_mm2 = extracted_cs.or(inherited_cs).unwrap_or(2.5);

    // 7. Determine circuit / component intent
    let is_three_phase = low.contains("drehstrom")
        || low.contains("3-polig")
        || low.contains("3polig")
        || low.contains("3-phasig")
        || low.contains("3phasig")
        || contains_word(&low, "3p")
        || low.contains("400v")
        || low.contains("400 v")
        || contains_word(&low, "kraft")
        || contains_word(&low, "cee");

    let is_motor_no_n = contains_word(&low, "motor")
        || low.contains("motorabgang")
        || low.contains("3l+pe")
        || low.contains("3p+pe")
        || (is_three_phase && (low.contains("ohne n") || low.contains("3-leiter")));

    let is_single_phase = low.contains("wechselstrom")
        || low.contains("1-polig")
        || low.contains("1polig")
        || low.contains("1-phasig")
        || low.contains("1phasig")
        || contains_word(&low, "1p")
        || low.contains("230v")
        || low.contains("230 v")
        || contains_word(&low, "licht")
        || low.contains("steckdose")
        || low.contains("schuko");

    let is_supply = low.contains("einspeisung")
        || low.contains("zuleitung")
        || low.contains("hauptklemme")
        || low.contains("einspeise");

    let is_fuse = low.contains("sicherung")
        || contains_word(&low, "hesi")
        || contains_word(&low, "wsi")
        || contains_word(&low, "afs");

    let is_transformer = low.contains("messwandler")
        || low.contains("wandler")
        || contains_word(&low, "urtk")
        || contains_word(&low, "ptme");

    let is_disconnect = (low.contains("trennklemme")
        || low.contains("messertrenn")
        || low.contains("prüftrenn")
        || contains_word(&low, "mt")
        || contains_word(&low, "tg"))
        && !is_transformer
        && !low.contains("n-trenn");

    let is_reserve = contains_word(&low, "reserve")
        || low.contains("reserveklemme")
        || (blank_marker
            && !is_three_phase
            && !is_single_phase
            && !is_motor_no_n
            && !is_supply
            && !is_fuse
            && !is_disconnect
            && !is_transformer);

    let is_pure_pe = (contains_word(&low, "pe")
        || low.contains("schutzleiter")
        || low.contains("erdung")
        || low.contains("pe-klemme"))
        && !is_three_phase
        && !is_single_phase
        && !is_motor_no_n
        && !low.contains("pe/l");

    let is_pure_n = (contains_word(&low, "n")
        || low.contains("n-klemme")
        || low.contains("neutralleiter")
        || contains_word(&low, "blau"))
        && !is_three_phase
        && !is_single_phase
        && !is_motor_no_n
        && !low.contains("pe/l");

    // 8. Check if user named a specific catalog terminal directly (e.g. "URTK/SP", "2002-3217", "QTC 1,5-TG", "PTTB 2,5")
    let has_circuit_keyword = is_three_phase
        || is_motor_no_n
        || is_single_phase
        || is_supply
        || is_fuse
        || is_disconnect
        || is_reserve
        || is_pure_pe
        || is_pure_n
        || low.contains("abgang")
        || low.contains("abgänge");

    if (!has_circuit_keyword || is_transformer)
        && let Some(direct_spec) = try_match_direct_catalog_spec(text_body, catalog)
    {
        let mfg =
            PromptManufacturer::from_catalog_str(&direct_spec.manufacturer).unwrap_or(inherited_mfg);
        let short = direct_spec.name.split(" (").next().unwrap_or(&direct_spec.name);
        return Some(ParsedClause {
            count,
            manufacturer: mfg,
            style: explicit_style.unwrap_or(default_style),
            explicit_style: explicit_style.is_some(),
            cross_section_mm2,
            explicit_cs: extracted_cs.is_some(),
            use_nt: explicit_nt.unwrap_or(default_nt),
            explicit_nt: explicit_nt.is_some(),
            prefer_screw,
            explicit_screw,
            blank_marker,
            kind: PromptCircuitKind::DirectSpec(direct_spec.id.clone()),
            intent_label_de: format!("Direkt-Typ {short}"),
            intent_label_en: format!("Direct Type {short}"),
        });
    }

    let mfg = explicit_mfg.unwrap_or(inherited_mfg);
    let style = explicit_style.unwrap_or(default_style);
    let use_nt = explicit_nt.unwrap_or(default_nt);

    let (kind, label_de, label_en) = if is_motor_no_n {
        (
            PromptCircuitKind::ThreePhasePe,
            format!("Motor-Abgang 3L+PE ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
            format!("Motor Feeder 3L+PE ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
        )
    } else if is_supply {
        let cs = extracted_cs.unwrap_or(16.0);
        return Some(ParsedClause {
            count,
            manufacturer: mfg,
            style: PromptStyle::FeedThrough,
            explicit_style: false,
            cross_section_mm2: cs,
            explicit_cs: extracted_cs.is_some(),
            use_nt,
            explicit_nt: explicit_nt.is_some(),
            prefer_screw,
            explicit_screw,
            blank_marker,
            kind: PromptCircuitKind::SupplyFeeder,
            intent_label_de: format!("Einspeisung 3L+N+PE ({:.1} mm²)", cs).replace(".0 ", " "),
            intent_label_en: format!("Supply Feeder 3L+N+PE ({:.1} mm²)", cs).replace(".0 ", " "),
        });
    } else if is_three_phase {
        let mode_str = match style {
            PromptStyle::Installation => {
                if use_nt {
                    "Inst. NT"
                } else {
                    "Inst. N"
                }
            }
            PromptStyle::DoubleLevel => "Doppelstock",
            PromptStyle::TripleLevel => "Dreistock",
            PromptStyle::FeedThrough => "Durchgang",
        };
        (
            PromptCircuitKind::ThreePhaseNPe,
            format!(
                "Drehstrom-Abgang 3L+N+PE ({:.1} mm², {mode_str})",
                cross_section_mm2
            )
            .replace(".0 ", " "),
            format!(
                "3-Phase Feeder 3L+N+PE ({:.1} mm², {mode_str})",
                cross_section_mm2
            )
            .replace(".0 ", " "),
        )
    } else if is_single_phase {
        let mode_str = match style {
            PromptStyle::Installation => {
                if use_nt {
                    "Inst. NT"
                } else {
                    "Inst. N"
                }
            }
            PromptStyle::DoubleLevel => "Doppelstock",
            PromptStyle::TripleLevel => "Dreistock",
            PromptStyle::FeedThrough => "Durchgang",
        };
        (
            PromptCircuitKind::SinglePhaseNPe,
            format!(
                "Wechselstrom-Abgang L+N+PE ({:.1} mm², {mode_str})",
                cross_section_mm2
            )
            .replace(".0 ", " "),
            format!(
                "1-Phase Feeder L+N+PE ({:.1} mm², {mode_str})",
                cross_section_mm2
            )
            .replace(".0 ", " "),
        )
    } else if is_fuse {
        (
            PromptCircuitKind::Fuse,
            format!("Sicherungsklemme ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
            format!("Fuse Terminal ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
        )
    } else if is_transformer {
        (
            PromptCircuitKind::TransformerTest,
            "Messwandlertrennklemme (6 mm²)".to_string(),
            "CT Test Disconnect Terminal (6 mm²)".to_string(),
        )
    } else if is_disconnect {
        (
            PromptCircuitKind::Disconnect,
            format!("Trennklemme ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
            format!("Disconnect Terminal ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
        )
    } else if is_pure_pe {
        (
            PromptCircuitKind::GroundPe,
            format!("Schutzleiterklemme PE ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
            format!("PE Ground Terminal ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
        )
    } else if is_pure_n {
        (
            PromptCircuitKind::NeutralN,
            format!("Neutralleiterklemme N ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
            format!("Neutral Terminal N ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
        )
    } else if is_reserve {
        (
            PromptCircuitKind::Reserve,
            format!("Reserveklemme ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
            format!("Spare Terminal ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
        )
    } else if low.contains("abgang")
        || low.contains("abgänge")
        || explicit_style == Some(PromptStyle::Installation)
    {
        // E.g. "4x Installationsklemme 2,5mm²" or "4x Abgang 2,5mm²" defaults to 1-phase installation/feeder
        (
            PromptCircuitKind::SinglePhaseNPe,
            format!("Abgang L+N+PE ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
            format!("Feeder L+N+PE ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
        )
    } else if low.contains("klemme")
        || low.contains("durchgang")
        || low.contains("24v")
        || low.contains("0v")
        || extracted_cs.is_some()
        || explicit_style.is_some()
    {
        (
            PromptCircuitKind::SingleFeedThrough,
            format!("Reihenklemme ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
            format!("Terminal Block ({:.1} mm²)", cross_section_mm2).replace(".0 ", " "),
        )
    } else {
        let direct_spec = try_match_direct_catalog_spec(text_body, catalog)?;
        let dmfg =
            PromptManufacturer::from_catalog_str(&direct_spec.manufacturer).unwrap_or(inherited_mfg);
        let short = direct_spec.name.split(" (").next().unwrap_or(&direct_spec.name);
        return Some(ParsedClause {
            count,
            manufacturer: dmfg,
            style,
            explicit_style: explicit_style.is_some(),
            cross_section_mm2,
            explicit_cs: extracted_cs.is_some(),
            use_nt,
            explicit_nt: explicit_nt.is_some(),
            prefer_screw,
            explicit_screw,
            blank_marker,
            kind: PromptCircuitKind::DirectSpec(direct_spec.id.clone()),
            intent_label_de: format!("Direkt-Typ {short}"),
            intent_label_en: format!("Direct Type {short}"),
        });
    };

    Some(ParsedClause {
        count,
        manufacturer: mfg,
        style,
        explicit_style: explicit_style.is_some(),
        cross_section_mm2,
        explicit_cs: extracted_cs.is_some(),
        use_nt,
        explicit_nt: explicit_nt.is_some(),
        prefer_screw,
        explicit_screw,
        blank_marker,
        kind,
        intent_label_de: label_de,
        intent_label_en: label_en,
    })
}

fn try_match_direct_catalog_spec<'a>(
    raw_body: &str,
    catalog: &'a TerminalCatalog,
) -> Option<&'a TerminalSpec> {
    // Strip common noise words before looking up by article/type
    let mut cleaned = raw_body.to_string();
    for noise in [
        "Messwandlertrennklemme",
        "messwandlertrennklemme",
        "Wandlerklemme",
        "wandlerklemme",
        "Wandler",
        "wandler",
        "Reihenklemme",
        "reihenklemme",
        "Klemmen",
        "klemmen",
        "Klemme",
        "klemme",
        "leer",
        "leeres Schild",
    ] {
        cleaned = cleaned.replace(noise, " ");
    }
    let trimmed = cleaned.trim();
    if trimmed.len() < 3 {
        return None;
    }

    if let Some(sp) = catalog.find_by_article_or_type(None, trimmed, trimmed) {
        return Some(sp);
    }

    // Also check individual tokens (e.g. "2002-3217", "URTK/SP", "0311126")
    for token in trimmed.split_whitespace() {
        let tok = token.trim_matches(|c: char| c == ',' || c == ';' || c == '(' || c == ')');
        if tok.len() >= 4
            && (tok.contains('-') || tok.contains('/') || tok.chars().all(|c| c.is_ascii_digit()))
            && let Some(sp) = catalog.find_by_article_or_type(None, tok, tok)
        {
            return Some(sp);
        }
    }

    None
}

/// Extracts a leading or embedded multiplier quantity (`"4x ..."`, `"4 Stk ..."`, `"vier ..."`).
fn extract_quantity(text: &str) -> (usize, &str) {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return (1, "");
    }

    // 1. Leading digits followed by optional `x`, `×`, `*`, `stk`, `stück`, `mal`
    let digit_end = trimmed
        .char_indices()
        .take_while(|(_, c)| c.is_ascii_digit())
        .last()
        .map(|(idx, c)| idx + c.len_utf8())
        .unwrap_or(0);

    if digit_end > 0 {
        let num_str = &trimmed[..digit_end];
        let rest = trimmed[digit_end..].trim_start();
        let rest_low = rest.to_lowercase();
        // Make sure we didn't match "2002-..." or "2,5mm²" or "3-polig" or "24V"
        let is_part_of_other = rest.starts_with('-')
            || rest.starts_with(',')
            || rest.starts_with('.')
            || rest_low.starts_with("mm")
            || rest_low.starts_with("qmm")
            || rest_low.starts_with("v ")
            || rest_low == "v"
            || rest_low.starts_with("pol")
            || rest_low.starts_with("phas")
            || rest_low.starts_with("etag")
            || rest_low.starts_with("stock")
            || rest_low.starts_with("leiter");

        if !is_part_of_other && let Ok(n) = num_str.parse::<usize>() {
            let after_unit = if let Some(r) = rest.strip_prefix('x').or_else(|| rest.strip_prefix('X')).or_else(|| rest.strip_prefix('×')).or_else(|| rest.strip_prefix('*')) {
                r.trim_start()
            } else if rest_low.starts_with("stk.") || rest_low.starts_with("stk ") {
                rest[4..].trim_start()
            } else if rest_low.starts_with("stück ") {
                rest["stück ".len()..].trim_start()
            } else if rest_low.starts_with("mal ") {
                rest[4..].trim_start()
            } else {
                rest
            };
            return (n.clamp(1, 250), after_unit);
        }
    }

    // 2. German/English number words at start ("ein", "zwei", "drei", "vier", ...)
    if let Some((first_word, rest)) = trimmed.split_once(char::is_whitespace) {
        let fw = first_word.to_lowercase();
        let word_num = match fw.as_str() {
            "ein" | "eine" | "einen" | "one" => Some(1),
            "zwei" | "two" => Some(2),
            "drei" | "three" => Some(3),
            "vier" | "four" => Some(4),
            "fünf" | "fuenf" | "five" => Some(5),
            "sechs" | "six" => Some(6),
            "sieben" | "seven" => Some(7),
            "acht" | "eight" => Some(8),
            "neun" | "nine" => Some(9),
            "zehn" | "ten" => Some(10),
            "zwölf" | "zwoelf" | "twelve" => Some(12),
            _ => None,
        };
        if let Some(n) = word_num {
            return (n, rest.trim_start());
        }
    }

    (1, trimmed)
}

/// Extracts a wire cross-section in mm² (`1.5`, `2.5`, `4`, `6`, `10`, `16`, `35`, `50`, `95`).
fn extract_cross_section_mm2(low: &str) -> Option<f32> {
    let bytes = low.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b',' || bytes[i] == b'.') {
                i += 1;
            }
            let num_slice = &low[start..i];
            let after = low[i..].trim_start();
            if after.starts_with("mm²")
                || after.starts_with("mm2")
                || after.starts_with("qmm")
                || after.starts_with("mm ")
            {
                let norm = num_slice.replace(',', ".");
                if let Ok(val) = norm.parse::<f32>()
                    && (0.5..=240.0).contains(&val)
                {
                    return Some(snap_standard_cross_section(val));
                }
            }
        } else {
            i += 1;
        }
    }
    None
}

fn snap_standard_cross_section(val: f32) -> f32 {
    const STANDARDS: [f32; 9] = [1.5, 2.5, 4.0, 6.0, 10.0, 16.0, 35.0, 50.0, 95.0];
    let mut best = 2.5;
    let mut best_diff = f32::MAX;
    for s in STANDARDS {
        let d = (val - s).abs();
        if d < best_diff {
            best_diff = d;
            best = s;
        }
    }
    best
}

fn contains_word(haystack_lower: &str, word: &str) -> bool {
    haystack_lower
        .split(|c: char| !c.is_alphanumeric() && c != '+' && c != '-')
        .any(|tok| tok == word)
}

/// Resolves the unit sequence of `TerminalSpec`s for a single occurrence (`1×`) of `clause`.
fn resolve_unit_specs<'a>(
    clause: &ParsedClause,
    catalog: &'a TerminalCatalog,
) -> Vec<&'a TerminalSpec> {
    let find = |id: &str| catalog.find_by_id(id);
    let ids_to_specs = |ids: &[&str]| -> Vec<&'a TerminalSpec> {
        ids.iter().filter_map(|id| find(id)).collect()
    };

    if let PromptCircuitKind::DirectSpec(ref id) = clause.kind {
        return find(id).into_iter().collect();
    }

    let cs = clause.cross_section_mm2;
    let mfg = clause.manufacturer;
    let nt = clause.use_nt;
    let screw = clause.prefer_screw;

    match clause.kind {
        PromptCircuitKind::ThreePhaseNPe => match mfg {
            PromptManufacturer::PhoenixContact => match clause.style {
                PromptStyle::Installation => {
                    if cs >= 4.0 {
                        let peln = if nt { "phoenix-pti-4-pelnt" } else { "phoenix-pti-4-peln" };
                        ids_to_specs(&["phoenix-pti-4-ll", peln])
                    } else {
                        let peln = if nt { "phoenix-pti-25-pelnt" } else { "phoenix-pti-25-peln" };
                        ids_to_specs(&["phoenix-pti-25-ll", peln])
                    }
                }
                PromptStyle::DoubleLevel => {
                    if cs <= 1.5 {
                        ids_to_specs(&["phoenix-pttb-15s", "phoenix-pttb-15s-ln", "phoenix-pttb-15s-pe"])
                    } else if cs >= 4.0 {
                        ids_to_specs(&["phoenix-pttb-4", "phoenix-pttb-4", "phoenix-pttb-4-bu", "phoenix-pttb-4-pe"])
                    } else {
                        ids_to_specs(&["phoenix-pttb-25", "phoenix-pttb-25-ln-real", "phoenix-pttb-25-pe"])
                    }
                }
                PromptStyle::TripleLevel => {
                    if cs <= 1.5 {
                        ids_to_specs(&["phoenix-pt-15s-pell", "phoenix-pt-15s-peln"])
                    } else {
                        ids_to_specs(&["phoenix-pt-25-pell", "phoenix-pt-25-peln"])
                    }
                }
                PromptStyle::FeedThrough => {
                    let (l, n, pe) = phoenix_1level_triplet(cs, screw);
                    ids_to_specs(&[l, l, l, n, pe])
                }
            },
            PromptManufacturer::Wago => match clause.style {
                PromptStyle::Installation => {
                    if cs >= 4.0 {
                        let peln = if nt { "wago-2005-7641" } else { "wago-2005-7646" };
                        ids_to_specs(&["wago-2005-7642", peln])
                    } else {
                        let peln = if nt { "wago-2003-7641" } else { "wago-2003-7646" };
                        ids_to_specs(&["wago-2003-7642", peln])
                    }
                }
                PromptStyle::DoubleLevel => {
                    ids_to_specs(&["wago-2002-2201", "wago-2002-2204", "wago-2002-2207"])
                }
                PromptStyle::TripleLevel => {
                    ids_to_specs(&["wago-2002-3227", "wago-2002-3217"])
                }
                PromptStyle::FeedThrough => {
                    let (l, n, pe) = wago_1level_triplet(cs);
                    ids_to_specs(&[l, l, l, n, pe])
                }
            },
            PromptManufacturer::Weidmueller => match clause.style {
                PromptStyle::Installation => {
                    if cs >= 4.0 {
                        let peln = if nt { "weidmuller-aitb-4-ntlp" } else { "weidmuller-aitb-4-nlp" };
                        ids_to_specs(&["weidmuller-aitb-4-ll", peln])
                    } else {
                        let peln = if nt { "weidmuller-aitb-25-ntlp" } else { "weidmuller-aitb-25-nlp" };
                        ids_to_specs(&["weidmuller-aitb-25-ll", peln])
                    }
                }
                PromptStyle::DoubleLevel => {
                    if screw {
                        ids_to_specs(&["weidmuller-wdk-25", "weidmuller-wdk-25-n", "weidmuller-wdk-25-pe"])
                    } else if cs >= 4.0 {
                        ids_to_specs(&["weidmuller-a2t-4", "weidmuller-a2t-4-ft-pe", "weidmuller-a2c-4-bl"])
                    } else {
                        ids_to_specs(&["weidmuller-a2t-25", "weidmuller-a2t-25-ft-pe", "weidmuller-a2c-25-bl"])
                    }
                }
                PromptStyle::TripleLevel => {
                    ids_to_specs(&["weidmuller-a3t-25-pell", "weidmuller-a3t-25-peln"])
                }
                PromptStyle::FeedThrough => {
                    let (l, n, pe) = weidmuller_1level_triplet(cs, screw);
                    ids_to_specs(&[l, l, l, n, pe])
                }
            },
        },

        PromptCircuitKind::ThreePhasePe => match mfg {
            PromptManufacturer::PhoenixContact => match clause.style {
                PromptStyle::Installation => {
                    if cs >= 4.0 {
                        ids_to_specs(&["phoenix-pti-4-ll", "phoenix-pti-4-pell"])
                    } else {
                        ids_to_specs(&["phoenix-pti-25-ll", "phoenix-pti-25-pell"])
                    }
                }
                PromptStyle::DoubleLevel => {
                    ids_to_specs(&["phoenix-pttb-25", "phoenix-pttb-25-pel"])
                }
                PromptStyle::TripleLevel => {
                    ids_to_specs(&["phoenix-pt-25-3l", "phoenix-pt-25-3pe"])
                }
                PromptStyle::FeedThrough => {
                    let (l, _, pe) = phoenix_1level_triplet(cs, screw);
                    ids_to_specs(&[l, l, l, pe])
                }
            },
            PromptManufacturer::Wago => match clause.style {
                PromptStyle::Installation => {
                    if cs >= 4.0 {
                        ids_to_specs(&["wago-2005-7642", "wago-2005-7645"])
                    } else {
                        ids_to_specs(&["wago-2003-7642", "wago-2003-7645"])
                    }
                }
                PromptStyle::DoubleLevel => {
                    ids_to_specs(&["wago-2002-2201", "wago-2002-2208"])
                }
                PromptStyle::TripleLevel => {
                    ids_to_specs(&["wago-2002-3201", "wago-2002-3207"])
                }
                PromptStyle::FeedThrough => {
                    let (l, _, pe) = wago_1level_triplet(cs);
                    ids_to_specs(&[l, l, l, pe])
                }
            },
            PromptManufacturer::Weidmueller => match clause.style {
                PromptStyle::Installation => {
                    if cs >= 4.0 {
                        ids_to_specs(&["weidmuller-aitb-4-ll", "weidmuller-aitb-4-llp"])
                    } else {
                        ids_to_specs(&["weidmuller-aitb-25-ll", "weidmuller-aitb-25-nlp"])
                    }
                }
                PromptStyle::DoubleLevel => {
                    ids_to_specs(&["weidmuller-a2t-25", "weidmuller-a2t-25-ft-pe"])
                }
                PromptStyle::TripleLevel => {
                    ids_to_specs(&["weidmuller-a3t-25", "weidmuller-a3t-25-pe"])
                }
                PromptStyle::FeedThrough => {
                    let (l, _, pe) = weidmuller_1level_triplet(cs, screw);
                    ids_to_specs(&[l, l, l, pe])
                }
            },
        },

        PromptCircuitKind::SinglePhaseNPe => match mfg {
            PromptManufacturer::PhoenixContact => match clause.style {
                PromptStyle::Installation => {
                    if cs >= 4.0 {
                        let peln = if nt { "phoenix-pti-4-pelnt" } else { "phoenix-pti-4-peln" };
                        ids_to_specs(&[peln])
                    } else {
                        let peln = if nt { "phoenix-pti-25-pelnt" } else { "phoenix-pti-25-peln" };
                        ids_to_specs(&[peln])
                    }
                }
                PromptStyle::DoubleLevel => {
                    if cs <= 1.5 {
                        ids_to_specs(&["phoenix-pttb-15s-ln", "phoenix-pttb-15s-pe"])
                    } else {
                        ids_to_specs(&["phoenix-pttb-25-ln-real", "phoenix-pttb-25-pe"])
                    }
                }
                PromptStyle::TripleLevel => {
                    if cs <= 1.5 {
                        ids_to_specs(&["phoenix-pt-15s-peln"])
                    } else {
                        ids_to_specs(&["phoenix-pt-25-peln"])
                    }
                }
                PromptStyle::FeedThrough => {
                    let (l, n, pe) = phoenix_1level_triplet(cs, screw);
                    ids_to_specs(&[l, n, pe])
                }
            },
            PromptManufacturer::Wago => match clause.style {
                PromptStyle::Installation => {
                    if cs >= 4.0 {
                        let peln = if nt { "wago-2005-7641" } else { "wago-2005-7646" };
                        ids_to_specs(&[peln])
                    } else {
                        let peln = if nt { "wago-2003-7641" } else { "wago-2003-7646" };
                        ids_to_specs(&[peln])
                    }
                }
                PromptStyle::DoubleLevel => {
                    ids_to_specs(&["wago-2002-2204", "wago-2002-2207"])
                }
                PromptStyle::TripleLevel => {
                    ids_to_specs(&["wago-2002-3217"])
                }
                PromptStyle::FeedThrough => {
                    let (l, n, pe) = wago_1level_triplet(cs);
                    ids_to_specs(&[l, n, pe])
                }
            },
            PromptManufacturer::Weidmueller => match clause.style {
                PromptStyle::Installation => {
                    if cs >= 4.0 {
                        let peln = if nt { "weidmuller-aitb-4-ntlp" } else { "weidmuller-aitb-4-nlp" };
                        ids_to_specs(&[peln])
                    } else {
                        let peln = if nt { "weidmuller-aitb-25-ntlp" } else { "weidmuller-aitb-25-nlp" };
                        ids_to_specs(&[peln])
                    }
                }
                PromptStyle::DoubleLevel => {
                    if screw {
                        ids_to_specs(&["weidmuller-wdk-25-n", "weidmuller-wdk-25-pe"])
                    } else {
                        ids_to_specs(&["weidmuller-a2t-25-ft-pe", "weidmuller-a2c-25-bl"])
                    }
                }
                PromptStyle::TripleLevel => {
                    ids_to_specs(&["weidmuller-a3t-25-peln"])
                }
                PromptStyle::FeedThrough => {
                    let (l, n, pe) = weidmuller_1level_triplet(cs, screw);
                    ids_to_specs(&[l, n, pe])
                }
            },
        },

        PromptCircuitKind::SupplyFeeder => match mfg {
            PromptManufacturer::PhoenixContact => {
                let (l, n, pe) = phoenix_1level_triplet(cs, screw);
                ids_to_specs(&[l, l, l, n, pe])
            }
            PromptManufacturer::Wago => {
                let (l, n, pe) = wago_1level_triplet(cs);
                ids_to_specs(&[l, l, l, n, pe])
            }
            PromptManufacturer::Weidmueller => {
                let (l, n, pe) = weidmuller_1level_triplet(cs, screw);
                ids_to_specs(&[l, l, l, n, pe])
            }
        },

        PromptCircuitKind::Fuse => match mfg {
            PromptManufacturer::PhoenixContact => {
                if screw {
                    ids_to_specs(&["phoenix-ut-4-hesi"])
                } else {
                    ids_to_specs(&["phoenix-pt-4-hesi"])
                }
            }
            PromptManufacturer::Wago => {
                if cs >= 6.0 {
                    ids_to_specs(&["wago-2006-1611"])
                } else {
                    ids_to_specs(&["wago-2002-1611"])
                }
            }
            PromptManufacturer::Weidmueller => {
                if screw {
                    ids_to_specs(&["weidmuller-wsi-6"])
                } else {
                    ids_to_specs(&["weidmuller-afs-4"])
                }
            }
        },

        PromptCircuitKind::TransformerTest => match mfg {
            PromptManufacturer::PhoenixContact => {
                if screw {
                    ids_to_specs(&["phoenix-urtk-sp"])
                } else {
                    ids_to_specs(&["phoenix-ptme-6"])
                }
            }
            PromptManufacturer::Wago => ids_to_specs(&["wago-2007-8821"]),
            PromptManufacturer::Weidmueller => ids_to_specs(&["weidmuller-wtl-6-1"]),
        },

        PromptCircuitKind::Disconnect => match mfg {
            PromptManufacturer::PhoenixContact => {
                if cs <= 1.5 {
                    ids_to_specs(&["phoenix-pt-15s-mt"])
                } else if cs >= 4.0 {
                    if screw {
                        ids_to_specs(&["phoenix-ut-4-mt"])
                    } else {
                        ids_to_specs(&["phoenix-pt-4-mt"])
                    }
                } else if screw {
                    ids_to_specs(&["phoenix-ut-25-mt"])
                } else {
                    ids_to_specs(&["phoenix-pt-25-mt"])
                }
            }
            PromptManufacturer::Wago => ids_to_specs(&["wago-2002-1661"]),
            PromptManufacturer::Weidmueller => {
                if screw {
                    ids_to_specs(&["weidmuller-wtr-25"])
                } else {
                    ids_to_specs(&["weidmuller-adt-25"])
                }
            }
        },

        PromptCircuitKind::GroundPe => match mfg {
            PromptManufacturer::PhoenixContact => {
                let (_, _, pe) = phoenix_1level_triplet(cs, screw);
                ids_to_specs(&[pe])
            }
            PromptManufacturer::Wago => {
                let (_, _, pe) = wago_1level_triplet(cs);
                ids_to_specs(&[pe])
            }
            PromptManufacturer::Weidmueller => {
                let (_, _, pe) = weidmuller_1level_triplet(cs, screw);
                ids_to_specs(&[pe])
            }
        },

        PromptCircuitKind::NeutralN => match mfg {
            PromptManufacturer::PhoenixContact => {
                let (_, n, _) = phoenix_1level_triplet(cs, screw);
                ids_to_specs(&[n])
            }
            PromptManufacturer::Wago => {
                let (_, n, _) = wago_1level_triplet(cs);
                ids_to_specs(&[n])
            }
            PromptManufacturer::Weidmueller => {
                let (_, n, _) = weidmuller_1level_triplet(cs, screw);
                ids_to_specs(&[n])
            }
        },

        PromptCircuitKind::Reserve | PromptCircuitKind::SingleFeedThrough => match mfg {
            PromptManufacturer::PhoenixContact => match clause.style {
                PromptStyle::DoubleLevel => ids_to_specs(&["phoenix-pttb-25"]),
                PromptStyle::TripleLevel => ids_to_specs(&["phoenix-pt-25-3l"]),
                _ => {
                    let (l, _, _) = phoenix_1level_triplet(cs, screw);
                    ids_to_specs(&[l])
                }
            },
            PromptManufacturer::Wago => match clause.style {
                PromptStyle::DoubleLevel => ids_to_specs(&["wago-2002-2201"]),
                PromptStyle::TripleLevel => ids_to_specs(&["wago-2002-3201"]),
                _ => {
                    let (l, _, _) = wago_1level_triplet(cs);
                    ids_to_specs(&[l])
                }
            },
            PromptManufacturer::Weidmueller => match clause.style {
                PromptStyle::DoubleLevel => {
                    if screw {
                        ids_to_specs(&["weidmuller-wdk-25"])
                    } else {
                        ids_to_specs(&["weidmuller-a2t-25"])
                    }
                }
                PromptStyle::TripleLevel => ids_to_specs(&["weidmuller-a3t-25"]),
                _ => {
                    let (l, _, _) = weidmuller_1level_triplet(cs, screw);
                    ids_to_specs(&[l])
                }
            },
        },

        PromptCircuitKind::DirectSpec(_) => unreachable!(),
    }
}

fn phoenix_1level_triplet(cs: f32, screw: bool) -> (&'static str, &'static str, &'static str) {
    if screw {
        if cs >= 35.0 {
            ("phoenix-ut-35", "phoenix-ut-35-bu", "phoenix-ut-35-pe")
        } else if cs >= 16.0 {
            ("phoenix-ut-16", "phoenix-ut-16-bu", "phoenix-ut-16-pe")
        } else if cs >= 10.0 {
            ("phoenix-ut-10", "phoenix-ut-10-bu", "phoenix-ut-10-pe")
        } else if cs >= 6.0 {
            ("phoenix-ut-6", "phoenix-ut-6-bu", "phoenix-ut-6-pe")
        } else if cs >= 4.0 {
            ("phoenix-ut-4", "phoenix-ut-4-bu", "phoenix-ut-4-pe")
        } else {
            ("phoenix-ut-25", "phoenix-ut-25-bu", "phoenix-ut-25-pe")
        }
    } else if cs >= 95.0 {
        ("phoenix-ptpower-95", "phoenix-ptpower-95-bu", "phoenix-ptpower-95-pe")
    } else if cs >= 50.0 {
        ("phoenix-ptpower-50", "phoenix-ptpower-50-bu", "phoenix-ptpower-50-pe")
    } else if cs >= 35.0 {
        ("phoenix-ptpower-35", "phoenix-ptpower-35-bu", "phoenix-ptpower-35-pe")
    } else if cs >= 16.0 {
        ("phoenix-pt-16n", "phoenix-pt-16n-bu", "phoenix-pt-16n-pe")
    } else if cs >= 10.0 {
        ("phoenix-pt-10", "phoenix-pt-10-bu", "phoenix-pt-10-pe")
    } else if cs >= 6.0 {
        ("phoenix-pt-6", "phoenix-pt-6-bu", "phoenix-pt-6-pe")
    } else if cs >= 4.0 {
        ("phoenix-pt-4", "phoenix-pt-4-bu", "phoenix-pt-4-pe")
    } else if cs <= 1.5 {
        ("phoenix-pt-15s", "phoenix-pt-15s-bu", "phoenix-pt-15s-pe")
    } else {
        ("phoenix-pt-25", "phoenix-pt-25-bu", "phoenix-pt-25-pe")
    }
}

fn wago_1level_triplet(cs: f32) -> (&'static str, &'static str, &'static str) {
    if cs >= 95.0 {
        ("wago-285-195", "wago-285-194", "wago-285-197")
    } else if cs >= 50.0 {
        ("wago-285-150", "wago-285-154", "wago-285-157")
    } else if cs >= 35.0 {
        ("wago-285-135", "wago-285-134", "wago-285-137")
    } else if cs >= 16.0 {
        ("wago-2016-1201", "wago-2016-1204", "wago-2016-1207")
    } else if cs >= 10.0 {
        ("wago-2010-1201", "wago-2010-1204", "wago-2010-1207")
    } else if cs >= 6.0 {
        ("wago-2006-1201", "wago-2006-1204", "wago-2006-1207")
    } else if cs >= 4.0 {
        ("wago-2004-1201", "wago-2004-1204", "wago-2004-1207")
    } else if cs <= 1.5 {
        ("wago-2001-1201", "wago-2001-1204", "wago-2001-1207")
    } else {
        ("wago-2002-1201", "wago-2002-1204", "wago-2002-1207")
    }
}

fn weidmuller_1level_triplet(cs: f32, screw: bool) -> (&'static str, &'static str, &'static str) {
    if screw {
        if cs >= 35.0 {
            ("weidmuller-wdu-35", "weidmuller-wdu-35-bl", "weidmuller-wpe-35")
        } else if cs >= 16.0 {
            ("weidmuller-wdu-16", "weidmuller-wdu-16-bl", "weidmuller-wpe-16")
        } else if cs >= 10.0 {
            ("weidmuller-wdu-10", "weidmuller-wdu-10-bl", "weidmuller-wpe-10")
        } else if cs >= 6.0 {
            ("weidmuller-wdu-6", "weidmuller-wdu-6-bl", "weidmuller-wpe-6")
        } else if cs >= 4.0 {
            ("weidmuller-wdu-4", "weidmuller-wdu-4-bl", "weidmuller-wpe-4")
        } else {
            ("weidmuller-wdu-25", "weidmuller-wdu-25-bl", "weidmuller-wpe-25")
        }
    } else if cs >= 35.0 {
        ("weidmuller-a2c-35", "weidmuller-a2c-35-bl", "weidmuller-a2c-35-pe")
    } else if cs >= 16.0 {
        ("weidmuller-a2c-16", "weidmuller-a2c-16-bl", "weidmuller-a2c-16-pe")
    } else if cs >= 10.0 {
        ("weidmuller-a2c-10", "weidmuller-a2c-10-bl", "weidmuller-a2c-10-pe")
    } else if cs >= 6.0 {
        ("weidmuller-a2c-6", "weidmuller-a2c-6-bl", "weidmuller-a2c-6-pe")
    } else if cs >= 4.0 {
        ("weidmuller-a2c-4", "weidmuller-a2c-4-bl", "weidmuller-a2c-4-pe")
    } else if cs <= 1.5 {
        ("weidmuller-a2c-15", "weidmuller-a2c-15-bl", "weidmuller-a2c-15-pe")
    } else {
        ("weidmuller-a2c-25", "weidmuller-a2c-25-bl", "weidmuller-a2c-25-pe")
    }
}

fn format_specs_summary(specs: &[&TerminalSpec], blank_marker: bool) -> String {
    let mut counts: Vec<(&str, usize)> = Vec::new();
    for sp in specs {
        let short = sp.name.split(" (").next().unwrap_or(&sp.name).trim();
        if let Some((_, c)) = counts.iter_mut().find(|(name, _)| *name == short) {
            *c += 1;
        } else {
            counts.push((short, 1));
        }
    }
    let base = counts
        .into_iter()
        .map(|(name, c)| format!("{c}× {name}"))
        .collect::<Vec<_>>()
        .join(" + ");
    if blank_marker {
        format!("{base} [Leer]")
    } else {
        base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_drehstrom_installation_and_mixed_clauses_across_manufacturers() {
        let catalog = TerminalCatalog::load_embedded();
        let ctx = PromptContext::default();

        // User's exact example: "4x Drehstrom Abgang 2,5mm² Installationsklemmen + 6x Licht 1,5mm² + 1x 16mm² Einspeisung"
        let plan = parse_and_resolve_prompt(
            "4x Drehstrom Abgang 2,5mm² Installationsklemmen + 6x Licht 1,5mm² + 1x 16mm² Einspeisung",
            ctx,
            &catalog,
        );

        assert!(plan.unrecognized_clauses.is_empty(), "{:?}", plan.unrecognized_clauses);
        assert_eq!(plan.segments.len(), 3);

        // Segment 0: 4x Drehstrom PTI 2,5 -> 4x PTI 2,5-L/L + 4x PTI 2,5-PE/L/NT = 8 blocks
        let s0 = &plan.segments[0];
        assert_eq!(s0.count, 4);
        assert_eq!(s0.blocks.len(), 8);
        assert_eq!(
            s0.blocks.iter().filter(|b| b.spec_id == "phoenix-pti-25-ll").count(),
            4
        );
        assert_eq!(
            s0.blocks.iter().filter(|b| b.spec_id == "phoenix-pti-25-pelnt").count(),
            4
        );

        // Segment 1: 6x Licht 1,5mm² (inherits Installation style + Phoenix) -> 6x PTI 2,5-PE/L/NT
        let s1 = &plan.segments[1];
        assert_eq!(s1.count, 6);
        assert_eq!(s1.blocks.len(), 6);
        assert!(s1.blocks.iter().all(|b| b.spec_id == "phoenix-pti-25-pelnt"));

        // Segment 2: 1x 16mm² Einspeisung -> 3x PT 16 N + 1x PT 16 N BU + 1x PT 16 N-PE = 5 blocks
        let s2 = &plan.segments[2];
        assert_eq!(s2.blocks.len(), 5);
        assert_eq!(
            s2.blocks.iter().filter(|b| b.spec_id == "phoenix-pt-16n").count(),
            3
        );
        assert_eq!(
            s2.blocks.iter().filter(|b| b.spec_id == "phoenix-pt-16n-bu").count(),
            1
        );
        assert_eq!(
            s2.blocks.iter().filter(|b| b.spec_id == "phoenix-pt-16n-pe").count(),
            1
        );
    }

    #[test]
    fn parses_wago_weidmuller_motors_fuses_direct_types_and_blank_reserve() {
        let catalog = TerminalCatalog::load_embedded();
        let ctx = PromptContext {
            default_manufacturer: PromptManufacturer::Wago,
            prefer_nt_for_installation: true,
            default_style: PromptStyle::Installation,
        };

        let plan = parse_and_resolve_prompt(
            "3x Drehstrom 2,5mm² 2003 + 2x Motor 4mm² Durchgang + 4x URTK/SP + 2x Reserve leer",
            ctx,
            &catalog,
        );
        assert!(plan.unrecognized_clauses.is_empty(), "{:?}", plan.unrecognized_clauses);
        assert_eq!(plan.segments.len(), 4);

        // 1. 3x WAGO 2003 Drehstrom -> 3x 2003-7642 + 3x 2003-7641 = 6 blocks
        assert_eq!(plan.segments[0].blocks.len(), 6);
        assert_eq!(plan.segments[0].blocks[0].spec_id, "wago-2003-7642");
        assert_eq!(plan.segments[0].blocks[1].spec_id, "wago-2003-7641");

        // 2. 2x Motor 4mm² Durchgang -> 2 * (3x 2004-1201 + 1x 2004-1207) = 8 blocks
        assert_eq!(plan.segments[1].blocks.len(), 8);
        assert_eq!(
            plan.segments[1]
                .blocks
                .iter()
                .filter(|b| b.spec_id == "wago-2004-1201")
                .count(),
            6
        );
        assert_eq!(
            plan.segments[1]
                .blocks
                .iter()
                .filter(|b| b.spec_id == "wago-2004-1207")
                .count(),
            2
        );

        // 3. 4x URTK/SP -> direct catalog match phoenix-urtk-sp
        assert_eq!(plan.segments[2].blocks.len(), 4);
        assert!(plan.segments[2]
            .blocks
            .iter()
            .all(|b| b.spec_id == "phoenix-urtk-sp"));

        // 4. 2x Reserve leer -> blank_marker = true
        assert_eq!(plan.segments[3].blocks.len(), 2);
        assert!(plan.segments[3].blocks.iter().all(|b| b.blank_marker));
    }
}
