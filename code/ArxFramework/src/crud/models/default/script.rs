pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;
    
    #[cfg(feature = "automation")]
    #[derive(Debug, Clone)]
    pub struct Script {
        pub id: u32,
        pub name: String,
        pub code: String,
        pub store: AllocType,
        pub ops: CrudOperations,
    }

    impl Script {
        pub fn new(id: u32, name: String, code: String) -> Self {
            Script { 
                id, 
                name,
                code,
                store: AllocType::Database,
                ops: CrudOperations{
                    create: true,
                    read: true,
                    update: true,
                    delete: true,
                    list: false,
                    search: true,
                    revoke: false,
                },
            }
        }
    }
}