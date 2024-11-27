pub mod model {
    
    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 

    cfg_if! {
        /* 
        #[cfg] Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */
                
        if #[cfg(feature = "embedded")]{

            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;

            #[derive(Debug, Clone)]
            pub struct Device {
                pub id: u32,
                pub name: String,
                pub device_type: String,
                pub store: AllocType,
                pub memory: Box<[u8]>,
                pub ops: CrudOperations,
            }

            impl Device {
                pub fn new(id: u32, name: String, memory: Box<[u8]> ) -> Self {
                    Device { 
                        id, 
                        name,
                        device_type: "default".to_string(),
                        store: AllocType::InMemory,
                        memory,
                        ops: CrudOperations{
                            create: true,
                            read: true,
                            update: true,
                            delete: true,
                            list: true,
                            search: true,
                            revoke: true,
                        } 
                    }
                }
            }
        }
    }
}