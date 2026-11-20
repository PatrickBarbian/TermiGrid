use crate::i18n::Language;
use crate::model::TierColor;

/// Picks the localized string for `lang` from `[DE, EN, FR, IT, ES, PL, NL]`.
#[inline]
pub const fn pick(lang: Language, texts: [&'static str; 7]) -> &'static str {
    match lang {
        Language::De => texts[0],
        Language::En => texts[1],
        Language::Fr => texts[2],
        Language::It => texts[3],
        Language::Es => texts[4],
        Language::Pl => texts[5],
        Language::Nl => texts[6],
    }
}

// --- Generic & Shared UI Labels ---

pub const fn edit(l: Language) -> &'static str {
    pick(l, ["Bearbeiten", "Edit", "Modifier", "Modifica", "Editar", "Edytuj", "Bewerken"])
}
pub const fn save(l: Language) -> &'static str {
    pick(l, ["Speichern", "Save", "Enregistrer", "Salva", "Guardar", "Zapisz", "Opslaan"])
}
pub const fn cancel(l: Language) -> &'static str {
    pick(l, ["Abbrechen", "Cancel", "Annuler", "Annulla", "Cancelar", "Anuluj", "Annuleren"])
}
pub const fn close(l: Language) -> &'static str {
    pick(l, ["Schließen", "Close", "Fermer", "Chiudi", "Cerrar", "Zamknij", "Sluiten"])
}
pub const fn delete(l: Language) -> &'static str {
    pick(l, ["Löschen", "Delete", "Supprimer", "Elimina", "Eliminar", "Usuń", "Verwijderen"])
}
pub const fn clear_search(l: Language) -> &'static str {
    pick(l, ["Suche leeren", "Clear search", "Effacer la recherche", "Cancella ricerca", "Limpiar búsqueda", "Wyczyść wyszukiwanie", "Zoekopdracht wissen"])
}
pub const fn reset_factory(l: Language) -> &'static str {
    pick(l, ["Werkszustand", "Reset default", "Réinitialiser", "Ripristina", "Restablecer", "Przywróć domyślne", "Standaard herstellen"])
}
pub const fn badge_modified(l: Language) -> &'static str {
    pick(l, ["Angepasst", "Modified", "Modifié", "Modificato", "Modificado", "Zmodyfikowano", "Aangepast"])
}
pub const fn badge_custom(l: Language) -> &'static str {
    pick(l, ["Eigene", "Custom", "Perso.", "Pers.", "Propia", "Własny", "Eigen"])
}
pub const fn actions_col(l: Language) -> &'static str {
    pick(l, ["Aktionen", "Actions", "Actions", "Azioni", "Acciones", "Akcje", "Acties"])
}
pub const fn undo_short(l: Language) -> &'static str {
    pick(l, ["Rückgängig", "Undo", "Annuler", "Annulla", "Deshacer", "Cofnij", "Ongedaan maken"])
}
pub const fn undo_tip(l: Language) -> &'static str {
    pick(l, ["Rückgängig / Undo (Strg+Z)", "Undo (Ctrl+Z)", "Annuler (Ctrl+Z)", "Annulla (Ctrl+Z)", "Deshacer (Ctrl+Z)", "Cofnij (Ctrl+Z)", "Ongedaan maken (Ctrl+Z)"])
}
pub const fn name_colon(l: Language) -> &'static str {
    pick(l, ["Name:", "Name:", "Nom :", "Nome:", "Nombre:", "Nazwa:", "Naam:"])
}
pub const fn info_colon(l: Language) -> &'static str {
    pick(l, ["Info:", "Info:", "Info :", "Info:", "Info:", "Info:", "Info:"])
}
pub const fn desc_optional_hint(l: Language) -> &'static str {
    pick(l, ["Beschreibung (optional)", "Description (optional)", "Description (optionnelle)", "Descrizione (opzionale)", "Descripción (opcional)", "Opis (opcjonalny)", "Beschrijving (optioneel)"])
}
pub const fn strip_word(l: Language) -> &'static str {
    pick(l, ["Leiste", "Strip", "Bornier", "Morsettiera", "Bornero", "Listwa", "Klemmenstrook"])
}
pub const fn zoom_in_tip(l: Language) -> &'static str {
    pick(l, ["Vergrößern", "Zoom in", "Zoom avant", "Ingrandisci", "Acercar", "Powiększ", "Inzoomen"])
}
pub const fn open_in_browser_tip(l: Language) -> &'static str {
    pick(l, ["Im Browser öffnen", "Open in browser", "Ouvrir dans le navigateur", "Apri nel browser", "Abrir en navegador", "Otwórz w przeglądarce", "Openen in browser"])
}
pub const fn select_none(l: Language) -> &'static str {
    pick(l, ["Keine auswählen", "Select none", "Tout désélectionner", "Deseleziona tutto", "Deseleccionar todo", "Odznacz wszystko", "Niets selecteren"])
}
pub const fn duplicate(l: Language) -> &'static str {
    pick(l, ["Duplizieren", "Duplicate", "Dupliquer", "Duplica", "Duplicar", "Duplikuj", "Dupliceren"])
}

// --- Header / Update / About / Close Confirm ---

pub const fn commands_btn(l: Language) -> &'static str {
    pick(l, ["Befehle  Strg+K", "Commands  Ctrl+K", "Commandes  Ctrl+K", "Comandi  Ctrl+K", "Comandos  Ctrl+K", "Polecenia  Ctrl+K", "Opdrachten  Ctrl+K"])
}
pub const fn commands_btn_mobile(l: Language) -> &'static str {
    pick(l, ["Befehle", "Commands", "Commandes", "Comandi", "Comandos", "Polecenia", "Opdrachten"])
}
pub const fn commands_tip(l: Language) -> &'static str {
    pick(l, [
        "Befehlspalette & Klemmen-Schnellsuche öffnen (Strg+K)",
        "Open command palette & quick terminal search (Ctrl+K)",
        "Ouvrir la palette de commandes et recherche rapide (Ctrl+K)",
        "Apri tavolozza comandi e ricerca rapida (Ctrl+K)",
        "Abrir paleta de comandos y búsqueda rápida (Ctrl+K)",
        "Otwórz paletę poleceń i szybkie wyszukiwanie (Ctrl+K)",
        "Opdrachtenpalet en snelzoeken openen (Ctrl+K)",
    ])
}
pub const fn cmd_palette_tip_only(l: Language) -> &'static str {
    pick(l, [
        "Befehlspalette öffnen (Strg+K)",
        "Open command palette (Ctrl+K)",
        "Ouvrir la palette de commandes (Ctrl+K)",
        "Apri tavolozza comandi (Ctrl+K)",
        "Abrir paleta de comandos (Ctrl+K)",
        "Otwórz paletę poleceń (Ctrl+K)",
        "Opdrachtenpalet openen (Ctrl+K)",
    ])
}
pub const fn about_tip(l: Language) -> &'static str {
    pick(l, [
        "Info, Updates & Open-Source-Lizenzen",
        "Info, updates & open-source licenses",
        "Infos, mises à jour et licences open source",
        "Info, aggiornamenti e licenze open source",
        "Info, actualizaciones y licencias open source",
        "Info, aktualizacje i licencje open source",
        "Info, updates en open-sourcelicenties",
    ])
}
pub const fn about_modal_title(l: Language) -> &'static str {
    pick(l, [
        "Über TermiGrid & Lizenzen",
        "About TermiGrid & Licenses",
        "À propos de TermiGrid et licences",
        "Informazioni su TermiGrid e licenze",
        "Acerca de TermiGrid y licencias",
        "O TermiGrid i licencjach",
        "Over TermiGrid en licenties",
    ])
}
pub const fn about_guide_title(l: Language) -> &'static str {
    pick(l, [
        "Über TermiGrid – Info & Lizenzen",
        "About TermiGrid – Info & Licenses",
        "À propos de TermiGrid – Infos & Licences",
        "Informazioni su TermiGrid – Info & Licenze",
        "Acerca de TermiGrid – Info y Licencias",
        "O TermiGrid – Informacje i licencje",
        "Over TermiGrid – Info & Licenties",
    ])
}
pub const fn about_author_line(l: Language) -> &'static str {
    pick(l, [
        "• Patrick Barbian — Projektinhaber, Elektrotechnik-Konzept & Entwicklung",
        "• Patrick Barbian — Project owner, electrical engineering concept & development",
        "• Patrick Barbian — Propriétaire du projet, concept électrotechnique & développement",
        "• Patrick Barbian — Titolare del progetto, concetto elettrotecnico e sviluppo",
        "• Patrick Barbian — Propietario del proyecto, concepto electrotécnico y desarrollo",
        "• Patrick Barbian — Właściciel projektu, koncepcja elektrotechniczna i rozwój",
        "• Patrick Barbian — Projecteigenaar, elektrotechnisch concept & ontwikkeling",
    ])
}
pub const fn about_brand_notice(l: Language) -> &'static str {
    pick(l, [
        "Marken-Hinweis: WAGO (TOPJOB S), Phoenix Contact (CLIPLINE complete / PT / PTI) und Weidmüller (Klippon Connect) sind eingetragene Marken der jeweiligen Hersteller.",
        "Trademark notice: WAGO (TOPJOB S), Phoenix Contact (CLIPLINE complete / PT / PTI), and Weidmüller (Klippon Connect) are registered trademarks of their respective owners.",
        "Mentions de marques : WAGO (TOPJOB S), Phoenix Contact (CLIPLINE complete / PT / PTI) et Weidmüller (Klippon Connect) sont des marques déposées de leurs fabricants respectifs.",
        "Avviso marchi: WAGO (TOPJOB S), Phoenix Contact (CLIPLINE complete / PT / PTI) e Weidmüller (Klippon Connect) sono marchi registrati dei rispettivi produttori.",
        "Aviso de marcas: WAGO (TOPJOB S), Phoenix Contact (CLIPLINE complete / PT / PTI) y Weidmüller (Klippon Connect) son marcas registradas de sus respectivos fabricantes.",
        "Uwaga o znakach: WAGO (TOPJOB S), Phoenix Contact (CLIPLINE complete / PT / PTI) i Weidmüller (Klippon Connect) są zastrzeżonymi znakami towarowymi odpowiednich producentów.",
        "Merkvermelding: WAGO (TOPJOB S), Phoenix Contact (CLIPLINE complete / PT / PTI) en Weidmüller (Klippon Connect) zijn geregistreerde handelsmerken van hun respectieve fabrikanten.",
    ])
}
pub fn update_badge(l: Language, tag: &str) -> String {
    match l {
        Language::De => format!("Update: {tag}"),
        Language::En => format!("Update: {tag}"),
        Language::Fr => format!("MàJ : {tag}"),
        Language::It => format!("Aggiorn.: {tag}"),
        Language::Es => format!("Actualiz.: {tag}"),
        Language::Pl => format!("Aktual.: {tag}"),
        Language::Nl => format!("Update: {tag}"),
    }
}
pub const fn close_confirm_title(l: Language) -> &'static str {
    pick(l, [
        "Änderungen vor dem Schließen speichern?",
        "Save changes before closing?",
        "Enregistrer les modifications avant de fermer ?",
        "Salvare le modifiche prima di chiudere?",
        "¿Guardar los cambios antes de cerrar?",
        "Zapisać zmiany przed zamknięciem?",
        "Wijzigingen opslaan vóór afsluiten?",
    ])
}
pub fn close_confirm_project(l: Language, name: &str) -> String {
    match l {
        Language::De => format!("Das Projekt „{name}“ enthält ungespeicherte Änderungen."),
        Language::En => format!("The project \"{name}\" has unsaved changes."),
        Language::Fr => format!("Le projet « {name} » contient des modifications non enregistrées."),
        Language::It => format!("Il progetto «{name}» contiene modifiche non salvate."),
        Language::Es => format!("El proyecto «{name}» tiene cambios sin guardar."),
        Language::Pl => format!("Projekt „{name}” zawiera niezapisane zmiany."),
        Language::Nl => format!("Het project \"{name}\" bevat niet-opgeslagen wijzigingen."),
    }
}
pub fn close_confirm_counts(l: Language, strips: usize, blocks: usize) -> String {
    match l {
        Language::De => format!("{strips} Klemmleiste(n) · {blocks} Klemme(n) — Möchten Sie diese Änderungen jetzt speichern oder verwerfen?"),
        Language::En => format!("{strips} terminal strip(s) · {blocks} block(s) — Save or discard these changes?"),
        Language::Fr => format!("{strips} bornier(s) · {blocks} borne(s) — Enregistrer ou ignorer ces modifications ?"),
        Language::It => format!("{strips} morsettiera/e · {blocks} morsetti — Salvare o scartare le modifiche?"),
        Language::Es => format!("{strips} bornero(s) · {blocks} borna(s) — ¿Guardar o descartar estos cambios?"),
        Language::Pl => format!("{strips} listew · {blocks} zacisków — Zapisać czy odrzucić zmiany?"),
        Language::Nl => format!("{strips} klemmenstrook/-en · {blocks} klemmen — Wijzigingen opslaan of negeren?"),
    }
}
pub const fn save_and_close(l: Language) -> &'static str {
    pick(l, ["Speichern & Schließen", "Save & Close", "Enregistrer & Fermer", "Salva e chiudi", "Guardar y cerrar", "Zapisz i zamknij", "Opslaan & sluiten"])
}
pub const fn discard_changes(l: Language) -> &'static str {
    pick(l, ["Verwerfen", "Discard", "Ignorer", "Scarta", "Descartar", "Odrzuć", "Negeren"])
}

// --- Guided Pipeline & Welcome Screen ---

