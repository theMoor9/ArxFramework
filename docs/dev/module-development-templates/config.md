# config/

---

# `global_config.rs`

Il `global_config.rs` è il modulo responsabile del raggruppamento di tutte le variabili di configurazione degli altri moduli ed è definito tramite il CLI.

```Rust
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
```

---
# `memory_config.rs`

```Rust
pub struct MemoryConfig {
    pub pool_size: usize,    // Dimensione del pool di buffer (per PoolBased)
    pub buffer_size: usize,  // Dimensione del buffer (per Embedded)
    pub memory_scale: u8,   // Scala per la dimensione della memoria dei modelli
    }
    
pub struct MemoryConfig {
    pub pool_size: usize,    // Dimensione del pool di buffer (per PoolBased)
    pub buffer_size: usize,  // Dimensione del buffer (per Embedded)
    pub memory_scale: u8,   // Scala per la dimensione della memoria dei modelli
}

impl Default for MemoryConfig {
    fn default() -> Self {
        MemoryConfig {
            pool_size: 10,      // Valore di default: 10 buffer per il pool
            buffer_size: 1024,   // Valore di default: 1024 byte per buffer
            memory_scale: 1,  // Valore di default: scala 1.0 per la dimensione della memori
        }
    }
}

impl MemoryConfig {
    pub fn new(pool_size: usize, buffer_size: usize, memory_scale: u8) -> Self {
        MemoryConfig {
            pool_size,
            buffer_size,
            memory_scale,
        }
    }
}
```

---
# `network_config.rs`

```Rust
use std::time::Duration;
use log::{info, error};

// Configurazione per connessione al database
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub database_url: String,
    pub max_connections: Option<u32>,
    pub retry_attempts: Option<u32>,
    pub max_idle_time: Option<Duration>,       // Durata massima per mantenere una connessione inattiva
    pub connection_timeout: Option<Duration>,  // Tempo massimo di attesa per stabilire una connessione
}

// Configurazione per i diversi tipi di database
pub enum DatabaseType {
    PostgreSQL(ConnectionConfig),
    SQLite(ConnectionConfig),
    MongoDB(ConnectionConfig),
}

impl DatabaseType {
    pub fn new() -> Result<Self, &'static str> {
        #[cfg(feature = "webapp")] // flag per applicazione
        return Ok(DatabaseType::PostgreSQL(ConnectionConfig {
            // Implementazioni di configurazione
        }));
        
		// altr implementazioni per applicazione
        
        Err("Database configuration not set for the application")
    }

    // Funzione di log per monitorare lo stato della connessione
    pub fn log_status(&self) {
        match self {
            // Logging
        }
    }
}

```

---
