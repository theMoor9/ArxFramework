use crate::network::connection_management::{ConnectionManager, DbConnection};
use diesel::{
    PgConnection, 
    sqlite::SqliteConnection,
    RunQueryDsl,
};
use mongodb::{Client, bson::{doc, Bson}};
use std::collections::HashMap;
use log::{info};

// Definisce un errore personalizzato per la generazione delle tabelle
enum TableGeneratorError {
    DieselError(diesel::result::Error),
    MongoError(mongodb::error::Error),
    UnknownError(String),
}
impl From<diesel::result::Error> for TableGeneratorError {
    fn from(err: diesel::result::Error) -> Self {
        TableGeneratorError::DieselError(err)
    }
}
impl From<mongodb::error::Error> for TableGeneratorError {
    fn from(err: mongodb::error::Error) -> Self {
        TableGeneratorError::MongoError(err)
    }
}

/// Crea una tabella nel database se non esiste già.
/// 
/// # Parametri
/// - `connection`: Funzione per eseguire la query di creazione della tabella.
/// - `table_name`: Nome della tabella da creare.
/// - `fields`: Mappa dei campi della tabella con i relativi tipi.
/// 
/// # Ritorna
/// Un risultato che indica se la creazione della tabella è andata a buon fine o meno.
fn create_table<F>(
    connection: F, 
    table_name: &str, 
    fields: &HashMap<&str, &str>
) -> Result<(), TableGeneratorError>
where
    F: FnOnce(&str) -> Result<usize, diesel::result::Error>,
{
    let mut query = format!("CREATE TABLE IF NOT EXISTS {} (", table_name);
    for (field, field_type) in fields.iter() {
        query.push_str(&format!("{} {},", field, field_type));
    }
    query.pop();
    query.push_str(");");

    connection(&query).map_err(TableGeneratorError::from)?;
    Ok(())
}


/// Crea una tabella in PostgreSQL se non esiste già.
///
/// # Parametri
/// - `connection`: Connessione a PostgreSQL.
/// - `table_name`: Nome della tabella da creare.
/// - `fields`: Mappa dei campi della tabella con i relativi tipi.
fn create_postgresql_table(
    connection: &mut PgConnection, 
    table_name: &str, 
    fields: &HashMap<&str, &str>
) -> Result<(), TableGeneratorError> {
    create_table(
        |query| diesel::sql_query(query).execute(connection), 
        table_name, 
        fields
    );
    info!("Tablella PostgreSQL {} creata", table_name);
    Ok(())
}

/// Crea una tabella in SQLite se non esiste già.
///
/// # Parametri
/// - `connection`: Connessione a SQLite.
/// - `table_name`: Nome della tabella da creare.
/// - `fields`: Mappa dei campi della tabella con i relativi tipi.
fn create_sqlite_table(
    connection: &mut SqliteConnection, 
    table_name: &str, 
    fields: &HashMap<&str, &str>
) -> Result<(), TableGeneratorError> {
    create_table(
        |query| diesel::sql_query(query).execute(connection), 
        table_name, 
        fields
    );
    info!("Tablella SQLite {} creata", table_name);
    Ok(())
}

/// Mappa il tipo Rust in una rappresentazione BSON di MongoDB
/// 
/// # Parametri
/// - `type_name`: Nome del tipo Rust da mappare.
/// 
/// # Ritorna
/// Un valore BSON corrispondente al tipo Rust fornito.
/// 
/// # Note
/// Si usa `Bson::String(String::new())` per strutture ed enum generalmente semplici per ridurre la complessità e migliorare le prestazioni. 
/// Questa scelta evita overhead di memoria e semplifica la serializzazione/deserializzazione, mantenendo la flessibilità per future modifiche.
/// Quando necessario, i dati possono essere facilmente mappati a strutture più complesse tramite Serde.

fn map_to_bson(type_name: &str) -> Bson {
    match type_name {
        "u32" | "i32" => Bson::Int32(0), // Usa un valore di esempio (può essere modificato)
        "u64" | "i64" => Bson::Int64(0),
        "String" => Bson::String(String::new()),
        "bool" => Bson::Boolean(false),
        "f32" | "f64" => Bson::Double(0.0),
        
        // Tipi opzionali (nullabili)
        "Option<u32>" | "Option<i32>" => Bson::String(String::new()),
        "Option<String>" => Bson::String(String::new()),
        
        // Tipi personalizzati o complessi
        "AllocType" => Bson::String(String::new()),
        "CrudOperations" => Bson::String(String::new()),
        "Box<[u8]>" =>  Bson::String(String::new()),
        "ExeLogStatus" | "MacroStatus" | "ProjectStatus" => Bson::String(String::new()),
        "ExecutionFrequency" => Bson::String(String::new()),
        "Option<ProjectMetadata>" => Bson::String(String::new()),
        "chrono::NaiveDateTime" => Bson::String(String::new()),
        _ => Bson::String(String::new()), // Default per tipi sconosciuti
    }
}

