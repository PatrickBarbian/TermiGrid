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
