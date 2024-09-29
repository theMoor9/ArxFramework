#[cfg(feature = "webapp")]
#[derive(Debug, Clone)]
pub struct Comment {
    pub id: u32,
    pub content: String,
    pub author_id: u32,
    pub article_id: u32,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}
