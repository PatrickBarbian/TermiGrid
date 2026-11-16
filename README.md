<p align="center">
  <img src="assets/icon.png" alt="TermiGrid Logo" width="140" />
</p>

<h1 align="center">TermiGrid</h1>

<p align="center">
  <strong>Reihenklemmen-Konfigurator &amp; Etiketten-Exporter für den Schaltschrankbau</strong><br/>
  <em>Cross-platform Terminal Block Strip Designer &amp; Printer Label Exporter (Windows · macOS Universal · Android)</em>
</p>

---

## Überblick / Overview

**TermiGrid** ist eine eigenständige, schnelle Desktop- und Mobilanwendung (Rust + `egui`) für den Schaltschrankbau. Sie ermöglicht das Anlegen, visuelle Konfigurieren, Mischen und Exportieren von Reihenklemmenleisten (z. B. `-X1`) für Beschriftungsdrucker (WAGO SmartPrinter, Phoenix Contact THERMOMARK, Weidmüller PrintJet / M-Print PRO usw.).

### Hauptfunktionen

- **Klemmleisten-Generator:** Neue Leisten (z. B. `-X1`) mit Start- und Endnummer (`1..50`) oder fester Blockanzahl und Standard-Klemmentyp anlegen.
- **Intelligente Mehrstock-Nummerierung:**
  - Verteilt eine lineare Nummernfolge dynamisch von unten nach oben über 1-, 2- und 3-Etagen-Klemmen.
  - **Beispiel Standard-Doppelstock (`L/L`):** Unten `1, 3, 5, 7` — Oben `2, 4, 6, 8`.
  - **Beispiel mit Neutralleiter (`L/N` + `L/L`):** Fest belegte Ebenen wie `N` verbrauchen keine fortlaufende Nummer → Unten `1, 2, 3, 5`, Oben `N, N, 4, 6`.
  - **Installationsklemmen mit PE (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`):** Schutzleiter (`PE`) werden in der separaten **PE-Zeile unterhalb** der Hauptzeile ausgerichtet, überspringen die laufende Nummerierung und bleiben für den Etikettendruck standardmäßig leer (optional aktivierbar).
- **2D-CAD-Editor:**
  - Gestapelte Ebenen-Ansicht mit proportionaler Klemmenbreite, Zoom (`Strg + Mausrad`) und vertikaler `-90°` oder horizontal gruppierter Typ-Anzeige.
  - **Direkte Bearbeitung per Doppelklick** auf ein Klemmenfeld (`Enter` = übernehmen, leer = Automatik, `Esc` = abbrechen).
  - **Auswahl:** Einzelklick, `Strg + Klick` (Mehrfach), `Shift + Klick` (Bereich) und aufgezogenes Auswahlrechteck (Marquee Bulk-Select).
  - **Mischbestückung:** Ausgewählte Klemmen sofort auf andere Typen/Hersteller umstellen; Nummerierung und Vorschau aktualisieren sich in Echtzeit.
- **Umfangreiche Klemmen-Datenbank (186 Typen) & Hersteller-Logos:**
  - **WAGO** (TOPJOB S `2000–2016`, Drücker `2200–2216`, Hebel `2102–2116`, `2022` X-COM, `280/281`)
  - **Phoenix Contact** (`PT 1,5/S`, `PT 2,5–35`, `PTPOWER`, `PTTB`, `PTTBS`, `PTV`, `PTI`, `ST`, `UT`)
  - **Weidmüller** (`A-Series`, `AITB`, `W-Series`, `DLI`, `Z-Series`)
  - Eigene Klemmentypen können direkt im Tab **Klemmen-Datenbank** angelegt und im Projekt gespeichert werden.
- **Passendes Zubehör in der rechten Spalte:**
  - Zeigt für ausgewählte Klemmen automatisch passende **Enddeckel / Abschlussplatten**, **Brücken / Querverbinder**, **Beschriftungsmaterial**, **Endklammern** und **N-Sammelschienen**.
- **Excel- (`.xlsx`) & CSV-Export für Drucksoftware:**
  - Drei Arbeitsblätter (`Druckfolge`, `Ebenen-Spalten`, `Ebenen-Matrix`).
  - Standardmäßig **ebenenweise** (`Ebenenweise (Streifendruck)`): alle Schilder von **Ebene 1** zusammenhängend, dann **Ebene 2**, dann **Ebene 3**.
  - **ALLE Zellen** werden strikt im Excel-Textformat (`@`) geschrieben, sodass Werte wie `1-2`, `01` oder `10/2` niemals in Daten oder Zahlen umformatiert werden.
- **Projektverwaltung & UX:**
  - Speichern und Laden als `.termi` oder `.json`.
  - **7 Sprachen:** Deutsch (Standard), English, Français, Italiano, Español, Polski, Nederlands.
  - **Hell- & Dunkelmodus** sowie integrierte **Anleitung**.
  - **Responsives Layout:** 3-Spalten-Arbeitsbereich auf Desktop, Tablets und aufgeklappten Foldables; automatische kompakte Tab-Ansicht auf Smartphones und zugeklappten Foldables.

---

## Downloads & Installation (Releases)

Aktuelle Builds stehen auf der [**GitHub Releases**](../../releases)-Seite bereit:

| Plattform | Datei | Hinweise |
|---|---|---|
| **Windows (x86_64)** | `TermiGrid-windows-x86_64.exe` | Portable Einzeldatei (kein Installer nötig) |
| **macOS Universal (Intel + Apple Silicon M1–M4)** | `TermiGrid-macos-universal.dmg` | Enthält `TermiGrid.app` als Universal Binary (`x86_64` + `arm64`) |
| **Android (Smartphones, Foldables & Tablets)** | `TermiGrid-android-universal.apk` | Multi-ABI APK (`arm64-v8a`, `armeabi-v7a`, `x86_64`), Android 7.0+ (API 24–35), `resizeableActivity` für Foldables |

### Hinweise zur Installation auf macOS & Android
- **macOS:** `.dmg` öffnen und `TermiGrid.app` in `Programme` ziehen. Falls Gatekeeper beim ersten Start warnt (da ohne Apple Developer ID signiert), einmal Rechtsklick → *Öffnen* wählen.
- **Android:** `.apk` herunterladen und Installation aus unbekannten Quellen für den Dateimanager/Browser erlauben. Exporte und Projektdateien werden unter `/sdcard/Download/TermiGrid/` abgelegt.

---

## Kurzanleitung / Quick Guide

1. **Leiste anlegen:** Links auf `+ Neue Leiste` klicken, Kennzeichnung (z. B. `-X1`), Start-/Endnummer (`1..50`) und Standard-Klemmentyp wählen.
2. **Mischbestückung:** Klemmen im CAD-Editor per Klick oder Rahmen markieren → rechts neuen Typ wählen → `Typ auf Auswahl anwenden`.
3. **Einzelnummer ändern:** Doppelklick direkt auf die Zahl in der CAD-Klemme → Wunschtext eingeben → `Enter` (leeres Feld stellt Automatik wieder her).
4. **Zubehör prüfen:** Markierte Klemmen zeigen rechts unter *Passendes Zubehör* sofort Enddeckel, Brücken, Markierer und Endklammern.
5. **Für Drucker exportieren:** `Excel Export (.xlsx)` wählen → im Blatt `Druckfolge` oder `Ebenen-Spalten` stehen alle Schilder je Ebene zusammenhängend als reiner Text (`@`).

---

## Selbst kompilieren / Building from Source

Voraussetzung: aktuelles Rust-Toolchain (`rustup` / `cargo`).

```bash
# Tests & Linter
cargo clippy --all-targets -- -D warnings
cargo test

# Windows / macOS / Linux Desktop Release
cargo build --release --bin termigrid

# macOS Universal (auf macOS)
rustup target add x86_64-apple-darwin aarch64-apple-darwin
cargo build --release --bin termigrid --target x86_64-apple-darwin
cargo build --release --bin termigrid --target aarch64-apple-darwin

# Android APK (mit Android SDK/NDK + cargo-apk)
cargo install cargo-apk --locked
cargo apk build --release --lib
```

---

## Lizenz / License

MIT License — Copyright © 2026 Patrick Barbian
