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
    }

    impl SensorData {
        pub fn new(
            id: u32, 
            device_id: u32, 
            timestamp: String, 
            data: String,
            memory: Box<[u8]>,
        ) -> Self {
            Self {
                id,
                device_id,
                timestamp,
                data,
                store: AllocType::InMemory,
                memory,
            }
        }
    }
}