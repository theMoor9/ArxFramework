use std::fs::{self, DirEntry};
use std::path::Path;
use syn::{self, Item, ItemStruct, ItemMod};
use std::collections::HashMap;

/// Funzione che esegue il parsing di un modulo Rust e restituisce le struct trovate all'interno.
/// 
/// # Argomenti
/// * `item_mod` - Il modulo da analizzare.
/// 
/// # Ritorna
/// Un `Result<Vec<ItemStruct>, String>` che contiene una lista di struct trovate nel modulo,
/// oppure un messaggio di errore in caso di fallimento del parsing.
fn parse_mod_items(item_mod: &ItemMod) -> Result<Vec<ItemStruct>, String> {
    let mut structs = Vec::new();

    // Verifica se il modulo ha contenuti (ad esempio, file separati o item inclusi)
    if let Some(module_path) = &item_mod.content {
        for mod_item in module_path.1.clone() {
            match mod_item {
                Item::Struct(item_struct) => {
                    structs.push(item_struct); // Aggiungi la struct trovata
                }
                Item::Mod(item_submod) => {
                    // Se c'è un sottogruppo di moduli, esplora anche quello
                    structs.extend(parse_mod_items(&item_submod)?);
                }
                _ => {} // Ignora altri tipi di item
            }
        }
    }

    Ok(structs)
}

/// Funzione che mappa i tipi Rust a tipi SQL corrispondenti.
/// 
/// # Argomenti
/// * `type_name` - Il nome del tipo Rust da mappare.
/// 
/// # Ritorna
/// Una stringa che rappresenta il tipo SQL corrispondente.
fn map_rust_type_to_sql(type_name: &str) -> &str {
    match type_name {
        // Tipi numerici
        "u32" | "i32" => "INTEGER",
        "u64" | "i64" => "BIGINT",
        "String" => "TEXT",
        "bool" => "BOOLEAN",
        "f32" => "REAL",
        "f64" => "DOUBLE PRECISION",
        
        // Tipi opzionali (nullabili)
        "Option<u32>" | "Option<i32>" => "INTEGER NULL",
        "Option<String>" => "TEXT NULL",
        
        // Tipi personalizzati o complessi
        "AllocType" => "TEXT", // Rappresentabile come stringa
        "CrudOperations" => "JSON", // JSON per flessibilità
        "Box<[u8]>" => "BYTEA", // Tipico per dati binari
        "ExeLogStatus" | "MacroStatus" | "ProjectStatus" => "TEXT CHECK (value IN ('Active', 'Disabled', 'Completed'))", // Enum con vincoli
        "ExecutionFrequency" => "TEXT", // Frequenze come stringhe
        "Option<ProjectMetadata>" => "JSON NULL", // Serializzato come JSON
        "chrono::NaiveDateTime" => "TIMESTAMP", // Data e ora
        _ => "TEXT", // Default per tipi sconosciuti
    }
}

/// Funzione che genera un messaggio per ogni struct trovata in un file `.rs`.
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
        let field_type = map_rust_type_to_sql(&field.ty);
        message.push_str(&format!("{}\n{}\n", field_name, field_type));
    }

    Ok(message)
}

/// Funzione che esegue il parsing di un file `.rs` e restituisce tutte le struct presenti al suo interno.
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
                structs.extend(parse_mod_items(&item_mod)?); // Estende la lista con le struct nei moduli
            }
            Item::Struct(item_struct) => {
                structs.push(item_struct); // Aggiungi direttamente la struct se non è incapsulata in un modulo
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

/// Funzione che esegue lo scraping dei file `.rs` in una cartella e raccoglie le struct e i loro campi.
/// Ritorna una lista di mappe contenenti il nome delle struct e i rispettivi campi con i loro tipi SQL.
/// 
/// # Argomenti
/// * `directory` - La cartella contenente i file `.rs` da scansionare e analizzare.
/// 
/// # Ritorna
/// Un `Result<Vec<HashMap<String, HashMap<String, String>>>, String>` che contiene
/// una lista di strutture mappate (nome struct -> campi -> tipi SQL), oppure un messaggio di errore.
pub fn scrape(directory: &str) -> Result<Vec<HashMap<String, HashMap<String, String>>>, String> {
    // Ottieni tutti i file `.rs` dalla cartella specificata
    let files = read_rs_dir(directory)?;

    // Vettore per raccogliere le strutture
    let mut all_structs = Vec::new();

    // Itera su ciascun file trovato
    for file_path in files {
        println!("Elaborando il file: {}", file_path);
        
        // Parsea ciascun file per ottenere le struct presenti
        let structs = parse_rs_file(&file_path)?;

        if structs.is_empty() {
            println!("Nessuna struct trovata nel file: {}", file_path);
        }

        // Per ogni struct trovata, genera una mappa con il nome della struct e i campi
        let mut struct_map = HashMap::new();
        
        for item in structs.iter() {
            let struct_name = item.ident.to_string();  // Nome della struct
            let fields = item.fields.iter().collect::<Vec<_>>();  // Campi della struct

            // Crea una mappa per i campi della struct
            let mut fields_map = HashMap::new();
            for field in fields {
                let field_name = field.ident.as_ref().map(|f| f.to_string()).unwrap_or_else(|| "Unnamed".to_string());
                let field_type = map_rust_type_to_sql(&field.ty);  // Mappa il tipo Rust al tipo SQL
                fields_map.insert(field_name, field_type);
            }

            // Inserisce la struct con i relativi campi nella mappa
            struct_map.insert(struct_name, fields_map);
        }

        // Aggiunge struct_map al vettore all_structs
        all_structs.push(struct_map);
    }

    Ok(all_structs)
}
