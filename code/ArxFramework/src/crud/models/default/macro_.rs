
pub mod model {    
    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "automation")]
    #[derive(Debug, Clone)]
    pub struct Macro {
        pub id: u32,
        pub name: String,
        pub commands: Vec<String>,
        pub store: AllocType,
        pub memory: Box<[u8]>,
    }

    impl Macro {
        pub fn new(
            id: u32, 
            name: String, 
            commands: Vec<String>,
            memory: Box<[u8]>,
        ) -> Self {
            Self {
                id,
                name,
                commands,
                store: AllocType::InMemory,
                memory, 
            }
        }
    }
}