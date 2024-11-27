pub mod model {
    
    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 

    cfg_if! {
        /* 
        #[cfg] Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */
        if #[cfg(feature = "desktop")]{

            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;

            #[derive(Debug, Clone)]
            pub struct Preferences {
                pub id: u32,
                pub language: String,
                pub auto_save: bool,
                pub store: AllocType,
                pub ops: CrudOperations,
            }

            impl Preferences {
                pub fn new(id: u32, language: String, auto_save: bool) -> Self {
                    Preferences {
                        id,
                        language,
                        auto_save,
                        store: AllocType::Database,
                        ops: CrudOperations{
                            create: true,
                            read: true,
                            update: true,
                            delete: true,
                            list: true,
                            search: false,
                            revoke: false,
                        }
                    }
                }
            }
        }
    }
}