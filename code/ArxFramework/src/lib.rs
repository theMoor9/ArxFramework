use log::{info, error};
use crate::core_system::CoreError;


/// Ottimizzazione in fase di compilazione grazie alle features del .toml
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
