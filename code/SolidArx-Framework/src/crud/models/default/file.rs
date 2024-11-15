pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(any(feature = "webapp", feature = "desktop"))]
    #[derive(Debug, Clone)]
    pub struct File {
        pub id: u32,
        pub file_name: String,
        pub file_path: String,
        pub store: AllocType,
        pub ops: CrudOperations,
    }

    impl File {
        pub fn new(id: u32, file_name: String, file_path: String) -> Self {
            File { 
                id, 
                file_name,
                file_path,
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