#[cfg(any(feature = "webapp", feature = "desktop"))]
#[derive(Debug, Clone)]
pub struct File {
    pub id: u32,
    pub file_name: String,
    pub file_path: String,
    pub store: Allocation,
}

#[cfg(any(feature = "webapp", feature = "desktop"))]
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

impl File {
    pub fn new(id: u32, file_name: String, file_path: String) -> Self {
        Self { 
            id, 
            file_name,
            file_path,
            store: Allocation::Database,
        }
    }
}