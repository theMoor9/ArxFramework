#[cfg(feature = "automation")]
#[derive(Debug, Clone)]
pub struct Schedule {
    pub id: u32,
    pub task_id: u32,
    pub cron_expression: String,
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

impl Schedule {
    pub fn new(id: u32, task_id: u32, cron_expression: String) -> Self {
        Self { 
            id, 
            task_id,
            cron_expression,
            store: Allocation::Database,
         }
    }
}