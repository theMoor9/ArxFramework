pub mod model {

    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 


    cfg_if! {
        /* 
        Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */
        if #[cfg(feature = "embedded")]{
            
            use crate::crud::crud_ops::AllocType;
            use crate::crud::crud_ops::CrudOperations;

            #[derive(Debug, Clone)]
            pub struct SensorData {
                pub id: u32,
                pub device_id: u32,
                pub timestamp: String,
                pub data: String,  // Puoi specificare il formato dei dati se necessario
                pub store: AllocType,
                pub memory: Box<[u8]>,
                pub ops: CrudOperations,
            }

            impl SensorData {
                pub fn new(
                    id: u32, 
                    device_id: u32, 
                    timestamp: String, 
                    data: String,
                    memory: Box<[u8]>,
                ) -> Self {
                    SensorData {
                        id,
                        device_id,
                        timestamp,
                        data,
                        store: AllocType::InMemory,
                        memory,
                        ops:CrudOperations{
                            create: true,
                            read: true,
                            update: false,
                            delete: true,
                            list: true,
                            search: true,
                            revoke: false,
                        },
                    }
                }
            }
        }
    }
}