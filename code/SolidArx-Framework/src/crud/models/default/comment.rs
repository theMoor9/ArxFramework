pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "webapp")]
    #[derive(Debug, Clone)]
    pub struct Comment {
        pub id: u32,
        pub content: String,
        pub author_id: u32,
        pub article_id: u32,
        pub store: AllocType,
        pub ops: CrudOperations,
    }

    impl Comment {
        pub fn new(id: u32, content: String, author_id: u32, article_id: u32) -> Self {
            Comment {
                id,
                content,
                author_id,
                article_id,
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