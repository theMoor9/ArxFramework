#[cfg(any(feature = "automation", feature = "embedded"))]
#[derive(Debug, Clone)]
pub struct Configuration {
    pub id: u32,
    
    // Campo specifico per `embedded`, reso opzionale se non necessario
    #[cfg(feature = "embedded")]
    pub device_id: Option<u32>,
    
    pub key: String,
    pub value: String,
    pub store: Allocation,
    pub memory: Box<[u8]>,
}

#[cfg(any(feature = "automation", feature = "embedded"))]
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

impl Configuration {
    pub fn new(
        id: u32,
        #[cfg(feature = "embedded")] device_id: Option<u32>,
        key: String,
        value: String,
        memory: Box<[u8]>,
    ) -> Self {
        Self {
            id,
            #[cfg(feature = "embedded")]
            device_id,
            key,
            value,
            store: Allocation::InMemory,
            memory,
        }
    }
}