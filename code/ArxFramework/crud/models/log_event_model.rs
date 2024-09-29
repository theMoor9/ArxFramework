#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
pub struct LogEvent {
    pub id: u32,
    pub device_id: u32,
    pub event_type: String,
    pub timestamp: String,
    pub description: String,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    List,
}
