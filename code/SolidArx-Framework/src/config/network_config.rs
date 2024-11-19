use std::time::Duration;
use log::{info, error};

/// Struttura che rappresenta le configurazioni specifiche per ogni tipo di database
/// - `database_url`: URL del database da utilizzare.
/// - `max_connections`: Numero massimo di connessioni simultanee consentite.
/// - `retry_attempts`: Numero massimo di tentativi di riconnessione in caso di fallimento.
/// - `max_idle_time`: Durata massima per mantenere una connessione inattiva.
/// - `connection_timeout`: Tempo massimo di attesa per stabilire una connessione.#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub disable_database: bool,
    pub database_url: String,
    pub max_connections: Option<u32>,
    pub retry_attempts: Option<u32>,
    pub max_idle_time: Option<u64>,       // Durata massima per mantenere una connessione inattiva
    pub connection_timeout: Option<u64>,  // Tempo massimo di attesa per stabilire una connessione
}

/// Enum per definire i diversi tipi di sistemi di database supportati.
/// Ogni variante contiene una `ConnectionConfig` per il rispettivo tipo di database.
pub enum DatabaseType {
    PostgreSQL(ConnectionConfig),
    SQLite(ConnectionConfig),
    MongoDB(ConnectionConfig),
}

impl DatabaseType {
    /// Crea una nuova istanza di `DatabaseType` in base alla configurazione dell'applicazione.
    /// Restituisce un `Result` che contiene un errore se nessuna configurazione è definita per l'app corrente.
    pub fn new(disable_database ,database_url, max_connections, retry_attempts, max_idle_time, connection_timeout) -> Result<Self, &'static str> {

        match database_url {
            None => {
                error!("Database URL not set for the application");
                return Err("Database URL not set for the application try use the CLI command to set the database URL.\
                            Type 'Arx Help' for more information");
            }
            Some(url) => {
                if max_connection == None {
                    #[cfg(feature = "webapp")]
                    apply_max_connections = Some(100);
                    #[cfg(feature = "api_backend")]
                    apply_max_connections = Some(50);
                    #[cfg(feature = "desktop")]
                    apply_max_connections = Some(5);
                    #[cfg(feature = "automation")]
                    apply_max_connections = Some(20);
                    #[cfg(feature = "embedded")]
                    apply_max_connections = Some(1);
                }
        
                if retry_attempts == None {
                    #[cfg(feature = "webapp")]
                    apply_retry_attempts = Some(3);
                    #[cfg(feature = "api_backend")]
                    apply_retry_attempts = Some(3);
                    #[cfg(feature = "desktop")]
                    apply_retry_attempts = Some(1);
                    #[cfg(feature = "automation")]
                    apply_retry_attempts = Some(5);
                    #[cfg(feature = "embedded")]
                    apply_retry_attempts = Some(1);
                }
                
                if max_idle_time == None {
                    #[cfg(feature = "webapp")]
                    apply_max_idle_time = Some(Duration::from_secs(300));
                    #[cfg(feature = "api_backend")]
                    apply_max_idle_time = Some(Duration::from_secs(600));
                    #[cfg(feature = "desktop")]
                    apply_max_idle_time = Some(Duration::from_secs(1200));
                    #[cfg(feature = "automation")]
                    apply_max_idle_time = Some(Duration::from_secs(3600));
                    #[cfg(feature = "embedded")]
                    apply_max_idle_time = Some(Duration::from_secs(600));
                }
        
                if connection_timeout == None {
                    #[cfg(feature = "webapp")]
                    apply_connection_timeout = Some(Duration::from_secs(5));
                    #[cfg(feature = "api_backend")]
                    apply_connection_timeout = Some(Duration::from_secs(10));
                    #[cfg(feature = "desktop")]
                    apply_connection_timeout = Some(Duration::from_secs(15));
                    #[cfg(feature = "automation")]
                    apply_connection_timeout = Some(Duration::from_secs(30));
                    #[cfg(feature = "embedded")]
                    apply_connection_timeout = Some(Duration::from_secs(5));
                }
        
                
            }
        }
       
        let apply_database_url = database_url.unwrap();

        // Configurazione per applicazioni Web, utilizzando PostgreSQL
        #[cfg(feature = "webapp")]
        return Ok(DatabaseType::PostgreSQL(ConnectionConfig {
            apply_database_url,
            apply_max_connections,
            apply_retry_attempts,
            apply_max_idle_time,       
            apply_connection_timeout,   
        }));
        // Configurazione per API backend, utilizzando PostgreSQL
        #[cfg(feature = "api_backend")]
        return Ok(DatabaseType::PostgreSQL(ConnectionConfig {
            apply_database_url,
            apply_max_connections,
            apply_retry_attempts,
            apply_max_idle_time,
            apply_connection_timeout,
        }));
        // Configurazione per applicazioni desktop, utilizzando SQLite
        #[cfg(feature = "desktop")]
        return Ok(DatabaseType::PostgreSQL(ConnectionConfig {
            apply_database_url,
            apply_max_connection,
            apply_retry_attempts,
            apply_max_idle_time,
            apply_connection_timeout,
        }));
        // Configurazione per applicazioni embedded, utilizzando SQLite
        #[cfg(feature = "automation")]
        return Ok(DatabaseType::MongoDB(ConnectionConfig {
            apply_database_url,
            apply_max_connections,
            apply_retry_attempts,
            apply_max_idle_time,  
            apply_connection_timeout,
        }));
        // Configurazione per applicazioni embedded, utilizzando SQLite
        #[cfg(feature = "embedded")]
        return Ok(DatabaseType::SQLite(ConnectionConfig {
            apply_database_url,
            apply_max_connections,
            apply_retry_attempts,
            apply_max_idle_time,
            apply_connection_timeout,
        }));
    }

    /// Funzione di log che fornisce informazioni di stato sulla connessione al database.
    /// Ogni configurazione viene registrata come `info` nel log.
    pub fn log_status(&self) {
        match self {
            // Cattura ogni variante di database e logga i rispettivi dettagli di configurazione
            DatabaseType::PostgreSQL(config) | DatabaseType::SQLite(config) | DatabaseType::MongoDB(config) => {
                info!("Database URL: {}", config.database_url);
                // Log del numero massimo di connessioni, se definito
                if let Some(max_conn) = config.max_connections {
                    info!("Max Connections: {}", max_conn);
                }
                // Log del numero di tentativi di riconnessione, se definito
                if let Some(retries) = config.retry_attempts {
                    info!("Retry Attempts: {}", retries);
                }
                // Log della durata massima di inattività, se definita
                if let Some(idle_time) = config.max_idle_time {
                    info!("Max Idle Time: {:?}", idle_time);
                }
                // Log di attesa massimo per la connessione, se definito
                if let Some(timeout) = config.connection_timeout {
                    info!("Connection Timeout: {:?}", timeout);
                }
            }
        }
    }
}