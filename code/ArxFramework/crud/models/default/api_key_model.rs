#[cfg(feature = "api")]
#[derive(Debug, Clone)]
pub struct ApiKey {
    pub id: u32,
    pub key: String,
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

impl user {
    pub fn new(id:u32, key:String, user_id:u32) -> Self {
        Self {
            id,
            key,
            user_id,
            store: Allocation::Database,
        }
    }
}