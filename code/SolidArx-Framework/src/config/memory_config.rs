/// Configurazione della memoria per il sistema.
///
/// Questa configurazione è utilizzata per gestire le impostazioni relative alla memoria,
/// come la dimensione del pool di buffer o la dimensione del buffer nei sistemi embedded.
pub struct MemoryConfig {
    pub pool_size: usize,    // Dimensione del pool di buffer (per PoolBased)
    pub buffer_size: usize,  // Dimensione del buffer (per Embedded)
    pub memory_scale: u8,   // Scala per la dimensione della memoria dei modelli
}

/// Implementazione del valore di default per `MemoryConfig`.
///
/// Fornisce impostazioni predefinite per la configurazione della memoria.
impl Default for MemoryConfig {
    fn default() -> Self {
        MemoryConfig {
            pool_size: 10,      // Valore di default: 10 buffer per il pool
            buffer_size: 1024,   // Valore di default: 1024 byte per buffer
            memory_scale: 1,  // Valore di default: scala 1.0 per la dimensione della memoria
        }
    }
}

/// Implementazione della configurazione della memoria.
/// Questa struttura contiene le impostazioni per la dimensione del pool e dei buffer e si trova in global_config.rs
impl MemoryConfig {
    /// Crea una nuova configurazione della memoria con le impostazioni predefinite.
    pub fn new(pool_size: usize, buffer_size: usize, memory_scale: u8) -> Self {
        MemoryConfig {
            pool_size,
            buffer_size,
            memory_scale,
        }
    }
}