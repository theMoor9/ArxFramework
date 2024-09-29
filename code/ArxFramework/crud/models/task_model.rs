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
}

#[cfg(any(feature = "automation", feature = "desktop", feature = "embedded"))]
#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}

