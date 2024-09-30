#[cfg(feature = "webapp")]
#[derive(Debug, Clone)]
pub struct Article {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub author_id: u32,
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

impl Article {
    pub fn new(id: u32, title: String, content: String, author_id: u32) -> Self {
        Self {
            id,
            title,
            content,
            author_id,
            store: Allocation::Database,
        }
    }
}