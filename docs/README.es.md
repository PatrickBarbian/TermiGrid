<div align="center">

<img src="../assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**Configurador de regleteros de bornes y exportador de etiquetas para cuadros eléctricos**

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

**TermiGrid** es una herramienta multiplataforma (escrita en Rust con `egui` / `eframe 0.37`) para planificar regleteros de bornes en cuadros eléctricos y exportar listas de etiquetado listas para imprimir en impresoras industriales de marcado.

> **✨ VibeCoded:** Este proyecto ha sido **VibeCoded** – diseñado y desarrollado con **asistencia de IA** en estrecha colaboración con el autor del proyecto.

---

## Funciones principales

- **277 tipos de bornes integrados + bornes personalizados:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (pulsador), `2102–2116` (palanca), `2022` X-COM, `2007` medida/prueba, `280/281` y `285` alta corriente (`35–95 mm²`)
  - **Phoenix Contact:** `PT 1,5/S` incl. `PTIO`, `PT 2,5–35`, `PTPOWER 35–95`, `PTTB`, `PTTBS`, `PTV`, `PTI`/`UTI` instalación, `ST`, `UT` & `UK 5 N`
  - **Weidmüller:** `A-Series` (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), `AITB` instalación, `W-Series` (`WDU/WPE/WDK/WTR/WSI/DLI`) & `Z-Series` (`ZDU/ZPE/ZDK/ZDL`)
  - **Bornes personalizados:** Crea tus propios bornes en la app incluyendo `1×` o `2×` cámaras de puente
- **Numeración multinivel, modos N/PE y símbolos especiales:**
  - Bornes de instalación y multipiso (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, etc.): Cualquier **conexión PE** se alinea siempre en su propia fila **debajo del nivel principal inferior**.
  - **Numeración N y PE:** Selecciona por regletero entre fijo (`N`/`PE`), contador compartido (`1, 2, …`), `1N, 2N` / `1PE, 2PE` compartidos o **contador propio N/PE (`1N, 2N …` / `1PE, 2PE …`)**.
  - **Prefijo / Sufijo / Símbolos en lote:** Marca varios bornes y añade `+`, `-`, `~`, `=`, `.1`, `Ω`, etc. antes o después del número en todos o en un piso específico (se conserva al renumerar).
- **Modo Puentes y Cálculo de Potenciales:**
  - Crea **potenciales** por color (`+24V`, `GND`, `L1` …) y pulsa los niveles de bornes directamente en el CAD.
  - **Cálculo automático:** Determina polos de puente peine (`2002-405`, `FBS 5-5`, `ZQV`), marca **pines a quitar (`Quitar pin ×`)**, planifica **puentes de cable (`DB`)** entre pisos (`E1 <-> E2`) o cambios de paso, y distingue **1 cámara** (puente de cable si supera polos máx.) vs **2 cámaras** (cascada en cámaras 1 y 2).
- **Editor CAD 2D interactivo (Escritorio y Táctil):**
  - Vista CAD sin carril DIN innecesario con logotipos integrados (**WAGO**, **Phoenix Contact**, **Weidmüller**), selector (**Mover vista**, **Selección**, **Modo Puentes**), desplazamiento táctil y arrastre con botón central/derecho.
  - **Visualización y edición directa:** Vertical `-90°` o horizontal agrupada; **doble clic** para editar (`Enter` = aplicar, vacío = automático, `Esc` = cancelar).
- **Vista previa del producto, enlace del fabricante y accesorios:**
  - Muestra **foto del producto (cuando está disponible) o perfil técnico**, **número de cámaras**, **enlace directo a la página del fabricante**, además de **tapas, puentes, marcadores, topes finales y barras N**.
- **Exportación lista para impresora (`.xlsx` & `.csv`) y Proyectos (`.termi` / `.json`):**
  - **Excel (`.xlsx`):** 3 hojas (`Druckfolge` / `ByTierStrip` por defecto, `Ebenen-Spalten`, `Ebenen-Matrix`), todas las celdas formateadas como **Texto Excel (`@`)**.
- **7 idiomas, temas y comprobador de actualizaciones en vivo:**
  - **DE, EN, FR, IT, ES, PL, NL** – Claro/Oscuro de alto contraste, comprobación de GitHub en Escritorio y Android, enlaces directos a **GitHub** y **Buy Me a Coffee**.

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
- **Android:** Descarga el APK y permite la instalación desde orígenes desconocidos para tu navegador o gestor de archivos. Gracias a la alineación de 16 KB y `resizeableActivity`, es compatible con dispositivos actuales (p. ej. Pixel 10 Pro XL) y plegables. Las exportaciones se guardan en `/sdcard/Download/TermiGrid/`.

---

## Guía rápida

1. **Añadir bornes:** Selecciona fabricante y modelo en el panel derecho (o usa la búsqueda) y haz clic en *Añadir borne* o *Insertar antes/después*.
2. **Ajustar número inicial y visualización:** Configura el número inicial en la parte superior y alterna entre visualización vertical (`-90°`) y horizontal según prefieras.
3. **Editar etiquetas:** Haz doble clic en cualquier etiqueta del editor CAD para escribir un texto manual (`Enter` para confirmar, dejar vacío para volver a la numeración automática).
4. **Consultar accesorios:** Selecciona los bornes y consulta al instante tapas finales, puentes, marcadores, topes finales y barras N en *Accesorios compatibles*.
5. **Exportar lista de impresión:** Elige el orden (*Por nivel / tira* por defecto) y si deseas incluir `'PE'`, y exporta en `.xlsx` o `.csv`.

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
