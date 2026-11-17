# TermiGrid – Agent Guide & Project Reference (`AGENTS.md`)

Dieses Dokument wird von **Antigravity** automatisch geladen und enthält alle wichtigen Architektur-, Katalog-, UI- und Release-Regeln für **TermiGrid** (`https://github.com/PatrickBarbian/TermiGrid`).

---

## 1. Projektübersicht
- **Name:** TermiGrid
- **Zweck:** Plattformübergreifender DIN-Schienen-Klemmenleisten-Konfigurator, CAD-Planer & Beschriftungs-Exporter (CSV / Excel `.xlsx` / Projekt-JSON `.tgrid.json`) für WAGO, Phoenix Contact und Weidmüller.
- **Tech-Stack:** Rust (Edition 2024) + `egui` / `eframe 0.37` + `rust_xlsxwriter` + `image`.
- **Zielplattformen:**
  - **Windows:** `TermiGrid-windows-x86_64.exe`
  - **macOS:** `TermiGrid-macos-universal.dmg` (Universal Binary `x86_64` + `aarch64`)
  - **Android:** `TermiGrid-android-arm64.apk` (`aarch64-linux-android` via `cargo-apk`)

---

## 2. Verzeichnis- & Modulstruktur

- [`Cargo.toml`](Cargo.toml): Abhängigkeiten und zentrale Versionsnummer (`version = "1.x.y"`).
- [`assets/terminal_specs.json`](assets/terminal_specs.json): Basis-Spezifikationen der 277 Reihenklemmen (WAGO, Phoenix Contact, Weidmüller).
- [`src/catalog/mod.rs`](src/catalog/mod.rs):
  - Lädt den Katalog und sortiert ihn nach **Hersteller** und anschließend per **Natural Sort** nach **Typ** (`natural_cmp_str`, damit z. B. `2,5 mm²` vor `10 mm²` steht).
  - `resolve_manufacturer_meta(&self.id)` hat **höchste Priorität** vor JSON-Feldern in `effective_article_number()`, `effective_product_url()` und `effective_image_url()`, damit alle Artikelnummern, Direktlinks und CaaS/DAM-Produktbilder garantiert aktuell und verifiziert bleiben.
  - **Phoenix Contact URL-Regeln:**
    - Mehrstockklemmen (2- und 3-Etagen): immer `mehrstockklemme-...` (niemals `dreistockklemme-`!)
    - Mehrleiterklemmen (TWIN / QUATTRO ohne PE): `mehrleiterklemme-...`
    - Alle Schutzleiterklemmen (auch PTI / STI / UTI / TWIN-PE): `schutzleiterklemme-...`
    - Installationsklemmen (ohne PE): `installationsklemme-...`
    - Trennklemmen (`-MT`, `-TG`): `trennklemme-...`
    - Messwandlertrennklemmen (`PTME`): `messwandlertrennklemme-...`
    - Sicherungsklemmen (`-HESI`): `sicherungsklemme-...`
    - Sensor-/Aktorklemmen (`PTIO`): `sensor-aktor-klemme-...`
    - Hochstromklemmen (`PTPOWER`): `hochstromklemme-...`
  - **Produktbilder (CDN ohne Bot-Blockade):**
    - WAGO: `https://www.wago.com/medias/?context=...`
    - Phoenix Contact: `https://caas.phoenixcontact.com/caas/v1/stable/media/<media_id>/full/b408?format=jpg`
    - Weidmüller: `https://assets.dam.weidmueller.com/assets/<asset_id>/<uuid>/M_..._tif.jpg`
- [`src/catalog/accessories.rs`](src/catalog/accessories.rs):
  - Ermittelt passendes Zubehör je Klemme (`end_plate`, `jumpers`, `markers`, `end_bracket`, `extra`).
  - `end_plate_width_tenths_mm(spec)`: Gibt die reale physische Breite der Abschluss-/Zwischenplatte in `0,1 mm` zurück (`0` bei geschlossenen Gehäusen wie PE-Einzelklemmen `USLKG 5`, `WPE 16/35`, Sicherungsklemmen `-HESI` / `WSI 6` / `AFS 4`, Hochstromklemmen `285-*` / `PTPOWER` / `A2C 35` / `WDU 35`).
