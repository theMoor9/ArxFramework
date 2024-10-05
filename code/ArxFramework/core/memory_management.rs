//! Modulo per la gestione della memoria in base al tipo di applicazione.
//!
//! Questo modulo fornisce un gestore della memoria che implementa diverse strategie di allocazione
//! a seconda del tipo di applicazione. Le strategie attualmente supportate includono:
//! - `Standard`: allocazione standard, utilizza il sistema di allocazione predefinito di Rust.
//! - `PoolBased`: allocazione basata su un pool di buffer pre-allocati per migliorare le prestazioni.
//! - `CustomEmbedded`: allocazione personalizzata per applicazioni embedded con requisiti specifici.
//!
//! È possibile configurare la dimensione dei buffer e del pool utilizzando la struttura `MemoryConfig`.

use crate::config::global_config::{ApplicationType, MemoryConfig};
use crate::system_core::CoreError;
use log::{info};
use std::collections::VecDeque;

// struttura globale TASKS_IN_MEMORY che mantiene tutti i Task in memoria
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::modules::default::{
    task_model::Task,
    device_model::Device,
    job_model::Job,
    macro_model::Macro,
    sensor_data_model::SensorData,
    log_event_model::LogEvent,
    command_model::Command,
    configuration_model::Configuration,
};


lazy_static! {
    pub static ref TASKS_IN_MEMORY: Mutex<HashMap<u32, Task>> = Mutex::new(HashMap::new());
    pub static ref CONFIGURATIONS_IN_MEMORY: Mutex<HashMap<u32, Configuration>> = Mutex::new(HashMap::new());
    pub static ref DEVICES_IN_MEMORY: Mutex<HashMap<u32, Device>> = Mutex::new(HashMap::new());
    pub static ref JOBS_IN_MEMORY: Mutex<HashMap<u32, Job>> = Mutex::new(HashMap::new());
    pub static ref MACROS_IN_MEMORY: Mutex<HashMap<u32, Macro>> = Mutex::new(HashMap::new());
    pub static ref SENSOR_DATA_IN_MEMORY: Mutex<HashMap<u32, SensorData>> = Mutex::new(HashMap::new());
    pub static ref LOG_EVENTS_IN_MEMORY: Mutex<HashMap<u32, LogEvent>> = Mutex::new(HashMap::new());
    pub static ref COMMANDS_IN_MEMORY: Mutex<HashMap<u32, Command>> = Mutex::new(HashMap::new());
}




/// Enum per rappresentare le diverse strategie di allocazione della memoria.
enum AllocationStrategy {
    Standard,
    PoolBased,
    CustomEmbedded,
}

/// `MemoryManager` gestisce l'allocazione e la deallocazione della memoria in base alla strategia
/// selezionata dall'applicazione.
///
/// # Campi
/// - `default_allocation_strategy`: La strategia di allocazione utilizzata.
/// - `pool`: Un pool di buffer pre-allocati (usato solo nella strategia `PoolBased`).
/// - `memory_config`: Configurazione della memoria di default fornita dall'utente.
pub struct MemoryManager {
    default_allocation_strategy: AllocationStrategy,
    pool: Option<VecDeque<Box<[u8]>>>, // Pool per l'allocazione basata su pool
    memory_config: MemoryConfig,  // Configurazione della memoria di default 
}

impl MemoryManager {
    /// Crea un nuovo gestore della memoria per l'applicazione specificata.
    ///
    /// # Parametri
    /// - `app_type`: Il tipo di applicazione (ad esempio, `WebApp`, `ApiBackend`, `DesktopApp`, ecc.).
    /// - `memory_config`: La configurazione della memoria che specifica le dimensioni dei buffer e del pool.
    ///
    /// # Ritorna
    /// Un'istanza di `MemoryManager` o un errore di tipo `CoreError` in caso di fallimento.
    pub fn new(app_type: ApplicationType, memory_config: MemoryConfig) -> Result<Self, CoreError> {
        info!("Inizializzazione del MemoryManager...");

        // Determina la strategia di allocazione in base al tipo di applicazione.
        let strategy = match app_type {
            ApplicationType::WebApp | ApplicationType::ApiBackend => AllocationStrategy::PoolBased,
            ApplicationType::DesktopApp => AllocationStrategy::Standard,
            ApplicationType::AutomationScript => AllocationStrategy::Standard,
            ApplicationType::EmbeddedSystem => AllocationStrategy::CustomEmbedded,
        };

        // Inizializza il pool solo se la strategia è `PoolBased`, utilizzando il `pool_size` configurato.
        let pool = if let AllocationStrategy::PoolBased = strategy {
            let mut pool = VecDeque::new();
            for _ in 0..memory_config.pool_size {
                pool.push_back(vec![0u8; memory_config.buffer_size].into_boxed_slice());
            }
            Some(pool)
        } else {
            None
        };

        Ok(MemoryManager { default_allocation_strategy: strategy, pool, memory_config })
    }

