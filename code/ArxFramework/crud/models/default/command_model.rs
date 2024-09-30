#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
pub struct Command {
    pub id: u32,
    pub device_id: u32,
    pub command_type: String,
    pub issued_at: String,
    pub store: Allocation,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
}

#[derive(Debug)]
pub enum Allocation {
    InMemory,
    Database,
}

impl Command {
    pub fn new(id: u32, device_id: u32, command_type: String, issued_at: String) -> Self {
        Self {
            id,
            device_id,
            command_type,
            issued_at,
            store: Allocation::InMemory,
        }
    }
}