fn main() {
    #[cfg(windows)]
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "TermiGrid");
        res.set(
            "FileDescription",
            "TermiGrid – Reihenklemmen-Konfigurator für Schaltschrankbau",
        );
        res.set("LegalCopyright", "Copyright © 2026 Patrick Barbian");
        if let Err(err) = res.compile() {
            println!("cargo:warning=winresource skipped: {err}");
        }
    }
}
