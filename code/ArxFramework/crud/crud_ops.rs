use crate::models::dev::*;
use crate::models::default::*;
use crate::memory_manager::{AllocationStrategy, memory_manager};
use crate::global_config::MemoryConfig;
//use crate::::connection; NECESSARIO PER LA CONNESSIONE AL DATABASE
use log::{info};

// Importazione variabili statiche per mantenere i modelli in memoria
use crate::memory_management::{
    TASKS_IN_MEMORY, 
    CONFIGURATIONS_IN_MEMORY, 
    DEVICES_IN_MEMORY, 
    JOBS_IN_MEMORY, 
    MACROS_IN_MEMORY, 
    SENSOR_DATA_IN_MEMORY, 
    LOG_EVENTS_IN_MEMORY, 
    COMMANDS_IN_MEMORY
};

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
    fn read(id: u64) -> Result<T, String>;
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
    fn delete(id: u64) -> Result<(), String>;
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
    fn revoke(id: u64) -> Result<(), String>;
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
                        let size = 1024 * memory_scale;
                
                        // Allocazione della memoria per il Task
                        let task_memory = memory_manager::allocate(Some(AllocationStrategy::Standard), size)?;
                
                        // Creazione del Task con i dati ricevuti
                        let mut task = Task::new(
                            item.id,
                            item.description,
                            #[cfg(feature = "automation")] item.schedule,
                            #[cfg(feature = "desktop")] item.completed,
                            #[cfg(feature = "embedded")] item.device_id,
                            task_memory,
                        );
                
                        // Persistenza del Task
                        match task.store {
                            Allocation::InMemory => {
                                // Memorizza il Task in memoria
                                let mut tasks = TASKS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                                tasks.insert(task.id, task.clone());
                            }
                            Allocation::Database => {
                                // Memorizza il Task nel database
                                sqlx::query!(
                                    "INSERT INTO tasks (id, description) VALUES ($1, $2)",
                                    task.id,
                                    task.description,
                                )
                                .execute(&mut connection)  // Connessione a PostgreSQL da gestire nel modulo api_server.rs
                                .map_err(|e| format!("Errore di inserimento nel database: {}", e))?;
                            }
                        }
                
                        info!("Allocata memoria di {} byte per il Task con ID: {}", size, task.id);
                        Ok(task)
                    }
                
                    // Configurazioni temporanee, che possono essere più complesse
                    "modules::default::configuration_model::Configuration" => {
                        info!("Allocazione in PoolBased per Configuration");
                        let size = 1024 * memory_scale; // 1 KB per configurazioni temporanee
                        let config_memory = memory_manager::allocate(Some(AllocationStrategy::PoolBased), size)?;
                
                        // Creazione della configurazione
                        let mut configuration = Configuration::new(item.id, item.key, item.value, config_memory);
                
                        // Persistenza della Configurazione
                        match configuration.store {
                            Allocation::InMemory => {
                                let mut configurations = CONFIGURATIONS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                                configurations.insert(configuration.id, configuration.clone());
                            }
                            Allocation::Database => {
                                sqlx::query!(
                                    "INSERT INTO configurations (id, key, value) VALUES ($1, $2, $3)",
                                    configuration.id,
                                    configuration.key,
                                    configuration.value,
                                )
                                .execute(&mut connection)// Connessione a PostgreSQL da gestire nel modulo api_server.rs
                                .map_err(|e| format!("Errore di inserimento nel database: {}", e))?;
                            }
                        }
                
                        info!("Allocata memoria di {} byte per la Configurazione con ID: {}", size, configuration.id);
                        Ok(configuration)
                    }
                
                    // Device, necessitano di un'allocazione CustomEmbedded
                    "modules::default::device_model::Device" => {
                        info!("Allocazione CustomEmbedded per Device");
                        let size = 1024 * memory_scale; // 1 KB per device
                        let device_memory = memory_manager::allocate(Some(AllocationStrategy::CustomEmbedded), size)?;
                
                        let mut device = Device::new(item.id, item.name, device_memory);
                
                        match device.store {
                            Allocation::InMemory => {
                                let mut devices = DEVICES_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                                devices.insert(device.id, device.clone());
                            }
                            Allocation::Database => {
                                sqlx::query!(
                                    "INSERT INTO devices (id, name) VALUES ($1, $2)",
                                    device.id,
                                    device.name,
                                )
                                .execute(&mut connection)// Connessione a PostgreSQL da gestire nel modulo api_server.rs
                                .map_err(|e| format!("Errore di inserimento nel database: {}", e))?;
                            }
                        }
                
                        info!("Allocata memoria di {} byte per il Device con ID: {}", size, device.id);
                        Ok(device)
                    }
                
                    // Job temporanei, dimensione media, usiamo PoolBased
                    "modules::default::job_model::Job" => {
                        info!("Allocazione PoolBased per Job");
                        let size = 1024 * memory_scale;
                        let job_memory = memory_manager::allocate(Some(AllocationStrategy::PoolBased), size)?;
                
                        let mut job = Job::new(item.id, item.description, job_memory);
                
                        match job.store {
                            Allocation::InMemory => {
                                let mut jobs = JOBS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                                jobs.insert(job.id, job.clone());
                            }
                            Allocation::Database => {
                                sqlx::query!(
                                    "INSERT INTO jobs (id, description) VALUES ($1, $2)",
                                    job.id,
                                    job.description,
                                )
                                .execute(&mut connection)// Connessione a PostgreSQL da gestire nel modulo api_server.rs
                                .map_err(|e| format!("Errore di inserimento nel database: {}", e))?;
                            }
                        }
                
                        info!("Allocata memoria di {} byte per il Job con ID: {}", size, job.id);
                        Ok(job)
                    }
                
                    // Macro
                    "modules::default::macro_model::Macro" => {
                        info!("Allocazione in memoria Standard per Macro");
                        let size = 1024 * memory_scale; 
                        let macro_memory = memory_manager::allocate(Some(AllocationStrategy::Standard), size)?;
                
                        let mut macro_task = Macro::new(item.id, item.description, macro_memory);
                
                        match macro_task.store {
                            Allocation::InMemory => {
                                let mut macros = MACROS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                                macros.insert(macro_task.id, macro_task.clone());
                            }
                            Allocation::Database => {
                                sqlx::query!(
                                    "INSERT INTO macros (id, description) VALUES ($1, $2)",
                                    macro_task.id,
                                    macro_task.description,
                                )
                                .execute(&mut connection)// Connessione a PostgreSQL da gestire nel modulo api_server.rs
                                .map_err(|e| format!("Errore di inserimento nel database: {}", e))?;
                            }
                        }
                
                        info!("Allocata memoria di {} byte per la Macro con ID: {}", size, macro_task.id);
                        Ok(macro_task)
                    }
                
                    // SensorData
                    "modules::default::sensor_data_model::SensorData" => {
                        info!("Allocazione CustomEmbedded per Sensor Data");
                        let size = 1024 * memory_scale;
                        let sensor_data_memory = memory_manager::allocate(Some(AllocationStrategy::CustomEmbedded), size)?;

                        let mut sensor_data = SensorData::new(item.id, item.value, sensor_data_memory);

                        match sensor_data.store {
                            Allocation::InMemory => {
                                let mut sensor_data_map = SENSORDATA_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                                sensor_data_map.insert(sensor_data.id, sensor_data.clone());
                            }
                            Allocation::Database => {
                                sqlx::query!(
                                    "INSERT INTO sensor_data (id, value) VALUES ($1, $2)",
                                    sensor_data.id,
                                    sensor_data.value,
                                )
                                .execute(&mut connection)// Connessione a PostgreSQL da gestire nel modulo api_server.rs
                                .map_err(|e| format!("Errore di inserimento nel database: {}", e))?;
                            }
                        }

                        info!("Allocata memoria di {} byte per SensorData con ID: {}", size, sensor_data.id);
                        Ok(sensor_data)
                    }

                    // LogEvent
                    "modules::default::log_event_model::LogEvent" => {
                        info!("Allocazione PoolBased per Log/Event");
                        let size = 1024 * memory_scale;
                        let log_event_memory = memory_manager::allocate(Some(AllocationStrategy::PoolBased), size)?;

                        let mut log_event = LogEvent::new(item.id, item.message, log_event_memory);

                        match log_event.store {
                            Allocation::InMemory => {
                                let mut logs = LOGS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                                logs.insert(log_event.id, log_event.clone());
                            }
                            Allocation::Database => {
                                sqlx::query!(
                                    "INSERT INTO log_events (id, message) VALUES ($1, $2)",
                                    log_event.id,
                                    log_event.message,
                                )
                                .execute(&mut connection)// Connessione a PostgreSQL da gestire nel modulo api_server.rs
                                .map_err(|e| format!("Errore di inserimento nel database: {}", e))?;
                            }
                        }

                        info!("Allocata memoria di {} byte per LogEvent con ID: {}", size, log_event.id);
                        Ok(log_event)
                    }

                    // Command
                    "modules::default::command_model::Command" => {
                        info!("Allocazione CustomEmbedded per Command");
                        let size = 3072 * memory_scale;
                        let command_memory = memory_manager::allocate(Some(AllocationStrategy::CustomEmbedded), size)?;

                        let mut command = Command::new(item.id, item.action, command_memory);

                        match command.store {
                            Allocation::InMemory => {
                                let mut commands = COMMANDS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                                commands.insert(command.id, command.clone());
                            }
                            Allocation::Database => {
                                sqlx::query!(
                                    "INSERT INTO commands (id, action) VALUES ($1, $2)",
                                    command.id,
                                    command.action,
                                )
                                .execute(&mut connection)// Connessione a PostgreSQL da gestire nel modulo api_server.rs
                                .map_err(|e| format!("Errore di inserimento nel database: {}", e))?;
                            }
                        }

                        info!("Allocata memoria di {} byte per il Command con ID: {}", size, command.id);
                        Ok(command)
                    }

                    _ => {
                        info!("Allocazione in memoria di Default per modello non gestito specificamente");
                        let size = 1024 * memory_scale; // Size per modelli non gestiti dedicati 1024 byte per range applicativo di media 
                        memory_manager::allocate(None, size)?;
                        Ok(item)
                    }
                }
                
                Ok(item)
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
            fn read(id: u64) -> Result<$model, String> {
                match std::any::type_name::<$model>() {
                    // Task (InMemory)
                    "modules::default::task_model::Task" => {
                        let tasks = TASKS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                        if let Some(task) = tasks.get(&id) {
                            return Ok(task.clone());
                        }
                    }

                    // SensorData (InMemory)
                    "modules::default::sensor_data_model::SensorData" => {
                        let sensor_data = SENSOR_DATA_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                        if let Some(data) = sensor_data.get(&id) {
                            return Ok(data.clone());
                        }
                    }

                    // Macro (InMemory)
                    "modules::default::macro_model::Macro" => {
                        let macros = MACROS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                        if let Some(macro_task) = macros.get(&id) {
                            return Ok(macro_task.clone());
                        }
                    }

                    // LogEvent (InMemory)
                    "modules::default::log_event_model::LogEvent" => {
                        let log_events = LOG_EVENTS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                        if let Some(log_event) = log_events.get(&id) {
                            return Ok(log_event.clone());
                        }
                    }

                    // Job (InMemory)
                    "modules::default::job_model::Job" => {
                        let jobs = JOBS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                        if let Some(job) = jobs.get(&id) {
                            return Ok(job.clone());
                        }
                    }

                    // Device (InMemory)
                    "modules::default::device_model::Device" => {
                        let devices = DEVICES_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                        if let Some(device) = devices.get(&id) {
                            return Ok(device.clone());
                        }
                    }

                    // Configuration (InMemory)
                    "modules::default::configuration_model::Configuration" => {
                        let configurations = CONFIGURATIONS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                        if let Some(configuration) = configurations.get(&id) {
                            return Ok(configuration.clone());
                        }
                    }

                    // Command (InMemory)
                    "modules::default::command_model::Command" => {
                        let commands = COMMANDS_IN_MEMORY.lock().map_err(|e| format!("Errore di lock sul mutex: {}", e))?;
                        if let Some(command) = commands.get(&id) {
                            return Ok(command.clone());
                        }
                    }

                    // User (Database)
                    "modules::default::user_model::User" => {
                        // Recupera l'utente dal database
                        let user_row = sqlx::query_as!(User, "SELECT id, name FROM users WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(user_row);
                    }

                    // Article (Database)
                    "modules::default::article_model::Article" => {
                        let article_row = sqlx::query_as!(Article, "SELECT id, title FROM articles WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(article_row);
                    }

                    // Comment (Database)
                    "modules::default::comment_model::Comment" => {
                        let comment_row = sqlx::query_as!(Comment, "SELECT id, content FROM comments WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(comment_row);
                    }

                    // Category (Database)
                    "modules::default::category_model::Category" => {
                        let category_row = sqlx::query_as!(Category, "SELECT id, name FROM categories WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(category_row);
                    }

                    // Tag (Database)
                    "modules::default::tag_model::Tag" => {
                        let tag_row = sqlx::query_as!(Tag, "SELECT id, name FROM tags WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(tag_row);
                    }

                    // File/Image (Database)
                    "modules::default::file_model::FileImage" => {
                        let file_row = sqlx::query_as!(FileImage, "SELECT id, path FROM files WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(file_row);
                    }

                    // Page (Database)
                    "modules::default::page_model::Page" => {
                        let page_row = sqlx::query_as!(Page, "SELECT id, content FROM pages WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(page_row);
                    }

                    // API Key (Database)
                    "modules::default::api_key_model::ApiKey" => {
                        let api_key_row = sqlx::query_as!(ApiKey, "SELECT id, key FROM api_keys WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(api_key_row);
                    }

                    // Token (Database)
                    "modules::default::token_model::Token" => {
                        let token_row = sqlx::query_as!(Token, "SELECT id, token FROM tokens WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(token_row);
                    }

                    // Request Log (Database)
                    "modules::default::request_log_model::RequestLog" => {
                        let log_row = sqlx::query_as!(RequestLog, "SELECT id, request FROM request_logs WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(log_row);
                    }

                    // Endpoint (Database)
                    "modules::default::endpoint_model::Endpoint" => {
                        let endpoint_row = sqlx::query_as!(Endpoint, "SELECT id, path FROM endpoints WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(endpoint_row);
                    }

                    // Permission (Database)
                    "modules::default::permission_model::Permission" => {
                        let permission_row = sqlx::query_as!(Permission, "SELECT id, name FROM permissions WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(permission_row);
                    }

                    // Rate Limit Rule (Database)
                    "modules::default::rate_limit_rule_model::RateLimitRule" => {
                        let rate_limit_row = sqlx::query_as!(RateLimitRule, "SELECT id, rule FROM rate_limit_rules WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(rate_limit_row);
                    }

                    // Settings (Database)
                    "modules::default::settings_model::Settings" => {
                        let settings_row = sqlx::query_as!(Settings, "SELECT id, key, value FROM settings WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(settings_row);
                    }

                    // Document (Database)
                    "modules::default::document_model::Document" => {
                        let document_row = sqlx::query_as!(Document, "SELECT id, title FROM documents WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(document_row);
                    }

                    // Preferences (Database)
                    "modules::default::preferences_model::Preferences" => {
                        let preferences_row = sqlx::query_as!(Preferences, "SELECT id, name FROM preferences WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(preferences_row);
                    }

                    // Project (Database)
                    "modules::default::project_model::Project" => {
                        let project_row = sqlx::query_as!(Project, "SELECT id, name FROM projects WHERE id = $1", id)
                            .fetch_one(&mut connection)
                            .map_err(|e| format!("Errore di lettura dal database: {}", e))?;
                        
                        return Ok(project_row);
                    }

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
            fn delete(id: u64) -> Result<(), String> {
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
    };
}

/// Macro per implementare le operazioni di Search e Revoke.
/// 
/// Questa macro aggiunge la funzionalità di ricerca e revoca per un tipo specificato.
macro_rules! impl_search_and_revoke {
    ($model:ty) => {
        impl Search<$model> for $model {
            fn search(query: &str) -> Vec<$model> {
                // Simulazione della logica di ricerca nel database o in memoria.
                vec![]
            }
        }

        impl Revoke for $model {
            fn revoke(id: u64) -> Result<(), String> {
                // Simulazione della logica di revoca, ad esempio per chiavi API o token.
                Ok(())
            }
        }
    };
}

// Applicazione delle macro ai modelli

impl_crud_ops!(User);
impl_crud_ops!(Article);
impl_crud_ops!(Comment);
impl_crud_ops!(Category);
impl_crud_ops!(Tag);
impl_crud_ops!(ApiKey);
impl_crud_ops!(Token);
impl_crud_ops!(RequestLog);
impl_crud_ops!(Endpoint);
impl_crud_ops!(Permission);
impl_crud_ops!(RateLimitRule);
impl_crud_ops!(Settings);
impl_crud_ops!(Document);
impl_crud_ops!(File);
impl_crud_ops!(Preferences);
impl_crud_ops!(Task);
impl_crud_ops!(Project);
impl_crud_ops!(Script);
impl_crud_ops!(ExecutionLog);
impl_crud_ops!(Schedule);
impl_crud_ops!(Configuration);
impl_crud_ops!(Job);
impl_crud_ops!(Macro);
impl_crud_ops!(Device);
impl_crud_ops!(SensorData);
impl_crud_ops!(FirmwareVersion);
impl_crud_ops!(LogEvent);
impl_crud_ops!(Command);



// Aggiungi qui ulteriori modelli per l'applicazione CRUD generica

// Esempio di utilizzo per i modelli con Search e Revoke
impl_search_and_revoke!(User);
impl_search_and_revoke!(Article);

// Aggiungi ulteriori modelli se necessario

//Note sviluppo
// variabile: connection è definita in api_server::connection
// variabile: STATIC_MEMORY_SCALE è definita in questo file in maniera temporanea
//            Sarà definita in una variabile di configurazione nel main creata tramite il CLI 
