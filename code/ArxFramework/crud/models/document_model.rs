#[cfg(feature = "desktop")]
#[derive(Debug, Clone)]
pub struct Document {
    pub id: u32,
    pub title: String,
    pub content: String,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}
