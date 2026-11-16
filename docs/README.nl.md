<div align="center">

<img src="../assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**Klemmenstrook-configurator & Label-exporter voor de schakelkastbouw**

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

**TermiGrid** is een platformonafhankelijke tool (geschreven in Rust met `egui` / `eframe 0.37`) voor het ontwerpen van klemmenstroken in de schakelkastbouw en het exporteren van printklare coderingslijsten voor professionele label- en markeringsprinters.

> **✨ VibeCoded:** Dit project is **VibeCoded** – ontworpen en ontwikkeld met **AI-ondersteuning** in nauwe samenwerking met de projectauteur.

---

## Belangrijkste functies

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
- **Android:** Download de APK en sta installeren vanuit onbekende bronnen toe voor je browser of bestandsbeheerder. Dankzij 16 KB pagina-uitlijning en `resizeableActivity` ook geschikt voor moderne toestellen (zoals Pixel 10 Pro XL) en foldables. Exports worden opgeslagen in `/sdcard/Download/TermiGrid/`.

---

## Snelstartgids

1. **Klemmen toevoegen:** Kies fabrikant en type in de rechterkolom (of gebruik het zoekveld) en klik op *Klem toevoegen* of *Ervoor/Erna invoegen*.
2. **Startnummer & typeweergave instellen:** Stel bovenaan het startnummer in en wissel desgewenst tussen verticale (`-90°`) en horizontale typeweergave.
3. **Teksten aanpassen:** Dubbelklik op een codering in de CAD-editor om handmatige tekst in te voeren (`Enter` bevestigt, leeg veld herstelt automatische nummering).
4. **Accessoires bekijken:** Selecteer klemmen en lees direct eindplaten, bruggen, markers, eindsteunen en N-rails af onder *Bijpassende accessoires*.
5. **Printlijst exporteren:** Kies de printvolgorde (*Per niveau / strook* standaard) en optioneel `'PE' in export opnemen`, en exporteer als `.xlsx` of `.csv`.

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
