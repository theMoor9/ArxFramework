pub mod model {
    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "desktop")]
    #[derive(Debug, Clone)]
    pub struct Preferences {
        pub id: u32,
        pub language: String,
        pub auto_save: bool,
        pub store: AllocType,
    }

    impl Preferences {
        pub fn new(id: u32, language: String, auto_save: bool) -> Self {
            Self {
                id,
                language,
                auto_save,
                store: AllocType::Database,
            }
        }
    }
}