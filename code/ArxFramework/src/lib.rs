//! Modulo principale del framework ArxFramework.
//!
//! Questo modulo definisce e gestisce l'esportazione dei moduli principali del framework,
//! permettendo un'ottimizzazione del codice in fase di compilazione grazie alle feature specificate
//! nel `Cargo.toml`. In questo modo, solo i moduli necessari vengono inclusi nell'applicazione finale,
//! migliorando le prestazioni e riducendo l'overhead.
//!
//! ### Come Funziona l'Ottimizzazione:
//!
//! Ogni modulo è incluso solo se la relativa feature è attiva. Questo riduce l'overhead e migliora le prestazioni,
//! in quanto il compilatore includerà solo il codice necessario per l'applicazione specifica.
//!
//! ### Moduli Esportati:
pub mod config;
pub mod core;
pub mod monitoring;
pub mod cli;

/// Ottimizzazione in fase di compilazione grazie alle feature specificate nel `Cargo.toml`.
///
/// Queste direttive permettono di includere o escludere moduli in base alle feature abilitate.
/// Solo i moduli necessari per il tipo di applicazione scelto verranno compilati e inclusi.

#[cfg(feature = "auth")]
pub extern crate auth;

#[cfg(feature = "crud")]
pub extern crate crud;

#[cfg(feature = "api")]
pub extern crate api;

#[cfg(feature = "file_management")]
pub extern crate file_management;

#[cfg(feature = "task_automation")]
pub extern crate task_automation;

#[cfg(feature = "blockchain")]
pub extern crate blockchain;

#[cfg(feature = "ml")]
pub extern crate ml;

#[cfg(feature = "frontend")]
pub extern crate frontend;

// ### Estensione del Framework
//
// Per gli sviluppatori che desiderano estendere il framework aggiungendo nuovi moduli, è possibile
// farlo seguendo questo metodo standard:
//
// 1. **Crea un nuovo modulo**: Posiziona il modulo all'interno della directory di progetto appropriata.
//    Per esempio, se il nuovo modulo si chiama `my_new_module`, crealo nella directory corrispondente.
//
// 2. **Aggiungi una feature nel `Cargo.toml`**: Definisci una nuova feature per il tuo modulo. Ad esempio:
//    ```toml
//    [features]
//    my_new_module = ["my_new_module"]
//    ```
//
// 3. **Aggiorna `lib.rs`**: Includi il nuovo modulo in `lib.rs` utilizzando una clausola condizionale:
//    ```rust
//    #[cfg(feature = "my_new_module")]
//    pub mod my_new_module;
//    ```
//
// Questo approccio garantisce che il framework resti flessibile e scalabile, permettendo agli sviluppatori
// di aggiungere nuovi moduli senza impattare negativamente sulle performance del sistema.
