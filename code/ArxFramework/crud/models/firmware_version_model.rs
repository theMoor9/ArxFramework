#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
pub struct FirmwareVersion {
    pub id: u32,
    pub device_id: u32,
    pub version: String,
    pub release_date: String,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
}
