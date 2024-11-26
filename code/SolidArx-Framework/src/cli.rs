use clap::{Parser, Subcommand};
use std::str::FromStr;
use crate::config::global_config::{
    ApplicationType,
    DatabaseType,
}; 

/// CLI per ArxFramework
#[derive(Parser)]
#[command(name = "Arx")]
#[command(about = "CLI per il framework Arx", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Converte il CLI input in un tipo di database
impl FromStr for DatabaseType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "postgresql" | "postgre" | "pgs" => Ok(DatabaseType::PostgreSQL(ConnectionConfig::default())),
            "sqlite" | "sl" => Ok(DatabaseType::SQLite(ConnectionConfig::default())),
            "mongodb" | "mongo" | "mg" => Ok(DatabaseType::MongoDB(ConnectionConfig::default())),
            "none" | "None" => Ok(DatabaseType::None),
            _ => Err(format!("Tipo di database non riconosciuto: {}", input)),
        }
    }
}

/// Converte il ClI input in un tipo di applicazione

impl FromStr for ApplicationType {

    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "webapp" | "web" | "wa" => Ok(ApplicationType::WebApp),
            "apibackend" | "api" | "ab" => Ok(ApplicationType::ApiBackend),
            "desktopapp" | "desktop" | "da" => Ok(ApplicationType::DesktopApp),
            "automationscript" | "automation" | "as" => Ok(ApplicationType::AutomationScript),
            "embeddedsystem" | "embedded" | "es" => Ok(ApplicationType::EmbeddedSystem),
            _ => Err(format!("Tipo di applicazione non riconosciuto: {}", input)),
        }
    }

}

/// Comandi supportati da Arx
#[derive(Subcommand)]
pub enum Commands {
    /// Inizializza un nuovo progetto con un tipo di applicazione specifico
    Init {
        /// Il tipo di applicazione da inizializzare (WebApp, ApiBackend, DesktopApp, etc.)
        #[arg(short = 'a', long = "app-type")]
        app_type: ApplicationType,
        /// Memory multiplier per mole di memoria
        /// Usato per genrare variabile di config per il modulo crud_ops.rs
        #[arg(short = 'm', long = "memory-scale", default_value_t = 1)] // Valore di default: 1.0 
        memory_scale: u8, 
        /// Numero massimo di thread per l'applicazione
        #[arg(short = 't', long = "max-threads", default_value_t = 8)] // Valore di default: 8
        max_threads: u8,
        /// Buffer size per la memoria
        #[arg(short = 'b', long = "buffer-size", default_value_t = 0)] // Valore di default: 1024
        buffer_size: usize,
        /// Pool size per la memoria
        #[arg(short = 'p', long = "pool-size", default_value_t = 0)] // Valore di default: 8
        pool_size: usize,
    },
    Database {

        /// Disabilita l'utilizzo del database
        #[arg(short = 'd', long = "database-type"), default_value_t = DatabaseType::None]
        database_type_reference: DatabaseType,

        /// URL del database da utilizzare
        #[arg(short = 'url', long = "database-url"), default_value_t = None]
        database_url: Option<String>,

        /// Numero massimo di connessioni simultanee consentite
        #[arg(short = 'c', long = "max-connections", default_value_t = None)]
        max_connections: Option<u32>,

        /// Numero massimo di tentativi di riconnessione in caso di fallimento
        #[arg(short = 'r', long = "retry-attempts", default_value_t = None)]
        max_attempts: Option<u32>,

        /// Durata massima per mantenere una connessione inattiva
        #[arg(short = 'i', long = "max-idle-time", default_value_t = None)]
        max_idle_time: Option<u64>,

        /// Tempo massimo di attesa per stabilire una connessione
        #[arg(short = 't', long = "connection-timeout", default_value_t = None)]
        connection_timeout: Option<u64>,

        // I campi sono utilizzati in:
        // - `config/network_config.rs` per la configurazione della connessione al database
        // - `network/connection_management.rs` per la gestione delle connessioni al database
        // - `core/system_core.rs` nella funzione run() per lo scraping dei dati
    }
    Help{
        "HELP\n
        Command: Arx\n
        arx init  # Init app\n

        Options:\n 
        --app_type or -a\n
        \n
            Application types:\n
            --app_type <WebApp>\n
            --app_type <ApiBackend>\n
            --app_type <DesktopApp>\n
            --app_type <AutomationScript>\n
            --app_type <EmbeddedSystem>\n
        \n
        --memory_scale u8 or --m u8           # Set Memory Multiplier, DEFAULT: 1\n
        --max_threads u8 or --t u8            # Set Max Threads, DEFAULT: 8\n
        --buffer_size usize or --b usize      # Set Buffer Size, DEFAULT: 1024\n
        --version or --v                      # Display Version\n
        \n
        Command: Arx database\n
        arx database #Define Database\n 
        \n
        Options:\n
        --database_type <Bool> or --d <Bool>                         # Disable Database, Default: true\n
        --database_url Option<String>  or --url Option<String>          # Set Database URL, Default: None\n
        --max_connections Option<u32> or --c Option<u32>                # Set Max Connections, Default: None\ \n
        --retry_attempts Option<u32> or --r Option<u32>                 # Set Retry Attempts, Default: None\\n
        --max_idle_time Option<u64> or --i Option<u64>                  # Set Max Idle Time, Default: None\\n
        --connection_timeout Option<u64> or --t Option<u64>             # Set Connection Timeout, Default: None\n\n
        
        Default None implica gestione predefinita da parte del codice\\n"
    },
}

/// Parsing degli argomenti e ritorno della configurazione CLI
pub fn parse_arguments() -> Result<Cli, clap::Error> {
    Cli::try_parse()
}