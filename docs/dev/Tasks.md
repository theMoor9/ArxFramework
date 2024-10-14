# Steps

1. Completare tutti i moduli e documentazioni in maniera generale 
2. Revisionare i moduli in cerca di elementi da adattare al sistema
3. Correzione documentazioni

# Elementi

### **core/**

- [x]  `system_core.rs`: Gestione centrale del sistema implementata.
	- [ ] code
	- [x] dev documentation
	- [x] documentation
	- [x] Logger
	- [x] In-code Documentation
- [x]  `memory_management.rs`: Ottimizzazione della memoria gestita correttamente.
	- [ ] code
	- [x] dev documentation
	- [x] documentation
	- [x] Logger
	- [x] In-code Documentation


### **auth/**

- [ ]  `auth_core.rs`: Funzionalità di autenticazione core definite.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation
- [ ]  `auth_wrapper.py`: Wrapper Python per integrazioni funzionante.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation

### **crud/**

- [ ]  `crud_operations.rs`: Operazioni CRUD implementate per tutti i casi d'uso.
	- [ ] code
	- [ ] implementare `connection` con api_server.rs con sqlx
	- [ ] Recupero dati da variabile di config inizializzata nel `main.rs` per la variabile globale atta allo scalo della memoria `STATIC_MEMORY_SCALE`.
	- [x] dev documentation
	- [x] documentation
	- [x] Logger
	- [x] In-code Documentation
- [ ]  `models.rs`: Modelli dati definiti e mappati correttamente.
	- [ ] code
	- [x] dev documentation
	- [ ] documentation
	- [x] Logger
	- [x] In-code Documentation

### **api/**

- [ ]  `api_server.rs`: Server API principale avviabile e funzionante.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation
- [ ]  `routes.rs`: Percorsi API definiti e mappati correttamente.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation
- [ ]  `fastapi_integration.py`: Integrazione con FastAPI operativa.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentationon

### **file_management/**

- [ ]  `file_ops.rs`: Operazioni su file implementate e testate.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation
- [ ]  `resource_manager.rs`: Gestione risorse ottimizzata.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation

### **monitoring/**

- [ ]  `logger.rs`: Sistema di logging configurato e funzionante.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation
	
- [ ]  `metrics.rs`: Raccolta e reporting delle metriche attivi.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation

### **task_automation**

- [ ]  `task_core.rs`: Funzioni core per automazione sviluppate.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation
- [ ]  `automation_scripts.py`: Script Python di automazione funzionanti.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation

### **blockchain/**

- [ ]  `blockchain_integration.rs`: Interfaccia blockchain funzionante.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation
- [ ]  `smart_contracts.rs`: Gestione degli smart contract implementata.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation

### **frontend/**

- [ ]  `App.svelte`: Componente root Svelte sviluppato e funzionante.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation
- [ ]  `main.js`: Entry point dell'applicazione frontend configurato correttamente.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation
- [ ]  `components/`: Cartella componenti contiene elementi riutilizzabili.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation

### **ml/**

- [ ]  `ml_models.py`: Modelli ML implementati e funzionanti.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation
- [ ]  `data_processing.rs`: Elaborazione dati performante sviluppata.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation

### **architectures/**

- [ ]  Schemi per le diverse applicazioni presenti .

### **docs/**

- [ ]  File di documentazione aggiornati per ogni modulo.

### **config/**

- [ ]  `global_config.rs`: File di configurazione globale completo e configurato.
	- [ ] code
	- [x] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [x] In-code Documentation

### **src/**

- [ ]  `main.rs`: Main costruito con standard procedures.
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation
- [ ]  `lib.rs`: Tutte le crates e moduli inclusi e collegati correttamente.
	- [x] Code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] In-code Documentation
- [ ] `cli.rs`: Command Line Interface per facilitare il setup
	- [ ] code
	- [ ] dev documentation
	- [ ] documentation
	- [ ] Logger
	- [ ] In-code Documentation

---
#  Note Steps

### **A. Implementazione dei Moduli Esterni**

- **Sviluppare i Moduli `auth`, `crud`, `api_layer`, ecc.**:
    - Implementare i moduli esterni secondo l'interfaccia definita dal trait `SystemComponent`.
    - Assicurarsi che ogni modulo gestisca correttamente l'inizializzazione, lo shutdown e la gestione degli errori.

### crud/

####  CRUD Models
- [x]  
##### **1. Web App**
User, Article/Blog Post, Comment, Category, Tag, File/Image, Page
##### **2. API Backend**
User, API Key, Token, Request Log, Endpoint, Permission, Rate Limit Rule
##### **3. Desktop App**
User, Settings, Document, File, Preferences, Task, Project
##### **4. Automazione e Scripting**
Script, Task, Execution Log, Schedule, Configuration, Job, Macro
##### **5. Sistemi Embedded**
Device, Sensor Data, Configuration, Firmware Version, Log/Event, Command, Task

#### CRUD Operations
- [x] 
Per **ogni modello** avremo le seguenti operazioni:

- **Create**: Crea un nuovo elemento nel database.
- **Read**: Legge un elemento specifico dal database.
- **Update**: Aggiorna i dati di un elemento esistente.
- **Delete**: Elimina un elemento specifico dal database.
- **List**: Restituisce una lista di tutti gli elementi o una selezione.
- **Search**: (Facoltativa) Ricerca avanzata di elementi nel database.
- **Revoke**: (Facoltativa) Invalida o revoca un elemento (usata per chiavi API o token).

