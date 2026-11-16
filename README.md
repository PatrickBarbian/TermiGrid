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

- **186 hinterlegte Klemmentypen + eigene Klemmen:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (mit Betätigungsdrücker), `2102–2116`, `2022` X-COM sowie `280/281`
  - **Phoenix Contact:** `PT 1,5/S`, `PT 2,5–35`, `PTPOWER`, `PTTB`, `PTTBS`, `PTV`, `PTI` Installationsklemmen sowie `ST` & `UT`
  - **Weidmüller:** `A-Series`, `AITB` Installationsklemmen, `W-Series`, `DLI` & `Z-Series`
  - **Eigene Klemmen:** Eigene Klemmentypen direkt in der App anlegen und mit Hersteller-Zuordnung verwalten
- **Ebenen-basierte Nummerierung & PE-Ausrichtung:**
  - Installations- und Mehrstockklemmen (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI` usw.): Ein vorhandener **PE-Anschluss** wird stets in einer eigenen PE-Zeile **unterhalb der unteren Hauptebene** ausgerichtet, sodass fortlaufende Nummern auf einer sauberen horizontalen Ebene bleiben.
  - **Beispiel Mehrstockklemme:** Auf einer Klemmenleiste mit 2-Stock-Klemmen zählt Ebene 1 (unten) automatisch ungerade (`1, 3, 5 …`), Ebene 2 (oben) gerade (`2, 4, 6 …`); nachfolgende 1-Stock-Klemmen setzen auf Ebene 1 nahtlos fort (`7, 8, 9 …`).
- **Interaktiver 2D CAD-Editor (ohne Hutschienen-Ballast):**
  - Klare CAD-Ansicht ohne störende DIN-Schiene mit eingebetteten Hersteller-Logos (**WAGO**, **Phoenix Contact**, **Weidmüller**).
  - **Typ-Anzeige:** Wahlweise **vertikal (-90° gedreht)** je Klemme oder **horizontal gruppiert** unterhalb der Klemmenleiste.
  - **Inline-Bearbeitung:** **Doppelklick** auf eine Klemmenbeschriftung öffnet die Direkteingabe (`Enter` = übernehmen, leer lassen = Automatik-Nummer wiederherstellen, `Esc` = abbrechen).
  - **Mehrfachauswahl:** Einzelklick, `Strg+Klick`, `Shift+Klick` und Rahmenauswahl (Marquee) zum gemeinsamen Zuweisen, Einfügen davor/dahinter oder Löschen.
- **Passendes Zubehör automatisch angezeigt:**
  - Die rechte Eigenschaften-Spalte zeigt zu den ausgewählten Klemmen automatisch passendes Zubehör an: **Enddeckel / Abschlussplatten**, **Querbrücken / Verbinder**, **Beschriftungsstreifen / Marker**, **Endklammern** sowie **N-Sammelschienen**.
- **Druckerfertiger Export (`.xlsx` & `.csv`):**
  - **Excel (`.xlsx`):** 3 strukturierte Arbeitsblätter:
    1. `Druckfolge` (`ByTierStrip` als Standard – alle Schilder einer Ebene zusammenhängend am Stück, z. B. erst komplett Ebene 1, danach Ebene 2, danach Ebene 3; optional auch Spalten- oder Block-Reihenfolge sowie optional mit/ohne `PE`)
    2. `Ebenen-Spalten`
    3. `Ebenen-Matrix`
    - Alle Zellen werden strikt als **Excel-Text (`@`)** formatiert, damit führende Nullen (`01`) oder Punkt-Notation (`1.1`) niemals zu Zahlen/Daten konvertiert werden.
  - **CSV (`.csv`):** UTF-8 mit BOM und Semikolon `;` als Standard-Trennzeichen.
  - **Projektdateien:** Laden und Speichern als `.termi` oder `.json` inklusive manueller Überschreibungen und eigener Klemmentypen.
- **7 Sprachen, Themes & integrierter Update-Checker:**
  - Oberfläche umschaltbar zwischen **DE, EN, FR, IT, ES, PL, NL** – mit hohem Kontrast im **Hell- und Dunkelmodus**.
  - **GitHub Update-Checker** direkt in der App sowie direkte Links zu **GitHub** und **Buy Me a Coffee**.

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

- **186 références de bornes intégrées + bornes personnalisées :**
  - **WAGO :** TOPJOB S (`2000–2016`), `2200–2216` (avec bouton-poussoir), `2102–2116`, `2022` X-COM et `280/281`
  - **Phoenix Contact :** `PT 1,5/S`, `PT 2,5–35`, `PTPOWER`, `PTTB`, `PTTBS`, `PTV`, `PTI` bornes d'installation ainsi que `ST` & `UT`
  - **Weidmüller :** `A-Series`, `AITB` bornes d'installation, `W-Series`, `DLI` & `Z-Series`
  - **Bornes personnalisées :** Créez et gérez vos propres types de bornes directement dans l'application
- **Numérotation multi-niveaux & alignement PE :**
  - Bornes d'installation et multi-étages (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, etc.) : Tout **raccordement PE** présent est toujours aligné sur une ligne PE dédiée **sous l'étage principal inférieur**, afin de garder les repères numériques sur une ligne horizontale claire.
  - **Exemple multi-étages :** Sur une barrette de bornes à 2 étages, l'étage 1 (bas) numérote automatiquement en impair (`1, 3, 5 …`) et l'étage 2 (haut) en pair (`2, 4, 6 …`) ; les bornes suivantes à 1 étage poursuivent sans rupture sur l'étage 1 (`7, 8, 9 …`).
- **Éditeur CAO 2D interactif (sans encombrement de rail DIN) :**
  - Vue CAO épurée sans rail DIN superflu, avec logos fabricants intégrés (**WAGO**, **Phoenix Contact**, **Weidmüller**).
  - **Affichage des références :** Au choix **vertical (orienté à `-90°`)** par borne ou **regroupé horizontalement** sous le bornier.
  - **Édition directe (inline) :** Un **double-clic** sur un repère permet de le modifier directement (`Entrée` = valider, champ vide = restaurer la numérotation auto, `Échap` = annuler).
  - **Sélection multiple :** Clic simple, `Ctrl+Clic`, `Maj+Clic` et sélection par rectangle pour réaffecter un type, insérer avant/après ou supprimer en lot.
- **Accessoires compatibles affichés automatiquement :**
  - Le panneau latéral droit affiche automatiquement les accessoires adaptés aux bornes sélectionnées : **flasques d'extrémité**, **peignes de pontage / jonctions**, **bandelettes de marquage**, **butées d'arrêt** et **barres collectrices N**.
- **Export prêt pour l'impression (`.xlsx` & `.csv`) :**
  - **Excel (`.xlsx`) :** 3 feuilles structurées :
    1. `Druckfolge` / Ordre d'impression (`ByTierStrip` par défaut – tous les repères d'un même niveau regroupés en continu, d'abord tout le niveau 1, puis le niveau 2, puis le niveau 3 ; ordre par colonne ou par bloc et inclusion de `PE` en option)
    2. `Ebenen-Spalten` / Colonnes par niveau
    3. `Ebenen-Matrix` / Matrice des niveaux
    - Toutes les cellules sont formatées strictement en **Texte Excel (`@`)** afin que les zéros initiaux (`01`) ou notations avec point (`1.1`) ne soient jamais convertis en nombres ou en dates.
  - **CSV (`.csv`) :** UTF-8 avec BOM et point-virgule `;` par défaut.
  - **Fichiers projet :** Enregistrement et chargement au format `.termi` ou `.json` avec textes manuels et bornes personnalisées.
- **7 langues, thèmes & vérificateur de mises à jour intégré :**
  - Interface disponible en **DE, EN, FR, IT, ES, PL, NL** – avec thèmes **Clair et Sombre** à fort contraste.
  - **Vérificateur de mises à jour GitHub** intégré ainsi que liens directs vers **GitHub** et **Buy Me a Coffee**.

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

- **186 modelli di morsetti integrati + morsetti personalizzati:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (con pulsante), `2102–2116`, `2022` X-COM e `280/281`
  - **Phoenix Contact:** `PT 1,5/S`, `PT 2,5–35`, `PTPOWER`, `PTTB`, `PTTBS`, `PTV`, `PTI` morsetti per installazione, più `ST` & `UT`
  - **Weidmüller:** `A-Series`, `AITB` morsetti per installazione, `W-Series`, `DLI` & `Z-Series`
  - **Morsetti personalizzati:** Crea e gestisci i tuoi morsetti personalizzati direttamente nell'app
- **Numerazione multilivello & allineamento PE:**
  - Morsetti di installazione e multipiano (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, ecc.): Un **collegamento PE** presente viene sempre allineato su una riga PE dedicata **sotto il livello principale inferiore**, mantenendo la numerazione progressiva su una linea orizzontale uniforme.
  - **Esempio multipiano:** Su una morsettiera con morsetti a 2 piani, il piano 1 (basso) numera automaticamente i dispari (`1, 3, 5 …`) e il piano 2 (alto) i pari (`2, 4, 6 …`); i successivi morsetti a 1 piano continuano senza interruzioni sul piano 1 (`7, 8, 9 …`).
- **Editor CAD 2D interattivo (senza guida DIN superflua):**
  - Area CAD pulita senza guida DIN superflua, con loghi dei produttori integrati (**WAGO**, **Phoenix Contact**, **Weidmüller**).
  - **Visualizzazione tipo:** A scelta **verticale (ruotata di `-90°`)** per ogni morsetto oppure **raggruppata orizzontalmente** sotto la morsettiera.
  - **Modifica diretta (inline):** Un **doppio clic** su una siglatura apre la modifica immediata (`Invio` = applica, campo vuoto = ripristina numerazione automatica, `Esc` = annulla).
  - **Selezione multipla:** Clic singolo, `Ctrl+Clic`, `Maiusc+Clic` e selezione con riquadro per assegnare tipi, inserire prima/dopo o eliminare in blocco.
- **Accessori compatibili mostrati automaticamente:**
  - Il pannello destro Proprietà mostra automaticamente gli accessori adatti ai morsetti selezionati: **piastre terminali**, **ponticelli / cavallotti**, **strisce di siglatura**, **morsetti finali** e **barre collettrici N**.
- **Esportazione pronta per la stampa (`.xlsx` & `.csv`):**
  - **Excel (`.xlsx`):** 3 fogli strutturati:
    1. `Druckfolge` / Sequenza di stampa (`ByTierStrip` come predefinito – ogni piano esportato di seguito in blocco continuo: prima tutto il piano 1, poi il piano 2, poi il piano 3; ordinamento per colonna o blocco e inclusione `PE` opzionali)
    2. `Ebenen-Spalten` / Colonne per piano
    3. `Ebenen-Matrix` / Matrice dei piani
    - Tutte le celle vengono formattate rigorosamente come **Testo Excel (`@`)** in modo che zeri iniziali (`01`) o notazioni con punto (`1.1`) non vengano mai convertiti in numeri o date.
  - **CSV (`.csv`):** UTF-8 con BOM e punto e virgola `;` come separatore predefinito.
  - **File di progetto:** Salvataggio e caricamento in formato `.termi` o `.json` inclusi testi manuali e morsetti personalizzati.
- **7 lingue, temi & controllo aggiornamenti integrato:**
  - Interfaccia commutabile tra **DE, EN, FR, IT, ES, PL, NL** – con temi **Chiaro e Scuro** ad alto contrasto.
  - **Controllo aggiornamenti GitHub** integrato oltre a link diretti a **GitHub** e **Buy Me a Coffee**.

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

- **186 tipos de bornes integrados + bornes personalizados:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (con pulsador), `2102–2116`, `2022` X-COM y `280/281`
  - **Phoenix Contact:** `PT 1,5/S`, `PT 2,5–35`, `PTPOWER`, `PTTB`, `PTTBS`, `PTV`, `PTI` bornes de instalación, además de `ST` y `UT`
  - **Weidmüller:** `A-Series`, `AITB` bornes de instalación, `W-Series`, `DLI` y `Z-Series`
  - **Bornes personalizados:** Crea y gestiona tus propios modelos de bornes directamente dentro de la aplicación
- **Numeración multinivel y alineación PE:**
  - Bornes de instalación y multipiso (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, etc.): Cualquier **conexión PE** existente se alinea siempre en su propia fila PE **debajo del nivel principal inferior**, manteniendo los números consecutivos sobre una línea horizontal limpia.
  - **Ejemplo multipiso:** En un regletero de bornes de 2 pisos, el nivel 1 (inferior) numera automáticamente los impares (`1, 3, 5 …`) y el nivel 2 (superior) los pares (`2, 4, 6 …`); los bornes siguientes de 1 piso continúan sin cortes en el nivel 1 (`7, 8, 9 …`).
- **Editor CAD 2D interactivo (sin ruido visual de carril DIN):**
  - Vista CAD limpia sin carril DIN innecesario, con logotipos integrados de fabricantes (**WAGO**, **Phoenix Contact**, **Weidmüller**).
  - **Visualización de tipo:** Elige entre **vertical (rotado `-90°`)** por cada borne o **agrupado horizontalmente** debajo del regletero.
  - **Edición en línea:** Un **doble clic** sobre cualquier etiqueta permite editarla al instante (`Enter` = aplicar, campo vacío = restaurar numeración automática, `Esc` = cancelar).
  - **Selección múltiple:** Clic simple, `Ctrl+Clic`, `Shift+Clic` y selección por marco para reasignar tipos, insertar antes/después o eliminar en bloque.
- **Accesorios compatibles mostrados automáticamente:**
  - El panel derecho de Propiedades muestra automáticamente los accesorios compatibles para los bornes seleccionados: **tapas finales**, **puentes de conexión**, **tiras de marcado**, **topes finales** y **barras colectoras N**.
- **Exportación lista para impresora (`.xlsx` y `.csv`):**
  - **Excel (`.xlsx`):** 3 hojas estructuradas:
    1. `Druckfolge` / Secuencia de impresión (`ByTierStrip` por defecto – exporta cada nivel de forma contigua en una tira, p. ej. primero todo el nivel 1, luego el nivel 2 y después el nivel 3; orden por columna o bloque y opción de incluir/excluir `PE`)
    2. `Ebenen-Spalten` / Columnas por nivel
    3. `Ebenen-Matrix` / Matriz de niveles
    - Todas las celdas se formatean estrictamente como **Texto Excel (`@`)** para que los ceros a la izquierda (`01`) o notaciones con punto (`1.1`) nunca se conviertan en números ni fechas.
  - **CSV (`.csv`):** UTF-8 con BOM y punto y coma `;` como separador predeterminado.
  - **Archivos de proyecto:** Guarda y carga `.termi` o `.json` incluyendo textos manuales y bornes personalizados.
- **7 idiomas, temas y comprobador de actualizaciones integrado:**
  - Interfaz intercambiable entre **DE, EN, FR, IT, ES, PL, NL** – con temas **Claro y Oscuro** de alto contraste.
  - **Comprobador de actualizaciones de GitHub** integrado junto con enlaces directos a **GitHub** y **Buy Me a Coffee**.

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

- **186 wbudowanych typów złączek + własne złączki:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (z przyciskiem), `2102–2116`, `2022` X-COM oraz `280/281`
  - **Phoenix Contact:** `PT 1,5/S`, `PT 2,5–35`, `PTPOWER`, `PTTB`, `PTTBS`, `PTV`, `PTI` złączki instalacyjne, a także `ST` i `UT`
  - **Weidmüller:** `A-Series`, `AITB` złączki instalacyjne, `W-Series`, `DLI` i `Z-Series`
  - **Własne złączki:** Twórz i zarządzaj własnymi definicjami złączek bezpośrednio w aplikacji
- **Numeracja wielopoziomowa i wyrównanie PE:**
  - Złączki instalacyjne i wielopiętrowe (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI` itp.): Istniejący **zacisk PE** jest zawsze wyrównany w osobnym wierszu PE **poniżej dolnego poziomu głównego**, dzięki czemu numeracja ciągła pozostaje na jednej równej linii poziomej.
  - **Przykład wielopiętrowy:** Na listwie ze złączkami 2-piętrowymi poziom 1 (dolny) numeruje się automatycznie nieparzyście (`1, 3, 5 …`), a poziom 2 (górny) parzyście (`2, 4, 6 …`); kolejne złączki 1-piętrowe płynnie kontynuują numerację na poziomie 1 (`7, 8, 9 …`).
