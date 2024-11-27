pub mod model {

    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 


    cfg_if! {
        /* 
        Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */

        if #[cfg(feature = "automation")]{

            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;

            #[derive(Debug, Clone)]
            pub struct Script {
                pub id: u32,
                pub name: String,
                pub code: String,
                pub store: AllocType,
                pub ops: CrudOperations,
            }

            impl Script {
                pub fn new(id: u32, name: String, code: String) -> Self {
                    Script { 
                        id, 
                        name,
                        code,
                        store: AllocType::Database,
                        ops: CrudOperations{
                            create: true,
                            read: true,
                            update: true,
                            delete: true,
                            list: false,
                            search: true,
                            revoke: false,
                        },
                    }
                }
            }
        }
    }
}