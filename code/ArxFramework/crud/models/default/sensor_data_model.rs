#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
pub struct SensorData {
    pub id: u32,
    pub device_id: u32,
    pub timestamp: String,
    pub data: String,  // Puoi specificare il formato dei dati se necessario
    pub store: Allocation
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

impl SensorData {
    pub fn new(id: u32, device_id: u32, timestamp: String, data: String) -> Self {
        Self {
            id,
            device_id,
            timestamp,
            data,
            store: Allocation::InMemory,
        }
    }
}