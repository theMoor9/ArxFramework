#[cfg(feature = "api")]
#[derive(Debug, Clone)]
pub struct ApiKey {
    pub id: u32,
    pub key: String,
    pub user_id: u32,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Revoke,
}
