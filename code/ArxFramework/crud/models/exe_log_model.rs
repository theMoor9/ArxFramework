#[cfg(feature = "automation")]
#[derive(Debug, Clone)]
pub struct ExecutionLog {
    pub id: u32,
    pub script_id: u32,
    pub execution_time: String,
    pub status: String,
}

#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    List,
}
