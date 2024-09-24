// src/lib.rs

use log::{info, error};
use crate::core_system::CoreError;

#[cfg(feature = "core_system")]
pub mod core_system;

#[cfg(feature = "auth")]
pub mod auth;

#[cfg(feature = "crud")]
pub mod crud;

#[cfg(feature = "api_layer")]
pub mod api_layer;

#[cfg(feature = "file_management")]
pub mod file_management;

#[cfg(feature = "task_automation")]
pub mod task_automation;

#[cfg(feature = "blockchain")]
pub mod blockchain;

#[cfg(feature = "ml")]
pub mod ml;

#[cfg(feature = "monitoring")]
pub mod monitoring;

/// Inizializza tutti i moduli attivi.
///
/// Questa funzione inizializza tutti i moduli che sono stati attivati tramite le feature.
/// Include anche l'inizializzazione del sistema di logging.
pub fn initialize_modules() -> Result<(), CoreError> {
    #[cfg(feature = "monitoring")]
    {
        // Inizializza il logger all'avvio
        monitoring::logger::setup_logging().map_err(|e| {
            eprintln!("Errore nell'inizializzazione del Logger: {}", e);
            CoreError::InitializationError(format!("Logger initialization failed: {}", e))
        })?;
        info!("Logger inizializzato");
    }

    #[cfg(feature = "core_system")]
    {
        info!("Inizializzazione del Core System");
        core_system::initialize().map_err(|e| {
            error!("Errore nell'inizializzazione del Core System: {}", e);
            e
        })?;
    }

    #[cfg(feature = "auth")]
    {
        info!("Inizializzazione del modulo Auth");
        auth::initialize().map_err(|e| {
            error!("Errore nell'inizializzazione di Auth: {}", e);
            e
        })?;
    }

    #[cfg(feature = "crud")]
    {
        info!("Inizializzazione del modulo CRUD");
        crud::initialize().map_err(|e| {
            error!("Errore nell'inizializzazione di CRUD: {}", e);
            e
        })?;
    }

    Ok(())
}