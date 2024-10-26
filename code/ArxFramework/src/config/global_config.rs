//! Modulo per la gestione della configurazione globale del sistema.
//!
//! Questo modulo fornisce la configurazione globale del sistema, inclusi i tipi di applicazioni supportati
//! e le impostazioni di memoria. È possibile estendere il sistema con nuove variabili di configurazione
//! in modo compatibile e sicuro, mantenendo la semplicità. Perfetto per i dev che piace "incespicare" :D
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

/// Configurazione della memoria per il sistema.
///
/// Questa configurazione è utilizzata per gestire le impostazioni relative alla memoria,
/// come la dimensione del pool di buffer o la dimensione del buffer nei sistemi embedded.
pub struct MemoryConfig {
    pub pool_size: usize,    // Dimensione del pool di buffer (per PoolBased)
    pub buffer_size: usize,  // Dimensione del buffer (per Embedded)
    pub memory_scale: u8,   // Scala per la dimensione della memoria dei modelli
}

/// Implementazione del valore di default per `MemoryConfig`.
///
/// Fornisce impostazioni predefinite per la configurazione della memoria.
impl Default for MemoryConfig {
    fn default() -> Self {
        MemoryConfig {
            pool_size: 10,      // Valore di default: 10 buffer per il pool
            buffer_size: 1024,   // Valore di default: 1024 byte per buffer
            memory_scale: 1,  // Valore di default: scala 1.0 per la dimensione della memoria
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
// impl Default for CoreConfig {
//     fn default() -> Self {
//         CoreConfig {
//             app_type: ApplicationType::WebApp,
//             max_threads: 4,
//             cache_size: 256, // Imposta un valore di default per la cache
//         }
//     }
// }
// ```