# monitoring/
| Modulo                     | Linguaggio Principale | Linguaggio di Supporto | Wrapping | Framework/Librerie Principali |
| -------------------------- | --------------------- | ---------------------- | -------- | ----------------------------- |
| Monitoraggio e logging     | Rust                  | -                      | -        | tracing, prometheus           |

#Modulo-Layer-1-per-Code-Base 

---

### Istruzioni e note

Il logger

**Attenzione**: Occorre personalizzare le informazioni riportate dalle funzioni di logging.

```Rust
// monitoring/logger.rs

use fern::Dispatch;
use chrono::Local;
use log::{info, warn, error};

/// Configura il sistema di logging usando `fern`.
///
/// I log vengono scritti sia sulla console che su file.
/// I messaggi di log includono timestamp e livelli di log (info, warning, error, etc.).
///
/// # Ritorna
/// * `Ok(())` se l'inizializzazione ha successo.
/// * `Err(fern::InitError)` se ci sono problemi nell'inizializzazione.
pub fn setup_logging() -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info) // Livello di log globale
        .chain(std::io::stdout())      // Log in console
        .chain(fern::log_file("monitoring/logs/arx_framework.log")?) // Log in file
        .apply()?;
    Ok(())
}

/// Funzione per monitorare lo stato dei moduli.
///
/// # Argomenti
/// - `module_name`: Nome del modulo da monitorare.
/// - `status`: Stato attuale del modulo (true = operativo, false = errore).
pub fn monitor_module_status(module_name: &str, status: bool) {
    if status {
        info!("Il modulo '{}' è operativo", module_name);
    } else {
        error!("Errore nel modulo '{}'", module_name);
    }
}

```

### USO

```Rust
```