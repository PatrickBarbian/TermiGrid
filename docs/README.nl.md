<div align="center">

<img src="../assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**DIN-rail klemmenstrook-configurator, CAD-planner, EPLAN P8 roundtrip & markeringsexporter voor schakelkastbouw**

[![Release](https://img.shields.io/github/v/release/PatrickBarbian/TermiGrid)](https://github.com/PatrickBarbian/TermiGrid/releases)
[![VibeCoded](https://img.shields.io/badge/VibeCoded-AI%20Assisted-blueviolet)](#overzicht--vibecoded-vermelding)
[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-support-orange?logo=buy-me-a-coffee)](https://buymeacoffee.com/02eOWKzm5W)

<p>
  <a href="../README.md">🇩🇪 Deutsch</a> ·
  <a href="README.en.md">🇬🇧 English</a> ·
  <a href="README.fr.md">🇫🇷 Français</a> ·
  <a href="README.it.md">🇮🇹 Italiano</a> ·
  <a href="README.es.md">🇪🇸 Español</a> ·
  <a href="README.pl.md">🇵🇱 Polski</a> ·
  <strong>🇳🇱 Nederlands</strong>
</p>

</div>

---

## Overzicht & VibeCoded-vermelding

**TermiGrid** is een snelle, platformonafhankelijke engineeringtool (geschreven in Rust Editie 2024 met `egui` / `eframe 0.37`) voor het ontwerpen, visualiseren en configureren van DIN-rail klemmenstroken in de schakelkastbouw. Het overbrugt naadloos het gat tussen elektrotechnisch CAD (EPLAN Electric P8), interactieve 2D-werkplaatsplanning, nauwkeurige fysieke accessoireberekening (eindplaten, kamverbinders met uitgebroken pinnen, eindsteunen) en industriële markeringsprinters (WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO).

> **✨ VibeCoded:** Dit project is **VibeCoded** – ontworpen, geprogrammeerd en verfijnd met **AI-ondersteuning** in nauwe samenwerking met de projectauteur.

---

## Belangrijkste functies

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

---

## Downloads & Draagbaarheid

Alle kant-en-klare pakketten staan op de **[Releases-pagina](https://github.com/PatrickBarbian/TermiGrid/releases)**:

| Platform | Architectuur / Doel | Bestand | Opmerkingen |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100% portable** (los `.exe`-bestand met `+crt-static` – geen installatie en geen VCRUNTIME nodig) |
| **macOS** | Universeel (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Sleep de `.app`-bundel naar Programma's |
| **Android** | Universeel (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphones, tablets & foldables (Android 7.0 – 15 / API 24–35, 16 KB pagina-uitlijning, v1/v2/v3 ondertekend) |

### Platformopmerkingen

- **Windows:** Direct starten met een dubbelklik vanaf USB-stick of lokale map – geen beheerdersrechten of externe runtimebibliotheken nodig.
- **macOS (niet-ondertekende app):** Gebruik bij de eerste start **Rechtermuisklik -> Open** of sta openen toe via *Systeeminstellingen -> Privacy en beveiliging*.
- **Android:** Download de APK en sta installeren vanuit onbekende bronnen toe voor je browser of bestandsbeheerder. Dankzij 16 KB pagina-uitlijning en `resizeableActivity` ook geschikt voor moderne toestellen (zoals Pixel 10 Pro XL) en foldables. Exports worden direct opgeslagen in `/sdcard/Download/TermiGrid/`.

---

## Snelstartgids

1. **Klemmen toevoegen:** Kies fabrikant en type in de rechterkolom (of gebruik het zoekveld, strooksjablonen of de snel-invoerassistent, bijv. `4x Drehstrom Abgang 2,5mm²`).
2. **Import & Roundtrip EPLAN:** Importeer klemmen via CLI (`--import-eplan-csv`), handmatige CSV-import of sleep PDF-stuklijsten in het venster.
3. **CAD bewerken & Brug-modus:** Dubbelklik op een codering om tekst aan te passen, schakel naar de Brug-modus om potentialen aan te brengen door te slepen.
4. **Automatische accessoires & Eindplaten:** Eindplaten, scheidingsplaten, kamverbinders met afbreekbare pinnen en eindsteunen worden automatisch berekend.
5. **Exporteren & Printen:** Exporteer naar 4-delige Excel-werkbladen (`.xlsx`), CSV of genereer het EPLAN retourpakket met 1 klik (`_EPLAN_Zubehoer_Import.csv` & `.xml` + DXF-tekeningen).

---

## Zelf compileren (Build from Source)

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

---

## Links & Ondersteuning

- **GitHub Repository:** https://github.com/PatrickBarbian/TermiGrid
- **Releases & Downloads:** https://github.com/PatrickBarbian/TermiGrid/releases
- **Ondersteuning / Buy Me a Coffee:** https://buymeacoffee.com/02eOWKzm5W

---

## Licentie

Uitgebracht onder de **MIT-licentie**.
