#[cfg(feature = "api")]
#[derive(Debug, Clone)]
pub struct Token {
    pub id: u32,
    pub token: String,
    pub user_id: u32,
    pub store: Allocation,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Revoke,
}

#[derive(Debug)]
pub enum Allocation {
    InMemory,
    Database,
}

impl Token {
    pub fn new(id: u32, token: String, user_id: u32) -> Self {
        Token {
            id,
            token,
            user_id,
            store: Allocation::Database,
        }
    }
}