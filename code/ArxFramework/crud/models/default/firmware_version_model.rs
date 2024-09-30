#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
pub struct FirmwareVersion {
    pub id: u32,
    pub device_id: u32,
    pub version: String,
    pub release_date: String,
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

impl FirmwareVersion {
    pub fn new(id: u32, device_id: u32, version: String, release_date: String) -> Self {
        Self {
            id,
            device_id,
            version,
            release_date,
            store: Allocation::Database,
        }
    }
}