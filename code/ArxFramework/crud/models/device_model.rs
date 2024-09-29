#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
pub struct Device {
    pub id: u32,
    pub name: String,
    pub device_type: String,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}
