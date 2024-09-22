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
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::sync::{Arc, Mutex};
use chrono::Local;
use colored::*;
use once_cell::sync::OnceCell; 
use std::collections::HashMap; 

static FILE_HANDLES: OnceCell<Mutex<HashMap<String, Arc<Mutex<File>>>>> = OnceCell::new();

/// Enum per i livelli di log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// Logger thread-safe per la gestione dei log del sistema.
pub struct Log {
    /// Livello minimo di log da registrare.
    level: LogLevel,
    /// Nome del modulo, utilizzato per generare il nome del file di log.
    module: String,
    /// File di log, protetto da Mutex per accesso thread-safe.
    file: Arc<Mutex<File>>,
    /// Indica se stampare i messaggi anche in console.
    to_console: bool,
}

impl Log {
    /// Crea un nuovo logger per un modulo specifico.
    ///
    /// # Argomenti
    ///
    /// * `level` - Il livello minimo di log da registrare.
    /// * `module` - Il nome del modulo.
    /// * `to_console` - Se `true`, i messaggi di log saranno stampati anche in console.
    ///
    /// # Ritorna
    ///
    /// Un'istanza di `Log`.
    ///
    /// # Panics
    ///
    /// Se non è possibile creare la cartella dei log o aprire il file di log.
	
    pub fn new(level: LogLevel, module: &str, to_console: bool) -> Result<Self, std::io::Error> {
        // Inizializza la mappa dei file di log
        FILE_HANDLES.get_or_init(|| Mutex::new(HashMap::new()));
        
        // Crea la cartella dei log se non esiste.
        std::fs::create_dir_all("logs")?;
        
        // Genera il nome del file di log.
        let file_name = format!("logs/{}_log.txt", module);
        
        // Ottiene o crea il file di log condiviso
        let file_arc = {
            let mut handles = FILE_HANDLES.get().unwrap().lock().unwrap();
            handles.entry(module.to_string()).or_insert_with(|| {
                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&file_name)
                    .expect("Non è possibile aprire o creare il file di log");
                Arc::new(Mutex::new(file))
            }).clone()
        };
        
        Ok(Log {
            level,
            module: module.to_string(),
            file: file_arc,
            to_console,
        })
    }

    /// Registra un messaggio di log con un livello specifico.
    ///
    /// # Argomenti
    ///
    /// * `level` - Il livello del messaggio.
    /// * `message` - Il contenuto del messaggio.
    pub fn logging(&self, level: LogLevel, message: &str) {
        if level >= self.level {
            // Formatta il timestamp.
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            // Crea il messaggio di log.
            let log_message = format!("[{}] {}: {}", timestamp, level, message);

            // Aggiunge il colore al messaggio per la console.
            let colored_message = match level {
                LogLevel::Debug => log_message.cyan(),
                LogLevel::Info => log_message.green(),
                LogLevel::Warning => log_message.yellow(),
                LogLevel::Error => log_message.red(),
            };

            // Scrive il messaggio nel file di log.
            self.write_to_file(&log_message);

            // Se abilitato, stampa il messaggio in console.
            if self.to_console {
                println!("{}", colored_message);
            }
        }
    }

    /// Registra un messaggio di debug.
    ///
    /// # Argomenti
    ///
    /// * `message` - Il contenuto del messaggio.
    pub fn debug(&self, message: &str) {
        self.logging(LogLevel::Debug, message);
    }

    /// Registra un messaggio informativo.
    ///
    /// # Argomenti
    ///
    /// * `message` - Il contenuto del messaggio.
    pub fn info<S: AsRef<str>>(&self, message: S) {
	    self.logging(LogLevel::Info, message.as_ref());
	}


    /// Registra un messaggio di avviso.
    ///
    /// # Argomenti
    ///
    /// * `message` - Il contenuto del messaggio.
    pub fn warning(&self, message: &str) {
        self.logging(LogLevel::Warning, message);
    }

    /// Registra un messaggio di errore.
    ///
    /// # Argomenti
    ///
    /// * `message` - Il contenuto del messaggio.
    pub fn error(&self, message: &str) {
        self.logging(LogLevel::Error, message);
    }

    /// Scrive il messaggio nel file di log in modo thread-safe.
    ///
    /// # Argomenti
    ///
    /// * `message` - Il messaggio da scrivere nel file di log.
    fn write_to_file(&self, message: &str) {
        if let Ok(mut file) = self.file.lock() {
            if let Err(e) = writeln!(file, "{}", message) {
                eprintln!("Errore durante la scrittura nel file di log: {}", e);
            }
        } else {
            eprintln!("Errore nell'accedere al file di log per il modulo '{}'", self.module);
        }
    }
}

// Implementazione del trait Display per LogLevel.
impl std::fmt::Display for LogLevel {
    /// Converte il `LogLevel` in una stringa leggibile.
    ///
    /// # Argomenti
    ///
    /// * `f` - Il formatter di output.
    ///
    /// # Ritorna
    ///
    /// Un risultato che indica successo o fallimento nella scrittura.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level_str = match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
        };
        write!(f, "{}", level_str)
    }
}
```
