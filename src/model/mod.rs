pub mod bridging;
pub mod numbering;

use serde::{Deserialize, Serialize};

use crate::catalog::TerminalSpec;

/// Defines how a physical tier (connection point) receives its label during strip numbering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TierBehavior {
    /// Consumes the next number from the strip's linear sequence (e.g., 1, 2, 3...).
    Sequential,
    /// Displays a fixed static string (e.g., "N", "+", "-", "L1") and does not advance the counter.
    Fixed(String),
    /// Protective Earth (PE): usually unlabeled for printing, does not advance the counter.
    ProtectiveEarth,
    /// Explicitly unlabeled / spare tier: empty print label, does not advance the counter.
    Unlabeled,
}

impl TierBehavior {
    /// Returns true if this tier consumes a sequential number when no custom override is active.
    pub fn consumes_sequence(&self) -> bool {
        matches!(self, Self::Sequential)
    }

    /// Returns true if this tier represents Protective Earth (PE).
    pub fn is_pe(&self) -> bool {
        matches!(self, Self::ProtectiveEarth)
    }

    /// Returns true if this tier is a standard Neutral ("N") fixed connection.
    pub fn is_neutral(&self) -> bool {
        matches!(self, Self::Fixed(s) if s.trim().eq_ignore_ascii_case("N"))
    }
}

/// Visual color hint for the 2D CAD representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TierColor {
    #[default]
    StandardGray,
    BlueNeutral,
    GreenYellowPe,
    RedPhase,
    OrangeAux,
}

/// Order in which sequential numbers are distributed across tiers of a single terminal block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum NumberingOrder {
    /// Standard industrial order: lower tier (level 0) first, then middle, then upper.
    #[default]
    BottomToTop,
    /// Reverse order: upper tier first, down to lower tier.
    TopToBottom,
}

/// Represents a single physical tier / connection level inside a terminal block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionPoint {
    /// 0-based physical tier index from bottom (0 = Lower/Unten, 1 = Middle/Upper, 2 = Upper/Oben).
    pub level_index: usize,
    /// Numbering behavior inherited from the terminal specification.
    pub behavior: TierBehavior,
    /// Optional manual override entered by the user; suppresses automatic sequential numbering.
    pub custom_override: Option<String>,
    /// Optional prefix applied before the base label (e.g., "+", "-", "X1:").
    #[serde(default)]
    pub label_prefix: String,
    /// Optional suffix applied after the base label (e.g., "~", ".1", "*").
    #[serde(default)]
    pub label_suffix: String,
    /// Visual color classification for CAD rendering.
    pub color: TierColor,
    /// Computed label intended for marker printing (empty string for unlabeled/PE).
    pub computed_label: String,
    /// Label rendered in the 2D CAD view (shows "PE" for ProtectiveEarth even when print label is empty).
    pub cad_display_label: String,
}

/// Type alias matching domain terminology ("Tier" / "ConnectionPoint").
pub type Tier = ConnectionPoint;

impl ConnectionPoint {
    /// Creates a new connection point with empty computed labels (populated by `recalculate_labels`).
    pub fn new(level_index: usize, behavior: TierBehavior, color: TierColor) -> Self {
        Self {
            level_index,
            behavior,
            custom_override: None,
            label_prefix: String::new(),
            label_suffix: String::new(),
            color,
            computed_label: String::new(),
            cad_display_label: String::new(),
        }
    }
}

/// Represents one physical terminal block mounted on the DIN rail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalBlock {
    /// Unique identifier within the strip/project.
    pub id: u64,
    /// Catalog specification identifier (e.g., "wago-2002-2201").
    pub spec_id: String,
    /// Display model name (e.g., "2002-2201 (2-Etagen L/L)").
    pub spec_name: String,
    /// Manufacturer name (e.g., "WAGO", "Phoenix Contact", "Weidmüller").
    pub manufacturer: String,
    /// Physical block width in tenths of a millimeter (e.g., 52 = 5.2 mm).
    pub width_tenths_mm: u16,
    /// Tier numbering direction for this block.
    pub numbering_order: NumberingOrder,
    /// Physical tiers ordered from bottom (index 0) to top (index len - 1).
    pub tiers: Vec<ConnectionPoint>,
}

impl TerminalBlock {
    /// Returns the number of physical levels (tiers) on this terminal block.
    pub fn level_count(&self) -> usize {
        self.tiers.len()
    }

