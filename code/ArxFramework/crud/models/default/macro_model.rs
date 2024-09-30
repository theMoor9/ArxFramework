#[cfg(feature = "automation")]
#[derive(Debug, Clone)]
pub struct Macro {
    pub id: u32,
    pub name: String,
    pub commands: Vec<String>,
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

impl Macro {
    pub fn new(id: u32, name: String, commands: Vec<String>) -> Self {
        Self {
            id,
            name,
            commands,
            store Allocation::InMemory,
        }
    }
}