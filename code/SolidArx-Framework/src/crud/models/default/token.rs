pub mod model {

    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 


    cfg_if! {
        /* 
        Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */
        if #[cfg(feature = "api")] {

            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;
            
            #[derive(Debug, Clone)]
            pub struct Token {
                pub id: u32,
                pub token: String,
                pub user_id: u32,
                pub store: AllocType,
                pub ops: CrudOperations,
            }

            impl Token {
                pub fn new(id: u32, token: String, user_id: u32) -> Self {
                    Token {
                        id,
                        token,
                        user_id,
                        store: AllocType::Database,
                        ops: CrudOperations{
                            create: true,
                            read: true,
                            update: false,
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