# core/

| Modulo       | Linguaggio Principale | Linguaggio di Supporto | Wrapping | Framework/Librerie Principali | Considerazioni per lo Sviluppo<br> |
| ------------ | --------------------- | ---------------------- | -------- | ----------------------------- | ---------------------------------- |
| Core Sistema | Rust                  | -                      | -        | tokio (async runtime)         | Ottimizzazione per concorre        |


#Modulo-Layer-1-per-Code-Base 


---


### Istruzioni e note
Dentro lib.rs in src sono contenuti i crate
in questo caso `pub mod core` che da accesso a system_core.rs

**Attenzione**: Occorre personalizzare le informazioni riportate dalle funzioni di logging.


`system_core.rs`:

```Rust
//! Modulo core del sistema per la gestione delle operazioni principali.

use crate::config::global_config::{CoreConfig, ApplicationType}; 
use crate::memory::MemoryManager;
use log::{info, error};
use once_cell::sync::Lazy;

#[cfg(feature = "core_system")]
use crate::core_system;
#[cfg(feature = "auth")]
use crate::auth;
#[cfg(feature = "crud")]
use crate::crud;
#[cfg(feature = "api_layer")]
use crate::api_layer;
#[cfg(feature = "file_management")]
use crate::file_management;
#[cfg(feature = "task_automation")]
use crate::task_automation;
#[cfg(feature = "monitoring")]
use crate::monitoring;

#[derive(Debug)]
pub enum CoreError {
    InitializationError(String),
    ResourceAllocationError(String),
    ConfigurationError(String),
    UnsupportedOperationError(String),
    GenericError(String),
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::InitializationError(msg) => write!(f, "InitializationError: {}", msg),
            CoreError::ResourceAllocationError(msg) => write!(f, "ResourceAllocationError: {}", msg),
            CoreError::ConfigurationError(msg) => write!(f, "ConfigurationError: {}", msg),
            CoreError::UnsupportedOperationError(msg) => write!(f, "UnsupportedOperationError: {}", msg),
            CoreError::GenericError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for CoreError {}

pub struct CoreSystem {
    config: CoreConfig,
    memory_manager: MemoryManager,
}

impl CoreSystem {
    /// Crea una nuova istanza di `CoreSystem` in base alla configurazione.
    pub fn new(config: CoreConfig) -> Result<Self, CoreError> {
        info!("Inizializzazione del CoreSystem...");
        let memory_manager = MemoryManager::new(config.app_type).map_err(|e| {
            error!("Errore nell'inizializzazione del MemoryManager: {}", e);
            CoreError::InitializationError(e.to_string())
        })?;

        Ok(CoreSystem { config, memory_manager })
    }

    /// Helper per inizializzare moduli e gestire errori
    fn initialize_module<F>(&self, module_name: &str, init_func: F) -> Result<(), CoreError>
    where
        F: FnOnce() -> Result<(), CoreError>,
    {
        info!("Inizializzazione del modulo {}", module_name);
        init_func().map_err(|e| {
            error!("Errore nell'inizializzazione del modulo {}: {}", module_name, e);
            CoreError::InitializationError(format!("{} initialization failed: {}", module_name, e))
        })
    }

    /// Funzione per eseguire le operazioni in base al tipo di applicazione.
    pub fn run(&self) -> Result<(), CoreError> {
        match self.config.app_type {
            ApplicationType::WebApp => {
                info!("Configurazione per WebApp");
                self.initialize_module("Core System", || core_system::initialize())?;
                logging::monitor_module_status("Core System", true); // Modulo operativo
                self.initialize_module("Authentication", || auth::initialize())?;
                self.initialize_module("CRUD", || crud::initialize())?;
                self.initialize_module("API Layer", || api_layer::initialize())?;
                self.initialize_module("Monitoring", || monitoring::initialize())?;
                self.initialize_module("Frontend", || frontend::initialize())?;
            }
            ApplicationType::ApiBackend => {
               //Init moduli
            }
            ApplicationType::DesktopApp => {
				//Init moduli
            }
            ApplicationType::AutomationScript => {
				//Init moduli
            }
            ApplicationType::EmbeddedSystem => {
				//Init moduli
            }
        }
        Ok(())
    }
}
```

##### Possibili implementazioni

