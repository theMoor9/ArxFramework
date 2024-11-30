/// Il modulo `ConnectionManager` gestisce la connessione a vari database in base alla configurazione specificata.
/// Supporta PostgreSQL, SQLite e MongoDB, con funzionalità di retry per tentativi di connessione falliti.

use mongodb::{Client, options::ClientOptions}; 
use diesel::sqlite::SqliteConnection;
use diesel::pg::PgConnection;
use log::{error, info};
use std::time::Duration;
use tokio::time::sleep;
use async_trait::async_trait;

use crate::config::network_config::{ConnectionConfig, DatabaseType};

/// Enum per rappresentare errori di connessione al database
#[derive(Debug)]
enum ConnectionErrors {
    Postgres(String),
    SQLite(String),
    Mongo(String),
    Init(String),
    UnknownError(String),
}

impl std::fmt::Display for ConnectionErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionErrors::Postgres(msg) => write!(f, "Errore PostgreSQL: {}", msg),
            ConnectionErrors::SQLite(msg) => write!(f, "Errore SQLite: {}", msg),
            ConnectionErrors::Mongo(msg) => write!(f, "Errore MongoDB: {}", msg),
            ConnectionErrors::Init(msg) => write!(f, "Errore inizializzazione: {}", msg),
            ConnectionErrors::UnknownError(msg) => write!(f, "Errore sconosciuto: {}", msg),
        }
    }
}

impl std::error::Error for ConnectionErrors {}

pub enum DbConnection {
    Postgres(PgConnection),
    SQLite(SqliteConnection),
    MongoDB(Client),
}

/// Struttura `ConnectionManager`
/// 
/// La struttura principale per la gestione delle connessioni.
/// Contiene un campo `database` che memorizza i dettagli della configurazione del database.
#[derive(Debug,Clone)]
pub struct ConnectionManager {
    database: DatabaseType,
}

#[async_trait]
pub trait DatabaseConnection {
    fn new(db: DatabaseType) -> Self;
    fn initialize_connection(&self) -> Result<PgConnection, diesel::ConnectionError>;
    fn connect(&self) -> Result<DbConnection, ConnectionErrors>;
}

#[async_trait]
impl DatabaseConnection for ConnectionManager {
    /// Crea una nuova istanza di `ConnectionManager` con la configurazione fornita.
    ///
    /// # Parametri
    /// - `database`: Configurazione della connessione di tipo `ConnectionConfig`.
    ///
    /// # Ritorna
    /// Una nuova istanza di `ConnectionManager`
    fn new(db: DatabaseType) -> Self {
        match db {
            DatabaseType::None => {
                error!("Database non configurato.");
                panic!("Database non configurato.");
            }
            _ => {
                Self { 
                    database: db,
                }
            }
        }           
    }

    /// Inizializza la connessione al database con un meccanismo di retry per i tentativi falliti.
    ///
    /// Questa funzione tenta di stabilire una connessione al database configurato,
    /// riprovando in caso di fallimento fino al numero massimo di tentativi definiti.
    ///
    /// # Ritorna
    /// - `Ok(DbConnection)`: Connessione stabilita con successo.
    /// - `Err(diesel::ConnectionError)`: Errore se il massimo numero di tentativi è superato.
    async fn initialize_connection(&self) -> Result<PgConnection, ConnectionErrors> {
        let mut attempts = 0;
        
        loop {
            // Tenta di stabilire la connessione
            match self.connect() {
                Ok(connection) => {
                    info!("Connessione stabile.");
                    return Ok(connection);
                },
                Err(e) => {
                    attempts += 1;
                    error!("Tentativo {} fallito: {}", attempts, e);

                    /* 
                    Controlla se il numero di tentativi è superiore al massimo
                    estrapolando tramite un match il contenuto dell enum DatabaseType in self.database
                    */
                    match &self.database {
                        DatabaseType::PostgreSQL(config) 
                        | DatabaseType::SQLite(config) 
                        | DatabaseType::MongoDB(config) => {
                            if attempts >= config.retry_attempts {
                                error!("Superato il numero massimo di tentativi di connessione.");
                                return ConnectionErrors::Init(e.to_string());
                            }
                        }
                        DatabaseType::None => {
                            error!("Nessun database configurato.");
                            return ConnectionErrors::Init(e.to_string());
                        }
                    }

                    /* 
                    Attende per il periodo definito in `connection_timeout` con backoff esponenziale
                    Stessa meccanica di estrapolazione del contenuto dell'enum DatabaseType in self.database
                    */
                    let backoff = match &self.database{
                        DatabaseType::PostgreSQL(config) 
                        | DatabaseType::SQLite(config) 
                        | DatabaseType::MongoDB(config) => config.connection_timeout.unwrap().as_secs() * (attempts as u64),
                        DatabaseType::None => {
                            error!("Nessun database configurato.");
                            return ConnectionErrors::Init(e.to_string());
                        }
                    };
                    info!("Ritenterò tra {} secondi...", backoff);
                    sleep(Duration::from_secs(backoff)).await;
                }
            }
        }
    }
    

    /// Funzione per stabilire una connessione al database, in base al tipo configurato.
    ///
    /// Utilizza un `match` sul tipo di database per determinare il meccanismo di connessione appropriato.
    ///
    /// # Ritorna
    /// - `Ok(())`: Se la connessione è stabilita con successo.
    /// - `Err(ConnectionErrors)`: Se si verifica un errore durante il tentativo di connessione.
    async fn connect(&self) -> Result<DbConnection, ConnectionErrors> {
        match self.database {
            DatabaseType:: PostgreSQL(connection_config) => {
                PgConnection::establish(connection_config.database_url.unwrap())
                    .map(DbConnection::Postgres)
                    .map_err(|e| ConnectionErrors::Postgres(e.to_string()))?;
                info!("Connessione stabilita con successo al database PostgreSQL.");
            }
            DatabaseType::SQLite(connection_config) => {
                SqliteConnection::establish(connection_config.database_url.unwrap())
                    .map(DbConnection::SQLite)
                    .map_err(|e| ConnectionErrors::SQLite(e.to_string()))?;
                info!("Connessione stabilita con successo al database SQLite.");
            }
            DatabaseType::MongoDB(connection_config) => {
                // Parsing delle opzioni di connessione MongoDB dalla URL
                let client_options = ClientOptions::parse(connection_config.database_url.unwrap())
                    .await
                    .map_err(|e| ConnectionErrors::Mongo(e.to_string()))?;
                let client = Client::with_options(client_options)
                    .map_err(|e| ConnectionErrors::Mongo(e.to_string()))?;
                Ok(DbConnection::MongoDB(client))
            }
            DatabaseType::None => {
                error!("Database non configurato.");
                panic!("Database non configurato.");
            }
            _ => {
                error!("Tipo di database non supportato.");
                Err(ConnectionErrors::UnknownError("Tipo di database non supportato.".to_string()))
            }
        }
    }
}