- **Interaktywny edytor CAD 2D (bez zbędnej szyny DIN):**
  - Czytelny widok CAD bez zbędnej szyny DIN, z wbudowanymi logotypami producentów (**WAGO**, **Phoenix Contact**, **Weidmüller**).
  - **Wyświetlanie typu:** Do wyboru **pionowo (obrócone o `-90°`)** dla każdej złączki lub **pogrupowane poziomo** pod listwą.
  - **Edycja bezpośrednia (inline):** **Dwukrotne kliknięcie** oznaczenia złączki otwiera edycję na miejscu (`Enter` = zatwierdź, puste pole = przywróć numerację automatyczną, `Esc` = anuluj).
  - **Zaznaczanie wielokrotne:** Pojedyncze kliknięcie, `Ctrl+kliknięcie`, `Shift+kliknięcie` oraz zaznaczanie ramką do wspólnego przypisywania typu, wstawiania przed/za lub usuwania.
- **Automatyczne wyświetlanie pasujących akcesoriów:**
  - Prawy panel Właściwości automatycznie pokazuje pasujące akcesoria dla zaznaczonych złączek: **płytki końcowe**, **mostki / zwory**, **paski oznaczeniowe**, **trzymacze końcowe** oraz **szyny zbiorcze N**.
- **Eksport gotowy do druku (`.xlsx` i `.csv`):**
  - **Excel (`.xlsx`):** 3 przejrzyste arkusze:
    1. `Druckfolge` / Kolejność druku (domyślnie `ByTierStrip` – wszystkie oznaczniki danego poziomu w jednym ciągłym ciągu, np. najpierw cały poziom 1, potem poziom 2, potem poziom 3; opcjonalnie układ kolumnowy lub blokowy oraz uwzględnienie `PE`)
    2. `Ebenen-Spalten` / Kolumny poziomów
    3. `Ebenen-Matrix` / Macierz poziomów
    - Wszystkie komórki są sformatowane ściśle jako **Tekst Excel (`@`)**, aby zera wiodące (`01`) lub zapis z kropką (`1.1`) nigdy nie zostały zamienione na liczby lub daty.
  - **CSV (`.csv`):** UTF-8 z BOM oraz średnik `;` jako domyślny separator.
  - **Pliki projektu:** Zapis i wczytywanie w formacie `.termi` lub `.json` wraz z ręcznymi zmianami i własnymi złączkami.
