#[cfg(feature = "desktop")]
#[derive(Debug, Clone)]
pub struct Preferences {
    pub id: u32,
    pub language: String,
    pub auto_save: bool,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
}
