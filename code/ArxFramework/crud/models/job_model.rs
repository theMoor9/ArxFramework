#[cfg(feature = "automation")]
#[derive(Debug, Clone)]
pub struct Job {
    pub id: u32,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}
