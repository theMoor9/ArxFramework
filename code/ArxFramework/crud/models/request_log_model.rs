#[cfg(feature = "api")]
#[derive(Debug, Clone)]
pub struct RequestLog {
    pub id: u32,
    pub endpoint: String,
    pub request_time: String,
    pub response_code: u16,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    List,
}
