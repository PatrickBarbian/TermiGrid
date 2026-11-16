#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use termigrid_core::load_app_icon;
use termigrid_core::ui::TermiGridApp;

#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result {
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("TermiGrid – Reihenklemmen-Konfigurator")
        .with_inner_size([1360.0, 840.0])
        .with_min_inner_size([360.0, 560.0]);

    if let Some(icon) = load_app_icon() {
        viewport = viewport.with_icon(std::sync::Arc::new(icon));
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "TermiGrid",
        options,
        Box::new(|cc| Ok(Box::new(TermiGridApp::new(cc)))),
    )
}

#[cfg(target_os = "android")]
fn main() {}
