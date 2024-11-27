pub mod model {    
    
    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 

    cfg_if! {
        /* 
        #[cfg] Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
                */
        if #[cfg(feature = "embedded")] {

            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;
            
            #[derive(Debug, Clone)]
            pub struct LogEvent {
                pub id: u32,
                pub device_id: u32,
                pub event_type: String,
                pub timestamp: String,
                pub description: String,
                pub store: AllocType,
                pub memory: Box<[u8]>,
                pub ops: CrudOperations,
            }

            impl LogEvent {
                pub fn new(
                    id: u32, 
                    device_id: u32, 
                    event_type: String, 
                    timestamp: String, 
                    description: String,
                    memory: Box<[u8]>,
                ) -> LogEvent {
                    LogEvent {
                        id,
                        device_id,
                        event_type,
                        timestamp,
                        description,
                        store: AllocType::InMemory,
                        memory,
                        ops: CrudOperations{
                            create: true,
                            read: true,
                            update: false,
                            delete: false,
                            list: false,
                            search: true,
                            revoke: false,
                        } 
                    }
                }
            }
        }
    }
}