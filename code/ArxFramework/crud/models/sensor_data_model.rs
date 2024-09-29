#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
pub struct SensorData {
    pub id: u32,
    pub device_id: u32,
    pub timestamp: String,
    pub data: String,  // Puoi specificare il formato dei dati se necessario
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    List,
}
