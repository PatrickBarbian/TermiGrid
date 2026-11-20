use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::catalog::{TerminalCatalog, TerminalSpec};
use crate::model::TerminalStrip;

static ID_COUNTER: AtomicU64 = AtomicU64::new(1);

fn default_library_version() -> u32 {
    1
}

/// Generates a unique persistent template identifier.
fn generate_template_id() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    let seq = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("tpl-{ts:x}-{seq}")
}

/// A reusable, user-named terminal strip template stored in the personal library.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StripTemplate {
    /// Unique identifier within the personal library.
    pub id: String,
    /// User-defined template name (e.g., "Standard Einspeisung 24V DC").
    pub name: String,
    /// Optional descriptive notes / search keywords.
    #[serde(default)]
    pub description: String,
    /// Preconfigured terminal strip (blocks, numbering modes, potentials, end brackets).
    pub strip: TerminalStrip,
    /// Any user-defined custom terminal specs referenced by `strip` for project portability.
    #[serde(default)]
    pub custom_specs: Vec<TerminalSpec>,
}

impl StripTemplate {
    /// Summarizes the unique manufacturers present in this template's blocks.
    pub fn manufacturers_summary(&self) -> Vec<String> {
        let mut set = BTreeSet::new();
        for b in &self.strip.blocks {
            set.insert(b.manufacturer.clone());
        }
        set.into_iter().collect()
    }

    /// Produces a compact grouped summary of terminal types (e.g. `"8× 2002-1201, 2× 2002-1207"`).
    pub fn block_type_summary(&self, max_entries: usize) -> String {
        let mut counts: Vec<(String, usize)> = Vec::new();
        for b in &self.strip.blocks {
            let short = b.short_type_name().to_string();
            if let Some((_, cnt)) = counts.iter_mut().find(|(n, _)| n == &short) {
                *cnt += 1;
            } else {
                counts.push((short, 1));
            }
        }
        let total_groups = counts.len();
        let shown: Vec<String> = counts
            .into_iter()
            .take(max_entries)
            .map(|(name, cnt)| format!("{cnt}× {name}"))
            .collect();
        if total_groups > max_entries {
            format!("{} … (+{})", shown.join(", "), total_groups - max_entries)
        } else {
            shown.join(", ")
        }
    }
}

/// Persistent personal library of reusable terminal strip templates (`strip_library.json`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StripLibrary {
    #[serde(default = "default_library_version")]
    pub format_version: u32,
    #[serde(default)]
    pub templates: Vec<StripTemplate>,
}

impl Default for StripLibrary {
    fn default() -> Self {
        Self {
            format_version: default_library_version(),
            templates: Vec::new(),
        }
    }
}

/// Resolves the cross-platform persistent storage path for `strip_library.json`.
pub fn default_library_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA")
            && !appdata.trim().is_empty()
        {
            return PathBuf::from(appdata)
                .join("TermiGrid")
                .join("strip_library.json");
        }
        if let Ok(local) = std::env::var("LOCALAPPDATA")
            && !local.trim().is_empty()
        {
            return PathBuf::from(local)
                .join("TermiGrid")
                .join("strip_library.json");
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME")
            && !home.trim().is_empty()
        {
            return PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("TermiGrid")
                .join("strip_library.json");
        }
    }

    #[cfg(target_os = "android")]
    {
        let pkg_files =
            PathBuf::from("/data/user/0/com.patrickbarbian.termigrid/files/strip_library.json");
        if let Some(parent) = pkg_files.parent()
            && (parent.exists() || std::fs::create_dir_all(parent).is_ok())
        {
            return pkg_files;
        }
    }

    #[cfg(all(unix, not(target_os = "macos"), not(target_os = "android")))]
    {
        if let Ok(xdg) = std::env::var("XDG_DATA_HOME")
            && !xdg.trim().is_empty()
        {
            return PathBuf::from(xdg)
                .join("TermiGrid")
                .join("strip_library.json");
        }
        if let Ok(home) = std::env::var("HOME")
            && !home.trim().is_empty()
        {
            return PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("TermiGrid")
                .join("strip_library.json");
        }
    }

    std::env::temp_dir()
        .join("TermiGrid")
        .join("strip_library.json")
}

impl StripLibrary {
    /// Loads the personal strip library from the default OS data path (returns empty library if missing).
    pub fn load_default() -> Self {
        let path = default_library_path();
        Self::load_from_path(&path).unwrap_or_default()
    }

