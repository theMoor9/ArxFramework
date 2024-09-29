#[cfg(feature = "api")]
#[derive(Debug, Clone)]
pub struct Permission {
    pub id: u32,
    pub name: String,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}
