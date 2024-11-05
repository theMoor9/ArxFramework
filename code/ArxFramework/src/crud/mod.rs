pub mod crud_ops;
pub mod models{
    pub mod default{
        pub mod task_model;
    }
    pub mod dev;
}

pub fn initialize() -> Result<(), String> {
    // Logica di inizializzazione per CRUD
    println!("Initializing CRUD module...");
    Ok(())
}

