pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;
    
    #[cfg(feature = "embedded")]
    #[derive(Debug, Clone)]
    pub struct Device {
        pub id: u32,
        pub name: String,
        pub device_type: String,
        pub store: AllocType,
        pub memory: Box<[u8]>,
    }

    impl Device {
        pub fn new(id: u32, name: String, memory: Box<[u8]> ) -> Self {
            Self { 
                id, 
                name,
                device_type: "default".to_string(),
                store: AllocType::InMemory,
                memory,
            }
        }
    }
}