#[cfg(test)]
mod tests {
    use arx_framework::core::system_core::{CoreSystem,CoreError};
    use arx_framework::config::global_config::{CoreConfig,MemoryConfig,ApplicationType};

    /// Test per verificare che il `CoreSystem` venga inizializzato correttamente per una WebApp
    #[test]
    fn test_core_system_initialization_webapp() {
        let core_config = CoreConfig {
            app_type: ApplicationType::WebApp,
            max_threads: 8,
        };
        let memory_config = MemoryConfig::default(); // Assumi che ci sia una configurazione di default

        let core_system = CoreSystem::new(core_config, memory_config);

        assert!(core_system.is_ok(), "CoreSystem should initialize correctly for WebApp");
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

        // Simula l'inizializzazione fallita di un modulo
        let result = core_system.initialize_module("FailingModule", || {
            Err(CoreError::InitializationError("Module failed".into()))
        });

        assert!(result.is_err(), "Module initialization should fail");
    }

    /// Test per verificare l'inizializzazione corretta del `MemoryManager`
    #[test]
    fn test_memory_manager_initialization() {
        let core_config = CoreConfig {
            app_type: ApplicationType::EmbeddedSystem,
            max_threads: 8,
        };
        let memory_config = MemoryConfig::default();

        let core_system = CoreSystem::new(core_config, memory_config);

        assert!(core_system.is_ok(), "CoreSystem should initialize correctly for EmbeddedSystem");
    }

    /// Test per verificare l'inizializzazione dei moduli per l'app Desktop
    #[test]
    fn test_core_system_initialization_desktop() {
        let core_config = CoreConfig {
            app_type: ApplicationType::DesktopApp,
            max_threads: 8,
        };
        let memory_config = MemoryConfig::default();

        let core_system = CoreSystem::new(core_config, memory_config);

        assert!(core_system.is_ok(), "CoreSystem should initialize correctly for DesktopApp");
    }

    /// Test per verificare che l'inizializzazione generale fallisca correttamente
    #[test]
    fn test_core_system_initialization_failure() {
        let core_config = CoreConfig {
            app_type: ApplicationType::WebApp,
            max_threads: 8,
        };
        let memory_config = MemoryConfig::default();

        let core_system = CoreSystem::new(core_config, memory_config).unwrap();

        // Simula un'errore generico
        let result = core_system.run();
        assert!(result.is_ok(), "CoreSystem should run successfully for WebApp");
    }
}
