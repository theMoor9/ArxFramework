#[cfg(feature = "automation")]
#[derive(Debug, Clone)]
pub struct Job {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub store Allocation,
    pub memory: Box<[u8]>,
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

impl Job {
    pub fn new(
        id: u32, 
        name: String, 
        description: String,
        memory: Box<[u8]>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            store Allocation::InMemory,
            memory,
        }   
    }
}