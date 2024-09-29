#[cfg(feature = "desktop")]
#[derive(Debug, Clone)]
pub struct Settings {
    pub id: u32,
    pub theme: String,
    pub notifications_enabled: bool,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
}
