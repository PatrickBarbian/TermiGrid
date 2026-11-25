<div align="center">

<img src="../assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**Configurador de regleteros de bornes para carril DIN, planificador CAD, roundtrip EPLAN P8 & exportador de marcado para cuadros eléctricos**

[![Release](https://img.shields.io/github/v/release/PatrickBarbian/TermiGrid)](https://github.com/PatrickBarbian/TermiGrid/releases)
[![VibeCoded](https://img.shields.io/badge/VibeCoded-AI%20Assisted-blueviolet)](#descripción-general-y-nota-vibecoded)
[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-support-orange?logo=buy-me-a-coffee)](https://buymeacoffee.com/02eOWKzm5W)

<p>
  <a href="../README.md">🇩🇪 Deutsch</a> ·
  <a href="README.en.md">🇬🇧 English</a> ·
  <a href="README.fr.md">🇫🇷 Français</a> ·
  <a href="README.it.md">🇮🇹 Italiano</a> ·
  <strong>🇪🇸 Español</strong> ·
  <a href="README.pl.md">🇵🇱 Polski</a> ·
  <a href="README.nl.md">🇳🇱 Nederlands</a>
</p>

</div>

---

## Descripción general y nota VibeCoded

**TermiGrid** es una herramienta de ingeniería rápida y multiplataforma (escrita en Rust Edición 2024 con `egui` / `eframe 0.37`) para diseñar, visualizar y configurar regleteros de bornes para carril DIN en el montaje de cuadros eléctricos. Une de forma fluida el CAD eléctrico (EPLAN Electric P8), la planificación 2D interactiva de taller, el cálculo físico riguroso de accesorios (tapas finales, puentes peine con pines cortados, topes finales) y los sistemas industriales de impresión de etiquetas (WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO).

> **✨ VibeCoded:** Este proyecto ha sido **VibeCoded** – diseñado, programado y perfeccionado con **asistencia de IA** en estrecha colaboración con el autor del proyecto.

---

## Funciones principales

- **782+ tipos de bornes verificados + 26 topes finales para carril DIN + bornes personalizados:**
  - **WAGO:** TOPJOB S (`2000–2016` estándar, `2200–2216` con pulsador, `2102–2116` con palanca), bornes de tres pisos `2002-32xx` con PE en el piso inferior, bornes de instalación (`2003`, `2005`, `2203`, `2205`), `2022` X-COM, `2007` medida/seccionamiento, `280/281` clásico y `285` alta corriente (`35–95 mm²`).
  - **Phoenix Contact:** COMPLETE line, Push-in de `PT 1,5/S` a `PT 35`, bornes para sensores/actuadores `PTIO`, `PTPOWER 35–95`, doble piso `PTTB`/`PTTBS`, conexión lateral `PTV`, bornes de instalación `PTI` / `UTN` / `UTI`, seccionables (`QTC 1,5-TG`, `URTK/SP`, `URTK 6`), resorte `ST`, tornillo `UT`, `UK 5 N`.
  - **Weidmüller:** Klippon Connect Serie A (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), bornes de instalación `AITB`, Serie P de distribución y potencial (`PPV 4/8/16` & `PRV 4/8/16` con marcador único por borne), Serie W (`WDU/WPE/WDK/WTR/WSI/DLI`), Serie Z (`ZDU/ZPE/ZDK/ZDL`).
  - **26 Topes finales para carril TS 35:** Topes terminales originales de fabricantes líderes (WAGO 249-116/117, Phoenix Contact CLIPFIX 35 / E/UK, Weidmüller WEW 35/2, etc.).
  - **Bornes personalizados & Biblioteca de plantillas:** Editor visual de bornes (`custom_specs.json`) y biblioteca de regleteros modelo (`strip_library.json`) para guardar y reutilizar configuraciones frecuentes.

- **Actualizador de base de datos en vivo desde GitHub (Hot-Reload sin reiniciar):**
  - Comprobación automática y no bloqueante al arrancar para detectar actualizaciones de `terminal_specs.json` en GitHub.
  - Botón destacado «Actualización de base de datos disponible»: descarga en 1 clic, validación, almacenamiento local y recarga en caliente instantánea sin reinstalar ni recompilar la aplicación.

- **Asistente local de entrada rápida (Quick-Prompt, 100% fuera de línea sin LLM en la nube):**
  - Entrada rápida en texto natural: ej. `4x Drehstrom Abgang 2,5mm² Installationsklemmen`, `4x 3-pol PTI 2,5`, `6x +24V QUATTRO gebrückt`, `2x Reserve leer`.
  - Vista previa en tiempo real e inserción por lote en el regletero activo con un solo clic.

- **Integración con EPLAN Electric P8 & Roundtrip completo:**
  - **Importación CSV por CLI:** Ejecución desde línea de comandos con `TermiGrid.exe --import-eplan-csv "export.csv"` o desde `Menú → Importar CSV de EPLAN...` con eliminación automática de prefijos configurables (`PXC.`, `WAG.`, `WDM.`).
  - **Importación inteligente de listas de piezas PDF de EPLAN:** Arrastra archivos PDF, reconocimiento automático de bornes de instalación, barras colectoras de neutro y alimentaciones con cálculo de soportes (cada 200 mm).
  - **Paquete de retorno EPLAN en 1 clic:** Genera `_EPLAN_Zubehoer_Import.csv` y `.xml` (con prefijos de fabricante restaurados automáticamente, sin mezclar marcas) y planos DXF individuales por regletero (`/DXF/<BMK>.dxf`) listos para importación de páginas DXF en EPLAN.
  - **Script puente en C# para EPLAN:** Exportación de `TermiGrid_EPLAN_Bridge.cs` en un clic para integración directa en la barra de herramientas de EPLAN P8.
  - **Ajustes de EPLAN ordenados:** Acceso discreto desde `Menú → Interfaz EPLAN...` para configurar prefijos, separadores y carpetas de destino.

- **Editor CAD, Modo de edición automático & Generación de tapas finales:**
  - Vista CAD 2D a escala real con representación de tapas finales (D) y placas separadoras con su grosor físico exacto.
  - **Modo de edición automático:** Se activa automáticamente al realizar cualquier cambio en el CAD.
  - **Cálculo automático de tapas:** Se calculan y actualizan al exportar, en la vista e-Planer o durante el roundtrip con EPLAN.
  - Edición de textos con doble clic (admite etiquetas vacías para reservar espacio en impresoras), desplazamiento táctil y con botón central, zoom fluido Ctrl+rueda.

- **Modo Puentes, Cascada de doble cámara & Instrucciones de corte de pines:**
  - Crea potenciales por color (`+24V`, `GND`, `L1` …) y asígnalos arrastrando sobre los niveles de los bornes.
  - Cálculo real de puentes peine con cascada en cámaras 1 y 2.
  - Instrucciones físicas de corte de pines (`PIN ENTFERNEN BEI`) en bornes saltados.
  - Puentes de cable automáticos (`DB`) al cambiar de piso o de paso entre bornes.

- **Guía integrada con buscador & Generador de prompt para IA (Pestaña `Guía`):**
  - 13 temas detallados con búsqueda de texto completo multitérmino, filtros por categoría y accesos rápidos.
  - Instrucciones paso a paso para añadir bornes personalizados en local (editor visual o JSON).
  - Plantilla de prompt lista para copiar en herramientas de IA (ChatGPT, Claude...) y generar archivos JSON de bornes sin fallos.

- **Exportación lista para impresora (`.xlsx` y `.csv`):**
  - 4 hojas de cálculo Excel especializadas:
    1. `Druckdaten (Flach)` (celdas estrictamente en formato de texto `@` para WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO)
    2. `Etagen-Druckstreifen` (tiras continuas por piso E1..E4)
    3. `Bestückungsplan (Matrix)`
    4. `Stückliste & Montage` (lista de piezas agregada, tapas finales, puentes peine con pines cortados, topes finales y longitud total de carril DIN)

- **Caché en disco fuera de línea para fotos & CDN verificados:**
  - Fotos oficiales de productos mediante CDN sin bloqueos anti-bot (WAGO, Phoenix Contact CaaS, Weidmüller DAM).
  - Caché local persistente en disco (`photo_cache`) – funciona al 100% sin conexión tras la primera descarga.

- **Multiplataforma, 7 idiomas y Temas:**
  - Windows (`.exe` portable), macOS (`.dmg` Universal), Android (`.apk` Universal), Linux.
  - 7 idiomas: Alemán, Inglés, Francés, Italiano, Español, Polaco, Holandés.
  - Tema del Sistema / Claro / Oscuro, Deshacer/Rehacer (`Ctrl+Z` / `Ctrl+Y`), atajos (`Alt+Izquierda/Derecha`, `Ctrl+D`).

---

## Descargas y portabilidad

Todos los paquetes compilados están disponibles en la **[página de Releases](https://github.com/PatrickBarbian/TermiGrid/releases)**:

| Plataforma | Arquitectura / Destino | Archivo | Notas |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100 % portable** (ejecutable único compilado con `+crt-static` – sin instalación ni VCRUNTIME) |
| **macOS** | Universal (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Arrastra el paquete `.app` a Aplicaciones |
| **Android** | Universal (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphones, tablets y plegables (Android 7.0 – 15 / API 24–35, alineación de página de 16 KB, firma v1/v2/v3) |

### Notas por plataforma

- **Windows:** Ejecutar directamente con doble clic desde una memoria USB o carpeta local – no requiere permisos de administrador ni librerías externas.
- **macOS (aplicación sin firma):** En el primer inicio usa **Clic derecho -> Abrir** o autoriza la app en *Ajustes del Sistema -> Privacidad y seguridad*.
- **Android:** Descarga el APK y permite la instalación desde orígenes desconocidos para tu navegador o gestor de archivos. Gracias a la alineación de 16 KB y `resizeableActivity`, es compatible con dispositivos actuales (p. ej. Pixel 10 Pro XL) y plegables. Las exportaciones se guardan directamente en `/sdcard/Download/TermiGrid/`.

---

## Guía rápida

1. **Añadir bornes:** Selecciona fabricante y modelo en el panel derecho (o usa la búsqueda, plantillas de regleteros o el asistente de entrada rápida, ej. `4x Drehstrom Abgang 2,5mm²`).
2. **Importación y Roundtrip EPLAN:** Importa bornes por argumento CLI (`--import-eplan-csv`), importación manual CSV o arrastra listas de piezas PDF.
3. **Edición CAD y Modo Puentes:** Haz doble clic en una etiqueta para modificar el texto, cambia a Modo Puentes para trazar potenciales arrastrando con el ratón.
4. **Accesorios automáticos y Tapas:** Tapas finales, separadores, puentes peine con pines cortados y topes terminales se calculan automáticamente.
5. **Exportación e Impresión:** Exporta a hojas de cálculo Excel de 4 páginas (`.xlsx`), CSV o genera el paquete de retorno EPLAN en 1 clic (`_EPLAN_Zubehoer_Import.csv` y `.xml` + DXF).

---

## Compilar desde el código fuente

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

---

## Enlaces y apoyo

- **Repositorio GitHub:** https://github.com/PatrickBarbian/TermiGrid
- **Releases y descargas:** https://github.com/PatrickBarbian/TermiGrid/releases
- **Apoyo / Buy Me a Coffee:** https://buymeacoffee.com/02eOWKzm5W

---

## Licencia

Publicado bajo la **licencia MIT**.
