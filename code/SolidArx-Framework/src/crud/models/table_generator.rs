use crate::crud::models::table_scraper::{scrape};
use diesel::{PgConnection, SqliteConnection};
use mongodb::{Client, bson::{doc, Bson}};
use std::collections::HashMap;
use log::{info};


/// Crea una tabella in PostgreSQL se non esiste già.
///
/// # Parametri
/// - `connection`: Connessione a PostgreSQL.
/// - `table_name`: Nome della tabella da creare.
/// - `fields`: Mappa dei campi della tabella con i relativi tipi.
fn create_postgresql_table(
    connection: &PgConnection, 
    table_name: &str, 
    fields: &HashMap<String, String>
) -> Result<(), diesel::result::Error> {
    // Costruisce la query SQL per la creazione della tabella
    let mut query = format!("CREATE TABLE IF NOT EXISTS {} (", table_name);
    
    // Aggiunge i campi alla query
    for (field, field_type) in fields.iter() {
        query.push_str(&format!("{} {},", field, field_type));
    }

    // Rimuove l'ultima virgola e aggiunge la parentesi finale
    query.pop();
    query.push_str(");");

    // Esegue la query per creare la tabella
    diesel::sql_query(query).execute(connection)?;

    Ok(())
}

/// Crea una tabella in SQLite se non esiste già.
///
/// # Parametri
/// - `connection`: Connessione a SQLite.
/// - `table_name`: Nome della tabella da creare.
/// - `fields`: Mappa dei campi della tabella con i relativi tipi.
fn create_sqlite_table(
    connection: &SqliteConnection, 
    table_name: &str, 
    fields: &HashMap<String, String>
) -> Result<(), diesel::result::Error> {
    // Costruisce la query SQL per la creazione della tabella
    let mut query = format!("CREATE TABLE IF NOT EXISTS {} (", table_name);
    
    // Aggiunge i campi alla query
    for (field, field_type) in fields.iter() {
        query.push_str(&format!("{} {},", field, field_type));
    }

    // Rimuove l'ultima virgola e aggiunge la parentesi finale
    query.pop();
    query.push_str(");");

    // Esegue la query per creare la tabella
    diesel::sql_query(query).execute(connection)?;

    Ok(())
}

/// Crea una collezione in MongoDB, aggiungendo un documento di esempio.
///
/// # Parametri
/// - `client`: Client MongoDB per eseguire le operazioni.
/// - `database_name`: Nome del database MongoDB.
/// - `collection_name`: Nome della collezione da creare.
/// - `fields`: Mappa dei campi della collezione con i relativi tipi.
async fn create_mongodb_table(
    client: &Client, 
    database_name: &str, 
    collection_name: &str, 
    fields: &HashMap<String, String>
) -> Result<(), Box<dyn std::error::Error>> {
    // Ottiene il database e la collezione
    let db = client.database(database_name);
    let collection = db.collection(collection_name);

    // Crea un documento di esempio da inserire nella collezione
    let document = fields.iter().map(|(field, field_type)| {
        // Tratta ogni tipo come una stringa per ora
        (field.clone(), Bson::String(field_type.clone()))
    }).collect::<Vec<_>>();

    // Inserisce un documento di esempio nella collezione
    collection.insert_one(doc! { "example": document }, None).await?;

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
    structs: Vec<HashMap<String, HashMap<String, String>>>, 
    connection_manager: &ConnectionManager
) -> Result<(), Box<dyn std::error::Error>> {
    // Ottiene la connessione al database tramite il connection manager
    let connection = connection_manager.connect().await?;

    // Gestisce la connessione al database e crea le tabelle in base al tipo di DB
    match connection {
        DbConnection::Postgres(pg_conn) => {
            for struct_info in structs {
                // Estrae il nome della tabella dalla mappa
                let table_name = struct_info.get("name").unwrap();
                let fields = struct_info.clone();
                // Crea la tabella in PostgreSQL
                create_postgresql_table(&pg_conn, table_name, &fields)?;
                info!("Tabella {} creata su PostgreSQL", table_name);
            }
        }
        DbConnection::SQLite(sqlite_conn) => {
            for struct_info in structs {
                let table_name = struct_info.get("name").unwrap();
                let fields = struct_info.clone();
                // Crea la tabella in SQLite
                create_sqlite_table(&sqlite_conn, table_name, &fields)?;
                info!("Tabella {} creata su SQLite", table_name);
            }
        }
        DbConnection::MongoDB(mongo_client) => {
            for struct_info in structs {
                let collection_name = struct_info.get("name").unwrap();
                let fields = struct_info.clone();
                // Crea la collezione in MongoDB
                create_mongodb_table(&mongo_client, "my_db", collection_name, &fields).await?;
                info!("Collezione {} creata su MongoDB", collection_name);
            }
        }
    }

    Ok(())
}
