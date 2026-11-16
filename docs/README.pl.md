<div align="center">

<img src="../assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**Konfigurator listew zaciskowych i eksporter etykiet do budowy szaf sterowniczych**

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

**TermiGrid** to wieloplatformowe narzędzie (napisane w Rust z użyciem `egui` / `eframe 0.37`) do projektowania listew zaciskowych w szafach sterowniczych oraz eksportu gotowych do druku list opisowych dla przemysłowych drukarek oznaczników.

> **✨ VibeCoded:** Ten projekt powstał jako **VibeCoded** – został zaprojektowany i stworzony z **pomocą AI** w ścisłej współpracy z autorem projektu.

---

## Główne funkcje

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
- **Android:** Pobierz APK i zezwól na instalację z nieznanych źródeł dla przeglądarki lub menedżera plików. Dzięki wyrównaniu stron 16 KB i `resizeableActivity` aplikacja działa także na nowych urządzeniach (np. Pixel 10 Pro XL) i ekranach składanych. Eksporty zapisują się w `/sdcard/Download/TermiGrid/`.

---

## Szybka instrukcja

1. **Dodaj złączki:** W prawej kolumnie wybierz producenta i typ (lub użyj wyszukiwarki) i kliknij *Dodaj złączkę* albo *Wstaw przed/za*.
2. **Ustaw numer startowy i wyświetlanie typu:** Ustaw numer początkowy na górze i w razie potrzeby przełącz między pionowym (`-90°`) a poziomym wyświetlaniem typu.
3. **Edytuj opisy:** Kliknij dwukrotnie etykietę w edytorze CAD, aby wpisać własny tekst (`Enter` potwierdza, puste pole przywraca automatyczną numerację).
4. **Sprawdź akcesoria:** Zaznacz złączki i odczytaj pasujące płytki końcowe, mostki, oznaczniki, trzymacze końcowe oraz szyny N w sekcji *Pasujące akcesoria*.
5. **Eksportuj listę druku:** Wybierz kolejność druku (domyślnie *Poziomami / taśmowo*) oraz opcjonalnie `'PE' w eksporcie`, a następnie eksportuj jako `.xlsx` lub `.csv`.

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