- [`src/model/mod.rs`](src/model/mod.rs), [`src/model/numbering.rs`](src/model/numbering.rs), [`src/model/bridging.rs`](src/model/bridging.rs):
  - Datenmodell (`Project`, `TerminalStrip`, `TerminalBlock`, `BridgeGroup`, `PotentialType`).
  - `calculate_end_plates(strip, catalog)`: Berechnet automatisch benötigte Abschluss- und Zwischenplatten bei Profilwechsel (Hersteller, Serie, Etagenanzahl, Bauhöhe) und am Leistenende.
  - `calculate_bridging(strip, catalog)`: Berechnet Kammbrücken, Kaskadierung bei Doppelbrückenschacht, übersprungene Klemmen (entfernte Pins `cut_positions`) und Drahtbrücken.
- [`src/ui/mod.rs`](src/ui/mod.rs):
  - Haupt-UI, geführte Start-Pipeline (*Neues Projekt* → *Klemmleiste anlegen* → *CAD-Ansicht*), Theme-Umschaltung (`System`, `Hell`, `Dunkel`), **Undo/Redo-Stack** (`undo_stack` / `redo_stack`, `Strg+Z` / `Strg+Y`) sowie Klemmen verschieben/duplizieren (`Alt+Links/Rechts`, `Strg+D`).
- [`src/ui/cad_canvas.rs`](src/ui/cad_canvas.rs):
  - Interaktive 2D-CAD-Ansicht:
    - Zoom per `Strg + Mausrad` (oder Pinch-Geste auf Touch).
    - Panning per **gedrücktem Mausrad (mittlere Maustaste)** + Ziehen.
    - Reale Maßstabsdarstellung der **Abschluss-/Zwischenplatten (D)** zwischen den Klemmen und am Leistenende.
    - **Kammbrücken** werden unterhalb der jeweiligen Klemmebene gezeichnet (`tier_gap`), inklusive metallischer Pins und rotem `×` bei herausgebrochenen Pins.
    - Im **Brücken-Modus (`BridgePaint`)**: Inline-Potential-Schnellleiste direkt über dem CAD-Canvas + Massenbrückung per **Linksklick & Ziehen** über mehrere Klemmen.
- [`src/ui/product_media.rs`](src/ui/product_media.rs):
  - Asynchroner Bild-Loader mit **persistentem Offline-Disk-Cache** (`photo_cache` im App-Datenverzeichnis).
- [`src/export/xlsx.rs`](src/export/xlsx.rs) & [`src/export/csv.rs`](src/export/csv.rs):
  - Exportiert 4 Excel-Tabellenblätter:
    1. `Druckdaten (Flach)` (reine Textzellen für Beschriftungsdrucker wie WAGO Smart Designer, Phoenix Project Complete, Weidmüller M-Print PRO)
    2. `Etagen-Druckstreifen` (durchgehende Streifen je Etage E1..E4)
    3. `Bestückungsplan (Matrix)`
    4. `Stückliste & Montage` (aggregierte Klemmen-Stückliste, Enddeckel-Bedarf, Kammbrücken inkl. Pin-Kürzungsanweisung, Endwinkel & Gesamtlänge auf der Hutschiene)

---

## 3. Pflicht-Prüfungen vor jedem Commit / Release

Vor jedem Commit oder Release müssen folgende Befehle fehlerfrei (`0 warnings`, alle Tests grün) durchlaufen:

```bash
cargo clippy --all-targets -- -D warnings
cargo test
```

(Falls das Android-Target installiert ist, zusätzlich: `cargo clippy --lib --target aarch64-linux-android -- -D warnings`)

---

## 4. Release-Workflow (GitHub Actions)

Die Datei [`.github/workflows/ci-release.yml`](.github/workflows/ci-release.yml) baut automatisch alle 3 Plattformen (Windows `.exe`, macOS `.dmg`, Android `.apk`) und erstellt das GitHub-Release, sobald ein Tag `v*` gepusht wird (oder der Workflow manuell per `workflow_dispatch` gestartet wird).

**Schritte für ein neues Release:**
1. Version in [`Cargo.toml`](Cargo.toml) hochsetzen (z. B. `version = "1.2.7"`) und `cargo test` ausführen (aktualisiert `Cargo.lock`).
2. Änderungen committen:
   ```bash
   git add -A
   git commit -m "feat(v1.2.7): ..."
   ```
3. Git-Tag anlegen und zusammen mit `main` pushen:
   ```bash
   git tag -a v1.2.7 -m "TermiGrid v1.2.7"
   git push origin main v1.2.7
   ```
4. Die App prüft beim Start über [`src/ui/updater.rs`](src/ui/updater.rs) automatisch das neueste GitHub-Release und bietet dem Nutzer das passende Update an.
