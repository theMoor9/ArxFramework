#[cfg(test)]
mod tests {
    use arx_framework::core::system_core::{CoreSystem};
    use arx_framework::config::global_config::{CoreConfig,MemoryConfig,ApplicationType};
    use arx_framework::core::memory_management::define_pool_size;
    use arx_framework::core::memory_management::define_buffer_size;
    use arx_framework::core::memory_management::define_multiplier;

    /// Test per verificare che il `CoreSystem` venga inizializzato correttamente per una WebApp
    #[test]
    fn test_core_system_initialization_webapp() {
        // Sezione che si configura tramite CLI inizializzata staticamente per il test
        let core_config = CoreConfig {
            app_type: ApplicationType::AutomationScript,
            max_threads: 8,
        };

        // Configurazione di default per la memoria e unittest completo per global_config.rs 
        // e implementazione di MemoryConfig in memory_management.rs
        let mut memory_config = MemoryConfig::default(); // Assumo che ci sia una configurazione di default unica implementazione di global_config.rs
        assert_eq!(memory_config.pool_size, 10, "Default pool size should be 10");

        memory_config = MemoryConfig::new(
            define_pool_size(core_config.app_type.clone(),10), 
            define_buffer_size(core_config.app_type.clone(),1024), 
            define_multiplier(core_config.app_type.clone(),0) // 0 Valore di default
        ); // Assumo che ci sia una funzione di istanza in memory_management.rs

        // Inizializza il `CoreSystem` con la configurazione per una WebApp
        // Comprende il testing di MemoryManager new()
        let core_system = CoreSystem::new(core_config, memory_config);

        assert!(core_system.is_ok(), "CoreSystem should initialize correctly for WebApp");

        // Esegue il `CoreSystem` e verifica che sia stato inizializzato correttamente
        let result = core_system.unwrap().run();

        
        assert!(result.is_ok(), "CoreSystem should inizialize other modules correctly for WebApp");
    }

    /// Test per verificare l'inizializzazione fallita di un modulo
    #[test]
    fn test_module_initialization_failure() {
        let core_config = CoreConfig {
            app_type: ApplicationType::WebApp,
            max_threads: 8,
        };
        let memory_config = MemoryConfig::new(
            define_pool_size(core_config.app_type.clone(),0), // limite  valore massimo
            define_buffer_size(core_config.app_type.clone(),0), // Valore di default
            define_multiplier(core_config.app_type.clone(),u8::MAX) // limite valore massimo
        ); 

        assert_eq!(memory_config.pool_size, 150 * 1024 * 1024);// Controllo che il valore sia stato impostato correttamente
        assert_eq!(memory_config.buffer_size, 16 * 1024 * 1024 ); // ""  ""
        assert_eq!(memory_config.memory_scale,u8::MAX ); // ""  ""

        // Tentativo di eccedenza valori massimi i campi di CoreSystem
        let core_system = CoreSystem::new(core_config, memory_config);

        let result = core_system.unwrap().run();

        assert!(result.is_err(), "Module initialization should fail");
    }
}