pub const fn pdf_bom_short(l: Language) -> &'static str {
    pick(l, ["PDF-Stückliste", "PDF BOM", "Nomenclature PDF", "Distinta PDF", "Lista PDF", "Lista PDF", "PDF-stuklijst"])
}
pub const fn pdf_bom_tip(l: Language) -> &'static str {
    pick(l, [
        "EPLAN-Stückliste (.pdf) intelligent importieren (auch per Drag & Drop)",
        "Import EPLAN BOM (.pdf) automatically (also via drag & drop)",
        "Importer automatiquement une nomenclature EPLAN (.pdf) (glisser-déposer pris en charge)",
        "Importa distinta EPLAN (.pdf) automaticamente (anche via drag & drop)",
        "Importar lista de materiales EPLAN (.pdf) automáticamente (también arrastrando)",
        "Inteligentny import zestawienia EPLAN (.pdf) (także metodą przeciągnij i upuść)",
        "EPLAN-stuklijst (.pdf) slim importeren (ook via slepen & neerzetten)",
    ])
}
pub const fn pipeline_subtitle(l: Language) -> &'static str {
    pick(l, [
        "Geradliniger Ablauf: 1. Startquelle wählen → 2. Klemmen im CAD anpassen → 3. Endplatten generieren → 4. Brücken, Excel-Export oder ePlan 1:1.",
        "Guided workflow: 1. Choose start source → 2. Adjust terminals in CAD → 3. Generate end plates → 4. Bridges, Excel export or ePlan 1:1.",
        "Flux guidé : 1. Choisir la source → 2. Ajuster les bornes en CAO → 3. Générer les flasques → 4. Ponts, Excel ou ePlan 1:1.",
        "Flusso guidato: 1. Scegli avvio → 2. Adatta morsetti in CAD → 3. Genera piastre terminali → 4. Ponticelli, Excel o ePlan 1:1.",
        "Flujo guiado: 1. Elegir inicio → 2. Ajustar bornas en CAD → 3. Generar tapas finales → 4. Puentes, Excel o ePlan 1:1.",
        "Przejrzysty proces: 1. Wybierz start → 2. Dopasuj zaciski w CAD → 3. Generuj płytki końcowe → 4. Mostki, Excel lub ePlan 1:1.",
        "Gestructureerde workflow: 1. Kies start → 2. Pas klemmen aan in CAD → 3. Eindplaten genereren → 4. Bruggen, Excel of ePlan 1:1.",
    ])
}
pub const fn step_endplates_short(l: Language) -> &'static str {
    pick(l, ["3. Endplatten", "3. End Plates", "3. Flasques", "3. Piastre", "3. Tapas", "3. Płytki", "3. Eindplaten"])
}
pub const fn step_export_short(l: Language) -> &'static str {
    pick(l, ["4. Export / ePlan", "4. Export / ePlan", "4. Export / ePlan", "4. Export / ePlan", "4. Export / ePlan", "4. Eksport / ePlan", "4. Export / ePlan"])
}
pub const fn choose_project_start(l: Language) -> &'static str {
    pick(l, ["Projektstart wählen", "Choose Project Start", "Choisir le démarrage", "Scegli avvio progetto", "Elegir inicio del proyecto", "Wybierz start projektu", "Kies projectstart"])
}
pub const fn welcome_new_project(l: Language) -> &'static str {
    pick(l, ["Neues Projekt", "New Project", "Nouveau projet", "Nuovo progetto", "Nuevo proyecto", "Nowy projekt", "Nieuw project"])
}
pub const fn welcome_load_project(l: Language) -> &'static str {
    pick(l, ["Bestehendes Projekt laden", "Load Existing Project", "Charger un projet existant", "Carica progetto esistente", "Cargar proyecto existente", "Wczytaj istniejący projekt", "Bestaand project laden"])
}
pub const fn welcome_load_pdf(l: Language) -> &'static str {
    pick(l, ["PDF-Stückliste laden", "Load PDF BOM", "Charger nomenclature PDF", "Carica distinta PDF", "Cargar lista PDF", "Wczytaj listę PDF", "PDF-stuklijst laden"])
}
pub const fn welcome_from_lib(l: Language) -> &'static str {
    pick(l, ["Aus Bibliothek", "From Library", "Depuis la bibliothèque", "Dalla libreria", "Desde biblioteca", "Z biblioteki", "Uit bibliotheek"])
}
pub fn welcome_from_lib_count(l: Language, count: usize) -> String {
    match l {
        Language::De => format!("Aus Bibliothek ({count})"),
        Language::En => format!("From Library ({count})"),
        Language::Fr => format!("Bibliothèque ({count})"),
        Language::It => format!("Libreria ({count})"),
        Language::Es => format!("Biblioteca ({count})"),
        Language::Pl => format!("Z biblioteki ({count})"),
        Language::Nl => format!("Uit bibliotheek ({count})"),
    }
}
pub const fn welcome_from_lib_tip(l: Language) -> &'static str {
    pick(l, [
        "Persönliche Klemmleisten-Bibliothek durchsuchen & direkt starten",
        "Browse personal terminal strip library & start directly",
        "Parcourir la bibliothèque personnelle de borniers et démarrer",
        "Sfoglia la libreria personale di morsettiere e avvia",
        "Explorar la biblioteca personal de borneros y comenzar",
        "Przeglądaj osobistą bibliotekę listew i rozpocznij",
        "Persoonlijke klemmenstrook-bibliotheek openen en direct starten",
    ])
}

pub const fn step1_edit_terminals(l: Language) -> &'static str {
    pick(l, ["1. Klemmen anpassen", "1. Edit Terminals", "1. Ajuster bornes", "1. Adatta morsetti", "1. Ajustar bornas", "1. Dopasuj zaciski", "1. Klemmen aanpassen"])
}
pub const fn step2_gen_endplates(l: Language) -> &'static str {
    pick(l, ["2. Endplatten generieren", "2. Generate End Plates", "2. Générer flasques", "2. Genera piastre", "2. Generar tapas", "2. Generuj płytki", "2. Eindplaten genereren"])
}
pub const fn step3_bridge_excel_eplan(l: Language) -> &'static str {
    pick(l, ["3. Brücken / Excel / ePlan", "3. Bridges / Excel / ePlan", "3. Ponts / Excel / ePlan", "3. Ponticelli / Excel / ePlan", "3. Puentes / Excel / ePlan", "3. Mostki / Excel / ePlan", "3. Bruggen / Excel / ePlan"])
}
pub const fn gen_endplates_btn(l: Language) -> &'static str {
    pick(l, ["Endplatten generieren", "Generate End Plates", "Générer flasques", "Genera piastre", "Generar tapas finales", "Generuj płytki końcowe", "Eindplaten genereren"])
}
pub const fn gen_endplates_tip(l: Language) -> &'static str {
    pick(l, [
        "Abschluss-/Zwischenplatten berechnen und auf NT-Trennklemmen prüfen",
        "Calculate end/partition plates and check NT disconnect terminals",
        "Calculer les flasques/séparateurs et vérifier les bornes NT",
        "Calcola piastre terminali/separatrici e verifica morsetti NT",
        "Calcular tapas finales/separadoras y comprobar bornas NT",
        "Oblicz płytki końcowe/rozdzielające i sprawdź zaciski NT",
        "Bereken eind-/tussenplaten en controleer NT-scheidingsklemmen",
    ])
}
pub const fn gen_endplates_empty_tip(l: Language) -> &'static str {
    pick(l, [
        "Zuerst Klemmen auf der Klemmleiste platzieren",
        "Place terminals on the strip first",
        "Placez d'abord des bornes sur le bornier",
        "Inserire prima i morsetti sulla morsettiera",
        "Colocar primero bornas en el bornero",
        "Najpierw umieść zaciski na listwie",
        "Plaats eerst klemmen op de klemmenstrook",
    ])
}
pub const fn step1_done_pill(l: Language) -> &'static str {
    pick(l, ["✓ 1. Klemmen", "✓ 1. Terminals", "✓ 1. Bornes", "✓ 1. Morsetti", "✓ 1. Bornas", "✓ 1. Zaciski", "✓ 1. Klemmen"])
}
pub fn step2_done_pill(l: Language, count: usize) -> String {
    match l {
        Language::De => format!("✓ 2. Endplatten ({count}×)"),
        Language::En => format!("✓ 2. End Plates ({count}×)"),
        Language::Fr => format!("✓ 2. Flasques ({count}×)"),
        Language::It => format!("✓ 2. Piastre ({count}×)"),
        Language::Es => format!("✓ 2. Tapas ({count}×)"),
        Language::Pl => format!("✓ 2. Płytki ({count}×)"),
        Language::Nl => format!("✓ 2. Eindplaten ({count}×)"),
    }
}
pub const fn step3_next_pill(l: Language) -> &'static str {
    pick(l, ["3. Nächster Schritt", "3. Next Step", "3. Étape suivante", "3. Passo successivo", "3. Siguiente paso", "3. Następny krok", "3. Volgende stap"])
}
pub const fn reedit_btn(l: Language) -> &'static str {
    pick(l, ["Erneut bearbeiten", "Edit Again", "Modifier", "Modifica di nuovo", "Volver a editar", "Edytuj ponownie", "Opnieuw bewerken"])
}
pub const fn reedit_tip(l: Language) -> &'static str {
    pick(l, [
        "Zurück in den Klemmen-Bearbeitungsmodus wechseln, um Klemmen oder Nummerierung anzupassen",
        "Return to terminal editing mode to adjust terminals or numbering",
        "Revenir en mode d'édition des bornes pour modifier bornes ou numérotation",
        "Torna alla modalità di modifica morsetti per modificare morsetti o numerazione",
        "Volver al modo de edición para ajustar bornas o numeración",
        "Wróć do trybu edycji zacisków, aby zmienić zaciski lub numerację",
        "Terug naar klemmen bewerken om klemmen of nummering aan te passen",
    ])
}
pub const fn cad_view_btn(l: Language) -> &'static str {
    pick(l, ["CAD-Ansicht", "CAD View", "Vue CAO", "Vista CAD", "Vista CAD", "Widok CAD", "CAD-weergave"])
}
pub const fn cad_view_tip(l: Language) -> &'static str {
    pick(l, [
        "Standard-CAD-Ansicht mit Endplatten & Beschriftung",
        "Standard CAD view with end plates & labels",
        "Vue CAO standard avec flasques et repérage",
        "Vista CAD standard con piastre e siglatura",
        "Vista CAD estándar con tapas y marcadores",
        "Standardowy widok CAD z płytkami i oznaczeniami",
        "Standaard CAD-weergave met eindplaten & codering",
    ])
}
pub const fn bridge_mode_btn(l: Language) -> &'static str {
    pick(l, ["Brücken-Modus", "Bridge Mode", "Mode Ponts", "Modalità Ponticelli", "Modo Puentes", "Tryb mostków", "Bruggen-modus"])
}
pub const fn eplan_mode_btn(l: Language) -> &'static str {
    pick(l, ["ePlan-Modus", "ePlan Mode", "Mode ePlan", "Modalità ePlan", "Modo ePlan", "Tryb ePlan", "ePlan-modus"])
}
pub const fn excel_export_btn(l: Language) -> &'static str {
    pick(l, ["Excel-Export", "Excel Export", "Export Excel", "Esporta Excel", "Exportar Excel", "Eksport Excel", "Excel-export"])
}
pub const fn excel_export_tip(l: Language) -> &'static str {
    pick(l, [
        "Exportiert alle Klemmleisten als 4-Blatt-Excel-Arbeitsmappe (.xlsx)",
        "Exports all terminal strips as a 4-sheet Excel workbook (.xlsx)",
        "Exporte tous les borniers en classeur Excel à 4 feuilles (.xlsx)",
        "Esporta tutte le morsettiere in una cartella Excel a 4 fogli (.xlsx)",
        "Exporta todos los borneros en un libro Excel de 4 hojas (.xlsx)",
        "Eksportuje wszystkie listwy jako 4-arkuszowy skoroszyt Excel (.xlsx)",
        "Exporteert alle klemmenstroken als 4-blads Excel-werkmap (.xlsx)",
    ])
}
pub const fn csv_tip(l: Language) -> &'static str {
    pick(l, [
        "Flache Druckdaten als .csv exportieren",
        "Export flat print labels as .csv",
        "Exporter les étiquettes à plat au format .csv",
        "Esporta i dati di stampa in formato .csv",
        "Exportar datos planos de impresión como .csv",
        "Eksportuj płaskie dane nadruku jako .csv",
        "Platte afdrukgegevens exporteren als .csv",
    ])
}
pub const fn dxf_btn(l: Language) -> &'static str {
    pick(l, ["DXF (1:1 mm)", "DXF (1:1 mm)", "DXF (1:1 mm)", "DXF (1:1 mm)", "DXF (1:1 mm)", "DXF (1:1 mm)", "DXF (1:1 mm)"])
}
pub const fn dxf_tip(l: Language) -> &'static str {
    pick(l, [
        "Exportiert alle Klemmleisten maßstabsgetreu (1:1 mm) als DXF für EPLAN Electric P8",
        "Exports all terminal strips at 1:1 mm scale as DXF for EPLAN Electric P8",
        "Exporte tous les borniers à l'échelle 1:1 mm en DXF pour EPLAN Electric P8",
        "Esporta tutte le morsettiere in scala 1:1 mm come DXF per EPLAN Electric P8",
        "Exporta todos los borneros a escala 1:1 mm en DXF para EPLAN Electric P8",
        "Eksportuje wszystkie listwy w skali 1:1 mm jako DXF dla EPLAN Electric P8",
        "Exporteert alle klemmenstroken op schaal 1:1 mm als DXF voor EPLAN Electric P8",
    ])
}

// --- Numbering Sequence & Inspector ---

