pub mod model {

    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 


    cfg_if! {
        /* 
        #[cfg] Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */        
        if #[cfg(feature = "api")]{

            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;

            #[derive(Debug, Clone)]
            pub struct RequestLog {
                pub id: u32,
                pub endpoint: String,
                pub request_time: String,
                pub response_code: u16,
                pub store: AllocType,
                pub ops: CrudOperations,

            }

            impl RequestLog {
                pub fn new(id: u32, endpoint: String, request_time: String, response_code: u16) -> Self {
                    RequestLog {
                        id,
                        endpoint,
                        request_time,
                        response_code,
                        store: AllocType::Database,
                        ops:CrudOperations{
                            create: true,
                            read: true,
                            update: false,
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