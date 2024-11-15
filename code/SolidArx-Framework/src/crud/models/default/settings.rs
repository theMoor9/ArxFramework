pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;
    
    #[cfg(feature = "desktop")]
    #[derive(Debug, Clone)]
    pub struct Settings {
        pub id: u32,
        pub theme: String,
        pub notifications_enabled: bool,
        pub store: AllocType,
        pub ops: CrudOperations,
    }

    impl Settings {
        pub fn new(id: u32, theme: String, notifications_enabled: bool) -> Self {
            Settings {
                id,
                theme,
                notifications_enabled,
                store: AllocType::Database,
                ops:CrudOperations{
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