pub const fn sequence_label(l: Language) -> &'static str {
    pick(l, ["Folge", "Order", "Ordre", "Ordine", "Orden", "Kolejność", "Volgorde"])
}
pub const fn sequence_label_long(l: Language) -> &'static str {
    pick(l, ["Zählfolge", "Numbering Order", "Ordre de numérotation", "Ordine di numerazione", "Orden de numeración", "Kolejność numeracji", "Nummeringsvolgorde"])
}
pub const fn seq_by_block(l: Language) -> &'static str {
    pick(l, [
        "Klemmenweise (1,2 | 3,4)",
        "Per block (1,2 | 3,4)",
        "Par borne (1,2 | 3,4)",
        "Per morsetto (1,2 | 3,4)",
        "Por borna (1,2 | 3,4)",
        "Wg zacisku (1,2 | 3,4)",
        "Per klem (1,2 | 3,4)",
    ])
}
pub const fn seq_by_level(l: Language) -> &'static str {
    pick(l, [
        "Ebenenweise (1,2,3 | 4,5,6)",
        "By level (1,2,3 | 4,5,6)",
        "Par niveau (1,2,3 | 4,5,6)",
        "Per piano (1,2,3 | 4,5,6)",
        "Por piso (1,2,3 | 4,5,6)",
        "Wg poziomu (1,2,3 | 4,5,6)",
        "Per etage (1,2,3 | 4,5,6)",
    ])
}
pub const fn seq_by_level_long(l: Language) -> &'static str {
    pick(l, [
        "Ebenenweise (E1: 1,2,3 | E2: 4,5,6)",
        "By level (L1: 1,2,3 | L2: 4,5,6)",
        "Par niveau (N1 : 1,2,3 | N2 : 4,5,6)",
        "Per piano (P1: 1,2,3 | P2: 4,5,6)",
        "Por piso (N1: 1,2,3 | N2: 4,5,6)",
        "Wg poziomu (P1: 1,2,3 | P2: 4,5,6)",
        "Per etage (E1: 1,2,3 | E2: 4,5,6)",
    ])
}
pub fn selected_count(l: Language, n: usize) -> String {
    match l {
        Language::De => format!("{n} markiert"),
        Language::En => format!("{n} selected"),
        Language::Fr => format!("{n} sél."),
        Language::It => format!("{n} sel."),
        Language::Es => format!("{n} sel."),
        Language::Pl => format!("{n} zazn."),
        Language::Nl => format!("{n} gesel."),
    }
}
pub const fn eb_left_badge(l: Language) -> &'static str {
    pick(l, ["EH Links", "EB Left", "Butée G", "Fermo SX", "Tope Izq.", "Trzymacz L", "Eindst. L"])
}
pub const fn eb_right_badge(l: Language) -> &'static str {
    pick(l, ["EH Rechts", "EB Right", "Butée D", "Fermo DX", "Tope Der.", "Trzymacz P", "Eindst. R"])
}
pub const fn apply_type_to_selection(l: Language) -> &'static str {
    pick(l, [
        "Typ auf Auswahl anwenden",
        "Apply type to selection",
        "Appliquer à la sélection",
        "Applica alla selezione",
        "Aplicar a la selección",
        "Zastosuj do zaznaczenia",
        "Toepassen op selectie",
    ])
}
pub const fn pole1_btn(l: Language) -> &'static str {
    pick(l, ["1-Polig", "1-Pole", "1 Pôle", "1 Polo", "1 Polo", "1-Bieg.", "1-Polig"])
}
pub const fn pole3_btn(l: Language) -> &'static str {
    pick(l, ["3-Polig", "3-Pole", "3 Pôles", "3 Poli", "3 Polos", "3-Bieg.", "3-Polig"])
}
pub fn pole_group_tip(l: Language, poles: u8, summary: &str) -> String {
    match l {
        Language::De => format!("{poles}-Polig Blockgruppe einfügen: {summary}"),
        Language::En => format!("Insert {poles}-pole block group: {summary}"),
        Language::Fr => format!("Insérer groupe {poles} pôles : {summary}"),
        Language::It => format!("Inserisci gruppo {poles} poli: {summary}"),
        Language::Es => format!("Insertar grupo {poles} polos: {summary}"),
        Language::Pl => format!("Wstaw grupę {poles}-biegunową: {summary}"),
        Language::Nl => format!("{poles}-polige groep invoegen: {summary}"),
    }
}
pub fn pole_group_unavail(l: Language, poles: u8) -> String {
    let scheme = if poles == 1 { "L + N + PE" } else { "3× L + N + PE" };
    match l {
        Language::De => format!("Kein {poles}-Polig Block ({scheme}) für diesen Klemmentyp verfügbar"),
        Language::En => format!("No {poles}-pole group ({scheme}) available for this terminal type"),
        Language::Fr => format!("Aucun groupe {poles} pôles ({scheme}) disponible pour ce type"),
        Language::It => format!("Nessun gruppo {poles} poli ({scheme}) disponibile per questo tipo"),
        Language::Es => format!("No hay grupo {poles} polos ({scheme}) disponible para este tipo"),
        Language::Pl => format!("Brak grupy {poles}-biegunowej ({scheme}) dla tego typu zacisku"),
        Language::Nl => format!("Geen {poles}-polige groep ({scheme}) beschikbaar voor dit klemtype"),
    }
}
pub const fn move_left_tip(l: Language) -> &'static str {
    pick(l, ["Nach links verschieben (Alt+Links)", "Move left (Alt+Left)", "Déplacer à gauche (Alt+Gauche)", "Sposta a sinistra (Alt+Sinistra)", "Mover a la izquierda (Alt+Izq.)", "Przesuń w lewo (Alt+Lewo)", "Naar links verplaatsen (Alt+Links)"])
}
pub const fn move_right_tip(l: Language) -> &'static str {
    pick(l, ["Nach rechts verschieben (Alt+Rechts)", "Move right (Alt+Right)", "Déplacer à droite (Alt+Droite)", "Sposta a destra (Alt+Destra)", "Mover a la derecha (Alt+Der.)", "Przesuń w prawo (Alt+Prawo)", "Naar rechts verplaatsen (Alt+Rechts)"])
}
pub const fn clone_selection_tip(l: Language) -> &'static str {
    pick(l, ["Auswahl klonen (Strg+D)", "Clone selection (Ctrl+D)", "Dupliquer la sélection (Ctrl+D)", "Duplica selezione (Ctrl+D)", "Duplicar selección (Ctrl+D)", "Duplikuj zaznaczenie (Ctrl+D)", "Selectie klonen (Ctrl+D)"])
}
pub fn add_same_after_tip(l: Language, idx: usize, name: &str) -> String {
    match l {
        Language::De => format!("Gleiche Klemme wie #{} ({name}) direkt danach hinzufügen", idx + 1),
        Language::En => format!("Add same terminal as #{} ({name}) right after", idx + 1),
        Language::Fr => format!("Ajouter la même borne que #{} ({name}) juste après", idx + 1),
        Language::It => format!("Aggiungi lo stesso morsetto di #{} ({name}) subito dopo", idx + 1),
        Language::Es => format!("Añadir misma borna que #{} ({name}) justo después", idx + 1),
        Language::Pl => format!("Dodaj taki sam zacisk jak #{} ({name}) tuż za", idx + 1),
        Language::Nl => format!("Zelfde klem als #{} ({name}) direct erna toevoegen", idx + 1),
    }
}
pub fn add_default_end_tip(l: Language, name: &str) -> String {
    match l {
        Language::De => format!("Standardklemme ({name}) am Leistenende hinzufügen"),
        Language::En => format!("Add default terminal ({name}) at strip end"),
        Language::Fr => format!("Ajouter borne standard ({name}) en fin de bornier"),
        Language::It => format!("Aggiungi morsetto standard ({name}) a fine morsettiera"),
        Language::Es => format!("Añadir borna estándar ({name}) al final del bornero"),
        Language::Pl => format!("Dodaj standardowy zacisk ({name}) na końcu listwy"),
        Language::Nl => format!("Standaardklem ({name}) aan het einde toevoegen"),
    }
}
pub fn convert_n_to_nt_tip(l: Language, count: usize) -> String {
    match l {
        Language::De => format!("Wandelt alle {count} N-Klemmen dieser Leiste in passende NT-Trennklemmen um"),
        Language::En => format!("Converts all {count} N terminals on this strip into matching NT disconnect terminals"),
        Language::Fr => format!("Convertit les {count} bornes N de ce bornier en bornes sectionnables NT"),
        Language::It => format!("Converte tutti i {count} morsetti N in morsetti sezionatori NT"),
        Language::Es => format!("Convierte las {count} bornas N en bornas seccionadoras NT"),
        Language::Pl => format!("Zamienia wszystkie {count} zaciski N na rozłączne zaciski NT"),
        Language::Nl => format!("Zet alle {count} N-klemmen om in bijpassende NT-scheidingsklemmen"),
    }
}
pub const fn convert_n_to_nt_none(l: Language) -> &'static str {
    pick(l, [
        "Keine umwandelbaren N-Klemmen vorhanden",
        "No convertible N terminals found",
        "Aucune borne N convertible",
        "Nessun morsetto N convertibile",
        "No hay bornas N convertibles",
        "Brak zacisków N do zamiany",
        "Geen omzetbare N-klemmen aanwezig",
    ])
}
pub fn convert_nt_to_n_tip(l: Language, count: usize) -> String {
    match l {
        Language::De => format!("Wandelt alle {count} NT-Trennklemmen wieder in Standard-N-Klemmen um"),
        Language::En => format!("Converts all {count} NT disconnect terminals back into standard N terminals"),
        Language::Fr => format!("Reconvertit les {count} bornes NT en bornes N standard"),
        Language::It => format!("Riconverte tutti i {count} morsetti NT in morsetti N standard"),
        Language::Es => format!("Reconvierte las {count} bornas NT en bornas N estándar"),
        Language::Pl => format!("Przywraca wszystkie {count} zaciski NT do standardowych zacisków N"),
        Language::Nl => format!("Zet alle {count} NT-scheidingsklemmen terug naar standaard N-klemmen"),
    }
}
pub const fn convert_nt_to_n_none(l: Language) -> &'static str {
    pick(l, [
        "Keine umwandelbaren NT-Trennklemmen vorhanden",
        "No convertible NT disconnect terminals found",
        "Aucune borne sectionnable NT convertible",
        "Nessun morsetto sezionatore NT convertibile",
        "No hay bornas seccionadoras NT convertibles",
        "Brak zacisków NT do zamiany",
        "Geen omzetbare NT-scheidingsklemmen aanwezig",
    ])
}
pub const fn endplates_deferred_hint(l: Language) -> &'static str {
    pick(l, [
        "Endplatten & NT-Prüfung erscheinen nach Klick auf „Endplatten generieren“.",
        "End plates & NT check appear after clicking \"Generate End Plates\".",
        "Les flasques et la vérification NT apparaissent après « Générer flasques ».",
        "Piastre terminali e verifica NT appaiono dopo «Genera piastre».",
        "Las tapas y la comprobación NT aparecen al pulsar «Generar tapas finales».",
        "Płytki końcowe i kontrola NT pojawią się po kliknięciu „Generuj płytki końcowe”.",
        "Eindplaten & NT-controle verschijnen na klikken op \"Eindplaten genereren\".",
    ])
}

// --- Left Sidebar Stacked Sheet Deck ---

pub const fn strips_tab(l: Language) -> &'static str {
    pick(l, ["Klemmleisten", "Terminal Strips", "Borniers", "Morsettiere", "Borneros", "Listwy", "Klemmenstroken"])
}
pub const fn library_tab(l: Language) -> &'static str {
    pick(l, ["Bibliothek", "Library", "Bibliothèque", "Libreria", "Biblioteca", "Biblioteka", "Bibliotheek"])
}
pub fn library_tab_count(l: Language, n: usize) -> String {
    format!("{} ({n})", library_tab(l))
}
pub const fn sidebar_toggle_tip(l: Language) -> &'static str {
    pick(l, [
        "Linke Seitenleiste (Klemmleisten & Bibliothek) ein-/ausblenden",
        "Show/hide left sidebar (Terminal Strips & Library)",
        "Afficher/masquer le panneau gauche (Borniers & Bibliothèque)",
        "Mostra/nascondi barra laterale sinistra (Morsettiere e Libreria)",
        "Mostrar/ocultar barra lateral izquierda (Borneros y Biblioteca)",
        "Pokaż/ukryj lewy panel (Listwy i Biblioteka)",
        "Linkerzijbalk (Klemmenstroken & Bibliotheek) tonen/verbergen",
    ])
}
pub const fn to_library_tip(l: Language) -> &'static str {
    pick(l, [
        "Zur Bibliothek wechseln (Vorlagen mit Mini-ePlan-Vorschau)",
        "Switch to Library (templates with mini ePlan preview)",
        "Passer à la bibliothèque (modèles avec aperçu ePlan)",
        "Passa alla libreria (modelli con anteprima ePlan)",
        "Cambiar a la biblioteca (plantillas con vista previa ePlan)",
        "Przejdź do biblioteki (szablony z podglądem ePlan)",
        "Naar bibliotheek (sjablonen met mini ePlan-voorbeeld)",
    ])
}
pub const fn to_strips_tip(l: Language) -> &'static str {
    pick(l, [
        "Zu den Projekt-Klemmleisten wechseln",
        "Switch to project terminal strips",
        "Passer aux borniers du projet",
        "Passa alle morsettiere del progetto",
        "Cambiar a los borneros del proyecto",
        "Przejdź do listew projektu",
        "Naar project-klemmenstroken",
    ])
}
pub const fn dot_strips_tip(l: Language) -> &'static str {
    pick(l, ["Klemmleisten (Hauptansicht)", "Terminal Strips (Main view)", "Borniers (Vue principale)", "Morsettiere (Vista principale)", "Borneros (Vista principal)", "Listwy (Widok główny)", "Klemmenstroken (Hoofdaanzicht)"])
}
pub const fn dot_library_tip(l: Language) -> &'static str {
    pick(l, ["Bibliothek (Vorlagen mit Mini-ePlan)", "Library (Templates with mini ePlan)", "Bibliothèque (Modèles avec ePlan)", "Libreria (Modelli con ePlan)", "Biblioteca (Plantillas con ePlan)", "Biblioteka (Szablony z ePlan)", "Bibliotheek (Sjablonen met mini ePlan)"])
}
pub const fn in_library_btn(l: Language) -> &'static str {
    pick(l, ["In Bibliothek", "Save to Library", "Dans biblioth.", "In libreria", "A biblioteca", "Do biblioteki", "Naar biblioth."])
}
pub const fn in_library_tip(l: Language) -> &'static str {
    pick(l, [
        "Aktive Klemmleiste unter eigenem Namen in der Bibliothek speichern",
        "Save active terminal strip under a custom name in your library",
        "Enregistrer le bornier actif sous un nom dans la bibliothèque",
        "Salva la morsettiera attiva con nome nella libreria",
        "Guardar el bornero activo con nombre en la biblioteca",
        "Zapisz aktywną listwę pod własną nazwą w bibliotece",
        "Actieve klemmenstrook onder eigen naam opslaan in bibliotheek",
    ])
}
pub const fn lib_manage_tip(l: Language) -> &'static str {
    pick(l, [
        "Bibliothek verwalten (Umbenennen / Löschen)",
        "Manage library (Rename / Delete)",
        "Gérer la bibliothèque (Renommer / Supprimer)",
        "Gestisci libreria (Rinomina / Elimina)",
        "Gestionar biblioteca (Renombrar / Eliminar)",
        "Zarządzaj biblioteką (Zmień nazwę / Usuń)",
        "Bibliotheek beheren (Hernoemen / Verwijderen)",
    ])
}
pub const fn lib_save_active_tip(l: Language) -> &'static str {
    pick(l, [
        "Aktive Klemmleiste in Bibliothek speichern",
        "Save active terminal strip to library",
        "Enregistrer le bornier actif dans la bibliothèque",
        "Salva morsettiera attiva nella libreria",
        "Guardar bornero activo en la biblioteca",
        "Zapisz aktywną listwę w bibliotece",
        "Actieve klemmenstrook opslaan in bibliotheek",
    ])
}
pub const fn lib_search_hint(l: Language) -> &'static str {
    pick(l, ["Vorlage suchen…", "Search templates…", "Rechercher modèle…", "Cerca modello…", "Buscar plantilla…", "Szukaj szablonu…", "Sjabloon zoeken…"])
}
pub const fn lib_empty_title(l: Language) -> &'static str {
    pick(l, ["Noch keine Vorlagen gespeichert.", "No templates saved yet.", "Aucun modèle enregistré.", "Nessun modello salvato.", "Aún no hay plantillas guardadas.", "Brak zapisanych szablonów.", "Nog geen sjablonen opgeslagen."])
}
pub const fn lib_empty_sub(l: Language) -> &'static str {
    pick(l, [
        "Aktive Leiste über ‚In Bibliothek‘ sichern, um sie hier mit ePlan-Vorschau abzulegen.",
        "Save the active strip via 'Save to Library' to store it here with an ePlan preview.",
        "Enregistrez le bornier actif via « Dans biblioth. » pour l'afficher ici avec aperçu ePlan.",
        "Salva la morsettiera attiva con «In libreria» per vederla qui con anteprima ePlan.",
        "Guarda el bornero activo con «A biblioteca» para verlo aquí con vista previa ePlan.",
        "Zapisz aktywną listwę przyciskiem „Do biblioteki”, aby zapisać ją tutaj z podglądem ePlan.",
        "Sla de actieve strook op via 'Naar biblioth.' om hem hier met ePlan-voorbeeld weer te geven.",
    ])
}
pub fn lib_no_hits(l: Language, q: &str) -> String {
    match l {
        Language::De => format!("Keine Treffer für „{q}“."),
        Language::En => format!("No matches for \"{q}\"."),
        Language::Fr => format!("Aucun résultat pour « {q} »."),
        Language::It => format!("Nessun risultato per «{q}»."),
        Language::Es => format!("Sin resultados para «{q}»."),
        Language::Pl => format!("Brak wyników dla „{q}”."),
        Language::Nl => format!("Geen resultaten voor \"{q}\"."),
    }
}
pub const fn insert_btn(l: Language) -> &'static str {
    pick(l, ["Einfügen", "Insert", "Insérer", "Inserisci", "Insertar", "Wstaw", "Invoegen"])
}
pub const fn insert_lib_tip(l: Language) -> &'static str {
    pick(l, [
        "Diese Klemmleisten-Vorlage ins aktive Projekt einfügen",
        "Insert this terminal strip template into the active project",
        "Insérer ce modèle de bornier dans le projet actif",
        "Inserisci questo modello nel progetto attivo",
        "Insertar esta plantilla en el proyecto activo",
        "Wstaw ten szablon listwy do aktywnego projektu",
        "Dit klemmenstrooksjabloon invoegen in actief project",
    ])
}

// --- Library Modal ---

