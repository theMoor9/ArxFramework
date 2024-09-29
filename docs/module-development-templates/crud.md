# crud/

| Modulo        | Linguaggio Principale | Linguaggio di Supporto | Wrapping | Framework/Librerie Principali | Considerazioni per lo Sviluppo<br>                     |
| ------------- | --------------------- | ---------------------- | -------- | ----------------------------- | ------------------------------------------------------ |
| Gestione CRUD | Rust                  | -                      | -        | diesel (ORM)                  | Design pattern Repository, astrazione del database<br> |

#Modulo-Layer-1-per-Code-Base 

---
# Aggiunta nuovo modello

Per aggiungere un nuovo modello `.rs` :

1. **Crea il file del modello**: All'interno della cartella `models/`, crea un file `.rs` con il nome del modello, ad esempio `post_model.rs`, `id_model.rs`, `character_model.rs`, ecc... 
    
2. **Definisci la struct del modello**: Definisci la struttura del modello con i campi necessari. Usa `#[cfg(...)]` per gestire campi specifici per diverse applicazioni (ad esempio, Web App, API, ecc.).

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

3. **Aggiungi l'enum CrudOperations**: Definisci un enum `CrudOperations` per indicare le operazioni CRUD supportate dal modello.
    
    ```Rust
	#[cfg(any(feature = "webapp", feature = "api"))] #[derive(Debug)] 
	pub enum CrudOperations {
		Create,
		Read,
	    Update,
	    #[cfg(feature = "api")] 
	    Delete, // solo per api apps
	}
	```
    
4. **Importa il modello nel sistema CRUD**: Nel file `crud_ops.rs`, usa la macro per implementare le operazioni CRUD per il nuovo modello:

    ```Rust
	impl_crud_ops!(Post);
	```
    
    
**Compila con la feature desiderata**: Al momento della compilazione il progetto abiliterà automaticamente le feature necessarie con :
	
    `cargo build --features "webapp"`

---

# `models/userexample_model.rs`

```Rust
// Definizione dei modello secondo standard

#[cfg(any(feature = "webapp", feature = "api", feature = "desktop", feature = "automation", feature = "embedded"))]
#[derive(Debug, Clone)]
pub struct user {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub whatever_you_want_to_add: ...,
    // Puoi aggiungere anche altri campi come necessario
    #[cfg(feature = "api")]
    pub exclusive_to_api: ...,
}

#[cfg(any(feature = "webapp", feature = "api", feature = "desktop", feature = "automation", feature = "embedded"))]
#[derive(Debug)]
pub enum CrudOperations {
    Create,
    Read,
    Update,
	 // Puoi aggiungere anche altre operazioni come necessario
}
```

**Aggiungere campi condizionali per una specifica feature**: Per aggiungere campi che devono esistere solo in alcune applicazioni, puoi usare l'attributo `#[cfg(...)]`. Ad esempio, il campo `exclusive_to_api` viene aggiunto solo se la feature `api` è abilitata:

```Rust
#[cfg(feature = "api")]
pub exclusive_to_api: ...,
```

In questo modo, il campo `exclusive_to_api` sarà incluso solo quando compili il progetto con la feature `api`:

```bash
cargo build --features "api"
```

---

# `crud_op.rs`

```Rust
// crud_ops.rs

use crate::models::*; // Importa tutti i modelli

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

// Macro per implementare i trait CRUD comuni
macro_rules! impl_crud_ops {
    ($model:ty) => {
        impl Create<$model> for $model {
            fn create(item: $model) -> Result<$model, String> {
                // Simulazione della logica di creazione (inserimento nel database) per ogni modello che implementa il trait con `match`statement
                Ok(item)
            }
        }

        impl Read<$model> for $model {
            fn read(id: u64) -> Result<$model, String> {
                // Simulazione della logica di lettura (recupero dal database) per ogni modello che implementa il trait con `match`statement
                Err(format!("Elemento con ID {} non trovato", id))
            }
        }

        impl Update<$model> for $model {
            fn update(item: $model) -> Result<$model, String> {
                // Simulazione della logica di aggiornamento (modifica nel database) per ogni modello che implementa il trait con `match`statement
                Ok(item)
            }
        }

        impl Delete for $model {
            fn delete(id: u64) -> Result<(), String> {
                // Simulazione della logica di eliminazione (rimozione dal database) per ogni modello che implementa il trait con `match`statement
                Ok(())
            }
        }

        impl List<$model> for $model {
            fn list() -> Vec<$model> {
                // Simulazione della logica di elenco di tutti gli elementi  per ogni modello che implementa il trait con `match`statement
                vec![]
            }
        }
    };
}

// Macro per implementare la ricerca e la revoca
macro_rules! impl_search_and_revoke {
    ($model:ty) => {
        impl Search<$model> for $model {
            fn search(query: &str) -> Vec<$model> {
                // Simulazione della logica di ricerca (filtraggio nel database) per ogni modello che implementa il trait con `match`statement
                vec![]
            }
        }

        impl Revoke for $model {
            fn revoke(id: u64) -> Result<(), String> {
                // Simulazione della logica di revoca (ad esempio, revoca token o permessi) per ogni modello che implementa il trait con `match`statement
                Ok(())
            }
        }
    };
}

// Applicazione delle macro ai modelli

impl_crud_ops!(User);
impl_crud_ops!(Article);
impl_crud_ops!(Comment);

// Aggiungi qui ulteriori modelli per l'applicazione CRUD generica

// Esempio di utilizzo per i modelli con Search e Revoke
impl_search_and_revoke!(User);
impl_search_and_revoke!(Article);

// Aggiungi ulteriori modelli se necessario

```

### USO

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