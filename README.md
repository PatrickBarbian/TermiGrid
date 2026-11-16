<p align="center">
  <img src="assets/icon.png" alt="TermiGrid Logo" width="140" />
</p>

<h1 align="center">TermiGrid</h1>

<p align="center">
  <strong>Reihenklemmen-Konfigurator &amp; Etiketten-Exporter für den Schaltschrankbau</strong><br/>
  <em>VibeCoded mit KI-Unterstützung (Google Antigravity / Gemini)</em>
</p>

<p align="center">
  <a href="https://github.com/PatrickBarbian/TermiGrid/releases"><img src="https://img.shields.io/github/v/release/PatrickBarbian/TermiGrid" alt="GitHub Release" /></a>
  <a href="https://buymeacoffee.com/02eOWKzm5W"><img src="https://img.shields.io/badge/Buy%20Me%20A%20Coffee-support-orange?logo=buy-me-a-coffee" alt="Buy Me A Coffee" /></a>
  <img src="https://img.shields.io/badge/VibeCoded-AI%20Assisted-blueviolet" alt="VibeCoded" />
</p>

<p align="center">
  <strong>Sprachen / Languages:</strong>
  🇩🇪 <strong>Deutsch</strong> ·
  <a href="docs/README.en.md">🇬🇧 English</a> ·
  <a href="docs/README.fr.md">🇫🇷 Français</a> ·
  <a href="docs/README.it.md">🇮🇹 Italiano</a> ·
  <a href="docs/README.es.md">🇪🇸 Español</a> ·
  <a href="docs/README.pl.md">🇵🇱 Polski</a> ·
  <a href="docs/README.nl.md">🇳🇱 Nederlands</a>
</p>

---

## Überblick

**TermiGrid** ist eine eigenständige, portable Desktop- und Mobilanwendung (Rust + `egui`) für den Schaltschrankbau. Sie ermöglicht das Anlegen, visuelle Konfigurieren, Mischen und Exportieren von Reihenklemmenleisten (z. B. `-X1`) für Beschriftungsdrucker (WAGO SmartPrinter, Phoenix Contact THERMOMARK, Weidmüller PrintJet / M-Print PRO usw.).

> **Hinweis zur Entstehung:** Dieses Projekt wurde **VibeCoded** – konzipiert und gesteuert von Patrick Barbian mit KI-Unterstützung durch **Google Antigravity / Gemini**.

### Hauptfunktionen

- **Klemmleisten-Generator:** Neue Leisten (z. B. `-X1`) mit Start- und Endnummer (`1..50`) oder fester Blockanzahl anlegen.
- **Intelligente Mehrstock-Nummerierung:**
  - Verteilt eine lineare Nummernfolge dynamisch von unten nach oben über 1-, 2- und 3-Etagen-Klemmen.
  - **Standard-Doppelstock (`L/L`):** Unten `1, 3, 5, 7` — Oben `2, 4, 6, 8`.
  - **Mit Neutralleiter (`L/N` + `L/L`):** Feste Ebenen wie `N` verbrauchen keine fortlaufende Nummer → Unten `1, 2, 3, 5`, Oben `N, N, 4, 6`.
  - **Installationsklemmen mit PE (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`):** Schutzleiter (`PE`) sitzen in der separaten **PE-Zeile unterhalb** der Hauptzeile, überspringen die laufende Nummerierung und bleiben für den Druck standardmäßig leer.
- **2D-CAD-Editor:** Gestapelte Ebenen-Ansicht mit proportionaler Klemmenbreite, Zoom (`Strg + Mausrad`), vertikaler `-90°` oder horizontal gruppierter Typ-Anzeige, direkter Textbearbeitung per **Doppelklick**, Bereichs-/Mehrfach-/Rahmenauswahl und sofortiger Mischbestückung.
- **Klemmen-Datenbank (186 Typen) & Hersteller-Logos:** WAGO (`2000–2016`, `2200–2216`, `2102–2116`, `2022`, `280/281`), Phoenix Contact (`PT`, `PTTB`, `PTTBS`, `PTV`, `PTI`, `ST`, `UT`, `PTPOWER`) und Weidmüller (`A-Series`, `AITB`, `W-Series`, `DLI`, `Z-Series`) – plus eigene Klemmentypen direkt im Projekt speicherbar.
- **Passendes Zubehör:** Zeigt in der rechten Spalte automatisch **Enddeckel**, **Brücken / Querverbinder**, **Beschriftungsmaterial**, **Endklammern** und **N-Sammelschienen** für ausgewählte Klemmen.
- **Excel- (`.xlsx`) & CSV-Export:** Drei Blätter (`Druckfolge`, `Ebenen-Spalten`, `Ebenen-Matrix`), standardmäßig ebenenweise (`E1` → `E2` → `E3`) und mit **allen Zellen strikt als Text (`@`)** formatiert.
- **Projektmanagement, Updates & 7 Sprachen:** `.termi`/`.json`-Dateien, Hell-/Dunkelmodus, integrierter **GitHub-Update-Checker**, responsives Layout für **Desktop, Tablets, Smartphones und Foldables** sowie vollständige UI & Anleitung in **7 Sprachen**.

---

## Downloads & Portabilität ([Releases](https://github.com/PatrickBarbian/TermiGrid/releases))

| Plattform | Datei | Hinweise |
|---|---|---|
| **Windows (x86_64)** | `TermiGrid-windows-x86_64.exe` | **100 % portable Einzeldatei** (inkl. statischer C-Runtime `+crt-static`, kein Installer, keine DLLs nötig) |
| **macOS Universal (Intel + Apple Silicon)** | `TermiGrid-macos-universal.dmg` | Universal Binary (`x86_64` + `arm64`) für Intel-Macs & M1–M4 |
| **Android (Smartphones, Foldables & Tablets)** | `TermiGrid-android-universal.apk` | Multi-ABI APK (`arm64-v8a`, `armeabi-v7a`, `x86_64`), API 24–35, `resizeableActivity` für Foldables |

---

## Unterstützung & Links

- 🔗 **GitHub Repository:** [https://github.com/PatrickBarbian/TermiGrid](https://github.com/PatrickBarbian/TermiGrid)
- 📦 **Releases & Updates:** [https://github.com/PatrickBarbian/TermiGrid/releases](https://github.com/PatrickBarbian/TermiGrid/releases)
- ☕ **Support / Buy Me a Coffee:** [https://buymeacoffee.com/02eOWKzm5W](https://buymeacoffee.com/02eOWKzm5W)

---

## Lizenz

MIT License — Copyright © 2026 Patrick Barbian
