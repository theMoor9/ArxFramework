pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "embedded")]
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