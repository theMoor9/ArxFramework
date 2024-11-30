//! Modulo core del sistema per la gestione delle operazioni principali.
//!
//! Questo modulo è responsabile della gestione e dell'orchestrazione dei vari moduli
//! in base al tipo di applicazione. Gestisce l'inizializzazione dei moduli richiesti
//! e assicura che siano eseguiti correttamente. La memoria è gestita tramite un 
//! MemoryManager, che seleziona la strategia di allocazione più adatta per il default se non è stata definita.
//!
//! Nota: Il modulo system_core.rs è statico per tutte le applicazioni tranne per i sistemi embedded.
//! Per i sistemi embedded, il modulo fornisce un'infrastruttura di base ma permette personalizzazioni
//! a livello di codice, per consentire compatibilità con hardware specifico e ambienti con risorse limitate.
//!
//! La struttura del framework garantisce la compatibilità con la maggior parte delle applicazioni standard (WebApp, API Backend, Desktop App, ecc.),
//! eseguendo in modo sicuro e centralizzato tutti i moduli, mentre per gli ambienti embedded offre flessibilità per le personalizzazioni richieste.

use crate::config::{
    global_config::{CoreConfig, ApplicationType},
    memory_config::MemoryConfig,
    database_config::DatabaseType,
};

use crate::core::memory_management::MemoryManager;
use crate::monitoring::logger;
use log::{info, warn ,error};


/// Sezione di importin per la gestione della connessione al database
// Importa la funzione per la connessione al database
use crate::network::connection_management::ConnectionManager;
// Importa la funzione per lo scrapeing delle tabelle dal codice
use crate::crud::models::table_scraper::scrape;
// Importa la funzione per la generazione delle tables nel database
use crate::crud::models::table_generator::generate_tables;


/*DEV: IL data base si inizializza in new() ricevendo nel costruttore le configurazione dal cli del main */


#[cfg(feature = "auth")]
use crate::auth;
#[cfg(feature = "crud")]
use crate::crud;
#[cfg(feature = "api")]
use crate::api;
#[cfg(feature = "file_management")]
use crate::file_management;
#[cfg(feature = "task_automation")]
use crate::task_automation;
#[cfg(feature = "frontend")]    
use crate::frontend;


/// Definizione degli errori principali che possono verificarsi nel sistema core.
#[derive(Debug)]
pub enum CoreError {
    InitializationError(String),
    ResourceAllocationError(String),
    ConfigurationError(String),
    UnsupportedOperationError(String),
    GenericError(String),
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::InitializationError(msg) => write!(f, "InitializationError: {}", msg),
            CoreError::ResourceAllocationError(msg) => write!(f, "ResourceAllocationError: {}", msg),
            CoreError::ConfigurationError(msg) => write!(f, "ConfigurationError: {}", msg),
            CoreError::UnsupportedOperationError(msg) => write!(f, "UnsupportedOperationError: {}", msg),
            CoreError::GenericError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for CoreError {}

/// CoreSystem è la struttura centrale che gestisce l'intero sistema.
/// Si occupa dell'inizializzazione dei moduli e della gestione della memoria.
///
/// # Campi
/// - config: La configurazione principale del sistema, che specifica il tipo di applicazione.
/// - memory_manager: Gestore della memoria, che implementa strategie di allocazione in base al tipo di applicazione.
pub struct CoreSystem {
    config: CoreConfig,
    memory_manager: MemoryManager,
    connection_manager: Option<ConnectionManager>,
}

macro_rules! init_module {
    ($module_name:expr, $init_func:expr) => {
        {
            info!("Inizializzazione del modulo {}", $module_name);
            if let Err(e) = $init_func() {
                error!("Errore nell'inizializzazione del modulo {}: {}", $module_name, e);
                return Err(CoreError::InitializationError(format!("{} initialization failed: {}", $module_name, e)));
            }
            logger::monitor_module_status($module_name, None);
            Ok(())
        }
    }
}

impl CoreSystem {
    /// Crea una nuova istanza di CoreSystem in base alla configurazione fornita.
    /// Questa funzione inizializza anche il MemoryManager.
    ///
    /// # Parametri shrthand syntax
    /// - config: La configurazione globale che definisce il tipo di applicazione.
    /// - memory_config: La configurazione della memoria che specifica le dimensioni dei buffer e del pool.
    /// - database_config: La configurazione del database che specifica il tipo di database e le impostazioni di connessione.
    ///
    /// # Ritorna
    /// Un'istanza di CoreSystem o un errore di inizializzazione (CoreError).
    pub fn new(config: CoreConfig, memory_config: MemoryConfig, database_config: DatabaseType) -> Result<Self, CoreError> {
        info!("Inizializzazione del CoreSystem...");
        let app_type = &config.app_type;
        let memory_manager = MemoryManager::new(config.app_type.clone(), memory_config).map_err(|e| {
            error!("Errore nell'inizializzazione del MemoryManager: {}", e);
            CoreError::InitializationError(e.to_string())
        })?;
        let connection_manager = match database_config.clone() {
            DatabaseType::None => {
                warn!("Configurazione del database non impostata per l'applicazione");
                None
            }
            _ => Some({
                    ConnectionManager::new(database_config).map_err(|e| {
                        error!("Errore nell'inizializzazione del ConnectionManager: {}", e);
                        CoreError::InitializationError(e.to_string())
                    })?;
                })
        }
        

        info!("CoreSystem inizializzato con app_type: {:?}",app_type);
        Ok(CoreSystem { config, memory_manager , Some<connection_manager>})
    }


