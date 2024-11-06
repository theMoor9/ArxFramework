pub mod model {
    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

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

        pub store: AllocType,
        pub memory: Box<[u8]>,
    }

    impl Task {
        pub fn new(
            id: u32,
            description: String,
            #[cfg(feature = "automation")] schedule: Option<String>,
            #[cfg(feature = "desktop")] completed: Option<bool>,
            #[cfg(feature = "embedded")] device_id: Option<u32>,
            memory: Box<[u8]>,
        ) -> Self {
            Task {
                id,
                description,
                #[cfg(feature = "automation")]
                schedule,
                #[cfg(feature = "desktop")]
                completed,
                #[cfg(feature = "embedded")]
                device_id,
                store: AllocType::InMemory,
                memory,
            }
        }
    }
}