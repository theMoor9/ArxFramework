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
            pub struct Command {
                pub id: u32,
                pub device_id: u32,
                pub command_type: String,
                pub issued_at: String,
                pub store: AllocType,
                pub memory: Box<[u8]>,
                pub ops: CrudOperations,
            }

            impl Command {
                pub fn new(
                    id: u32, 
                    device_id: u32, 
                    command_type: String, 
                    issued_at: String,
                    memory: Box<[u8]>,
                ) -> Self {

                    Command {
                        id,
                        device_id,
                        command_type,
                        issued_at,
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
                        },
                    }
                }
            }
        }
    }
}