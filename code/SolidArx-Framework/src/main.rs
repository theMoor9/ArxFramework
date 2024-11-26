use solid_arx::cli::{parse_arguments,Commands};
use solid_arx::core::system_core::CoreSystem;
use solid_arx::config::{
    global_config::CoreConfig,
    memory_config::MemoryConfig,
    database_config::DatabaseType,
};
use solid_arx::monitoring::logger::setup_logging;
use log::info;

/// Funzione helper per gestire il comando Init e generare le configurazioni del Core e della Memoria
fn handle_init(
    cli_args: &solid_arx::cli::Cli,
) -> Result<(CoreConfig, MemoryConfig), Box<dyn std::error::Error>> {
    if let Commands::Init {
        app_type,
        memory_scale,
        max_threads,
        buffer_size,
        pool_size,
    } = &cli_args.command
    {
        info!("Inizializzazione del progetto:");
        info!("App Type: {:?}", app_type);
        info!("Memory Scale: {}", memory_scale);
        info!("Max Threads: {}", max_threads);
        info!("Buffer Size: {}", buffer_size);
        info!("Pool Size: {}", pool_size);

        // Configurazioni Core e Memoria
        let core_config = CoreConfig::new(app_type.clone(), *max_threads);
        let memory_config = MemoryConfig::new(
            define_pool_size(app_type.clone(), *pool_size),
            define_buffer_size(app_type.clone(), *buffer_size),
            define_multiplier(app_type.clone(), *memory_scale),
        );

        return Ok((core_config, memory_config));
    }

    Err("Comando Init non trovato".into())
}

/// Funzione helper per gestire il comando Database e generare la configurazione del database
fn handle_database(
    cli_args: &solid_arx::cli::Cli,
) -> Result<DatabaseType, Box<dyn std::error::Error>> {
    if let Commands::Database {
        database_type,
        database_url,
        max_connections,
        retry_attempts,
        max_idle_time,
        connection_timeout,
    } = &cli_args.command
    {
        info!("Configurazione del database:");
        info!("Database: {}", database_type_reference);
        info!("Database URL: {:?}", database_url);
        info!("Max Connections: {:?}", max_connections);
        info!("Retry Attempts: {:?}", retry_attempts);
        info!("Max Idle Time: {:?}", max_idle_time);
        info!("Connection Timeout: {:?}", connection_timeout);

        return Ok(DatabaseType::new(
            database_type_reference,
            database_url,
            max_connections,
            retry_attempts,
            max_ile_time,
            connection_timeout,
        ));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inizializza il sistema di logging
    setup_logging().expect("Errore nell'inizializzazione del sistema di logging");

    // Parsing degli argomenti passati dal CLI
    let cli_args = parse_arguments().expect("Errore nel parsing degli argomenti CLI");

    // Gestione dei comandi
    if let Commands::Help { message } = cli_args.command {
        println!("{}", message);
        return Ok(()); // Terminazione immediata per Help
    }

    // Ottenere le configurazioni (Init, Database) con valori di default
    let (core_config, memory_config) = handle_init(&cli_args)?;
    let database_config = handle_database(&cli_args)?;

    // Inizializza il CoreSystem con la configurazione ottenuta
    let core_system = CoreSystem::new(
        core_config, 
        memory_config, 
        database_config
        ).expect("Errore nell'inizializzazione del Core System");

    // Esegui il core system
    core_system.run()?;

    Ok(())
}

