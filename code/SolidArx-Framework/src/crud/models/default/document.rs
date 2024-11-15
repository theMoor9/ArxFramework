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
        pub ops: CrudOperations,
    }

    impl Document {
        pub fn new(id: u32, title: String, content: String) -> Self {
            Document {
                id,
                title,
                content,
                store: AllocType::Database,
                ops: CrudOperations{
                    create: true,
                    read: true,
                    update: true,
                    delete: true,
                    list: true,
                    search: true,
                    revoke: false,
                } 
            }
        }
    }
}