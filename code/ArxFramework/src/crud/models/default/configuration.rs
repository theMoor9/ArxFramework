pub mod model {

    use crate::crud::crud_ops::AllocType;
    use crate::crud::crud_ops::CrudOperations;
    
    #[cfg(any(feature = "automation", feature = "embedded"))]
    #[derive(Debug, Clone)]
    pub struct Configuration {
        pub id: u32,
        
        // Campo specifico per `embedded`, reso opzionale se non necessario
        #[cfg(feature = "embedded")]
        pub device_id: Option<u32>,
        
        pub key: String,
        pub value: String,
        pub store: AllocType,
        pub memory: Box<[u8]>,
        pub ops: CrudOperations,
    }

    impl Configuration {
        pub fn new(
            id: u32,
            #[cfg(feature = "embedded")] device_id: Option<u32>,
            key: String,
            value: String,
            memory: Box<[u8]>,
        ) -> Self {
            Configuration {
                id,
                #[cfg(feature = "embedded")]
                device_id,
                key,
                value,
                store: AllocType::InMemory,
                memory,
                ops: CrudOperations{
                    create: true,
                    read: true,
                    update: true,
                    delete: true,
                    list: false,
                    search: true,
                    revoke: true,
                } 
            }
        }
    }
}