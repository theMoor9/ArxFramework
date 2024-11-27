pub mod model {
    
    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 

    cfg_if! {
        /* 
        #[cfg] Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */
        if #[cfg(feature = "webapp")] {

            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;
            
            #[derive(Debug, Clone)]
            pub struct Comment {
                pub id: u32,
                pub content: String,
                pub author_id: u32,
                pub article_id: u32,
                pub store: AllocType,
                pub ops: CrudOperations,
            }

            impl Comment {
                pub fn new(id: u32, content: String, author_id: u32, article_id: u32) -> Self {
                    Comment {
                        id,
                        content,
                        author_id,
                        article_id,
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