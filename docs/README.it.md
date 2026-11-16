<div align="center">

<img src="../assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**Configuratore di morsettiere ed esportatore di etichette per quadri elettrici**

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

**TermiGrid** è uno strumento multipiattaforma (scritto in Rust con `egui` / `eframe 0.37`) per progettare morsettiere nei quadri elettrici ed esportare liste di siglatura pronte per la stampa su stampanti industriali per etichette.

> **✨ VibeCoded:** Questo progetto è **VibeCoded** – ideato e sviluppato con il **supporto dell'IA** in stretta collaborazione con l'autore del progetto.

---

## Funzionalità principali

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

---

## Download & Portabilità

Tutti i pacchetti pronti sono disponibili nella **[pagina Releases](https://github.com/PatrickBarbian/TermiGrid/releases)**:

| Piattaforma | Architettura / Target | File | Note |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100% portabile** (eseguibile singolo con `+crt-static` – nessuna installazione, nessun VCRUNTIME richiesto) |
| **macOS** | Universale (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Trascina il bundle `.app` in Applicazioni |
| **Android** | Universale (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphone, tablet & pieghevoli (Android 7.0 – 15 / API 24–35, allineamento pagine 16 KB, firma v1/v2/v3) |

### Note per piattaforma

- **Windows:** Avviabile direttamente con doppio clic da chiavetta USB o cartella locale – non richiede diritti di amministratore né runtime esterni.
- **macOS (app non firmata):** Al primo avvio usa **Clic destro -> Apri** oppure consenti l'esecuzione in *Impostazioni di Sistema -> Privacy e sicurezza*.
- **Android:** Scarica l'APK e consenti l'installazione da origini sconosciute per il browser/file manager. Grazie all'allineamento a 16 KB e a `resizeableActivity`, è compatibile anche con dispositivi moderni (es. Pixel 10 Pro XL) e pieghevoli. I file esportati vengono salvati in `/sdcard/Download/TermiGrid/`.

---

## Guida rapida

1. **Aggiungere morsetti:** Scegli produttore e modello nella colonna destra (o usa la ricerca) e clicca su *Aggiungi morsetto* oppure *Inserisci prima/dopo*.
2. **Impostare numero iniziale & visualizzazione tipo:** Imposta il numero iniziale in alto e alterna tra visualizzazione verticale (`-90°`) e orizzontale.
3. **Modificare i testi:** Fai doppio clic su una siglatura nell'editor CAD per inserire un testo manuale (`Invio` per confermare, campo vuoto per ripristinare l'automatico).
4. **Verificare gli accessori:** Seleziona i morsetti e controlla subito piastre terminali, ponticelli, targhette, fermamorsetti e barre N nella sezione *Accessori compatibili*.
5. **Esportare la lista di stampa:** Seleziona l'ordine di stampa (*Per piano / nastro* come predefinito) e l'opzione `'PE' nell'esportazione`, quindi esporta in `.xlsx` o `.csv`.

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
