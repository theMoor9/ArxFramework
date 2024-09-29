#[cfg(feature = "api")]
#[derive(Debug, Clone)]
pub struct RateLimitRule {
    pub id: u32,
    pub limit: u32,
    pub period: String,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}
