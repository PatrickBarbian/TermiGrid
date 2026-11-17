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

- **277 références de bornes intégrées + bornes personnalisées :**
  - **WAGO :** TOPJOB S (`2000–2016`), `2200–2216` (bouton-poussoir), `2102–2116` (levier), `2022` X-COM, `2007` mesure/test, `280/281` et `285` fort courant (`35–95 mm²`)
  - **Phoenix Contact :** `PT 1,5/S` incl. `PTIO`, `PT 2,5–35`, `PTPOWER 35–95`, `PTTB`, `PTTBS`, `PTV`, `PTI`/`UTI` bornes d'installation, `ST`, `UT` & `UK 5 N`
  - **Weidmüller :** `A-Series` (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), `AITB` installation, `W-Series` (`WDU/WPE/WDK/WTR/WSI/DLI`) & `Z-Series` (`ZDU/ZPE/ZDK/ZDL`)
  - **Bornes personnalisées :** Créez vos propres bornes directement dans l'app avec `1×` ou `2×` chambres de pontage
- **Numérotation multi-niveaux, modes N/PE & caractères spéciaux :**
  - Bornes d'installation et multi-étages (`2003`, `2005`, `2203`, `2206`, `PTI`, `AITB`, `DLI`, etc.) : Tout **raccordement PE** présent est toujours aligné sur une ligne dédiée **sous l'étage principal inférieur**.
  - **Numérotation N & PE :** Basculez par bornier entre fixe (`N`/`PE`), compteur partagé (`1, 2, …`), `1N, 2N` / `1PE, 2PE` partagés ou **compteur N/PE propre (`1N, 2N …` / `1PE, 2PE …`)**.
  - **Préfixe / Suffixe / Caractères spéciaux par lot :** Sélectionnez plusieurs bornes et ajoutez `+`, `-`, `~`, `=`, `.1`, `Ω`, etc. avant ou après le repère (conservé lors de la renumérotation).
- **Mode Pontage & Calcul de potentiels :**
  - Créez plusieurs **potentiels** colorés (`+24V`, `GND`, `L1` …) et touchez/cliquez les étages directement dans la vue CAO.
  - **Calcul automatique :** Détermine le nombre de pôles de peigne requis (ex. `2002-405`, `FBS 5-5`, `ZQV`), signale les **broches à couper (`Couper broche ×`)**, planifie les **ponts filaires (`DB`)** entre étages (`E1 <-> E2`) ou changements de pas, et gère **1 chambre** (pont filaire si > pôles max) vs **2 chambres** (cascade superposée sur chambres 1 & 2).
- **Éditeur CAO 2D interactif (Bureau & Tactile) :**
  - Vue CAO sans rail DIN superflu, logos fabricants intégrés (**WAGO**, **Phoenix Contact**, **Weidmüller**), sélecteur d'outils (**Déplacer**, **Sélection**, **Mode Pontage**), déplacement tactile & bouton milieu/droit.
  - **Affichage & édition directe :** Vertical `-90°` ou horizontal groupé ; **double-clic** pour éditer en direct (`Entrée` = valider, vide = auto, `Échap` = annuler).
- **Aperçu produit, lien fabricant & accessoires compatibles :**
  - Affiche **photo produit (si accessible) ou profil technique**, **nombre de chambres**, **lien direct vers la page produit fabricant**, plus **flasques, peignes, repères, butées et barres N**.
- **Export prêt pour l'impression (`.xlsx` & `.csv`) & Fichiers projet (`.termi` / `.json`) :**
  - **Excel (`.xlsx`) :** 3 feuilles (`Druckfolge` / `ByTierStrip` par défaut, `Ebenen-Spalten`, `Ebenen-Matrix`), toutes les cellules formatées en **Texte Excel (`@`)**.
- **7 langues, thèmes & vérificateur de mises à jour en direct :**
  - **DE, EN, FR, IT, ES, PL, NL** – Clair/Sombre contraste élevé, vérification GitHub sur Bureau & Android, liens directs **GitHub** & **Buy Me a Coffee**.

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
