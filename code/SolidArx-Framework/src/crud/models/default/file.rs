pub mod model {

    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 

    cfg_if! {
        /* 
        #[cfg] Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */
        if #[cfg(any(feature = "webapp", feature = "desktop"))] {

            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;

            #[derive(Debug, Clone)]
            pub struct File {
                pub id: u32,
                pub file_name: String,
                pub file_path: String,
                pub store: AllocType,
                pub ops: CrudOperations,
            }

            impl File {
                pub fn new(id: u32, file_name: String, file_path: String) -> Self {
                    File { 
                        id, 
                        file_name,
                        file_path,
                        store: AllocType::Database,
                        ops: CrudOperations{
                            create: true,
                            read: true,
                            update: true,
                            delete: true,
                            list: true,
                            search: true,
                            revoke: false,
                        } 
                    }
                }
            }
        }
    }
}