pub const fn save_to_lib_modal_title(l: Language) -> &'static str {
    pick(l, ["Klemmleiste in Bibliothek speichern", "Save Terminal Strip to Library", "Enregistrer bornier en bibliothèque", "Salva morsettiera in libreria", "Guardar bornero en biblioteca", "Zapisz listwę w bibliotece", "Klemmenstrook opslaan in bibliotheek"])
}
pub const fn save_to_lib_desc(l: Language) -> &'static str {
    pick(l, [
        "Speichert Aufbau, Klemmentypen, Brückungs-Potentiale, Nummerierungsmodus und Endhalter als wiederverwendbare Vorlage.",
        "Saves layout, terminal types, bridge potentials, numbering mode and end brackets as a reusable template.",
        "Enregistre la structure, les types de bornes, les potentiels, la numérotation et les butées comme modèle réutilisable.",
        "Salva struttura, tipi di morsetti, potenziali, numerazione e fermi terminali come modello riutilizzabile.",
        "Guarda estructura, tipos de bornas, potenciales, numeración y topes finales como plantilla reutilizable.",
        "Zapisuje układ, typy zacisków, potencjały mostków, tryb numeracji i trzymacze jako szablon wielokrotnego użytku.",
        "Slaat indeling, klemtypen, brugpotentialen, nummeringsmodus en eindsteunen op als herbruikbaar sjabloon.",
    ])
}
pub fn active_strip_line(l: Language, name: &str, count: usize) -> String {
    match l {
        Language::De => format!("Aktive Leiste: {name} ({count} Klemmen)"),
        Language::En => format!("Active strip: {name} ({count} blocks)"),
        Language::Fr => format!("Bornier actif : {name} ({count} bornes)"),
        Language::It => format!("Morsettiera attiva: {name} ({count} morsetti)"),
        Language::Es => format!("Bornero activo: {name} ({count} bornas)"),
        Language::Pl => format!("Aktywna listwa: {name} ({count} zacisków)"),
        Language::Nl => format!("Actieve strook: {name} ({count} klemmen)"),
    }
}
pub const fn tpl_name_required(l: Language) -> &'static str {
    pick(l, ["Vorlagenname *", "Template name *", "Nom du modèle *", "Nome modello *", "Nombre de plantilla *", "Nazwa szablonu *", "Sjabloonnaam *"])
}
pub const fn tpl_name_prompt(l: Language) -> &'static str {
    pick(l, ["Bitte einen Namen für die Vorlage eingeben.", "Please enter a template name.", "Veuillez saisir un nom de modèle.", "Inserire un nome per il modello.", "Introduzca un nombre para la plantilla.", "Podaj nazwę szablonu.", "Voer een naam in voor het sjabloon."])
}
pub fn tpl_overwrite_chk(l: Language, name: &str) -> String {
    match l {
        Language::De => format!("Vorhandene Vorlage „{name}“ aktualisieren (überschreiben)"),
        Language::En => format!("Update (overwrite) existing template \"{name}\""),
        Language::Fr => format!("Mettre à jour (écraser) le modèle « {name} »"),
        Language::It => format!("Aggiorna (sovrascrivi) il modello «{name}»"),
        Language::Es => format!("Actualizar (sobrescribir) plantilla «{name}»"),
        Language::Pl => format!("Zaktualizuj (nadpisz) istniejący szablon „{name}”"),
        Language::Nl => format!("Bestaand sjabloon \"{name}\" bijwerken (overschrijven)"),
    }
}
pub const fn lib_modal_title(l: Language) -> &'static str {
    pick(l, ["Persönliche Klemmleisten-Bibliothek", "Personal Terminal Strip Library", "Bibliothèque personnelle de borniers", "Libreria personale morsettiere", "Biblioteca personal de borneros", "Osobista biblioteka listew", "Persoonlijke klemmenstrook-bibliotheek"])
}
pub const fn lib_pcwide_desc(l: Language) -> &'static str {
    pick(l, [
        "Projektübergreifend auf diesem PC gespeichert (für alle Projekte verfügbar).",
        "Stored across projects on this PC (available in every project).",
        "Enregistrée sur ce PC de façon permanente (disponible dans tous les projets).",
        "Salvata su questo PC (disponibile in tutti i progetti).",
        "Guardada en este PC entre proyectos (disponible en todos los proyectos).",
        "Zapisywana na tym komputerze pomiędzy projektami (dostępna wszędzie).",
        "Projectoverstijgend opgeslagen op deze pc (beschikbaar in alle projecten).",
    ])
}
pub const fn lib_search_full_hint(l: Language) -> &'static str {
    pick(l, [
        "Nach Name, Beschreibung, Klemmentyp oder Art.-Nr. suchen …",
        "Search by name, description, terminal type or part no.…",
        "Rechercher par nom, description, type ou réf.…",
        "Cerca per nome, descrizione, tipo o cod. articolo…",
        "Buscar por nombre, descripción, tipo o n.º de art.…",
        "Szukaj wg nazwy, opisu, typu lub nr kat.…",
        "Zoek op naam, beschrijving, klemtype of art.nr.…",
    ])
}
pub const fn lib_target_bmk(l: Language) -> &'static str {
    pick(l, [
        "Ziel-Bezeichnung beim Einfügen (optional):",
        "Target strip designation on insert (optional):",
        "Repère cible à l'insertion (optionnel) :",
        "Sigla di destinazione all'inserimento (opz.):",
        "Designación destino al insertar (opcional):",
        "Docelowe oznaczenie przy wstawianiu (opcj.):",
        "Doelcodering bij invoegen (optioneel):",
    ])
}
pub const fn lib_none_yet(l: Language) -> &'static str {
    pick(l, ["Noch keine eigenen Klemmleisten gespeichert.", "No saved terminal strips yet.", "Aucun bornier personnel enregistré.", "Nessuna morsettiera salvata.", "Aún no hay borneros guardados.", "Nie zapisano jeszcze żadnych listew.", "Nog geen eigen klemmenstroken opgeslagen."])
}
pub const fn lib_none_hint(l: Language) -> &'static str {
    pick(l, [
        "Konfiguriere eine Klemmleiste im Editor und klicke links unter „Klemmleisten“ auf „In Bibliothek“, um sie hier dauerhaft mit eigenem Namen abzulegen.",
        "Configure a terminal strip in the editor and click \"Save to Library\" under \"Terminal Strips\" on the left to store it permanently.",
        "Configurez un bornier dans l'éditeur puis cliquez à gauche sur « Dans biblioth. » pour l'enregistrer durablement ici.",
        "Configura una morsettiera nell'editor e clicca a sinistra su «In libreria» per salvarla qui in modo permanente.",
        "Configura un bornero en el editor y pulsa «A biblioteca» a la izquierda para guardarlo permanentemente.",
        "Skonfiguruj listwę w edytorze i kliknij „Do biblioteki” po lewej, aby zapisać ją na stałe.",
        "Configureer een klemmenstrook in de editor en klik links op \"Naar biblioth.\" om hem hier permanent op te slaan.",
    ])
}
pub fn lib_empty_query(l: Language, q: &str) -> String {
    match l {
        Language::De => format!("Keine Vorlagen gefunden für „{q}“."),
        Language::En => format!("No templates found for \"{q}\"."),
        Language::Fr => format!("Aucun modèle trouvé pour « {q} »."),
        Language::It => format!("Nessun modello trovato per «{q}»."),
        Language::Es => format!("No se encontraron plantillas para «{q}»."),
        Language::Pl => format!("Nie znaleziono szablonów dla „{q}”."),
        Language::Nl => format!("Geen sjablonen gevonden voor \"{q}\"."),
    }
}
pub const fn tpl_name_colon(l: Language) -> &'static str {
    pick(l, ["Vorlagenname:", "Template name:", "Nom du modèle :", "Nome modello:", "Nombre plantilla:", "Nazwa szablonu:", "Sjabloonnaam:"])
}
pub const fn insert_into_proj(l: Language) -> &'static str {
    pick(l, ["In Projekt einfügen", "Insert into Project", "Insérer dans le projet", "Inserisci nel progetto", "Insertar en proyecto", "Wstaw do projektu", "In project invoegen"])
}
pub const fn start_proj_with(l: Language) -> &'static str {
    pick(l, ["Projekt damit starten", "Start Project with This", "Démarrer projet avec", "Avvia progetto con", "Iniciar proyecto con", "Rozpocznij projekt od", "Project hiermee starten"])
}
pub const fn confirm_delete_q(l: Language) -> &'static str {
    pick(l, ["Wirklich löschen?", "Really delete?", "Supprimer ?", "Eliminare davvero?", "¿Eliminar realmente?", "Na pewno usunąć?", "Echt verwijderen?"])
}
pub const fn save_active_strip_btn(l: Language) -> &'static str {
    pick(l, ["Aktive Leiste speichern", "Save Active Strip", "Enregistrer bornier actif", "Salva morsettiera attiva", "Guardar bornero activo", "Zapisz aktywną listwę", "Actieve strook opslaan"])
}

// --- New Strip Modal & NT Install Supply Modal ---

pub const fn no_matching_terminals(l: Language) -> &'static str {
    pick(l, ["Keine passenden Klemmen gefunden.", "No matching terminals found.", "Aucune borne correspondante.", "Nessun morsetto corrispondente.", "No se encontraron bornas.", "Brak pasujących zacisków.", "Geen bijpassende klemmen gevonden."])
}
pub fn global_search_hits(l: Language, count: usize) -> String {
    match l {
        Language::De => format!("Globale Suche über alle Hersteller ({count} Treffer)"),
        Language::En => format!("Global search across all manufacturers ({count} matches)"),
        Language::Fr => format!("Recherche globale tous fabricants ({count} résultats)"),
        Language::It => format!("Ricerca globale su tutti i produttori ({count} risultati)"),
        Language::Es => format!("Búsqueda global en todos los fabricantes ({count} resultados)"),
        Language::Pl => format!("Globalne wyszukiwanie dla wszystkich marek ({count} wyników)"),
        Language::Nl => format!("Globaal zoeken over alle fabrikanten ({count} resultaten)"),
    }
}
pub fn more_hits_hint(l: Language, rem: usize) -> String {
    match l {
        Language::De => format!("… {rem} weitere Treffer (Suchbegriff eingrenzen)"),
        Language::En => format!("… {rem} more matches (refine search query)"),
        Language::Fr => format!("… {rem} autres résultats (précisez la recherche)"),
        Language::It => format!("… altri {rem} risultati (affina la ricerca)"),
        Language::Es => format!("… {rem} resultados más (afinar búsqueda)"),
        Language::Pl => format!("… jeszcze {rem} wyników (zawęź wyszukiwanie)"),
        Language::Nl => format!("… nog {rem} resultaten (verfijn zoekopdracht)"),
    }
}
pub const fn matching_series_types(l: Language) -> &'static str {
    pick(l, ["Passende Typen (gleiche Serie / Profil)", "Matching types (same series / profile)", "Types compatibles (même série / profil)", "Tipi compatibili (stessa serie / profilo)", "Tipos compatibles (misma serie / perfil)", "Pasujące typy (ta sama seria / profil)", "Bijpassende typen (zelfde serie / profiel)"])
}
pub const fn more_types(l: Language) -> &'static str {
    pick(l, ["Weitere Typen", "Other types", "Autres types", "Altri tipi", "Otros tipos", "Inne typy", "Overige typen"])
}
pub fn nt_modal_title(l: Language, name: &str, pos: usize, total: usize) -> String {
    match l {
        Language::De => format!("NT-Konfiguration für {name} ({pos}/{total})"),
        Language::En => format!("NT configuration for {name} ({pos}/{total})"),
        Language::Fr => format!("Configuration NT pour {name} ({pos}/{total})"),
        Language::It => format!("Configurazione NT per {name} ({pos}/{total})"),
        Language::Es => format!("Configuración NT para {name} ({pos}/{total})"),
        Language::Pl => format!("Konfiguracja NT dla {name} ({pos}/{total})"),
        Language::Nl => format!("NT-configuratie voor {name} ({pos}/{total})"),
    }
}
pub fn nt_modal_title_single(l: Language, name: &str) -> String {
    match l {
        Language::De => format!("NT-Konfiguration für {name}"),
        Language::En => format!("NT configuration for {name}"),
        Language::Fr => format!("Configuration NT pour {name}"),
        Language::It => format!("Configurazione NT per {name}"),
        Language::Es => format!("Configuración NT para {name}"),
        Language::Pl => format!("Konfiguracja NT dla {name}"),
        Language::Nl => format!("NT-configuratie voor {name}"),
    }
}
pub const fn nt_modal_heading(l: Language) -> &'static str {
    pick(l, [
        "Installations- / NT-Klemme, N-Einspeisung & Sammelschienenträger",
        "Installation / NT Terminal, N-Supply & Busbar Carriers",
        "Borne d'installation / NT, alimentation N et supports de barre",
        "Morsetto installazione / NT, alimentazione N e supporti barra",
        "Borna de instalación / NT, alimentación N y soportes de barra",
        "Zacisk instalacyjny / NT, zasilanie N i wsporniki szyny",
        "Installatie- / NT-klem, N-voeding & railhouders",
    ])
}
pub const fn nt_modal_subtitle(l: Language) -> &'static str {
    pick(l, [
        "Für NT-Trennklemmen (mit N-Trennschlitten) bitte Ausführung, N-Einspeiseklemme und Sammelschienenträger (Auflagebock) wählen:",
        "Select installation variant, N-supply terminal, and busbar carriers for NT disconnect terminals (with neutral slide):",
        "Pour les bornes NT (avec curseur N), choisissez la variante, la borne d'alimentation N et les supports de barre :",
        "Per i morsetti sezionatori NT scegliere la variante, il morsetto di alimentazione N e i supporti barra:",
        "Para bornas seccionadoras NT seleccione la variante, borna de alimentación N y soportes de barra:",
        "Dla zacisków NT wybierz wariant, zacisk zasilający N oraz wsporniki szyny zbiorczej:",
        "Kies voor NT-scheidingsklemmen (met N-scheidingsschuif) de uitvoering, N-voedingsklem en railhouders:",
    ])
}
pub const fn nt_pdf_hints(l: Language) -> &'static str {
    pick(l, ["Hinweise aus PDF-Stückliste:", "Hints from PDF BOM:", "Indications de la nomenclature PDF :", "Note dalla distinta PDF:", "Notas de la lista PDF:", "Wskazówki z listy PDF:", "Aanwijzingen uit PDF-stuklijst:"])
}
pub const fn nt_step1(l: Language) -> &'static str {
    pick(l, ["1. Ausführung der Installationsklemme (NT / Standard)", "1. Installation Terminal Variant (NT / Standard)", "1. Variante de borne d'installation (NT / Standard)", "1. Variante morsetto installazione (NT / Standard)", "1. Variante borna de instalación (NT / Estándar)", "1. Wariant zacisku instalacyjnego (NT / Standard)", "1. Uitvoering installatieklem (NT / Standaard)"])
}
pub const fn nt_step2(l: Language) -> &'static str {
    pick(l, ["2. N-Einspeiseklemme hinzufügen?", "2. Add N-Supply Terminal?", "2. Ajouter une borne d'alimentation N ?", "2. Aggiungere morsetto alimentazione N?", "2. ¿Añadir borna de alimentación N?", "2. Dodać zacisk zasilający N?", "2. N-voedingsklem toevoegen?"])
}
pub const fn nt_keep_selected(l: Language) -> &'static str {
    pick(l, ["Gewählte Klemme beibehalten", "Keep selected terminal", "Conserver la borne sélectionnée", "Mantieni morsetto selezionato", "Mantener borna seleccionada", "Zachowaj wybrany zacisk", "Geselecteerde klem behouden"])
}
pub const fn nt_selected_nt(l: Language) -> &'static str {
    pick(l, ["NT-Trennklemme mit N-Trennschlitten ausgewählt", "NT disconnect terminal with N-slide selected", "Borne sectionnable NT avec curseur sélectionnée", "Morsetto NT con cursore selezionato", "Borna NT con deslizador seleccionada", "Wybrano zacisk NT z suwakiem N", "NT-scheidingsklem met N-schuif geselecteerd"])
}
pub const fn nt_selected(l: Language) -> &'static str {
    pick(l, ["Ausgewählt", "Selected", "Sélectionné", "Selezionato", "Seleccionado", "Wybrano", "Geselecteerd"])
}
pub const fn nt_already_supply(l: Language) -> &'static str {
    pick(l, ["Hinweis: In dieser Klemmleiste befindet sich bereits eine N-Einspeiseklemme.", "Note: This strip already contains an N-supply terminal.", "Remarque : ce bornier contient déjà une borne d'alimentation N.", "Nota: questa morsettiera contiene già un morsetto di alimentazione N.", "Nota: este bornero ya contiene una borna de alimentación N.", "Uwaga: ta listwa zawiera już zacisk zasilający N.", "Opmerking: deze strook bevat al een N-voedingsklem."])
}
pub const fn nt_yes_before(l: Language) -> &'static str {
    pick(l, ["Ja – Vor den Installationsklemmen einfügen (links einspeisen)", "Yes – Insert before installation blocks (left feed)", "Oui – Insérer avant les bornes d'installation (gauche)", "Sì – Inserisci prima dei morsetti d'installazione (sinistra)", "Sí – Insertar antes de las bornas de instalación (izquierda)", "Tak – Wstaw przed zaciskami instalacyjnymi (z lewej)", "Ja – Vóór de installatieklemmen invoegen (links voeden)"])
}
pub const fn nt_yes_after(l: Language) -> &'static str {
    pick(l, ["Ja – Nach den Installationsklemmen einfügen (rechts einspeisen)", "Yes – Insert after installation blocks (right feed)", "Oui – Insérer après les bornes d'installation (droite)", "Sì – Inserisci dopo i morsetti d'installazione (destra)", "Sí – Insertar después de las bornas de instalación (derecha)", "Tak – Wstaw po zaciskach instalacyjnych (z prawej)", "Ja – Na de installatieklemmen invoegen (rechts voeden)"])
}
pub const fn nt_no_supply(l: Language) -> &'static str {
    pick(l, ["Nein – Keine N-Einspeiseklemme hinzufügen", "No – Do not add N-supply terminal", "Non – Ne pas ajouter de borne d'alimentation N", "No – Non aggiungere morsetto di alimentazione N", "No – No añadir borna de alimentación N", "Nie – Nie dodawaj zacisku zasilającego N", "Nee – Geen N-voedingsklem toevoegen"])
}
pub const fn nt_pick_supply(l: Language) -> &'static str {
    pick(l, ["Auswahl N-Einspeiseklemme:", "Select N-supply terminal:", "Choix borne d'alimentation N :", "Scelta morsetto alimentazione N:", "Selección borna alimentación N:", "Wybór zacisku zasilającego N:", "Keuze N-voedingsklem:"])
}
pub const fn nt_step3(l: Language) -> &'static str {
    pick(l, ["3. Sammelschienenträger / Auflagebock (N-Sammelschiene 10×3 mm)", "3. Busbar Carriers / Support (Neutral Busbar 10×3 mm)", "3. Supports de barre collectrice N (10×3 mm)", "3. Supporti barra collettore N (10×3 mm)", "3. Soportes de barra colectora N (10×3 mm)", "3. Wsporniki szyny zbiorczej N (10×3 mm)", "3. Railhouders / ondersteuning (N-verzamelrail 10×3 mm)"])
}
pub const fn nt_already_carriers(l: Language) -> &'static str {
    pick(l, ["Hinweis: In dieser Klemmleiste sind bereits Sammelschienenträger / Auflageböcke platziert.", "Note: Busbar carriers are already placed on this strip.", "Remarque : des supports de barre sont déjà présents sur ce bornier.", "Nota: sono già presenti supporti barra su questa morsettiera.", "Nota: ya hay soportes de barra en este bornero.", "Uwaga: na tej listwie umieszczono już wsporniki szyny.", "Opmerking: er zijn al railhouders geplaatst op deze strook."])
}
pub const fn nt_outer_carriers(l: Language) -> &'static str {
    pick(l, [
        "Äußere Sammelschienenträger / Auflageböcke vor N-Einspeiseklemme & nach letzter Klemme setzen",
        "Place outer busbar carriers before N-supply & after last installation terminal",
        "Placer les supports extérieurs avant l'alimentation N et après la dernière borne",
        "Posiziona supporti barra esterni prima dell'alimentazione N e dopo l'ultimo morsetto",
        "Colocar soportes de barra exteriores antes de alimentación N y tras última borna",
        "Umieść zewnętrzne wsporniki szyny przed zasilaniem N i za ostatnim zaciskiem",
        "Plaats buitenste railhouders vóór de N-voeding en na de laatste klem",
    ])
}
pub const fn nt_intermediate_supports(l: Language) -> &'static str {
    pick(l, [
        "Bei langen Klemmenreihen automatisch alle 200 mm Zwischenstütze einfügen",
        "Automatically insert intermediate busbar support every 200 mm for long rows",
        "Insérer automatiquement un support intermédiaire tous les 200 mm",
        "Inserisci automaticamente supporto intermedio ogni 200 mm per file lunghe",
        "Insertar soporte intermedio automáticamente cada 200 mm en filas largas",
        "Automatycznie wstawiaj wspornik pośredni co 200 mm przy długich listwach",
        "Automatisch om de 200 mm een tussensteun plaatsen bij lange klemmenrijen",
    ])
}
pub const fn nt_apply_insert(l: Language) -> &'static str {
    pick(l, ["Übernehmen & Einfügen", "Apply & Insert", "Appliquer & Insérer", "Applica e inserisci", "Aplicar e insertar", "Zastosuj i wstaw", "Toepassen & invoegen"])
}