    /// Alloca memoria in base alla strategia configurata.
    ///
    /// # Parametri
    /// - `strategy`: La strategia di allocazione opzionale (`AllocationStrategy`). Se `None`, verrà utilizzata la strategia di default.
    /// - `size`: La quantità di memoria da allocare in byte.
    ///
    /// # Ritorna
    /// Un buffer di memoria (`Box<[u8]>`) o un errore di tipo `CoreError` in caso di fallimento.
    ///
    /// # Nota
    /// - La strategia `Standard` alloca dinamicamente la memoria.
    /// - La strategia `PoolBased` utilizza buffer pre-allocati dal pool. Se il pool è esaurito, viene effettuata un'allocazione dinamica.
    /// - La strategia `CustomEmbedded` utilizza una configurazione fissa per i buffer, che è specificata dalla configurazione della memoria (`memory_config`).
    pub fn allocate(&mut self, strategy: Option<AllocationStrategy>, size: usize) -> Result<Box<[u8]>, CoreError> {
        let alloc_strategy = strategy.unwrap_or(self.default_allocation_strategy);
    
        info!("Allocazione di {} byte di memoria con strategia {:?}...", size, alloc_strategy);
        match alloc_strategy {
            AllocationStrategy::Standard => {
                let buffer = vec![0u8; size].into_boxed_slice();
                Ok(buffer)
            },
            AllocationStrategy::PoolBased => {
                if let Some(ref mut pool) = self.pool {
                    if let Some(buffer) = pool.pop_front() {
                        Ok(buffer)
                    } else {
                        // Pool esaurito, alloca dinamicamente
                        let buffer = vec![0u8; size].into_boxed_slice();
                        Ok(buffer)
                    }
                } else {
                    Err(CoreError::ResourceAllocationError("Pool non disponibile".to_string()))
                }
            },
            AllocationStrategy::CustomEmbedded => {
                // Usa la dimensione configurata per i buffer negli embedded.
                let buffer = vec![0u8; self.memory_config.buffer_size].into_boxed_slice();
                Ok(buffer)
            },
        }
    }
    

    /// Dealloca memoria precedentemente allocata.
    ///
    /// # Parametri
    /// - `buffer`: Il buffer di memoria da deallocare.
    ///
    /// # Ritorna
    /// `Ok(())` se la deallocazione ha successo, oppure un errore di tipo `CoreError`.
    ///
    /// # Nota
    /// - Nella strategia `Standard`, Rust dealloca automaticamente la memoria.
    /// - Nella strategia `PoolBased`, il buffer viene restituito al pool.
    /// - Nella strategia `CustomEmbedded`, non è richiesta alcuna azione specifica.
    pub fn deallocate(&mut self, buffer: Box<[u8]>) -> Result<(), CoreError> {
        info!("Deallocazione della memoria...");
        match self.default_allocation_strategy {
            AllocationStrategy::Standard => {
                // Rust dealloca automaticamente la memoria.
                Ok(())
            },
            AllocationStrategy::PoolBased => {
                // Restituisce il buffer al pool.
                if let Some(ref mut pool) = self.pool {
                    pool.push_back(buffer);
                    Ok(())
                } else {
                    Err(CoreError::ResourceAllocationError("Pool non disponibile".to_string()))
                }
            },
            AllocationStrategy::CustomEmbedded => {
                // Gestione personalizzata per sistemi embedded.
                Ok(())
            },
        }
    }
}

/// # Aggiunta di Nuovi Modelli o Strategie di Allocazione
/// Se desideri aggiungere nuovi modelli o strategie di allocazione, segui questi passaggi:
/// 1. **Nuova Strategia**: Aggiungi una nuova variante all'enum `AllocationStrategy` per rappresentare la tua strategia.
/// 2. **Logica di Allocazione**: Aggiorna i metodi `allocate` e `deallocate` per gestire la nuova strategia.
/// 3. **Configurazione**: Assicurati di aggiungere nuove opzioni alla struttura `MemoryConfig` per configurare il comportamento della nuova strategia.
/// 4. **Testing**: Scrivi test unitari per verificare il corretto funzionamento della tua nuova strategia di allocazione.