    /// Reloads the personal strip library from the default OS data path if the file exists,
    /// keeping all open projects/windows on the same PC synchronized.
    pub fn refresh_from_default(&mut self) {
        let path = default_library_path();
        if path.exists()
            && let Ok(disk_lib) = Self::load_from_path(&path)
        {
            *self = disk_lib;
        }
    }

    /// Saves the personal strip library to the default OS data path.
    pub fn save_default(&self) -> Result<PathBuf, String> {
        let path = default_library_path();
        self.save_to_path(&path)?;
        Ok(path)
    }

    /// Loads and normalizes a `StripLibrary` JSON file from `path`.
    pub fn load_from_path(path: &Path) -> Result<Self, String> {
        let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut lib: Self = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
        lib.normalize();
        Ok(lib)
    }

    /// Serializes and writes the `StripLibrary` as pretty JSON to `path`, creating parent folders as needed.
    pub fn save_to_path(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }

    /// Normalizes all stored strip templates (`sync_next_block_id` & `recalculate_labels`).
    pub fn normalize(&mut self) {
        for tpl in &mut self.templates {
            tpl.strip.sync_next_block_id();
            tpl.strip.recalculate_labels();
        }
    }

    /// Saves `strip` into the personal library as a named template (creating a new entry or
    /// overwriting `overwrite_id` if provided and present). Returns the saved template ID.
    pub fn save_template(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        strip: &TerminalStrip,
        project_custom_specs: &[TerminalSpec],
        overwrite_id: Option<&str>,
    ) -> String {
        let raw_name = name.into();
        let clean_name = if raw_name.trim().is_empty() {
            strip.name.trim().to_string()
        } else {
            raw_name.trim().to_string()
        };
        let clean_name = if clean_name.is_empty() {
            "Vorlage".to_string()
        } else {
            clean_name
        };
        let clean_desc = description.into().trim().to_string();

        let mut cloned_strip = strip.clone();
        cloned_strip.sync_next_block_id();
        cloned_strip.recalculate_labels();

        // Preserve any custom specs used by this strip so it works across any project.
        let mut used_spec_ids: BTreeSet<String> = cloned_strip
            .blocks
            .iter()
            .map(|b| b.spec_id.clone())
            .collect();
        used_spec_ids.insert(cloned_strip.default_spec_id.clone());
        let custom_specs: Vec<TerminalSpec> = project_custom_specs
            .iter()
            .filter(|sp| used_spec_ids.contains(&sp.id))
            .cloned()
            .collect();

        if let Some(target_id) = overwrite_id
            && let Some(existing) = self.templates.iter_mut().find(|t| t.id == target_id)
        {
            existing.name = clean_name;
            existing.description = clean_desc;
            existing.strip = cloned_strip;
            existing.custom_specs = custom_specs;
            return existing.id.clone();
        }

        let mut id = generate_template_id();
        while self.templates.iter().any(|t| t.id == id) {
            id = generate_template_id();
        }

        // Insert newest templates at the front for quick access.
        self.templates.insert(
            0,
            StripTemplate {
                id: id.clone(),
                name: clean_name,
                description: clean_desc,
                strip: cloned_strip,
                custom_specs,
            },
        );
        id
    }

    /// Renames an existing template and updates its description.
    pub fn rename_template(
        &mut self,
        id: &str,
        new_name: impl Into<String>,
        new_description: impl Into<String>,
    ) -> bool {
        let name = new_name.into();
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return false;
        }
        let Some(tpl) = self.templates.iter_mut().find(|t| t.id == id) else {
            return false;
        };
        tpl.name = trimmed.to_string();
        tpl.description = new_description.into().trim().to_string();
        true
    }

    /// Deletes a template by ID. Returns `true` if a template was removed.
    pub fn delete_template(&mut self, id: &str) -> bool {
        let before = self.templates.len();
        self.templates.retain(|t| t.id != id);
        self.templates.len() != before
    }

    /// Finds a template by ID.
    pub fn find_by_id(&self, id: &str) -> Option<&StripTemplate> {
        self.templates.iter().find(|t| t.id == id)
    }

    /// Finds the first template whose name matches `name` (case-insensitive).
    pub fn find_by_name_ignore_case(&self, name: &str) -> Option<&StripTemplate> {
        let target = name.trim();
        if target.is_empty() {
            return None;
        }
        self.templates
            .iter()
            .find(|t| t.name.eq_ignore_ascii_case(target))
    }

    /// Filters templates using case-insensitive whitespace-separated search tokens across
    /// template name, description, strip name/description, terminal types, manufacturers,
    /// article numbers, and bridge potential names.
    pub fn filter<'a>(&'a self, query: &str, catalog: &TerminalCatalog) -> Vec<&'a StripTemplate> {
        let tokens: Vec<String> = query
            .split_whitespace()
            .map(|t| t.trim().to_ascii_lowercase())
            .filter(|t| !t.is_empty())
            .collect();
        if tokens.is_empty() {
            return self.templates.iter().collect();
        }

        self.templates
            .iter()
            .filter(|tpl| {
                tokens.iter().all(|tok| template_matches_token(tpl, tok, catalog))
            })
            .collect()
    }
}

