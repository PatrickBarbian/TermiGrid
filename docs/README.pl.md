<div align="center">

<img src="../assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**Konfigurator listew zaciskowych na szynę DIN, planer CAD, roundtrip EPLAN P8 & eksporter oznaczników do szaf sterowniczych**

[![Release](https://img.shields.io/github/v/release/PatrickBarbian/TermiGrid)](https://github.com/PatrickBarbian/TermiGrid/releases)
[![VibeCoded](https://img.shields.io/badge/VibeCoded-AI%20Assisted-blueviolet)](#przegląd-i-informacja-vibecoded)
[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-support-orange?logo=buy-me-a-coffee)](https://buymeacoffee.com/02eOWKzm5W)

<p>
  <a href="../README.md">🇩🇪 Deutsch</a> ·
  <a href="README.en.md">🇬🇧 English</a> ·
  <a href="README.fr.md">🇫🇷 Français</a> ·
  <a href="README.it.md">🇮🇹 Italiano</a> ·
  <a href="README.es.md">🇪🇸 Español</a> ·
  <strong>🇵🇱 Polski</strong> ·
  <a href="README.nl.md">🇳🇱 Nederlands</a>
</p>

</div>

---

## Przegląd i informacja VibeCoded

**TermiGrid** to szybkie, wieloplatformowe narzędzie inżynieryjne (napisane w Rust Edycja 2024 z `egui` / `eframe 0.37`) do projektowania, wizualizacji i konfigurowania listew zaciskowych na szynie DIN w prefabrykacji szaf sterowniczych. Łączy ono świat projektowania CAD (EPLAN Electric P8), interaktywnego planowania 2D na warsztacie, precyzyjnego fizycznego doboru akcesoriów (płytki końcowe, mostki grzebieniowe z wyłamywanymi pinami, trzymacze końcowe) oraz przemysłowych systemów drukowania oznaczników (WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO).

> **✨ VibeCoded:** Ten projekt powstał jako **VibeCoded** – został zaprojektowany, zaprogramowany i dopracowany z **pomocą AI** w ścisłej współpracy z autorem projektu.

---

## Główne funkcje

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

---

## Pobieranie i przenośność

Wszystkie gotowe pakiety znajdują się na **[stronie Releases](https://github.com/PatrickBarbian/TermiGrid/releases)**:

| Platforma | Architektura / Cel | Plik | Uwagi |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100% przenośny** (pojedynczy plik `.exe` z `+crt-static` – bez instalacji i bez VCRUNTIME) |
| **macOS** | Uniwersalny (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Przeciągnij pakiet `.app` do folderu Aplikacje |
| **Android** | Uniwersalny (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartfony, tablety i urządzenia składane (Android 7.0 – 15 / API 24–35, wyrównanie stron 16 KB, podpis v1/v2/v3) |

### Wskazówki dla platform

- **Windows:** Uruchom bezpośrednio dwuklikiem z pendrive'a lub folderu lokalnego – nie wymaga uprawnień administratora ani dodatkowych bibliotek uruchomieniowych.
- **macOS (niepodpisana aplikacja):** Przy pierwszym uruchomieniu użyj **Prawy przycisk myszy -> Otwórz** lub zezwól na uruchomienie w *Ustawienia systemowe -> Prywatność i ochrona*.
- **Android:** Pobierz APK i zezwól na instalację z nieznanych źródeł dla przeglądarki lub menedżera plików. Dzięki wyrównaniu stron 16 KB i `resizeableActivity` aplikacja działa także na nowych urządzeniach (np. Pixel 10 Pro XL) i ekranach składanych. Eksporty zapisują się bezpośrednio w `/sdcard/Download/TermiGrid/`.

---

## Szybka instrukcja

1. **Dodaj złączki:** W prawej kolumnie wybierz producenta i typ (lub użyj wyszukiwarki, szablonów listew albo asystenta szybkiego wprowadzania, np. `4x Drehstrom Abgang 2,5mm²`).
2. **Import i Roundtrip EPLAN:** Importuj złączki przez parametr CLI (`--import-eplan-csv`), ręczny import CSV lub przeciągnij zestawienie PDF.
3. **Edycja CAD & Tryb Mostków:** Kliknij dwukrotnie oznacznik, aby zmienić tekst, przełącz w Tryb Mostków, by malować potencjały przeciągnięciem myszy.
4. **Automatyczne akcesoria & Płytki końcowe:** Płytki końcowe, przegrody, mostki z wyłamywanymi pinami i trzymacze końcowe są wyliczane automatycznie.
5. **Eksport i Druk:** Eksportuj do 4-arkuszowego pliku Excel (`.xlsx`), CSV lub wygeneruj pakiet zwrotny EPLAN za 1 kliknięciem (`_EPLAN_Zubehoer_Import.csv` & `.xml` + rysunki DXF).

---

## Kompilacja ze źródeł

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

---

## Linki i wsparcie

- **Repozytorium GitHub:** https://github.com/PatrickBarbian/TermiGrid
- **Releases i pobieranie:** https://github.com/PatrickBarbian/TermiGrid/releases
- **Wsparcie / Buy Me a Coffee:** https://buymeacoffee.com/02eOWKzm5W

---

## Licencja

Udostępniono na **licencji MIT**.