// --- PDF Import Modal ---

pub const fn pdf_import_title(l: Language) -> &'static str {
    pick(l, ["PDF-Stückliste importieren (EPLAN)", "Import PDF BOM (EPLAN)", "Importer nomenclature PDF (EPLAN)", "Importa distinta PDF (EPLAN)", "Importar lista PDF (EPLAN)", "Importuj listę PDF (EPLAN)", "PDF-stuklijst importeren (EPLAN)"])
}
pub fn pdf_import_file_title(l: Language, src: &str) -> String {
    match l {
        Language::De => format!("PDF-Stückliste importieren: {src}"),
        Language::En => format!("Import PDF BOM: {src}"),
        Language::Fr => format!("Importer nomenclature PDF : {src}"),
        Language::It => format!("Importa distinta PDF: {src}"),
        Language::Es => format!("Importar lista PDF: {src}"),
        Language::Pl => format!("Importuj listę PDF: {src}"),
        Language::Nl => format!("PDF-stuklijst importeren: {src}"),
    }
}
pub const fn pdf_no_data(l: Language) -> &'static str {
    pick(l, ["Keine Stücklistendaten geladen.", "No BOM data loaded.", "Aucune nomenclature chargée.", "Nessun dato distinta caricato.", "Sin datos de lista cargados.", "Brak wczytanych danych listy.", "Geen stuklijstgegevens geladen."])
}
pub fn pdf_summary(l: Language, strips: usize, blocks: usize, foreign: usize, filtered: usize) -> String {
    match l {
        Language::De => format!("{strips} Klemmleisten erkannt  |  {blocks} Klemmen gesamt  |  {foreign} Fremdartikel übersprungen  |  {filtered} Nicht-Klemmen (-Q/-T/-W) gefiltert"),
        Language::En => format!("{strips} strips detected  |  {blocks} terminals total  |  {foreign} non-catalog skipped  |  {filtered} non-terminals (-Q/-T/-W) filtered"),
        Language::Fr => format!("{strips} borniers détectés  |  {blocks} bornes au total  |  {foreign} articles ignorés  |  {filtered} hors-bornes (-Q/-T/-W) filtrés"),
        Language::It => format!("{strips} morsettiere rilevate  |  {blocks} morsetti totali  |  {foreign} articoli saltati  |  {filtered} non-morsetti (-Q/-T/-W) filtrati"),
        Language::Es => format!("{strips} borneros detectados  |  {blocks} bornas en total  |  {foreign} artículos omitidos  |  {filtered} no-bornas (-Q/-T/-W) filtrados"),
        Language::Pl => format!("Wykryto {strips} listew  |  {blocks} zacisków łącznie  |  pominięto {foreign} obcych  |  odfiltrowano {filtered} (-Q/-T/-W)"),
        Language::Nl => format!("{strips} stroken herkend  |  {blocks} klemmen totaal  |  {foreign} overgeslagen  |  {filtered} niet-klemmen (-Q/-T/-W) gefilterd"),
    }
}
pub const fn pdf_all_selectable(l: Language) -> &'static str {
    pick(l, ["Alle wählbaren", "All selectable", "Tous sélectionnables", "Tutti selezionabili", "Todos seleccionables", "Wszystkie dostępne", "Alle selecteerbare"])
}
pub const fn pdf_none_found(l: Language) -> &'static str {
    pick(l, [
        "Keine Klemmleisten (-X...) mit unterstützten Herstellern (WAGO, Phoenix Contact, Weidmüller) gefunden.",
        "No terminal strips (-X...) with supported manufacturers (WAGO, Phoenix Contact, Weidmüller) found.",
        "Aucun bornier (-X...) de fabricant pris en charge (WAGO, Phoenix Contact, Weidmüller) trouvé.",
        "Nessuna morsettiera (-X...) con produttori supportati (WAGO, Phoenix Contact, Weidmüller) trovata.",
        "No se encontraron borneros (-X...) de fabricantes soportados (WAGO, Phoenix Contact, Weidmüller).",
        "Nie znaleziono listew (-X...) obsługiwanych producentów (WAGO, Phoenix Contact, Weidmüller).",
        "Geen klemmenstroken (-X...) van ondersteunde fabrikanten (WAGO, Phoenix Contact, Weidmüller) gevonden.",
    ])
}
pub const fn pdf_nt_badge(l: Language) -> &'static str {
    pick(l, ["NT-Klemmen", "NT Terminals", "Bornes NT", "Morsetti NT", "Bornas NT", "Zaciski NT", "NT-klemmen"])
}
pub const fn pdf_end_parts_badge(l: Language) -> &'static str {
    pick(l, ["Endteile auto-ergänzt", "End parts auto-added", "Pièces fin ajoutées", "Terminali auto-aggiunti", "Tapas auto-añadidas", "Dodano elementy końc.", "Einddelen toegevoegd"])
}
pub const fn pdf_jumpers_ignored(l: Language) -> &'static str {
    pick(l, ["Brücker ignoriert", "Jumpers ignored", "Ponts ignorés", "Ponticelli ignorati", "Puentes ignorados", "Pominięto mostki", "Bruggen genegeerd"])
}
pub fn pdf_ignored_jumpers_detail(l: Language, txt: &str) -> String {
    match l {
        Language::De => format!("Brücker aus PDF ignoriert: {txt}"),
        Language::En => format!("Jumpers ignored from PDF: {txt}"),
        Language::Fr => format!("Ponts ignorés du PDF : {txt}"),
        Language::It => format!("Ponticelli ignorati dal PDF: {txt}"),
        Language::Es => format!("Puentes ignorados del PDF: {txt}"),
        Language::Pl => format!("Pominięte mostki z PDF: {txt}"),
        Language::Nl => format!("Bruggen uit PDF genegeerd: {txt}"),
    }
}
pub const fn pdf_arrangement_variant(l: Language) -> &'static str {
    pick(l, ["Anordnungsvariante (Stückzahlen identisch):", "Layout variant (quantities identical):", "Variante de disposition (quantités identiques) :", "Variante disposizione (quantità identiche):", "Variante de disposición (cantidades idénticas):", "Wariant układu (ilości identyczne):", "Indelingsvariant (aantallen identiek):"])
}
pub fn pdf_skipped_header(l: Language, f: usize, filt: usize) -> String {
    match l {
        Language::De => format!("Übersprungen & ignoriertes Zubehör ({f} Fremdhersteller, {filt} Nicht-Klemmen)"),
        Language::En => format!("Skipped & ignored accessories ({f} other manufacturers, {filt} non-terminals)"),
        Language::Fr => format!("Accessoires ignorés ({f} autres fabricants, {filt} hors-bornes)"),
        Language::It => format!("Accessori saltati e ignorati ({f} altri produttori, {filt} non-morsetti)"),
        Language::Es => format!("Accesorios omitidos e ignorados ({f} otros fabricantes, {filt} no-bornas)"),
        Language::Pl => format!("Pominięte akcesoria ({f} innych marek, {filt} nie-zacisków)"),
        Language::Nl => format!("Overgeslagen & genegeerd toebehoren ({f} overige merken, {filt} niet-klemmen)"),
    }
}
pub fn pdf_confirm_btn(l: Language, count: usize) -> String {
    match (l, count) {
        (Language::De, 0) => "Ausgewählte Leisten übernehmen".to_string(),
        (Language::De, n) => format!("Ausgewählte Leisten übernehmen ({n})"),
        (Language::En, 0) => "Import Selected Strips".to_string(),
        (Language::En, n) => format!("Import Selected Strips ({n})"),
        (Language::Fr, 0) => "Importer borniers sélectionnés".to_string(),
        (Language::Fr, n) => format!("Importer borniers sélectionnés ({n})"),
        (Language::It, 0) => "Importa morsettiere selezionate".to_string(),
        (Language::It, n) => format!("Importa morsettiere selezionate ({n})"),
        (Language::Es, 0) => "Importar borneros seleccionados".to_string(),
        (Language::Es, n) => format!("Importar borneros seleccionados ({n})"),
        (Language::Pl, 0) => "Importuj wybrane listwy".to_string(),
        (Language::Pl, n) => format!("Importuj wybrane listwy ({n})"),
        (Language::Nl, 0) => "Geselecteerde stroken importeren".to_string(),
        (Language::Nl, n) => format!("Geselecteerde stroken importeren ({n})"),
    }
}

// --- Database Tab & Spec Editor Modal ---

