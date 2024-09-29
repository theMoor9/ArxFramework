#[cfg(feature = "api")]
#[derive(Debug, Clone)]
pub struct Endpoint {
    pub id: u32,
    pub name: String,
    pub path: String,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}
