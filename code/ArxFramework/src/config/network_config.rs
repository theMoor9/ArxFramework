use std::time::Duration;
use log::{info, error};

/// Struttura che rappresenta le configurazioni specifiche per ogni tipo di database
/// - `database_url`: URL del database da utilizzare.
/// - `max_connections`: Numero massimo di connessioni simultanee consentite.
/// - `retry_attempts`: Numero massimo di tentativi di riconnessione in caso di fallimento.
/// - `max_idle_time`: Durata massima per mantenere una connessione inattiva.
/// - `connection_timeout`: Tempo massimo di attesa per stabilire una connessione.#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub database_url: String,
    pub max_connections: Option<u32>,
    pub retry_attempts: Option<u32>,
    pub max_idle_time: Option<Duration>,       // Durata massima per mantenere una connessione inattiva
    pub connection_timeout: Option<Duration>,  // Tempo massimo di attesa per stabilire una connessione
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
    pub fn new() -> Result<Self, &'static str> {
        // Configurazione per applicazioni Web, utilizzando PostgreSQL
        #[cfg(feature = "webapp")]
        return Ok(DatabaseType::PostgreSQL(ConnectionConfig {
            database_url: "postgres://localhost/webapp_db".to_string(),
            max_connections: Some(100),
            retry_attempts: Some(3),
            max_idle_time: Some(Duration::from_secs(300)),       // 5 minuti
            connection_timeout: Some(Duration::from_secs(5)),   // 30 secondi
        }));
        // Configurazione per API backend, utilizzando PostgreSQL
        #[cfg(feature = "api_backend")]
        return Ok(DatabaseType::PostgreSQL(ConnectionConfig {
            database_url: "postgres://localhost/api_backend_db".to_string(),
            max_connections: Some(50),
            retry_attempts: Some(3),
            max_idle_time: Some(Duration::from_secs(600)),
            connection_timeout: Some(Duration::from_secs(10)),
        }));
        // Configurazione per applicazioni desktop, utilizzando SQLite
        #[cfg(feature = "desktop")]
        return Ok(DatabaseType::SQLite(ConnectionConfig {
            database_url: "sqlite://desktop_app.db".to_string(),
            max_connections: Some(5),
            retry_attempts: Some(1),
            max_idle_time: Some(Duration::from_secs(1200)),
            connection_timeout: Some(Duration::from_secs(15)),
        }));
        // Configurazione per applicazioni embedded, utilizzando SQLite
        #[cfg(feature = "automation")]
        return Ok(DatabaseType::MongoDB(ConnectionConfig {
            database_url: "mongodb://localhost:27017/automation_db".to_string(),
            max_connections: Some(20),
            retry_attempts: Some(5),
            max_idle_time: Some(Duration::from_secs(3600)),  // 10 minuti
            connection_timeout: Some(Duration::from_secs(30)),
        }));
        // Configurazione per applicazioni embedded, utilizzando SQLite
        #[cfg(feature = "embedded")]
        return Ok(DatabaseType::SQLite(ConnectionConfig {
            database_url: "sqlite://embedded_app.db".to_string(),
            max_connections: Some(1),
            retry_attempts: Some(1),
            max_idle_time: Some(Duration::from_secs(600)),
            connection_timeout: Some(Duration::from_secs(5)),
        }));

        Err("Database configuration not set for the application")
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