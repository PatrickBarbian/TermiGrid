<div align="center">

<img src="assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**Reihenklemmen-Konfigurator & Etiketten-Exporter für den Schaltschrankbau**

[![Release](https://img.shields.io/github/v/release/PatrickBarbian/TermiGrid)](https://github.com/PatrickBarbian/TermiGrid/releases)
[![VibeCoded](https://img.shields.io/badge/VibeCoded-AI%20Assisted-blueviolet)](#vibe-coded)
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

**TermiGrid** ist ein plattformübergreifendes Werkzeug (geschrieben in Rust mit `egui` / `eframe 0.37`) zum Planen von Klemmenleisten im Schaltschrankbau und zum Exportieren druckfertiger Beschriftungslisten für professionelle Etiketten- und Markierungsdrucker.

> **✨ VibeCoded:** Dieses Projekt wurde **VibeCoded** – vollständig entworfen und entwickelt mit **KI-Unterstützung** in enger Zusammenarbeit mit dem Projektautor.

---

### Hauptfunktionen

- **277 hinterlegte Klemmentypen + eigene Klemmen:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (mit Betätigungsdrücker), `2102–2116` (Hebel), `2022` X-COM, `2007` Stromwandler, `280/281` sowie `285` Hochstrom (`35–95 mm²`)
  - **Phoenix Contact:** `PT 1,5/S` inkl. `PTIO`, `PT 2,5–35`, `PTPOWER 35–95`, `PTTB`, `PTTBS`, `PTV`, `PTI` / `UTI` Installationsklemmen, `ST`, `UT` & `UK 5 N`
  - **Weidmüller:** `A-Series` (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), `AITB` Installationsklemmen, `W-Series` (`WDU/WPE/WDK/WTR/WSI/DLI`) & `Z-Series` (`ZDU/ZPE/ZDK/ZDL`)
  - **Eigene Klemmen:** Eigene Klemmentypen inkl. `1×` oder `2×` Brückerkammern direkt in der App anlegen
- **Ebenen-basierte Nummerierung, N/PE-Modi & Sonderzeichen:**
  - Installations- und Mehrstockklemmen (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI` usw.): Ein vorhandener **PE-Anschluss** wird stets in einer eigenen PE-Zeile **unterhalb der unteren Hauptebene** ausgerichtet.
  - **N- & PE-Nummerierung:** Umschaltbar pro Leiste zwischen fest (`N`/`PE`), mit der Hauptfolge mitgezählt (`1, 2, …`), mitgezählt als `1N, 2N` / `1PE, 2PE` oder mit **eigenem Zähler (`1N, 2N …` / `1PE, 2PE …`)**.
  - **Präfix / Suffix / Sonderzeichen für Auswahl:** Beliebig viele Klemmen markieren und Sonderzeichen wie `+`, `-`, `~`, `=`, `.1`, `Ω` etc. vor oder nach der Zahl für alle oder gezielte Ebenen anwenden (bleibt beim Umnummerieren dynamisch erhalten).
- **Brücken-Modus & Potentialberechnung:**
  - Mehrere farbliche **Potentiale** anlegen (`+24V`, `GND`, `L1` …) und Klemmen-Ebenen direkt im CAD antippen/anklicken.
  - **Automatische Berechnung:** Ermittelt benötigte Kammbrücken-Polzahlen (z. B. `2002-405`, `FBS 5-5`, `ZQV`), markiert **herauszubrechende Pins (`Pin entfernen ×`)** bei übersprungenen Klemmen, plant **Drahtbrücken (`DB`)** bei Mehrstock-Ebenenwechseln (`E1 <-> E2`) oder Rasterwechseln und unterscheidet automatisch zwischen **1-Kammer-Klemmen** (Drahtbrücke ab Überschreitung der max. Zubehör-Polzahl) und **Doppelkammer-Klemmen** (überlappende Kaskadierung in Kammer 1 & 2).
- **Interaktiver 2D CAD-Editor (Desktop & Touch-optimiert):**
  - Klare CAD-Ansicht ohne DIN-Schiene mit Hersteller-Logos (**WAGO**, **Phoenix Contact**, **Weidmüller**), Modus-Umschalter (**Verschieben**, **Rahmenauswahl**, **Brücken-Modus**), Touch-Panning sowie Mittel-/Rechtsklick-Drag.
  - **Typ-Anzeige & Inline-Bearbeitung:** `-90°` vertikal oder horizontal gruppiert; **Doppelklick** auf Beschriftung zum direkten Editieren (`Enter` = übernehmen, leer = Auto, `Esc` = abbrechen).
- **Produktvorschau, Hersteller-Link & Passendes Zubehör:**
  - Im rechten Panel erscheinen zur gewählten Klemme automatisch **Produktfoto (wenn abrufbar) bzw. technische Profilvorschau**, **Kammeranzahl**, ein direkter **Link zur Hersteller-Produktseite** sowie **Enddeckel, Brücken, Marker, Endklammern und N-Sammelschienen**.
- **Druckerfertiger Export (`.xlsx` & `.csv`):**
  - **Excel (`.xlsx`):** 3 Arbeitsblätter (`Druckfolge` mit `ByTierStrip`-Standard, `Ebenen-Spalten`, `Ebenen-Matrix`) – alle Zellen strikt als **Excel-Text (`@`)**.
  - **CSV (`.csv`) & Projektdateien (`.termi` / `.json`)** inklusive Potentialen, Sonderzeichen und eigenen Klemmen.
- **Geführter Projekt-Start („Projekt-Line“), 7 Sprachen & System-Theme:**
  - Startbildschirm führt Schritt für Schritt von *Neues Projekt / Laden* -> *Erste Klemmleiste* -> *CAD-Editor*. **DE, EN, FR, IT, ES, PL, NL** – Theme folgt standardmäßig dem **System-Modus** (manuell auf Hell/Dunkel umschaltbar) mit klaren Kontrast-Aktionsfarben, nativem Update-Check auf Desktop & Android sowie **GitHub** & **Buy Me a Coffee**.

---

### Downloads & Portabilität

Alle fertigen Pakete sind auf der **[Releases-Seite](https://github.com/PatrickBarbian/TermiGrid/releases)** verfügbar:

| Plattform | Architektur / Ziel | Datei | Hinweise |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100 % portabel** (Einzeldatei mit `+crt-static` – keine Installation, kein VCRUNTIME nötig) |
| **macOS** | Universal (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | `.app`-Bundle in den Programme-Ordner ziehen |
| **Android** | Universal (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Für Smartphones, Tablets & Foldables (Android 7.0 – 15 / API 24–35, 16-KB-Page-Alignment, v1/v2/v3 signiert) |

#### Plattform-Hinweise

- **Windows:** Direkt per Doppelklick von USB-Stick oder lokalem Ordner starten – benötigt keine Administratorrechte und keine Laufzeitbibliotheken.
- **macOS (Unsigniertes App-Bundle):** Beim ersten Start ggf. **Rechtsklick -> Öffnen** verwenden oder unter *Systemeinstellungen -> Datenschutz & Sicherheit* freigeben.
- **Android:** APK herunterladen und Installation aus unbekannten Quellen für den Browser/Dateimanager erlauben. Durch 16-KB-Page-Alignment und `resizeableActivity` auch mit aktuellen Geräten (z. B. Pixel 10 Pro XL) und Foldables kompatibel. Exporte landen direkt unter `/sdcard/Download/TermiGrid/`.

---

### Kurzanleitung

1. **Klemmen hinzufügen:** In der rechten Spalte Hersteller und Typ wählen (oder Suchfeld nutzen) und auf *+ Klemme anhängen* bzw. *Davor/Dahinter einfügen* klicken.
2. **Startnummer & Typ-Anzeige festlegen:** Oben die Startnummer einstellen und bei Bedarf zwischen vertikaler (`-90°`) und horizontaler Typ-Anzeige wechseln.
3. **Texte anpassen:** Im CAD-Editor doppelt auf eine Beschriftung klicken, um einen manuellen Text zu vergeben (`Enter` zum Bestätigen, leeres Feld setzt die Automatik zurück).
4. **Zubehör prüfen:** Klemmen markieren und im Bereich *Passendes Zubehör* direkt Enddeckel, Brücken, Marker, Endklammern und N-Sammelschienen ablesen.
5. **Druckliste exportieren:** Druckreihenfolge (*Ebenenweise / Streifendruck* als Standard) sowie optional `'PE' im Export ausgeben` wählen und als `.xlsx` oder `.csv` exportieren.

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
<summary><strong>🇬🇧 English — Complete Documentation (Click to expand / Zur vollständigen Datei: docs/README.en.md)</strong></summary>

### Overview & VibeCoded Note

**TermiGrid** is a cross-platform tool (written in Rust with `egui` / `eframe 0.37`) for planning DIN-rail terminal strips in control cabinet engineering and exporting printer-ready label lists for industrial marking systems.

> **✨ VibeCoded:** This project is **VibeCoded** – designed and built with **AI assistance** in close collaboration with the project author.

*Standalone file:* [docs/README.en.md](docs/README.en.md)

### Key Features

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

### Downloads & Portability

All prebuilt binaries are available on the **[Releases page](https://github.com/PatrickBarbian/TermiGrid/releases)**:

| Platform | Architecture / Target | File | Notes |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100% portable** (single `.exe` built with `+crt-static` – no installer, no VCRUNTIME needed) |
| **macOS** | Universal (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Drag the `.app` bundle into Applications |
| **Android** | Universal (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphones, tablets & foldables (Android 7.0 – 15 / API 24–35, 16 KB page-aligned, v1/v2/v3 signed) |

#### Platform Notes

- **Windows:** Launch directly via double-click from a USB drive or local folder – no administrator rights or external runtimes required.
- **macOS (unsigned `.app`):** On first launch, use **Right-click -> Open** or allow execution under *System Settings -> Privacy & Security*.
- **Android:** Download the APK and allow installing from unknown sources for your browser/file manager. With 16 KB page alignment and `resizeableActivity`, it supports modern devices (e.g. Pixel 10 Pro XL) and foldables. Exports are saved to `/sdcard/Download/TermiGrid/`.

### Quick Guide

1. **Add terminals:** Choose manufacturer and terminal type in the right panel (or use the search bar) and click *Append Terminal* or *Insert Before/After*.
2. **Set start number & type display:** Adjust the starting number at the top and switch between vertical (`-90°`) and horizontal type labels as needed.
3. **Edit labels:** Double-click any label in the CAD editor to enter custom text (`Enter` to confirm, clear input to revert to auto-numbering).
4. **Check accessories:** Select terminals and inspect matching end plates, jumpers, markers, end brackets, and N-busbars in the *Matching Accessories* section.
5. **Export label list:** Select print order (*Tier by Tier / Strip* by default) and whether to include `'PE'`, then export as `.xlsx` or `.csv`.

### Build from Source

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

### Links & Support

- **GitHub Repository:** https://github.com/PatrickBarbian/TermiGrid
- **Releases & Downloads:** https://github.com/PatrickBarbian/TermiGrid/releases
- **Support / Buy Me a Coffee:** https://buymeacoffee.com/02eOWKzm5W

### License

Licensed under the **MIT License**.

</details>

<a id="lang-fr"></a>
<details>
<summary><strong>🇫🇷 Français — Documentation complète (Cliquer pour ouvrir / Fichier dédié : docs/README.fr.md)</strong></summary>

### Aperçu & Note VibeCoded

**TermiGrid** est un outil multiplateforme (écrit en Rust avec `egui` / `eframe 0.37`) permettant de concevoir des borniers pour armoires électriques et d'exporter des listes de repérage prêtes à imprimer pour les imprimantes industrielles d'étiquettes.

> **✨ VibeCoded :** Ce projet est **VibeCoded** – conçu et développé avec l'**assistance de l'IA** en étroite collaboration avec l'auteur du projet.

*Fichier dédié :* [docs/README.fr.md](docs/README.fr.md)

### Fonctionnalités principales

- **277 références de bornes intégrées + bornes personnalisées :**
  - **WAGO :** TOPJOB S (`2000–2016`), `2200–2216` (bouton-poussoir), `2102–2116` (levier), `2022` X-COM, `2007` mesure/test, `280/281` et `285` fort courant (`35–95 mm²`)
  - **Phoenix Contact :** `PT 1,5/S` incl. `PTIO`, `PT 2,5–35`, `PTPOWER 35–95`, `PTTB`, `PTTBS`, `PTV`, `PTI`/`UTI` bornes d'installation, `ST`, `UT` & `UK 5 N`
  - **Weidmüller :** `A-Series` (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), `AITB` installation, `W-Series` (`WDU/WPE/WDK/WTR/WSI/DLI`) & `Z-Series` (`ZDU/ZPE/ZDK/ZDL`)
  - **Bornes personnalisées :** Créez vos propres bornes directement dans l'app avec `1×` ou `2×` chambres de pontage
- **Numérotation multi-niveaux, modes N/PE & caractères spéciaux :**
  - Bornes d'installation et multi-étages (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, etc.) : Tout **raccordement PE** présent est toujours aligné sur une ligne dédiée **sous l'étage principal inférieur**.
  - **Numérotation N & PE :** Basculez par bornier entre fixe (`N`/`PE`), compteur partagé (`1, 2, …`), `1N, 2N` / `1PE, 2PE` partagés ou **compteur N/PE propre (`1N, 2N …` / `1PE, 2PE …`)**.
  - **Préfixe / Suffixe / Caractères spéciaux par lot :** Sélectionnez plusieurs bornes et ajoutez `+`, `-`, `~`, `=`, `.1`, `Ω`, etc. avant ou après le repère (conservé lors de la renumérotation).
- **Mode Pontage & Calcul de potentiels :**
  - Créez plusieurs **potentiels** colorés (`+24V`, `GND`, `L1` …) et touchez/cliquez les étages directement dans la vue CAO.
  - **Calcul automatique :** Détermine le nombre de pôles de peigne requis (ex. `2002-405`, `FBS 5-5`, `ZQV`), signale les **broches à couper (`Couper broche ×`)**, planifie les **ponts filaires (`DB`)** entre étages (`E1 <-> E2`) ou changements de pas, et gère **1 chambre** (pont filaire si > pôles max) vs **2 chambres** (cascade superposée sur chambres 1 & 2).
- **Éditeur CAO 2D interactif (Bureau & Tactile) :**
  - Vue CAO sans rail DIN superflu, logos fabricants intégrés (**WAGO**, **Phoenix Contact**, **Weidmüller**), sélecteur d'outils (**Déplacer**, **Sélection**, **Mode Pontage**), déplacement tactile & bouton milieu/droit.
  - **Affichage & édition directe :** Vertical `-90°` ou horizontal groupé ; **double-clic** pour éditer en direct (`Entrée` = valider, vide = auto, `Échap` = annuler).
- **Aperçu produit, lien fabricant & accessoires compatibles :**
  - Affiche **photo produit (si accessible) ou profil technique**, **nombre de chambres**, **lien direct vers la page produit fabricant**, plus **flasques, peignes, repères, butées et barres N**.
- **Export prêt pour l'impression (`.xlsx` & `.csv`) & Fichiers projet (`.termi` / `.json`) :**
  - **Excel (`.xlsx`) :** 3 feuilles (`Druckfolge` / `ByTierStrip` par défaut, `Ebenen-Spalten`, `Ebenen-Matrix`), toutes les cellules formatées en **Texte Excel (`@`)**.
- **Démarrage guidé, 7 langues & Thème Système :**
  - Assistant de démarrage *Nouveau projet / Charger* -> *Premier bornier* -> *Éditeur CAO*. **DE, EN, FR, IT, ES, PL, NL** – suit le **thème Système** par défaut (commutable Système/Clair/Sombre), boutons d'action contrastés, vérification GitHub sur Bureau & Android, **GitHub** & **Buy Me a Coffee**.

### Téléchargements & Portabilité

Tous les paquets prêts à l'emploi sont disponibles sur la **[page Releases](https://github.com/PatrickBarbian/TermiGrid/releases)** :

| Plateforme | Architecture / Cible | Fichier | Remarques |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100 % portable** (exécutable unique compilé avec `+crt-static` – sans installation ni VCRUNTIME) |
| **macOS** | Universel (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Glisser le bundle `.app` dans Applications |
| **Android** | Universel (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphones, tablettes & pliables (Android 7.0 – 15 / API 24–35, alignement mémoire 16 Ko, signature v1/v2/v3) |

#### Notes par plateforme

- **Windows :** Lancer directement par double-clic depuis une clé USB ou un dossier local – aucun droit administrateur ni bibliothèque externe requis.
- **macOS (application non signée) :** Au premier lancement, utiliser **Clic droit -> Ouvrir** ou autoriser l'ouverture dans *Réglages Système -> Confidentialité et sécurité*.
- **Android :** Télécharger l'APK et autoriser l'installation depuis des sources inconnues pour votre navigateur/gestionnaire de fichiers. Grâce à l'alignement 16 Ko et à `resizeableActivity`, l'app fonctionne aussi sur les appareils récents (ex. Pixel 10 Pro XL) et les téléphones pliables. Les exports sont enregistrés dans `/sdcard/Download/TermiGrid/`.

### Guide rapide

1. **Ajouter des bornes :** Choisir le fabricant et le modèle dans le panneau de droite (ou via la recherche), puis cliquer sur *Ajouter une borne* ou *Insérer avant/après*.
2. **Définir le numéro de départ & l'affichage du type :** Régler le numéro initial en haut et basculer si besoin entre l'affichage vertical (`-90°`) et horizontal.
3. **Modifier les repères :** Double-cliquer sur une étiquette dans l'éditeur CAO pour saisir un texte manuel (`Entrée` pour valider, champ vide pour revenir à la numérotation automatique).
4. **Consulter les accessoires :** Sélectionner les bornes et lire directement flasques, ponts, repères, butées et barres N dans la section *Accessoires compatibles*.
5. **Exporter la liste d'impression :** Choisir l'ordre d'impression (*Par niveau / en bande* par défaut) ainsi que l'option `'PE' dans l'export`, puis exporter en `.xlsx` ou `.csv`.

### Compiler depuis les sources

Prérequis : chaîne d'outils Rust récente (`rustup` / `cargo`).

```bash
# Contrôles qualité & tests
cargo clippy --all-targets -- -D warnings
cargo test

# Build Release bureau (Windows / Linux / macOS natif)
cargo build --release --bin termigrid

# macOS Universel (Intel + Apple Silicon)
cargo build --release --bin termigrid --target x86_64-apple-darwin
cargo build --release --bin termigrid --target aarch64-apple-darwin

# Android Universal APK (avec cargo-apk & Android NDK installés)
cargo apk build --release --lib
```

### Liens & Soutien

- **Dépôt GitHub :** https://github.com/PatrickBarbian/TermiGrid
- **Releases & Téléchargements :** https://github.com/PatrickBarbian/TermiGrid/releases
- **Soutien / Buy Me a Coffee :** https://buymeacoffee.com/02eOWKzm5W

### Licence

Publié sous **licence MIT**.

</details>

<a id="lang-it"></a>
<details>
<summary><strong>🇮🇹 Italiano — Documentazione completa (Clicca per espandere / File dedicato: docs/README.it.md)</strong></summary>

### Panoramica & Nota VibeCoded

**TermiGrid** è uno strumento multipiattaforma (scritto in Rust con `egui` / `eframe 0.37`) per progettare morsettiere nei quadri elettrici ed esportare liste di siglatura pronte per la stampa su stampanti industriali per etichette.

> **✨ VibeCoded:** Questo progetto è **VibeCoded** – ideato e sviluppato con il **supporto dell'IA** in stretta collaborazione con l'autore del progetto.

*File dedicato:* [docs/README.it.md](docs/README.it.md)

### Funzionalità principali

- **277 modelli di morsetti integrati + morsetti personalizzati:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (pulsante), `2102–2116` (leva), `2022` X-COM, `2007` misura/test, `280/281` e `285` alta corrente (`35–95 mm²`)
  - **Phoenix Contact:** `PT 1,5/S` incl. `PTIO`, `PT 2,5–35`, `PTPOWER 35–95`, `PTTB`, `PTTBS`, `PTV`, `PTI`/`UTI` morsetti per installazione, `ST`, `UT` & `UK 5 N`
  - **Weidmüller:** `A-Series` (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), `AITB` installazione, `W-Series` (`WDU/WPE/WDK/WTR/WSI/DLI`) & `Z-Series` (`ZDU/ZPE/ZDK/ZDL`)
  - **Morsetti personalizzati:** Crea morsetti personalizzati direttamente nell'app incluse `1×` o `2×` camere ponticello
- **Numerazione multilivello, modalità N/PE & simboli speciali:**
  - Morsetti di installazione e multipiano (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, ecc.): Un **collegamento PE** presente viene sempre allineato su una riga dedicata **sotto il livello principale inferiore**.
  - **Numerazione N & PE:** Impostabile per morsettiera tra fisso (`N`/`PE`), sequenza condivisa (`1, 2, …`), `1N, 2N` / `1PE, 2PE` condivisi o **contatore N/PE proprio (`1N, 2N …` / `1PE, 2PE …`)**.
  - **Prefisso / Suffisso / Simboli su selezione:** Seleziona più morsetti e aggiungi `+`, `-`, `~`, `=`, `.1`, `Ω`, ecc. prima o dopo il numero su tutti i piani o un piano mirato (dinamico anche cambiando il numero iniziale).
- **Modo Ponticelli & Calcolo Potenziali:**
  - Crea **potenziali** a colori (`+24V`, `GND`, `L1` …) e tocca/clicca i piani direttamente nella vista CAD.
  - **Calcolo automatico:** Calcola i poli del ponticello necessari (es. `2002-405`, `FBS 5-5`, `ZQV`), segnala i **pin da rimuovere (`Rimuovere pin ×`)**, pianifica **ponticelli a filo (`DB`)** tra piani (`E1 <-> E2`) o cambi di passo, e gestisce **1 camera** (ponticello a filo oltre i poli max) vs **2 camere** (cascata su camera 1 & 2).
- **Editor CAD 2D interattivo (Desktop & Touch):**
  - Vista CAD senza guida DIN superflua con loghi produttori (**WAGO**, **Phoenix Contact**, **Weidmüller**), selettore strumenti (**Sposta vista**, **Riquadro**, **Modo Ponticelli**), scorrimento touch e trascinamento tasto centrale/destro.
  - **Visualizzazione & modifica diretta:** Verticale `-90°` o orizzontale raggruppata; **doppio clic** per modificare (`Invio` = applica, vuoto = auto, `Esc` = annulla).
- **Anteprima prodotto, link produttore & accessori compatibili:**
  - Mostra **foto prodotto (se raggiungibile) o profilo tecnico**, **numero camere**, **link diretto alla pagina prodotto del produttore**, più **piastre, ponticelli, targhette, fermamorsetti e barre N**.
- **Esportazione pronta per la stampa (`.xlsx` & `.csv`) & File di progetto (`.termi` / `.json`):**
  - **Excel (`.xlsx`):** 3 fogli (`Druckfolge` / `ByTierStrip` predefinito, `Ebenen-Spalten`, `Ebenen-Matrix`), tutte le celle formattate in **Testo Excel (`@`)**.
- **Avvio guidato, 7 lingue & Tema di Sistema:**
  - Procedura guidata *Nuovo progetto / Carica* -> *Prima morsettiera* -> *Editor CAD*. **DE, EN, FR, IT, ES, PL, NL** – segue il **tema di Sistema** predefinito (commutabile Sistema/Chiaro/Scuro), pulsanti ad alto contrasto, controllo GitHub su Desktop & Android, **GitHub** & **Buy Me a Coffee**.

### Download & Portabilità

Tutti i pacchetti pronti sono disponibili nella **[pagina Releases](https://github.com/PatrickBarbian/TermiGrid/releases)**:

| Piattaforma | Architettura / Target | File | Note |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100% portabile** (eseguibile singolo con `+crt-static` – nessuna installazione, nessun VCRUNTIME richiesto) |
| **macOS** | Universale (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Trascina il bundle `.app` in Applicazioni |
| **Android** | Universale (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphone, tablet & pieghevoli (Android 7.0 – 15 / API 24–35, allineamento pagine 16 KB, firma v1/v2/v3) |

#### Note per piattaforma

- **Windows:** Avviabile direttamente con doppio clic da chiavetta USB o cartella locale – non richiede diritti di amministratore né runtime esterni.
- **macOS (app non firmata):** Al primo avvio usa **Clic destro -> Apri** oppure consenti l'esecuzione in *Impostazioni di Sistema -> Privacy e sicurezza*.
- **Android:** Scarica l'APK e consenti l'installazione da origini sconosciute per il browser/file manager. Grazie all'allineamento a 16 KB e a `resizeableActivity`, è compatibile anche con dispositivi moderni (es. Pixel 10 Pro XL) e pieghevoli. I file esportati vengono salvati in `/sdcard/Download/TermiGrid/`.

### Guida rapida

1. **Aggiungere morsetti:** Scegli produttore e modello nella colonna destra (o usa la ricerca) e clicca su *Aggiungi morsetto* oppure *Inserisci prima/dopo*.
2. **Impostare numero iniziale & visualizzazione tipo:** Imposta il numero iniziale in alto e alterna tra visualizzazione verticale (`-90°`) e orizzontale.
3. **Modificare i testi:** Fai doppio clic su una siglatura nell'editor CAD per inserire un testo manuale (`Invio` per confermare, campo vuoto per ripristinare l'automatico).
4. **Verificare gli accessori:** Seleziona i morsetti e controlla subito piastre terminali, ponticelli, targhette, fermamorsetti e barre N nella sezione *Accessori compatibili*.
5. **Esportare la lista di stampa:** Seleziona l'ordine di stampa (*Per piano / nastro* come predefinito) e l'opzione `'PE' nell'esportazione`, quindi esporta in `.xlsx` o `.csv`.

### Compilare dai sorgenti

Prerequisito: toolchain Rust aggiornata (`rustup` / `cargo`).

```bash
# Controlli qualità & test
cargo clippy --all-targets -- -D warnings
cargo test

# Build Release desktop (Windows / Linux / macOS nativo)
cargo build --release --bin termigrid

# macOS Universale (Intel + Apple Silicon)
cargo build --release --bin termigrid --target x86_64-apple-darwin
cargo build --release --bin termigrid --target aarch64-apple-darwin

# Android Universal APK (con cargo-apk & Android NDK installati)
cargo apk build --release --lib
```

### Link & Supporto

- **Repository GitHub:** https://github.com/PatrickBarbian/TermiGrid
- **Releases & Download:** https://github.com/PatrickBarbian/TermiGrid/releases
- **Supporto / Buy Me a Coffee:** https://buymeacoffee.com/02eOWKzm5W

### Licenza

Distribuito con **licenza MIT**.

</details>

<a id="lang-es"></a>
<details>
<summary><strong>🇪🇸 Español — Documentación completa (Clic para expandir / Archivo independiente: docs/README.es.md)</strong></summary>

### Descripción general y nota VibeCoded

**TermiGrid** es una herramienta multiplataforma (escrita en Rust con `egui` / `eframe 0.37`) para planificar regleteros de bornes en cuadros eléctricos y exportar listas de etiquetado listas para imprimir en impresoras industriales de marcado.

> **✨ VibeCoded:** Este proyecto ha sido **VibeCoded** – diseñado y desarrollado con **asistencia de IA** en estrecha colaboración con el autor del proyecto.

*Archivo independiente:* [docs/README.es.md](docs/README.es.md)

### Funciones principales

- **277 tipos de bornes integrados + bornes personalizados:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (pulsador), `2102–2116` (palanca), `2022` X-COM, `2007` medida/prueba, `280/281` y `285` alta corriente (`35–95 mm²`)
  - **Phoenix Contact:** `PT 1,5/S` incl. `PTIO`, `PT 2,5–35`, `PTPOWER 35–95`, `PTTB`, `PTTBS`, `PTV`, `PTI`/`UTI` instalación, `ST`, `UT` & `UK 5 N`
  - **Weidmüller:** `A-Series` (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), `AITB` instalación, `W-Series` (`WDU/WPE/WDK/WTR/WSI/DLI`) & `Z-Series` (`ZDU/ZPE/ZDK/ZDL`)
  - **Bornes personalizados:** Crea tus propios bornes en la app incluyendo `1×` o `2×` cámaras de puente
- **Numeración multinivel, modos N/PE y símbolos especiales:**
  - Bornes de instalación y multipiso (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, etc.): Cualquier **conexión PE** se alinea siempre en su propia fila **debajo del nivel principal inferior**.
  - **Numeración N y PE:** Selecciona por regletero entre fijo (`N`/`PE`), contador compartido (`1, 2, …`), `1N, 2N` / `1PE, 2PE` compartidos o **contador propio N/PE (`1N, 2N …` / `1PE, 2PE …`)**.
  - **Prefijo / Sufijo / Símbolos en lote:** Marca varios bornes y añade `+`, `-`, `~`, `=`, `.1`, `Ω`, etc. antes o después del número en todos o en un piso específico (se conserva al renumerar).
- **Modo Puentes y Cálculo de Potenciales:**
  - Crea **potenciales** por color (`+24V`, `GND`, `L1` …) y pulsa los niveles de bornes directamente en el CAD.
  - **Cálculo automático:** Determina polos de puente peine (`2002-405`, `FBS 5-5`, `ZQV`), marca **pines a quitar (`Quitar pin ×`)**, planifica **puentes de cable (`DB`)** entre pisos (`E1 <-> E2`) o cambios de paso, y distingue **1 cámara** (puente de cable si supera polos máx.) vs **2 cámaras** (cascada en cámaras 1 y 2).
- **Editor CAD 2D interactivo (Escritorio y Táctil):**
  - Vista CAD sin carril DIN innecesario con logotipos integrados (**WAGO**, **Phoenix Contact**, **Weidmüller**), selector (**Mover vista**, **Selección**, **Modo Puentes**), desplazamiento táctil y arrastre con botón central/derecho.
  - **Visualización y edición directa:** Vertical `-90°` o horizontal agrupada; **doble clic** para editar (`Enter` = aplicar, vacío = automático, `Esc` = cancelar).
- **Vista previa del producto, enlace del fabricante y accesorios:**
  - Muestra **foto del producto (cuando está disponible) o perfil técnico**, **número de cámaras**, **enlace directo a la página del fabricante**, además de **tapas, puentes, marcadores, topes finales y barras N**.
- **Exportación lista para impresora (`.xlsx` & `.csv`) y Proyectos (`.termi` / `.json`):**
  - **Excel (`.xlsx`):** 3 hojas (`Druckfolge` / `ByTierStrip` por defecto, `Ebenen-Spalten`, `Ebenen-Matrix`), todas las celdas formateadas como **Texto Excel (`@`)**.
- **Inicio guiado, 7 idiomas y Tema del Sistema:**
  - Asistente *Nuevo proyecto / Abrir* -> *Primer regletero* -> *Editor CAD*. **DE, EN, FR, IT, ES, PL, NL** – sigue el **tema del Sistema** por defecto (cambiable Sistema/Claro/Oscuro), botones destacados de alto contraste, comprobación de GitHub en Escritorio y Android, **GitHub** y **Buy Me a Coffee**.

### Descargas y portabilidad

Todos los paquetes compilados están disponibles en la **[página de Releases](https://github.com/PatrickBarbian/TermiGrid/releases)**:

| Plataforma | Arquitectura / Destino | Archivo | Notas |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100 % portable** (ejecutable único compilado con `+crt-static` – sin instalación ni VCRUNTIME) |
| **macOS** | Universal (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Arrastra el paquete `.app` a Aplicaciones |
| **Android** | Universal (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphones, tablets y plegables (Android 7.0 – 15 / API 24–35, alineación de página de 16 KB, firma v1/v2/v3) |

#### Notas por plataforma

- **Windows:** Ejecutar directamente con doble clic desde una memoria USB o carpeta local – no requiere permisos de administrador ni librerías externas.
- **macOS (aplicación sin firma):** En el primer inicio usa **Clic derecho -> Abrir** o autoriza la app en *Ajustes del Sistema -> Privacidad y seguridad*.
- **Android:** Descarga el APK y permite la instalación desde orígenes desconocidos para tu navegador o gestor de archivos. Gracias a la alineación de 16 KB y `resizeableActivity`, es compatible con dispositivos actuales (p. ej. Pixel 10 Pro XL) y plegables. Las exportaciones se guardan en `/sdcard/Download/TermiGrid/`.

### Guía rápida

1. **Añadir bornes:** Selecciona fabricante y modelo en el panel derecho (o usa la búsqueda) y haz clic en *Añadir borne* o *Insertar antes/después*.
2. **Ajustar número inicial y visualización:** Configura el número inicial en la parte superior y alterna entre visualización vertical (`-90°`) y horizontal según prefieras.
3. **Editar etiquetas:** Haz doble clic en cualquier etiqueta del editor CAD para escribir un texto manual (`Enter` para confirmar, dejar vacío para volver a la numeración automática).
4. **Consultar accesorios:** Selecciona los bornes y consulta al instante tapas finales, puentes, marcadores, topes finales y barras N en *Accesorios compatibles*.
5. **Exportar lista de impresión:** Elige el orden (*Por nivel / tira* por defecto) y si deseas incluir `'PE'`, y exporta en `.xlsx` o `.csv`.

### Compilar desde el código fuente

Requisito: toolchain de Rust actualizada (`rustup` / `cargo`).

```bash
# Comprobación de calidad y pruebas
cargo clippy --all-targets -- -D warnings
cargo test

# Compilación Release de escritorio (Windows / Linux / macOS nativo)
cargo build --release --bin termigrid

# macOS Universal (Intel + Apple Silicon)
cargo build --release --bin termigrid --target x86_64-apple-darwin
cargo build --release --bin termigrid --target aarch64-apple-darwin

# Android Universal APK (con cargo-apk y Android NDK instalados)
cargo apk build --release --lib
```

### Enlaces y apoyo

- **Repositorio GitHub:** https://github.com/PatrickBarbian/TermiGrid
- **Releases y descargas:** https://github.com/PatrickBarbian/TermiGrid/releases
- **Apoyo / Buy Me a Coffee:** https://buymeacoffee.com/02eOWKzm5W

### Licencia

Publicado bajo la **licencia MIT**.

</details>

<a id="lang-pl"></a>
<details>
<summary><strong>🇵🇱 Polski — Pełna dokumentacja (Kliknij, aby rozwinąć / Osobny plik: docs/README.pl.md)</strong></summary>

### Przegląd i informacja VibeCoded

**TermiGrid** to wieloplatformowe narzędzie (napisane w Rust z użyciem `egui` / `eframe 0.37`) do projektowania listew zaciskowych w szafach sterowniczych oraz eksportu gotowych do druku list opisowych dla przemysłowych drukarek oznaczników.

> **✨ VibeCoded:** Ten projekt powstał jako **VibeCoded** – został zaprojektowany i stworzony z **pomocą AI** w ścisłej współpracy z autorem projektu.

*Osobny plik:* [docs/README.pl.md](docs/README.pl.md)

### Główne funkcje

- **277 wbudowanych typów złączek + własne złączki:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (z przyciskiem), `2102–2116` (dźwignia), `2022` X-COM, `2007` pomiarowe, `280/281` oraz `285` wysokoprądowe (`35–95 mm²`)
  - **Phoenix Contact:** `PT 1,5/S` z `PTIO`, `PT 2,5–35`, `PTPOWER 35–95`, `PTTB`, `PTTBS`, `PTV`, `PTI`/`UTI` instalacyjne, `ST`, `UT` & `UK 5 N`
  - **Weidmüller:** `A-Series` (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), `AITB` instalacyjne, `W-Series` (`WDU/WPE/WDK/WTR/WSI/DLI`) & `Z-Series` (`ZDU/ZPE/ZDK/ZDL`)
  - **Własne złączki:** Twórz własne złączki w aplikacji z wyborem `1×` lub `2×` komór mostka
- **Numeracja wielopoziomowa, tryby N/PE i znaki specjalne:**
  - Złączki instalacyjne i wielopiętrowe (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI` itd.): Zacisk **PE** jest zawsze wyrównany w osobnym wierszu **poniżej dolnego poziomu głównego**.
  - **Numeracja N i PE:** Przełączana na listwę między stałym (`N`/`PE`), wspólnym licznikiem (`1, 2, …`), wspólnym `1N, 2N` / `1PE, 2PE` lub **własnym licznikiem N/PE (`1N, 2N …` / `1PE, 2PE …`)**.
  - **Prefiks / Sufiks / Znaki specjalne dla zaznaczonych:** Zaznacz dowolne złączki i dodaj `+`, `-`, `~`, `=`, `.1`, `Ω` itd. przed lub po numerze (zachowywane przy zmianie numeracji).
- **Tryb Mostków i obliczanie potencjałów:**
  - Twórz kolorowe **potencjały** (`+24V`, `GND`, `L1` …) i klikaj/dotykaj poziomy złączek bezpośrednio w widoku CAD.
  - **Automatyczne obliczanie:** Wylicza długość mostka grzebieniowego (np. `2002-405`, `FBS 5-5`, `ZQV`), oznacza **piny do wyłamania (`Wyłamać pin ×`)** przy pominiętych złączkach, planuje **mostki przewodowe (`DB`)** między poziomami (`E1 <-> E2`) lub różnymi rastrami oraz rozróżnia złączki **1-komorowe** (mostek przewodowy powyżej maks. pinów) i **2-komorowe** (kaskada na komorach 1 i 2).
- **Interaktywny edytor CAD 2D (Desktop & Dotyk):**
  - Czytelny widok CAD bez szyny DIN z logotypami producentów (**WAGO**, **Phoenix Contact**, **Weidmüller**), przełącznikiem narzędzi (**Przesuń**, **Zaznacz ramką**, **Tryb Mostków**), przesuwaniem dotykowym oraz środkowym/prawym przyciskiem myszy.
  - **Wyświetlanie i edycja w miejscu:** Pionowo `-90°` lub grupowo poziomo; **dwuklik** pozwala od razu zmienić opis (`Enter` = zatwierdź, puste = auto, `Esc` = anuluj).
- **Podgląd produktu, link producenta i pasujące akcesoria:**
  - Pokazuje **zdjęcie produktu (jeśli dostępne) lub profil techniczny**, **liczbę komór**, **link do strony produktu producenta** oraz **płytki końcowe, mostki, oznaczniki, trzymacze końcowe i szyny N**.
- **Eksport gotowy do druku (`.xlsx` i `.csv`) i Pliki projektu (`.termi` / `.json`):**
  - **Excel (`.xlsx`):** 3 arkusze (`Druckfolge` / domyślnie `ByTierStrip`, `Ebenen-Spalten`, `Ebenen-Matrix`), wszystkie komórki jako **Tekst Excel (`@`)**.
- **Kreator startowy, 7 języków i Motyw Systemowy:**
  - Kreator krok po kroku: *Nowy projekt / Wczytaj* -> *Pierwsza listwa* -> *Edytor CAD*. **DE, EN, FR, IT, ES, PL, NL** – domyślnie używa **motywu Systemowego** (przełączanego System/Jasny/Ciemny), kontrastowe przyciski akcji, sprawdzanie GitHub na Desktop & Android, **GitHub** i **Buy Me a Coffee**.

### Pobieranie i przenośność

Wszystkie gotowe pakiety znajdują się na **[stronie Releases](https://github.com/PatrickBarbian/TermiGrid/releases)**:

| Platforma | Architektura / Cel | Plik | Uwagi |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100% przenośny** (pojedynczy plik `.exe` z `+crt-static` – bez instalacji i bez VCRUNTIME) |
| **macOS** | Uniwersalny (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Przeciągnij pakiet `.app` do folderu Aplikacje |
| **Android** | Uniwersalny (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartfony, tablety i urządzenia składane (Android 7.0 – 15 / API 24–35, wyrównanie stron 16 KB, podpis v1/v2/v3) |

#### Wskazówki dla platform

- **Windows:** Uruchom bezpośrednio dwuklikiem z pendrive'a lub folderu lokalnego – nie wymaga uprawnień administratora ani dodatkowych bibliotek uruchomieniowych.
- **macOS (niepodpisana aplikacja):** Przy pierwszym uruchomieniu użyj **Prawy przycisk myszy -> Otwórz** lub zezwól na uruchomienie w *Ustawienia systemowe -> Prywatność i ochrona*.
- **Android:** Pobierz APK i zezwól na instalację z nieznanych źródeł dla przeglądarki lub menedżera plików. Dzięki wyrównaniu stron 16 KB i `resizeableActivity` aplikacja działa także na nowych urządzeniach (np. Pixel 10 Pro XL) i ekranach składanych. Eksporty zapisują się w `/sdcard/Download/TermiGrid/`.

### Szybka instrukcja

1. **Dodaj złączki:** W prawej kolumnie wybierz producenta i typ (lub użyj wyszukiwarki) i kliknij *Dodaj złączkę* albo *Wstaw przed/za*.
2. **Ustaw numer startowy i wyświetlanie typu:** Ustaw numer początkowy na górze i w razie potrzeby przełącz między pionowym (`-90°`) a poziomym wyświetlaniem typu.
3. **Edytuj opisy:** Kliknij dwukrotnie etykietę w edytorze CAD, aby wpisać własny tekst (`Enter` potwierdza, puste pole przywraca automatyczną numerację).
4. **Sprawdź akcesoria:** Zaznacz złączki i odczytaj pasujące płytki końcowe, mostki, oznaczniki, trzymacze końcowe oraz szyny N w sekcji *Pasujące akcesoria*.
5. **Eksportuj listę druku:** Wybierz kolejność druku (domyślnie *Poziomami / taśmowo*) oraz opcjonalnie `'PE' w eksporcie`, a następnie eksportuj jako `.xlsx` lub `.csv`.

### Kompilacja ze źródeł

Wymaganie: aktualny zestaw narzędzi Rust (`rustup` / `cargo`).

```bash
# Kontrola jakości i testy
cargo clippy --all-targets -- -D warnings
cargo test

# Budowanie Release na komputer (natywne Windows / Linux / macOS)
cargo build --release --bin termigrid

# macOS Universal (Intel + Apple Silicon)
cargo build --release --bin termigrid --target x86_64-apple-darwin
cargo build --release --bin termigrid --target aarch64-apple-darwin

# Android Universal APK (z zainstalowanym cargo-apk i Android NDK)
cargo apk build --release --lib
```

### Linki i wsparcie

- **Repozytorium GitHub:** https://github.com/PatrickBarbian/TermiGrid
- **Releases i pobieranie:** https://github.com/PatrickBarbian/TermiGrid/releases
- **Wsparcie / Buy Me a Coffee:** https://buymeacoffee.com/02eOWKzm5W

### Licencja

Udostępniono na **licencji MIT**.

</details>

<a id="lang-nl"></a>
<details>
<summary><strong>🇳🇱 Nederlands — Volledige documentatie (Klik om uit te klappen / Los bestand: docs/README.nl.md)</strong></summary>

### Overzicht & VibeCoded-vermelding

**TermiGrid** is een platformonafhankelijke tool (geschreven in Rust met `egui` / `eframe 0.37`) voor het ontwerpen van klemmenstroken in de schakelkastbouw en het exporteren van printklare coderingslijsten voor professionele label- en markeringsprinters.

> **✨ VibeCoded:** Dit project is **VibeCoded** – ontworpen en ontwikkeld met **AI-ondersteuning** in nauwe samenwerking met de projectauteur.

*Los bestand:* [docs/README.nl.md](docs/README.nl.md)

### Belangrijkste functies

- **277 ingebouwde klemtypes + eigen klemmen:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (drukknop), `2102–2116` (hendel), `2022` X-COM, `2007` meet/scheiding, `280/281` en `285` hoogstroom (`35–95 mm²`)
  - **Phoenix Contact:** `PT 1,5/S` incl. `PTIO`, `PT 2,5–35`, `PTPOWER 35–95`, `PTTB`, `PTTBS`, `PTV`, `PTI`/`UTI` installatieklemmen, `ST`, `UT` & `UK 5 N`
  - **Weidmüller:** `A-Series` (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), `AITB` installatie, `W-Series` (`WDU/WPE/WDK/WTR/WSI/DLI`) & `Z-Series` (`ZDU/ZPE/ZDK/ZDL`)
  - **Eigen klemmen:** Maak eigen klemmen direct in de app incl. `1×` of `2×` brugkamers
- **Meerniveaunummering, N/PE-modi & speciale tekens:**
  - Installatie- en meeretageklemmen (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, enz.): Een aanwezige **PE-aansluiting** wordt altijd uitgelijnd op een eigen rij **onder het onderste hoofdniveau**.
  - **N- & PE-nummering:** Per strook instelbaar tussen vast (`N`/`PE`), doorgenummerd (`1, 2, …`), gedeeld `1N, 2N` / `1PE, 2PE` of **eigen N/PE-teller (`1N, 2N …` / `1PE, 2PE …`)**.
  - **Batch Prefix / Suffix / Speciale tekens:** Selecteer meerdere klemmen en voeg `+`, `-`, `~`, `=`, `.1`, `Ω`, enz. toe voor of na het nummer (blijft behouden bij hernummeren).
- **Brug-modus & Potentiaalberekening:**
  - Maak gekleurde **potentialen** (`+24V`, `GND`, `L1` …) en klik/tik op klemniveaus direct in het CAD-overzicht.
  - **Automatische berekening:** Bepaalt het aantal polen van kamverbinders (bijv. `2002-405`, `FBS 5-5`, `ZQV`), markeert **uit te breken pinnen (`Pin verwijderen ×`)**, plant **draadbruggen (`DB`)** bij etagewissels (`E1 <-> E2`) of steekverschillen en onderscheidt **1-kamerklemmen** (draadbrug boven max. polen) van **2-kamerklemmen** (cascade in kamer 1 & 2).
- **Interactieve 2D CAD-editor (Desktop & Touch):**
  - Overzichtelijke CAD-weergave zonder DIN-rail met fabrikantlogo's (**WAGO**, **Phoenix Contact**, **Weidmüller**), gereedschapsknoppen (**Verschuiven**, **Kaderselectie**, **Brug-modus**), touch-verschuiving & midden-/rechtsklik-slepen.
  - **Weergave & inline bewerken:** Verticaal `-90°` of horizontaal gegroepeerd; **dubbelklik** om direct aan te passen (`Enter` = toepassen, leeg = auto, `Esc` = annuleren).
- **Productvoorbeeld, fabrikantlink & bijpassende accessoires:**
  - Toont **productfoto (indien beschikbaar) of technisch profiel**, **aantal brugkamers**, **directe link naar de fabrikantpagina** en **eindplaten, bruggen, markers, eindsteunen en N-rails**.
- **Printklare export (`.xlsx` & `.csv`) & Projectbestanden (`.termi` / `.json`):**
  - **Excel (`.xlsx`):** 3 werkbladen (`Druckfolge` / `ByTierStrip` standaard, `Ebenen-Spalten`, `Ebenen-Matrix`), alle cellen strikt als **Excel-tekst (`@`)**.
- **Begeleide start, 7 talen & Systeemthema:**
  - Stappenplan bij openen: *Nieuw project / Laden* -> *Eerste klemmenstrook* -> *CAD-editor*. **DE, EN, FR, IT, ES, PL, NL** – volgt standaard het **Systeemthema** (handmatig Systeem/Licht/Donker), contrastrijke actieknoppen, live GitHub-check op Desktop & Android, **GitHub** & **Buy Me a Coffee**.

### Downloads & Draagbaarheid

Alle kant-en-klare pakketten staan op de **[Releases-pagina](https://github.com/PatrickBarbian/TermiGrid/releases)**:

| Platform | Architectuur / Doel | Bestand | Opmerkingen |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100% portable** (los `.exe`-bestand met `+crt-static` – geen installatie en geen VCRUNTIME nodig) |
| **macOS** | Universeel (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Sleep de `.app`-bundel naar Programma's |
| **Android** | Universeel (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphones, tablets & foldables (Android 7.0 – 15 / API 24–35, 16 KB pagina-uitlijning, v1/v2/v3 ondertekend) |

#### Platformopmerkingen

- **Windows:** Direct starten met een dubbelklik vanaf USB-stick of lokale map – geen beheerdersrechten of externe runtimebibliotheken nodig.
- **macOS (niet-ondertekende app):** Gebruik bij de eerste start **Rechtermuisklik -> Open** of sta openen toe via *Systeeminstellingen -> Privacy en beveiliging*.
- **Android:** Download de APK en sta installeren vanuit onbekende bronnen toe voor je browser of bestandsbeheerder. Dankzij 16 KB pagina-uitlijning en `resizeableActivity` ook geschikt voor moderne toestellen (zoals Pixel 10 Pro XL) en foldables. Exports worden opgeslagen in `/sdcard/Download/TermiGrid/`.

### Snelstartgids

1. **Klemmen toevoegen:** Kies fabrikant en type in de rechterkolom (of gebruik het zoekveld) en klik op *Klem toevoegen* of *Ervoor/Erna invoegen*.
2. **Startnummer & typeweergave instellen:** Stel bovenaan het startnummer in en wissel desgewenst tussen verticale (`-90°`) en horizontale typeweergave.
3. **Teksten aanpassen:** Dubbelklik op een codering in de CAD-editor om handmatige tekst in te voeren (`Enter` bevestigt, leeg veld herstelt automatische nummering).
4. **Accessoires bekijken:** Selecteer klemmen en lees direct eindplaten, bruggen, markers, eindsteunen en N-rails af onder *Bijpassende accessoires*.
5. **Printlijst exporteren:** Kies de printvolgorde (*Per niveau / strook* standaard) en optioneel `'PE' in export opnemen`, en exporteer als `.xlsx` of `.csv`.

### Zelf compileren (Build from Source)

Vereiste: recente Rust-toolchain (`rustup` / `cargo`).

```bash
# Kwaliteitscontrole & tests
cargo clippy --all-targets -- -D warnings
cargo test

# Desktop Release-build (native Windows / Linux / macOS)
cargo build --release --bin termigrid

# macOS Universeel (Intel + Apple Silicon)
cargo build --release --bin termigrid --target x86_64-apple-darwin
cargo build --release --bin termigrid --target aarch64-apple-darwin

# Android Universal APK (met cargo-apk & Android NDK geïnstalleerd)
cargo apk build --release --lib
```

### Links & Ondersteuning

- **GitHub Repository:** https://github.com/PatrickBarbian/TermiGrid
- **Releases & Downloads:** https://github.com/PatrickBarbian/TermiGrid/releases
- **Ondersteuning / Buy Me a Coffee:** https://buymeacoffee.com/02eOWKzm5W

### Licentie

Uitgebracht onder de **MIT-licentie**.

</details>
