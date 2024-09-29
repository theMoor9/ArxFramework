#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
pub struct Command {
    pub id: u32,
    pub device_id: u32,
    pub command_type: String,
    pub issued_at: String,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
}
