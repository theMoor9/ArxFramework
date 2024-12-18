use log::{info};

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "webapp")] {
        use crate::crud::models::default::{
            user::model::User, article::model::Article, comment::model::Comment, category::model::Category,
            tag::model::Tag, file::model::File, page::model::Page,
        };
    } else if #[cfg(feature = "api_backend")] {
        use crate::crud::models::default::{
            user::model::User, api_key::model::ApiKey, token::model::Token, request_log::model::RequestLog,
            endpoint::model::Endpoint, permission::model::Permission, rate_limit_rule::model::RateLimitRule,
        };
    } else if #[cfg(feature = "desktop")] {
        use crate::crud::models::default::{
            user::model::User, settings::model::Settings, document::model::Document, file::model::File,
            preferences::model::Preferences, task::model::Task, project::model::Project,
        };
    } else if #[cfg(feature = "automation")] {
        use crate::crud::models::default::{
            script::model::Script, task::model::Task, execution_log::model::ExecutionLog,
            schedule::model::Schedule, configuration::model::Configuration, job::model::Job,
            macro_script::model::MacroScript,
        };
    } else if #[cfg(feature = "embedded")] {
        use crate::crud::models::default::{
            device::model::Device, sensor_data::model::SensorData, configuration::model::Configuration,
            firmware_version::model::FirmwareVersion, log_event::model::LogEvent, command::model::Command, task::model::Task,
        };
    }
}

use crate::core::memory_management::{AllocationStrategy, MemoryManager};
use crate::config::memory_config::MemoryConfig;

//DATABASE
//use diesel::prelude::*;
//use crate::::connection; 




// Importazione variabili statiche per mantenere i modelli in memoria
use crate::core::memory_management::{
    TASKS_IN_MEMORY, 
    CONFIGURATIONS_IN_MEMORY, 
    DEVICES_IN_MEMORY, 
    JOBS_IN_MEMORY, 
    MACROS_IN_MEMORY, 
    SENSOR_DATA_IN_MEMORY, 
    LOG_EVENTS_IN_MEMORY, 
    COMMANDS_IN_MEMORY
};

#[derive(Debug,Clone)]
pub enum AllocType {
    InMemory,
    Database,
}
#[derive(Debug,Clone)]
pub struct CrudOperations {
    pub create: bool,
    pub read: bool,
    pub update: bool,
    pub delete: bool,
    pub list: bool,
    pub search: bool,
    pub revoke: bool,
}

/// Trait che definisce l'operazione di creazione per un generico tipo `T`.
/// 
/// Questo trait implementa la logica per creare un nuovo elemento di tipo `T`,
/// utilizzando la memoria o il database a seconda della configurazione di allocazione.
pub trait Create<T> {
    fn create(item: T) -> Result<T, String>;
}

/// Trait che definisce l'operazione di lettura per un generico tipo `T`.
/// 
/// Permette di leggere un elemento dal database o dalla memoria in base al suo ID.
pub trait Read<T> {
    fn read(id: u32) -> Result<T, String>;
}

/// Trait che definisce l'operazione di aggiornamento per un generico tipo `T`.
/// 
/// Aggiorna un elemento esistente nel database o in memoria.
pub trait Update<T> {
    fn update(item: T) -> Result<T, String>;
}

/// Trait che definisce l'operazione di eliminazione.
/// 
/// Elimina un elemento dal database o dalla memoria in base al suo ID.
pub trait Delete {
    fn delete(id: u32) -> Result<(), String>;
}

/// Trait che definisce l'operazione di elencazione per un generico tipo `T`.
/// 
/// Elenca tutti gli elementi presenti in memoria o nel database.
pub trait List<T> {
    fn list() -> Vec<T>;
}

/// Trait che definisce l'operazione di ricerca per un generico tipo `T`.
/// 
/// Effettua una ricerca tra gli elementi in base a una query specifica.
pub trait Search<T> {
    fn search(query: &str) -> Vec<T>;
}

/// Trait che definisce l'operazione di revoca.
/// 
/// Revoca un elemento specifico in base al suo ID, come ad esempio un token o un permesso.
pub trait Revoke {
    fn revoke(id: u32) -> Result<(), String>;
}

