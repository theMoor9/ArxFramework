pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;
    use chrono;


    // Struttura opzionale per i metadati del progetto
    #[derive(Debug, Clone)]
    pub struct ProjectMetadata {
        pub description: Option<String>,      // Descrizione del progetto
        pub tags: Option<Vec<String>>,        // Tag o etichette per la classificazione
        pub version: Option<String>,          // Versione del progetto
        pub contributors: Option<Vec<String>>, // Nomi o ID dei contributori
    }
    // Enum per definire i possibili stati del progetto
    #[derive(Debug, Clone)]
    pub enum ProjectStatus {
        Active,
        Paused,
        Completed,
    }
    
    #[cfg(feature = "automation")]
    #[derive(Debug, Clone)]
    pub struct Project {
        pub id: u32,
        pub name: String,
        pub path: String,
        pub created_at: chrono::NaiveDateTime,
        pub updated_at: chrono::NaiveDateTime,
        pub status: ProjectStatus,           // Stato del progetto
        pub metadata: Option<ProjectMetadata>,
        pub store: AllocType,
        pub ops: CrudOperations,
    }

    

    impl Project {
        pub fn new(id: u32, name: String, path: String, status: ProjectStatus, metadata: Option<ProjectMetadata>) -> Self {
            Project { 
                id, 
                name, 
                path, 
                created_at: chrono::Local::now().naive_local(),
                updated_at: chrono::Local::now().naive_local(),
                status, 
                metadata,
                store: AllocType::Database,
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