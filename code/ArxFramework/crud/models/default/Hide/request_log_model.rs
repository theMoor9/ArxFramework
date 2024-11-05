#[cfg(feature = "api")]
#[derive(Debug, Clone)]
pub struct RequestLog {
    pub id: u32,
    pub endpoint: String,
    pub request_time: String,
    pub response_code: u16,
    pub store: Allocation,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    List,
}

#[derive(Debug)]
pub enum Allocation {
    InMemory,
    Database,
}

impl RequestLog {
    pub fn new(id: u32, endpoint: String, request_time: String, response_code: u16) -> Self {
        Self {
            id,
            endpoint,
            request_time,
            response_code,
            store: Allocation::Database,
        }
    }
}