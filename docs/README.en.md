<div align="center">

<img src="../assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**Terminal Block Strip Configurator & Label Exporter for Control Cabinet Engineering**

[![Release](https://img.shields.io/github/v/release/PatrickBarbian/TermiGrid)](https://github.com/PatrickBarbian/TermiGrid/releases)
[![VibeCoded](https://img.shields.io/badge/VibeCoded-AI%20Assisted-blueviolet)](#overview--vibecoded-note)
[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-support-orange?logo=buy-me-a-coffee)](https://buymeacoffee.com/02eOWKzm5W)

<p>
  <a href="../README.md">🇩🇪 Deutsch</a> ·
  <strong>🇬🇧 English</strong> ·
  <a href="README.fr.md">🇫🇷 Français</a> ·
  <a href="README.it.md">🇮🇹 Italiano</a> ·
  <a href="README.es.md">🇪🇸 Español</a> ·
  <a href="README.pl.md">🇵🇱 Polski</a> ·
  <a href="README.nl.md">🇳🇱 Nederlands</a>
</p>

</div>

---

## Overview & VibeCoded Note

**TermiGrid** is a cross-platform tool (written in Rust with `egui` / `eframe 0.37`) for planning DIN-rail terminal strips in control cabinet engineering and exporting printer-ready label lists for industrial marking systems.

> **✨ VibeCoded:** This project is **VibeCoded** – designed and built with **AI assistance** in close collaboration with the project author.

---

## Key Features

- **186 built-in terminal block specs + custom blocks:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (push-button), `2102–2116`, `2022` X-COM, and `280/281`
  - **Phoenix Contact:** `PT 1,5/S`, `PT 2,5–35`, `PTPOWER`, `PTTB`, `PTTBS`, `PTV`, `PTI` installation terminals, plus `ST` & `UT`
  - **Weidmüller:** `A-Series`, `AITB` installation terminals, `W-Series`, `DLI`, & `Z-Series`
  - **Custom terminals:** Create and manage your own custom terminal definitions directly inside the app
- **Multi-level numbering & PE alignment:**
  - Installation and multi-tier terminals (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, etc.): Any existing **PE connection** is consistently aligned in a dedicated PE row **below the bottom main tier**, keeping sequential numbers on one clean horizontal baseline.
  - **Multi-tier example:** On a strip of 2-tier blocks, Tier 1 (bottom) automatically numbers odd (`1, 3, 5 …`) and Tier 2 (top) even (`2, 4, 6 …`); subsequent 1-tier blocks seamlessly continue on Tier 1 (`7, 8, 9 …`).
- **Interactive 2D CAD Editor (without DIN-rail clutter):**
  - Clean CAD canvas without an unnecessary DIN rail, featuring embedded manufacturer logos (**WAGO**, **Phoenix Contact**, **Weidmüller**).
  - **Type display:** Switch between **vertical (`-90°` rotated)** per terminal or **horizontally grouped** underneath the strip.
  - **Inline editing:** **Double-click** any terminal label to edit directly (`Enter` = apply, empty = restore automatic numbering, `Esc` = cancel).
  - **Multi-selection:** Single click, `Ctrl+Click`, `Shift+Click`, and marquee drag selection to reassign types, insert before/after, or delete together.
- **Automatic matching accessories:**
  - The right-hand Properties panel automatically displays matching accessories for the selected terminals: **End plates / covers**, **Jumpers / cross-connectors**, **Marking strips / tags**, **End brackets**, and **N-busbars**.
- **Printer-ready Export (`.xlsx` & `.csv`):**
  - **Excel (`.xlsx`):** 3 structured worksheets:
    1. `Druckfolge` / Print Sequence (`ByTierStrip` by default – outputs every tier contiguously as a strip, e.g. all of Tier 1 first, then Tier 2, then Tier 3; optionally column- or block-ordered, with/without `PE`)
    2. `Ebenen-Spalten` / Tier Columns
    3. `Ebenen-Matrix` / Tier Matrix
    - All cells are strictly formatted as **Excel Text (`@`)** so leading zeros (`01`) or dot notation (`1.1`) are never converted into numbers or dates.
  - **CSV (`.csv`):** UTF-8 with BOM and semicolon `;` as the default delimiter.
  - **Project files:** Save and load `.termi` or `.json` files including manual overrides and custom terminals.
- **7 Languages, Themes & built-in Update Checker:**
  - Instant UI switching between **DE, EN, FR, IT, ES, PL, NL** – with high-contrast **Light and Dark** themes.
  - Built-in **GitHub Update Checker** plus direct **GitHub** and **Buy Me a Coffee** links.

---

## Downloads & Portability

All prebuilt binaries are available on the **[Releases page](https://github.com/PatrickBarbian/TermiGrid/releases)**:

| Platform | Architecture / Target | File | Notes |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100% portable** (single `.exe` built with `+crt-static` – no installer, no VCRUNTIME needed) |
| **macOS** | Universal (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Drag the `.app` bundle into Applications |
| **Android** | Universal (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphones, tablets & foldables (Android 7.0 – 15 / API 24–35, 16 KB page-aligned, v1/v2/v3 signed) |

### Platform Notes

- **Windows:** Launch directly via double-click from a USB drive or local folder – no administrator rights or external runtimes required.
- **macOS (unsigned `.app`):** On first launch, use **Right-click -> Open** or allow execution under *System Settings -> Privacy & Security*.
- **Android:** Download the APK and allow installing from unknown sources for your browser/file manager. With 16 KB page alignment and `resizeableActivity`, it supports modern devices (e.g. Pixel 10 Pro XL) and foldables. Exports are saved to `/sdcard/Download/TermiGrid/`.

---

## Quick Guide

1. **Add terminals:** Choose manufacturer and terminal type in the right panel (or use the search bar) and click *Append Terminal* or *Insert Before/After*.
2. **Set start number & type display:** Adjust the starting number at the top and switch between vertical (`-90°`) and horizontal type labels as needed.
3. **Edit labels:** Double-click any label in the CAD editor to enter custom text (`Enter` to confirm, clear input to revert to auto-numbering).
4. **Check accessories:** Select terminals and inspect matching end plates, jumpers, markers, end brackets, and N-busbars in the *Matching Accessories* section.
5. **Export label list:** Select print order (*Tier by Tier / Strip* by default) and whether to include `'PE'`, then export as `.xlsx` or `.csv`.

---

## Build from Source

Requires a recent Rust toolchain (`rustup` / `cargo`).

```bash
# Quality checks & tests
cargo clippy --all-targets -- -D warnings
cargo test

# Desktop release build (native Windows / Linux / macOS)
cargo build --release --bin termigrid

# macOS Universal (Intel + Apple Silicon)
cargo build --release --bin termigrid --target x86_64-apple-darwin
cargo build --release --bin termigrid --target aarch64-apple-darwin

# Android Universal APK (with cargo-apk & Android NDK installed)
cargo apk build --release --lib
```

---

## Links & Support

- **GitHub Repository:** https://github.com/PatrickBarbian/TermiGrid
- **Releases & Downloads:** https://github.com/PatrickBarbian/TermiGrid/releases
- **Support / Buy Me a Coffee:** https://buymeacoffee.com/02eOWKzm5W

---

## License

Licensed under the **MIT License**.
