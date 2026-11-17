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