pub fn db_custom_badge(l: Language, edited: usize, custom: usize) -> String {
    match (l, edited, custom) {
        (Language::De, e, 0) => format!("{e} manuell angepasst"),
        (Language::De, 0, c) => format!("{c} eigene Typen"),
        (Language::De, e, c) => format!("{e} angepasst · {c} eigene"),
        (Language::En, e, 0) => format!("{e} manually edited"),
        (Language::En, 0, c) => format!("{c} custom types"),
        (Language::En, e, c) => format!("{e} edited · {c} custom"),
        (Language::Fr, e, 0) => format!("{e} modifiés"),
        (Language::Fr, 0, c) => format!("{c} personnalisés"),
        (Language::Fr, e, c) => format!("{e} modifiés · {c} perso."),
        (Language::It, e, 0) => format!("{e} modificati"),
        (Language::It, 0, c) => format!("{c} personalizzati"),
        (Language::It, e, c) => format!("{e} modificati · {c} pers."),
        (Language::Es, e, 0) => format!("{e} editados"),
        (Language::Es, 0, c) => format!("{c} propios"),
        (Language::Es, e, c) => format!("{e} editados · {c} propios"),
        (Language::Pl, e, 0) => format!("{e} zmodyfikowanych"),
        (Language::Pl, 0, c) => format!("{c} własnych"),
        (Language::Pl, e, c) => format!("{e} zmodyf. · {c} własne"),
        (Language::Nl, e, 0) => format!("{e} aangepast"),
        (Language::Nl, 0, c) => format!("{c} eigen typen"),
        (Language::Nl, e, c) => format!("{e} aangepast · {c} eigen"),
    }
}
pub const fn db_edit_hint(l: Language) -> &'static str {
    pick(l, [
        "Jeder Klemmentyp kann über „Bearbeiten“ manuell angepasst und PC-weit gespeichert werden.",
        "Any terminal type can be manually edited via \"Edit\" and saved permanently on this PC.",
        "Chaque type de borne peut être modifié manuellement via « Modifier » et enregistré sur ce PC.",
        "Ogni tipo di morsetto può essere modificato tramite «Modifica» e salvato su questo PC.",
        "Cualquier tipo de borna puede editarse manualmente mediante «Editar» y guardarse en este PC.",
        "Każdy typ zacisku można ręcznie edytować przyciskiem „Edytuj” i zapisać na tym komputerze.",
        "Elk klemtype kan handmatig via \"Bewerken\" worden aangepast en pc-breed worden opgeslagen.",
    ])
}
pub const fn db_global_search(l: Language) -> &'static str {
    pick(l, [
        "Globale Suche (alle Hersteller)",
        "Global search (all manufacturers)",
        "Recherche globale (tous fabricants)",
        "Ricerca globale (tutti i produttori)",
        "Búsqueda global (todos los fabricantes)",
        "Globalne wyszukiwanie (wszystkie marki)",
        "Globaal zoeken (alle fabrikanten)",
    ])
}
pub fn spec_editor_title(l: Language, name: &str) -> String {
    match l {
        Language::De => format!("Klemmentyp bearbeiten — {name}"),
        Language::En => format!("Edit terminal type — {name}"),
        Language::Fr => format!("Modifier le type de borne — {name}"),
        Language::It => format!("Modifica tipo morsetto — {name}"),
        Language::Es => format!("Editar tipo de borna — {name}"),
        Language::Pl => format!("Edytuj typ zacisku — {name}"),
        Language::Nl => format!("Klemtype bewerken — {name}"),
    }
}
pub fn spec_editor_subtitle(l: Language, id: &str) -> String {
    match l {
        Language::De => format!("ID: {id} · Änderungen werden sofort PC-weit (custom_specs.json) sowie im aktuellen Projekt gespeichert."),
        Language::En => format!("ID: {id} · Changes are saved PC-wide (custom_specs.json) and in the active project immediately."),
        Language::Fr => format!("ID : {id} · Modifications enregistrées sur ce PC (custom_specs.json) et dans le projet."),
        Language::It => format!("ID: {id} · Le modifiche vengono salvate su questo PC (custom_specs.json) e nel progetto."),
        Language::Es => format!("ID: {id} · Los cambios se guardan en este PC (custom_specs.json) y en el proyecto actual."),
        Language::Pl => format!("ID: {id} · Zmiany zapisywane są globalnie (custom_specs.json) i w projekcie."),
        Language::Nl => format!("ID: {id} · Wijzigingen worden direct pc-breed (custom_specs.json) en in het project opgeslagen."),
    }
}
pub const fn spec_master_data(l: Language) -> &'static str {
    pick(l, ["Stammdaten", "Master data", "Données de base", "Dati anagrafici", "Datos base", "Dane podstawowe", "Basisgegevens"])
}
pub const fn spec_label_name(l: Language) -> &'static str {
    pick(l, ["Typ / Bezeichnung:", "Type / Designation:", "Type / Désignation :", "Tipo / Sigla:", "Tipo / Designación:", "Typ / Oznaczenie:", "Type / Aanduiding:"])
}
pub const fn spec_label_mfg(l: Language) -> &'static str {
    pick(l, ["Hersteller:", "Manufacturer:", "Fabricant :", "Produttore:", "Fabricante:", "Producent:", "Fabrikant:"])
}
pub const fn spec_label_series(l: Language) -> &'static str {
    pick(l, ["Serie:", "Series:", "Série :", "Serie:", "Serie:", "Seria:", "Serie:"])
}
pub const fn spec_label_art(l: Language) -> &'static str {
    pick(l, ["Artikelnummer:", "Part number:", "Référence :", "Codice articolo:", "N.º de artículo:", "Nr katalogowy:", "Artikelnummer:"])
}
pub const fn spec_label_desc_de(l: Language) -> &'static str {
    pick(l, ["Beschreibung (DE):", "Description (DE):", "Description (DE) :", "Descrizione (DE):", "Descripción (DE):", "Opis (DE):", "Beschrijving (DE):"])
}
pub const fn spec_label_desc_en(l: Language) -> &'static str {
    pick(l, ["Beschreibung (EN):", "Description (EN):", "Description (EN) :", "Descrizione (EN):", "Descripción (EN):", "Opis (EN):", "Beschrijving (EN):"])
}
pub const fn spec_geo_heading(l: Language) -> &'static str {
    pick(l, ["Geometrie, Etagen & Brücken", "Geometry, Levels & Bridges", "Géométrie, Niveaux & Ponts", "Geometria, Piani e Ponticelli", "Geometría, Pisos y Puentes", "Geometria, Poziomy i Mostki", "Geometrie, Etages & Bruggen"])
}
pub const fn spec_levels(l: Language) -> &'static str {
    pick(l, ["Etagen:", "Levels:", "Niveaux :", "Piani:", "Pisos:", "Poziomy:", "Etages:"])
}
pub const fn spec_width(l: Language) -> &'static str {
    pick(l, ["Breite:", "Width:", "Largeur :", "Larghezza:", "Ancho:", "Szerokość:", "Breedte:"])
}
pub const fn spec_height(l: Language) -> &'static str {
    pick(l, ["Bauhöhe:", "Height:", "Hauteur :", "Altezza:", "Altura:", "Wysokość:", "Bouwhoogte:"])
}
pub const fn spec_rail_offset(l: Language) -> &'static str {
    pick(l, ["TS35-Versatz:", "TS35 offset:", "Décalage TS35 :", "Offset TS35:", "Desfase TS35:", "Przesunięcie TS35:", "TS35-offset:"])
}
pub const fn spec_jumpers_per_tier(l: Language) -> &'static str {
    pick(l, ["Brückerschächte / Etage:", "Jumper slots / level:", "Puits de pontage / niveau :", "Vani ponticello / piano:", "Ranuras de puente / piso:", "Gniazda mostków / poziom:", "Brugschachten / etage:"])
}
pub const fn spec_jumper_0(l: Language) -> &'static str {
    pick(l, ["0× (keine)", "0× (none)", "0× (aucun)", "0× (nessuno)", "0× (ninguno)", "0× (brak)", "0× (geen)"])
}
pub const fn spec_jumper_1(l: Language) -> &'static str {
    pick(l, ["1× (einfach)", "1× (single)", "1× (simple)", "1× (singolo)", "1× (simple)", "1× (pojedyncze)", "1× (enkel)"])
}
pub const fn spec_jumper_2(l: Language) -> &'static str {
    pick(l, ["2× (doppelt)", "2× (double)", "2× (double)", "2× (doppio)", "2× (doble)", "2× (podwójne)", "2× (dubbel)"])
}
pub const fn spec_max_poles(l: Language) -> &'static str {
    pick(l, ["Max. Brücker-Pole:", "Max jumper poles:", "Max pôles de pont :", "Max poli ponticello:", "Máx. polos de puente:", "Maks. liczba pól mostka:", "Max. aantal brugpolen:"])
}
pub const fn spec_poles_suffix(l: Language) -> &'static str {
    pick(l, ["-polig", "-pole", " pôles", " poli", " polos", "-bieg.", "-polig"])
}
pub const fn spec_num_order(l: Language) -> &'static str {
    pick(l, ["Zählrichtung:", "Tier order:", "Sens niveaux :", "Direzione piani:", "Orden de pisos:", "Kierunek numeracji:", "Telrichting:"])
}
pub const fn spec_bottom_to_top(l: Language) -> &'static str {
    pick(l, ["Unten → Oben", "Bottom → Top", "Bas → Haut", "Basso → Alto", "Abajo → Arriba", "Dół → Góra", "Onder → Boven"])
}
pub const fn spec_top_to_bottom(l: Language) -> &'static str {
    pick(l, ["Oben → Unten", "Top → Bottom", "Haut → Bas", "Alto → Basso", "Arriba → Abajo", "Góra → Dół", "Boven → Onder"])
}
pub fn spec_tier_n(l: Language, n: usize) -> String {
    match l {
        Language::De | Language::Nl => format!("Etage E{n}:"),
        Language::En => format!("Level L{n}:"),
        Language::Fr => format!("Niveau N{n} :"),
        Language::It | Language::Pl => format!("Piano P{n}:"),
        Language::Es => format!("Piso N{n}:"),
    }
}
pub const fn spec_links_heading(l: Language) -> &'static str {
    pick(l, ["Links (Produktseite & Produktfoto URL)", "Links (Product Page & Image URL)", "Liens (Page produit et URL image)", "Link (Pagina prodotto e URL immagine)", "Enlaces (Página producto y URL imagen)", "Linki (Strona produktu i URL zdjęcia)", "Links (Productpagina & Afbeelding-URL)"])
}
pub const fn spec_product_url(l: Language) -> &'static str {
    pick(l, ["Produkt-URL:", "Product URL:", "URL produit :", "URL prodotto:", "URL producto:", "URL produktu:", "Product-URL:"])
}
pub const fn spec_image_url(l: Language) -> &'static str {
    pick(l, ["Bild-URL:", "Image URL:", "URL image :", "URL immagine:", "URL imagen:", "URL zdjęcia:", "Afbeelding-URL:"])
}
pub const fn tier_color_label(l: Language, color: TierColor) -> &'static str {
    match color {
        TierColor::StandardGray => pick(l, ["Grau", "Gray", "Gris", "Grigio", "Gris", "Szary", "Grijs"]),
        TierColor::BlueNeutral => pick(l, ["Blau (N)", "Blue (N)", "Bleu (N)", "Blu (N)", "Azul (N)", "Niebieski (N)", "Blauw (N)"]),
        TierColor::GreenYellowPe => pick(l, ["Grün-Gelb (PE)", "Green-Yellow (PE)", "Vert-Jaune (PE)", "Giallo-Verde (PE)", "Verde-Amarillo (PE)", "Żółto-Zielony (PE)", "Groen-Geel (PE)"]),
        TierColor::RedPhase => pick(l, ["Rot", "Red", "Rouge", "Rosso", "Rojo", "Czerwony", "Rood"]),
        TierColor::OrangeAux => pick(l, ["Orange (Hilf/Trenn)", "Orange (Aux/Disc.)", "Orange (Aux/Sect.)", "Arancione (Aus.)", "Naranja (Aux.)", "Pomarańczowy (Pom.)", "Oranje (Hulp/Scheid.)"]),
    }
}

// --- CAD Canvas & Product Media ---

pub fn levels_summary(l: Language, levels: usize) -> &'static str {
    if levels == 1 {
        pick(l, ["Etage", "Level", "Niveau", "Piano", "Piso", "Poziom", "Etage"])
    } else {
        pick(l, ["Etagen", "Levels", "Niveaux", "Piani", "Pisos", "Poziomy", "Etages"])
    }
}
pub const fn empty_strip_canvas(l: Language) -> &'static str {
    pick(l, ["Leere Klemmleiste", "Empty terminal strip", "Bornier vide", "Morsettiera vuota", "Bornero vacío", "Pusta listwa", "Lege klemmenstrook"])
}
pub fn insert_same_after_canvas(l: Language, idx: usize, name: &str) -> String {
    match l {
        Language::De => format!("Gleiche Klemme direkt nach #{} hinzufügen ({name})", idx + 1),
        Language::En => format!("Add same terminal right after #{} ({name})", idx + 1),
        Language::Fr => format!("Ajouter la même borne juste après #{} ({name})", idx + 1),
        Language::It => format!("Aggiungi lo stesso morsetto subito dopo #{} ({name})", idx + 1),
        Language::Es => format!("Añadir la misma borna justo después de #{} ({name})", idx + 1),
        Language::Pl => format!("Dodaj taki sam zacisk tuż po #{} ({name})", idx + 1),
        Language::Nl => format!("Zelfde klem direct na #{} toevoegen ({name})", idx + 1),
    }
}
pub fn non_bridgeable_chambers(l: Language, chambers_label: &str) -> String {
    let suffix = pick(l, ["(nicht brückbar)", "(non-bridgeable)", "(non pontable)", "(non ponticellabile)", "(no puenteable)", "(niemożliwy do mostkowania)", "(niet brugbaar)"]);
    format!("0× {chambers_label} {suffix}")
}

// --- Command Palette ---

