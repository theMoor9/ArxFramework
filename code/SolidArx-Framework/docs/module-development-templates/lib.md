
```Rust
use crate::monitoring::logger::{Log, LogLevel}; 
use once_cell::sync::Lazy; 
use crate::core_system::{CoreError, CoreSystem}; 
use crate::config::global_config::CoreConfig;

static LOGGER: Lazy<Log> = Lazy::new(|| {
    Log::new(LogLevel::Info, "lib", true).expect("Impossibile inizializzare il logger")
});


// Includi i moduli condizionalmente
#[cfg(feature = "core_system")]
extern crate core;

#[cfg(feature = "monitoring")]
extern crate monitoring;

#[cfg(feature = "auth")]
extern crate auth;

#[cfg(feature = "crud")]
extern crate crud;

#[cfg(feature = "api_layer")]
extern crate api;

#[cfg(feature = "file_management")]
extern crate  file_management;

#[cfg(feature = "task_automation")]
extern crate  task_automation;

#[cfg(feature = "blockchain")]
extern crate  blockchain;

#[cfg(feature = "ml")]
extern crate  ml;

// Funzione per inizializzare tutti i moduli attivi
pub fn initialize_modules() -> Result<(), CoreError> {
    // Inizializza condizionalmente i moduli in base alle feature attive
    #[cfg(feature = "core_system")]
    {
        LOGGER.info("Inizializzazione del Core System");
        core_system::initialize().map_err(|e| {
            LOGGER.error(&format!("Errore nell'inizializzazione del Core System: {}", e));
            e
        })?;
    }

    #[cfg(feature = "monitoring")]
    {
        LOGGER.info("Inizializzazione del Monitoring");
        monitoring::initialize().map_err(|e| {
            LOGGER.error(&format!("Errore nell'inizializzazione del Monitoring: {}", e));
            e
        })?;
    }

    #[cfg(feature = "auth")]
    {
        LOGGER.info("Inizializzazione del modulo Auth");
        auth::initialize().map_err(|e| {
            LOGGER.error(&format!("Errore nell'inizializzazione di Auth: {}", e));
            e
        })?;
    }

    // Ripeti per gli altri moduli...

    Ok(())
}

```
