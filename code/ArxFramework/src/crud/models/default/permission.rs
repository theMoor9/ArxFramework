pub mod model {
    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "api")]
    #[derive(Debug, Clone)]
    pub struct Permission {
        pub id: u32,
        pub name: String,
        pub store: AllocType,
    }


    impl Permission {
        pub fn new(id: u32, name: String) -> Self {
            Self { 
                id, 
                name,
                store: AllocType::Database, 
            }
        }
    }
}