pub mod model {
    
    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 


    cfg_if! {
        /* 
        Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */
        if #[cfg(feature = "desktop")] {
            
            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;

            #[derive(Debug, Clone)]
            pub struct Settings {
                pub id: u32,
                pub theme: String,
                pub notifications_enabled: bool,
                pub store: AllocType,
                pub ops: CrudOperations,
            }

            impl Settings {
                pub fn new(id: u32, theme: String, notifications_enabled: bool) -> Self {
                    Settings {
                        id,
                        theme,
                        notifications_enabled,
                        store: AllocType::Database,
                        ops:CrudOperations{
                            create: true,
                            read: true,
                            update: true,
                            delete: true,
                            list: true,
                            search: false,
                            revoke: false,
                        },
                    }
                }
            }
        }

    }
}