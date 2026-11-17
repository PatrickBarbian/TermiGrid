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

- **277 built-in terminal block specs + custom blocks:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (push-button), `2102–2116` (lever), `2022` X-COM, `2007` CT test, `280/281`, and `285` high-current (`35–95 mm²`)
  - **Phoenix Contact:** `PT 1,5/S` incl. `PTIO`, `PT 2,5–35`, `PTPOWER 35–95`, `PTTB`, `PTTBS`, `PTV`, `PTI`/`UTI` installation terminals, `ST`, `UT`, & `UK 5 N`
  - **Weidmüller:** `A-Series` (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), `AITB` installation terminals, `W-Series` (`WDU/WPE/WDK/WTR/WSI/DLI`), & `Z-Series` (`ZDU/ZPE/ZDK/ZDL`)
  - **Custom terminals:** Define custom blocks directly in the app including `1×` vs `2×` jumper chambers
- **Multi-level numbering, N/PE modes & special characters:**
  - Installation & multi-tier terminals (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, etc.): Any existing **PE connection** is aligned on a dedicated PE row **below the bottom main tier**.
  - **N & PE numbering:** Switch per strip between fixed (`N`/`PE`), shared sequential numbering (`1, 2, …`), shared `1N, 2N` / `1PE, 2PE`, or **separate N/PE counters (`1N, 2N …` / `1PE, 2PE …`)**.
  - **Batch prefix / suffix / special characters:** Select any number of terminals and attach symbols (`+`, `-`, `~`, `=`, `.1`, `Ω`, etc.) before or after numbers across all or targeted tiers (preserved across renumbering).
- **Bridge Mode & Potential Calculation:**
  - Create color-coded **potentials** (`+24V`, `GND`, `L1` …) and tap/click terminal tiers directly in CAD.
  - **Automatic calculation:** Computes required comb jumper poles (e.g. `2002-405`, `FBS 5-5`, `ZQV`), marks **cut-out pins (`Cut pin ×`)** over skipped blocks, inserts **wire bridges (`DB`)** across multi-tier jumps (`E1 <-> E2`) or pitch changes, and respects **single-chamber** (wire bridge beyond max poles) vs **double-chamber** terminals (overlapping slot-1/slot-2 cascades).
- **Interactive 2D CAD Editor (Desktop & Touch-friendly):**
  - Clean CAD canvas without DIN-rail clutter, embedded brand logos (**WAGO**, **Phoenix Contact**, **Weidmüller**), tool switcher (**Pan**, **Box Select**, **Bridge Mode**), touch panning & middle/right-drag scrolling.
  - **Type display & inline editing:** `-90°` vertical or horizontal groups; **double-click** labels to edit inline (`Enter` = apply, empty = auto, `Esc` = cancel).
- **Product Preview, Manufacturer Link & Matching Accessories:**
  - Displays **product photo (when reachable) / technical profile preview**, **jumper chamber count**, clickable **manufacturer product page link**, plus **end plates, jumpers, markers, end brackets, and N-busbars**.
- **Printer-ready Export (`.xlsx` & `.csv`) & Project files (`.termi` / `.json`):**
  - **Excel (`.xlsx`):** 3 worksheets (`Druckfolge` / `ByTierStrip` default, `Ebenen-Spalten`, `Ebenen-Matrix`), strictly formatted as **Excel Text (`@`)**.
- **Guided Project Start, 7 Languages & System Theme:**
  - Startup pipeline guides from *New Project / Open* -> *First Terminal Strip* -> *CAD Editor*. **DE, EN, FR, IT, ES, PL, NL** – follows **OS System Theme** by default (toggle System/Light/Dark), high-contrast action buttons, live GitHub update check on Desktop & Android, plus **GitHub** and **Buy Me a Coffee**.

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
