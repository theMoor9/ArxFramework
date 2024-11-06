pub mod model {    

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;
    
    #[cfg(feature = "embedded")]
    #[derive(Debug, Clone)]
    pub struct LogEvent {
        pub id: u32,
        pub device_id: u32,
        pub event_type: String,
        pub timestamp: String,
        pub description: String,
        pub store: AllocType,
        pub memory: Box<[u8]>,
    }

    impl LogEvent {
        pub fn new(
            id: u32, 
            device_id: u32, 
            event_type: String, 
            timestamp: String, 
            description: String,
            memory: Box<[u8]>,
        ) -> LogEvent {
            LogEvent {
                id,
                device_id,
                event_type,
                timestamp,
                description,
                store: AllocType::InMemory,
                memory,
            }
        }
    }
}