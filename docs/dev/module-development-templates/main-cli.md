# Main - CLI

**`main.rs`**, viene utilizzato per avviare l'intera applicazione/framework. Il ruolo del **CLI** è di configurare l'ambiente e passare i parametri al **`main`**, che a sua volta configura il resto dei moduli.

### Struttura del Processo:

1. **CLI**: Raccoglie le configurazioni (tipo di applicazione, numero di thread, ecc.) e le passa al **`main.rs`**.
    
2. **`main.rs`**: Avvia l'applicazione in base ai parametri ricevuti, passando la palla al **`system_core`**.
    
3. **`system_core`**: Inizializza tutti i moduli necessari (come `auth`, `crud`, `api_layer`, ecc.) e gestisce l'intera logica del framework.
    

### Funzionamento:

- Il **`main.rs`** gestisce il **punto di ingresso** unico e coordina tutte le operazioni iniziali.
- Il **CLI** è un'**interfaccia** che consente all'utente di scegliere come configurare e avviare il sistema.
- **`system_core`** esegue la logica interna e gestisce il ciclo di vita dell'applicazione, basandosi sulle scelte effettuate tramite il CLI.

Il CLI quindi:

1. **Sarà un binario separato** che l'utente può installare e richiamare da terminale.
2. **Gestirà la configurazione dell'applicazione** (come i parametri dell'applicazione e la selezione dei moduli) e avvierà il framework in base a tali parametri.
3. **Richiamerà il  `system core` del framework o altre funzioni importanti del framework** con le giuste variabili di configurazione, avviando il sistema nella cartella progetto.



# src/`cli.rs`
Attraverso il CLI saranno personalizzati i seguenti aspetti:

- Tipo di Applicazione.
- Il setup del .toml per l'nclusione dei moduli strettamente necessari in relazione al tipo di applicazione scelta.
- Numero di threads per il core.
- Le configurazioni di memoria come il pool size e il buffer size.


```Rust
use clap::{Parser, Subcommand};
use crate::config::global_config::ApplicationType; 

/// CLI per ArxFramework
#[derive(Parser)]
#[command(name = "Arx")]
#[command(about = "CLI per il framework Arx", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Comandi supportati da Arx
#[derive(Subcommand)]
pub enum Commands {
    /// Inizializza un nuovo progetto con un tipo di applicazione specifico
    Init {
        /// Il tipo di applicazione da inizializzare (WebApp, ApiBackend, DesktopApp, etc.)
        #[arg(short, long)]
        app_type: ApplicationType,
    },
    /// Mostra informazioni sulla versione del framework
    Version, // Implementa Version
    Help, // Implementa Help
}

/// Parsing degli argomenti e ritorno della configurazione CLI
pub fn parse_arguments() -> Result<Cli, clap::Error> {
    Cli::try_parse()
}
```


# src/`main.rs`

#### Necessario: 

1. Personalizzare `max_threads` per ogni app specificando nel documento finale in [[ArxFramework/docs/module-tameplates/main-cli|main-cli]] la procedura per personalizzazioni future
2. Implementare la sezione `Commands::Version` collegandolo alle informazioni del `.toml`
3. Implementare la sezione `Commands::Help` creando una formattazione adeguata per i messaggi d'uso, includendo  in [[ArxFramework/docs/module-tameplates/main-cli|main-cli]] la procedura per personalizzazioni future


```Rust
use crate::core_system::CoreSystem;
use crate::cli::{parse_arguments, Cli, Commands}; // Assicurati di importare il CLI
use crate::config::global_config::ApplicationType;
use log::info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inizializza il sistema di logging
    monitoring::logger::setup_logging().expect("Errore nell'inizializzazione del sistema di logging");

    // Parsing degli argomenti passati dal CLI
    let cli_args = parse_arguments().expect("Errore nel parsing degli argomenti CLI");

    // Gestione del comando Init
    let config = match cli_args.command {
        Commands::Init { app_type } => {
            // Inizializzazione in base al tipo di applicazione scelto
            match app_type {
                ApplicationType::WebApp => {
                    info!("Inizializzazione del progetto WebApp");
                    // Genera la configurazione per una WebApp
                    CoreConfig {
                        app_type: ApplicationType::WebApp,
                        max_threads: 4, // esempio di configurazione
                    }
                }
                ApplicationType::ApiBackend => {
                    info!("Inizializzazione del progetto ApiBackend");
                    CoreConfig {
                        app_type: ApplicationType::ApiBackend,
                        max_threads: 8, // esempio
                    }
                }
                ApplicationType::DesktopApp => {
                    info!("Inizializzazione del progetto DesktopApp");
                    CoreConfig {
                        app_type: ApplicationType::DesktopApp,
                        max_threads: 2, // esempio
                    }
                }
                ApplicationType::AutomationScript => {
                    info!("Inizializzazione del progetto AutomationScript");
                    CoreConfig {
                        app_type: ApplicationType::AutomationScript,
                        max_threads: 2, // esempio
                    }
                }
                ApplicationType::EmbeddedSystem => {
                    info!("Inizializzazione del progetto EmbeddedSystem");
                    CoreConfig {
                        app_type: ApplicationType::EmbeddedSystem,
                        max_threads: 1, // esempio
                    }
                }
            }
        }
        // Implementazione per Version e Help
        Commands::Version => {
            println!("ArxFramework versione 1.0.0");
            return Ok(()); // Esce dall'applicazione
        }
        Commands::Help => {
            println!("Esempio di utilizzo: arx init --AppType webapp");
            return Ok(());
        }
    };

    // Inizializza il CoreSystem con la configurazione ottenuta
    let core_system = CoreSystem::new(config).expect("Errore nell'inizializzazione del Core System");

    // Esegui il core system
    core_system.run()?;

    Ok(())
}
```