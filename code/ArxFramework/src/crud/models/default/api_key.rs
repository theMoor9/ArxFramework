pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "api")]
    #[derive(Debug, Clone)]
    pub struct ApiKey {
        pub id: u32,
        pub key: String,
        pub user_id: u32,
        pub store: AllocType,
        pub ops: CrudOperations,
    }

    impl ApiKey {
        pub fn new(id:u32, key:String, user_id:u32) -> Self {
            ApiKey {
                id,
                key,
                user_id,
                store: AllocType::Database,
                ops:CrudOperations{
                    create: true,
                    read: true,
                    update: false,
                    delete: true,
                    list: true,
                    search: false,
                    revoke: true,
                }
            }
        }
    }
}