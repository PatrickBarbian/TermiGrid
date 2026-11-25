<div align="center">

<img src="../assets/icon.png" width="128" alt="TermiGrid Logo" />

# TermiGrid

**Configurateur de borniers sur rail DIN, planificateur CAO, passerelle EPLAN P8 & exportateur de repérage pour armoires électriques**

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

**TermiGrid** est un outil d'ingénierie rapide et multiplateforme (écrit en Rust Édition 2024 avec `egui` / `eframe 0.37`) pour planifier, visualiser et configurer des borniers sur rail DIN dans la construction d'armoires électriques. Il fait le lien entre la CAO électrique (EPLAN Electric P8), la planification 2D interactive d'atelier, le calcul physique précis des accessoires (flasques d'extrémité, peignes de pontage avec broches coupées, butées d'arrêt) et les systèmes industriels d'impression de repères (WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO).

> **✨ VibeCoded :** Ce projet est **VibeCoded** – conçu, développé et perfectionné avec l'**assistance de l'IA** en étroite collaboration avec l'auteur du projet.

---

## Fonctionnalités principales

- **782+ références de bornes vérifiées + 26 butées d'arrêt pour rail DIN + bornes personnalisées :**
  - **WAGO :** TOPJOB S (`2000–2016` standard, `2200–2216` à poussoir, `2102–2116` à levier), bornes à trois étages `2002-32xx` avec PE au niveau inférieur, bornes d'installation (`2003`, `2005`, `2203`, `2205`), `2022` X-COM, `2007` mesure/sectionnement, `280/281` classique et `285` fort courant (`35–95 mm²`).
  - **Phoenix Contact :** COMPLETE line, Push-in `PT 1,5/S` à `PT 35`, bornes capteur/actionneur `PTIO`, `PTPOWER 35–95`, double étage `PTTB`/`PTTBS`, connexion latérale `PTV`, bornes d'installation `PTI` / `UTN` / `UTI`, sectionnables (`QTC 1,5-TG`, `URTK/SP`, `URTK 6`), ressort `ST`, vis `UT`, `UK 5 N`.
  - **Weidmüller :** Klippon Connect Série A (`A2C/A3C/A4C/A2T/A3T/ADT/AIO`), bornes d'installation `AITB`, Série P de répartition et potentiel (`PPV 4/8/16` & `PRV 4/8/16` avec étiquette unique par borne), Série W (`WDU/WPE/WDK/WTR/WSI/DLI`), Série Z (`ZDU/ZPE/ZDK/ZDL`).
  - **26 butées d'arrêt pour rail TS 35 :** Butées de grands constructeurs (WAGO 249-116/117, Phoenix Contact CLIPFIX 35 / E/UK, Weidmüller WEW 35/2, etc.).
  - **Bornes personnalisées & Bibliothèque de modèles :** Éditeur visuel de bornes (`custom_specs.json`) et bibliothèque de borniers modèles (`strip_library.json`) pour sauvegarder vos configurations récurrentes.

- **Mise à jour directe de la base via GitHub (Hot-Reload sans redémarrer) :**
  - Détection automatique et non bloquante au démarrage des mises à jour de la base `terminal_specs.json` sur GitHub.
  - Bouton bien visible « Mise à jour de la base disponible » : téléchargement en 1 clic, validation, stockage local et rechargement à chaud immédiat sans réinstaller ni recompiler l'application.

- **Assistant de saisie rapide local (Quick-Prompt, 100 % hors-ligne sans LLM cloud) :**
  - Saisie intuitive en texte clair : ex. `4x Drehstrom Abgang 2,5mm² Installationsklemmen`, `4x 3-pol PTI 2,5`, `6x +24V QUATTRO gebrückt`, `2x Reserve leer`.
  - Prévisualisation instantanée et insertion par lot dans le bornier actif en un clic.

- **Passerelle EPLAN Electric P8 & Aller-retour complet :**
  - **Import CSV en ligne de commande :** Lancement avec `TermiGrid.exe --import-eplan-csv "export.csv"` ou via `Menu → Importer CSV EPLAN...` avec nettoyage automatique des préfixes fabricants configurables (`PXC.`, `WAG.`, `WDM.`).
  - **Import intelligent de nomenclatures PDF EPLAN :** Glisser-déposer de listes PDF, détection automatique des bornes d'installation, barres N et alimentations avec calcul des supports de barre collectrice (tous les 200 mm).
  - **Paquet retour EPLAN en 1 clic :** Génération de `_EPLAN_Zubehoer_Import.csv` & `.xml` (avec réapplication automatique des préfixes fabricants, sans mélange de marques) et plans DXF individuels par bornier (`/DXF/<BMK>.dxf`) prêts pour l'import de pages EPLAN.
  - **Script passerelle C# pour EPLAN :** Export de `TermiGrid_EPLAN_Bridge.cs` en un clic pour intégration directe dans les barres d'outils d'EPLAN P8.
  - **Boîte de dialogue discrète :** Accessible sous `Menu → Interface EPLAN...` pour configurer préfixes, délimiteurs et dossiers d'export.

