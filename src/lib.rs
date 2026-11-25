pub mod catalog;
pub mod eplan;
pub mod export;
pub mod i18n;
pub mod import;
pub mod model;
pub mod prompt;
pub mod ui;

/// Loads the embedded TermiGrid application window icon.
pub fn load_app_icon() -> Option<egui::IconData> {
    let bytes = include_bytes!("../assets/icon.png");
    let img = image::load_from_memory(bytes).ok()?.to_rgba8();
    let (width, height) = img.dimensions();
    Some(egui::IconData {
        rgba: img.into_raw(),
        width,
        height,
    })
}

/// Android entry point invoked by `NativeActivity` (`cargo-apk`).
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: winit::platform::android::activity::AndroidApp) {
    ui::android_insets::init_android_refs(app.vm_as_ptr(), app.activity_as_ptr());
    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "TermiGrid",
        options,
        Box::new(|cc| Ok(Box::new(ui::TermiGridApp::new(cc)))),
    );
}
