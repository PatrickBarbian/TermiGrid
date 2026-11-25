use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::import::rules::CompiledImportRules;
use crate::model::library::default_library_path;

fn default_prefix_phoenix() -> String {
    "PXC.".to_string()
}

fn default_prefix_wago() -> String {
    "WAG.".to_string()
}

fn default_prefix_weidmuller() -> String {
    "WDM.".to_string()
}

fn default_csv_delimiter() -> char {
    ';'
}

fn default_true() -> bool {
    true
}

/// Persistent EPLAN P8 roundtrip interface settings stored in `config.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct EplanConfig {
    /// Vendor prefix used by EPLAN for Phoenix Contact article numbers (default `"PXC."`).
    #[serde(default = "default_prefix_phoenix")]
    pub prefix_phoenix: String,
    /// Vendor prefix used by EPLAN for WAGO article numbers (default `"WAG."`).
    #[serde(default = "default_prefix_wago")]
    pub prefix_wago: String,
    /// Vendor prefix used by EPLAN for Weidmüller article numbers (default `"WDM."`).
    #[serde(default = "default_prefix_weidmuller")]
    pub prefix_weidmuller: String,
    /// Optional default output directory for EPLAN return-import packages.
    /// When `None`, defaults to the parent folder of the last imported EPLAN CSV (or prompts via picker).
    pub default_export_dir: Option<PathBuf>,
    /// Delimiter used when parsing and exporting EPLAN CSV files (default `';'`).
    #[serde(default = "default_csv_delimiter")]
    pub csv_delimiter: char,
    /// When `true` (default), `_EPLAN_Zubehoer_Import.*` exports only newly added/calculated
    /// accessories (end plates, partition plates, end brackets) minus pre-existing EPLAN items.
    #[serde(default = "default_true")]
    pub export_added_accessories_only: bool,
}

impl Default for EplanConfig {
    fn default() -> Self {
        Self {
            prefix_phoenix: default_prefix_phoenix(),
            prefix_wago: default_prefix_wago(),
            prefix_weidmuller: default_prefix_weidmuller(),
            default_export_dir: None,
            csv_delimiter: default_csv_delimiter(),
            export_added_accessories_only: true,
        }
    }
}

/// Top-level persistent application configuration persisted in `config.json`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub eplan: EplanConfig,
}

/// Resolves the cross-platform persistent storage path for `config.json` beside `strip_library.json`.
pub fn default_config_path() -> PathBuf {
    default_library_path().with_file_name("config.json")
}

impl AppConfig {
    /// Loads `config.json` from the OS data directory (falling back to defaults if missing/invalid).
    pub fn load_default() -> Self {
        Self::load_from_path(&default_config_path()).unwrap_or_default()
    }

    /// Saves `config.json` to the OS data directory.
    pub fn save_default(&self) -> Result<PathBuf, String> {
        let path = default_config_path();
        self.save_to_path(&path)?;
        Ok(path)
    }

    /// Loads `AppConfig` from a specific file path.
    pub fn load_from_path(path: &Path) -> Result<Self, String> {
        let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut cfg: Self = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
        cfg.eplan.sanitize();
        Ok(cfg)
    }

    /// Serializes and writes `AppConfig` as pretty JSON to `path`, creating parent folders as needed.
    pub fn save_to_path(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut copy = self.clone();
        copy.eplan.sanitize();
        let json = serde_json::to_string_pretty(&copy).map_err(|e| e.to_string())?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }
}

impl EplanConfig {
    /// Ensures delimiter and prefix strings are clean after load/edit.
    pub fn sanitize(&mut self) {
        if self.csv_delimiter == '\0' || self.csv_delimiter == '\n' || self.csv_delimiter == '\r' {
            self.csv_delimiter = ';';
        }
        self.prefix_phoenix = self.prefix_phoenix.trim().to_string();
        self.prefix_wago = self.prefix_wago.trim().to_string();
        self.prefix_weidmuller = self.prefix_weidmuller.trim().to_string();
    }

    /// Returns the configured vendor prefix for `manufacturer`, if it is one of our 3 supported brands.
    pub fn prefix_for_manufacturer(&self, manufacturer: &str) -> Option<&str> {
        let m = manufacturer.trim().to_lowercase();
        if m.contains("phoenix") || m == "pxc" || m == "phx" {
            Some(self.prefix_phoenix.as_str())
        } else if m.contains("wago") || m == "wag" {
            Some(self.prefix_wago.as_str())
        } else if m.contains("weidm") || m == "wdm" || m == "wei" || m == "wmi" {
            Some(self.prefix_weidmuller.as_str())
        } else {
            None
        }
    }

