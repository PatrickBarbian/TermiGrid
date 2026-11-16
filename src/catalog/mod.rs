pub mod accessories;

use serde::{Deserialize, Serialize};

use crate::catalog::accessories::{resolve_accessories, TerminalAccessories};
use crate::model::{ConnectionPoint, NumberingOrder, TerminalBlock, TierBehavior, TierColor};

/// Embedded hardcoded database of common WAGO, Phoenix Contact, and Weidmüller terminal blocks.
const EMBEDDED_SPECS_JSON: &str = include_str!("../../assets/terminal_specs.json");

/// Blueprint for a single tier within a catalog specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TierSpec {
    pub behavior: TierBehavior,
    pub color: TierColor,
}

/// Technical specification of a terminal block type from the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalSpec {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub series: String,
    pub levels: usize,
    pub width_tenths_mm: u16,
    pub numbering_order: NumberingOrder,
    pub description_de: String,
    pub description_en: String,
    pub tiers: Vec<TierSpec>,
    #[serde(default)]
    pub accessories: Option<TerminalAccessories>,
}

impl TerminalSpec {
    /// Instantiates a new `TerminalBlock` with the given ID from this specification.
    pub fn instantiate_block(&self, block_id: u64) -> TerminalBlock {
        let tiers = self
            .tiers
            .iter()
            .enumerate()
            .map(|(idx, ts)| ConnectionPoint::new(idx, ts.behavior.clone(), ts.color))
            .collect();

        TerminalBlock {
            id: block_id,
            spec_id: self.id.clone(),
            spec_name: self.name.clone(),
            manufacturer: self.manufacturer.clone(),
            width_tenths_mm: self.width_tenths_mm,
            numbering_order: self.numbering_order,
            tiers,
        }
    }

    /// Counts how many tiers in this spec consume a sequential number.
    pub fn sequential_tier_count(&self) -> usize {
        self.tiers
            .iter()
            .filter(|t| t.behavior.consumes_sequence())
            .count()
    }

    /// Returns a compact pattern summary such as "L / L", "PE / L / N", or "N".
    pub fn pattern_summary(&self) -> String {
        self.tiers
            .iter()
            .map(|t| match &t.behavior {
                TierBehavior::Sequential => "L".to_string(),
                TierBehavior::Fixed(s) => s.clone(),
                TierBehavior::ProtectiveEarth => "PE".to_string(),
                TierBehavior::Unlabeled => "—".to_string(),
            })
            .collect::<Vec<_>>()
            .join(" / ")
    }
}

/// In-memory catalog loaded from the embedded JSON terminal database.
#[derive(Debug, Clone)]
pub struct TerminalCatalog {
    specs: Vec<TerminalSpec>,
}

impl Default for TerminalCatalog {
    fn default() -> Self {
        Self::load_embedded()
    }
}

impl TerminalCatalog {
    /// Loads the embedded JSON terminal catalog.
    pub fn load_embedded() -> Self {
        let specs: Vec<TerminalSpec> = serde_json::from_str(EMBEDDED_SPECS_JSON)
            .expect("Embedded terminal_specs.json must be valid JSON");
        Self { specs }
    }

    /// Rebuilds the catalog from embedded specs plus project-specific custom specs.
    pub fn reload_with_custom(&mut self, custom_specs: &[TerminalSpec]) {
        *self = Self::load_embedded();
        for spec in custom_specs {
            self.add_custom_spec(spec.clone());
        }
    }

    /// Adds or replaces a custom terminal specification in the active catalog.
    pub fn add_custom_spec(&mut self, spec: TerminalSpec) {
        if let Some(existing) = self.specs.iter_mut().find(|s| s.id == spec.id) {
            *existing = spec;
        } else {
            self.specs.push(spec);
        }
    }

    /// Returns all available terminal specifications.
    pub fn all(&self) -> &[TerminalSpec] {
        &self.specs
    }

    /// Finds a specification by its unique ID.
    pub fn find_by_id(&self, id: &str) -> Option<&TerminalSpec> {
        self.specs.iter().find(|s| s.id == id)
    }

    /// Returns the distinct manufacturer names in catalog order.
    pub fn manufacturers(&self) -> Vec<&str> {
        let mut out = Vec::new();
        for spec in &self.specs {
            if !out.contains(&spec.manufacturer.as_str()) {
                out.push(spec.manufacturer.as_str());
            }
        }
        out
    }

    /// Resolves matching accessories (end plate, jumpers, markers, end bracket) for `spec`.
    pub fn accessories_for(&self, spec: &TerminalSpec) -> TerminalAccessories {
        spec.accessories
            .clone()
            .unwrap_or_else(|| resolve_accessories(spec))
    }

    /// Applies a new specification to an existing `TerminalBlock`, preserving its `id` and replacing its physical tiers.
    pub fn apply_spec_to_block(block: &mut TerminalBlock, spec: &TerminalSpec) {
        block.spec_id = spec.id.clone();
        block.spec_name = spec.name.clone();
        block.manufacturer = spec.manufacturer.clone();
        block.width_tenths_mm = spec.width_tenths_mm;
        block.numbering_order = spec.numbering_order;
        block.tiers = spec
            .tiers
            .iter()
            .enumerate()
            .map(|(idx, ts)| ConnectionPoint::new(idx, ts.behavior.clone(), ts.color))
            .collect();
    }
}
