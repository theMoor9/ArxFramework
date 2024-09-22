# Tameplates

---

## Moduli Layer 1 per Code Base 
### Istruzioni e note
Dentro lib.rs in src sono contenuti i crate
in questo caso `pub mod cli` che da accesso a cli.rs

**Attenzione**: Occorre personalizzare i messaggi del CLI e le informazioni riportate dalle funzioni di logging.

`cli.rs`:

```Rust
use clap::{Parser, ValueEnum};
use once_cell::sync::Lazy;
use crate::config::global_config::{CoreConfig, ApplicationType};
use crate::monitoring::logger::{Log, LogLevel};

/// Logger globale per il modulo CLI.
static LOGGER: Lazy<Log> = Lazy::new(|| {
    Log::new(LogLevel::Info, "cli", true).expect("Impossibile inizializzare il logger")
});


/// Configurazione del CLI per ArxFramework.
///
/// Questo struct definisce gli argomenti della linea di comando che l'utente può fornire
/// per configurare il core del framework.
#[derive(Parser, Debug)]
#[clap(
    name = "ArxFramework CLI",
    version = "1.0",
    author = "Your Name",
    about = "Configura il sistema core di ArxFramework"
)]
pub struct CliConfig {
    /// Imposta il tipo di applicazione.
    #[clap(long, value_enum, help = "Specifica il tipo di applicazione")]
    app_type: AppTypeArg,

    /// Imposta il numero massimo di thread.
    #[clap(long, default_value = "4", help = "Numero massimo di thread da utilizzare")]
    max_threads: usize,
}

/// Enumerazione dei tipi di applicazione supportati.
///
/// Questa enumerazione viene utilizzata per mappare gli argomenti forniti dall'utente
/// ai tipi di applicazione definiti nel core del framework.
#[derive(ValueEnum, Clone, Debug)]
pub enum AppTypeArg {
    /// Applicazione Web
    Webapp,
    /// Backend API
    Api,
    /// Applicazione Desktop
    Desktop,
    /// Script di Automazione
    Automation,
    /// Sistema Embedded
    Embedded,
}

impl From<AppTypeArg> for ApplicationType {
    /// Converte un `AppTypeArg` in un `ApplicationType` del core del framework.
    ///
    /// # Argomenti
    ///
    /// * `app_type_arg` - Il tipo di applicazione specificato dall'utente.
    ///
    /// # Ritorna
    ///
    /// Un valore di `ApplicationType` corrispondente.
    fn from(app_type_arg: AppTypeArg) -> Self {
        match app_type_arg {
            AppTypeArg::Webapp => ApplicationType::WebApp,
            AppTypeArg::Api => ApplicationType::ApiBackend,
            AppTypeArg::Desktop => ApplicationType::DesktopApp,
            AppTypeArg::Automation => ApplicationType::AutomationScript,
            AppTypeArg::Embedded => ApplicationType::EmbeddedSystem,
        }
    }
}

/// Parsea gli argomenti della linea di comando e restituisce la configurazione core.
///
/// Questa funzione legge gli argomenti forniti dall'utente tramite la linea di comando,
/// li valida e li converte in una struttura `CoreConfig` utilizzata dal core del framework.
///
/// # Ritorna
///
/// Un'istanza di `CoreConfig` contenente le impostazioni specificate dall'utente.
pub fn parse_core_config_cli() -> CoreConfig {
    LOGGER.info("Parsing command line arguments for core configuration.");

    // Parsea gli argomenti della linea di comando
    let cli_config = CliConfig::parse();

    // Logga le impostazioni ricevute
    LOGGER.info(&format!("Application Type: {:?}", cli_config.app_type));
    LOGGER.info(&format!("Max Threads: {}", cli_config.max_threads));

    // Crea e restituisce la configurazione core
    CoreConfig {
        app_type: cli_config.app_type.into(),
        max_threads: cli_config.max_threads,
    }
}
```