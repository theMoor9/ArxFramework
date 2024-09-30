#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
pub struct LogEvent {
    pub id: u32,
    pub device_id: u32,
    pub event_type: String,
    pub timestamp: String,
    pub description: String,
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

impl LogEvent {
    pub fn new(id: u32, device_id: u32, event_type: String, timestamp: String, description: String) -> LogEvent {
        LogEvent {
            id,
            device_id,
            event_type,
            timestamp,
            description,
            store: Allocation::InMemory,
        }
    }
}