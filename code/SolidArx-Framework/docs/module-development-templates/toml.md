
```toml
# Controlla i percorsi

[package]
name = "arx_framework"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"  # Oppure path relativo al .toml

# Dipendenze comuni a tutti i moduli
[dependencies]
chrono = "0.4"
colored = "2.0"
clap = { version = "3.2.0", features = ["derive"] }
once_cell = "1.17.1"

# Dipendenze opzionali (crate esterni)
auth = { path = "./auth/", optional = true }
crud = { path = "./crud/", optional = true }
api_layer = { path = "./api/", optional = true }
file_management = { path = "./file_management/", optional = true }
task_automation = { path = "./task_automation/", optional = true }
blockchain = { path = "./blockchain/", optional = true }
ml = { path = "./ml/", optional = true }
#frontend = { path = "./frontend/", optional = true } # Se `frontend` è un crate Rust leva commento

# Feature Flags
[features]
# Feature di base inclusa sempre
default = ["core_system", "monitoring"]

# Definizione delle feature per le dipendenze opzionali 
auth = ["auth"] 
crud = ["crud"] 
api_layer = ["api_layer"] 
file_management = ["file_management"] 
task_automation = ["task_automation"] 
blockchain = ["blockchain"] 
ml = ["ml"] 
frontend = ["frontend"]

# Definizione delle feature per le diverse applicazioni
webapp = ["auth", "crud", "api_layer", "frontend"]
api_backend = ["auth", "crud", "api_layer"]
desktop = ["auth", "crud", "file_management", "frontend"]
automation = ["task_automation"]
embedded = []  # Sistemi embedded usano solo core e monitoring

# Definizione delle feature di base
core_system = []
monitoring = []
```