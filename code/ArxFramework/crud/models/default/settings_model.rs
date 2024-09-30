#[cfg(feature = "desktop")]
#[derive(Debug, Clone)]
pub struct Settings {
    pub id: u32,
    pub theme: String,
    pub notifications_enabled: bool,
    pub store: Allocation,
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

impl Settings {
    pub fn new(id: u32, theme: String, notifications_enabled: bool) -> Self {
        Self {
            id,
            theme,
            notifications_enabled,
            store: Allocation::Database,
        }
    }
}