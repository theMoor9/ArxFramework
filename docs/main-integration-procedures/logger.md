# Standard Procedure

>Procedura Standard d'Uso per `logger.rs`

---

## Layer 1 Code Base

Utilizziamo il modulo `logger` principalmente per generare

- Uso

```Rust
// In un qualsiasi modulo o nel main

use crate::monitoring::logger::{Log,LogLevel};
/*
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}
*/


pub fn authenticate_user() {
	let logger = Log::new(LogLevel::Info, "auth");
	
	logger.info("Tentativo di autenticazione utente iniziato");
	
	// Se succede qualcosa di strano...
	logger.warning("Password debole rilevata");

	// Se l'autenticazione fallisce...
	logger.error("Autenticazione utente fallita");

	// Se serve loggare più dettagli
	logger.debug("Dettagli di debug dell'autenticazione");
}
```