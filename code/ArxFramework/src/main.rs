use ArxFramework::cli::{parse_arguments,Commands};
use ArxFramework::core::system_core::CoreSystem;
use ArxFramework::config::global_config{CoreConfig,MemoryConfig};
use ArxFramework::monitoring::logger::setup_logging;
use log::info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inizializza il sistema di logging
    setup_logging().expect("Errore nell'inizializzazione del sistema di logging");

    // Parsing degli argomenti passati dal CLI
    let cli_args = parse_arguments().expect("Errore nel parsing degli argomenti CLI");

    // Gestione del comando Init
    let config_tuple = match cli_args.command {
        Commands::Init { app_type, memory_scale, max_threads } => {
            info!("Inizializzazione del progetto con i seguenti parametri:");
            info!("Tipo di applicazione: {:?}", app_type);
            info!("Memory scale: {}", memory_scale);
            info!("Max threads: {}", max_threads);

            // Genera la configurazione basata sui parametri passati dal CLI
            let core_config = CoreConfig {
                app_type,
                max_threads,
            };

            // Configurazione della memoria
            let memory_config = MemoryConfig {
                pool_size: calculate_pool_size(app_type, memory_scale),
                buffer_size: calculate_buffer_size(app_type, memory_scale),
                memory_scale,
            };

            info!("Configurazione della memoria completata: pool_size={}, buffer_size={}", memory_config.pool_size, memory_config.buffer_size);

            (core_config,memory_config) // Restituisci la configurazione
        },
        Commands::Help => {
            println!("HELP");
            println!("arx init --app_type webapp   # Init app");
            // Lista delle applicazioni
            println!("Application types:");
            println!("WebApp");
            println!("ApiBackend");
            println!("DesktopApp");
            println!("AutomationScript");
            println!("EmbeddedSystem");
            println!("arx --memory_scale 1           # Set Memory Multiplier, DEFAULT: 1");
            println!("arx --max_threads 8            # Set Max Threads, DEFAULT: 8");
            println!("arx --version                  # Display Version");
            return Ok(());
        }
    };

    // Inizializza il CoreSystem con la configurazione ottenuta
    let core_system = CoreSystem::new(config_tuple.0, config_tuple.1).expect("Errore nell'inizializzazione del Core System");

    // Esegui il core system
    core_system.run()?;

    Ok(())
}