/// Crea una collezione in MongoDB, aggiungendo un documento di esempio.
///
/// # Parametri
/// - `client`: Client MongoDB per eseguire le operazioni.
/// - `database_name`: Nome del database MongoDB.
/// - `collection_name`: Nome della collezione da creare.
/// - `fields`: Mappa dei campi della collezione con i relativi tipi.
/// 
/// # Ritorna
/// Un risultato che indica se la creazione della collezione è andata a buon fine o meno.
async fn create_mongodb_table(
    client: &Client, 
    database_name: &str, 
    collection_name: &str, 
    fields: &HashMap<&str, &str>
) -> Result<(), TableGeneratorError> {
    // Ottiene il database e la collezione
    let db = client.database(database_name);
    let collection = db.collection(collection_name);

    // Crea un documento di esempio dai campi forniti
    let document = fields.iter().map(|(field, field_type)| {
        // Tratta ogni tipo come stringa per ora
        (field, map_to_bson(field_type))
    }).collect::<Vec<_>>();

    // Inserisce un documento di esempio nella collezione
    collection
        .insert_one(doc! { "example": document })
        .await
        .map_err(TableGeneratorError::from)?; // Conversione dell'errore MongoDB
    info!("Collezione MongoDB {} creata", collection_name);
    Ok(())
}


/// Funzione principale per generare le tabelle nel database selezionato.
///
/// # Parametri
/// - `structs`: Vettore di hash map contenenti i dettagli delle tabelle da creare.
/// - `connection_manager`: Oggetto per gestire la connessione al database.
///
/// # Ritorna
/// Un risultato che indica se la generazione delle tabelle è andata a buon fine o meno.
pub async fn generate_tables(
    structs: Vec<HashMap<&str, HashMap<&str, &str>>>, 
    connection_manager: &ConnectionManager
) -> Result<(), TableGeneratorError> {
    // Ottiene la connessione al database tramite il connection manager
    let mut connection = connection_manager.connect().await?;

    // Gestisce la connessione al database e crea le tabelle in base al tipo di DB
    match connection {
        DbConnection::Postgres(pg_conn) => {
            for struct_info in structs {
                // Estrae il nome della tabella dalla mappa
                let table_name = struct_info.get("name");
                let table_name_str = match table_name {
                    Some(table) => *table.get("name").unwrap_or(&"default_table"),  // Trova "name" nel HashMap
                    None => "default_table",  // Se table_name è None, usa "default_table"
                };
                // Estrai i campi dalla mappa
                let fields = struct_info
                    .get("fields") // Trova il sotto-HashMap corrispondente a "fields"
                    .cloned() // Clona per ottenere una copia
                    .unwrap_or_else(HashMap::new); // Usa un HashMap vuoto se non esist
                // Crea la tabella in PostgreSQL
                create_postgresql_table(&mut pg_conn, table_name_str, &fields)?;
                info!("Tabella {:?} creata su PostgreSQL", table_name_str);
            }
        }
        DbConnection::SQLite(sqlite_conn) => {
            for struct_info in structs {
                let table_name = struct_info.get("name");
                let table_name_str = match table_name {
                    Some(table) => *table.get("name").unwrap_or(&"default_table"),  // Trova "name" nel HashMap
                    None => "default_table",  // Se table_name è None, usa "default_table"
                };
                // Estrai i campi dalla mappa
                let fields = struct_info
                    .get("fields") // Trova il sotto-HashMap corrispondente a "fields"
                    .cloned() // Clona per ottenere una copia
                    .unwrap_or_else(HashMap::new); // Usa un HashMap vuoto se non esist
                // Crea la tabella in SQLite
                create_sqlite_table(&mut sqlite_conn, table_name_str, &fields)?;
                info!("Tabella {:?} creata su SQLite", table_name_str);
            }
        }
        DbConnection::MongoDB(mongo_client) => {
            for struct_info in structs {
                let collection_name = struct_info.get("name");
                let collection_name_str = match collection_name {
                    Some(collection) => *collection.get("name").unwrap_or(&"default_table"),
                    None => "default_table",
                };
                // Estrai i campi dalla mappa
                let fields = struct_info
                    .get("fields") // Trova il sotto-HashMap corrispondente a "fields"
                    .cloned() // Clona per ottenere una copia
                    .unwrap_or_else(HashMap::new); // Usa un HashMap vuoto se non esist
                // Crea la collezione in MongoDB
                create_mongodb_table(&mut mongo_client, "models", collection_name_str, &fields).await?;
                info!("Collezione {} creata su MongoDB", collection_name_str);
            }
        }
    }
    info!("Tabelle create con successo");
    Ok(())
}
