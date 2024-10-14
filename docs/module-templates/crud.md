# crud/

|Module|Primary Language|Support Language|Wrapping|Main Frameworks/Libraries|Development Considerations|
|---|---|---|---|---|---|
|CRUD Management|Rust|-|-|diesel (ORM)|Repository design pattern, database abstraction|

---

## **Module Layer 1 for Code Base**

The **CRUD** module is located within the `modules/` directory, which contains two main subdirectories:

- **`modules/default/`**: This directory contains the standard predefined models necessary for the basic operation of the application. Each file within this directory follows the convention of being named with the `_model.rs` suffix and defines basic models such as `user_model.rs`, `article_model.rs`, and similar entities.
    
- **`modules/dev/`**: This directory is dedicated to models that the developer can add based on the specific needs of the project. These models, also with the `_model.rs` suffix, can extend or customize the behavior of predefined models, or define entirely new ones.
    

---

## **File: `models/dev/userexample_model.rs`**

### **Model Structure**

Each `.rs` file in the `modules/` directory has the task of:

1. **Defining the model structure**: These models describe the entities with which the application will interact. The structures define the fields associated with each model (for example, a user could have an `id`, `username`, and `email`).
    
2. **Available operations**: The models include an enum `CrudOperations` that defines which CRUD operations (Create, Read, Update, Delete) are available for each specific model. Operations may vary depending on the context (for example, a model might have the `Delete` operation only for APIs).
    
3. **Memory Type**: Each model specifies the type of memory it should be stored in, which could be:
    
    - **Allocated memory** (`InMemory`): The model's data is managed temporarily in memory, useful for volatile or temporary tasks.
    - **Database**: The data is stored and managed persistently in the database, useful for entities that need to be saved long-term.

```Rust
// Example of a model definition following the standard

#[cfg(any(feature = "webapp", feature = "api", feature = "desktop", feature = "automation", feature = "embedded"))]
#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub additional_field: ...,
    // Add more fields as needed
    #[cfg(feature = "api")]
    pub exclusive_to_api: ...,
}

#[cfg(any(feature = "webapp", feature = "api", feature = "desktop", feature = "automation", feature = "embedded"))]
#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    // Add other operations as needed
}

```

### **Usage**

**Adding conditional fields for a specific feature**: To add fields that should exist only in certain applications, use the `#[cfg(...)]` attribute. For example, the `exclusive_to_api` field is added only if the `api` feature is enabled:

```Rust
#[cfg(feature = "api")] pub exclusive_to_api: ...,
```

Thus, the `exclusive_to_api` field will only be included when you compile the project with the `api` feature:

```Rust
cargo build --features "api"
```

---

## **File: `crud_op.rs`**

The **`crud_ops.rs`** file contains the logic that implements the CRUD operations for the models defined in **`modules/`**. Its primary purpose is to manage the insertion, updating, and deletion of data in two memory types:

- **Allocated Memory (InMemory)**: The CRUD operations interact directly with the data stored in memory during the application's runtime. This approach is mainly used for temporary or volatile tasks.
    
- **Database**: CRUD operations interact with a persistent database, ensuring that the data remains saved even after the application is closed or restarted.
    

The **`crud_ops.rs`** module handles the centralized logic needed to insert, update, and delete data in both contexts, ensuring that the behavior is consistent based on the memory type selected for each model.

Additionally, **`crud_ops.rs`** serves as an intermediary between the user interface (or API interface) and the actual data management, translating user inputs into concrete actions on the data.

