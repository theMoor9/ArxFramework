# Command Line Interface

## Tameplate

```Rust
use clap::{App, Arg, ArgMatches};
use crate::config::global_config::{CoreConfig,ApplicationType};
use crate::monitoring::logger::LogLevel;
/*
Dentro lib.rs in src sono contenuti i crate
in questo caso `pub mod core` che da accesso a system_core.rs
*/ 

// Viene restituito il config file
/*
OCCORRE PERSONALIZARE I MESSAGGI DEL CLI
*/
pub fn parse_config_cli() -> CoreConfig {
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
        .arg(Arg::with_name("log-level")
            .long("log-level")
            .value_name("LEVEL")
            .help("Sets the log level")
            .takes_value(true)
            .default_value("info")
            .possible_values(&["debug", "info", "warning", "error"]))
        .get_matches();

    CoreConfig {
        app_type: parse_app_type(&matches),
        max_threads: parse_max_threads(&matches),
        log_level: parse_log_level(&matches),
    }
}

fn parse_app_type(matches: &ArgMatches) -> ApplicationType {
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
    matches.value_of("max-threads")
        .unwrap()
        .parse()
        .expect("Failed to parse max-threads")
}

fn parse_log_level(matches: &ArgMatches) -> LogLevel {
    match matches.value_of("log-level").unwrap() {
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warning" => LogLevel::Warning,
        "error" => LogLevel::Error,
        _ => unreachable!(),
    }
}
```