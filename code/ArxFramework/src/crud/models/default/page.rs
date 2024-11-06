pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "webapp")]
    #[derive(Debug, Clone)]
    pub struct Page {
        pub id: u32,
        pub title: String,
        pub content: String,
        pub store: AllocType,
    }
    
    impl Page {
        pub fn new(id: u32, title: String, content: String) -> Self {
            Self {
                id,
                title,
                content,
                store: AllocType::Database,
            }
        }
    }
}