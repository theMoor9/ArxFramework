# Tameplates

---

## Moduli Layer 1 per Code Base 
### Istruzioni e note
Dentro lib.rs in src sono contenuti i crate
in questo caso `pub mod cli` che da accesso a cli.rs

**Attenzione**: Occorre personalizzare i messaggi del CLI e le informazioni riportate dalle funzioni di logging.

`cli.rs`:

```Rust
use clap::{App, Arg, ArgMatches};
use crate::config::global_config::{CoreConfig,ApplicationType};
use crate::monitoring::logger::{Log,LogLevel};
const LOGGER = Log::new(LogLevel::Info, "cli");
	


// Viene restituito il config file
pub fn parse_config_cli() -> CoreConfig {
	// logger.info("Informazioni sull inizializzazzione dei logger")
    let matches = App::new("ArxFramework CLI")
        .version("1.0")
        .author("Your Name")
        .about("Configures the ArxFramework Core System")
        .arg(Arg::with_name("app-type")
            .long("app-type")
            .value_name("TYPE")
            .help("Sets the application type")
            .takes_value(true)
            .required(true)
            .possible_values(&["webapp", "api", "desktop", "automation", "embedded"]))
        .arg(Arg::with_name("max-threads")
            .long("max-threads")
            .value_name("NUMBER")
            .help("Sets the maximum number of threads")
            .takes_value(true)
            .default_value("4"))
        .get_matches();
	// logger.info("Informazioni sul ritorno del Core config")
    CoreConfig {
        app_type: parse_app_type(&matches),
        max_threads: parse_max_threads(&matches),
    }
}

fn parse_app_type(matches: &ArgMatches) -> ApplicationType {
	// logger.info("Informazioni sul parsing dell tipo di applicazione")
    match matches.value_of("app-type").unwrap() {
        "webapp" => ApplicationType::WebApp,
        "api" => ApplicationType::ApiBackend,
        "desktop" => ApplicationType::DesktopApp,
        "automation" => ApplicationType::AutomationScript,
        "embedded" => ApplicationType::EmbeddedSystem,
        _ => unreachable!(),
    }
    
}

fn parse_max_threads(matches: &ArgMatches) -> usize {
	// logger.info("Informazioni sul parsing dei thread massimi imposti")
    matches.value_of("max-threads")
        .unwrap()
        .parse()
        .expect("Failed to parse max-threads")
}
```