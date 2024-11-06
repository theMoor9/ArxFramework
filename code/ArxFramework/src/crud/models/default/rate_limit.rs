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
    }

    impl RateLimitRule {
        pub fn new(id: u32, limit: u32, period: String) -> RateLimitRule {
            RateLimitRule {
                id,
                limit,
                period,
                store: AllocType::Database,
            }
        }
    }
}