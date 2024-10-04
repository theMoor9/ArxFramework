use crate::models;
use crate::memory_manager::{AllocationStrategy, memory_manager};
use log::{info};

pub trait Create<T> {
    fn create(item: T) -> Result<T, String>;
}

pub trait Read<T> {
    fn read(id: u64) -> Result<T, String>;
}

pub trait Update<T> {
    fn update(item: T) -> Result<T, String>;
}

pub trait Delete {
    fn delete(id: u64) -> Result<(), String>;
}

pub trait List<T> {
    fn list() -> Vec<T>;
}

pub trait Search<T> {
    fn search(query: &str) -> Vec<T>;
}

pub trait Revoke {
    fn revoke(id: u64) -> Result<(), String>;
}

// Macro per implementare i trait CRUD comuni
macro_rules! impl_crud_ops {
    ($model:ty) => {
        impl Create<$model> for $model {
            fn create(item: $model) -> Result<$model, String> {
                match std::any::type_name::<$model>() {

                    // Task temporanei, quindi la memoria standard va bene per velocità e semplicità
                    "modules::default::task_model::Task" => {
                        info!("Allocazione in memoria Standard per Task");
                        let size = 256; // 256 byte per task temporanei
                        memory_manager::allocate(Some(AllocationStrategy::Standard), size)?;
                        Ok(item)
                    }
                    
                    // Configurazioni temporanee, che possono essere più complesse
                    "modules::default::configuration_model::Configuration" => {
                        info!("Allocazione in PoolBased per Configuration");
                        let size = 512; // 512 byte per configurazioni, per includere dati variabili
                        memory_manager::allocate(Some(AllocationStrategy::PoolBased), size)?;
                        Ok(item)
                    }

                    // Device, necessitano di un'allocazione CustomEmbedded
                    "modules::default::device_model::Device" => {
                        info!("Allocazione CustomEmbedded per Device");
                        let size = 1024; // 1 KB per device, dati più dettagliati
                        memory_manager::allocate(Some(AllocationStrategy::CustomEmbedded), size)?;
                        Ok(item)
                    }

                    // Job temporanei, dimensione media, usiamo PoolBased
                    "modules::default::job_model::Job" => {
                        info!("Allocazione PoolBased per Job");
                        let size = 512; // 512 byte per job temporanei
                        memory_manager::allocate(Some(AllocationStrategy::PoolBased), size)?;
                        Ok(item)
                    }

                    // Macro sono generalmente piccole
                    "modules::default::macro_model::Macro" => {
                        info!("Allocazione in memoria Standard per Macro");
                        let size = 128; // 128 byte per macro semplici
                        memory_manager::allocate(Some(AllocationStrategy::Standard), size)?;
                        Ok(item)
                    }

                    // Dati dei sensori possono essere più complessi
                    "modules::default::sensor_data_model::SensorData" => {
                        info!("Allocazione CustomEmbedded per Sensor Data");
                        let size = 2048; // 2 KB per i dati dei sensori in tempo reale
                        memory_manager::allocate(Some(AllocationStrategy::CustomEmbedded), size)?;
                        Ok(item)
                    }

                    // Log/Event richiedono allocazione rapida e ripetuta, PoolBased
                    "modules::default::log_event_model::LogEvent" => {
                        info!("Allocazione PoolBased per Log/Event");
                        let size = 512; // 512 byte per log di eventi
                        memory_manager::allocate(Some(AllocationStrategy::PoolBased), size)?;
                        Ok(item)
                    }

                    // Comandi in tempo reale, CustomEmbedded per ottimizzare l'utilizzo locale
                    "modules::default::command_model::Command" => {
                        info!("Allocazione CustomEmbedded per Command");
                        let size = 256; // 256 byte per i comandi
                        memory_manager::allocate(Some(AllocationStrategy::CustomEmbedded), size)?;
                        Ok(item)
                    }

                    _ => {
                        info!("Allocazione in memoria Standard per modello non gestito specificamente");
                        let size = 256; // Default size se non specificato diversamente
                        memory_manager::allocate(Some(AllocationStrategy::Standard), size)?;
                        Ok(item)
                    }
                }
                
                Ok(item)
            }
        }

        impl Read<$model> for $model {
            fn read(id: u64) -> Result<$model, String> {
                // Simulazione della logica di lettura (recupero dal database) per ogni modello che implementa il trait con `match`statement
                Err(format!("Elemento con ID {} non trovato", id))
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

// Macro per implementare la ricerca e la revoca
macro_rules! impl_search_and_revoke {
    ($model:ty) => {
        impl Search<$model> for $model {
            fn search(query: &str) -> Vec<$model> {
                // Simulazione della logica di ricerca (filtraggio nel database) per ogni modello che implementa il trait con `match`statement
                vec![]
            }
        }

        impl Revoke for $model {
            fn revoke(id: u64) -> Result<(), String> {
                // Simulazione della logica di revoca (ad esempio, revoca token o permessi) per ogni modello che implementa il trait con `match`statement
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