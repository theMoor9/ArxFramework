# Tameplates
| Modulo                 | Linguaggio Principale | Linguaggio di Supporto | Wrapping     | Framework/Librerie Principali | Considerazioni per lo Sviluppo<br>                          |
| ---------------------- | --------------------- | ---------------------- | ------------ | ----------------------------- | ----------------------------------------------------------- |
| Monitoraggio e logging | Rust                  | Python                 | Esclusi S.E. | tracing, prometheus           | Implementare trace distribuiti, metriche personalizzate<br> |

---

## Moduli Layer 1 per Code Base `main.rs`

### Dependencies

`cargo.toml`

```Rust
[dependencies] 
chrono = "0.4" 
colored = "2.0"
```


`logger.rs`

```Rust
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;
use colored::*;

// Enum per i livelli di log
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

// Logger statico
pub struct Log {
    level: LogLevel,  // Livello di log statico
    module: String,   // Nome del modulo per generare il file di log
}

impl Log {
    // Inizializza il logger con un livello di log e il nome del modulo
    pub fn new(level: LogLevel, module: &str) -> Self {
        Log { 
            level, 
            module: module.to_string() 
        }
    }

    // Funzione principale per loggare il messaggio
    pub fn logging(&self, level: LogLevel, message: &str) {
        if level as u8 >= self.level as u8 {
            // Formattazione del timestamp
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let log_message = format!("[{}] {}: {}", timestamp, level, message);
            
            // Aggiungere i colori (utili anche se si volesse comunque stampare in console per debug)
            let colored_message = match level {
                LogLevel::Debug => log_message.cyan().to_string(),
                LogLevel::Info => log_message.green().to_string(),
                LogLevel::Warning => log_message.yellow().to_string(),
                LogLevel::Error => log_message.red().to_string(),
            };

            // Scrivi il log nel file del modulo
            self.write_to_file(&log_message);

            // Opzionale: stampare comunque in console
            // println!("{}", colored_message);
        }
    }

    // Funzioni di logging per ogni livello
    pub fn debug(&self, message: &str) {
        self.logging(LogLevel::Debug, message);
    }

    pub fn info(&self, message: &str) {
        self.logging(LogLevel::Info, message);
    }

    pub fn warning(&self, message: &str) {
        self.logging(LogLevel::Warning, message);
    }

    pub fn error(&self, message: &str) {
        self.logging(LogLevel::Error, message);
    }

    // Funzione per scrivere nel file di log
    fn write_to_file(&self, message: &str) {
        // Nome del file di log in base al modulo
        let file_name = format!("logs/{}_log.txt", self.module);
        
        // Apriamo o creiamo il file con append (aggiungiamo le nuove righe alla fine del file)
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_name)
            .expect("Non è possibile aprire o creare il file di log");

        // Scriviamo il messaggio nel file, aggiungendo una nuova linea
        if let Err(e) = writeln!(file, "{}", message) {
            eprintln!("Errore durante la scrittura nel file di log: {}", e);
        }
    }
}

// Implementazione del display per il LogLevel
impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

```

---

## Layer 2 Wrapper