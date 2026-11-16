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

- **186 tipos de bornes integrados + bornes personalizados:**
  - **WAGO:** TOPJOB S (`2000–2016`), `2200–2216` (con pulsador), `2102–2116`, `2022` X-COM y `280/281`
  - **Phoenix Contact:** `PT 1,5/S`, `PT 2,5–35`, `PTPOWER`, `PTTB`, `PTTBS`, `PTV`, `PTI` bornes de instalación, además de `ST` y `UT`
  - **Weidmüller:** `A-Series`, `AITB` bornes de instalación, `W-Series`, `DLI` y `Z-Series`
  - **Bornes personalizados:** Crea y gestiona tus propios modelos de bornes directamente dentro de la aplicación
- **Numeración multinivel y alineación PE:**
  - Bornes de instalación y multipiso (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, etc.): Cualquier **conexión PE** existente se alinea siempre en su propia fila PE **debajo del nivel principal inferior**, manteniendo los números consecutivos sobre una línea horizontal limpia.
  - **Ejemplo multipiso:** En un regletero de bornes de 2 pisos, el nivel 1 (inferior) numera automáticamente los impares (`1, 3, 5 …`) y el nivel 2 (superior) los pares (`2, 4, 6 …`); los bornes siguientes de 1 piso continúan sin cortes en el nivel 1 (`7, 8, 9 …`).
- **Editor CAD 2D interactivo (sin ruido visual de carril DIN):**
  - Vista CAD limpia sin carril DIN innecesario, con logotipos integrados de fabricantes (**WAGO**, **Phoenix Contact**, **Weidmüller**).
  - **Visualización de tipo:** Elige entre **vertical (rotado `-90°`)** por cada borne o **agrupado horizontalmente** debajo del regletero.
  - **Edición en línea:** Un **doble clic** sobre cualquier etiqueta permite editarla al instante (`Enter` = aplicar, campo vacío = restaurar numeración automática, `Esc` = cancelar).
  - **Selección múltiple:** Clic simple, `Ctrl+Clic`, `Shift+Clic` y selección por marco para reasignar tipos, insertar antes/después o eliminar en bloque.
- **Accesorios compatibles mostrados automáticamente:**
  - El panel derecho de Propiedades muestra automáticamente los accesorios compatibles para los bornes seleccionados: **tapas finales**, **puentes de conexión**, **tiras de marcado**, **topes finales** y **barras colectoras N**.
- **Exportación lista para impresora (`.xlsx` y `.csv`):**
  - **Excel (`.xlsx`):** 3 hojas estructuradas:
    1. `Druckfolge` / Secuencia de impresión (`ByTierStrip` por defecto – exporta cada nivel de forma contigua en una tira, p. ej. primero todo el nivel 1, luego el nivel 2 y después el nivel 3; orden por columna o bloque y opción de incluir/excluir `PE`)
    2. `Ebenen-Spalten` / Columnas por nivel
    3. `Ebenen-Matrix` / Matriz de niveles
    - Todas las celdas se formatean estrictamente como **Texto Excel (`@`)** para que los ceros a la izquierda (`01`) o notaciones con punto (`1.1`) nunca se conviertan en números ni fechas.
  - **CSV (`.csv`):** UTF-8 con BOM y punto y coma `;` como separador predeterminado.
  - **Archivos de proyecto:** Guarda y carga `.termi` o `.json` incluyendo textos manuales y bornes personalizados.
- **7 idiomas, temas y comprobador de actualizaciones integrado:**
  - Interfaz intercambiable entre **DE, EN, FR, IT, ES, PL, NL** – con temas **Claro y Oscuro** de alto contraste.
  - **Comprobador de actualizaciones de GitHub** integrado junto con enlaces directos a **GitHub** y **Buy Me a Coffee**.

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