    /// Strips configured (or well-known EPLAN) vendor prefixes from `raw_article`, returning
    /// `(detected_canonical_manufacturer, clean_article_number)`.
    ///
    /// Examples (with default prefixes):
    /// - `"PXC.3210567"` -> `(Some("Phoenix Contact"), "3210567")`
    /// - `"WAG.2002-3217"` -> `(Some("WAGO"), "2002-3217")`
    /// - `"WDM.1050000000"` -> `(Some("Weidmüller"), "1050000000")`
    pub fn strip_vendor_prefix(&self, raw_article: &str) -> (Option<&'static str>, String) {
        let trimmed = raw_article.trim();
        if trimmed.is_empty() {
            return (None, String::new());
        }

        let configured = [
            ("Phoenix Contact", self.prefix_phoenix.trim()),
            ("WAGO", self.prefix_wago.trim()),
            ("Weidmüller", self.prefix_weidmuller.trim()),
        ];

        for (canonical_mfg, pref) in configured {
            if pref.is_empty() {
                continue;
            }
            if let Some(rest) = strip_prefix_ignore_ascii_case(trimmed, pref) {
                let clean = rest.trim();
                if !clean.is_empty() {
                    return (Some(canonical_mfg), clean.to_string());
                }
            }
            // Also tolerate prefix without trailing dot/colon if user omitted it in config
            if !pref.ends_with('.') && !pref.ends_with(':') {
                for sep in ['.', ':'] {
                    let with_sep = format!("{pref}{sep}");
                    if let Some(rest) = strip_prefix_ignore_ascii_case(trimmed, &with_sep) {
                        let clean = rest.trim();
                        if !clean.is_empty() {
                            return (Some(canonical_mfg), clean.to_string());
                        }
                    }
                }
            }
        }

        // Standard fallback vendor prefixes (e.g. WAGO.2002-1201, WEI.1050000000, PXC:3210567)
        let (prefix_opt, clean) = CompiledImportRules::split_article_prefix(trimmed);
        if let Some(prefix) = prefix_opt {
            let mfg = match prefix.as_str() {
                "PXC" | "PHX" | "PHO" | "PHOENIX" | "PHOENIXCONTACT" | "PHOENIX CONTACT" => {
                    Some("Phoenix Contact")
                }
                "WAG" | "WAGO" => Some("WAGO"),
                "WDM" | "WEI" | "WMI" | "WM" | "WEID" | "WEIDMULLER" | "WEIDMUELLER"
                | "WEIDMÜLLER" => Some("Weidmüller"),
                _ => None,
            };
            return (mfg, clean);
        }

        (None, trimmed.to_string())
    }

    /// Re-attaches the configured EPLAN vendor prefix for `manufacturer` to `article`
    /// without double-prefixing (e.g., `("Phoenix Contact", "3212044")` -> `"PXC.3212044"`).
    pub fn attach_vendor_prefix(&self, manufacturer: &str, article: &str) -> String {
        let trimmed = article.trim();
        if trimmed.is_empty() || trimmed == "-" {
            return trimmed.to_string();
        }
        let Some(prefix) = self.prefix_for_manufacturer(manufacturer) else {
            return trimmed.to_string();
        };
        let pref = prefix.trim();
        if pref.is_empty() {
            return trimmed.to_string();
        }
        if strip_prefix_ignore_ascii_case(trimmed, pref).is_some() {
            return trimmed.to_string();
        }
        // Strip any prior vendor prefix from the same brand before re-attaching
        let (detected, clean) = self.strip_vendor_prefix(trimmed);
        let base = if detected.is_some_and(|m| m.eq_ignore_ascii_case(manufacturer)) {
            clean.as_str()
        } else {
            trimmed
        };
        format!("{pref}{base}")
    }
}

/// Normalizes an article number to lowercase ASCII alphanumeric for duplicate/pre-existing tracking.
pub fn normalize_article_key(article: &str) -> String {
    article
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn strip_prefix_ignore_ascii_case<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    let s_bytes = s.as_bytes();
    let p_bytes = prefix.as_bytes();
    if s_bytes.len() < p_bytes.len() {
        return None;
    }
    if s_bytes[..p_bytes.len()].eq_ignore_ascii_case(p_bytes) {
        // Ensure we split on a UTF-8 char boundary
        let split_idx = prefix.len();
        s.get(split_idx..)
    } else {
        None
    }
}