pub const fn cmd_search_hint(l: Language) -> &'static str {
    pick(l, [
        "Befehl, Klemmleiste oder Klemmentyp (z. B. 2002, PTI, Excel)…",
        "Command, terminal strip or block type (e.g. 2002, PTI, Excel)…",
        "Commande, bornier ou type de borne (ex. 2002, PTI, Excel)…",
        "Comando, morsettiera o tipo morsetto (es. 2002, PTI, Excel)…",
        "Comando, bornero o tipo de borna (p. ej. 2002, PTI, Excel)…",
        "Polecenie, listwa lub typ zacisku (np. 2002, PTI, Excel)…",
        "Opdracht, klemmenstrook of klemtype (bijv. 2002, PTI, Excel)…",
    ])
}
pub const fn cmd_no_hits(l: Language) -> &'static str {
    pick(l, [
        "Keine passenden Befehle oder Klemmen gefunden.",
        "No matching commands or terminals found.",
        "Aucune commande ni borne correspondante.",
        "Nessun comando o morsetto corrispondente.",
        "No se encontraron comandos ni bornas.",
        "Nie znaleziono pasujących poleceń ani zacisków.",
        "Geen overeenkomende opdrachten of klemmen gevonden.",
    ])
}
pub const fn cmd_footer_hint(l: Language) -> &'static str {
    pick(l, [
        "↑↓ Wählen  ·  Enter Ausführen  ·  Esc Schließen",
        "↑↓ Select  ·  Enter Run  ·  Esc Close",
        "↑↓ Choisir  ·  Entrée Exécuter  ·  Échap Fermer",
        "↑↓ Scegli  ·  Invio Esegui  ·  Esc Chiudi",
        "↑↓ Elegir  ·  Enter Ejecutar  ·  Esc Cerrar",
        "↑↓ Wybierz  ·  Enter Wykonaj  ·  Esc Zamknij",
        "↑↓ Kiezen  ·  Enter Uitvoeren  ·  Esc Sluiten",
    ])
}
pub fn cmd_strip_title(l: Language, name: &str) -> String {
    match l {
        Language::De => format!("Klemmleiste {name}"),
        Language::En => format!("Terminal strip {name}"),
        Language::Fr => format!("Bornier {name}"),
        Language::It => format!("Morsettiera {name}"),
        Language::Es => format!("Bornero {name}"),
        Language::Pl => format!("Listwa {name}"),
        Language::Nl => format!("Klemmenstrook {name}"),
    }
}
pub fn cmd_insert_block(l: Language, name: &str) -> String {
    match l {
        Language::De => format!("{name} einfügen"),
        Language::En => format!("Insert {name}"),
        Language::Fr => format!("Insérer {name}"),
        Language::It => format!("Inserisci {name}"),
        Language::Es => format!("Insertar {name}"),
        Language::Pl => format!("Wstaw {name}"),
        Language::Nl => format!("{name} invoegen"),
    }
}
pub const fn cmd_gen_title(l: Language) -> &'static str {
    pick(l, ["Generieren (Endplatten & NT prüfen)", "Generate (End plates & NT check)", "Générer (Flasques & vérification NT)", "Genera (Piastre e verifica NT)", "Generar (Tapas y comprobación NT)", "Generuj (Płytki i kontrola NT)", "Genereren (Eindplaten & NT-controle)"])
}
pub const fn cmd_gen_sub(l: Language) -> &'static str {
    pick(l, ["Berechnet Trennplatten und prüft NT-Einspeisung", "Calculates partition plates & checks NT feed", "Calcule les séparateurs et vérifie l'alimentation NT", "Calcola piastre separatrici e controlla NT", "Calcula separadores y comprueba alimentación NT", "Oblicza płytki rozdzielające i sprawdza zasilanie NT", "Berekent scheidingsplaten en controleert NT-voeding"])
}
pub const fn cmd_new_strip_title(l: Language) -> &'static str {
    pick(l, ["Neue Klemmleiste anlegen", "Create new terminal strip", "Créer un nouveau bornier", "Crea nuova morsettiera", "Crear nuevo bornero", "Utwórz nową listwę", "Nieuwe klemmenstrook aanmaken"])
}
pub const fn cmd_new_strip_sub(l: Language) -> &'static str {
    pick(l, ["Leisten-Assistent öffnen", "Open strip wizard", "Ouvrir l'assistant bornier", "Apri procedura guidata", "Abrir asistente de bornero", "Otwórz kreator listwy", "Strookassistent openen"])
}
pub const fn cmd_toggle_bridge_title(l: Language) -> &'static str {
    pick(l, ["Brücken-Modus umschalten", "Toggle Bridge Mode", "Basculer mode Ponts", "Attiva/disattiva Ponticelli", "Alternar modo Puentes", "Przełącz tryb mostków", "Bruggen-modus schakelen"])
}
pub const fn cmd_toggle_bridge_sub(l: Language) -> &'static str {
    pick(l, ["Potentiale & Kammbrücken zeichnen", "Draw potentials & comb jumpers", "Dessiner potentiels et peignes", "Disegna potenziali e ponticelli", "Dibujar potenciales y peines", "Rysuj potencjały i mostki grzebieniowe", "Potentialen & kammen tekenen"])
}
pub const fn cmd_toggle_eplan_title(l: Language) -> &'static str {
    pick(l, ["EPLAN 1:1 Aufbaubild umschalten", "Toggle EPLAN 1:1 Layout", "Basculer vue EPLAN 1:1", "Attiva/disattiva vista EPLAN 1:1", "Alternar vista EPLAN 1:1", "Przełącz widok EPLAN 1:1", "EPLAN 1:1 opbouw schakelen"])
}
pub const fn cmd_toggle_eplan_sub(l: Language) -> &'static str {
    pick(l, ["Maßstabsgetreue TS 35 Darstellung + DXF", "True-scale TS 35 view + DXF export", "Vue TS 35 échelle 1:1 + export DXF", "Vista TS 35 scala 1:1 + export DXF", "Vista TS 35 escala 1:1 + DXF", "Widok TS 35 1:1 + eksport DXF", "Ware schaal TS 35 weergave + DXF"])
}
pub const fn cmd_toggle_seq_title(l: Language) -> &'static str {
    pick(l, ["Zählfolge umschalten (Klemmenweise ↔ Ebenenweise)", "Toggle numbering order (Per block ↔ By level)", "Changer ordre numérotation (Borne ↔ Niveau)", "Cambia ordine numerazione (Morsetto ↔ Piano)", "Cambiar orden numeración (Borna ↔ Piso)", "Przełącz numerację (Zacisk ↔ Poziom)", "Telvolgorde wisselen (Per klem ↔ Per etage)"])
}
pub const fn cmd_toggle_seq_sub(l: Language) -> &'static str {
    pick(l, [
        "Umschalten zwischen 1,2 | 3,4 und Ebene 1: 1,2,3 → Ebene 2: 4,5,6",
        "Switch between 1,2 | 3,4 and Level 1: 1,2,3 → Level 2: 4,5,6",
        "Basculer entre 1,2 | 3,4 et Niveau 1 : 1,2,3 → Niveau 2 : 4,5,6",
        "Alterna tra 1,2 | 3,4 e Piano 1: 1,2,3 → Piano 2: 4,5,6",
        "Alternar entre 1,2 | 3,4 y Piso 1: 1,2,3 → Piso 2: 4,5,6",
        "Przełącz między 1,2 | 3,4 a Poziom 1: 1,2,3 → Poziom 2: 4,5,6",
        "Wisselen tussen 1,2 | 3,4 en Etage 1: 1,2,3 → Etage 2: 4,5,6",
    ])
}
pub const fn cmd_xlsx_title(l: Language) -> &'static str {
    pick(l, ["Excel-Export (.xlsx)", "Excel Export (.xlsx)", "Export Excel (.xlsx)", "Esporta Excel (.xlsx)", "Exportar Excel (.xlsx)", "Eksport Excel (.xlsx)", "Excel-export (.xlsx)"])
}
pub const fn cmd_csv_title(l: Language) -> &'static str {
    pick(l, ["CSV-Export (.csv)", "CSV Export (.csv)", "Export CSV (.csv)", "Esporta CSV (.csv)", "Exportar CSV (.csv)", "Eksport CSV (.csv)", "CSV-export (.csv)"])
}
pub const fn cmd_dxf_title(l: Language) -> &'static str {
    pick(l, ["DXF-Export (EPLAN 1:1 mm)", "DXF Export (EPLAN 1:1 mm)", "Export DXF (EPLAN 1:1 mm)", "Esporta DXF (EPLAN 1:1 mm)", "Exportar DXF (EPLAN 1:1 mm)", "Eksport DXF (EPLAN 1:1 mm)", "DXF-export (EPLAN 1:1 mm)"])
}
pub const fn cmd_dxf_sub(l: Language) -> &'static str {
    pick(l, ["Maßstabsgetreue CAD-Zeichnung exportieren", "Export true-scale CAD drawing", "Exporter le dessin CAO à l'échelle", "Esporta disegno CAD in scala reale", "Exportar dibujo CAD a escala real", "Eksportuj rysunek CAD w skali 1:1", "CAD-tekening op ware schaal exporteren"])
}
pub const fn cmd_n_to_nt_title(l: Language) -> &'static str {
    pick(l, ["Alle N-Klemmen in NT umwandeln", "Convert all N to NT", "Convertir bornes N en NT", "Converti morsetti N in NT", "Convertir bornas N a NT", "Zamień wszystkie N na NT", "Alle N-klemmen omzetten naar NT"])
}
pub const fn cmd_n_to_nt_sub(l: Language) -> &'static str {
    pick(l, ["Ersetzt Standard-N-Klemmen durch NT-Trennklemmen", "Replaces standard N terminals with NT disconnect blocks", "Remplace les bornes N standard par des bornes NT", "Sostituisce morsetti N standard con sezionatori NT", "Sustituye bornas N estándar por seccionadoras NT", "Zastępuje zaciski N zaciskami rozłącznymi NT", "Vervangt standaard N-klemmen door NT-scheidingsklemmen"])
}
pub const fn cmd_nt_to_n_title(l: Language) -> &'static str {
    pick(l, ["Alle NT-Klemmen in N zurückwandeln", "Convert all NT back to N", "Reconvertir bornes NT en N", "Riconverti morsetti NT in N", "Reconvertir bornas NT a N", "Przywróć wszystkie NT na N", "Alle NT-klemmen terugzetten naar N"])
}
pub const fn cmd_nt_to_n_sub(l: Language) -> &'static str {
    pick(l, ["Ersetzt NT-Trennklemmen durch Standard-N-Klemmen", "Replaces NT disconnect blocks with standard N terminals", "Remplace les bornes NT par des bornes N standard", "Sostituisce morsetti NT con morsetti N standard", "Sustituye bornas NT por bornas N estándar", "Zastępuje zaciski NT standardowymi zaciskami N", "Vervangt NT-klemmen door standaard N-klemmen"])
}
pub const fn cmd_save_proj_title(l: Language) -> &'static str {
    pick(l, ["Projekt speichern", "Save project", "Enregistrer le projet", "Salva progetto", "Guardar proyecto", "Zapisz projekt", "Project opslaan"])
}
pub const fn cmd_save_proj_sub(l: Language) -> &'static str {
    pick(l, ["Aktuelles Projekt als .termi sichern", "Save current project as .termi", "Sauvegarder le projet en .termi", "Salva progetto corrente come .termi", "Guardar proyecto actual como .termi", "Zapisz bieżący projekt jako .termi", "Huidig project opslaan als .termi"])
}
pub const fn cmd_undo_title(l: Language) -> &'static str {
    pick(l, ["Rückgängig (Undo)", "Undo", "Annuler (Undo)", "Annulla (Undo)", "Deshacer (Undo)", "Cofnij (Undo)", "Ongedaan maken (Undo)"])
}
pub const fn cmd_undo_sub(l: Language) -> &'static str {
    pick(l, ["Letzte Änderung rückgängig machen", "Revert the last change", "Annuler la dernière modification", "Annulla l'ultima modifica", "Deshacer el último cambio", "Cofnij ostatnią zmianę", "Laatste wijziging ongedaan maken"])
}
pub const fn cmd_redo_title(l: Language) -> &'static str {
    pick(l, ["Wiederherstellen (Redo)", "Redo", "Rétablir (Redo)", "Ripeti (Redo)", "Rehacer (Redo)", "Ponów (Redo)", "Opnieuw uitvoeren (Redo)"])
}
pub const fn cmd_redo_sub(l: Language) -> &'static str {
    pick(l, ["Rückgängig gemachten Schritt wiederherstellen", "Restore the undone step", "Rétablir l'étape annulée", "Ripristina il passo annullato", "Restaurar el paso deshecho", "Przywróć cofnięty krok", "Ongedaan gemaakte stap herstellen"])
}
pub const fn cmd_pdf_title(l: Language) -> &'static str {
    pick(l, ["PDF-Stückliste importieren", "Import PDF BOM", "Importer nomenclature PDF", "Importa distinta PDF", "Importar lista PDF", "Importuj listę PDF", "PDF-stuklijst importeren"])
}
pub const fn cmd_lib_title(l: Language) -> &'static str {
    pick(l, ["Bibliothek öffnen", "Open Library", "Ouvrir bibliothèque", "Apri libreria", "Abrir biblioteca", "Otwórz bibliotekę", "Bibliotheek openen"])
}
pub const fn cmd_lib_sub(l: Language) -> &'static str {
    pick(l, ["PC-weite Klemmleisten-Vorlagen durchsuchen", "Browse PC-wide terminal strip templates", "Parcourir les modèles de borniers du PC", "Sfoglia i modelli di morsettiere salvati", "Explorar plantillas de borneros guardadas", "Przeglądaj zapisane szablony listew", "Opgeslagen klemmenstrooksjablonen bekijken"])
}
pub const fn cmd_open_proj_title(l: Language) -> &'static str {
    pick(l, ["Projekt öffnen", "Open Project", "Ouvrir projet", "Apri progetto", "Abrir proyecto", "Otwórz projekt", "Project openen"])
}
pub const fn cmd_open_proj_sub(l: Language) -> &'static str {
    pick(l, ["Vorhandene .termi-Datei laden", "Load existing .termi file", "Charger un fichier .termi existant", "Carica un file .termi esistente", "Cargar archivo .termi existente", "Wczytaj istniejący plik .termi", "Bestaand .termi-bestand laden"])
}
pub const fn cmd_new_proj_title(l: Language) -> &'static str {
    pick(l, ["Neues Projekt starten", "Start New Project", "Démarrer nouveau projet", "Avvia nuovo progetto", "Iniciar nuevo proyecto", "Rozpocznij nowy projekt", "Nieuw project starten"])
}
pub const fn cmd_new_proj_sub(l: Language) -> &'static str {
    pick(l, ["Mit leerem Projekt beginnen", "Start with an empty project", "Commencer avec un projet vide", "Inizia con un progetto vuoto", "Comenzar con un proyecto vacío", "Zacznij od pustego projektu", "Beginnen met een leeg project"])
}
pub const fn cmd_theme_title(l: Language) -> &'static str {
    pick(l, ["Theme wechseln (System / Hell / Dunkel)", "Switch Theme (System / Light / Dark)", "Changer de thème (Système / Clair / Sombre)", "Cambia tema (Sistema / Chiaro / Scuro)", "Cambiar tema (Sistema / Claro / Oscuro)", "Zmień motyw (System / Jasny / Ciemny)", "Thema wisselen (Systeem / Licht / Donker)"])
}
pub const fn cmd_theme_sub(l: Language) -> &'static str {
    pick(l, ["Zwischen Hell-, Dunkel- und System-Design wechseln", "Switch between Light, Dark and System theme", "Basculer entre thème clair, sombre et système", "Alterna tra tema chiaro, scuro e di sistema", "Cambiar entre tema claro, oscuro y del sistema", "Przełącz między motywem jasnym, ciemnym i systemowym", "Wisselen tussen licht, donker en systeemthema"])
}

// --- Toast & Status Notifications ---

