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
        pub ops: CrudOperations,

    }

    impl RequestLog {
        pub fn new(id: u32, endpoint: String, request_time: String, response_code: u16) -> Self {
            RequestLog {
                id,
                endpoint,
                request_time,
                response_code,
                store: AllocType::Database,
                ops:CrudOperations{
                    create: true,
                    read: true,
                    update: false,
                    delete: true,
                    list: true,
                    search: false,
                    revoke: false,
                },
            }
        }
    }
}