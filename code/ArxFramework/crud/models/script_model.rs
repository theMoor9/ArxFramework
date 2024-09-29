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
