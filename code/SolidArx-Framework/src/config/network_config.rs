use log::{info, error};
use cfg_if::cfg_if;

/// Struttura che rappresenta le configurazioni specifiche per ogni tipo di database
/// - `database_url`: URL del database da utilizzare.
/// - `max_connections`: Numero massimo di connessioni simultanee consentite.
/// - `retry_attempts`: Numero massimo di tentativi di riconnessione in caso di fallimento.
/// - `max_idle_time`: Durata massima per mantenere una connessione inattiva.
/// - `connection_timeout`: Tempo massimo di attesa per stabilire una connessione.
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub database_url: Option<String>,
    pub max_connections: Option<u32>,
    pub retry_attempts: Option<u32>,
    pub max_idle_time: Option<u64>,       // Durata massima per mantenere una connessione inattiva
    pub connection_timeout: Option<u64>,  // Tempo massimo di attesa per stabilire una connessione
}

/// Enum per definire i diversi tipi di sistemi di database supportati.
/// Ogni variante contiene una `ConnectionConfig` per il rispettivo tipo di database.
#[derive(Debug, Clone)]
pub enum DatabaseType {
    PostgreSQL(ConnectionConfig),
    SQLite(ConnectionConfig),
    MongoDB(ConnectionConfig),
    None,
}


impl DatabaseType {
    /// Crea una nuova istanza di `DatabaseType` in base alla configurazione dell'applicazione.
    /// Restituisce un `Result` che contiene un errore se nessuna configurazione è definita per l'app corrente.
    pub fn new( 
        du: Option<String>, 
        mut mc: Option<u32>, 
        mut ra: Option<u32>, 
        mut mit: Option<u64>, 
        mut ct: Option<u64>
    ) -> Result<Self, &'static str> {

        // Sezione di controllo delle variabili costruttore
        // Imposta valori di default specifici per ogni app se non definiti
        
        if du == None {
            error!("Database URL non impostato per l' applicazione. Utilizzare il comando CLI per impostare l'URL del database.\
                    Digitare 'Arx Help' per ulteriori informazioni");
            return Err("Database URL non impostato per l' applicazione. Utilizzare il comando CLI per impostare l'URL del database.\
                    Digitare 'Arx Help' per ulteriori informazioni");
        }   

        // Questa sezione di occupa di impostare i valori di default per le variabili non definite
        if mc == None {
            cfg_if! {
                if #[cfg(feature = "webapp")] {
                    mc = Some(100);
                } else if #[cfg(feature = "api_backend")] {
                    mc = Some(50);
                } else if #[cfg(feature = "desktop")] {
                    mc = Some(5);
                } else if #[cfg(feature = "automation")] {
                    mc = Some(20);
                } else if #[cfg(feature = "embedded")] {
                    mc = Some(1);
                }
            }
        }
        
        if ra == None {
            cfg_if! {
                if #[cfg(feature = "webapp")] {
                    ra = Some(3);
                } else if #[cfg(feature = "api_backend")] {
                    ra = Some(3);
                } else if #[cfg(feature = "desktop")] {
                    ra = Some(1);
                } else if #[cfg(feature = "automation")] {
                    ra = Some(5);
                } else if #[cfg(feature = "embedded")] {
                    ra = Some(1);
                }
            }
        }
        
        if mit == None {
            cfg_if! {
                if #[cfg(feature = "webapp")] {
                    mit = Some(300);
                } else if #[cfg(feature = "api_backend")] {
                    mit = Some(600);
                } else if #[cfg(feature = "desktop")] {
                    mit = Some(1200);
                } else if #[cfg(feature = "automation")] {
                    mit = Some(3600);
                } else if #[cfg(feature = "embedded")] {
                    mit = Some(600);
                }
            }
        }
        
        if ct == None {
            cfg_if! {
                if #[cfg(feature = "webapp")] {
                    ct = Some(5);
                } else if #[cfg(feature = "api_backend")] {
                    ct = Some(10);
                } else if #[cfg(feature = "desktop")] {
                    ct = Some(15);
                } else if #[cfg(feature = "automation")] {
                    ct = Some(30);
                } else if #[cfg(feature = "embedded")] {
                    ct = Some(5);
                }
            }
        }
        
        return Ok(
            DatabaseType::PostgreSQL(
                ConnectionConfig {
                    database_url:du,
                    max_connections:mc,
                    retry_attempts:ra,
                    max_idle_time:mit,       
                    connection_timeout:ct,   
                }
            )
        );
            
    }
       

    /// Funzione di log che fornisce informazioni di stato sulla connessione al database.
    /// Ogni configurazione viene registrata come `info` nel log.
    pub fn log_status(&self) {
        match self {
            // Cattura ogni variante di database e logga i rispettivi dettagli di configurazione
            DatabaseType::PostgreSQL(config) | DatabaseType::SQLite(config) | DatabaseType::MongoDB(config) => {
                info!("Database URL: {}", config.database_url.as_ref().unwrap_or(&"Non specificato".to_string()));
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
                    info!("Max Idle Time: {}", idle_time);
                }
                // Log di attesa massimo per la connessione, se definito
                if let Some(timeout) = config.connection_timeout {
                    info!("Connection Timeout: {}", timeout);
                }
            }
            DatabaseType::None => {
                info!("Nessun database configurato.");
            }
        }
    }
}