- **7 języków, motywy i wbudowany moduł aktualizacji:**
  - Interfejs przełączany między **DE, EN, FR, IT, ES, PL, NL** – z motywami **Jasnym i Ciemnym** o wysokim kontraście.
  - Wbudowany **sprawdzacz aktualizacji GitHub** oraz bezpośrednie linki do **GitHub** i **Buy Me a Coffee**.

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

- **186 ingebouwde klemtypes + eigen klemmen:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (met drukknop), `2102–2116`, `2022` X-COM en `280/281`
  - **Phoenix Contact:** `PT 1,5/S`, `PT 2,5–35`, `PTPOWER`, `PTTB`, `PTTBS`, `PTV`, `PTI` installatieklemmen, plus `ST` & `UT`
  - **Weidmüller:** `A-Series`, `AITB` installatieklemmen, `W-Series`, `DLI` & `Z-Series`
  - **Eigen klemmen:** Maak en beheer je eigen aangepaste klemdefinities rechtstreeks in de app
- **Meerniveaunummering & PE-uitlijning:**
  - Installatie- en meeretageklemmen (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, enz.): Een aanwezige **PE-aansluiting** wordt altijd uitgelijnd op een eigen PE-rij **onder het onderste hoofdniveau**, zodat doorlopende nummers op één strakke horizontale lijn blijven.
  - **Voorbeeld meeretageklem:** Op een klemmenstrook met 2-etageklemmen telt niveau 1 (onder) automatisch oneven (`1, 3, 5 …`) en niveau 2 (boven) even (`2, 4, 6 …`); daaropvolgende 1-etageklemmen gaan naadloos verder op niveau 1 (`7, 8, 9 …`).
