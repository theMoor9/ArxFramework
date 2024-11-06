pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    #[cfg(feature = "api")]
    #[derive(Debug, Clone)]
    pub struct RequestLog {
        pub id: u32,
        pub endpoint: String,
        pub request_time: String,
        pub response_code: u16,
        pub store: AllocType,
    }

    impl RequestLog {
        pub fn new(id: u32, endpoint: String, request_time: String, response_code: u16) -> Self {
            Self {
                id,
                endpoint,
                request_time,
                response_code,
                store: AllocType::Database,
            }
        }
    }
}