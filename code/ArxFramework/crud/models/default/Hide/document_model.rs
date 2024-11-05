#[cfg(feature = "desktop")]
#[derive(Debug, Clone)]
pub struct Document {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub store: Allocation,
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

impl Document {
    pub fn new(id: u32, title: String, content: String) -> Self {
        Self {
            id,
            title,
            content,
            store: Allocation::Database,
        }
    }
}