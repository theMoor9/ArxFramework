/// Il modulo `ConnectionManager` gestisce la connessione a vari database in base alla configurazione specificata.
/// Supporta PostgreSQL, SQLite e MongoDB, con funzionalità di retry per tentativi di connessione falliti.

#[cfg(feature = "automation")] 
use mongodb::{Client, options::ClientOptions}; 
#[cfg(any(feature = "desktop", feature = "embedded"))]
use diesel::sqlite::SqliteConnection;
#[cfg(any(feature = "webapp", feature = "api_backend"))]
use diesel::pg::PgConnection;
use log::{error, info};
use std::time::Duration;
use tokio::time::sleep;

use crate::config::network_config::{ConnectionConfig, DatabaseType};

/// Enum per rappresentare errori di connessione al database
#[derive(Debug)]
enum ConnectionErrors {
    Postgres(String),
    SQLite(String),
    Mongo(String),
    UnknownError(String),
}

impl std::fmt::Display for ConnectionErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionErrors::Postgres(msg) => write!(f, "Errore PostgreSQL: {}", msg),
            ConnectionErrors::SQLite(msg) => write!(f, "Errore SQLite: {}", msg),
            ConnectionErrors::Mongo(msg) => write!(f, "Errore MongoDB: {}", msg),
            ConnectionErrors::UnknownError(msg) => write!(f, "Errore sconosciuto: {}", msg),
        }
    }
}

impl std::error::Error for ConnectionErrors {}

enum DbConnection {
    Postgres(PgConnection),
    SQLite(SqliteConnection),
    MongoDB(Client),
}

/// Struttura `ConnectionManager`
/// 
/// La struttura principale per la gestione delle connessioni.
/// Contiene un campo `config` che memorizza i dettagli della configurazione del database.
pub struct ConnectionManager {
    config: ConnectionConfig,
}

impl ConnectionManager {
    /// Crea una nuova istanza di `ConnectionManager` con la configurazione fornita.
    ///
    /// # Parametri
    /// - `config`: Configurazione della connessione di tipo `ConnectionConfig`.
    ///
    /// # Ritorna
    /// Una nuova istanza di `ConnectionManager`
    pub fn new(db: DatabaseType) -> Self {
        match db {
            DatabaseType::Postgres(connection_config) => {
                Self { 
                    config: connection_config,
                }
            }
            DatabaseType::SQLite(connection_config) => {
                Self { 
                    config: connection_config,
                }
            }
            DatabaseType::MongoDB(connection_config) => {
                Self { 
                    config: connection_config,
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
    async pub fn initialize_connection(&self) -> Result<PgConnection, diesel::ConnectionError> {
        let mut attempts = 0;
        
        loop {
            // Tenta di stabilire la connessione
            match self.connect().await {
                Ok(connection) => {
                    info!("Connessione stabile.");
                    return Ok(connection);
                }
                Err(e) => {
                    attempts += 1;
                    error!("Tentativo {} fallito: {}", attempts, e);

                    // Controlla se il numero massimo di tentativi è stato raggiunto
                    if attempts >= self.config.max_retries {
                        error!("Superato il numero massimo di tentativi di connessione.");
                        return Err(Box::new(e));
                    }

                    // Attende per il periodo definito in `retry_timeout` con backoff esponenziale
                    let backoff = self.config.retry_timeout.unwrap().as_secs() * (attempts as u64);
                    info!("Ritenterò tra {} secondi...", backoff);
                    sleep(Duration::from_secs(backoff)).await;
                }
            }
        }
    }

    /// Funzione privata per stabilire una connessione al database, in base al tipo configurato.
    ///
    /// Utilizza un `match` sul tipo di database per determinare il meccanismo di connessione appropriato.
    ///
    /// # Ritorna
    /// - `Ok(())`: Se la connessione è stabilita con successo.
    /// - `Err(ConnectionErrors)`: Se si verifica un errore durante il tentativo di connessione.
    async fn connect(&self) -> Result<DbConnection, ConnectionErrors> {

        match self.config.database_type {
            #[cfg(any(feature = "webapp", feature = "api_backend"))]
            DatabaseType::Postgres => {
                PgConnection::establish(&self.config.database_url)
                    .map(DbConnection::Postgres)
                    .map_err(|e| ConnectionErrors::Postgres(e.to_string()))
                info!("Connessione stabilita con successo al database PostgreSQL.");
            }
            #[cfg(any(feature = "desktop", feature = "embedded"))]
            DatabaseType::SQLite => {
                SqliteConnection::establish(&self.config.database_url)
                    .map(DbConnection::SQLite)
                    .map_err(|e| ConnectionErrors::SQLite(e.to_string()))
                info!("Connessione stabilita con successo al database SQLite.");
            }
            #[cfg(feature = "automation")]
            DatabaseType::MongoDB => {
                // Parsing delle opzioni di connessione MongoDB dalla URL
                let client_options = ClientOptions::parse(&self.config.database_url)
                    .await
                    .map_err(|e| ConnectionErrors::Mongo(e.to_string()))?;
                let client = Client::with_options(client_options)
                    .map_err(|e| ConnectionErrors::Mongo(e.to_string()))?;
                Ok(DbConnection::MongoDB(client))
            }
        }
    }
}
