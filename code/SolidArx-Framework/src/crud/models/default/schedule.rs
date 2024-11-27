pub mod model {
    
    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 


    cfg_if! {
        /* 
        #[cfg] Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */
        if #[cfg(feature = "automation")]{

            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;

            #[derive(Debug, Clone)]
            pub struct Schedule {
                pub id: u32,
                pub task_id: u32,
                pub cron_expression: String,
                pub store: AllocType,
                pub ops: CrudOperations,
            }

            impl Schedule {
                pub fn new(id: u32, task_id: u32, cron_expression: String) -> Self {
                    Schedule { 
                        id, 
                        task_id,
                        cron_expression,
                        store: AllocType::Database,
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