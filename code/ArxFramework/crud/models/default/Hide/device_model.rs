#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
pub struct Device {
    pub id: u32,
    pub name: String,
    pub device_type: String,
    pub store: Allocation,
    pub memory: Box<[u8]>,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Debug)]
pub enum Allocation {
    InMemory,
    Database,
}

impl Device {
    pub fn new(id: u32, name: String, memory: Box<[u8]> ) -> Self {
        Self { 
            id, 
            name,
            device_type: "default".to_string(),
            store: Allocation::InMemory,
            memory,
        }
    }
}