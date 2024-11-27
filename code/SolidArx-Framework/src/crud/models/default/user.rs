pub mod model {

    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 


    cfg_if! {
        /* 
        Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */
        if #[cfg(any(
                    feature = "webapp", 
                    feature = "api", 
                    feature = "desktop", 
                    feature = "automation", 
                    feature = "embedded"
                ))] {   

            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;

            #[derive(Debug, Clone)]
            pub struct User {
                pub id: u32,
                pub username: String,
                pub email: String,
                pub password: String,
                pub store: AllocType,
                pub ops: CrudOperations,
            }

            impl User {
                pub fn new(id: u32, username: String, email: String, password: String) -> Self {

                    User {
                        id,
                        username,
                        email,
                        password,
                        store: AllocType::Database,
                        ops: CrudOperations{
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