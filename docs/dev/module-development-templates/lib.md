
# src/

---

# `lib.rs`

Il modulo `lib.rs` è strettamente legato ai *flags* attivi del `cargo.toml` relativi al tipo di applicazione. Nel modulo sono specificati i *flags* come tag nelle configurazioni `#[cfg(feature = "core_system")]` limitando le librerie a disposizione ottimizzando la memoria per il progetto. 

*Flags*: Ottimizzare la fase di compilazione 

```Rust
// src/lib.rs

use log::{info, error};
use crate::core_system::CoreError;

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
