use solid_arx::cli::{parse_arguments,Commands};
use solid_arx::core::system_core::CoreSystem;
use solid_arx::config::{
    global_config::CoreConfig,
    memory_config::MemoryConfig,
    database_config::DatabaseConfig
};
use solid_arx::monitoring::logger::setup_logging;
use log::info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inizializza il sistema di logging
    setup_logging().expect("Errore nell'inizializzazione del sistema di logging");

    // Parsing degli argomenti passati dal CLI
    let cli_args = parse_arguments().expect("Errore nel parsing degli argomenti CLI");

    // Gestione del comando Init
    let core_config_tuple = match cli_args.command {
        Commands::Init { app_type, memory_scale, max_threads,buffer_size, pool_size } => {
            info!("Inizializzazione del progetto con i seguenti parametri:");
            info!("Tipo di applicazione: {:?}", app_type.clone());
            info!("Memory scale: {}", memory_scale);
            info!("Max threads: {}", max_threads);
            info!("Buffer size: {}", buffer_size);
            info!("Pool size: {}", pool_size);

            // Genera la configurazione basata sui parametri passati dal CLI
            let core_config = CoreConfig::new(app_type.clone(), max_threads); // Configurazione del CoreSystem{


            use solid_arx::core::memory_management::define_pool_size;
            use solid_arx::core::memory_management::define_buffer_size;
            use solid_arx::core::memory_management::define_multiplier;
            let defined_ps = define_pool_size(app_type.clone(),pool_size); // Imposta il valore di default per pool_size
            let defined_bs = define_buffer_size(app_type.clone(),buffer_size); // Imposta il valore di default per buffer_size
            let defined_ms = define_multiplier(app_type.clone(), memory_scale); // Imposta il valore di default per memory_scale

            // Configurazione della memoria
            let memory_config = MemoryConfig::new(defined_ps, defined_bs, defined_ms); // Configurazione della memoria

            info!("Configurazione della memoria completata: pool_size={}, buffer_size={}", memory_config.pool_size, memory_config.buffer_size);

            (core_config,memory_config) // Restituisci la configurazione
        },
        _ => {
            panic!("Comando non riconosciuto");
        }
    };
     
    let database_config = match cli_args.command {
        Commands::Database { disable_database, database_url, max_connections, retry_attempts, max_idle_time, connection_timeout } => {
            info!("Configurazione del database con i seguenti parametri:");
            info!("Disabilita database: {}", disable_database);
            info!("Database URL: {:?}", database_url);
            info!("Max connections: {:?}", max_connections);
            info!("Retry attempts: {:?}", retry_attempts);
            info!("Max idle time: {:?}", max_idle_time);
            info!("Connection timeout: {:?}", connection_timeout);

            DatabaseConfig::new(
                disable_database, 
                database_url, 
                max_connections, 
                etry_attempts, 
                max_idle_time, 
                onnection_timeout
            )
        },
        _ => {
            panic!("Comando non riconosciuto");
        }
    }; 

    // Inizializza il CoreSystem con la configurazione ottenuta
    let core_system = CoreSystem::new(core_config_tuple.0, core_config_tuple.1, database_config).expect("Errore nell'inizializzazione del Core System");

    // Esegui il core system
    core_system.run()?;

    Ok(())
}