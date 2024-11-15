pub mod model {
    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;
    
    #[cfg(feature = "automation")]
    #[derive(Debug, Clone)]
    pub struct Schedule {
        pub id: u32,
        pub task_id: u32,
        pub cron_expression: String,
        pub store: AllocType,
        pub ops: CrudOperations,
    }

    impl Schedule {
        pub fn new(id: u32, task_id: u32, cron_expression: String) -> Self {
            Schedule { 
                id, 
                task_id,
                cron_expression,
                store: AllocType::Database,
                ops: CrudOperations{
                    create: true,
                    read: true,
                    update: true,
                    delete: true,
                    list: true,
                    search: true,
                    revoke: true,
                },
            }
        }
    }
}