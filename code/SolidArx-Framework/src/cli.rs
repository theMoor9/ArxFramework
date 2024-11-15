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
        #[arg(short = 'd', long = "database_url")]
        database_url: String,

        #[arg(short = 'c', long = "max_connections", default_value_t = None)]
        max_connections: Option<u32>,

        #[arg(short = 'r', long = "retry_attempts", default_value_t = None)]
        retry_attempts: Option<u32>,

        #[arg(short = 'i', long = "max_idle_time", default_value_t = None)]
        max_idle_time: Option<u64>,

        #[arg(short = 'o', long = "connection_timeout", default_value_t = None)]
        connection_timeout: Option<u64>,
    }
    Help{
        "HELP\n
        Command: Arx\n
        arx init --app_type <webapp>   # Init app\n
        \n
        Application types:\n
        --app_type <WebApp>\n
        --app_type <ApiBackend>\n
        --app_type <DesktopApp>\n
        --app_type <AutomationScript>\n
        --app_type <EmbeddedSystem>\n
        \n
        Options:\n  
        --memory_scale 1           # Set Memory Multiplier, DEFAULT: 1\n
        --max_threads 8            # Set Max Threads, DEFAULT: 8\n
        --version                  # Display Version\n
        \n
        Command: Arx database\n
        arx database --database_url <url> --max_connections <max_connections> --retry_attempts <retry_attempts> --max_idle_time <max_idle_time> --connection_timeout <connection_timeout>\n
        \n
        Options:\n
        --database_url <url>                        # Set Database URL, Default: None\n
        --max_connections <max_connections>         # Set Max Connections, Default: None\ \n
        --retry_attempts <retry_attempts>           # Set Retry Attempts, Default: None\\n
        --max_idle_time <max_idle_time>             # Set Max Idle Time, Default: None\\n
        --connection_timeout <connection_timeout>   # Set Connection Timeout, Default: None\\n"


    },
}

/// Parsing degli argomenti e ritorno della configurazione CLI
pub fn parse_arguments() -> Result<Cli, clap::Error> {
    Cli::try_parse()
}