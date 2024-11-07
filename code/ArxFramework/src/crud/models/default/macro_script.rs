
pub mod model {    
    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;

    /// Enum per definire la frequenza di esecuzione delle macro
    #[derive(Debug, Clone)]
    pub enum ExecutionFrequency {
        Once,
        Daily,
        Weekly,
        Monthly,
        Custom(String), // Per frequenze personalizzate specificate dall'utente
    }

    /// Enum per lo stato delle macro
    #[derive(Debug, Clone)]
    pub enum MacroStatus {
        Active,
        Disabled,
        Completed,
    }

    #[cfg(feature = "automation")]
    #[derive(Debug, Clone)]
    pub struct Macro {
        pub id: u32,
        pub name: String,
        pub commands: Vec<String>,
        pub frequency: ExecutionFrequency,      // Frequenza di esecuzione pianificata
        pub status: MacroStatus,               // Stato della macro (attiva, disattivata, completata, ecc.)
        pub store: AllocType,
        pub memory: Box<[u8]>,
        pub ops: CrudOperations,
    }

    impl Macro {
        pub fn new(
            id: u32, 
            name: String, 
            commands: Vec<String>,
            frequency: ExecutionFrequency,
            status: MacroStatus,
            memory: Box<[u8]>,
        ) -> Self {
            Macro {
                id,
                name,
                commands,
                frequency,
                status,
                store: AllocType::InMemory,
                memory,
                ops: CrudOperations{
                    create: true,
                    read: true,
                    update: true,
                    delete: true,
                    list: true,
                    search: false,
                    revoke: false,
                } 
            }
        }
    }
}