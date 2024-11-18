//! Modulo per la gestione della configurazione globale del sistema.
//!
//! Questo modulo fornisce la configurazione globale del sistema, inclusi i tipi di applicazioni supportati
//! È possibile estendere il sistema con nuove variabili di configurazione
//! in modo compatibile e sicuro, mantenendo la semplicità. Perfetto per i dev a cui piace "incespicare" :D
//!
//! Per aggiungere una nuova variabile di configurazione, basta creare una nuova struttura di configurazione
//! e integrarla nel `CoreConfig`. Ecco come aggiungere nuove variabili mantenendo la compatibilità con il framework.

// Tipi di applicazione supportati
use clap::{Parser, ValueEnum}; // Necessario per l'implementazione di delle versioni di ApplicationType nel CLI
#[derive(Debug,Clone,Parser, ValueEnum)]
pub enum ApplicationType {
    WebApp,
    ApiBackend,
    DesktopApp,
    AutomationScript,
    EmbeddedSystem,
    None,
}

/// Configurazione globale del sistema.
///
/// Questo struct contiene le impostazioni principali del sistema, come il tipo di applicazione
/// e il numero massimo di thread. Può essere esteso con nuovi campi secondo necessità.
#[derive(Debug)]
pub struct CoreConfig {
    pub app_type: ApplicationType,
    pub max_threads: u8,
}

impl Default for CoreConfig {
    fn default() -> Self {
        CoreConfig {
            app_type: ApplicationType::None,
            max_threads: 4,
        }
    }
    fn new(app_type: ApplicationType, max_threads: u8) -> Self {
        CoreConfig {
            app_type,
            max_threads,
        }
    }
}

// Esempio di come estendere la configurazione globale
// con nuove variabili personalizzate.
//
// Se sei uno di quei dev che amano "incespicare" tra le variabili di configurazione,
// puoi tranquillamente aggiungere nuovi campi. Per esempio, per aggiungere una nuova
// variabile di configurazione `cache_size`, puoi farlo così:
//
// ```rust
// pub struct CoreConfig {
//     pub app_type: ApplicationType,
//     pub max_threads: usize,
//     pub cache_size: usize, // Nuova variabile per gestire la dimensione della cache
// }
// 

// ```