#####  Operazioni CRUD Applicabili ai Modelli

- **Create**: User, Article, Comment, Category, Tag, File/Image, Page, API Key, Token, Request Log, Endpoint, Permission, Rate Limit Rule, Settings, Document, Preferences, Task, Project
- **Read**: User, Article, Comment, Category, Tag, File/Image, Page, API Key, Token, Request Log, Endpoint, Permission, Rate Limit Rule, Settings, Document, Preferences, Task, Project
- **Update**: User, Article, Comment, Category, Tag, File/Image, Page, Endpoint, Permission, Rate Limit Rule, Settings, Document, Preferences, Task, Project
- **Delete**: User, Article, Comment, Category, Tag, File/Image, Page, API Key, Token, Request Log, Endpoint, Permission, Rate Limit Rule, Settings, Document, Preferences, Task, Project
- **List**: User, Article, Comment, Category, Tag, File/Image, Page, API Key, Token, Request Log, Endpoint, Permission, Rate Limit Rule, Settings, Document, Preferences, Task, Project
- **Search**: Article, Comment, File/Image, Page, Document
- **Revoke**: API Key, Token

##### Memory integration

- **Per database tradizionale (PostgreSQL)**:
    - Una libreria Rust come **`sqlx`**.
    - Un server PostgreSQL locale o remoto. (Dettaglio che andrà sviluppato dinamicamente visto che non riguarda la possibile app)
- **Per Solana**:
    - La libreria **`solana-client`** in Rust.
    - Un nodo RPC o una connessione alla rete Solana

### Integration
- [ ] 
#### Passo 1: Integrare il Main e il CLI e la Configurazione del Main
- [x] 
Devi prima finalizzare l'integrazione del **CLI** con il **main.rs** per poter estrarre correttamente le variabili di configurazione. La configurazione includerà:

- Tipo di applicazione (WebApp, API, ecc.).
- URL del database (o altre configurazioni specifiche per il server).
- Parametri di memoria (per memory_management.rs).

In questa fase, definisci come le opzioni passate tramite CLI vengono gestite e propagate nel framework.

#### Passo 2: Inizializzare il Sistema (Main + CLI + Configurazione Server)

Il prossimo passo sarà garantire che il **main.rs** inizializzi correttamente il sistema. Devi fare in modo che:

- La connessione al database sia stabilita correttamente.
- Le configurazioni del sistema siano propagate al resto del framework (es. tipo di applicazione, dimensioni del pool di memoria).
- Moduli come **memory_management.rs** ricevano la configurazione appropriata.

#### Passo 3: Creare il Modulo per la Connessione Server 

creare un modulo indipendente (es. **server/** o **db_connection/**) che gestisce tutte le connessioni server e di database. Questo modulo si occuperebbe di:

- **Inizializzare** la connessione al database.
- **Gestire** eventuali configurazioni specifiche (es. pool di connessioni, timeout).
- **Esportare** una variabile `connection` riutilizzabile da qualsiasi parte del framework, sia che si tratti di un'app API, un'app web, o operazioni CRUD.

#### Passo 4: Finalizzare `crud_ops.rs`

Una volta stabilita la connessione al database e propagata la configurazione, puoi completare **crud_ops.rs** e implementare le operazioni CRUD per i modelli. In questo passaggio, `crud_ops.rs` utilizzerà la connessione centralizzata per eseguire operazioni come `INSERT`, `UPDATE`, `DELETE`, ecc.




---

### **B. Test e Validazione**

- **Scrivere Test Unitari e di Integrazione**:
    
    - Verificare il corretto funzionamento dei moduli implementati.
    - Assicurarsi che le diverse strategie di allocazione della memoria funzionino come previsto.
- **Verificare il Comportamento in Ambienti Multi-Thread**:
    
    - Testare il `MemoryManager` e il logger in contesti multi-thread per assicurarsi che il locking funzioni correttamente.

### **C. Ottimizzazione e Pulizia del Codice**

- **Migliorare l'Efficienza del `MemoryManager`**:
    
    - Valutare se il pool di memoria può essere ottimizzato, ad esempio implementando dimensioni variabili dei buffer o meccanismi di riciclo più sofisticati.
- **Refactoring e Miglioramento della Manutenibilità**:
    
    - Rivedere il codice per individuare possibili miglioramenti o semplificazioni.
    - Assicurarsi che il codice sia ben documentato e conforme alle best practice di Rust.

### **D. Documentazione e Guide**

- **Refattorizzare la Documentazione**:
    - Refattorizzare esempi d'uso, guide e spiegazioni dettagliate per gli sviluppatori che utilizzeranno il framework su obsidian.
- **Generare Documentazione Automatica**:
    - Utilizzare `rustdoc` per generare la documentazione a partire dai commenti `///` presenti nel codice.

### **E. Gestione delle Configurazioni**

- **Estendere `global_config.rs`**:
    - Aggiungere configurazioni specifiche per i moduli esterni, come `ApiConfig`, `AuthConfig`, ecc.
    - Implementare meccanismi per caricare configurazioni da file o variabili d'ambiente.

### **F. Considerazioni sulla Sicurezza**

- **Validazione degli Input e Gestione delle Eccezioni**:
    - Assicurarsi che il sistema gestisca correttamente input non validi o malformati.
    - Implementare controlli di sicurezza nei moduli critici, come l'autenticazione e l'accesso ai dati.

---