fn template_matches_token(
    tpl: &StripTemplate,
    tok: &str,
    catalog: &TerminalCatalog,
) -> bool {
    if tpl.name.to_ascii_lowercase().contains(tok)
        || tpl.description.to_ascii_lowercase().contains(tok)
        || tpl.strip.name.to_ascii_lowercase().contains(tok)
        || tpl.strip.description.to_ascii_lowercase().contains(tok)
    {
        return true;
    }

    if tpl
        .strip
        .potentials
        .iter()
        .any(|p| p.name.to_ascii_lowercase().contains(tok))
    {
        return true;
    }

    tpl.strip.blocks.iter().any(|b| {
        if b.spec_id.to_ascii_lowercase().contains(tok)
            || b.spec_name.to_ascii_lowercase().contains(tok)
            || b.manufacturer.to_ascii_lowercase().contains(tok)
        {
            return true;
        }
        catalog
            .find_by_id(&b.spec_id)
            .and_then(|spec| spec.effective_article_number())
            .is_some_and(|art| art.to_ascii_lowercase().contains(tok))
    })
}

/// Instantiates a fresh `TerminalStrip` from `template`, guaranteeing a non-colliding strip
/// designation against `existing_strips` unless an explicit `target_designation` is provided.
pub fn instantiate_strip_from_template(
    template: &StripTemplate,
    target_designation: Option<&str>,
    existing_strips: &[TerminalStrip],
) -> TerminalStrip {
    let mut strip = template.strip.clone();

    if let Some(custom_name) = target_designation.map(str::trim).filter(|s| !s.is_empty()) {
        strip.name = custom_name.to_string();
    } else {
        let collides = existing_strips
            .iter()
            .any(|s| s.name.eq_ignore_ascii_case(strip.name.trim()));
        if collides || strip.name.trim().is_empty() {
            strip.name = next_available_strip_name(&strip.name, existing_strips);
        }
    }

    strip.sync_next_block_id();
    strip.recalculate_labels();
    strip.end_plates_generated = true;
    strip
}

/// Merges any custom terminal specs carried by `template` into `project_custom_specs` without
/// duplicating existing IDs. Returns `true` if at least one new custom spec was added.
pub fn merge_template_custom_specs(
    template: &StripTemplate,
    project_custom_specs: &mut Vec<TerminalSpec>,
) -> bool {
    merge_custom_specs_into(&template.custom_specs, project_custom_specs)
}

/// Merges `source` custom specs into `target` without duplicating IDs.
/// Returns `true` if at least one new custom spec was added.
pub fn merge_custom_specs_into(source: &[TerminalSpec], target: &mut Vec<TerminalSpec>) -> bool {
    let mut added = false;
    for sp in source {
        if !target.iter().any(|existing| existing.id == sp.id) {
            target.push(sp.clone());
            added = true;
        }
    }
    added
}

/// Inserts or replaces a custom/overridden terminal specification by ID.
pub fn upsert_custom_spec(target: &mut Vec<TerminalSpec>, spec: TerminalSpec) {
    if let Some(existing) = target.iter_mut().find(|s| s.id == spec.id) {
        *existing = spec;
    } else {
        target.push(spec);
    }
}

/// Removes a custom/overridden terminal specification by ID. Returns `true` if removed.
pub fn remove_custom_spec(target: &mut Vec<TerminalSpec>, id: &str) -> bool {
    let before = target.len();
    target.retain(|s| s.id != id);
    target.len() != before
}

/// Resolves the cross-platform persistent storage path for PC-wide custom terminal specs (`custom_specs.json`).
pub fn default_custom_specs_path() -> PathBuf {
    default_library_path().with_file_name("custom_specs.json")
}

