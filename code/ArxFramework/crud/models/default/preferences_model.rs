#[cfg(feature = "desktop")]
#[derive(Debug, Clone)]
pub struct Preferences {
    pub id: u32,
    pub language: String,
    pub auto_save: bool,
    pub store Allocation,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
}

#[derive(Debug)]
pub enum Allocation {
    InMemory,
    Database,
}

impl Preferences {
    pub fn new(id: u32, language: String, auto_save: bool) -> Self {
        Self {
            id,
            language,
            auto_save,
            store Allocation::Database,
        }
    }
}