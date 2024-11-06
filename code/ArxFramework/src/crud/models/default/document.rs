pub mod model{
    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "desktop")]
    #[derive(Debug, Clone)]
    pub struct Document {
        pub id: u32,
        pub title: String,
        pub content: String,
        pub store: AllocType,
    }

    impl Document {
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