#[cfg(any(feature = "automation", feature = "desktop", feature = "embedded"))]
#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,

    // Campo specifico per `automation`
    #[cfg(feature = "automation")]
    pub schedule: Option<String>,  // Usato per schedulazioni come cron jobs

    // Campo specifico per `desktop`
    #[cfg(feature = "desktop")]
    pub completed: Option<bool>,  // Indica se il task è completato

    // Campo specifico per `embedded`
    #[cfg(feature = "embedded")]
    pub device_id: Option<u32>,  // Associa il task a un dispositivo

    pub store: Allocation,
}

#[cfg(any(feature = "automation", feature = "desktop", feature = "embedded"))]
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

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            #[cfg(feature = "automation")]
            schedule: None,
            #[cfg(feature = "desktop")]
            completed: None,
            #[cfg(feature = "embedded")]
            device_id: None,
            store: Allocation::InMemory,
        }
    }
}
