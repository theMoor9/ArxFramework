use clap::{Parser, Subcommand};
use crate::config::global_config::ApplicationType; 

/// CLI per ArxFramework
#[derive(Parser)]
#[command(name = "Arx")]
#[command(about = "CLI per il framework Arx", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Comandi supportati da Arx
#[derive(Subcommand)]
pub enum Commands {
    /// Inizializza un nuovo progetto con un tipo di applicazione specifico
    Init {
        /// Il tipo di applicazione da inizializzare (WebApp, ApiBackend, DesktopApp, etc.)
        #[arg(short = 'a', long = "app_type")]
        app_type: ApplicationType,
        /// Memory multiplier per mole di memoria
        /// Usato per genrare variabile di config per il modulo crud_ops.rs
        #[arg(short = 'm', long = "memory_scale", default_value_t = 1)] // Valore di default: 1.0 
        memory_scale: u8, 
        /// Numero massimo di thread per l'applicazione
        #[arg(short = 't', long = "max_threads", default_value_t = 8)] // Valore di default: 8
        max_threads: u8,
        /// Buffer size per la memoria
        #[arg(short = 'b', long = "buffer_size", default_value_t = 0)] // Valore di default: 1024
        buffer_size: usize,
        /// Pool size per la memoria
        #[arg(short = 'p', long = "pool_size", default_value_t = 0)] // Valore di default: 8
        pool_size: usize,
    },
    Database {
        /// Disabilita l'utilizzo del database
        #[arg(short = 'd', long = "disable_database"), default_value_t = true]
        disable_database: bool,

        #[arg(short = 'url', long = "database_url"), default_value_t = None]
        database_url: Option<String>,

        #[arg(short = 'c', long = "max_connections", default_value_t = 20)]
        max_connections: Option<u32>,

        #[arg(short = 'r', long = "retry_attempts", default_value_t = 5)]
        retry_attempts: Option<u32>,

        #[arg(short = 'i', long = "max_idle_time", default_value_t = 20)]
        max_idle_time: Option<u64>,

        #[arg(short = 't', long = "connection_timeout", default_value_t = 1)]
        connection_timeout: Option<u64>,
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
        --disable_database <Bool> or --d <Bool>                         # Disable Database, Default: true\n
        --database_url Option<String>  or --url Option<String>          # Set Database URL, Default: None\n
        --max_connections Option<u32> or --c Option<u32>                # Set Max Connections, Default: 20\ \n
        --retry_attempts Option<u32> or --r Option<u32>                 # Set Retry Attempts, Default: 5\\n
        --max_idle_time Option<u64> or --i Option<u64>                  # Set Max Idle Time, Default: 30\\n
        --connection_timeout Option<u64> or --t Option<u64>             # Set Connection Timeout, Default: 1\\n"
    },
}

/// Parsing degli argomenti e ritorno della configurazione CLI
pub fn parse_arguments() -> Result<Cli, clap::Error> {
    Cli::try_parse()
}