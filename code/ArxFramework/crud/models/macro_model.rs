#[cfg(feature = "automation")]
#[derive(Debug, Clone)]
pub struct Macro {
    pub id: u32,
    pub name: String,
    pub commands: Vec<String>,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}
