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
use std::thread::sleep;

use crate::config::network_config::{ConnectionConfig, DatabaseType};

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
    pub fn new(config: ConnectionConfig) -> Self {
        Self { config }
    }

    /// Inizializza la connessione al database con un meccanismo di retry per i tentativi falliti.
    ///
    /// Questa funzione tenta di stabilire una connessione al database configurato,
    /// riprovando in caso di fallimento fino al numero massimo di tentativi definiti.
    ///
    /// # Ritorna
    /// - `Ok(PgConnection)`: Connessione stabilita con successo.
    /// - `Err(diesel::ConnectionError)`: Errore se il massimo numero di tentativi è superato.
    pub fn initialize_connection(&self) -> Result<PgConnection, diesel::ConnectionError> {
        let mut attempts = 0;
        
        loop {
            // Tenta di stabilire la connessione
            match self.connect() {
                Ok(connection) => {
                    info!("Connessione stabile.");
                    return Ok(connection);
                }
                Err(e) => {
                    attempts += 1;
                    error!("Tentativo {} fallito: {}", attempts, e);

                    // Controlla se il numero massimo di tentativi è stato raggiunto.
                    if attempts >= self.config.max_retries {
                        error!("Superato il numero massimo di tentativi di connessione.");
                        return Err(e);
                    }

                    // Attende per il periodo definito in `retry_timeout` con backoff esponenziale basato sui tentativi
                    info!(
                        "Ritenterò tra {:?} secondi...",
                        self.config.retry_timeout.as_secs()
                    );
                    sleep(self.config.retry_timeout.pow(attempts));
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
    /// - `Err(Box<dyn Error>)`: Se si verifica un errore durante il tentativo di connessione.
    async fn connect(&self) -> Result<DbConnection, Box<dyn Error>> {

        match self.config.database_type {
            #[cfg(any(feature = "webapp", feature = "api_backend"))]
            DatabaseType::Postgres => {
                let result = PgConnection::establish(&self.config.database_url)
                    .map_err(|e| Box::new(e) as Box<dyn Error>)?;
                info!("Connessione stabilita con successo al database PostgreSQL.");
                Ok(result)
            }
            #[cfg(any(feature = "desktop", feature = "embedded"))]
            DatabaseType::SQLite => {
                let result = SqliteConnection::establish(&self.config.database_url)
                    .map_err(|e| Box::new(e) as Box<dyn Error>)?;
                info!("Connessione stabilita con successo al database SQLite.");
                Ok(result)
            }
            #[cfg(feature = "automation")]
            DatabaseType::MongoDB => {
                // Parsing delle opzioni di connessione MongoDB dalla URL
                let client_options = ClientOptions::parse(&self.config.database_url).await?;
                let client = Client::with_options(client_options)?;
                info!("Connessione stabilita con successo al database MongoDB.");
                Ok(client)
            }
        }
    }
}
