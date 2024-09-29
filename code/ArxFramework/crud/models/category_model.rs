#[cfg(feature = "webapp")]
#[derive(Debug, Clone)]
pub struct Category {
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
