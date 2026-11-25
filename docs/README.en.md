<div align="center">

<img src="../assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**DIN-Rail Terminal Block Strip Configurator, CAD Planner, EPLAN P8 Roundtrip & Label Exporter for Control Cabinet Engineering**

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

**TermiGrid** is a fast, cross-platform desktop and mobile engineering tool (written in Rust Edition 2024 with `egui` / `eframe 0.37`) for planning, visualizing, and configuring DIN-rail terminal strips in electrical control cabinet manufacturing. It bridges the gap between electrical CAD (EPLAN Electric P8), interactive 2D workshop layout planning, physical accessory calculation (end plates, comb jumpers with cut pins, end brackets), and industrial printer marking systems (WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO).

> **✨ VibeCoded:** This project is **VibeCoded** – designed, engineered, and continuously refined with **AI assistance** in close collaboration with the project author.

---

## Key Features

- **782+ Verified Terminal Block Specs + 26 DIN-Rail End Brackets + Custom Blocks:**
  - **WAGO:** TOPJOB S (`2000–2016` standard, `2200–2216` with push-button, `2102–2116` with lever), `2002-32xx` triple-deck with PE on the bottom level, installation terminals (`2003`, `2005`, `2203`, `2205`), `2022` X-COM, `2007` CT test/disconnect, `280/281` classic, and `285` high-current (`35–95 mm²`).
  - **Phoenix Contact:** COMPLETE line, Push-in `PT 1,5/S` to `PT 35`, `PTIO` sensor/actuator, `PTPOWER 35–95`, `PTTB`/`PTTBS` double-level, `PTV` lateral conductor connection, `PTI` / `UTN` / `UTI` installation terminals, disconnect terminals (`QTC 1,5-TG`, `URTK/SP`, `URTK 6`), `ST` spring-cage, `UT` screw, `UK 5 N`.
  - **Weidmüller:** Klippon Connect A-Series (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), `AITB` installation terminals, P-Series marshalling and potential distribution terminals (`PPV 4/8/16` & `PRV 4/8/16` with single-marker labeling), W-Series (`WDU/WPE/WDK/WTR/WSI/DLI`), Z-Series (`ZDU/ZPE/ZDK/ZDL`).
  - **26 TS 35 End Brackets:** Dedicated end clamps from leading manufacturers (WAGO 249-116/117, Phoenix Contact CLIPFIX 35 / E/UK, Weidmüller WEW 35/2, etc.).
  - **Custom Blocks & Strip Template Library:** Visual terminal block editor (`custom_specs.json` PC-wide and project-embedded) and template library (`strip_library.json`) for saving and reusing recurring strip assemblies.

- **GitHub Live Database Updater (Hot-Reload without App Rebuild):**
  - Automatic non-blocking background check on startup for terminal database updates on GitHub.
  - Clear **"Database Update Available"** button in the UI: 1-click download, validation check, local persistence, and live hot-reload without restarting or rebuilding the app.

- **Local Quick-Prompt Assistant (100% Offline, No Cloud LLM):**
  - Fast plain-text circuit builder: e.g., `4x Drehstrom Abgang 2,5mm² Installationsklemmen`, `4x 3-pol PTI 2,5`, `6x +24V QUATTRO gebrückt`, `2x Reserve leer`.
  - Live preview of generated terminal blocks and instant 1-click batch insertion into the active strip.

- **EPLAN Electric P8 Integration & Full Roundtrip:**
  - **CLI CSV Import:** Launch via command line `TermiGrid.exe --import-eplan-csv "export.csv"` or `Menu → Import EPLAN CSV...` with automatic stripping of configurable vendor prefixes (`PXC.`, `WAG.`, `WDM.`).
  - **Intelligent EPLAN PDF BOM Import:** Drag & drop PDF parts lists, automatic recognition of installation blocks, N-busbars, and feed-in blocks with busbar support bracket calculation (every 200 mm).
  - **1-Click EPLAN Return Package:** Generates `_EPLAN_Zubehoer_Import.csv` & `.xml` (with automatically restored vendor prefixes, strictly brand-matched) and clean individual DXF drawings per strip (`/DXF/<BMK>.dxf`) for direct EPLAN DXF page import.
  - **EPLAN C# Bridge Script:** Export `TermiGrid_EPLAN_Bridge.cs` at the click of a button for seamless menu/toolbar integration inside EPLAN P8.
  - **Discreet EPLAN Settings Dialog:** Tucked away in `Menu → EPLAN Interface...` for configuring prefixes, delimiters, and export directories.

