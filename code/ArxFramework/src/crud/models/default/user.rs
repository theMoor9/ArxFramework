pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(any(feature = "webapp", feature = "api", feature = "desktop", feature = "automation", feature = "embedded"))]
    #[derive(Debug, Clone)]
    pub struct User {
        pub id: u32,
        pub username: String,
        pub email: String,
        pub password: String,
        pub store: AllocType,
    }

    impl User {
        pub fn new(id: u32, username: String, email: String, password: String) -> Self {
            Self {
                id,
                username,
                email,
                password,
                store: AllocType::Database,
            }
        }
    }
    
}