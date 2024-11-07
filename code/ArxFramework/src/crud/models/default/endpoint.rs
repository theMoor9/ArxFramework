pub mod model{

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "api")]
    #[derive(Debug, Clone)]
    pub struct Endpoint {
        pub id: u32,
        pub name: String,
        pub path: String,
        pub store: AllocType,
    }

    impl Endpoint {
        pub fn new(id: u32, name: String, path: String) -> Self {
            Self { 
                id, 
                name,
                path,
                store: AllocType::Database,
            }
        }
    }
}