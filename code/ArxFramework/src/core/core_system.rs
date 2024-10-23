//! Modulo core del sistema per la gestione delle operazioni principali.
//!
//! Questo modulo è responsabile della gestione e dell'orchestrazione dei vari moduli
//! in base al tipo di applicazione. Gestisce l'inizializzazione dei moduli richiesti
//! e assicura che siano eseguiti correttamente. La memoria è gestita tramite un 
//! MemoryManager, che seleziona la strategia di allocazione più adatta per il default se non è stata definita.
//!
//! Nota: Il modulo system_core.rs è statico per tutte le applicazioni tranne per i sistemi embedded.
//! Per i sistemi embedded, il modulo fornisce un'infrastruttura di base ma permette personalizzazioni
//! a livello di codice, per consentire compatibilità con hardware specifico e ambienti con risorse limitate.
//!
//! La struttura del framework garantisce la compatibilità con la maggior parte delle applicazioni standard (WebApp, API Backend, Desktop App, ecc.),
//! eseguendo in modo sicuro e centralizzato tutti i moduli, mentre per gli ambienti embedded offre flessibilità per le personalizzazioni richieste.

use crate::config::global_config::{CoreConfig, ApplicationType};
use crate::memory_management::MemoryManager;
use crate::monitoring::logger;
use log::{info, error};
use once_cell::sync::Lazy;
// Importa i modelli dalla cartella dev
use crate::crud::models::dev::*;
// Importa i modelli dalla cartella default
use crate::crud::models::default::*;

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

/// Definizione degli errori principali che possono verificarsi nel sistema core.
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

/// CoreSystem è la struttura centrale che gestisce l'intero sistema.
/// Si occupa dell'inizializzazione dei moduli e della gestione della memoria.
///
/// # Campi
/// - config: La configurazione principale del sistema, che specifica il tipo di applicazione.
/// - memory_manager: Gestore della memoria, che implementa strategie di allocazione in base al tipo di applicazione.
pub struct CoreSystem {
    config: CoreConfig,
    memory_manager: MemoryManager,
}

impl CoreSystem {
    /// Crea una nuova istanza di CoreSystem in base alla configurazione fornita.
    /// Questa funzione inizializza anche il MemoryManager.
    ///
    /// # Parametri
    /// - config: La configurazione globale che definisce il tipo di applicazione.
    ///
    /// # Ritorna
    /// Un'istanza di CoreSystem o un errore di inizializzazione (CoreError).
    pub fn new(core_config: CoreConfig, memory_config: MemoryConfig) -> Result<Self, CoreError> {
        info!("Inizializzazione del CoreSystem...");
        let memory_manager = MemoryManager::new(core_config.app_type, memory_config).map_err(|e| {
            error!("Errore nell'inizializzazione del MemoryManager: {}", e);
            CoreError::InitializationError(e.to_string())
        })?;
        info!("CoreSystem inizializzato con app_type: {:?}", config.app_type);
        Ok(CoreSystem { config, memory_manager })
    }

    /// Helper per gestire l'inizializzazione dei moduli.
    ///
    /// Questa funzione esegue una funzione di inizializzazione passata e gestisce
    /// gli eventuali errori, riportandoli nel log.
    ///
    /// # Parametri
    /// - module_name: Nome del modulo da inizializzare (usato per i log).
    /// - init_func: Funzione di inizializzazione del modulo.
    ///
    /// # Ritorna
    /// Ok(()) se il modulo è stato inizializzato correttamente, altrimenti un CoreError.
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

    /// Funzione principale che esegue le operazioni in base al tipo di applicazione.
    /// Inizializza i moduli richiesti per il tipo di applicazione configurata.
    ///
    /// # Ritorna
    /// Ok(()) se tutti i moduli sono stati inizializzati correttamente, altrimenti un CoreError.
    ///
    /// # Nota
    /// - Questa funzione utilizza la configurazione fornita in CoreConfig per determinare
    /// quali moduli devono essere inizializzati.
    pub fn run(&self) -> Result<(), CoreError> {
        match self.config.app_type {
            ApplicationType::WebApp => {
                info!("Configurazione per WebApp");
                init_module!("Authentication", || auth::initialize())?;
                init_module!("CRUD", || crud::initialize())?;
                init_module!("API Layer", || api_layer::initialize())?;
                init_module!("Frontend", || frontend::initialize())?;
            }

            ApplicationType::ApiBackend => {
                info!("Configurazione per API Backend");
                init_module!("Authentication", || auth::initialize())?;
                init_module!("CRUD", || crud::initialize())?;
                init_module!("API Layer", || api_layer::initialize())?;
            }

            ApplicationType::DesktopApp => {
                info!("Configurazione per App Desktop");
                init_module!("Authentication", || auth::initialize())?;
                init_module!("CRUD", || crud::initialize())?;
                init_module!("File Management", || file_management::initialize())?;
                init_module!("Frontend", || frontend::initialize())?;
            }

            ApplicationType::AutomationScript => {
                info!("Configurazione per Automazione e Scripting");
                init_module!("Task Automation", || task_automation::initialize())?;

                #[cfg(feature = "file_management")]
                init_module!("File Management", || file_management::initialize())?;
            }

            ApplicationType::EmbeddedSystem => {
                info!("Configurazione per Sistemi Embedded");
                // Inizializzazione di eventuali moduli specifici per sistemi embedded.
            }
        }

        Ok(())
    }
}

macro_rules! init_module {
    ($module_name:expr, $init_func:expr) => {
        {
            info!("Inizializzazione del modulo {}", $module_name);
            if let Err(e) = $init_func() {
                error!("Errore nell'inizializzazione del modulo {}: {}", $module_name, e);
                return Err(CoreError::InitializationError(format!("{} initialization failed: {}", $module_name, e)));
            }
            logger::monitor_module_status($module_name, true);
        }
    }
}

/* 
 Esempio di personalizzazione per Sistemi Embedded:

 
 // Inizializzazione personalizzata per un sistema embedded
 fn initialize_embedded_system() -> Result<(), CoreError> {
     info!("Configurazione specifica per Sistemi Embedded...");
     // Logica di inizializzazione specifica per l'hardware target
     // Potrebbe includere configurazione di GPIO, interfacce seriali, ecc.
     Ok(())
 }

 // Esempio di utilizzo:
 let result = initialize_embedded_system();
 if result.is_err() {
     error!("Errore nell'inizializzazione del sistema embedded");
 }
*/