/// Macro per implementare le operazioni CRUD comuni.
/// 
/// Questa macro implementa automaticamente i metodi CRUD (Create, Read, Update, Delete)
/// per un tipo specificato. Ogni modello gestisce sia l'allocazione in memoria che
/// l'inserimento nel database, in base alle preferenze definite per ciascun modello.
macro_rules! impl_crud_ops {
    ($model:ty) => {
        impl Create<$model> for $model {
            /// Crea una nuova istanza del modello specificato.
            ///
            /// La funzione `create` permette di creare un nuovo record per il modello specificato.
            /// La memoria allocata per il nuovo record viene determinata in base alla configurazione
            /// del tipo di modello e della strategia di allocazione, e utilizza il valore di `memory_scale`
            /// per calcolare la quantità di memoria da allocare.
            ///
            /// # Parametri
            ///
            /// - `item`: Un'istanza del modello che si desidera creare.
            /// - `memory_scale`: Un valore `u8` che rappresenta il moltiplicatore di memoria configurato.
            ///   Questo valore viene utilizzato per determinare quanta memoria deve essere allocata
            ///   per il modello creato. Più alto è il valore, più grande sarà l'allocazione.
            ///
            /// # Restituisce
            ///
            /// - `Result<$model, String>`: Restituisce un `Ok($model)` se la creazione è avvenuta con successo,
            ///   oppure un `Err(String)` con un messaggio di errore nel caso in cui si verifichi un problema.
            fn create(item: $model, memory_scale: u8) -> Result<$model, String> {
                match std::any::type_name::<$model>() {
                    // Task temporanei, quindi la memoria standard va bene per velocità e semplicità
                    "modules::default::task_model::Task" => {
                        info!("Allocazione in memoria per Task");
                
                        // Determina la dimensione da allocare. Supponiamo di voler allocare 1024 byte.
                        let size: usize = (1024 * memory_scale).into();
                
                        // Allocazione della memoria per il Task
                        let task_memory = MemoryManager::allocate(,Some(AllocationStrategy::Standard), size);
                
                        // Creazione del Task con i dati ricevuti
                        let mut task = Task::new(
                            item.id,
                            item.description, 
                            #[cfg(feature = "automation")]
                            item.schedule,
                            #[cfg(feature = "desktop")]
                            item.completed,
                            #[cfg(feature = "embedded")]
                            item.device_id,
                            task_memory,
                        );
                
                        // Persistenza del Task
                        match task.store {
                            AllocType::InMemory => {
                                // Memorizza il Task in memoria
                                let mut tasks = TASKS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                                tasks.insert(task.id, task.clone());
                            }
                            AllocType::Database => {
                                // Memorizza il Task nel database
                                
                            }
                        }
                
                        info!("Allocata memoria di {} byte per il Task con ID: {}", size, task.id);
                        return Ok(task);
                    }

                    // Implementazione per altri modelli

                    _ => {
                        info!("Allocazione in memoria di Default per modello non gestito specificamente");
                        let size = (1024 * memory_scale).into(); // Size per modelli non gestiti dedicati 1024 byte per range applicativo di media 
                        MemoryManager::allocate(,None, size)?;
                        return Ok(item);
                    }
                }
            }
        }

        impl Read<$model> for $model {
            /// Legge un elemento di tipo `$model` in base al suo ID.
            ///
            /// La funzione `read` cerca prima l'elemento nella memoria (InMemory), se applicabile,
            /// e se non lo trova, tenta di recuperarlo dal database.
            ///
            /// # Parametri
            /// - `id`: L'ID dell'elemento che si desidera leggere.
            ///
            /// # Ritorna
            /// - `Ok($model)` se l'elemento è stato trovato in memoria o nel database.
            /// - `Err(String)` se l'elemento non è stato trovato né in memoria né nel database.
            fn read(id: u32) -> Result<$model, String> {
                match std::any::type_name::<$model>() {
                    // Task (InMemory)
                    "modules::default::task_model::Task" => {
                        let tasks = TASKS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                        if let Some(task) = tasks.get(&id) {
                            return Ok(task.clone());
                        } else {
                            return Err(format!("Task con ID {} non trovato", id));
                        }
                    }

                    // Implementazione per altri modelli

                    _ => Err(format!("Elemento con ID {} non trovato", id)),
                }
            }
        }


        impl Update<$model> for $model {
            fn update(item: $model) -> Result<$model, String> {
                // Simulazione della logica di aggiornamento (modifica nel database) per ogni modello che implementa il trait con `match`statement
                Ok(item)
            }
        }

        impl Delete for $model {
            fn delete(_id: u32) -> Result<(), String> {
                // Simulazione della logica di eliminazione (rimozione dal database) per ogni modello che implementa il trait con `match`statement
                Ok(())
            }
        }

        impl List<$model> for $model {
            fn list() -> Vec<$model> {
                // Simulazione della logica di elenco di tutti gli elementi  per ogni modello che implementa il trait con `match`statement
                vec![]
            }
        }
        impl Search<$model> for $model {
            fn search(_query: &str) -> Vec<$model> {
                // Simulazione della logica di ricerca nel database o in memoria.
                vec![]
            }
        }

        impl Revoke for $model {
            fn revoke(_id: u32) -> Result<(), String> {
                // Simulazione della logica di revoca, ad esempio per chiavi API o token.
                Ok(())
            }
        }
    };
}

// Applicazione delle macro ai modelli

impl_crud_ops!(Task);



// Aggiungi qui ulteriori modelli per l'applicazione CRUD generica

// Esempio di utilizzo per i modelli con Search e Revoke
//impl_search_and_revoke!(User);
//impl_search_and_revoke!(Article);

// Aggiungi ulteriori modelli se necessario

//Note sviluppo
// variabile: connection è definita in api_server::connection
// variabile: STATIC_MEMORY_SCALE è definita in questo file in maniera temporanea
//            Sarà definita in una variabile di configurazione nel main creata tramite il CLI 
