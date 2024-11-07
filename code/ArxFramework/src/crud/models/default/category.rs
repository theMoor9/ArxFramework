pub mod model {    

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "webapp")]
    #[derive(Debug, Clone)]
    pub struct Category {
        pub id: u32,
        pub name: String,
        pub store: AllocType,
        pub ops: CrudOperations,
    }

    impl Category {
        pub fn new(id: u32, name: String) -> Self {
            Category {
                id,
                name,
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