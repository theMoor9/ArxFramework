#[cfg(feature = "automation")]
#[derive(Debug, Clone)]
pub struct ExecutionLog {
    pub id: u32,
    pub script_id: u32,
    pub execution_time: String,
    pub status: String,
    pub store: Allocation,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    List,
}

#[derive(Debug)]
pub enum Allocation {
    InMemory,
    Database,
}

impl ExecutionLog {
    pub fn new(id: u32, script_id: u32, execution_time: String, status: String) -> Self {
        Self {
            id,
            script_id,
            execution_time,
            status,
            store: Allocation::Database,
        }
    }
}