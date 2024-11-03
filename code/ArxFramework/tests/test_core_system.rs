#[cfg(test)]
mod tests {
    use arx_framework::core::system_core::{CoreSystem,CoreError};
    use arx_framework::config::global_config::{CoreConfig,MemoryConfig,ApplicationType};

    /// Test per verificare che il `CoreSystem` venga inizializzato correttamente per una WebApp
    #[test]
    fn test_core_system_initialization_webapp() {

        /// Sezione che si configura tramite CLI inizializzata staticamente per il test
        let core_config = CoreConfig {
            app_type: ApplicationType::WebApp,
            max_threads: 8,
        };

        /// Configurazione di default per la memoria e unittest completo per global_config.rs 
        /// e implementazione di MemoryConfig in memory_management.rs
        let mut memory_config = MemoryConfig::default(); // Assumo che ci sia una configurazione di default unica implementazione di global_config.rs
        memory_config = MemoryConfig::new(10, 1024, 1); // Assumo che ci sia una funzione di istanza in memory_management.rs

        /// Inizializza il `CoreSystem` con la configurazione per una WebApp
        /// Comprende il testing di MemoryManager new()
        let core_system = CoreSystem::new(core_config, memory_config);

        /// Esegue il `CoreSystem` e verifica che sia stato inizializzato correttamente
        let result = core_system.run();

        assert!(core_system.is_ok(), "CoreSystem should initialize correctly for WebApp");
        assert!(result.is_ok(), "CoreSystem should inizialize other modules correctly for WebApp");
    }

    /// Test per verificare l'inizializzazione fallita di un modulo
    #[test]
    fn test_module_initialization_failure() {
        let core_config = CoreConfig {
            app_type: ApplicationType::ApiBackend,
            max_threads: 8,
        };
        let memory_config = MemoryConfig::default();

        let core_system = CoreSystem::new(core_config, memory_config).unwrap();

        // Simulo l'inizializzazione fallita di un modulo
        let result = core_system.initialize_module("FailingModule", || {
            Err(CoreError::InitializationError("Module failed".into()))
        });

        assert!(result.is_err(), "Module initialization should fail");
    }


}
