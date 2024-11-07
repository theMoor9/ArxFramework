pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;
    
    #[cfg(feature = "webapp")]
    #[derive(Debug, Clone)]
    pub struct Article {
        pub id: u32,
        pub title: String,
        pub content: String,
        pub author_id: u32,
        pub store: AllocType,
        pub ops: CrudOperations,
    }

    impl Article {
        pub fn new(id: u32, title: String, content: String, author_id: u32) -> Self {
            Article {
                id,
                title,
                content,
                author_id,
                store: AllocType::Database,
                ops: CrudOperations{
                    create: true,
                    read: true,
                    update: true,
                    delete: true,
                    list: true,
                    search: true,
                    revoke: false,
                },
            }
        }
    }
}