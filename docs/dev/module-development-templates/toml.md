
# ArxFramework/

---

# `cargo.toml`

Il .toml fa uso di `features` flags per limitare le librerie necessarie in base all'applicazione selezionata.

```toml
[package]
name = "arx_framework"
version = "0.0.0"
edition = "2021"

[lib]
name = "arx_framework"
path = "src/lib.rs"  # Path relativo per la libreria

# Definizione del binario per il CLI 
[[bin]] 
name = "arx" 
path = "src/main.rs"  # Punto di ingresso

# Dipendenze comuni a tutti i moduli
[dependencies]
chrono = "0.4"  # Per timestamp nel logging
colored = "2.0"  # Per colorare i messaggi di log
clap = { version = "3.2.0", features = ["derive"] }  # Per il parsing del CLI
once_cell = "1.17.1"  # Per la gestione di valori statici
fern = "0.6"  # Per il logging
log = "0.4"  # Sistema di logging generale

# Dipendenze opzionali (crate esterni)
auth = { path = "./auth/", optional = true }
crud = { path = "./crud/", optional = true }
api_layer = { path = "./api/", optional = true }
file_management = { path = "./file_management/", optional = true }
task_automation = { path = "./task_automation/", optional = true }
blockchain = { path = "./blockchain/", optional = true }
ml = { path = "./ml/", optional = true }

# Feature Flags
[features]
# Feature di base, inclusa sempre
default = ["core_system", "monitoring"]

# Definizione delle feature per i moduli
auth = ["auth"]
crud = ["crud"]
api_layer = ["api_layer"]
file_management = ["file_management"]
task_automation = ["task_automation"]
blockchain = ["blockchain"]
ml = ["ml"]
frontend = []  # Da abilitare solo se si implementa un modulo Rust per frontend

# Definizione delle feature per i diversi tipi di applicazioni
webapp = ["auth", "crud", "api_layer", "frontend"]
api_backend = ["auth", "crud", "api_layer"]
desktop = ["auth", "crud", "file_management", "frontend"]
automation = ["task_automation"]
embedded = []  # I sistemi embedded utilizzano solo core_system e monitoring

# Feature di base
core_system = []
monitoring = []

```


#### USO

Nei moduli si istanzia i flags dinamici dipendenti dalla configurazione del .toml 

```Rust
#[cfg(feature = "core_system")]
pub mod core_system;

#[cfg(feature = "auth")]
pub mod auth;

#[cfg(feature = "crud")]
pub mod crud;
  
#[cfg(feature = "api_layer")]
pub mod api_layer;

#[cfg(feature = "file_management")]
pub mod file_management;

#[cfg(feature = "task_automation")]
pub mod task_automation;

#[cfg(feature = "blockchain")]
pub mod blockchain;

#[cfg(feature = "ml")]
pub mod ml;

#[cfg(feature = "monitoring")]
pub mod monitoring;
```