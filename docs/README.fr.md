<div align="center">

<img src="../assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**Configurateur de borniers & Exportateur d'étiquettes pour la construction d'armoires électriques**

[![Release](https://img.shields.io/github/v/release/PatrickBarbian/TermiGrid)](https://github.com/PatrickBarbian/TermiGrid/releases)
[![VibeCoded](https://img.shields.io/badge/VibeCoded-AI%20Assisted-blueviolet)](#aperçu--note-vibecoded)
[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-support-orange?logo=buy-me-a-coffee)](https://buymeacoffee.com/02eOWKzm5W)

<p>
  <a href="../README.md">🇩🇪 Deutsch</a> ·
  <a href="README.en.md">🇬🇧 English</a> ·
  <strong>🇫🇷 Français</strong> ·
  <a href="README.it.md">🇮🇹 Italiano</a> ·
  <a href="README.es.md">🇪🇸 Español</a> ·
  <a href="README.pl.md">🇵🇱 Polski</a> ·
  <a href="README.nl.md">🇳🇱 Nederlands</a>
</p>

</div>

---

## Aperçu & Note VibeCoded

**TermiGrid** est un outil multiplateforme (écrit en Rust avec `egui` / `eframe 0.37`) permettant de concevoir des borniers pour armoires électriques et d'exporter des listes de repérage prêtes à imprimer pour les imprimantes industrielles d'étiquettes.

> **✨ VibeCoded :** Ce projet est **VibeCoded** – conçu et développé avec l'**assistance de l'IA** en étroite collaboration avec l'auteur du projet.

---

## Fonctionnalités principales

- **186 références de bornes intégrées + bornes personnalisées :**
  - **WAGO :** TOPJOB S (`2000–2016`), `2200–2216` (avec bouton-poussoir), `2102–2116`, `2022` X-COM et `280/281`
  - **Phoenix Contact :** `PT 1,5/S`, `PT 2,5–35`, `PTPOWER`, `PTTB`, `PTTBS`, `PTV`, `PTI` bornes d'installation ainsi que `ST` & `UT`
  - **Weidmüller :** `A-Series`, `AITB` bornes d'installation, `W-Series`, `DLI` & `Z-Series`
  - **Bornes personnalisées :** Créez et gérez vos propres types de bornes directement dans l'application
- **Numérotation multi-niveaux & alignement PE :**
  - Bornes d'installation et multi-étages (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, etc.) : Tout **raccordement PE** présent est toujours aligné sur une ligne PE dédiée **sous l'étage principal inférieur**, afin de garder les repères numériques sur une ligne horizontale claire.
  - **Exemple multi-étages :** Sur une barrette de bornes à 2 étages, l'étage 1 (bas) numérote automatiquement en impair (`1, 3, 5 …`) et l'étage 2 (haut) en pair (`2, 4, 6 …`) ; les bornes suivantes à 1 étage poursuivent sans rupture sur l'étage 1 (`7, 8, 9 …`).
- **Éditeur CAO 2D interactif (sans encombrement de rail DIN) :**
  - Vue CAO épurée sans rail DIN superflu, avec logos fabricants intégrés (**WAGO**, **Phoenix Contact**, **Weidmüller**).
  - **Affichage des références :** Au choix **vertical (orienté à `-90°`)** par borne ou **regroupé horizontalement** sous le bornier.
  - **Édition directe (inline) :** Un **double-clic** sur un repère permet de le modifier directement (`Entrée` = valider, champ vide = restaurer la numérotation auto, `Échap` = annuler).
  - **Sélection multiple :** Clic simple, `Ctrl+Clic`, `Maj+Clic` et sélection par rectangle pour réaffecter un type, insérer avant/après ou supprimer en lot.
- **Accessoires compatibles affichés automatiquement :**
  - Le panneau latéral droit affiche automatiquement les accessoires adaptés aux bornes sélectionnées : **flasques d'extrémité**, **peignes de pontage / jonctions**, **bandelettes de marquage**, **butées d'arrêt** et **barres collectrices N**.
- **Export prêt pour l'impression (`.xlsx` & `.csv`) :**
  - **Excel (`.xlsx`) :** 3 feuilles structurées :
    1. `Druckfolge` / Ordre d'impression (`ByTierStrip` par défaut – tous les repères d'un même niveau regroupés en continu, d'abord tout le niveau 1, puis le niveau 2, puis le niveau 3 ; ordre par colonne ou par bloc et inclusion de `PE` en option)
    2. `Ebenen-Spalten` / Colonnes par niveau
    3. `Ebenen-Matrix` / Matrice des niveaux
    - Toutes les cellules sont formatées strictement en **Texte Excel (`@`)** afin que les zéros initiaux (`01`) ou notations avec point (`1.1`) ne soient jamais convertis en nombres ou en dates.
  - **CSV (`.csv`) :** UTF-8 avec BOM et point-virgule `;` par défaut.
  - **Fichiers projet :** Enregistrement et chargement au format `.termi` ou `.json` avec textes manuels et bornes personnalisées.
- **7 langues, thèmes & vérificateur de mises à jour intégré :**
  - Interface disponible en **DE, EN, FR, IT, ES, PL, NL** – avec thèmes **Clair et Sombre** à fort contraste.
  - **Vérificateur de mises à jour GitHub** intégré ainsi que liens directs vers **GitHub** et **Buy Me a Coffee**.

---

## Téléchargements & Portabilité

Tous les paquets prêts à l'emploi sont disponibles sur la **[page Releases](https://github.com/PatrickBarbian/TermiGrid/releases)** :

| Plateforme | Architecture / Cible | Fichier | Remarques |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-windows-x86_64.exe` | **100 % portable** (exécutable unique compilé avec `+crt-static` – sans installation ni VCRUNTIME) |
| **macOS** | Universel (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-macos-universal.dmg` | Glisser le bundle `.app` dans Applications |
| **Android** | Universel (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-android-universal.apk` | Smartphones, tablettes & pliables (Android 7.0 – 15 / API 24–35, alignement mémoire 16 Ko, signature v1/v2/v3) |

### Notes par plateforme

- **Windows :** Lancer directement par double-clic depuis une clé USB ou un dossier local – aucun droit administrateur ni bibliothèque externe requis.
- **macOS (application non signée) :** Au premier lancement, utiliser **Clic droit -> Ouvrir** ou autoriser l'ouverture dans *Réglages Système -> Confidentialité et sécurité*.
- **Android :** Télécharger l'APK et autoriser l'installation depuis des sources inconnues pour votre navigateur/gestionnaire de fichiers. Grâce à l'alignement 16 Ko et à `resizeableActivity`, l'app fonctionne aussi sur les appareils récents (ex. Pixel 10 Pro XL) et les téléphones pliables. Les exports sont enregistrés dans `/sdcard/Download/TermiGrid/`.

---

## Guide rapide

1. **Ajouter des bornes :** Choisir le fabricant et le modèle dans le panneau de droite (ou via la recherche), puis cliquer sur *Ajouter une borne* ou *Insérer avant/après*.
2. **Définir le numéro de départ & l'affichage du type :** Régler le numéro initial en haut et basculer si besoin entre l'affichage vertical (`-90°`) et horizontal.
3. **Modifier les repères :** Double-cliquer sur une étiquette dans l'éditeur CAO pour saisir un texte manuel (`Entrée` pour valider, champ vide pour revenir à la numérotation automatique).
4. **Consulter les accessoires :** Sélectionner les bornes et lire directement flasques, ponts, repères, butées et barres N dans la section *Accessoires compatibles*.
5. **Exporter la liste d'impression :** Choisir l'ordre d'impression (*Par niveau / en bande* par défaut) ainsi que l'option `'PE' dans l'export`, puis exporter en `.xlsx` ou `.csv`.

---

## Compiler depuis les sources

Prérequis : chaîne d'outils Rust récente (`rustup` / `cargo`).

```bash
# Contrôles qualité & tests
cargo clippy --all-targets -- -D warnings
cargo test

# Build Release bureau (Windows / Linux / macOS natif)
cargo build --release --bin termigrid

# macOS Universel (Intel + Apple Silicon)
cargo build --release --bin termigrid --target x86_64-apple-darwin
cargo build --release --bin termigrid --target aarch64-apple-darwin

# Android Universal APK (avec cargo-apk & Android NDK installés)
cargo apk build --release --lib
```

---

## Liens & Soutien

- **Dépôt GitHub :** https://github.com/PatrickBarbian/TermiGrid
- **Releases & Téléchargements :** https://github.com/PatrickBarbian/TermiGrid/releases
- **Soutien / Buy Me a Coffee :** https://buymeacoffee.com/02eOWKzm5W

---

## Licence

Publié sous **licence MIT**.
