# crud/

| Modulo        | Linguaggio Principale | Linguaggio di Supporto | Wrapping | Framework/Librerie Principali | Considerazioni per lo Sviluppo<br>                     |
| ------------- | --------------------- | ---------------------- | -------- | ----------------------------- | ------------------------------------------------------ |
| Gestione CRUD | Rust                  | -                      | -        | diesel (ORM)                  | Design pattern Repository, astrazione del database<br> |

#Modulo-Layer-1-per-Code-Base 

Il modulo **CRUD** si trova all'interno della cartella `modules/`, che contiene due sotto-cartelle principali:

- **`modules/default/`**: Questa cartella contiene i modelli standard predefiniti, necessari al funzionamento di base dell'applicazione. Ogni file all'interno di questa cartella segue la convenzione di essere nomenclato con il suffisso `_model.rs` e definisce i modelli base, come `user_model.rs`, `article_model.rs`, e simili.
    
- **`modules/dev/`**: Questa cartella è dedicata ai modelli che lo sviluppatore può aggiungere in base alle necessità specifiche del progetto. Questi modelli, anch'essi con suffisso `_model.rs`, possono estendere o personalizzare il comportamento dei modelli predefiniti, o definirne di completamente nuovi.

---

# `models/dev/userexample_model.rs`

### Struttura dei Modelli

Ogni file `.rs` nella cartella `modules/` ha il compito di:

1. **Definire la struttura dei modelli**: Questi modelli descrivono le entità con cui l'applicazione interagirà. Le strutture definiscono i campi associati a ciascun modello (ad esempio, un utente potrebbe avere un `id`, un `username`, e un `email`).
    
2. **Operazioni disponibili**: I modelli includono un enum `CrudOperations` che definisce quali operazioni CRUD (Create, Read, Update, Delete) sono disponibili per ogni specifico modello. Le operazioni possono variare a seconda del contesto (ad esempio, un modello potrebbe avere l'operazione `Delete` solo per le API).
    
3. **Tipo di Memoria**: Ogni modello specifica il tipo di memoria in cui deve essere inserito, che può essere:
    
    - **Memoria allocata** (`InMemory`): I dati del modello vengono gestiti temporaneamente in memoria, utile per task volatili o temporanei.
    - **Database**: I dati vengono inseriti e gestiti persistentemente nel database, utile per tutte le entità che necessitano di essere salvate a lungo termine.

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

### USO

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

Il file **`crud_ops.rs`** contiene la logica che implementa le operazioni CRUD per i modelli definiti in **`modules/`**. Il suo scopo principale è gestire l'inserimento, l'aggiornamento e la cancellazione dei dati nei due tipi di memoria:

- **Memoria allocata (InMemory)**: Le operazioni CRUD interagiscono direttamente con i dati tenuti in memoria durante l'esecuzione dell'applicazione. Questo approccio è utilizzato principalmente per task temporanei o volatili.
    
- **Database**: Le operazioni CRUD interagiscono con un database persistente, garantendo che i dati rimangano salvati anche dopo che l'applicazione è stata chiusa o riavviata.
    

Il modulo **`crud_ops.rs`** si occupa di gestire in maniera centralizzata la logica necessaria per inserire, aggiornare e cancellare i dati in entrambi i contesti, garantendo che il comportamento sia coerente a seconda del tipo di memoria selezionata per ciascun modello.

Inoltre, **`crud_ops.rs`** fa da tramite tra l'interfaccia utente (o l'interfaccia API) e la gestione effettiva dei dati, traducendo gli input dell'utente in azioni concrete sui dati.

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
	
4. **Aggiorna le eventuali implementazioni di conseguenza**: Dopo aver aggiunto il nuovo modello, assicurati di aggiornare le implementazioni esistenti che potrebbero dipendere dal nuovo modello o necessitare di adattamenti per funzionare correttamente con le nuove funzionalità.

	Ad esempio, potresti dover modificare le funzioni CRUD nel sistema per gestire il nuovo modello o aggiornare i riferimenti nei file di configurazione.
	
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
			    api_specific_field, // Campo opzionale per API
			}
		}
	}
	```

### In `crud_ops.rs`
1. L'implementazione è differente per il tipo di allocazione scelta per il modello
	- Per allocazione **InMemory** occorre personalizzare il tipo di allocazione secondo i criteri di allocazione di `memory_management.rs` in `impl_crud_ops{...}` come segue:
	
		 - `allocate()`: Implementare nel `match` di `create()` analogamente.
		```Rust
// Task temporanei, quindi la memoria standard va bene per velocità e semplicità
"modules::default::task_model::Task" => {
	info!("Allocazione in memoria Standard per Task");
	let size = 256; // 256 byte per task temporanei
	memory_manager::allocate(Some(AllocationStrategy::Standard), size)?;
	Ok(item)
}
		```
			
		- `deallocate()`
		```Rust
		
		```
  
	- Per allocazione **Database** non sono necessarie implementazioni in `impl_crud_ops{...}`

3. **Importa il modello nel sistema CRUD**: Nel file `crud_ops.rs`, usa la macro per implementare le operazioni CRUD per il nuovo modello:

    ```Rust
	impl_crud_ops!(Post);
	```
    
**Compila con la feature desiderata**: Ricordati, al momento della compilazione il progetto di abilitare automaticamente le feature necessarie con :
	
    `cargo build --features "webapp"`
