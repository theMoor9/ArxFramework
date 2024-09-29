#[cfg(any(feature = "automation", feature = "embedded"))]
#[derive(Debug, Clone)]
pub struct Configuration {
    pub id: u32,
    
    // Campo specifico per `embedded`, reso opzionale se non necessario
    #[cfg(feature = "embedded")]
    pub device_id: Option<u32>,
    
    pub key: String,
    pub value: String,
}

#[cfg(any(feature = "automation", feature = "embedded"))]
#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    Delete,
}
