//! Modulo core del sistema per la gestione delle operazioni principali.
//!
//! Questo modulo è responsabile della gestione e dell'orchestrazione dei vari moduli
//! in base al tipo di applicazione. Gestisce l'inizializzazione dei moduli richiesti
//! e assicura che siano eseguiti correttamente. La memoria è gestita tramite un 
//! `MemoryManager`, che seleziona la strategia di allocazione più adatta.
//!
//! Nota: Il modulo `system_core.rs` è statico per tutte le applicazioni tranne per i sistemi embedded.
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

/// `CoreSystem` è la struttura centrale che gestisce l'intero sistema.
/// Si occupa dell'inizializzazione dei moduli e della gestione della memoria.
///
/// # Campi
/// - `config`: La configurazione principale del sistema, che specifica il tipo di applicazione.
/// - `memory_manager`: Gestore della memoria, che implementa strategie di allocazione in base al tipo di applicazione.
pub struct CoreSystem {
    config: CoreConfig,
    memory_manager: MemoryManager,
}

impl CoreSystem {
    /// Crea una nuova istanza di `CoreSystem` in base alla configurazione fornita.
    /// Questa funzione inizializza anche il `MemoryManager`.
    ///
    /// # Parametri
    /// - `config`: La configurazione globale che definisce il tipo di applicazione.
    ///
    /// # Ritorna
    /// Un'istanza di `CoreSystem` o un errore di inizializzazione (`CoreError`).
    pub fn new(config: CoreConfig) -> Result<Self, CoreError> {
        info!("Inizializzazione del CoreSystem...");
        let memory_manager = MemoryManager::new(config.app_type).map_err(|e| {
            error!("Errore nell'inizializzazione del MemoryManager: {}", e);
            CoreError::InitializationError(e.to_string())
        })?;

        Ok(CoreSystem { config, memory_manager })
    }

    /// Helper per gestire l'inizializzazione dei moduli.
    ///
    /// Questa funzione esegue una funzione di inizializzazione passata e gestisce
    /// gli eventuali errori, riportandoli nel log.
    ///
    /// # Parametri
    /// - `module_name`: Nome del modulo da inizializzare (usato per i log).
    /// - `init_func`: Funzione di inizializzazione del modulo.
    ///
    /// # Ritorna
    /// `Ok(())` se il modulo è stato inizializzato correttamente, altrimenti un `CoreError`.
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
    /// `Ok(())` se tutti i moduli sono stati inizializzati correttamente, altrimenti un `CoreError`.
    ///
    /// # Nota
    /// - Questa funzione utilizza la configurazione fornita in `CoreConfig` per determinare
    /// quali moduli devono essere inizializzati.
    pub fn run(&self) -> Result<(), CoreError> {
        match self.config.app_type {
            ApplicationType::WebApp => {
                info!("Configurazione per WebApp");
                self.initialize_module("Authentication", || auth::initialize())?;
                logger::monitor_module_status("Authentication", true);
                self.initialize_module("CRUD", || crud::initialize())?;
                logger::monitor_module_status("CRUD", true);
                self.initialize_module("API Layer", || api_layer::initialize())?;
                logger::monitor_module_status("API Layer", true);
                self.initialize_module("Frontend", || frontend::initialize())?;
                logger::monitor_module_status("Frontend", true);
            }

            ApplicationType::ApiBackend => {
                info!("Configurazione per API Backend");
                self.initialize_module("Authentication", || auth::initialize())?;
                logger::monitor_module_status("Authentication", true);
                self.initialize_module("CRUD", || crud::initialize())?;
                logger::monitor_module_status("CRUD", true);
                self.initialize_module("API Layer", || api_layer::initialize())?;
                logger::monitor_module_status("API Layer", true);
            }

            ApplicationType::DesktopApp => {
                info!("Configurazione per App Desktop");
                self.initialize_module("Authentication", || auth::initialize())?;
                logger::monitor_module_status("Authentication", true);
                self.initialize_module("CRUD", || crud::initialize())?;
                logger::monitor_module_status("CRUD", true);
                self.initialize_module("File Management", || file_management::initialize())?;
                logger::monitor_module_status("File Management", true);
                self.initialize_module("Frontend", || frontend::initialize())?;
                logger::monitor_module_status("Frontend", true);
            }

            ApplicationType::AutomationScript => {
                info!("Configurazione per Automazione e Scripting");
                self.initialize_module("Task Automation", || task_automation::initialize())?;
                logger::monitor_module_status("Task Automation", true);
                #[cfg(feature = "file_management")]
                self.initialize_module("File Management", || file_management::initialize())?;
                logger::monitor_module_status("File Management", true);
            }

            ApplicationType::EmbeddedSystem => {
                info!("Configurazione per Sistemi Embedded");
                // Inizializzazione di eventuali moduli specifici per sistemi embedded.
            }
        }

        Ok(())
    }
}

/// Esempio di personalizzazione per Sistemi Embedded:
///
/// ```rust
/// // Inizializzazione personalizzata per un sistema embedded
/// fn initialize_embedded_system() -> Result<(), CoreError> {
///     info!("Configurazione specifica per Sistemi Embedded...");
///     // Logica di inizializzazione specifica per l'hardware target
///     // Potrebbe includere configurazione di GPIO, interfacce seriali, ecc.
///     Ok(())
/// }
///
/// // Esempio di utilizzo:
/// let result = initialize_embedded_system();
/// if result.is_err() {
///     error!("Errore nell'inizializzazione del sistema embedded");
/// }
/// ```