- **Interactieve 2D CAD-editor (zonder DIN-railballast):**
  - Overzichtelijke CAD-weergave zonder overbodige DIN-rail, met ingebouwde fabrikantlogo's (**WAGO**, **Phoenix Contact**, **Weidmüller**).
  - **Typeweergave:** Naar keuze **verticaal (`-90°` gedraaid)** per klem of **horizontaal gegroepeerd** onder de klemmenstrook.
  - **Inline bewerken:** **Dubbelklik** op een klemcodering om deze direct aan te passen (`Enter` = toepassen, leeg laten = automatische nummering herstellen, `Esc` = annuleren).
  - **Meervoudige selectie:** Enkelklik, `Ctrl+klik`, `Shift+klik` en kaderselectie om samen types toe te wijzen, ervoor/erna in te voegen of te verwijderen.
- **Automatisch bijpassende accessoires:**
  - Het rechterpaneel Eigenschappen toont automatisch passende accessoires voor de geselecteerde klemmen: **eindplaten**, **dwarsbruggen / doorverbinders**, **coderingsstroken / markers**, **eindsteunen** en **N-verzamelrails**.
- **Printklare export (`.xlsx` & `.csv`):**
  - **Excel (`.xlsx`):** 3 overzichtelijke werkbladen:
    1. `Druckfolge` / Printvolgorde (`ByTierStrip` standaard – alle labels per niveau aaneengesloten achter elkaar, bijv. eerst heel niveau 1, daarna niveau 2, daarna niveau 3; optioneel per kolom of blok en met/zonder `PE`)
    2. `Ebenen-Spalten` / Niveaukolommen
    3. `Ebenen-Matrix` / Niveaumatrix
    - Alle cellen worden strikt als **Excel-tekst (`@`)** opgemaakt, zodat voorloopnullen (`01`) of puntnotaties (`1.1`) nooit worden omgezet in getallen of datums.
  - **CSV (`.csv`):** UTF-8 met BOM en puntkomma `;` als standaard scheidingsteken.
  - **Projectbestanden:** Opslaan en laden als `.termi` of `.json` inclusief handmatige aanpassingen en eigen klemmen.
- **7 talen, thema's & ingebouwde Update Checker:**
  - Interface direct omschakelbaar tussen **DE, EN, FR, IT, ES, PL, NL** – met hoog contrast in **Lichte en Donkere** weergave.
  - Ingebouwde **GitHub Update Checker** plus directe links naar **GitHub** en **Buy Me a Coffee**.

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
