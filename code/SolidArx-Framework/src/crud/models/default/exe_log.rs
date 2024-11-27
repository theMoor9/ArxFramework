pub mod model {
    
    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 

    cfg_if! {
        /* 
        #[cfg] Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */
        
        if #[cfg(feature = "automation")] {

            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;

            /// Enum per lo stato delle macro
            #[derive(Debug, Clone)]
            pub enum ExeLogStatus {
                Active,
                Disabled,
                Completed,
            }

            #[derive(Debug, Clone)]
            pub struct ExecutionLog {
                pub id: u32,
                pub script_id: u32,
                pub execution_time: String,
                pub status: ExeLogStatus,
                pub store: AllocType,
                pub ops: CrudOperations,
            }
            
            impl ExecutionLog {
                pub fn new(id: u32, script_id: u32, execution_time: String, status: ExeLogStatus) -> Self {
                    ExecutionLog {
                        id,
                        script_id,
                        execution_time,
                        status,
                        store: AllocType::Database,
                        ops: CrudOperations{
                            create: true,
                            read: true,
                            update: false,
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