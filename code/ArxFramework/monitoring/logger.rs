use fern::Dispatch;
use chrono::Local;
use log::{info, warn, error};

/// Configura il sistema di logging usando `fern`.
///
/// I log vengono scritti sia sulla console che su file.
/// I messaggi di log includono timestamp, il livello di log (info, warning, error, etc.), 
/// e il target del log (nome del modulo da cui proviene il log).
///
/// # Ritorna
/// * `Ok(())` se l'inizializzazione ha successo.
/// * `Err(fern::InitError)` se ci sono problemi nell'inizializzazione del logging.
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
        .level(log::LevelFilter::Info)  // Livello di log globale, può essere configurato
        .chain(std::io::stdout())       // Scrittura del log nella console
        .chain(fern::log_file("monitoring/logs/arx_framework.log")?)  // Scrittura su file
        .apply()?;  // Applica la configurazione
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

/// Metodo standard per aggiungere nuovi tipi di messaggi di log.
/// 
/// Se sei un dev a cui piace "incespicare" e personalizzare tutto, puoi aggiungere nuovi livelli
/// o tipi di messaggi di log seguendo lo schema esistente.
/// 
/// # Esempio:
/// Per creare un nuovo tipo di messaggio di log:
/// 
/// ```rust
/// use log::trace;
///
/// trace!("Traccia dettagliata del funzionamento del modulo 'Example'...");
/// ```
/// 
/// Puoi aggiungere livelli come `trace`, `debug` o persino crearne di nuovi se ti senti avventuroso!
/// Basta seguire lo schema di `info!`, `warn!` e `error!` per estendere le funzionalità del logger.