    /// Returns how many tiers on this block consume a sequential number.
    pub fn sequential_tier_count(&self) -> usize {
        self.tiers
            .iter()
            .filter(|t| t.custom_override.is_none() && t.behavior.consumes_sequence())
            .count()
    }

    /// Computes the aligned visual/logical row index for tier `level_idx`.
    ///
    /// Row `0` is the main lower numbered baseline ("Ebene 1"), row `1` is "Ebene 2", etc.
    /// For single-tier PE blocks and hybrid blocks with a bottom PE tier (`PE/L`, `PE/L/N`, etc.),
    /// the bottom-most PE tier is placed at row `-1` ("PE") so numbered tiers above it stay
    /// horizontally aligned with feed-through blocks.
    /// For pure multi-level PE blocks (`PE/PE`, `PE/PE/PE` where all >= 2 tiers are `PE`),
    /// tiers stack upward cleanly at rows `0, 1, ...` aligned with 2/3-level terminal decks.
    pub fn tier_visual_row(&self, level_idx: usize) -> isize {
        let has_bottom_pe = self.tiers.first().is_some_and(|t| t.behavior.is_pe());
        let is_pure_multi_pe =
            self.tiers.len() >= 2 && self.tiers.iter().all(|t| t.behavior.is_pe());
        if has_bottom_pe && !is_pure_multi_pe {
            (level_idx as isize) - 1
        } else {
            level_idx as isize
        }
    }

    /// Returns a compact type label (stripping parenthetical suffixes like `" (2-Etagen L/L)"`).
    pub fn short_type_name(&self) -> &str {
        self.spec_name
            .split(" (")
            .next()
            .unwrap_or(&self.spec_name)
            .trim()
    }
}

/// Controls how Neutral (`N`) tiers are numbered along a terminal strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum NeutralLabelMode {
    /// Classic fixed `"N"` on every neutral tier.
    #[default]
    Fixed,
    /// Shares the strip's sequential counter, labeled as pure number (`"1"`, `"2"`, ...).
    SharedSequenceNumber,
    /// Shares the strip's sequential counter, formatted `<n>N` (e.g. `"3N"`).
    SharedSequenceSuffixN,
    /// Dedicated counter per strip starting at 1: `"1N"`, `"2N"`, `"3N"`...
    SeparateCounterSuffixN,
}

/// Controls how Protective Earth (`PE`) tiers are numbered along a terminal strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PeLabelMode {
    /// Classic `"PE"` in CAD (and unlabeled unless PE export is enabled).
    #[default]
    Fixed,
    /// Shares the strip's sequential counter, labeled as pure number (`"1"`, `"2"`, ...).
    SharedSequenceNumber,
    /// Shares the strip's sequential counter, formatted `<n>PE` (e.g. `"3PE"`).
    SharedSequenceSuffixPe,
    /// Dedicated counter per strip starting at 1: `"1PE"`, `"2PE"`, `"3PE"`...
    SeparateCounterSuffixPe,
}

/// Represents a named electrical potential whose assigned terminal tiers should be bridged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JumperPotential {
    pub id: u64,
    pub name: String,
    pub color_rgb: [u8; 3],
    /// Assigned connection points as `(block_id, level_index)`.
    #[serde(default)]
    pub members: Vec<(u64, usize)>,
}

fn default_next_potential_id() -> u64 {
    1
}

fn default_end_bracket_id() -> String {
    crate::catalog::accessories::DEFAULT_END_BRACKET_ID.to_string()
}

fn default_true() -> bool {
    true
}

/// Represents a complete DIN-rail terminal strip (e.g., "-X1").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalStrip {
    /// Strip designation (e.g., "-X1").
    pub name: String,
    /// Optional descriptive note (e.g., "Steuerspannung 24V DC").
    pub description: String,
    /// Starting number for the linear numbering sequence (e.g., 1).
    pub start_number: u32,
    /// Target end number entered upon strip creation (e.g., 50).
    pub target_end_number: u32,
    /// Default spec ID used when creating or appending blocks.
    pub default_spec_id: String,
    /// Ordered sequence of physical blocks from left to right.
    pub blocks: Vec<TerminalBlock>,
    /// Next unique block ID counter.
    next_block_id: u64,
    /// Numbering behavior for `N` tiers on this strip.
    #[serde(default)]
    pub neutral_mode: NeutralLabelMode,
    /// Numbering behavior for `PE` tiers on this strip.
    #[serde(default)]
    pub pe_mode: PeLabelMode,
    /// Named jumper potentials assigned in Bridge Mode.
    #[serde(default)]
    pub potentials: Vec<JumperPotential>,
    /// Next unique potential ID counter.
    #[serde(default = "default_next_potential_id")]
    next_potential_id: u64,
    /// Selected DIN-rail end bracket ID (`Endhalter`), defaulting to Phoenix Contact CLIPFIX 35-5.
    #[serde(default = "default_end_bracket_id")]
    pub end_bracket_id: String,
    /// Whether to mount an end bracket before the first terminal block (`vor #1`).
    #[serde(default = "default_true")]
    pub end_bracket_left: bool,
    /// Whether to mount an end bracket after the last terminal block (`nach #N`).
    #[serde(default = "default_true")]
    pub end_bracket_right: bool,
}

