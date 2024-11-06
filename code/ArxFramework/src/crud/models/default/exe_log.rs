pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "automation")]
    #[derive(Debug, Clone)]
    pub struct ExecutionLog {
        pub id: u32,
        pub script_id: u32,
        pub execution_time: String,
        pub status: String,
        pub store: AllocType,
    }
    
    impl ExecutionLog {
        pub fn new(id: u32, script_id: u32, execution_time: String, status: String) -> Self {
            Self {
                id,
                script_id,
                execution_time,
                status,
                store: AllocType::Database,
            }
        }
    }
}