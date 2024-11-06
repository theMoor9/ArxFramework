pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;
    
    #[cfg(feature = "embedded")]
    #[derive(Debug, Clone)]
    pub struct FirmwareVersion {
        pub id: u32,
        pub device_id: u32,
        pub version: String,
        pub release_date: String,
        pub store: AllocType,
    }

    impl FirmwareVersion {
        pub fn new(id: u32, device_id: u32, version: String, release_date: String) -> Self {
            Self {
                id,
                device_id,
                version,
                release_date,
                store: AllocType::Database,
            }
        }
    }
}