- **CAD Canvas, Automated Edit Mode & End Plate Calculation:**
  - True-to-scale 2D CAD canvas with end plates (D) and partition plates rendered in physical width.
  - **Automated Edit Mode:** Automatically switches on when modifying blocks in the CAD view.
  - **Automated End Plate Calculation:** End and partition plates are automatically computed and updated during export, e-Planer view, and EPLAN roundtrip.
  - Inline double-click label editing (supports empty/blank labels to preserve printer gaps), smooth touch & middle-click panning, Ctrl+wheel zooming.

- **Bridge Mode, Double-Chamber Cascading & Cut-Pin Instructions:**
  - Define color-coded potentials (`+24V`, `GND`, `L1` …) and assign them across tiers by clicking and dragging.
  - Real comb jumper calculation with overlapping 2-slot cascading.
  - Physical cut-pin instructions (`PIN ENTFERNEN BEI`) for skipped blocks.
  - Automatic wire bridge (`DB`) fallback on tier or pitch transitions.

- **Integrated Searchable Guide & AI Prompt Generator (`Guide` Tab):**
  - 13 detailed topics with multi-token full-text search, category filter tags, and quick-search shortcuts.
  - Step-by-step instructions for adding custom terminals locally (visually or via JSON).
  - Copy-ready AI prompt template for LLMs (ChatGPT, Claude, etc.) to generate validated terminal JSON files.

- **Printer-Ready Export (`.xlsx` & `.csv`):**
  - 4 specialized Excel sheets:
    1. `Druckdaten (Flach)` (pure `@` text cells for WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO)
    2. `Etagen-Druckstreifen` (continuous strip per tier E1..E4)
    3. `Bestückungsplan (Matrix)`
    4. `Stückliste & Montage` (aggregated parts list, end plates, comb jumpers with cut-pin instructions, end brackets & total DIN rail length)

- **Offline Photo Disk Cache & Verified CDNs:**
  - Official product photos via bot-block-free CDNs (WAGO, Phoenix Contact CaaS, Weidmüller DAM).
  - Persistent offline disk cache (`photo_cache`) – works completely offline after initial load.

- **Cross-Platform, 7 Languages & Themes:**
  - Windows (`.exe` portable), macOS (`.dmg` Universal), Android (`.apk` Universal), Linux.
  - 7 languages: German, English, French, Italian, Spanish, Polish, Dutch.
  - System / Light / Dark theme, Undo/Redo (`Ctrl+Z` / `Ctrl+Y`), keyboard shortcuts (`Alt+Left/Right`, `Ctrl+D`).

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
- **Android:** Download the APK and allow installing from unknown sources for your browser/file manager. With 16 KB page alignment and `resizeableActivity`, it supports modern devices (e.g. Pixel 10 Pro XL) and foldables. Exports are saved directly to `/sdcard/Download/TermiGrid/`.

---

## Quick Guide

1. **Add Terminals:** Choose manufacturer and terminal type in the right panel (or use search, strip templates, or the natural-language Quick-Prompt Assistant, e.g. `4x Drehstrom Abgang 2,5mm²`).
2. **EPLAN Import & Roundtrip:** Import terminal data via CLI argument (`--import-eplan-csv`), manual CSV import, or PDF parts list drag & drop.
3. **CAD Editing & Bridge Mode:** Double-click any label to modify text, switch to Bridge Mode to paint potentials across tiers by clicking and dragging.
4. **Automatic Accessories & End Plates:** End and partition plates, comb jumpers with cut pins, and end brackets are automatically computed.
5. **Export & Print:** Export 4-sheet Excel spreadsheets (`.xlsx`) or CSV, or generate a 1-click EPLAN Return Package (`_EPLAN_Zubehoer_Import.csv` & `.xml` + DXF drawings).

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
