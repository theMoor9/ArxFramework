#[cfg(feature = "api")]
#[derive(Debug, Clone)]
pub struct Token {
    pub id: u32,
    pub token: String,
    pub user_id: u32,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Revoke,
}
