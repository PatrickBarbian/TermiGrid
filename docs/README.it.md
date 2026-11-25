<div align="center">

<img src="../assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**Configuratore di morsettiere su guida DIN, progettatore CAD, roundtrip EPLAN P8 & esportatore di siglatura per quadri elettrici**

[![Release](https://img.shields.io/github/v/release/PatrickBarbian/TermiGrid)](https://github.com/PatrickBarbian/TermiGrid/releases)
[![VibeCoded](https://img.shields.io/badge/VibeCoded-AI%20Assisted-blueviolet)](#panoramica--nota-vibecoded)
[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-support-orange?logo=buy-me-a-coffee)](https://buymeacoffee.com/02eOWKzm5W)

<p>
  <a href="../README.md">🇩🇪 Deutsch</a> ·
  <a href="README.en.md">🇬🇧 English</a> ·
  <a href="README.fr.md">🇫🇷 Français</a> ·
  <strong>🇮🇹 Italiano</strong> ·
  <a href="README.es.md">🇪🇸 Español</a> ·
  <a href="README.pl.md">🇵🇱 Polski</a> ·
  <a href="README.nl.md">🇳🇱 Nederlands</a>
</p>

</div>

---

## Panoramica & Nota VibeCoded

**TermiGrid** è uno strumento di ingegneria rapido e multipiattaforma (sviluppato in Rust Edizione 2024 con `egui` / `eframe 0.37`) per progettare, visualizzare e configurare morsettiere su guida DIN nei quadri elettrici. Collega in modo fluido il CAD elettrico (EPLAN Electric P8), la pianificazione 2D interattiva da officina, il calcolo fisico preciso degli accessori (piastre terminali, ponticelli a pettine con pin rimossi, fermamorsetti) e i sistemi industriali di stampa siglature (WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO).

> **✨ VibeCoded:** Questo progetto è **VibeCoded** – ideato, sviluppato e costantemente perfezionato con il **supporto dell'IA** in stretta collaborazione con l'autore del progetto.

---

## Funzionalità principali

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

---

## Download & Portabilità

Tutti i pacchetti pronti sono disponibili nella **[pagina Releases](https://github.com/PatrickBarbian/TermiGrid/releases)**:

| Piattaforma | Architettura / Target | File (Release) | Note |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-v1.9.1-windows-x86_64.exe`<br/>*(oppure `TermiGrid-windows-x86_64.exe`)* | **100% portabile** (eseguibile singolo con `+crt-static` – nessuna installazione, nessun VCRUNTIME richiesto) |
| **macOS** | Universale (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-v1.9.1-macos-universal.dmg`<br/>*(oppure `TermiGrid-macos-universal.dmg`)* | Trascina il bundle `.app` in Applicazioni |
| **Android** | Universale (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-v1.9.1-android-universal.apk`<br/>*(oppure `TermiGrid-android-universal.apk`)* | Smartphone, tablet & pieghevoli (Android 7.0 – 15 / API 24–35, allineamento pagine 16 KB, keystore stabile, firma v1/v2/v3) |


### Note per piattaforma

- **Windows:** Avviabile direttamente con doppio clic da chiavetta USB o cartella locale – non richiede diritti di amministratore né runtime esterni.
- **macOS (app non firmata):** Al primo avvio usa **Clic destro -> Apri** oppure consenti l'esecuzione in *Impostazioni di Sistema -> Privacy e sicurezza*.
- **Android:** Scarica l'APK e consenti l'installazione da origini sconosciute per il browser/file manager. Grazie all'allineamento a 16 KB e a `resizeableActivity`, è compatibile anche con dispositivi moderni (es. Pixel 10 Pro XL) e pieghevoli. I file esportati vengono salvati direttamente in `/sdcard/Download/TermiGrid/`.

---

## Guida rapida

1. **Aggiungere morsetti:** Scegli produttore e modello nella colonna destra (o usa la ricerca, i modelli di morsettiera o l'assistente per inserimento rapido, es. `4x Drehstrom Abgang 2,5mm²`).
2. **Import & Roundtrip EPLAN:** Importa i morsetti tramite argomento CLI (`--import-eplan-csv`), importazione manuale CSV o trascina distinte PDF.
3. **Modifica CAD & Modo Ponticelli:** Fai doppio clic su una siglatura per modificare il testo, passa alla modalità Ponticelli per tracciare potenziali trascinando il mouse.
4. **Accessori automatici & Piastre terminali:** Piastre terminali, separatori, ponticelli con pin tagliati e fermamorsetti vengono calcolati automaticamente.
5. **Esportazione & Stampa:** Esporta in cartelle di lavoro Excel a 4 fogli (`.xlsx`), CSV o crea il pacchetto di ritorno EPLAN in 1 clic (`_EPLAN_Zubehoer_Import.csv` & `.xml` + disegni DXF).

---

## Compilare dai sorgenti

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

---

## Link & Supporto

- **Repository GitHub:** https://github.com/PatrickBarbian/TermiGrid
- **Releases & Download:** https://github.com/PatrickBarbian/TermiGrid/releases
- **Supporto / Buy Me a Coffee:** https://buymeacoffee.com/02eOWKzm5W

---

## Licenza

Distribuito con **licenza MIT**.