    /// Funzione principale che esegue le operazioni in base al tipo di applicazione.
    /// Inizializza i moduli richiesti per il tipo di applicazione configurata.
    ///
    /// # Ritorna
    /// Ok(()) se tutti i moduli sono stati inizializzati correttamente, altrimenti un CoreError.
    ///
    /// # Nota
    /// - Questa funzione utilizza la configurazione fornita in CoreConfig per determinare
    /// quali moduli devono essere inizializzati.
    #[allow(unreachable_code)]
    pub fn run(&self) -> Result<(), CoreError> {
        info!("Esecuzione del CoreSystem...");

        

        match self.connection_manager {
            Some(cm) => {
                // Inizializzazione della connessione al database
                info!("Inizializzazione della connessione al database...");
                cm.initialize_connection().await?;

                let default_path = "src/crud/models/default";
                let dev_path = "src/crud/models/dev";   

                // Generazione delle tabelle nel database
                info!("Generazione delle tabelle nel database...");
                // Generazione tabelle default
                generate_tables(scrape(default_path, cm.clone())?, cm,).await?;
                // Generazione tabelle dev
                generate_tables(scrape(dev_path, cm.clone())?, cm,).await?;
            }
            None => {
                warn!("Configurazione del database non impostata per l'applicazione");
            }
        }

        match self.config.app_type {
            ApplicationType::WebApp => {

                info!("Configurazione per WebApp");
                #[cfg(feature = "auth")]
                init_module!("Authentication", || auth::initialize())?;
                #[cfg(feature = "crud")]
                init_module!("CRUD", || crud::initialize())?;
                #[cfg(feature = "api")]
                init_module!("API Layer", || api::initialize())?;
                #[cfg(feature = "frontend")]
                init_module!("Frontend", || frontend::initialize())?;

                #[cfg(not(feature = "auth"))]
                return Err(CoreError::UnsupportedOperationError("Authentication module is required for WebApp".to_string()));
                #[cfg(not(feature = "crud"))]
                return Err(CoreError::UnsupportedOperationError("CRUD module is required for WebApp".to_string()));
                #[cfg(not(feature = "api"))]
                return Err(CoreError::UnsupportedOperationError("API module is required for WebApp".to_string()));
                #[cfg(not(feature = "frontend"))]
                return Err(CoreError::UnsupportedOperationError("Frontend module is required for WebApp".to_string()));
                
            }

            ApplicationType::ApiBackend => {
                info!("Configurazione per API Backend");
                #[cfg(feature = "auth")]
                init_module!("Authentication", || auth::initialize())?;
                #[cfg(feature = "crud")]
                init_module!("CRUD", || crud::initialize())?;
                #[cfg(feature = "api")]
                init_module!("API Layer", || api::initialize())?;


                #[cfg(not(feature = "auth"))]
                return Err(CoreError::UnsupportedOperationError("Authentication module is required for API Backend".to_string()));
                #[cfg(not(feature = "crud"))]
                return Err(CoreError::UnsupportedOperationError("CRUD module is required for API Backend".to_string()));
                #[cfg(not(feature = "api"))]
                return Err(CoreError::UnsupportedOperationError("API module is required for API Backend".to_string()));

            }

            ApplicationType::DesktopApp => {
                info!("Configurazione per App Desktop");
                #[cfg(feature = "auth")]
                init_module!("Authentication", || auth::initialize())?;
                #[cfg(feature = "crud")]
                init_module!("CRUD", || crud::initialize())?;
                #[cfg(feature = "file_management")]
                init_module!("File Management", || file_management::initialize())?;
                #[cfg(feature = "frontend")]
                init_module!("Frontend", || frontend::initialize())?;

                #[cfg(not(feature = "auth"))]
                return Err(CoreError::UnsupportedOperationError("Authentication module is required for Desktop App".to_string()));
                #[cfg(not(feature = "crud"))]
                return Err(CoreError::UnsupportedOperationError("CRUD module is required for Desktop App".to_string()));
                #[cfg(not(feature = "file_management"))]
                return Err(CoreError::UnsupportedOperationError("File Management module is required for Desktop App".to_string()));
                #[cfg(not(feature = "frontend"))]
                return Err(CoreError::UnsupportedOperationError("Frontend module is required for Desktop App".to_string()));
            }

            ApplicationType::AutomationScript => {
                info!("Configurazione per Automazione e Scripting");
                #[cfg(feature = "task_automation")]
                init_module!("Task Automation", || task_automation::initialize())?;
                #[cfg(feature = "file_management")]
                init_module!("File Management", || file_management::initialize())?;

                #[cfg(not(feature = "task_automation"))]
                return Err(CoreError::UnsupportedOperationError("Task Automation module is required for Automation Script".to_string()));
                #[cfg(not(feature = "file_management"))]
                return Err(CoreError::UnsupportedOperationError("File Management module is required for Automation Script".to_string()));
            }

            ApplicationType::EmbeddedSystem => {
                info!("Configurazione per Sistemi Embedded");
                // Inizializzazione di eventuali moduli specifici per sistemi embedded.
            }

            _ => {
                return Err(CoreError::ConfigurationError("Tipo di applicazione non supportato considera implementazione".to_string()));
            }
        }

        Ok(())
    }
}



/* 
 Esempio di personalizzazione per Sistemi Embedded:

 
 // Inizializzazione personalizzata per un sistema embedded
 fn initialize_embedded_system() -> Result<(), CoreError> {
     info!("Configurazione specifica per Sistemi Embedded...");
     // Logica di inizializzazione specifica per l'hardware target
     // Potrebbe includere configurazione di GPIO, interfacce seriali, ecc.
     Ok(())
 }

 // Esempio di utilizzo:
 let result = initialize_embedded_system();
 if result.is_err() {
     error!("Errore nell'inizializzazione del sistema embedded");
 }
*/