pub fn toast_spec_saved(l: Language, name: &str, updated_blocks: usize) -> String {
    match l {
        Language::De => {
            let s = if updated_blocks > 0 { format!(" ({updated_blocks} Klemmen im Projekt aktualisiert)") } else { String::new() };
            format!("Klemmentyp „{name}“ gespeichert (PC-weit aktiv){s}.")
        }
        Language::En => {
            let s = if updated_blocks > 0 { format!(" ({updated_blocks} block(s) updated in project)") } else { String::new() };
            format!("Terminal type \"{name}\" saved (PC-wide active){s}.")
        }
        Language::Fr => {
            let s = if updated_blocks > 0 { format!(" ({updated_blocks} borne(s) mise(s) à jour)") } else { String::new() };
            format!("Type « {name} » enregistré sur ce PC{s}.")
        }
        Language::It => {
            let s = if updated_blocks > 0 { format!(" ({updated_blocks} morsetti aggiornati)") } else { String::new() };
            format!("Tipo «{name}» salvato su questo PC{s}.")
        }
        Language::Es => {
            let s = if updated_blocks > 0 { format!(" ({updated_blocks} bornas actualizadas)") } else { String::new() };
            format!("Tipo «{name}» guardado en este PC{s}.")
        }
        Language::Pl => {
            let s = if updated_blocks > 0 { format!(" (zaktualizowano {updated_blocks} zacisków)") } else { String::new() };
            format!("Typ „{name}” zapisany na komputerze{s}.")
        }
        Language::Nl => {
            let s = if updated_blocks > 0 { format!(" ({updated_blocks} klemmen bijgewerkt)") } else { String::new() };
            format!("Klemtype \"{name}\" opgeslagen (pc-breed){s}.")
        }
    }
}
pub fn toast_spec_reset(l: Language, name: &str, restored: usize) -> String {
    match l {
        Language::De => {
            let s = if restored > 0 { format!(" ({restored} Klemmen aktualisiert)") } else { String::new() };
            format!("Klemmentyp „{name}“ auf Werkszustand zurückgesetzt{s}.")
        }
        Language::En => {
            let s = if restored > 0 { format!(" ({restored} block(s) updated)") } else { String::new() };
            format!("Terminal type \"{name}\" reset to factory default{s}.")
        }
        Language::Fr => format!("Type « {name} » réinitialisé par défaut."),
        Language::It => format!("Tipo «{name}» ripristinato ai valori predefiniti."),
        Language::Es => format!("Tipo «{name}» restablecido de fábrica."),
        Language::Pl => format!("Przywrócono ustawienia fabryczne typu „{name}”."),
        Language::Nl => format!("Klemtype \"{name}\" hersteld naar standaard."),
    }
}
pub const fn toast_spec_deleted(l: Language) -> &'static str {
    pick(l, ["Eigener Klemmentyp gelöscht.", "Custom terminal type deleted.", "Type personnalisé supprimé.", "Tipo personalizzato eliminato.", "Tipo propio eliminado.", "Własny typ zacisku usunięty.", "Eigen klemtype verwijderd."])
}
pub const fn toast_undone(l: Language) -> &'static str {
    pick(l, ["Schritt rückgängig gemacht.", "Step undone.", "Étape annulée.", "Passo annullato.", "Paso deshecho.", "Cofnięto krok.", "Stap ongedaan gemaakt."])
}
pub const fn toast_redone(l: Language) -> &'static str {
    pick(l, ["Schritt wiederhergestellt.", "Step redone.", "Étape rétablie.", "Passo ripristinato.", "Paso rehcho.", "Ponowiono krok.", "Stap hersteld."])
}
pub fn toast_inserted_from_lib(l: Language, strip_name: &str, tpl_name: &str) -> String {
    match l {
        Language::De => format!("Klemmleiste „{strip_name}“ aus Vorlage „{tpl_name}“ eingefügt."),
        Language::En => format!("Inserted strip \"{strip_name}\" from template \"{tpl_name}\"."),
        Language::Fr => format!("Bornier « {strip_name} » inséré depuis le modèle « {tpl_name} »."),
        Language::It => format!("Morsettiera «{strip_name}» inserita dal modello «{tpl_name}»."),
        Language::Es => format!("Bornero «{strip_name}» insertado desde plantilla «{tpl_name}»."),
        Language::Pl => format!("Wstawiono listwę „{strip_name}” z szablonu „{tpl_name}”."),
        Language::Nl => format!("Klemmenstrook \"{strip_name}\" uit sjabloon \"{tpl_name}\" ingevoegd."),
    }
}
pub fn toast_blocks_deleted(l: Language, count: usize) -> String {
    match l {
        Language::De => format!("{count} Klemme(n) gelöscht."),
        Language::En => format!("{count} terminal(s) deleted."),
        Language::Fr => format!("{count} borne(s) supprimée(s)."),
        Language::It => format!("{count} morsetto/i eliminato/i."),
        Language::Es => format!("{count} borna(s) eliminada(s)."),
        Language::Pl => format!("Usunięto {count} zacisk(ów)."),
        Language::Nl => format!("{count} klem(men) verwijderd."),
    }
}
pub fn toast_strip_created(l: Language, name: &str) -> String {
    match l {
        Language::De => format!("Klemmleiste „{name}“ angelegt — Klemmen anpassen und dann Endplatten generieren."),
        Language::En => format!("Strip \"{name}\" created — adjust terminals and then generate end plates."),
        Language::Fr => format!("Bornier « {name} » créé — ajustez les bornes puis générez les flasques."),
        Language::It => format!("Morsettiera «{name}» creata — adatta i morsetti e poi genera le piastre."),
        Language::Es => format!("Bornero «{name}» creado — ajusta las bornas y luego genera las tapas."),
        Language::Pl => format!("Utworzono listwę „{name}” — dopasuj zaciski, a następnie wygeneruj płytki."),
        Language::Nl => format!("Klemmenstrook \"{name}\" aangemaakt — pas klemmen aan en genereer eindplaten."),
    }
}
pub fn toast_strip_deleted(l: Language, name: &str) -> String {
    match l {
        Language::De => format!("Klemmleiste „{name}“ gelöscht."),
        Language::En => format!("Strip \"{name}\" deleted."),
        Language::Fr => format!("Bornier « {name} » supprimé."),
        Language::It => format!("Morsettiera «{name}» eliminata."),
        Language::Es => format!("Bornero «{name}» eliminado."),
        Language::Pl => format!("Usunięto listwę „{name}”."),
        Language::Nl => format!("Klemmenstrook \"{name}\" verwijderd."),
    }
}
pub fn toast_pdf_imported(l: Language, count: usize, total_nt: usize) -> String {
    match (l, total_nt) {
        (Language::De, 0) => format!("{count} Klemmleisten aus PDF importiert."),
        (Language::De, nt) => format!("{count} Klemmleisten aus PDF importiert ({nt}× NT-Konfiguration ausstehend)."),
        (Language::En, 0) => format!("{count} terminal strip(s) imported from PDF."),
        (Language::En, nt) => format!("{count} terminal strip(s) imported from PDF ({nt}× NT configuration pending)."),
        (Language::Fr, 0) => format!("{count} bornier(s) importé(s) du PDF."),
        (Language::Fr, nt) => format!("{count} bornier(s) importé(s) du PDF ({nt}× configuration NT requise)."),
        (Language::It, 0) => format!("{count} morsettiere importate da PDF."),
        (Language::It, nt) => format!("{count} morsettiere importate da PDF ({nt}× configurazione NT in sospeso)."),
        (Language::Es, 0) => format!("{count} bornero(s) importado(s) desde PDF."),
        (Language::Es, nt) => format!("{count} bornero(s) importado(s) desde PDF ({nt}× configuración NT pendiente)."),
        (Language::Pl, 0) => format!("Zaimportowano {count} listew z PDF."),
        (Language::Pl, nt) => format!("Zaimportowano {count} listew z PDF ({nt}× konfiguracja NT oczekuje)."),
        (Language::Nl, 0) => format!("{count} klemmenstroken uit PDF geïmporteerd."),
        (Language::Nl, nt) => format!("{count} klemmenstroken uit PDF geïmporteerd ({nt}× NT-configuratie openstaand)."),
    }
}
pub fn toast_tpl_saved(l: Language, name: &str) -> String {
    match l {
        Language::De => format!("Vorlage „{name}“ in persönlicher Bibliothek gespeichert."),
        Language::En => format!("Template \"{name}\" saved to personal library."),
        Language::Fr => format!("Modèle « {name} » enregistré dans votre bibliothèque."),
        Language::It => format!("Modello «{name}» salvato nella libreria personale."),
        Language::Es => format!("Plantilla «{name}» guardada en la biblioteca personal."),
        Language::Pl => format!("Szablon „{name}” zapisano w osobistej bibliotece."),
        Language::Nl => format!("Sjabloon \"{name}\" opgeslagen in persoonlijke bibliotheek."),
    }
}
pub const fn toast_tpl_updated(l: Language) -> &'static str {
    pick(l, ["Bibliothek-Vorlage aktualisiert.", "Library template updated.", "Modèle mis à jour.", "Modello aggiornato.", "Plantilla actualizada.", "Szablon zaktualizowany.", "Sjabloon bijgewerkt."])
}
pub const fn toast_tpl_removed(l: Language) -> &'static str {
    pick(l, ["Vorlage aus Bibliothek entfernt.", "Template removed from library.", "Modèle supprimé de la bibliothèque.", "Modello rimosso dalla libreria.", "Plantilla eliminada de la biblioteca.", "Usunięto szablon z biblioteki.", "Sjabloon verwijderd uit bibliotheek."])
}
pub const fn toast_edit_active(l: Language) -> &'static str {
    pick(l, ["Bearbeitungsmodus aktiv — Klemmen anpassen und danach Endplatten generieren.", "Editing mode active — adjust terminals and then generate end plates.", "Mode édition actif — ajustez les bornes puis générez les flasques.", "Modalità modifica attiva — adatta i morsetti e poi genera le piastre.", "Modo edición activo — ajusta las bornas y luego genera las tapas.", "Tryb edycji aktywny — dopasuj zaciski i wygeneruj płytki.", "Bewerkingsmodus actief — pas klemmen aan en genereer eindplaten."])
}
pub const fn toast_cad_active(l: Language) -> &'static str {
    pick(l, ["CAD-Ansicht aktiv.", "CAD view active.", "Vue CAO active.", "Vista CAD attiva.", "Vista CAD activa.", "Widok CAD aktywny.", "CAD-weergave actief."])
}
pub const fn toast_bridge_active(l: Language) -> &'static str {
    pick(l, ["Brücken-Modus aktiv — Klemmen-Ebenen im CAD anklicken oder überstreichen.", "Bridge mode active — click or drag over terminal levels in CAD.", "Mode Ponts actif — cliquez ou glissez sur les niveaux en CAO.", "Modalità Ponticelli attiva — clicca o trascina sui piani nel CAD.", "Modo Puentes activo — haz clic o arrastra sobre los pisos en CAD.", "Tryb mostków aktywny — kliknij lub przeciągnij po poziomach w CAD.", "Bruggen-modus actief — klik of sleep over etages in CAD."])
}
pub const fn toast_eplan_active(l: Language) -> &'static str {
    pick(l, ["ePlan 1:1 Aufbaubild aktiv — maßstabsgetreuer DXF-Export bereit.", "ePlan 1:1 layout active — true-scale DXF export ready.", "Vue ePlan 1:1 active — export DXF à l'échelle prêt.", "Vista ePlan 1:1 attiva — export DXF in scala pronto.", "Vista ePlan 1:1 activa — exportación DXF a escala lista.", "Widok ePlan 1:1 aktywny — eksport DXF w skali gotowy.", "ePlan 1:1 opbouw actief — DXF-export op ware schaal gereed."])
}
pub const fn toast_seq_by_level(l: Language) -> &'static str {
    pick(l, ["Zählfolge: Ebenenweise (E1: 1,2,3 → E2: 4,5,6).", "Order: By level (L1: 1,2,3 → L2: 4,5,6).", "Ordre : Par niveau (N1 : 1,2,3 → N2 : 4,5,6).", "Ordine: Per piano (P1: 1,2,3 → P2: 4,5,6).", "Orden: Por piso (N1: 1,2,3 → N2: 4,5,6).", "Kolejność: Wg poziomu (P1: 1,2,3 → P2: 4,5,6).", "Volgorde: Per etage (E1: 1,2,3 → E2: 4,5,6)."])
}
pub const fn toast_seq_by_block(l: Language) -> &'static str {
    pick(l, ["Zählfolge: Klemmenweise (1,2 | 3,4).", "Order: Per block (1,2 | 3,4).", "Ordre : Par borne (1,2 | 3,4).", "Ordine: Per morsetto (1,2 | 3,4).", "Orden: Por borna (1,2 | 3,4).", "Kolejność: Wg zacisku (1,2 | 3,4).", "Volgorde: Per klem (1,2 | 3,4)."])
}
pub fn toast_n_to_nt(l: Language, count: usize, need_gen: bool) -> String {
    match (l, need_gen) {
        (Language::De, false) => format!("{count} N-Klemme(n) in NT-Trennklemmen umgewandelt."),
        (Language::De, true) => format!("{count} N-Klemme(n) in NT-Trennklemmen umgewandelt — anschließend auf „Endplatten generieren“ klicken."),
        (Language::En, false) => format!("{count} N terminal(s) converted to NT disconnect blocks."),
        (Language::En, true) => format!("{count} N terminal(s) converted to NT — click \"Generate End Plates\" afterwards."),
        (Language::Fr, _) => format!("{count} borne(s) N convertie(s) en bornes NT."),
        (Language::It, _) => format!("{count} morsetti N convertiti in NT."),
        (Language::Es, _) => format!("{count} bornas N convertidas a NT."),
        (Language::Pl, _) => format!("Zamieniono {count} zacisków N na NT."),
        (Language::Nl, _) => format!("{count} N-klem(men) omgezet naar NT."),
    }
}
pub fn toast_nt_to_n(l: Language, count: usize, need_gen: bool) -> String {
    match (l, need_gen) {
        (Language::De, false) => format!("{count} NT-Trennklemme(n) in Standard-N-Klemmen zurückgewandelt."),
        (Language::De, true) => format!("{count} NT-Trennklemme(n) in Standard-N-Klemmen zurückgewandelt — anschließend auf „Endplatten generieren“ klicken."),
        (Language::En, false) => format!("{count} NT block(s) converted back to standard N terminals."),
        (Language::En, true) => format!("{count} NT block(s) converted back to N — click \"Generate End Plates\" afterwards."),
        (Language::Fr, _) => format!("{count} borne(s) NT reconvertie(s) en bornes N."),
        (Language::It, _) => format!("{count} morsetti NT riconvertiti in N."),
        (Language::Es, _) => format!("{count} bornas NT reconvertidas a N."),
        (Language::Pl, _) => format!("Przywrócono {count} zacisków NT na N."),
        (Language::Nl, _) => format!("{count} NT-klem(men) teruggezet naar N."),
    }
}
pub fn toast_blocks_inserted(l: Language, qty: usize, name: &str) -> String {
    match l {
        Language::De => format!("{qty}× {name} eingefügt."),
        Language::En => format!("Inserted {qty}× {name}."),
        Language::Fr => format!("{qty}× {name} insérée(s)."),
        Language::It => format!("Inserito {qty}× {name}."),
        Language::Es => format!("Insertado {qty}× {name}."),
        Language::Pl => format!("Wstawiono {qty}× {name}."),
        Language::Nl => format!("{qty}× {name} ingevoegd."),
    }
}
pub fn toast_group_inserted(l: Language, summary: &str) -> String {
    match l {
        Language::De => format!("Block eingefügt: {summary}"),
        Language::En => format!("Group inserted: {summary}"),
        Language::Fr => format!("Groupe inséré : {summary}"),
        Language::It => format!("Gruppo inserito: {summary}"),
        Language::Es => format!("Grupo insertado: {summary}"),
        Language::Pl => format!("Wstawiono grupę: {summary}"),
        Language::Nl => format!("Groep ingevoegd: {summary}"),
    }
}
pub const fn toast_strip_empty(l: Language) -> &'static str {
    pick(l, ["Klemmleiste enthält noch keine Klemmen.", "Terminal strip has no blocks yet.", "Le bornier ne contient pas encore de bornes.", "La morsettiera non contiene ancora morsetti.", "El bornero aún no tiene bornas.", "Listwa nie zawiera jeszcze żadnych zacisków.", "Klemmenstrook bevat nog geen klemmen."])
}
pub fn toast_endplates_nt_check(l: Language, plates: usize, strip: &str) -> String {
    match l {
        Language::De => format!("Endplatten ({plates}×) berechnet — NT-Konfiguration für „{strip}“ prüfen."),
        Language::En => format!("End plates ({plates}×) calculated — review NT configuration for \"{strip}\"."),
        Language::Fr => format!("Flasques ({plates}×) calculés — vérifiez la config. NT pour « {strip} »."),
        Language::It => format!("Piastre ({plates}×) calcolate — verificare configurazione NT per «{strip}»."),
        Language::Es => format!("Tapas ({plates}×) calculadas — revisar config. NT para «{strip}»."),
        Language::Pl => format!("Obliczono płytki ({plates}×) — sprawdź konfigurację NT dla „{strip}”."),
        Language::Nl => format!("Eindplaten ({plates}×) berekend — controleer NT-configuratie voor \"{strip}\"."),
    }
}
pub fn toast_endplates_ready(l: Language, plates: usize, strip: &str) -> String {
    match l {
        Language::De => format!("Endplatten für „{strip}“ generiert ({plates}×). Weiter: Brücken, Excel oder ePlan."),
        Language::En => format!("End plates generated for \"{strip}\" ({plates}×). Next: Bridges, Excel or ePlan."),
        Language::Fr => format!("Flasques générés pour « {strip} » ({plates}×). Suite : Ponts, Excel ou ePlan."),
        Language::It => format!("Piastre generate per «{strip}» ({plates}×). Poi: Ponticelli, Excel o ePlan."),
        Language::Es => format!("Tapas generadas para «{strip}» ({plates}×). Siguiente: Puentes, Excel o ePlan."),
        Language::Pl => format!("Wygenerowano płytki dla „{strip}” ({plates}×). Dalej: Mostki, Excel lub ePlan."),
        Language::Nl => format!("Eindplaten voor \"{strip}\" gegenereerd ({plates}×). Vervolg: Bruggen, Excel of ePlan."),
    }
}
pub fn toast_endplates_and_nt_applied(l: Language, plates: usize, strip: &str) -> String {
    match l {
        Language::De => format!("Endplatten ({plates}×) & NT-Zubehör für „{strip}“ übernommen."),
        Language::En => format!("End plates ({plates}×) & NT accessories applied for \"{strip}\"."),
        Language::Fr => format!("Flasques ({plates}×) et accessoires NT appliqués pour « {strip} »."),
        Language::It => format!("Piastre ({plates}×) e accessori NT applicati per «{strip}»."),
        Language::Es => format!("Tapas ({plates}×) y accesorios NT aplicados para «{strip}»."),
        Language::Pl => format!("Zastosowano płytki ({plates}×) i akcesoria NT dla „{strip}”."),
        Language::Nl => format!("Eindplaten ({plates}×) en NT-toebehoren voor \"{strip}\" toegepast."),
    }
}
pub fn toast_blocks_changed(l: Language, count: usize, spec_name: &str) -> String {
    match l {
        Language::De => format!("{count}× Klemme auf {spec_name} geändert."),
        Language::En => format!("Changed {count}× terminal(s) to {spec_name}."),
        Language::Fr => format!("{count}× borne(s) remplacée(s) par {spec_name}."),
        Language::It => format!("{count}× morsetto/i cambiato/i in {spec_name}."),
        Language::Es => format!("{count}× borna(s) cambiada(s) a {spec_name}."),
        Language::Pl => format!("Zmieniono {count}× zacisk(ów) na {spec_name}."),
        Language::Nl => format!("{count}× klem(men) gewijzigd naar {spec_name}."),
    }
}
pub fn toast_potential_added(l: Language, pname: &str) -> String {
    match l {
        Language::De => format!("Potential „{pname}“ hinzugefügt."),
        Language::En => format!("Potential \"{pname}\" added."),
        Language::Fr => format!("Potentiel « {pname} » ajouté."),
        Language::It => format!("Potenziale «{pname}» aggiunto."),
        Language::Es => format!("Potencial «{pname}» añadido."),
        Language::Pl => format!("Dodano potencjał „{pname}”."),
        Language::Nl => format!("Potentiaal \"{pname}\" toegevoegd."),
    }
}
