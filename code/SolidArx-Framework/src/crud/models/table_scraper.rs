use std::fs;
use syn::{self, Item, ItemStruct, ItemMod, parse_file};
use std::collections::HashMap;
use crate::config::network_config::DatabaseType ;
// Importa il modulo per  per ottenere la rappresentazione del tipo come una stringa
use quote::ToTokens; 

/// Funzione che esegue il parsing di un modulo Rust e restituisce le struct trovate all'interno.
/// 
/// # Argomenti
/// * `item_mod` - Il modulo da analizzare.
/// 
/// # Ritorna
/// Un `Result<Vec<ItemStruct>, String>` che contiene una lista di struct trovate nel modulo,
/// oppure un messaggio di errore in caso di fallimento del parsing.
/// 
/// # Note
/// Lo scraping dei file `.rs` viene fatto in maniera selettiva grazie alle feature implementate nel codice dei modelli
/// Se la struct non è stata attivata dalla feature corretta, non verrà inclusa nella mappa risultante.
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
fn map_to_sql(type_name: &str) -> &str {
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

/// Funzione che mappa i tipi Rust a tipi MongoDB corrispondenti.
/// 
/// # Argomenti
/// * `type_name` - Il nome del tipo Rust da mappare.
/// 
/// # Ritorna
/// Una stringa che rappresenta il tipo MongoDB corrispondente.
/// 
/// # Note
/// - MongoDB non ha vincoli di tipo rigidi, quindi mappiamo i tipi Rust a tipi generici.
fn map_to_mongo(type_name: &str) -> &str {
    match type_name {
        // Tipi numerici
        "u32" | "i32" => "int",
        "u64" | "i64" => "long",
        "String" => "string",
        "bool" => "bool",
        "f32" | "f64" => "double",
        
        // Tipi opzionali (nullabili)
        "Option<u32>" | "Option<i32>" => "int",
        "Option<String>" => "string",
        
        // Tipi personalizzati o complessi
        "AllocType" => "string",
        "CrudOperations" => "object",
        "Box<[u8]>" => "binData",
        "ExeLogStatus" | "MacroStatus" | "ProjectStatus" => "string",
        "ExecutionFrequency" => "string",
        "Option<ProjectMetadata>" => "object",
        "chrono::NaiveDateTime" => "date",
        _ => "string",
    }
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
    let syntax = parse_file(&file_content)
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
/// 
/// # Note
/// Lo scraping dei file `.rs` viene fatto in maniera selettiva grazie alle feature implementate nel codice dei modelli
/// Se la struct non è stata attivata dalla feature corretta, non verrà inclusa nella mappa risultante.
pub fn scrape(directory: &str, db_type: DatabaseType) -> Result<Vec<HashMap<String, HashMap<String, String>>>, String> {
    let files = read_rs_dir(directory)?;
    let mut all_structs = Vec::new();

    for file_path in files {
        println!("Elaborando il file: {}", file_path);
        let structs = parse_rs_file(&file_path)?;

        if structs.is_empty() {
            println!("Nessuna struct trovata nel file: {}", file_path);
        }

        let mut struct_map = HashMap::new();
        
        for item in structs.iter() {
            let struct_name = item.ident.to_string();
            let struct_name_ref: String = struct_name.clone(); // Uso di String

            let fields = item.fields.iter().collect::<Vec<_>>();
            let mut fields_map = HashMap::new();
            for field in fields {
                let field_name = field
                    .ident
                    .as_ref()
                    .map(|f| f.to_string()) // Convertito in String
                    .unwrap_or("Unnamed".to_string());

                let field_type_str = field.ty.to_token_stream().to_string();
                let field_type = match db_type {
                    DatabaseType::PostgreSQL(_) => map_to_sql(&field_type_str),
                    DatabaseType::SQLite(_) => map_to_sql(&field_type_str),
                    DatabaseType::MongoDB(_) => map_to_mongo(&field_type_str),
                    DatabaseType::None => panic!("Operazione impossibile, Database non configurato"),
                };

                let field_type_clone = field_type.to_string(); // Uso di String per evitare riferimenti
                let field_name_ref: String = field_name.clone(); // Converte in String

                fields_map.insert(field_name_ref, field_type_clone);
            }

            struct_map.insert(struct_name_ref, fields_map);
        }

        all_structs.push(struct_map);
    }

    Ok(all_structs)
}
