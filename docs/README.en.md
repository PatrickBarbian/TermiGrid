<p align="center">
  <img src="../assets/icon.png" alt="TermiGrid Logo" width="140" />
</p>

<h1 align="center">TermiGrid</h1>

<p align="center">
  <strong>Terminal Block Strip Configurator &amp; Printer Label Exporter for Control Cabinet Manufacturing</strong><br/>
  <em>VibeCoded with AI assistance (Google Antigravity / Gemini)</em>
</p>

<p align="center">
  <a href="https://github.com/PatrickBarbian/TermiGrid/releases"><img src="https://img.shields.io/github/v/release/PatrickBarbian/TermiGrid" alt="GitHub Release" /></a>
  <a href="https://buymeacoffee.com/02eOWKzm5W"><img src="https://img.shields.io/badge/Buy%20Me%20A%20Coffee-support-orange?logo=buy-me-a-coffee" alt="Buy Me A Coffee" /></a>
  <img src="https://img.shields.io/badge/VibeCoded-AI%20Assisted-blueviolet" alt="VibeCoded" />
</p>

<p align="center">
  <a href="../README.md">🇩🇪 Deutsch</a> ·
  🇬🇧 <strong>English</strong> ·
  <a href="README.fr.md">🇫🇷 Français</a> ·
  <a href="README.it.md">🇮🇹 Italiano</a> ·
  <a href="README.es.md">🇪🇸 Español</a> ·
  <a href="README.pl.md">🇵🇱 Polski</a> ·
  <a href="README.nl.md">🇳🇱 Nederlands</a>
</p>

---

## Overview

**TermiGrid** is a standalone, portable cross-platform application (Rust + `egui`) for electrical control cabinet manufacturing. Design terminal strips (e.g. `-X1`), mix single- and multi-level terminal blocks visually in a 2D CAD view, look up matching accessories, and export printer-ready `.xlsx` / `.csv` lists.

> **Note:** This project was **VibeCoded** by Patrick Barbian with AI assistance from **Google Antigravity / Gemini**.

### Key Features
- **Strip Generator & Dynamic Multi-Level Numbering:** Distributes sequential numbers bottom-to-top across 1-, 2-, and 3-deck terminals while keeping fixed tiers (`N`) untouched and aligning `PE` tiers in a dedicated PE row below the main numbered line.
- **Interactive 2D CAD Editor:** Proportional terminal blocks, `-90°` vertical or grouped horizontal type labels, inline double-click label editing, marquee bulk-select, and instant mixed-strip reassignment.
- **186 Terminal Specs + Custom Types:** WAGO, Phoenix Contact, and Weidmüller with brand logos and an in-app custom terminal builder.
- **Matching Accessories Inspector:** Automatically displays end plates, jumpers/bridges, marking strips, end brackets, and N-busbars for selected terminals.
- **Strict Text Excel Export (`.xlsx` / `.csv`):** Exports `Print Sequence`, `Tier Columns`, and `Tier Matrix` with contiguous per-tier ordering and strict Excel Text (`@`) cell formatting.
- **Built-in Update Checker & Responsive Layout:** Checks GitHub Releases for updates and adapts automatically across Windows, macOS (Universal Intel + Apple Silicon), and Android (Smartphones, Foldables & Tablets) in 7 languages.

## Downloads ([Releases](https://github.com/PatrickBarbian/TermiGrid/releases))
- **Windows (`x86_64`):** `TermiGrid-windows-x86_64.exe` (100% portable single `.exe` with statically linked CRT)
- **macOS Universal (Intel + Apple Silicon):** `TermiGrid-macos-universal.dmg`
- **Android (Phones, Foldables & Tablets):** `TermiGrid-android-universal.apk`

## Links & Support
- 🔗 **GitHub:** [https://github.com/PatrickBarbian/TermiGrid](https://github.com/PatrickBarbian/TermiGrid)
- ☕ **Buy Me a Coffee:** [https://buymeacoffee.com/02eOWKzm5W](https://buymeacoffee.com/02eOWKzm5W)
