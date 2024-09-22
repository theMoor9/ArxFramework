# Tameplate

Il `global_config.rs` è il modulo responsabile del raggruppamento di tutte le variabili di configurazione degli altri moduli

```Rust
pub enum ApplicationType{
	WebApp,
	ApiBackend,
	DesktopApp,
	AutomationScript,
	EmbeddedSystem,
}

pub struct CoreConfig {
    pub app_type: ApplicationType,
    pub max_threads: usize,
}

enum ApiConfig

```