/// Loads PC-wide custom terminal specs from `path` (returns empty Vec if missing/invalid).
pub fn load_global_custom_specs_from(path: &Path) -> Vec<TerminalSpec> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str::<Vec<TerminalSpec>>(&raw).ok())
        .unwrap_or_default()
}

/// Loads PC-wide custom terminal specs from the default OS data folder.
pub fn load_global_custom_specs() -> Vec<TerminalSpec> {
    load_global_custom_specs_from(&default_custom_specs_path())
}

/// Saves PC-wide custom terminal specs to `path`, creating parent folders as needed.
pub fn save_global_custom_specs_to(path: &Path, specs: &[TerminalSpec]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(specs).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

/// Saves PC-wide custom terminal specs to the default OS data folder.
pub fn save_global_custom_specs(specs: &[TerminalSpec]) -> Result<PathBuf, String> {
    let path = default_custom_specs_path();
    save_global_custom_specs_to(&path, specs)?;
    Ok(path)
}

fn next_available_strip_name(base_name: &str, existing_strips: &[TerminalStrip]) -> String {
    let trimmed = base_name.trim();
    if trimmed.is_empty() || trimmed.starts_with("-X") {
        let mut idx = 1usize;
        loop {
            let candidate = format!("-X{idx}");
            if !existing_strips
                .iter()
                .any(|s| s.name.eq_ignore_ascii_case(&candidate))
            {
                return candidate;
            }
            idx += 1;
        }
    } else {
        let mut idx = 2usize;
        loop {
            let candidate = format!("{trimmed} ({idx})");
            if !existing_strips
                .iter()
                .any(|s| s.name.eq_ignore_ascii_case(&candidate))
            {
                return candidate;
            }
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_save_load_rename_search_delete_and_instantiate() {
        let catalog = TerminalCatalog::load_embedded();
        let spec_feed = catalog.find_by_id("wago-2002-1201").unwrap();
        let spec_pe = catalog.find_by_id("wago-2002-1207").unwrap();

        let mut strip = TerminalStrip::generate_with_block_count("-X1", 1, 10, 4, spec_feed);
        strip.description = "24V Steuerspannung".to_string();
        strip.append_blocks(1, spec_pe);
        strip.add_potential("+24V", [239, 68, 68]);

        let mut lib = StripLibrary::default();
        let tpl_id = lib.save_template(
            "Einspeisung 24V",
            "Standard 5-fach Leiste für Schaltschrank",
            &strip,
            &[],
            None,
        );
        assert_eq!(lib.templates.len(), 1);

        // Save / Load roundtrip via temp file
        let tmp_dir = std::env::temp_dir().join(format!(
            "termigrid_lib_test_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let tmp_file = tmp_dir.join("strip_library.json");
        lib.save_to_path(&tmp_file).expect("save library");
        let loaded = StripLibrary::load_from_path(&tmp_file).expect("load library");
        assert_eq!(loaded.templates.len(), 1);
        assert_eq!(loaded.templates[0].name, "Einspeisung 24V");
        assert_eq!(loaded.templates[0].strip.blocks.len(), 5);
        let _ = std::fs::remove_dir_all(&tmp_dir);

        // Search by template name, manufacturer, article number, potential name
        assert_eq!(loaded.filter("24V", &catalog).len(), 1);
        assert_eq!(loaded.filter("WAGO 2002-1207", &catalog).len(), 1);
        assert_eq!(loaded.filter("Phoenix", &catalog).len(), 0);

        // Instantiate avoids name collision with existing -X1
        let existing = vec![strip.clone()];
        let inserted = instantiate_strip_from_template(&loaded.templates[0], None, &existing);
        assert_eq!(inserted.name, "-X2");
        assert_eq!(inserted.blocks.len(), 5);

        let explicit =
            instantiate_strip_from_template(&loaded.templates[0], Some("-X99"), &existing);
        assert_eq!(explicit.name, "-X99");

        // Rename & Delete
        let mut mutable = loaded;
        assert!(mutable.rename_template(&tpl_id, "24V Steuerung Neu", "Aktualisiert"));
        assert_eq!(mutable.find_by_id(&tpl_id).unwrap().name, "24V Steuerung Neu");
        assert!(mutable.delete_template(&tpl_id));
        assert!(mutable.templates.is_empty());
    }
}
