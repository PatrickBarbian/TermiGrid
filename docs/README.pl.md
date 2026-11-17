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
