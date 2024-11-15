pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "api")]
    #[derive(Debug, Clone)]
    pub struct Token {
        pub id: u32,
        pub token: String,
        pub user_id: u32,
        pub store: AllocType,
        pub ops: CrudOperations,
    }

    impl Token {
        pub fn new(id: u32, token: String, user_id: u32) -> Self {
            Token {
                id,
                token,
                user_id,
                store: AllocType::Database,
                ops: CrudOperations{
                    create: true,
                    read: true,
                    update: false,
                    delete: true,
                    list: true,
                    search: true,
                    revoke: true,
                },
            }
        }
    }
}