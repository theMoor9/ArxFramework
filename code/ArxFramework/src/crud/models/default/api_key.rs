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
    }

    impl ApiKey {
        pub fn new(id:u32, key:String, user_id:u32) -> Self {
            Self {
                id,
                key,
                user_id,
                store: AllocType::Database,
            }
        }
    }
}