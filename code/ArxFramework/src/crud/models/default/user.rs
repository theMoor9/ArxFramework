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
        pub ops: CrudOperations,
    }

    impl User {
        pub fn new(id: u32, username: String, email: String, password: String) -> Self {

            User {
                id,
                username,
                email,
                password,
                store: AllocType::Database,
                ops: CrudOperations{
                    create: true,
                    read: true,
                    update: true,
                    delete: true,
                    list: true,
                    search: false,
                    revoke: false,
                },
            }
        }
    }
    
}