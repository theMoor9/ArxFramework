# Standard Procedure

>Procedura Standard d'Uso per `global_config.rs`

---

## Layer 1 Code Base `main.rs`


**Inizializzazione**:

- Configurazione del crate config tramite [cli](./module-development-templates/cli.md):

```Rust
use crate::config::global_config::get_core_config;
```
    
- Creare la configurazione del core:
	
    
```Rust
let config = get_core_config()?;
```