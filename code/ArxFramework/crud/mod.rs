pub mod crud_ops;
pub mod models {
    pub mod dev;
    pub mod default{
        pub mod task_model;
        pub mod device_model;
        pub mod job_model;
        pub mod macro_model;
        pub mod sensor_data_model;
        pub mod log_event_model;
        pub mod command_model;
        pub mod configuration_model;
    }
}

pub fn initialize() -> Result<(), String> {
    // Logica di inizializzazione per CRUD
    println!("Initializing CRUD module...");
    Ok(())
}
