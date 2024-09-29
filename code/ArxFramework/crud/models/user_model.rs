#[cfg(any(feature = "webapp", feature = "api", feature = "desktop", feature = "automation", feature = "embedded"))]
#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[cfg(any(feature = "webapp", feature = "api", feature = "desktop", feature = "automation", feature = "embedded"))]
#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}