impl TerminalStrip {
    /// Creates an empty terminal strip with the given metadata.
    pub fn new(
        name: impl Into<String>,
        start_number: u32,
        target_end_number: u32,
        default_spec_id: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            start_number,
            target_end_number,
            default_spec_id: default_spec_id.into(),
            blocks: Vec::new(),
            next_block_id: 1,
            neutral_mode: NeutralLabelMode::Fixed,
            pe_mode: PeLabelMode::Fixed,
            potentials: Vec::new(),
            next_potential_id: 1,
            end_bracket_id: default_end_bracket_id(),
            end_bracket_left: true,
            end_bracket_right: true,
        }
    }

    /// Allocates a fresh unique block ID within this strip.
    pub fn allocate_block_id(&mut self) -> u64 {
        let id = self.next_block_id;
        self.next_block_id += 1;
        id
    }

    /// Allocates a fresh unique jumper potential ID within this strip.
    pub fn allocate_potential_id(&mut self) -> u64 {
        let id = self.next_potential_id.max(1);
        self.next_potential_id = id + 1;
        id
    }

    /// Ensures `next_block_id` and `next_potential_id` stay strictly greater than existing IDs.
    pub fn sync_next_block_id(&mut self) {
        let max_id = self.blocks.iter().map(|b| b.id).max().unwrap_or(0);
        if self.next_block_id <= max_id {
            self.next_block_id = max_id + 1;
        }
        let max_pot = self.potentials.iter().map(|p| p.id).max().unwrap_or(0);
        if self.next_potential_id <= max_pot {
            self.next_potential_id = max_pot + 1;
        }
    }

    /// Returns the total physical width of all terminal blocks in millimeters.
    pub fn total_width_mm(&self) -> f32 {
        self.blocks
            .iter()
            .map(|b| f32::from(b.width_tenths_mm) / 10.0)
            .sum()
    }

    /// Returns the number of configured DIN-rail end brackets (`0`, `1`, or `2`).
    pub fn end_bracket_count(&self) -> usize {
        if self.blocks.is_empty() {
            return 0;
        }
        usize::from(self.end_bracket_left) + usize::from(self.end_bracket_right)
    }

    /// Returns the total DIN-rail width occupied by the selected end brackets in millimeters.
    pub fn end_brackets_width_mm(&self) -> f32 {
        let count = self.end_bracket_count();
        if count == 0 {
            return 0.0;
        }
        let spec = crate::catalog::accessories::find_end_bracket(&self.end_bracket_id);
        (f32::from(spec.width_tenths_mm) / 10.0) * (count as f32)
    }

    /// Returns the total number of printable (non-empty) labels in this strip.
    pub fn printable_label_count(&self) -> usize {
        self.blocks
            .iter()
            .flat_map(|b| b.tiers.iter())
            .filter(|t| !t.computed_label.is_empty())
            .count()
    }
}

/// Top-level serializable TermiGrid project (.termi / .json).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    /// File schema version for forward compatibility.
    pub format_version: u32,
    /// Project name displayed in window title & exports.
    pub project_name: String,
    /// All terminal strips in the project.
    pub strips: Vec<TerminalStrip>,
    /// User-defined custom terminal specs stored with this project.
    #[serde(default)]
    pub custom_specs: Vec<TerminalSpec>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            format_version: 1,
            project_name: "Neues Projekt".to_string(),
            strips: Vec::new(),
            custom_specs: Vec::new(),
        }
    }
}

impl Project {
    /// Recalculates numbering across all strips in the project and syncs IDs.
    pub fn normalize(&mut self) {
        for strip in &mut self.strips {
            strip.sync_next_block_id();
            strip.recalculate_labels();
        }
    }
}
