#[cfg(feature = "api")]
#[derive(Debug, Clone)]
pub struct RateLimitRule {
    pub id: u32,
    pub limit: u32,
    pub period: String,
    pub store: Allocation,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Debug)]
pub enum Allocation {
    InMemory,
    Database,
}

impl RateLimitRule {
    pub fn new(id: u32, limit: u32, period: String) -> RateLimitRule {
        RateLimitRule {
            id,
            limit,
            period,
            store: Allocation::Database,
        }
    }
}