pub mod model {
    
    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 


    cfg_if! {
        /* 
        #[cfg] Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */
        if #[cfg(feature = "api")] {

            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;
            
            #[derive(Debug, Clone)]
            pub struct RateLimitRule {
                pub id: u32,
                pub limit: u32,
                pub period: String,
                pub store: AllocType,
                pub ops: CrudOperations,
            }

            impl RateLimitRule {
                pub fn new(id: u32, limit: u32, period: String) -> RateLimitRule {

                    RateLimitRule {
                        id,
                        limit,
                        period,
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