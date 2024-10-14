
# ArxFramework/

---

# `cargo.toml`

The `.toml` file uses `features` flags to limit the necessary libraries based on the selected application type.

### Dependencies

- **`chrono`**: This library is used for handling date and time operations. It’s especially useful for generating timestamps that can be inserted into logs or for any other functionality that requires time management.
    
- **`colored`**: Provides functionality to color the terminal output. This is useful for improving the readability of log messages by visually differentiating between errors, warnings, and informational messages.
    
- **`clap`**: A library for creating Command-Line Interface (CLI) applications. It simplifies argument parsing and command handling, making it easier to interact with users via the terminal. The version used here includes the `derive` feature, which further simplifies `clap` usage in Rust projects.
    
- **`once_cell`**: This library allows the creation of globally accessible variables that can be initialized only once. It’s particularly useful for resources that need to be accessible throughout the application but should only be initialized once, such as configurations or database connections.
    
- **`fern`**: A flexible logging system that allows for the customization of logging output, filters, and formats. It’s ideal for structuring logs in a readable format and filtering them based on their severity level.
    
- **`log`**: Provides a general logging framework that `fern` builds upon. It standardizes log levels (like info, warn, error) across the system, enabling consistent log management.
    
- **Optional Dependencies**:
    
    - **`auth`**: Manages authentication and security functionalities.
    - **`crud`**: Handles basic Create, Read, Update, and Delete (CRUD) operations for data management.
    - **`api_layer`**: Provides an API layer for handling HTTP requests and responses.
    - **`file_management`**: Offers functionalities for handling files, such as upload, download, and storage management.
    - **`task_automation`**: Manages task scheduling and automation processes.
    - **`blockchain`**: Facilitates blockchain integration, smart contract interactions, and transaction handling.
    - **`ml`**: Provides tools for machine learning tasks, such as handling models and performing inference.



```toml
[package]
name = "arx_framework"
version = "0.0.0"
edition = "2021"

[lib]
name = "arx_framework"
path = "src/lib.rs"  # Relative path for the library

# CLI binary definition
[[bin]] 
name = "arx" 
path = "src/main.rs"  # Entry point

# Common dependencies across all modules
[dependencies]
chrono = "0.4"  # For timestamps in logging
colored = "2.0"  # For coloring log messages
clap = { version = "3.2.0", features = ["derive"] }  # For CLI parsing
once_cell = "1.17.1"  # For managing static values
fern = "0.6"  # For logging
log = "0.4"  # General logging system

# Optional dependencies (external crates)
auth = { path = "./auth/", optional = true }
crud = { path = "./crud/", optional = true }
api_layer = { path = "./api/", optional = true }
file_management = { path = "./file_management/", optional = true }
task_automation = { path = "./task_automation/", optional = true }
blockchain = { path = "./blockchain/", optional = true }
ml = { path = "./ml/", optional = true }

# Feature Flags
[features]
# Base feature, always included
default = ["core_system", "monitoring"]

# Definition of features for modules
auth = ["auth"]
crud = ["crud"]
api_layer = ["api_layer"]
file_management = ["file_management"]
task_automation = ["task_automation"]
blockchain = ["blockchain"]
ml = ["ml"]
frontend = []  # To be enabled only if a Rust-based frontend module is implemented

# Definition of features for different types of applications
webapp = ["auth", "crud", "api_layer", "frontend"]
api_backend = ["auth", "crud", "api_layer"]
desktop = ["auth", "crud", "file_management", "frontend"]
automation = ["task_automation"]
embedded = []  # Embedded systems only use core_system and monitoring

# Base features
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