#[cfg(feature = "automation")]
#[derive(Debug, Clone)]
pub struct Schedule {
    pub id: u32,
    pub task_id: u32,
    pub cron_expression: String,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}
