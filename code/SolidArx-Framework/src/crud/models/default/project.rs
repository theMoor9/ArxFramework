pub mod model {
    
    // Usato per incapsulare i blocchi in relazione al `cfg` attivo per la generazione di codice in compile time
    use cfg_if::cfg_if; 

    cfg_if! {
        /* 
        #[cfg] Seppur ridondante in relazione al `crud_ops` è necessario per rendere 
        la generazione delle tables selettiva per `table_scraper.rs` 
        */
        if #[cfg(feature = "automation")] {

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
                Disabled,
                Completed,
            }
            
            
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
    }
}