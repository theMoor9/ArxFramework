use std::fs::{self, DirEntry};
use std::path::Path;
use syn::{self, Item, ItemStruct, ItemMod};
use std::collections::HashMap;



/// Funzione per fare il parsing di un modulo .
/// Restituisce una lista di struct presenti nel modulo.
/// 
/// # Argomenti
/// * `item_mod` - Il modulo da analizzare.
/// 
/// # Ritorna
/// Un `Result<Vec<ItemStruct>, String>` che contiene una lista di struct trovate nel modulo,
fn parse_mod_items(item_mod: &ItemMod) -> Result<Vec<ItemStruct>, String> {
    let mut structs = Vec::new();

    // Verifica se il modulo ha un file associato (se è un file separato)
    if let Some(module_path) = &item_mod.content {
        for mod_item in module_path.1.clone() {
            match mod_item {
                Item::Struct(item_struct) => {
                    structs.push(item_struct); // Aggiungi la struct trovata
                }
                Item::Mod(item_submod) => {
                    // Se c'è un altro modulo, esplora anche quello
                    structs.extend(parse_mod_items(&item_submod)?);
                }
                _ => {} // Ignora altri tipi di item
            }
        }
    }

    Ok(structs)
}

/// Funzione che genera un messaggio per ogni struct trovata in un file `.rs`.
/// Restituisce un messaggio informativo per testare lo script.
/// 
/// # Argomenti
/// * `struct_name` - Il nome della struct da analizzare.
/// * `fields` - I campi della struct da includere nel messaggio.
/// 
/// # Ritorna
/// Una stringa contenente un messaggio informativo per ciascuna struct.
fn generate_struct_message(struct_name: &str, fields: &[&syn::Field]) -> Result<String, String> {
    let mut message = format!("{}\n", struct_name);

    // Per ogni campo della struct, aggiungi una descrizione
    for field in fields {
        let field_name = field.ident.as_ref().map(|f| f.to_string()).unwrap_or_else(|| "Unnamed".to_string());
        let field_type = match &field.ty {
            syn::Type::Path(p) if p.path.is_ident("String") => "String",
            syn::Type::Path(p) if p.path.is_ident("i32") => "i32",
            syn::Type::Path(p) if p.path.is_ident("f64") => "f64",
            _ => "Custom", // Default tipo
        };
        message.push_str(&format!("{}:{}\n", field_name, field_type));
    }

    Ok(message)
}

/// Funzione per fare il parsing di un file `.rs` e restituire tutte le struct presenti.
/// 
/// # Argomenti
/// * `file_path` - Il percorso del file `.rs` da analizzare.
/// 
/// # Ritorna
/// Un `Result<Vec<ItemStruct>, String>` che contiene una lista di struct trovate nel file,
/// oppure un messaggio di errore se il parsing fallisce.
fn parse_rs_file(file_path: &str) -> Result<Vec<ItemStruct>, String> {
    // Legge il contenuto del file
    let file_content = fs::read_to_string(file_path)
        .map_err(|e| format!("Errore nella lettura del file: {}", e))?;
    
    // Parsea il contenuto del file usando la libreria `syn` per ottenere la sintassi AST
    let syntax = syn::parse_file(&file_content)
        .map_err(|e| format!("Errore nel parsing del file: {}", e))?;
    
    // Filtra gli item del file per ottenere solo quelli di tipo struct
    let mut structs = Vec::new();
    for item in syntax.items {
        match item {
            Item::Mod(item_mod) => {
                structs.extend(parse_mod_items(&item_mod)?); // Estendi la lista con le struct nei moduli
            }
            Item::Struct(item_struct) => {
                structs.push(item_struct); // Aggiungi direttamente la struct in caso non sia incapsulata in un modulo
            }
            _ => {} // Ignora altri tipi di item
        }
    }

    Ok(structs)
}

/// Funzione per leggere tutti i file `.rs` all'interno di una cartella.
/// 
/// # Argomenti
/// * `directory` - La cartella contenente i file `.rs` da scansionare.
/// 
/// # Ritorna
/// Un `Result<Vec<String>, String>` che contiene una lista di percorsi di file `.rs`
/// se l'operazione ha successo, oppure un messaggio di errore se si verifica un problema.
fn read_rs_dir(directory: &str) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    
    // Legge la directory e gestisce eventuali errori
    let entries = fs::read_dir(directory)
        .map_err(|e| format!("Errore nella lettura della cartella: {}", e))?;
    
    // Itera su ciascun file nella directory
    for entry in entries {
        let entry = entry.map_err(|e| format!("Errore nell'elenco dei file: {}", e))?;
        
        // Se il file ha estensione `.rs`, aggiungilo alla lista
        if entry.path().extension().and_then(|e| e.to_str()) == Some("rs") {
            files.push(entry.path().to_string_lossy().to_string());
        }
    }

    Ok(files)
}

/// Funzione che esegue il processo di scraping per tutti i file nella cartella.
/// Ritorna dei messaggi di debug invece di generare SQL o strutture MongoDB.
/// 
/// # Argomenti
/// * `directory` - La cartella contenente i file `.rs` da scansionare e analizzare.
/// 
/// # Ritorna
/// Un `Result<(), String>` che indica se il processo è stato completato correttamente o se si è verificato un errore.
pub fn scrape(directory: &str) -> Result<(), String> {
    // Ottieni tutti i file .rs dalla cartella specificata
    let files = read_rs_dir(directory)?;

    // Itera su ciascun file trovato
    for file_path in files {
        println!("Elaborando il file: {}", file_path);
        // Parsea ciascun file per ottenere le struct presenti
        let structs = parse_rs_file(&file_path)?;

        if structs.is_empty() {
            println!("Nessuna struct trovata nel file: {}", file_path);
        }
        // Per ogni struct trovata, genera un messaggio di debug
        for item in structs.iter() {
            let struct_name = item.ident.to_string();  // Nome della struct
            let fields = item.fields.iter().collect::<Vec<_>>();  // Campi della struct
            
            // Genera un messaggio con informazioni sulla struct
            let message = generate_struct_message(&struct_name, &fields);
            match message {
                Ok(msg) => println!("{}", msg),
                Err(err) => eprintln!("Errore nella generazione del messaggio: {}", err),
            }
        }
    }

    Ok(())
}

