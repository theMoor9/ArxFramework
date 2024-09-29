#[cfg(any(feature = "webapp", feature = "desktop"))]
#[derive(Debug, Clone)]
pub struct File {
    pub id: u32,
    pub file_name: String,
    pub file_path: String,
}

#[cfg(any(feature = "webapp", feature = "desktop"))]
#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}
