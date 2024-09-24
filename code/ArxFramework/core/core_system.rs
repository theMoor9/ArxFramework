//! Modulo core del sistema per la gestione delle operazioni principali.

use crate::config::global_config::{CoreConfig, ApplicationType};
use crate::memory::MemoryManager;
use crate::monitoring::logger;
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
                logger::monitor_module_status("Core System", true); // Modulo operativo
                self.initialize_module("Authentication", || auth::initialize())?;
                logger::monitor_module_status("Authentication", true);
                self.initialize_module("CRUD", || crud::initialize())?;
                logger::monitor_module_status("CRUD", true);
                self.initialize_module("API Layer", || api_layer::initialize())?;
                logger::monitor_module_status("API Layer", true);
                self.initialize_module("Monitoring", || monitoring::initialize())?;
                logger::monitor_module_status("Monitoring", true);
                self.initialize_module("Frontend", || frontend::initialize())?;
                logger::monitor_module_status("Frontend", true);
            }

            ApplicationType::ApiBackend => {
                info!("Configurazione per API Backend");
                self.initialize_module("Core System", || core_system::initialize())?;
                logger::monitor_module_status("Core System", true); // Modulo operativo
                self.initialize_module("Authentication", || auth::initialize())?;
                logger::monitor_module_status("Authentication", true);
                self.initialize_module("CRUD", || crud::initialize())?;
                logger::monitor_module_status("CRUD", true);
                self.initialize_module("API Layer", || api_layer::initialize())?;
                logger::monitor_module_status("API Layer", true);
                self.initialize_module("Monitoring", || monitoring::initialize())?;
                logger::monitor_module_status("Monitoring", true);
            }

            ApplicationType::DesktopApp => {
                info!("Configurazione per App Desktop");
                self.initialize_module("Core System", || core_system::initialize())?;
                logger::monitor_module_status("Core System", true); // Modulo operativo
                self.initialize_module("Authentication", || auth::initialize())?;
                logger::monitor_module_status("Authentication", true);
                self.initialize_module("CRUD", || crud::initialize())?;
                logger::monitor_module_status("CRUD", true);
                self.initialize_module("File Management", || file_management::initialize())?;
                logger::monitor_module_status("File Management", true);
                self.initialize_module("Monitoring", || monitoring::initialize())?;
                logger::monitor_module_status("Monitoring", true);
                self.initialize_module("Frontend", || frontend::initialize())?;
                logger::monitor_module_status("Frontend", true);
            }

            ApplicationType::AutomationScript => {
                info!("Configurazione per Automazione e Scripting");
                self.initialize_module("Core System", || core_system::initialize())?;
                logger::monitor_module_status("Core System", true); // Modulo operativo
                self.initialize_module("Monitoring", || monitoring::initialize())?;
                logger::monitor_module_status("Monitoring", true);
                self.initialize_module("Task Automation", || task_automation::initialize())?;
                logger::monitor_module_status("Task Automation", true);
                #[cfg(feature = "file_management")]
                self.initialize_module("File Management", || file_management::initialize())?;
                logger::monitor_module_status("File Management", true);
            }

            ApplicationType::EmbeddedSystem => {
                info!("Configurazione per Sistemi Embedded");
                self.initialize_module("Core System", || core_system::initialize())?;
                logger::monitor_module_status("Core System", true); // Modulo operativo
                self.initialize_module("Monitoring", || monitoring::initialize())?;
                logger::monitor_module_status("Monitoring", true);
                // Inizializzazione di eventuali moduli embedded-specifici
            }
        }

        Ok(())
    }
}