- **Éditeur CAO, Mode d'édition automatique & Génération de flasques :**
  - Vue CAO 2D à l'échelle avec affichage des flasques d'extrémité (D) et de séparation selon leur largeur réelle.
  - **Mode d'édition automatique :** S'active automatiquement dès qu'une borne est modifiée dans la CAO.
  - **Calcul automatique des flasques :** Générées et actualisées lors de l'export, de la vue e-Planer ou de l'aller-retour EPLAN.
  - Édition des repères par double-clic (gestion des repères vides pour préserver les espacements d'impression), défilement tactile & clic milieu, zoom fluide Ctrl+molette.

- **Mode Pontage, Cascade double chambre & Instructions de broches coupées :**
  - Création de potentiels colorés (`+24V`, `GND`, `L1` …) et affectation par glisser-déposer sur les étages.
  - Calcul réel des peignes de pontage avec cascade sur chambres 1 et 2.
  - Instructions physiques de coupe de broches (`PIN ENTFERNEN BEI`) pour les bornes enjambées.
  - Ponts filaires automatiques (`DB`) lors des changements d'étage ou de pas.

- **Guide intégré avec recherche avancée & Générateur de prompt IA (Onglet `Guide`) :**
  - 13 sections détaillées avec recherche plein texte multi-mots, filtres par catégorie et raccourcis.
  - Procédure pas à pas pour ajouter des bornes localement (éditeur visuel ou JSON).
  - Modèle de prompt prêt à copier pour les IA (ChatGPT, Claude...) afin de générer des fichiers JSON de bornes sans erreur.

- **Export prêt pour l'impression (`.xlsx` & `.csv`) :**
  - 4 feuilles Excel spécialisées :
    1. `Druckdaten (Flach)` (cellules au format texte strict `@` pour WAGO Smart Designer, Phoenix Contact Project Complete, Weidmüller M-Print PRO)
    2. `Etagen-Druckstreifen` (bandes continues par niveau E1..E4)
    3. `Bestückungsplan (Matrix)`
    4. `Stückliste & Montage` (liste de pièces agrégée, flasques, peignes avec broches coupées, butées et longueur totale de rail DIN)

- **Cache disque hors-ligne des photos & CDN vérifiés :**
  - Photos fabricants officielles via CDN sans blocage (WAGO, Phoenix Contact CaaS, Weidmüller DAM).
  - Cache local persistant (`photo_cache`) – fonctionne entièrement hors-ligne après le premier chargement.

- **Multiplateforme, 7 langues & Thèmes :**
  - Windows (`.exe` portable), macOS (`.dmg` Universel), Android (`.apk` Universel), Linux.
  - 7 langues : Allemand, Anglais, Français, Italien, Espagnol, Polonais, Néerlandais.
  - Thème Système / Clair / Sombre, Annuler/Rétablir (`Ctrl+Z` / `Ctrl+Y`), raccourcis (`Alt+Gauche/Droite`, `Ctrl+D`).

---

## Téléchargements & Portabilité

Tous les paquets prêts à l'emploi sont disponibles sur la **[page Releases](https://github.com/PatrickBarbian/TermiGrid/releases)** :

| Plateforme | Architecture / Cible | Fichier (Release) | Remarques |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64` | `TermiGrid-v1.9.1-windows-x86_64.exe`<br/>*(ou `TermiGrid-windows-x86_64.exe`)* | **100 % portable** (exécutable unique compilé avec `+crt-static` – sans installation ni VCRUNTIME) |
| **macOS** | Universel (`Intel x86_64` + `Apple Silicon arm64`) | `TermiGrid-v1.9.1-macos-universal.dmg`<br/>*(ou `TermiGrid-macos-universal.dmg`)* | Glisser le bundle `.app` dans Applications |
| **Android** | Universel (`arm64-v8a`, `armeabi-v7a`, `x86_64`) | `TermiGrid-v1.9.1-android-universal.apk`<br/>*(ou `TermiGrid-android-universal.apk`)* | Smartphones, tablettes & pliables (Android 7.0 – 15 / API 24–35, alignement mémoire 16 Ko, keystore stable, signature v1/v2/v3) |


### Notes par plateforme

- **Windows :** Lancer directement par double-clic depuis une clé USB ou un dossier local – aucun droit administrateur ni bibliothèque externe requis.
- **macOS (application non signée) :** Au premier lancement, utiliser **Clic droit -> Ouvrir** ou autoriser l'ouverture dans *Réglages Système -> Confidentialité et sécurité*.
- **Android :** Télécharger l'APK et autoriser l'installation depuis des sources inconnues pour votre navigateur/gestionnaire de fichiers. Grâce à l'alignement 16 Ko et à `resizeableActivity`, l'app fonctionne aussi sur les appareils récents (ex. Pixel 10 Pro XL) et les téléphones pliables. Les exports sont enregistrés directement dans `/sdcard/Download/TermiGrid/`.

---

## Guide rapide

1. **Ajouter des bornes :** Choisir le fabricant et le modèle dans le panneau de droite (ou utiliser la recherche, la bibliothèque de modèles ou l'assistant de saisie rapide, ex. `4x Drehstrom Abgang 2,5mm²`).
2. **Import & Aller-retour EPLAN :** Importer les bornes par ligne de commande (`--import-eplan-csv`), import manuel CSV ou glisser-déposer de listes PDF.
3. **Édition CAO & Mode Pontage :** Double-cliquer sur un repère pour éditer le texte, passer en Mode Pontage pour tracer des potentiels au clic-glisser.
4. **Accessoires automatiques & Flasques :** Flasques d'extrémité, peignes avec broches coupées et butées sont calculés automatiquement.
5. **Export & Impression :** Exporter en classeur Excel à 4 feuilles (`.xlsx`), CSV ou générer le paquet retour EPLAN en 1 clic (`_EPLAN_Zubehoer_Import.csv` & `.xml` + DXF).

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
