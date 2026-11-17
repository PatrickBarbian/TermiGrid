use std::sync::{Arc, Mutex};

use crate::ui::net::http_get_text;

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GITHUB_REPO_URL: &str = "https://github.com/PatrickBarbian/TermiGrid";
pub const GITHUB_RELEASES_URL: &str = "https://github.com/PatrickBarbian/TermiGrid/releases";
pub const BUYMEACOFFEE_URL: &str = "https://buymeacoffee.com/02eOWKzm5W";
const LATEST_RELEASE_API: &str =
    "https://api.github.com/repos/PatrickBarbian/TermiGrid/releases/latest";

/// Result of an asynchronous GitHub Release update check.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum UpdateStatus {
    #[default]
    Idle,
    Checking,
    UpToDate(String),
    UpdateAvailable {
        latest_tag: String,
        release_url: String,
    },
    Error(String),
}

/// Thread-safe handle for background GitHub update checks.
#[derive(Clone, Default)]
pub struct UpdateChecker {
    status: Arc<Mutex<UpdateStatus>>,
}

impl UpdateChecker {
    pub fn status(&self) -> UpdateStatus {
        self.status
            .lock()
            .map(|g| g.clone())
            .unwrap_or(UpdateStatus::Idle)
    }

    /// Triggers a non-blocking update check against GitHub Releases.
    pub fn check_async(&self, ctx: egui::Context) {
        if let Ok(mut guard) = self.status.lock() {
            if matches!(*guard, UpdateStatus::Checking) {
                return;
            }
            *guard = UpdateStatus::Checking;
        }

        let shared = Arc::clone(&self.status);
        std::thread::spawn(move || {
            let result = perform_update_check();
            if let Ok(mut guard) = shared.lock() {
                *guard = result;
            }
            ctx.request_repaint();
        });
    }
}

/// Opens a web URL in the system browser.
pub fn open_external_url(ctx: &egui::Context, url: &str) {
    ctx.open_url(egui::OpenUrl::new_tab(url));
}

fn perform_update_check() -> UpdateStatus {
    let body = match http_get_text(LATEST_RELEASE_API) {
        Ok(text) => text,
        Err(e) => return UpdateStatus::Error(e),
    };

    let json: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(e) => return UpdateStatus::Error(e.to_string()),
    };

    let tag = json
        .get("tag_name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let html_url = json
        .get("html_url")
        .and_then(|v| v.as_str())
        .unwrap_or(GITHUB_RELEASES_URL)
        .to_string();

    if tag.is_empty() {
        return UpdateStatus::Error("Empty release tag".to_string());
    }

    if is_newer_version(CURRENT_VERSION, &tag) {
        UpdateStatus::UpdateAvailable {
            latest_tag: tag,
            release_url: html_url,
        }
    } else {
        UpdateStatus::UpToDate(tag)
    }
}

/// Compares semantic versions like `"1.0.0"` and `"v1.0.1"`.
pub fn is_newer_version(current: &str, latest_tag: &str) -> bool {
    let parse = |s: &str| -> Option<(u32, u32, u32)> {
        let clean = s.trim().trim_start_matches(['v', 'V']);
        let core = clean.split('-').next()?;
        let mut parts = core.split('.');
        let maj = parts.next()?.parse().ok()?;
        let min = parts.next().unwrap_or("0").parse().ok()?;
        let pat = parts.next().unwrap_or("0").parse().ok()?;
        Some((maj, min, pat))
    };

    match (parse(current), parse(latest_tag)) {
        (Some(c), Some(l)) => l > c,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_newer_version;

    #[test]
    fn semver_comparison_detects_newer_tags() {
        assert!(is_newer_version("1.0.0", "v1.0.1"));
        assert!(is_newer_version("1.0.0", "v1.1.0"));
        assert!(is_newer_version("1.0.0", "2.0.0"));
        assert!(!is_newer_version("1.0.0", "v1.0.0"));
        assert!(!is_newer_version("1.1.0", "v1.0.9"));
    }
}
