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
    Help,
}

/// Parsing degli argomenti e ritorno della configurazione CLI
pub fn parse_arguments() -> Result<Cli, clap::Error> {
    Cli::try_parse()
}