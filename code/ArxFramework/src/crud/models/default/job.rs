pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "automation")]
    #[derive(Debug, Clone)]
    pub struct Job {
        pub id: u32,
        pub name: String,
        pub description: String,
        pub store: AllocType,
        pub memory: Box<[u8]>,
    }

    impl Job {
        pub fn new(
            id: u32, 
            name: String, 
            description: String,
            memory: Box<[u8]>,
        ) -> Self {
            Self {
                id,
                name,
                description,
                store: AllocType::InMemory,
                memory,
            }   
        }
    }
}