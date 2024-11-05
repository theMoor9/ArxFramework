#[cfg(feature = "automation")]
#[derive(Debug, Clone)]
pub struct Script {
    pub id: u32,
    pub name: String,
    pub code: String,
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

impl Script {
    pub fn new(id: u32, name: String, code: String) -> Self {
        Self { 
            id, 
            name, 
            code,  
            store: Allocation::Database,
        }
    }
}