- Gestire lo **spegnimento** dei componenti e la **pulizia delle risorse** alla fine dell'esecuzione dell'applicazione (ad esempio, chiusura dei moduli o deallocazione esplicita della memoria).
- Supportare operazioni specifiche del sistema come **avvio**, **arresto**, **riavvio**, o altre operazioni di manutenzione, gestendo dinamicamente i moduli durante l'esecuzione del sistema.

### USO

```Rust
```

---

 `memory_management.rs`:

```Rust
//! Modulo per la gestione della memoria in base al tipo di applicazione.

use crate::config::global_config::{ApplicationType,MemoryConfig};
use crate::system_core::CoreError;
use log::{info};
use std::collections::VecDeque;

enum AllocationStrategy {
    Standard,
    PoolBased,
    CustomEmbedded,
}

pub struct MemoryManager {
    allocation_strategy: AllocationStrategy,
    pool: Option<VecDeque<Box<[u8]>>>, // Pool per l'allocazione basata su pool
    memory_config: MemoryConfig,  // Configurazione della memoria
}

impl MemoryManager {
    pub fn new(app_type: ApplicationType, memory_config: MemoryConfig) -> Result<Self, CoreError> {
        info!("Inizializzazione del MemoryManager...");

        let strategy = match app_type {
            ApplicationType::WebApp | ApplicationType::ApiBackend => AllocationStrategy::PoolBased,
            ApplicationType::DesktopApp => AllocationStrategy::Standard,
            ApplicationType::AutomationScript => AllocationStrategy::Standard,
            ApplicationType::EmbeddedSystem => AllocationStrategy::CustomEmbedded,
        };

        // Inizializza il pool solo se la strategia è PoolBased, utilizzando il pool_size configurato
        let pool = if let AllocationStrategy::PoolBased = strategy {
            let mut pool = VecDeque::new();
            for _ in 0..memory_config.pool_size {
                pool.push_back(vec![0u8; memory_config.buffer_size].into_boxed_slice());
            }
            Some(pool)
        } else {
            None
        };

        Ok(MemoryManager { allocation_strategy: strategy, pool, memory_config })
    }

    pub fn allocate(&mut self, size: usize) -> Result<Box<[u8]>, CoreError> {
        info!("Allocazione di {} byte di memoria...", size);
        match self.allocation_strategy {
            AllocationStrategy::Standard => {
                let buffer = vec![0u8; size].into_boxed_slice();
                Ok(buffer)
            },
            AllocationStrategy::PoolBased => {
                if let Some(ref mut pool) = self.pool {
                    if let Some(buffer) = pool.pop_front() {
                        Ok(buffer)
                    } else {
                        let buffer = vec![0u8; size].into_boxed_slice();
                        Ok(buffer)
                    }
                } else {
                    Err(CoreError::ResourceAllocationError("Pool non disponibile".to_string()))
                }
            },
            AllocationStrategy::CustomEmbedded => {
                // Usa la dimensione configurata per il buffer negli embedded
                let buffer = vec![0u8; self.memory_config.buffer_size].into_boxed_slice();
                Ok(buffer)
            },
        }
    }

    pub fn deallocate(&mut self, buffer: Box<[u8]>) -> Result<(), CoreError> {
        info!("Deallocazione della memoria...");
        match self.allocation_strategy {
            AllocationStrategy::Standard => {
                // Lascia che Rust dealloca la memoria automaticamente
                Ok(())
            },
            AllocationStrategy::PoolBased => {
                // Restituisci il buffer al pool se disponibile
                if let Some(ref mut pool) = self.pool {
                    pool.push_back(buffer);
                    Ok(())
                } else {
                    Err(CoreError::ResourceAllocationError("Pool non disponibile".to_string()))
                }
            },
            AllocationStrategy::CustomEmbedded => {
                // Gestione personalizzata per sistemi embedded
                Ok(())
            },
        }
    }
}
```

##### Possibili implementazioni

- **Pool-Based**: Implementare un meccanismo di ridimensionamento dinamico del pool.
- **CustomEmbedded**: Se necessario, puoi gestire la memoria in maniera statica o usando risorse specifiche per embedded, come allocazioni a basso livello o gestione di blocchi di memoria fissi.


### USO

```Rust

```