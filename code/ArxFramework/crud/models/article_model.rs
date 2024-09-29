#[cfg(feature = "webapp")]
#[derive(Debug, Clone)]
pub struct Article {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub author_id: u32,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}
