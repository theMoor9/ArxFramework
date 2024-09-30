#[cfg(feature = "webapp")]
#[derive(Debug, Clone)]
pub struct Comment {
    pub id: u32,
    pub content: String,
    pub author_id: u32,
    pub article_id: u32,
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

impl Comment {
    pub fn new(id: u32, content: String, author_id: u32, article_id: u32) -> Self {
        Self {
            id,
            content,
            author_id,
            article_id,
            store: Allocation::Database,
        }
    }
}