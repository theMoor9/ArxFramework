pub mod model {
    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;
    
    #[cfg(feature = "api")]
    #[derive(Debug, Clone)]
    pub struct RateLimitRule {
        pub id: u32,
        pub limit: u32,
        pub period: String,
        pub store: AllocType,
        pub ops: CrudOperations,
    }

    impl RateLimitRule {
        pub fn new(id: u32, limit: u32, period: String) -> RateLimitRule {

            RateLimitRule {
                id,
                limit,
                period,
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