```Rust
// crud_ops.rs

use crate::models::*; // Import all models

pub trait Create<T> {
    fn create(item: T) -> Result<T, String>;
}

pub trait Read<T> {
    fn read(id: u64) -> Result<T, String>;
}

pub trait Update<T> {
    fn update(item: T) -> Result<T, String>;
}

pub trait Delete {
    fn delete(id: u64) -> Result<(), String>;
}

pub trait List<T> {
    fn list() -> Vec<T>;
}

pub trait Search<T> {
    fn search(query: &str) -> Vec<T>;
}

pub trait Revoke {
    fn revoke(id: u64) -> Result<(), String>;
}

// Macro to implement common CRUD traits
macro_rules! impl_crud_ops {
    ($model:ty) => {
        impl Create<$model> for $model {
            fn create(item: $model) -> Result<$model, String> {
                // Simulated create logic (insert into database) for each model implementing the trait with `match` statement
                Ok(item)
            }
        }

        impl Read<$model> for $model {
            fn read(id: u64) -> Result<$model, String> {
                // Simulated read logic (retrieve from database) for each model implementing the trait with `match` statement
                Err(format!("Element with ID {} not found", id))
            }
        }

        impl Update<$model> for $model {
            fn update(item: $model) -> Result<$model, String> {
                // Simulated update logic (modify in database) for each model implementing the trait with `match` statement
                Ok(item)
            }
        }

        impl Delete for $model {
            fn delete(id: u64) -> Result<(), String> {
                // Simulated delete logic (remove from database) for each model implementing the trait with `match` statement
                Ok(())
            }
        }

        impl List<$model> for $model {
            fn list() -> Vec<$model> {
                // Simulated list logic for all elements for each model implementing the trait with `match` statement
                vec![]
            }
        }
    };
}

// Macro to implement Search and Revoke traits
macro_rules! impl_search_and_revoke {
    ($model:ty) => {
        impl Search<$model> for $model {
            fn search(query: &str) -> Vec<$model> {
                // Simulated search logic (filter in database) for each model implementing the trait with `match` statement
                vec![]
            }
        }

        impl Revoke for $model {
            fn revoke(id: u64) -> Result<(), String> {
                // Simulated revoke logic (e.g., revoke token or permissions) for each model implementing the trait with `match` statement
                Ok(())
            }
        }
    };
}

// Apply macros to models

impl_crud_ops!(User);
impl_crud_ops!(Article);
impl_crud_ops!(Comment);

// Add more models for generic CRUD application

// Example usage of models with Search and Revoke traits
impl_search_and_revoke!(User);
impl_search_and_revoke!(Article);

// Add more models as necessary

```

### **Usage Example**

```Rust
extern crate crud;

use crud::crud_ops::Create;
use crud::models::User;

fn main() {
    let new_user = User { id: 1, name: "John".to_string(), email: "john@example.com".to_string() };
    let result = User::create(new_user);
    println!("{:?}", result);
}
```

---

## **Adding a New Model**

To add a new `.rs` model:

1. **Create the model file**: Within the `models/` folder, create an `.rs` file with the model's name, for example, `post_model.rs`, `id_model.rs`, `character_model.rs`, etc.
    
2. **Define the model struct**: Define the model structure with the necessary fields. Use `#[cfg(...)]` to handle fields specific to different applications (e.g., Web App, API).
    

```Rust
#[cfg(any(feature = "webapp", feature = "api"))]
#[derive(Debug, Clone)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    #[cfg(feature = "api")]
    pub api_specific_field: Option<String>,
}
```

3. **Add the CrudOperations enum**: Define a `CrudOperations` enum to indicate the CRUD operations supported by the model.

```Rust
#[cfg(any(feature = "webapp", feature = "api"))]
#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
    #[cfg(feature = "api")]
    Delete, // Only for API apps
}
```

4. **Update existing implementations as needed**: After adding the new model, ensure that you update any existing implementations that may depend on the new model or require adjustments for it to function correctly.

For example, you may need to modify the CRUD functions in the system to handle the new model or update references in configuration files.


```Rust
impl Post {
    pub fn new(
        id: u32,
        title: String,
        content: String,
        #[cfg(feature = "api")] api_specific_field: Option<String>
    ) -> Self {
        Self {
            id,
            title,
            content,
            #[cfg(feature = "api")]
            api_specific_field, // Optional field for API
        }
    }
}
```

### **In `crud_ops.rs`**

1. The implementation differs depending on the memory allocation type chosen for the model.
    
    - For **InMemory** allocation, customize the allocation type based on the criteria defined in `memory_management.rs` within the `impl_crud_ops{...}` as follows:
        
        - `allocate()`: Implement within the `match` of `create()` accordingly.
        
	```Rust
	"modules::default::task_model::Task" => {
	    info!("Standard memory allocation for Task");
	    let size = 256; // 256 bytes for temporary tasks
	    memory_manager::allocate(Some(AllocationStrategy::Standard), size)?;
	    Ok(item)
	}
	```
        
    - For **Database** allocation, no additional implementation is required in `impl_crud_ops{...}`.
        
2. **Import the model into the CRUD system**: In `crud_ops.rs`, use the macro to implement the CRUD operations for the new model:
    

```Rust
impl_crud_ops!(Post);
```

**Compile with the desired feature**: When compiling the project, make sure to automatically enable the necessary features using:


```sh
cargo build --features "webapp"
```