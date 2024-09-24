pub enum ApplicationType{
	WebApp,
	ApiBackend,
	DesktopApp,
	AutomationScript,
	EmbeddedSystem,
}

/// Configurazione globale del sistema.
pub struct CoreConfig {
    pub app_type: ApplicationType,
    pub max_threads: usize,
}

/// Configurazione della memoria per il sistema.
pub struct MemoryConfig {
    pub pool_size: usize,    // Dimensione del pool di buffer (per PoolBased)
    pub buffer_size: usize,  // Dimensione del buffer (per Embedded)
}

impl Default for MemoryConfig {
    fn default() -> Self {
        MemoryConfig {
            pool_size: 10,      // Valore di default: 10 buffer per il pool
            buffer_size: 1024,   // Valore di default: 1024 byte per buffer
        }
    }
}
