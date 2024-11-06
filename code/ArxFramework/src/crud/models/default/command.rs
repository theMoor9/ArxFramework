pub mod model {     

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "embedded")]
    #[derive(Debug, Clone)]
    pub struct Command {
        pub id: u32,
        pub device_id: u32,
        pub command_type: String,
        pub issued_at: String,
        pub store: AllocType,
        pub memory: Box<[u8]>,
    }

    impl Command {
        pub fn new(
            id: u32, 
            device_id: u32, 
            command_type: String, 
            issued_at: String,
            memory: Box<[u8]>,
        ) -> Self {
            Self {
                id,
                device_id,
                command_type,
                issued_at,
                store: AllocType::InMemory,
                memory,
            }
        }
    }
}