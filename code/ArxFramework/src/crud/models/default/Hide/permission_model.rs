#[cfg(feature = "api")]
#[derive(Debug, Clone)]
pub struct Permission {
    pub id: u32,
    pub name: String,
    pub store Allocation,
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

impl Permission {
    pub fn new(id: u32, name: String) -> Self {
        Self { 
            id, 
            name,
            store Allocation::Database, 
        }
    }
}