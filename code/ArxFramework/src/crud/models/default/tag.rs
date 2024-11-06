pub mod model {
    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "webapp")]
    #[derive(Debug, Clone)]
    pub struct Tag {
        pub id: u32,
        pub name: String,
        pub store: AllocType,
    }

    impl Tag {
        pub fn new(id: u32, name: String) -> Self {
            Self { 
                id, 
                name,
                store: AllocType::Database,
            }
        }
    }
}