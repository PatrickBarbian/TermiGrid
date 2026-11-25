<div align="center">

<img src="assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**DIN-Schienen-Klemmenleisten-Konfigurator, CAD-Planer, EPLAN P8 Roundtrip & Etiketten-Exporter für den Schaltschrankbau**

[![Release](https://img.shields.io/github/v/release/PatrickBarbian/TermiGrid)](https://github.com/PatrickBarbian/TermiGrid/releases)
[![VibeCoded](https://img.shields.io/badge/VibeCoded-AI%20Assisted-blueviolet)](#überblick--vibecoded-hinweis)
[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-support-orange?logo=buy-me-a-coffee)](https://buymeacoffee.com/02eOWKzm5W)

<p>
  <strong>🌐 Sprache direkt hier wechseln:</strong><br/>
  <a href="#lang-de">🇩🇪 Deutsch</a> ·
  <a href="#lang-en">🇬🇧 English</a> ·
  <a href="#lang-fr">🇫🇷 Français</a> ·
  <a href="#lang-it">🇮🇹 Italiano</a> ·
  <a href="#lang-es">🇪🇸 Español</a> ·
  <a href="#lang-pl">🇵🇱 Polski</a> ·
  <a href="#lang-nl">🇳🇱 Nederlands</a>
</p>
<p>
  <sub>Einzeldateien / Standalone: <a href="README.md">DE</a> · <a href="docs/README.en.md">EN</a> · <a href="docs/README.fr.md">FR</a> · <a href="docs/README.it.md">IT</a> · <a href="docs/README.es.md">ES</a> · <a href="docs/README.pl.md">PL</a> · <a href="docs/README.nl.md">NL</a></sub>
</p>

</div>

---

<a id="lang-de"></a>

## 🇩🇪 Deutsch (Standard)

### Überblick & VibeCoded-Hinweis

**TermiGrid** ist ein schnelles, plattformübergreifendes Engineering-Werkzeug (geschrieben in Rust Edition 2024 mit `egui` / `eframe 0.37`) zum vollständigen Planen, Visualisieren und Konfigurieren von DIN-Schienen-Klemmenleisten im Schaltschrankbau. Es schließt nahtlos die Lücke zwischen Elektro-CAD (EPLAN Electric P8), interaktiver 2D-Werkstattplanung, physischer Zubehörberechnung (Abschlussplatten, Kammbrücken mit Pin-Kürzungsanweisungen, Endwinkel) und industriellen Beschriftungs- und Markierungsdruckern (WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO).

> **✨ VibeCoded:** Dieses Projekt wurde **VibeCoded** – vollständig entworfen, programmiert und iterativ verfeinert mit **KI-Unterstützung** in enger Zusammenarbeit mit dem Projektautor.

---

### Hauptfunktionen

- **782+ verifizierte Klemmentypen + 26 Hutschiene-Endwinkel + eigene Klemmen:**
  - **WAGO:** TOPJOB S (`2000–2016` Standard, `2200–2216` mit Betätigungsdrücker, `2102–2116` mit Hebel), `2002-32xx` Dreistockklemmen mit PE auf unterster Ebene, Installationsklemmen (`2003`, `2005`, `2203`, `2205`), `2022` X-COM, `2007` Messwandler-/Trennklemmen, `280/281` Klassiker sowie `285` Hochstrom (`35–95 mm²`).
  - **Phoenix Contact:** COMPLETE line, Push-in `PT 1,5/S` bis `PT 35`, Sensor-/Aktorklemmen `PTIO`, `PTPOWER 35–95`, Doppelstock `PTTB`/`PTTBS`, seitlicher Leiteranschluss `PTV`, Installationsklemmen `PTI` / `UTN` / `UTI`, Trennklemmen (`QTC 1,5-TG`, `URTK/SP`, `URTK 6`), Federzug `ST`, Schraube `UT`, `UK 5 N`.
  - **Weidmüller:** Klippon Connect A-Reihe (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), Installationsklemmen `AITB`, P-Reihe Rangier- und Potenzialverteiler (`PPV 4/8/16` & `PRV 4/8/16` mit 1-Schild-Beschriftung je Klemme), W-Reihe (`WDU/WPE/WDK/WTR/WSI/DLI`), Z-Reihe (`ZDU/ZPE/ZDK/ZDL`).
  - **26 TS 35 Endwinkel:** Passgenaue Endklammern führender Hersteller (WAGO 249-116/117, Phoenix Contact CLIPFIX 35 / E/UK, Weidmüller WEW 35/2 etc.).
  - **Eigene Klemmen & Leisten-Bibliothek:** Visueller Klemmen-Editor (`custom_specs.json` PC-weit und projektgebunden) sowie Vorlagen-Bibliothek (`strip_library.json`) für wiederkehrende Leistenkombinationen.

- **GitHub Live-Datenbank-Updater (Hot-Reload ohne App-Neustart):**
  - Automatische, nicht-blockierende Hintergrundprüfung beim Start auf neuere Klemmen-Spezifikationen (`terminal_specs.json`) auf GitHub.
  - Gut sichtbarer **„Datenbank-Update verfügbar“**-Button in der UI: 1-Klick-Download, Validierungsprüfung, lokale Speicherung und Live-Reload im laufenden Betrieb – ganz ohne Neuinstallation oder Programm-Rebuild.

- **Lokaler Schnell-Eingabe-Assistent (Quick-Prompt, 100 % offline ohne Cloud-LLM):**
  - Blitzschnelle Schaltungsgenerierung per Kurztext: z. B. `4x Drehstrom Abgang 2,5mm² Installationsklemmen`, `4x 3-pol PTI 2,5`, `6x +24V QUATTRO gebrückt`, `2x Reserve leer`.
  - Live-Vorschau der generierten Klemmen und 1-Klick-Übernahme direkt in die aktuelle Klemmenleiste.

- **EPLAN Electric P8 & EPLAN 2025 Schnittstelle & Vollständiger Roundtrip:**
  - **CLI CSV-Import:** Start per Befehlszeile `TermiGrid.exe --import-eplan-csv "export.csv"` oder über `Menü → EPLAN CSV importieren...` mit automatischer Erkennung und Bereinigung von Hersteller-Präfixen (`PXC.`, `WAG.`, `WDM.`).
  - **EPLAN 2025 Menüband-Einbindung:** Fertiges C#-Bridge-Skript `TermiGrid_EPLAN_Bridge.cs` (direkt aus TermiGrid per Klick kopier-/exportierbar) zur 1-Klick-Ausführung per Menüband-Aktion `ExecuteScript`.
  - **Zubehör-Rückimport (Wichtig: Modus „Hinzufügen“):** Generiert `_EPLAN_Zubehoer_Import.csv` & `.xml` (nur neu hinzugekommenes Zubehör mit wieder angehängten Hersteller-Präfixen, streng herstellerrein). In EPLAN über `Projektdaten → Geräte → Importieren...` im Modus **Hinzufügen** einlesen – bestehende Klemmen und Schaltpläne bleiben unberührt.
  - **DXF-Aufbaubild ohne Seitenzerschuss:** Saubere Einzel-DXF-Dateien (`/DXF/<BMK>.dxf`) für den maßstabsgetreuen Schaltschrank-Aufbau – in EPLAN einfach über `Einfügen → Grafik → DXF/DWG` auf bestehende Aufbauseiten platzieren (nicht `Seite → Importieren`, um bestehende Grafiken nicht zu löschen).
  - **Intelligenter EPLAN PDF-Stücklistenimport:** Drag & Drop von EPLAN PDF-Plänen, automatische Zuweisung von Installationsklemmen, N-Sammelschienen und Einspeisungen inkl. Sammelschienenbock-Berechnung (alle 200 mm).
  - **Aufgeräumter EPLAN-Einstellungsdialog:** Diskret integriert unter `Menü → EPLAN-Schnittstelle...` zur Konfiguration von Präfixen, Trennzeichen und Exportordnern.


- **CAD-Editor, Automatischer Bearbeitungsmodus & Endplatten-Generierung:**
  - Maßstabsgetreue 2D-CAD-Ansicht mit automatischer Anzeige von Abschluss- (D) und Trennplatten in realer Stärke.
  - **Automatischer Bearbeitungsmodus:** Schaltet sich bei Bearbeitung im CAD selbstständig ein.
  - **Automatische Endplatten-Generierung:** Werden bei Export, e-Planer-Ansicht oder Roundtrip automatisch berechnet und aktualisiert.
  - Inline-Doppelklick-Beschriftung (inklusive `Leeres Schild` für freie Druckstellen in Markierungsstreifen), flüssiges Touch- & Maus-Panning (mittlere Maustaste), Zoom per Strg+Mausrad.

- **Brücken-Modus, Doppelkammer-Kaskadierung & Pin-Abtrennung:**
  - Farbige Potentiale anlegen (`+24V`, `GND`, `L1` …) und per Klick & Ziehen Klemmen ebenenweise zuweisen.
  - Reale Kammbrückenberechnung mit 2-Kammer-Kaskadierung über Schacht 1 und 2.
  - Konkrete Ausstanzanweisungen (`PIN ENTFERNEN BEI`) für übersprungene Klemmen.
  - Automatische Drahtbrücken (`DB`) bei Ebenen- oder Rasterwechseln.

- **Integrierte, durchsuchbare Anleitung & KI-Prompt-Generator (`Anleitung`-Reiter):**
  - 13 detaillierte Themenbereiche mit Volltextsuche, Kategorie-Filtern und Schnellsuche.
  - Schritt-für-Schritt-Anleitung zur lokalen Einbindung eigener Klemmen (visuell oder per JSON).
  - Kopierfertiger Prompt-Template für KI-Tools (ChatGPT, Claude etc.) zum fehlerfreien Generieren passender Klemmen-JSON-Dateien.

- **Druckerfertiger Export (`.xlsx` & `.csv`):**
  - 4 spezialisierte Excel-Tabellenblätter:
    1. `Druckdaten (Flach)` (reine `@` Textzellen für WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO)
    2. `Etagen-Druckstreifen` (durchgehende Streifen je Etage E1..E4)
    3. `Bestückungsplan (Matrix)`
    4. `Stückliste & Montage` (aggregierte Klemmen-Stückliste, Abschlussplatten, Kammbrücken mit Pin-Kürzungsanweisung, Endwinkel & Schienen-Gesamtlänge)

- **Offline-Foto-Disk-Cache & Verifizierte CDNs:**
  - Original-Hersteller-Produktfotos über blockadearme CDNs (WAGO, Phoenix Contact CaaS, Weidmüller DAM).
  - Persistenter lokaler Offline-Cache (`photo_cache`) – funktioniert nach dem ersten Laden vollständig ohne Internet.

- **Plattformübergreifend, 7 Sprachen & Themes:**
  - Windows (`.exe` portabel), macOS (`.dmg` Universal), Android (`.apk` Universal), Linux.
  - 7 Sprachen: Deutsch, English, Français, Italiano, Español, Polski, Nederlands.
  - System / Hell / Dunkel Theme, Undo/Redo (`Strg+Z` / `Strg+Y`), Tastenkürzel (`Alt+Links/Rechts`, `Strg+D`).

---

### Downloads & Portabilität

Alle fertigen Pakete sind auf der **[Releases-Seite](https://github.com/PatrickBarbian/TermiGrid/releases)** sowohl mit Versionsnummer im Dateinamen als auch unter den stabilen Standardnamen verfügbar:

| Plattform | Architektur / Ziel | Dateinamen (Release) | Hinweise |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-v1.9.1-windows-x86_64.exe`<br/>*(oder `TermiGrid-windows-x86_64.exe`)* | **100 % portabel** (Einzeldatei mit `+crt-static` – keine Installation, kein VCRUNTIME nötig) |
| **macOS** | Universal (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-v1.9.1-macos-universal.dmg`<br/>*(oder `TermiGrid-macos-universal.dmg`)* | `.app`-Bundle in den Programme-Ordner ziehen |
| **Android** | Universal (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-v1.9.1-android-universal.apk`<br/>*(oder `TermiGrid-android-universal.apk`)* | Für Smartphones, Tablets & Foldables (Android 7.0 – 15 / API 24–35, 16-KB-Page-Alignment, stabiler Release-Keystore & v1/v2/v3 signiert) |

#### Plattform-Hinweise

- **Windows:** Direkt per Doppelklick von USB-Stick oder lokalem Ordner starten – benötigt keine Administratorrechte und keine Laufzeitbibliotheken.
- **macOS (Unsigniertes App-Bundle):** Beim ersten Start ggf. **Rechtsklick -> Öffnen** verwenden oder unter *Systemeinstellungen -> Datenschutz & Sicherheit* freigeben.
- **Android:** APK herunterladen und Installation aus unbekannten Quellen für den Browser/Dateimanager erlauben. Durch den stabilen Signierschlüssel können künftige Updates nahtlos über die bestehende Installation installiert werden. Exporte landen direkt unter `/sdcard/Download/TermiGrid/`.

---

### Kurzanleitung

1. **Klemmen hinzufügen:** In der rechten Spalte Hersteller und Typ wählen (oder Suchfeld, Leistenvorlagen bzw. Schnell-Eingabe-Assistent nutzen, z. B. `4x Drehstrom Abgang 2,5mm²`).
2. **EPLAN Import & Roundtrip:** Klemmen per CLI-Parameter (`--import-eplan-csv`), manuellem CSV-Import oder Drag & Drop von PDF-Stücklisten laden.
3. **CAD-Bearbeitung & Brücken-Modus:** Doppelklick auf Beschriftungen zum Ändern von Texten, Umschalten in den Brücken-Modus für farbliche Potentiale per Klick & Ziehen.
4. **Automatische Endplatten & Zubehör:** Abschlussplatten, Trennplatten, Kammbrücken mit Pin-Kürzungsanweisung und Endwinkel werden vollautomatisch ermittelt.
5. **Export & Druck:** Export als 4-Blatt-Excel (`.xlsx`), CSV oder 1-Klick EPLAN Rückimport-Paket (`_EPLAN_Zubehoer_Import.csv` & `.xml` + DXF-Zeichnungen).

---

### ⚡ EPLAN 2025 & P8 Roundtrip: Schritt-für-Schritt-Anleitung

Die nahtlose EPLAN-Anbindung erfordert nur zwei einmalige Konfigurationsschritte in EPLAN und läuft danach vollautomatisch per 1-Klick:

#### 1. Einmalige Einrichtung: Menüband-Button in EPLAN 2025
1. In EPLAN 2025: **`Datei → Einstellungen → Benutzer → Verwaltung → Menüband`** aufrufen.
2. Eine neue Aktionsgruppe anlegen (z. B. `TermiGrid` innerhalb der Registerkarte `Klemmenleisten`).
3. Einen neuen Menübefehl hinzufügen:
   - **Aktion:** `ExecuteScript`
   - **Parameter:** `/ScriptFile:"C:\Tools\TermiGrid\TermiGrid_EPLAN_Bridge.cs"`
   - **Text / QuickInfo:** `Zu TermiGrid senden`
4. Das fertige C#-Skript `TermiGrid_EPLAN_Bridge.cs` (siehe unten) an diesem Pfad abspeichern (oder in TermiGrid unter `Menü → EPLAN-Schnittstelle... → EPLAN C#-Skript exportieren` mit vorbefülltem Pfad speichern).

#### 2. Einmalige Einrichtung: EPLAN-Beschriftungsschema (CSV)
Unter **`Dienstprogramme → Beschriftung → Ausgeben...`** ein Klemmen-Schema anlegen (Semikolon `;` getrennt, UTF-8 mit/ohne BOM):
- Benötigte Spalten: `BMK`, `Klemmennummer` (oder `TerminalNo`), `Etage` (oder `Level`), `Artikelnummer` (oder `ArticleNo`), `Typnummer` (oder `TypeNo`), `Menge` (oder `Qty`).
- Konfigurierbare Hersteller-Präfixe wie `PXC.`, `WAG.`, `WDM.` werden von TermiGrid automatisch erkannt und beim Abgleich gegen die interne Datenbank entfernt.

#### 3. Tägliches Arbeiten (1 Klick)
- Klick auf den Menüband-Button ruft `TermiGrid.exe --import-eplan-csv "..."` auf.
- TermiGrid öffnet sich sofort mit allen Klemmenleisten, exakter Klemmenreihenfolge und Klemmenbezeichnung.

#### 4. Zubehör-Rückimport in EPLAN 2025 (Wichtig: Modus „Hinzufügen“)
1. In TermiGrid: **`Menü → EPLAN-Schnittstelle... → EPLAN Rückimport-Paket erstellen`** klicken.
2. TermiGrid berechnet automatisch alle fehlenden Abschluss-/Zwischenplatten und strikt markenreine Endhalter und speichert `_EPLAN_Zubehoer_Import.csv` & `.xml` (nur das neu hinzugekommene Zubehör mit wieder angehängten Hersteller-Präfixen).
3. In EPLAN 2025: **`Projektdaten → Geräte → Importieren...`** aufrufen und die CSV/XML-Datei auswählen.
4. **WICHTIGER PRAXIS-TIPP:** Als Importmodus zwingend **„Hinzufügen“** wählen (NICHT „Überschreiben“!). Dadurch bleiben bestehende Klemmen und Schaltpläne vollkommen unberührt, und EPLAN ordnet das Zubehör anhand des vollständigen BMK sauber der jeweiligen Klemmleiste zu.

#### 5. DXF-Aufbaubild in EPLAN 2025 platzieren
- TermiGrid erzeugt für jede Klemmleiste eine maßstabsgetreue 1:1 DXF-Datei im Unterordner `/DXF/<Sanitized_BMK>.dxf`.
- **Tipp für Aufbauseiten:** Verwenden Sie in EPLAN **`Einfügen → Grafik → DXF/DWG`**, um das DXF direkt als Grafikblock auf jede bestehende Schaltschrank-Aufbauseite einzufügen! (Der Menüpunkt `Seite → Importieren → DXF/DWG` legt hingegen eine komplett neue Seite an).

<details>
<summary><strong>📄 Fertiges C#-Bridge-Skript: <code>TermiGrid_EPLAN_Bridge.cs</code> (Hier ausklappen zum Kopieren)</strong></summary>

```csharp
// TermiGrid <-> EPLAN Electric P8 Roundtrip Bridge Script
// Automatisch generiert von TermiGrid.
// In EPLAN unter Datei -> Einstellungen -> Benutzer -> Verwaltung -> Menüband als Aktion "ExecuteScript" einbinden.

using System;
using System.Diagnostics;
using System.IO;
using System.Windows.Forms;
using Eplan.EplApi.ApplicationFramework;
using Eplan.EplApi.Scripting;

public class TermiGridEplanBridge
{
    private const string TermiGridExePath = @"C:\Tools\TermiGrid\TermiGrid.exe";
    private const string DefaultRoundtripDir = @"";

    [Start]
    public void RunTermiGridRoundtrip()
    {
        try
        {
            if (!File.Exists(TermiGridExePath))
            {
                MessageBox.Show(
                    "TermiGrid.exe wurde nicht gefunden:\n" + TermiGridExePath,
                    "TermiGrid EPLAN Bridge",
                    MessageBoxButtons.OK,
                    MessageBoxIcon.Warning
                );
                return;
            }

            string csvPath = SelectOrFindExportCsv();
            if (string.IsNullOrEmpty(csvPath))
            {
                return;
            }

            var psi = new ProcessStartInfo
            {
                FileName = TermiGridExePath,
                Arguments = "--import-eplan-csv \"" + csvPath + "\"",
                UseShellExecute = false
            };
            Process.Start(psi);
        }
        catch (Exception ex)
        {
            MessageBox.Show(
                "Fehler beim Starten von TermiGrid:\n" + ex.Message,
                "TermiGrid EPLAN Bridge",
                MessageBoxButtons.OK,
                MessageBoxIcon.Error
            );
        }
    }

    private static string SelectOrFindExportCsv()
    {
        using (var dlg = new OpenFileDialog())
        {
            dlg.Title = "EPLAN Klemmen-Export CSV für TermiGrid auswählen";
            dlg.Filter = "EPLAN CSV Export (*.csv)|*.csv|Alle Dateien (*.*)|*.*";
            if (!string.IsNullOrEmpty(DefaultRoundtripDir) && Directory.Exists(DefaultRoundtripDir))
            {
                dlg.InitialDirectory = DefaultRoundtripDir;
            }
            return dlg.ShowDialog() == DialogResult.OK ? dlg.FileName : null;
        }
    }
}
```
</details>


---

### Selbst kompilieren (Build from Source)

Voraussetzung: aktuelle Rust-Toolchain (`rustup` / `cargo`).

```bash
# Qualitätsprüfung & Tests
cargo clippy --all-targets -- -D warnings
cargo test

# Desktop Release-Build (Windows / Linux / macOS nativ)
cargo build --release --bin termigrid

# macOS Universal (Intel + Apple Silicon)
cargo build --release --bin termigrid --target x86_64-apple-darwin
cargo build --release --bin termigrid --target aarch64-apple-darwin

# Android Universal APK (mit installiertem cargo-apk & Android NDK)
cargo apk build --release --lib
```

---

### Links & Unterstützung

- **GitHub Repository:** https://github.com/PatrickBarbian/TermiGrid
- **Releases & Downloads:** https://github.com/PatrickBarbian/TermiGrid/releases
- **Support / Buy Me a Coffee:** https://buymeacoffee.com/02eOWKzm5W

---

### Lizenz

Veröffentlicht unter der **MIT-Lizenz**.

---

## 🌐 Alle weiteren Sprachen direkt hier auf der Startseite umschalten

Klicke auf eine Sprache, um die vollständige Dokumentation direkt hier aufzuklappen (alternativ über die Einzeldateien unter `docs/`):

<a id="lang-en"></a>
<details>
<summary><strong>🇬🇧 English — Complete Documentation (Click to expand / Standalone: docs/README.en.md)</strong></summary>

### Overview & VibeCoded Note

**TermiGrid** is a fast, cross-platform desktop and mobile engineering tool (written in Rust Edition 2024 with `egui` / `eframe 0.37`) for planning, visualizing, and configuring DIN-rail terminal strips in electrical control cabinet manufacturing. It bridges the gap between electrical CAD (EPLAN Electric P8), interactive 2D workshop layout planning, physical accessory calculation (end plates, comb jumpers with cut pins, end brackets), and industrial printer marking systems (WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO).

> **✨ VibeCoded:** This project is **VibeCoded** – designed, engineered, and continuously refined with **AI assistance** in close collaboration with the project author.

*Standalone file:* [docs/README.en.md](docs/README.en.md)

### Key Features

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

### Downloads & Portability

| Platform | Architecture / Target | File | Notes |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100% portable** (single `.exe` built with `+crt-static` – no installer, no VCRUNTIME needed) |
| **macOS** | Universal (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Drag the `.app` bundle into Applications |
| **Android** | Universal (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphones, tablets & foldables (Android 7.0 – 15 / API 24–35, 16 KB page-aligned, v1/v2/v3 signed) |

### Quick Guide

1. **Add Terminals:** Choose manufacturer and terminal type in the right panel (or use search, strip templates, or the natural-language Quick-Prompt Assistant, e.g. `4x Drehstrom Abgang 2,5mm²`).
2. **EPLAN Import & Roundtrip:** Import terminal data via CLI argument (`--import-eplan-csv`), manual CSV import, or PDF parts list drag & drop.
3. **CAD Editing & Bridge Mode:** Double-click any label to modify text, switch to Bridge Mode to paint potentials across tiers by clicking and dragging.
4. **Automatic Accessories & End Plates:** End and partition plates, comb jumpers with cut pins, and end brackets are automatically computed.
5. **Export & Print:** Export 4-sheet Excel spreadsheets (`.xlsx`) or CSV, or generate a 1-click EPLAN Return Package (`_EPLAN_Zubehoer_Import.csv` & `.xml` + DXF drawings).

### Build from Source

```bash
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release --bin termigrid
```

</details>

<a id="lang-fr"></a>
<details>
<summary><strong>🇫🇷 Français — Documentation complète (Cliquer pour ouvrir / Fichier dédié : docs/README.fr.md)</strong></summary>

### Aperçu & Note VibeCoded

**TermiGrid** est un outil d'ingénierie rapide et multiplateforme (écrit en Rust Édition 2024 avec `egui` / `eframe 0.37`) pour planifier, visualiser et configurer des borniers sur rail DIN dans la construction d'armoires électriques. Il fait le lien entre la CAO électrique (EPLAN Electric P8), la planification 2D interactive d'atelier, le calcul physique précis des accessoires (flasques d'extrémité, peignes de pontage avec broches coupées, butées d'arrêt) et les systèmes industriels d'impression de repères (WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO).

> **✨ VibeCoded :** Ce projet est **VibeCoded** – conçu, développé et perfectionné avec l'**assistance de l'IA** en étroite collaboration avec l'auteur du projet.

*Fichier dédié :* [docs/README.fr.md](docs/README.fr.md)

### Fonctionnalités principales

- **782+ références de bornes vérifiées + 26 butées d'arrêt pour rail DIN + bornes personnalisées :**
  - **WAGO :** TOPJOB S (`2000–2016` standard, `2200–2216` à poussoir, `2102–2116` à levier), bornes à trois étages `2002-32xx` avec PE au niveau inférieur, bornes d'installation (`2003`, `2005`, `2203`, `2205`), `2022` X-COM, `2007` mesure/sectionnement, `280/281` classique et `285` fort courant (`35–95 mm²`).
  - **Phoenix Contact :** COMPLETE line, Push-in `PT 1,5/S` à `PT 35`, bornes capteur/actionneur `PTIO`, `PTPOWER 35–95`, double étage `PTTB`/`PTTBS`, connexion latérale `PTV`, bornes d'installation `PTI` / `UTN` / `UTI`, sectionnables (`QTC 1,5-TG`, `URTK/SP`, `URTK 6`), ressort `ST`, vis `UT`, `UK 5 N`.
  - **Weidmüller :** Klippon Connect Série A (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), bornes d'installation `AITB`, Série P de répartition et potentiel (`PPV 4/8/16` & `PRV 4/8/16` avec étiquette unique par borne), Série W (`WDU/WPE/WDK/WTR/WSI/DLI`), Série Z (`ZDU/ZPE/ZDK/ZDL`).
  - **26 butées d'arrêt pour rail TS 35 :** Butées de grands constructeurs (WAGO 249-116/117, Phoenix Contact CLIPFIX 35 / E/UK, Weidmüller WEW 35/2, etc.).
  - **Bornes personnalisées & Bibliothèque de modèles :** Éditeur visuel de bornes (`custom_specs.json`) et bibliothèque de borniers modèles (`strip_library.json`) pour sauvegarder vos configurations récurrentes.

- **Mise à jour directe de la base via GitHub (Hot-Reload sans redémarrer) :**
  - Détection automatique et non bloquante au démarrage des mises à jour de la base `terminal_specs.json` sur GitHub.
  - Bouton bien visible « Mise à jour de la base disponible » : téléchargement en 1 clic, validation, stockage local et rechargement à chaud immédiat sans réinstaller ni recompiler l'application.

- **Assistant de saisie rapide local (Quick-Prompt, 100 % hors-ligne sans LLM cloud) :**
  - Saisie intuitive en texte clair : ex. `4x Drehstrom Abgang 2,5mm² Installationsklemmen`, `4x 3-pol PTI 2,5`, `6x +24V QUATTRO gebrückt`, `2x Reserve leer`.
  - Prévisualisation instantanée et insertion par lot dans le bornier actif en un clic.

- **Passerelle EPLAN Electric P8 & Aller-retour complet :**
  - **Import CSV en ligne de commande :** Lancement avec `TermiGrid.exe --import-eplan-csv "export.csv"` ou via `Menu → Importer CSV EPLAN...` avec nettoyage automatique des préfixes fabricants configurables (`PXC.`, `WAG.`, `WDM.`).
  - **Import intelligent de nomenclatures PDF EPLAN :** Glisser-déposer de listes PDF, détection automatique des bornes d'installation, barres N et alimentations avec calcul des supports de barre collectrice (tous les 200 mm).
  - **Paquet retour EPLAN en 1 clic :** Génération de `_EPLAN_Zubehoer_Import.csv` & `.xml` (avec réapplication automatique des préfixes fabricants, sans mélange de marques) et plans DXF individuels par bornier (`/DXF/<BMK>.dxf`) prêts pour l'import de pages EPLAN.
  - **Script passerelle C# pour EPLAN :** Export de `TermiGrid_EPLAN_Bridge.cs` en un clic pour intégration directe dans les barres d'outils d'EPLAN P8.
  - **Boîte de dialogue discrète :** Accessible sous `Menu → Interface EPLAN...` pour configurer préfixes, délimiteurs et dossiers d'export.

- **Éditeur CAO, Mode d'édition automatique & Génération de flasques :**
  - Vue CAO 2D à l'échelle avec affichage des flasques d'extrémité (D) et de séparation selon leur largeur réelle.
  - **Mode d'édition automatique :** S'active automatiquement dès qu'une borne est modifiée dans la CAO.
  - **Calcul automatique des flasques :** Générées et actualisées lors de l'export, de la vue e-Planer ou de l'aller-retour EPLAN.
  - Édition des repères par double-clic (gestion des repères vides pour préserver les espacements d'impression), défilement tactile & clic milieu, zoom fluide Ctrl+molette.

- **Mode Pontage, Cascade double chambre & Instructions de broches coupées :**
  - Création de potentiels colorés (`+24V`, `GND`, `L1` …) et affectation par glisser-déposer sur les étages.
  - Calcul réel des peignes de pontage avec cascade sur chambres 1 et 2.
  - Instructions physiques de coupe de broches (`PIN ENTFERNEN BEI`) pour les bornes enjambées.
  - Ponts filaires automatiques (`DB`) lors des changements d'étage ou de pas.

- **Guide intégré avec recherche avancée & Générateur de prompt IA (Onglet `Guide`) :**
  - 13 sections détaillées avec recherche plein texte multi-mots, filtres par catégorie et raccourcis.
  - Procédure pas à pas pour ajouter des bornes localement (éditeur visuel ou JSON).
  - Modèle de prompt prêt à copier pour les IA (ChatGPT, Claude...) afin de générer des fichiers JSON de bornes sans erreur.

- **Export prêt pour l'impression (`.xlsx` & `.csv`) :**
  - 4 feuilles Excel spécialisées :
    1. `Druckdaten (Flach)` (cellules au format texte strict `@` pour WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO)
    2. `Etagen-Druckstreifen` (bandes continues par niveau E1..E4)
    3. `Bestückungsplan (Matrix)`
    4. `Stückliste & Montage` (liste de pièces agrégée, flasques, peignes avec broches coupées, butées et longueur totale de rail DIN)

- **Cache disque hors-ligne des photos & CDN vérifiés :**
  - Photos fabricants officielles via CDN sans blocage (WAGO, Phoenix Contact CaaS, Weidmüller DAM).
  - Cache local persistant (`photo_cache`) – fonctionne entièrement hors-ligne après le premier chargement.

- **Multiplateforme, 7 langues & Thèmes :**
  - Windows (`.exe` portable), macOS (`.dmg` Universel), Android (`.apk` Universel), Linux.
  - 7 langues : Allemand, Anglais, Français, Italien, Espagnol, Polonais, Néerlandais.
  - Thème Système / Clair / Sombre, Annuler/Rétablir (`Ctrl+Z` / `Ctrl+Y`), raccourcis (`Alt+Gauche/Droite`, `Ctrl+D`).

### Téléchargements & Portabilité

| Plateforme | Architecture / Cible | Fichier | Remarques |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100 % portable** (exécutable unique compilé avec `+crt-static` – sans installation ni VCRUNTIME) |
| **macOS** | Universel (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Glisser le bundle `.app` dans Applications |
| **Android** | Universel (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphones, tablettes & pliables (Android 7.0 – 15 / API 24–35, alignement mémoire 16 Ko, signature v1/v2/v3) |

### Guide rapide

1. **Ajouter des bornes :** Choisir le fabricant et le modèle dans le panneau de droite (ou utiliser la recherche, la bibliothèque de modèles ou l'assistant de saisie rapide, ex. `4x Drehstrom Abgang 2,5mm²`).
2. **Import & Aller-retour EPLAN :** Importer les bornes par ligne de commande (`--import-eplan-csv`), import manuel CSV ou glisser-déposer de listes PDF.
3. **Édition CAO & Mode Pontage :** Double-cliquer sur un repère pour éditer le texte, passer en Mode Pontage pour tracer des potentiels au clic-glisser.
4. **Accessoires automatiques & Flasques :** Flasques d'extrémité, peignes avec broches coupées et butées sont calculés automatiquement.
5. **Export & Impression :** Exporter en classeur Excel à 4 feuilles (`.xlsx`), CSV ou générer le paquet retour EPLAN en 1 clic (`_EPLAN_Zubehoer_Import.csv` & `.xml` + DXF).

### Compiler depuis les sources

```bash
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release --bin termigrid
```

</details>

<a id="lang-it"></a>
<details>
<summary><strong>🇮🇹 Italiano — Documentazione completa (Clicca per espandere / File dedicato: docs/README.it.md)</strong></summary>

### Panoramica & Nota VibeCoded

**TermiGrid** è uno strumento di ingegneria rapido e multipiattaforma (sviluppato in Rust Edizione 2024 con `egui` / `eframe 0.37`) per progettare, visualizzare e configurare morsettiere su guida DIN nei quadri elettrici. Collega in modo fluido il CAD elettrico (EPLAN Electric P8), la pianificazione 2D interattiva da officina, il calcolo fisico preciso degli accessori (piastre terminali, ponticelli a pettine con pin rimossi, fermamorsetti) e i sistemi industriali di stampa siglature (WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO).

> **✨ VibeCoded:** Questo progetto è **VibeCoded** – ideato, sviluppato e costantemente perfezionato con il **supporto dell'IA** in stretta collaborazione con l'autore del progetto.

*File dedicato:* [docs/README.it.md](docs/README.it.md)

### Funzionalità principali

- **782+ specifiche di morsetti verificate + 26 fermamorsetti per guida DIN + morsetti personalizzati:**
  - **WAGO:** TOPJOB S (`2000–2016` standard, `2200–2216` con pulsante, `2102–2116` con leva), morsetti a tre piani `2002-32xx` con PE al piano inferiore, morsetti per installazione (`2003`, `2005`, `2203`, `2205`), `2022` X-COM, `2007` prova/sezionamento, `280/281` classico e `285` alta corrente (`35–95 mm²`).
  - **Phoenix Contact:** COMPLETE line, Push-in da `PT 1,5/S` a `PT 35`, morsetti per sensori/attuatori `PTIO`, `PTPOWER 35–95`, doppio piano `PTTB`/`PTTBS`, connessione laterale `PTV`, morsetti per installazione `PTI` / `UTN` / `UTI`, sezionabili (`QTC 1,5-TG`, `URTK/SP`, `URTK 6`), molla `ST`, vite `UT`, `UK 5 N`.
  - **Weidmüller:** Klippon Connect Serie A (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), morsetti per installazione `AITB`, Serie P di smistamento e potenziale (`PPV 4/8/16` & `PRV 4/8/16` con siglatura a targhetta singola per morsetto), Serie W (`WDU/WPE/WDK/WTR/WSI/DLI`), Serie Z (`ZDU/ZPE/ZDK/ZDL`).
  - **26 Fermamorsetti per guida TS 35:** Supporti terminali originali dei principali produttori (WAGO 249-116/117, Phoenix Contact CLIPFIX 35 / E/UK, Weidmüller WEW 35/2, ecc.).
  - **Morsetti personalizzati & Libreria di modelli:** Editor visivo di morsetti (`custom_specs.json`) e libreria di morsettiere modello (`strip_library.json`) per salvare e riutilizzare configurazioni ricorrenti.

- **Aggiornamento database live da GitHub (Hot-Reload senza riavvio):**
  - Controllo automatico e non bloccante all'avvio degli aggiornamenti per `terminal_specs.json` su GitHub.
  - Pulsante ben visibile «Aggiornamento database disponibile»: download in 1 clic, validazione, salvataggio locale e ricaricamento a caldo immediato senza reinstallare o ricompilare l'app.

- **Assistente locale per inserimento rapido (Quick-Prompt, 100% offline senza cloud LLM):**
  - Inserimento rapido in testo libero: es. `4x Drehstrom Abgang 2,5mm² Installationsklemmen`, `4x 3-pol PTI 2,5`, `6x +24V QUATTRO gebrückt`, `2x Reserve leer`.
  - Anteprima in tempo reale e inserimento cumulativo nella morsettiera attiva con un solo clic.

- **Integrazione EPLAN Electric P8 & Roundtrip completo:**
  - **Importazione CSV da CLI:** Avvio da riga di comando con `TermiGrid.exe --import-eplan-csv "export.csv"` o da `Menu → Importa CSV EPLAN...` con rimozione automatica dei prefissi fornitore configurabili (`PXC.`, `WAG.`, `WDM.`).
  - **Importazione intelligente distinte PDF da EPLAN:** Trascina elenchi parti PDF, riconoscimento automatico di morsetti per installazione, barre N e alimentazioni con calcolo dei supporti barra collettrice (ogni 200 mm).
  - **Pacchetto di ritorno EPLAN in 1 clic:** Genera `_EPLAN_Zubehoer_Import.csv` & `.xml` (con prefissi fornitore riapplicati automaticamente, senza mischiare marchi) e disegni DXF individuali per ogni morsettiera (`/DXF/<BMK>.dxf`) pronti per l'import di pagine EPLAN.
  - **Script bridge C# per EPLAN:** Esporta `TermiGrid_EPLAN_Bridge.cs` con un clic per l'integrazione diretta nelle barre degli strumenti di EPLAN P8.
  - **Dialogo impostazioni pulito:** Accessibile da `Menu → Interfaccia EPLAN...` per configurare prefissi, separatori e cartelle di esportazione.

- **Editor CAD, Modalità di modifica automatica & Generazione piastre terminali:**
  - Vista CAD 2D in scala reale con rappresentazione di piastre terminali (D) e separatori con spessore fisico esatto.
  - **Modalità di modifica automatica:** Si attiva da sola non appena un morsetto viene modificato nella vista CAD.
  - **Calcolo automatico piastre terminali:** Generate e aggiornate durante l'esportazione, la vista e-Planer o il roundtrip EPLAN.
  - Modifica testi con doppio clic (inclusa la conservazione di targhette vuote per spaziatori di stampa), scorrimento touch e clic centrale, zoom fluido Ctrl+rotellina.

- **Modalità Ponticelli, Cascata a doppia camera & Istruzioni pin rimossi:**
  - Creazione di potenziali colorati (`+24V`, `GND`, `L1` …) e assegnazione ai piani trascinando il mouse.
  - Calcolo reale dei ponticelli a pettine con cascata su camere 1 e 2.
  - Istruzioni fisiche per la rimozione dei pin (`PIN ENTFERNEN BEI`) sui morsetti saltati.
  - Ponticelli a filo automatici (`DB`) nei salti di piano o di passo.

- **Guida integrata con ricerca & Generatore di prompt IA (Scheda `Guida`):**
  - 13 argomenti dettagliati con ricerca full-text multi-chiave, filtri per categoria e scorciatoie.
  - Guida passo-passo per aggiungere morsetti personalizzati in locale (tramite editor o file JSON).
  - Template di prompt per modelli di IA (ChatGPT, Claude...) pronto per generare specifiche JSON verificate senza errori.

- **Esportazione pronta per la stampa (`.xlsx` & `.csv`):**
  - 4 fogli di lavoro Excel specializzati:
    1. `Druckdaten (Flach)` (celle rigorosamente in formato testo `@` per WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO)
    2. `Etagen-Druckstreifen` (strisce continue per ogni piano E1..E4)
    3. `Bestückungsplan (Matrix)`
    4. `Stückliste & Montage` (distinta base aggregata, piastre terminali, ponticelli con istruzioni di taglio, fermamorsetti e lunghezza totale guida DIN)

- **Cache disco offline per foto & CDN verificati:**
  - Foto prodotto ufficiali tramite CDN senza blocchi anti-bot (WAGO, Phoenix Contact CaaS, Weidmüller DAM).
  - Cache disco locale persistente (`photo_cache`) – utilizzabile al 100% offline dopo il primo caricamento.

- **Multipiattaforma, 7 lingue & Temi:**
  - Windows (`.exe` portabile), macOS (`.dmg` Universale), Android (`.apk` Universale), Linux.
  - 7 lingue: Tedesco, Inglese, Francese, Italiano, Spagnolo, Polacco, Olandese.
  - Tema di Sistema / Chiaro / Scuro, Annulla/Ripeti (`Ctrl+Z` / `Ctrl+Y`), scorciatoie (`Alt+Sinistra/Destra`, `Ctrl+D`).

### Download & Portabilità

| Piattaforma | Architettura / Target | File | Note |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100% portabile** (eseguibile singolo con `+crt-static` – nessuna installazione, nessun VCRUNTIME richiesto) |
| **macOS** | Universale (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Trascina il bundle `.app` in Applicazioni |
| **Android** | Universale (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphone, tablet & pieghevoli (Android 7.0 – 15 / API 24–35, allineamento pagine 16 KB, firma v1/v2/v3) |

### Guida rapida

1. **Aggiungere morsetti:** Scegli produttore e modello nella colonna destra (o usa la ricerca, i modelli di morsettiera o l'assistente per inserimento rapido, es. `4x Drehstrom Abgang 2,5mm²`).
2. **Import & Roundtrip EPLAN:** Importa i morsetti tramite argomento CLI (`--import-eplan-csv`), importazione manuale CSV o trascina distinte PDF.
3. **Modifica CAD & Modo Ponticelli:** Fai doppio clic su una siglatura per modificare il testo, passa alla modalità Ponticelli per tracciare potenziali trascinando il mouse.
4. **Accessori automatici & Piastre terminali:** Piastre terminali, separatori, ponticelli con pin tagliati e fermamorsetti vengono calcolati automaticamente.
5. **Esportazione & Stampa:** Esporta in cartelle di lavoro Excel a 4 fogli (`.xlsx`), CSV o crea il pacchetto di ritorno EPLAN in 1 clic (`_EPLAN_Zubehoer_Import.csv` & `.xml` + disegni DXF).

### Compilare dai sorgenti

```bash
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release --bin termigrid
```

</details>

<a id="lang-es"></a>
<details>
<summary><strong>🇪🇸 Español — Documentación completa (Clic para expandir / Archivo independiente: docs/README.es.md)</strong></summary>

### Descripción general y nota VibeCoded

**TermiGrid** es una herramienta de ingeniería rápida y multiplataforma (escrita en Rust Edición 2024 con `egui` / `eframe 0.37`) para diseñar, visualizar y configurar regleteros de bornes para carril DIN en el montaje de cuadros eléctricos. Une de forma fluida el CAD eléctrico (EPLAN Electric P8), la planificación 2D interactiva de taller, el cálculo físico riguroso de accesorios (tapas finales, puentes peine con pines cortados, topes finales) y los sistemas industriales de impresión de etiquetas (WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO).

> **✨ VibeCoded:** Este proyecto ha sido **VibeCoded** – diseñado, programado y perfeccionado con **asistencia de IA** en estrecha colaboración con el autor del proyecto.

*Archivo independiente:* [docs/README.es.md](docs/README.es.md)

### Funciones principales

- **782+ tipos de bornes verificados + 26 topes finales para carril DIN + bornes personalizados:**
  - **WAGO:** TOPJOB S (`2000–2016` estándar, `2200–2216` con pulsador, `2102–2116` con palanca), bornes de tres pisos `2002-32xx` con PE en el piso inferior, bornes de instalación (`2003`, `2005`, `2203`, `2205`), `2022` X-COM, `2007` medida/seccionamiento, `280/281` clásico y `285` alta corriente (`35–95 mm²`).
  - **Phoenix Contact:** COMPLETE line, Push-in de `PT 1,5/S` a `PT 35`, bornes para sensores/actuadores `PTIO`, `PTPOWER 35–95`, doble piso `PTTB`/`PTTBS`, conexión lateral `PTV`, bornes de instalación `PTI` / `UTN` / `UTI`, seccionables (`QTC 1,5-TG`, `URTK/SP`, `URTK 6`), resorte `ST`, tornillo `UT`, `UK 5 N`.
  - **Weidmüller:** Klippon Connect Serie A (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), bornes de instalación `AITB`, Serie P de distribución y potencial (`PPV 4/8/16` & `PRV 4/8/16` con marcador único por borne), Serie W (`WDU/WPE/WDK/WTR/WSI/DLI`), Serie Z (`ZDU/ZPE/ZDK/ZDL`).
  - **26 Topes finales para carril TS 35:** Topes terminales originales de fabricantes líderes (WAGO 249-116/117, Phoenix Contact CLIPFIX 35 / E/UK, Weidmüller WEW 35/2, etc.).
  - **Bornes personalizados & Biblioteca de plantillas:** Editor visual de bornes (`custom_specs.json`) y biblioteca de regleteros modelo (`strip_library.json`) para guardar y reutilizar configuraciones frecuentes.

- **Actualizador de base de datos en vivo desde GitHub (Hot-Reload sin reiniciar):**
  - Comprobación automática y no bloqueante al arrancar para detectar actualizaciones de `terminal_specs.json` en GitHub.
  - Botón destacado «Actualización de base de datos disponible»: descarga en 1 clic, validación, almacenamiento local y recarga en caliente instantánea sin reinstalar ni recompilar la aplicación.

- **Asistente local de entrada rápida (Quick-Prompt, 100% fuera de línea sin LLM en la nube):**
  - Entrada rápida en texto natural: ej. `4x Drehstrom Abgang 2,5mm² Installationsklemmen`, `4x 3-pol PTI 2,5`, `6x +24V QUATTRO gebrückt`, `2x Reserve leer`.
  - Vista previa en tiempo real e inserción por lote en el regletero activo con un solo clic.

- **Integración con EPLAN Electric P8 & Roundtrip completo:**
  - **Importación CSV por CLI:** Ejecución desde línea de comandos con `TermiGrid.exe --import-eplan-csv "export.csv"` o desde `Menú → Importar CSV de EPLAN...` con eliminación automática de prefijos configurables (`PXC.`, `WAG.`, `WDM.`).
  - **Importación inteligente de listas de piezas PDF de EPLAN:** Arrastra archivos PDF, reconocimiento automático de bornes de instalación, barras colectoras de neutro y alimentaciones con cálculo de soportes (cada 200 mm).
  - **Paquete de retorno EPLAN en 1 clic:** Genera `_EPLAN_Zubehoer_Import.csv` y `.xml` (con prefijos de fabricante restaurados automáticamente, sin mezclar marcas) y planos DXF individuales por regletero (`/DXF/<BMK>.dxf`) listos para importación de páginas DXF en EPLAN.
  - **Script puente en C# para EPLAN:** Exportación de `TermiGrid_EPLAN_Bridge.cs` en un clic para integración directa en la barra de herramientas de EPLAN P8.
  - **Ajustes de EPLAN ordenados:** Acceso discreto desde `Menú → Interfaz EPLAN...` para configurar prefijos, separadores y carpetas de destino.

- **Editor CAD, Modo de edición automático & Generación de tapas finales:**
  - Vista CAD 2D a escala real con representación de tapas finales (D) y placas separadoras con su grosor físico exacto.
  - **Modo de edición automático:** Se activa automáticamente al realizar cualquier cambio en el CAD.
  - **Cálculo automático de tapas:** Se calculan y actualizan al exportar, en la vista e-Planer o durante el roundtrip con EPLAN.
  - Edición de textos con doble clic (admite etiquetas vacías para reservar espacio en impresoras), desplazamiento táctil y con botón central, zoom fluido Ctrl+rueda.

- **Modo Puentes, Cascada de doble cámara & Instrucciones de corte de pines:**
  - Crea potenciales por color (`+24V`, `GND`, `L1` …) y asígnalos arrastrando sobre los niveles de los bornes.
  - Cálculo real de puentes peine con cascada en cámaras 1 y 2.
  - Instrucciones físicas de corte de pines (`PIN ENTFERNEN BEI`) en bornes saltados.
  - Puentes de cable automáticos (`DB`) al cambiar de piso o de paso entre bornes.

- **Guía integrada con buscador & Generador de prompt para IA (Pestaña `Guía`):**
  - 13 temas detallados con búsqueda de texto completo multitérmino, filtros por categoría y accesos rápidos.
  - Instrucciones paso a paso para añadir bornes personalizados en local (editor visual o JSON).
  - Plantilla de prompt lista para copiar en herramientas de IA (ChatGPT, Claude...) y generar archivos JSON de bornes sin fallos.

- **Exportación lista para impresora (`.xlsx` y `.csv`):**
  - 4 hojas de cálculo Excel especializadas:
    1. `Druckdaten (Flach)` (celdas estrictamente en formato de texto `@` para WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO)
    2. `Etagen-Druckstreifen` (tiras continuas por piso E1..E4)
    3. `Bestückungsplan (Matrix)`
    4. `Stückliste & Montage` (lista de piezas agregada, tapas finales, puentes peine con pines cortados, topes finales y longitud total de carril DIN)

- **Caché en disco fuera de línea para fotos & CDN verificados:**
  - Fotos oficiales de productos mediante CDN sin bloqueos anti-bot (WAGO, Phoenix Contact CaaS, Weidmüller DAM).
  - Caché local persistente en disco (`photo_cache`) – funciona al 100% sin conexión tras la primera descarga.

- **Multiplataforma, 7 idiomas y Temas:**
  - Windows (`.exe` portable), macOS (`.dmg` Universal), Android (`.apk` Universal), Linux.
  - 7 idiomas: Alemán, Inglés, Francés, Italiano, Español, Polaco, Holandés.
  - Tema del Sistema / Claro / Oscuro, Deshacer/Rehacer (`Ctrl+Z` / `Ctrl+Y`), atajos (`Alt+Izquierda/Derecha`, `Ctrl+D`).

### Descargas y portabilidad

| Plataforma | Arquitectura / Destino | Archivo | Notas |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100 % portable** (ejecutable único compilado con `+crt-static` – sin instalación ni VCRUNTIME) |
| **macOS** | Universal (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Arrastra el paquete `.app` a Aplicaciones |
| **Android** | Universal (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphones, tablets y plegables (Android 7.0 – 15 / API 24–35, alineación de página de 16 KB, firma v1/v2/v3) |

### Guía rápida

1. **Añadir bornes:** Selecciona fabricante y modelo en el panel derecho (o usa la búsqueda, plantillas de regleteros o el asistente de entrada rápida, ej. `4x Drehstrom Abgang 2,5mm²`).
2. **Importación y Roundtrip EPLAN:** Importa bornes por argumento CLI (`--import-eplan-csv`), importación manual CSV o arrastra listas de piezas PDF.
3. **Edición CAD y Modo Puentes:** Haz doble clic en una etiqueta para modificar el texto, cambia a Modo Puentes para trazar potenciales arrastrando con el ratón.
4. **Accesorios automáticos y Tapas:** Tapas finales, separadores, puentes peine con pines cortados y topes terminales se calculan automáticamente.
5. **Exportación e Impresión:** Exporta a hojas de cálculo Excel de 4 páginas (`.xlsx`), CSV o genera el paquete de retorno EPLAN en 1 clic (`_EPLAN_Zubehoer_Import.csv` y `.xml` + DXF).

### Compilar desde el código fuente

```bash
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release --bin termigrid
```

</details>

<a id="lang-pl"></a>
<details>
<summary><strong>🇵🇱 Polski — Pełna dokumentacja (Kliknij, aby rozwinąć / Osobny plik: docs/README.pl.md)</strong></summary>

### Przegląd i informacja VibeCoded

**TermiGrid** to szybkie, wieloplatformowe narzędzie inżynieryjne (napisane w Rust Edycja 2024 z `egui` / `eframe 0.37`) do projektowania, wizualizacji i konfigurowania listew zaciskowych na szynie DIN w prefabrykacji szaf sterowniczych. Łączy ono świat projektowania CAD (EPLAN Electric P8), interaktywnego planowania 2D na warsztacie, precyzyjnego fizycznego doboru akcesoriów (płytki końcowe, mostki grzebieniowe z wyłamywanymi pinami, trzymacze końcowe) oraz przemysłowych systemów drukowania oznaczników (WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO).

> **✨ VibeCoded:** Ten projekt powstał jako **VibeCoded** – został zaprojektowany, zaprogramowany i dopracowany z **pomocą AI** w ścisłej współpracy z autorem projektu.

*Osobny plik:* [docs/README.pl.md](docs/README.pl.md)

### Główne funkcje

- **782+ zweryfikowanych typów złączek + 26 trzymaczy końcowych szyny DIN + własne złączki:**
  - **WAGO:** TOPJOB S (`2000–2016` standardowe, `2200–2216` z przyciskiem, `2102–2116` z dźwignią), złączki trzypiętrowe `2002-32xx` z PE na dolnym piętrze, złączki instalacyjne (`2003`, `2005`, `2203`, `2205`), `2022` X-COM, `2007` rozłączno-pomiarowe, `280/281` klasyczne oraz `285` wysokoprądowe (`35–95 mm²`).
  - **Phoenix Contact:** COMPLETE line, Push-in od `PT 1,5/S` do `PT 35`, złączki czujnik/element wykonawczy `PTIO`, `PTPOWER 35–95`, dwupiętrowe `PTTB`/`PTTBS`, boczne wprowadzanie przewodu `PTV`, złączki instalacyjne `PTI` / `UTN` / `UTI`, rozłączne (`QTC 1,5-TG`, `URTK/SP`, `URTK 6`), sprężynowe `ST`, śrubowe `UT`, `UK 5 N`.
  - **Weidmüller:** Klippon Connect Seria A (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), złączki instalacyjne `AITB`, Seria P rozdzielcza i potencjałowa (`PPV 4/8/16` & `PRV 4/8/16` z pojedynczym oznacznikiem na złączkę), Seria W (`WDU/WPE/WDK/WTR/WSI/DLI`), Seria Z (`ZDU/ZPE/ZDK/ZDL`).
  - **26 Trzymaczy końcowych na szynę TS 35:** Oryginalne trzymacze końcowe czołowych producentów (WAGO 249-116/117, Phoenix Contact CLIPFIX 35 / E/UK, Weidmüller WEW 35/2 itd.).
  - **Własne złączki & Biblioteka szablonów listew:** Wizualny edytor złączek (`custom_specs.json`) oraz biblioteka szablonów listew (`strip_library.json`) do zapisywania i ponownego wykorzystywania częstych konfiguracji.

- **Aktualizator bazy danych na żywo z GitHub (Hot-Reload bez restartu):**
  - Automatyczne i nieblokujące sprawdzanie w tle dostępności nowej wersji bazy `terminal_specs.json` na GitHubie przy starcie.
  - Wyraźny przycisk „Dostępna aktualizacja bazy danych”: pobieranie 1 kliknięciem, weryfikacja, zapis lokalny i natychmiastowe przeładowanie w trakcie pracy bez reinstalacji i przebudowywania aplikacji.

- **Lokalny asystent szybkiego wprowadzania (Quick-Prompt, 100% offline bez chmurowego LLM):**
  - Błyskawiczne wprowadzanie tekstem naturalnym: np. `4x Drehstrom Abgang 2,5mm² Installationsklemmen`, `4x 3-pol PTI 2,5`, `6x +24V QUATTRO gebrückt`, `2x Reserve leer`.
  - Podgląd na żywo i natychmiastowe wstawianie partii złączek do aktywnej listwy jednym kliknięciem.

- **Integracja EPLAN Electric P8 & Pełny Roundtrip:**
  - **Import CSV z CLI:** Uruchamianie z linii poleceń `TermiGrid.exe --import-eplan-csv "export.csv"` lub z menu `Menu → Importuj CSV z EPLAN...` z automatycznym oczyszczaniem konfigurowalnych prefiksów producentów (`PXC.`, `WAG.`, `WDM.`).
  - **Inteligentny import zestawień PDF z EPLAN:** Przeciągnij listę części PDF, automatyczne rozpoznawanie złączek instalacyjnych, szyn zbiorczych N i zasilań wraz z obliczaniem wsporników szyny N (co 200 mm).
  - **Pakiet zwrotny EPLAN za 1 kliknięciem:** Generuje pliki `_EPLAN_Zubehoer_Import.csv` & `.xml` (z automatycznie przywróconymi prefiksami producentów, bez mieszania marek) oraz czyste rysunki DXF dla każdej listwy (`/DXF/<BMK>.dxf`) gotowe do importu stron DXF w EPLAN.
  - **Skrypt mostka C# dla EPLAN:** Eksport skryptu `TermiGrid_EPLAN_Bridge.cs` jednym kliknięciem do bezproblemowej integracji w pasku narzędzi EPLAN P8.
  - **Dyskretne okno ustawień:** Dostępne w `Menu → Interfejs EPLAN...` do konfiguracji prefiksów, separatorów i katalogów eksportu.

- **Edytor CAD, Automatyczny tryb edycji & Generowanie płytek końcowych:**
  - Widok CAD 2D w rzeczywistej skali z wyświetlaniem płytek końcowych (D) i przegród o ich faktycznej grubości.
  - **Automatyczny tryb edycji:** Włącza się samoczynnie przy modyfikacji złączki w widoku CAD.
  - **Automatyczne obliczanie płytek:** Płytki są generowane i aktualizowane podczas eksportu, w widoku e-Planera oraz w procesie powrotu do EPLAN.
  - Edycja etykiet przez dwuklik (obsługa pustych oznaczników do zachowania odstępów w drukarce), przewijanie dotykowe i środkowym przyciskiem, płynny zoom Ctrl+kółko myszy.

- **Tryb Mostków, Kaskada dwukomorowa & Instrukcje wyłamywania pinów:**
  - Tworzenie kolorowych potencjałów (`+24V`, `GND`, `L1` …) i przypisywanie ich do poziomów przeciągnięciem myszy.
  - Rzeczywiste obliczanie mostków grzebieniowych z kaskadą na komorach 1 i 2.
  - Fizyczne instrukcje usuwania pinów (`PIN ENTFERNEN BEI`) dla omijanych złączek.
  - Automatyczne mostki przewodowe (`DB`) przy zmianie poziomu lub rastra.

- **Wbudowany podręcznik z wyszukiwarką & Generator promptu AI (Karta `Instrukcja`):**
  - 13 szczegółowych tematów z wyszukiwaniem pełnotekstowym, filtrami kategorii i skrótami.
  - Instrukcja krok po kroku dodawania własnych złączek lokalnie (wizualnie lub przez plik JSON).
  - Gotowy szablon promptu dla modeli AI (ChatGPT, Claude...) do generowania bezbłędnych plików JSON ze złączkami.

- **Eksport gotowy do druku (`.xlsx` i `.csv`):**
  - 4 wyspecjalizowane arkusze Excel:
    1. `Druckdaten (Flach)` (komórki ściśle jako tekst `@` dla WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO)
    2. `Etagen-Druckstreifen` (ciągłe taśmy oznaczników dla każdego poziomu E1..E4)
    3. `Bestückungsplan (Matrix)`
    4. `Stückliste & Montage` (zbiorcza lista złączek, płytki końcowe, mostki z instrukcją wyłamywania pinów, trzymacze końcowe i całkowita długość szyny)

- **Dyskowa pamięć podręczna zdjęć offline & Zweryfikowane CDN:**
  - Oficjalne zdjęcia produktów pobierane z CDN bez blokad (WAGO, Phoenix Contact CaaS, Weidmüller DAM).
  - Trwała lokalna pamięć podręczna na dysku (`photo_cache`) – po pierwszym pobraniu działa w 100% offline.

- **Wieloplatformowość, 7 języków i Motywy:**
  - Windows (`.exe` przenośny), macOS (`.dmg` Uniwersalny), Android (`.apk` Uniwersalny), Linux.
  - 7 języków: Niemiecki, Angielski, Francuski, Włoski, Hiszpański, Polski, Holenderski.
  - Motyw Systemowy / Jasny / Ciemny, Cofnij/Ponów (`Ctrl+Z` / `Ctrl+Y`), skróty (`Alt+Lewo/Prawo`, `Ctrl+D`).

### Pobieranie i przenośność

| Platforma | Architektura / Cel | Plik | Uwagi |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100% przenośny** (pojedynczy plik `.exe` z `+crt-static` – bez instalacji i bez VCRUNTIME) |
| **macOS** | Uniwersalny (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Przeciągnij pakiet `.app` do folderu Aplikacje |
| **Android** | Uniwersalny (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartfony, tablety i urządzenia składane (Android 7.0 – 15 / API 24–35, wyrównanie stron 16 KB, podpis v1/v2/v3) |

### Szybka instrukcja

1. **Dodaj złączki:** W prawej kolumnie wybierz producenta i typ (lub użyj wyszukiwarki, szablonów listew albo asystenta szybkiego wprowadzania, np. `4x Drehstrom Abgang 2,5mm²`).
2. **Import i Roundtrip EPLAN:** Importuj złączki przez parametr CLI (`--import-eplan-csv`), ręczny import CSV lub przeciągnij zestawienie PDF.
3. **Edycja CAD & Tryb Mostków:** Kliknij dwukrotnie oznacznik, aby zmienić tekst, przełącz w Tryb Mostków, by malować potencjały przeciągnięciem myszy.
4. **Automatyczne akcesoria & Płytki końcowe:** Płytki końcowe, przegrody, mostki z wyłamywanymi pinami i trzymacze końcowe są wyliczane automatycznie.
5. **Eksport i Druk:** Eksportuj do 4-arkuszowego pliku Excel (`.xlsx`), CSV lub wygeneruj pakiet zwrotny EPLAN za 1 kliknięciem (`_EPLAN_Zubehoer_Import.csv` & `.xml` + rysunki DXF).

### Kompilacja ze źródeł

```bash
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release --bin termigrid
```

</details>

<a id="lang-nl"></a>
<details>
<summary><strong>🇳🇱 Nederlands — Volledige documentatie (Klik om uit te klappen / Los bestand: docs/README.nl.md)</strong></summary>

### Overzicht & VibeCoded-vermelding

**TermiGrid** is een snelle, platformonafhankelijke engineeringtool (geschreven in Rust Editie 2024 met `egui` / `eframe 0.37`) voor het ontwerpen, visualiseren en configureren van DIN-rail klemmenstroken in de schakelkastbouw. Het overbrugt naadloos het gat tussen elektrotechnisch CAD (EPLAN Electric P8), interactieve 2D-werkplaatsplanning, nauwkeurige fysieke accessoireberekening (eindplaten, kamverbinders met uitgebroken pinnen, eindsteunen) en industriële markeringsprinters (WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO).

> **✨ VibeCoded:** Dit project is **VibeCoded** – ontworpen, geprogrammeerd en verfijnd met **AI-ondersteuning** in nauwe samenwerking met de projectauteur.

*Los bestand:* [docs/README.nl.md](docs/README.nl.md)

### Belangrijkste functies

- **782+ geverifieerde klemtypes + 26 DIN-rail eindsteunen + eigen klemmen:**
  - **WAGO:** TOPJOB S (`2000–2016` standaard, `2200–2216` met drukknop, `2102–2116` met hendel), `2002-32xx` driepolige etageklemmen met PE op het onderste niveau, installatieklemmen (`2003`, `2005`, `2203`, `2205`), `2022` X-COM, `2007` meet-/scheidingsklemmen, `280/281` klassiek en `285` hoogstroom (`35–95 mm²`).
  - **Phoenix Contact:** COMPLETE line, Push-in `PT 1,5/S` t/m `PT 35`, sensor/actorklemmen `PTIO`, `PTPOWER 35–95`, dubbeldeks `PTTB`/`PTTBS`, zijwaartse aansluiting `PTV`, installatieklemmen `PTI` / `UTN` / `UTI`, scheidingsklemmen (`QTC 1,5-TG`, `URTK/SP`, `URTK 6`), veerkracht `ST`, schroef `UT`, `UK 5 N`.
  - **Weidmüller:** Klippon Connect A-serie (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), installatieklemmen `AITB`, P-serie ranger- en potentiaalverdelers (`PPV 4/8/16` & `PRV 4/8/16` met één coderingsplaatje per klem), W-serie (`WDU/WPE/WDK/WTR/WSI/DLI`), Z-serie (`ZDU/ZPE/ZDK/ZDL`).
  - **26 TS 35 Eindsteunen:** Passende eindsteunen van toonaangevende fabrikanten (WAGO 249-116/117, Phoenix Contact CLIPFIX 35 / E/UK, Weidmüller WEW 35/2 enz.).
  - **Eigen klemmen & Strooksjablonen-bibliotheek:** Visuele klemeditor (`custom_specs.json`) en sjablonenbibliotheek (`strip_library.json`) voor veelgebruikte klemmenstroken.

- **GitHub Live Database-updater (Hot-Reload zonder herstarten):**
  - Automatische en niet-blokkerende achtergrondcontrole bij het opstarten naar updates van `terminal_specs.json` op GitHub.
  - Duidelijke knop «Database-update beschikbaar»: 1-klik download, validatie, lokale opslag en live hot-reload zonder de app opnieuw te installeren of te herbouwen.

- **Lokale snel-invoerassistent (Quick-Prompt, 100% offline zonder cloud-LLM):**
  - Snelle invoer in gewone taal: bijv. `4x Drehstrom Abgang 2,5mm² Installationsklemmen`, `4x 3-pol PTI 2,5`, `6x +24V QUATTRO gebrückt`, `2x Reserve leer`.
  - Live-voorbeeld en directe invoeging in de actieve klemmenstrook met één klik.

- **EPLAN Electric P8 Koppeling & Volledige Roundtrip:**
  - **CLI CSV-import:** Opstarten via de opdrachtregel met `TermiGrid.exe --import-eplan-csv "export.csv"` of via `Menu → EPLAN CSV importeren...` met automatisch strippen van configureerbare fabrikantvoorvoegsels (`PXC.`, `WAG.`, `WDM.`).
  - **Intelligente EPLAN PDF-stuklijstimport:** Drag & drop van PDF-stuklijsten, automatische herkenning van installatieklemmen, N-verzamelrails en voedingen inclusief verzamelrailsteunberekening (elke 200 mm).
  - **1-Klik EPLAN Retourpakket:** Genereert `_EPLAN_Zubehoer_Import.csv` & `.xml` (met automatisch herstelde fabrikantvoorvoegsels, strikt merkzuiver) en nette afzonderlijke DXF-tekeningen per strook (`/DXF/<BMK>.dxf`) voor directe EPLAN DXF-pagina-import.
  - **EPLAN C#-Bridge-script:** Exporteer `TermiGrid_EPLAN_Bridge.cs` met één klik voor naadloze integratie in de EPLAN P8 werkbalk.
  - **Overzichtelijk instellingenvenster:** Discreet ondergebracht in `Menu → EPLAN-interface...` om voorvoegsels, scheidingstekens en exportmappen te beheren.

- **CAD-editor, Automatische bewerkingsmodus & Eindplaten genereren:**
  - Schaalgetrouwe 2D-CAD-weergave met weergave van eindplaten (D) en scheidingsplaten in hun werkelijke dikte.
  - **Automatische bewerkingsmodus:** Schakelt zichzelf automatisch in bij wijzigingen in het CAD-venster.
  - **Automatische eindplaatberekening:** Berekend en bijgewerkt bij export, in de e-Planer en tijdens de EPLAN roundtrip.
  - Teksten bewerken met dubbelklik (inclusief behoud van lege plaatjes voor tussenruimtes bij printen), vloeiend pannen via touch en middelste muisknop, zoomen met Ctrl+muiswiel.

- **Brug-modus, Dubbelkamer-cascade & Pin-afbreekaanwijzingen:**
  - Gekleurde potentialen (`+24V`, `GND`, `L1` …) aanmaken en per niveau toewijzen door slepen.
  - Reële kamverbinderberekening met trapsgewijze cascade over kamer 1 en 2.
  - Fysieke aanwijzingen voor het verwijderen van pinnen (`PIN ENTFERNEN BEI`) bij overgeslagen klemmen.
  - Automatische draadbruggen (`DB`) bij niveauwissels of steekverschillen.

- **Geïntegreerde handleiding met zoekfunctie & AI-promptgenerator (Tabblad `Handleiding`):**
  - 13 gedetailleerde onderwerpen met full-text zoeken op meerdere trefwoorden, categoriefilters en snelkoppelingen.
  - Stapsgewijze handleiding om eigen klemmen lokaal toe te voegen (visueel of via JSON).
  - Kopieerklare prompttemplate voor AI-modellen (ChatGPT, Claude...) om foutloze klemmen-JSON-bestanden te genereren.

- **Printklare export (`.xlsx` & `.csv`):**
  - 4 gespecialiseerde Excel-werkbladen:
    1. `Druckdaten (Flach)` (cellen strikt als tekst `@` voor WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO)
    2. `Etagen-Druckstreifen` (doorlopende coderingsstroken per etage E1..E4)
    3. `Bestückungsplan (Matrix)`
    4. `Stückliste & Montage` (verzamelstuklijst, eindplaten, kamverbinders met afbreekinstructies, eindsteunen en totale raillengte)

- **Offline fotocache op schijf & Geverifieerde CDN's:**
  - Originele productfoto's via betrouwbare CDN's (WAGO, Phoenix Contact CaaS, Weidmüller DAM).
  - Blijvende lokale schijfcache (`photo_cache`) – werkt na de eerste download 100% offline.

- **Platformonafhankelijk, 7 talen & Thema's:**
  - Windows (`.exe` portable), macOS (`.dmg` Universeel), Android (`.apk` Universeel), Linux.
  - 7 talen: Duits, Engels, Frans, Italiaans, Spaans, Pools, Nederlands.
  - Systeem / Licht / Donker thema, Ongedaan maken / Opnieuw (`Ctrl+Z` / `Ctrl+Y`), sneltoetsen (`Alt+Links/Rechts`, `Ctrl+D`).

### Downloads & Draagbaarheid

| Platform | Architectuur / Doel | Bestand | Opmerkingen |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100% portable** (los `.exe`-bestand met `+crt-static` – geen installatie en geen VCRUNTIME nodig) |
| **macOS** | Universeel (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Sleep de `.app`-bundel naar Programma's |
| **Android** | Universeel (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphones, tablets & foldables (Android 7.0 – 15 / API 24–35, 16 KB pagina-uitlijning, v1/v2/v3 ondertekend) |

### Snelstartgids

1. **Klemmen toevoegen:** Kies fabrikant en type in de rechterkolom (of gebruik het zoekveld, strooksjablonen of de snel-invoerassistent, bijv. `4x Drehstrom Abgang 2,5mm²`).
2. **Import & Roundtrip EPLAN:** Importeer klemmen via CLI (`--import-eplan-csv`), handmatige CSV-import of sleep PDF-stuklijsten in het venster.
3. **CAD bewerken & Brug-modus:** Dubbelklik op een codering om tekst aan te passen, schakel naar de Brug-modus om potentialen aan te brengen door te slepen.
4. **Automatische accessoires & Eindplaten:** Eindplaten, scheidingsplaten, kamverbinders met afbreekbare pinnen en eindsteunen worden automatisch berekend.
5. **Exporteren & Printen:** Exporteer naar 4-delige Excel-werkbladen (`.xlsx`), CSV of genereer het EPLAN retourpakket met 1 klik (`_EPLAN_Zubehoer_Import.csv` & `.xml` + DXF-tekeningen).

### Zelf compileren (Build from Source)

```bash
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release --bin termigrid
```

</details>
