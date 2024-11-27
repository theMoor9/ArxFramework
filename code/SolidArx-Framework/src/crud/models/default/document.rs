pub mod model{

    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 

    cfg_if! {
        /* 
        #[cfg] Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */
    
        if #[cfg(feature = "desktop")] {
                    
            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;
                    
            #[derive(Debug, Clone)]
            pub struct Document {
                pub id: u32,
                pub title: String,
                pub content: String,
                pub store: AllocType,
                pub ops: CrudOperations,
            }

            impl Document {
                pub fn new(id: u32, title: String, content: String) -> Self {
                    Document {
                        id,
                        title,
                        content,
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