# Ingegneria di sviluppo

- **Definizione di Milestone Modulare**: Invece di lavorare su un modulo alla volta senza una visione chiara dell’insieme, definisci una roadmap con milestone modulari. Ogni milestone corrisponde a un blocco fondamentale del framework (es. gestione memoria, logging, CRUD). Per ogni milestone, identifica:
    
    - **Obiettivi minimi**: Cosa deve funzionare al 100% in quella fase (ad es. gestione base della memoria).
    - **Moduli collegati**: Quali moduli si collegano a quello che stai sviluppando in quella fase, e come (ad esempio il CRUD potrebbe connettersi alla gestione della memoria).
    - **Compatibilità**: Documenta come i moduli sviluppati in quella fase devono integrarsi con moduli futuri, anche se questi non sono ancora sviluppati.
    
- **Integrazione di Tool di Versionamento e Monitoraggio**: Git è essenziale, ma considera l’aggiunta di un tool di monitoraggio o gestione progetti (come **Trello** o **Notion**) che ti permetta di creare board con task suddivisi in moduli e sotto-task. Questi strumenti ti aiutano a tracciare progressi, modifiche e compatibilità tra i moduli, e a visualizzare come ogni parte si collega all’insieme.
    
- **Iterazioni Agili**: Procedi per cicli di sviluppo rapidi. Completa il core framework con versioni semplificate, ma funzionanti, dei moduli principali. Ogni ciclo dovrà aggiungere o perfezionare qualcosa, mentre mantieni sempre una base stabile. Usa test per verificare la compatibilità tra i moduli sviluppati man mano.
    
- **Documentazione continua**: Non aspettare la fine del progetto per documentare. Ogni modulo e milestone dovrebbe avere una documentazione schematica e precisa. Ciò ti permetterà di tenere traccia delle interazioni tra i moduli, facilitando le future integrazioni avanzate.
    
- **Rimodellamento del Piano di Lavoro**: Invece di procedere in modo procedurale (che può essere dispersivo), utilizza una strategia più iterativa e basata su priorità:
    
    - **Core System** (Memory Management, CRUD, Logging) come prima milestone, assicurandoti che ogni blocco sia ben testato e integrato.
    - **Integrazione di moduli avanzati** in maniera progressiva (ad es. Machine Learning e Blockchain) in fasi successive, con un occhio a mantenere la compatibilità e documentare le connessioni tra i moduli.
    - **Modulo CLI**: Lascia spazio per eventuali modifiche che dovrai fare in base alle integrazioni future.


# Elementi

### core/

- [ ]  `system_core.rs`: Gestione centrale del sistema implementata.
    - [x]  code
    - [x]  dev documentation
    - [ ]  documentation
    - [x]  Logger
    - [x]  In-code Documentation
    - [ ]  Unit test
    - [ ]  Review
- [ ]  `memory_management.rs`: Ottimizzazione della memoria gestita correttamente.
    - [x]  code
    - [x]  dev documentation
    - [ ]  documentation
    - [x]  Logger
    - [x]  In-code Documentation
    - [ ]  Unit test
    - [ ]  Review

### monitoring/

-  `logger.rs`: Sistema di logging configurato e funzionante.
    
    - [x]  code
    - [x]  dev documentation
    - [ ]  documentation
    - [x]  In-code Documentation
    - [ ]  Unit test
    - [ ]  Review
-  `metrics.rs`: Raccolta e reporting delle metriche attivi.
    
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test

### auth/

- [ ]  `auth_core.rs`: Funzionalità di autenticazione core definite.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test
- [ ]  `auth_wrapper.py`: Wrapper Python per integrazioni funzionante.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test

### crud/

- [ ]  `crud_operations.rs`: Operazioni CRUD implementate per tutti i casi d'uso.
    - [ ]  code
    - [ ]  implementare `connection` con api_server.rs con sqlx
    - [x]  Recupero dati da variabile di config inizializzata nel `main.rs` per la variabile globale atta allo scalo della memoria `STATIC_MEMORY_SCALE`.
    - [x]  dev documentation
    - [ ]  documentation
    - [x]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test
- [x]  `models.rs`: Modelli dati definiti e mappati correttamente.
    - [x]  code
    - [x]  dev documentation
    - [ ]  documentation
    - [x]  Logger
    - [x]  In-code Documentation
    - [ ]  Unit test

### api/

- [ ]  `api_server.rs`: Server API principale avviabile e funzionante.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test
- [ ]  `routes.rs`: Percorsi API definiti e mappati correttamente.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test
- [ ]  `fastapi_integration.py`: Integrazione con FastAPI operativa.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test

### network/
- [ ] `connection_manager.rs`: gestisce le connessioni attive e il pool di connessioni.
	- [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test
- [ ] `load_balancer.rs` : implementa vari algoritmi di load balancing.
	- [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test
- [ ] `resource_pool.rs`: gestisce le risorse di rete, come connessioni, in modo centralizzato.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test

### file_management/

- [ ]  `file_ops.rs`: Operazioni su file implementate e testate.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test
- [ ]  `resource_manager.rs`: Gestione risorse ottimizzata.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test

### frontend/

- [ ]  `App.svelte`: Componente root Svelte sviluppato e funzionante.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test
    - [ ] 
- [ ]  `main.js`: Entry point dell'applicazione frontend configurato correttamente.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test
- [ ]  `components/`: Cartella componenti contiene elementi riutilizzabili.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test

### task_automation/

- [ ]  `task_core.rs`: Funzioni core per automazione sviluppate.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test
- [ ]  `automation_scripts.py`: Script Python di automazione funzionanti.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test

### blockchain/

- [ ]  `blockchain_integration.rs`: Interfaccia blockchain funzionante.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test
- [ ]  `smart_contracts.rs`: Gestione degli smart contract implementata.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test

### ml/

- [ ]  `ml_models.py`: Modelli ML implementati e funzionanti.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test
- [ ]  `data_processing.rs`: Elaborazione dati performante sviluppata.
    - [ ]  code
    - [ ]  dev documentation
    - [ ]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test

### config/

- [ ]  `global_config.rs`: File di configurazione globale completo e configurato.
    - [x]  code
    - [x]  dev documentation
    - [ ]  documentation
    - [x]  Logger
    - [x]  In-code Documentation
    - [x]  Unit test
- [ ] `network_config.rs`: file di configurazione per settare parametri come il numero massimo di connessioni, timeout delle connessioni e algoritmo di bilanciamento.
    - [x]  code
    - [x]  dev documentation
    - [ ]  documentation
    - [x]  Logger
    - [x]  In-code Documentation
    - [ ]  Unit test
- [ ] `memory_config.rs`
    - [x]  code
    - [x]  dev documentation
    - [ ]  documentation
    - [x]  Logger
    - [x]  In-code Documentation
    - [x]  Unit test

### src/

- [ ]  `main.rs`: Main costruito con standard procedures.
    - [ ]  code
    - [x]  dev documentation
    - [x]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test
- [ ]  `lib.rs`: Tutte le crates e moduli inclusi e collegati correttamente.
    - [x]  code
    - [ ]  dev documentation
    - [x]  documentation
    - [ ]  In-code Documentation
    - [ ]  Unit test
- [ ]  `cli.rs`: Command Line Interface per facilitare il setup.
    - [ ]  code
    - [x]  dev documentation
    - [x]  documentation
    - [ ]  Logger
    - [ ]  In-code Documentation
    - [ ]  Unit test

---
#  Note Steps

### **A. Implementazione dei Moduli Esterni**

- **Sviluppare i Moduli `auth`, `crud`, `api_layer`, ecc.**:
    - Implementare i moduli esterni secondo l'interfaccia definita dal trait `SystemComponent`.
    - Assicurarsi che ogni modulo gestisca correttamente l'inizializzazione, lo shutdown e la gestione degli errori.

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

#### Passo 3: Sviluppo del Modulo di Rete

- **Obiettivo**: Fornire funzionalità di connessione server, inclusi _connection pooling_, _load balancing_ e gestione delle connessioni persistenti, se necessario.
- **Dipendenze**: Questo modulo sarà la base per il sistema CRUD lato server. Dovrà quindi prevedere i tipi di connessione (es. WebSocket per connessioni persistenti o HTTP per richieste stateless).
- **Interfaccia di rete**: Assicurati che ci sia un’interfaccia semplice e unificata per altre componenti, come il CRUD, che possano invocare richieste di rete.

#### Passo 4: Modulo di Autenticazione
    
- **Obiettivo**: Assicurarsi che ogni richiesta CRUD e ogni operazione di rete sia autorizzata.
- **Dipendenze**: Il modulo di autenticazione dovrà integrarsi con il modulo di rete, gestendo token di accesso e identificatori di sessione, oltre a eventuali chiavi API.
- **Integrazione con CRUD**: Prepara una funzione di verifica di autenticazione che possa essere richiamata nel modulo CRUD prima di eseguire operazioni lato server.

#### Passo 5: Implementazione delle Operazioni CRUD

Una volta stabilita la connessione al database e propagata la configurazione, puoi completare **crud_ops.rs** e implementare le operazioni CRUD per i modelli. In questo passaggio, `crud_ops.rs` utilizzerà la connessione centralizzata per eseguire operazioni come `INSERT`, `UPDATE`, `DELETE`, ecc.

- **In Memory**: Continua con le operazioni in memoria (creazione, lettura, aggiornamento, eliminazione) per garantire che la logica CRUD sia chiara e funzionante.
- **Estensione per Server**: Integra il modulo CRUD con la rete, aggiungendo logica per decidere se l'operazione è locale (in memory) o remota (server).
- **Sicurezza**: Assicurati che tutte le operazioni lato server richiamino il modulo di autenticazione per la verifica delle autorizzazioni.

- ##### Implementazione crud/

	- [x] **CRUD Default Models**
	 **1. Web App**
		User, Article/Blog Post, Comment, Category, Tag, File/Image, Page
	 **2. API Backend**
		User, API Key, Token, Request Log, Endpoint, Permission, Rate Limit Rule
	 **3. Desktop App**
		User, Settings, Document, File, Preferences, Task, Project
	 **4. Automazione e Scripting**
		Script, Task, Execution Log, Schedule, Configuration, Job, Macro Script
	 **5. Sistemi Embedded**
		Device, Sensor Data, Configuration, Firmware Version, Log/Event, Command, Task
	
	- [ ]  **CRUD Operations**
		Per **ogni modello** avremo le seguenti possibili operazioni:
		
		- [ ] **Create**: Crea un nuovo elemento nel database.
		- [ ] **Read**: Legge un elemento specifico dal database.
		- [ ] **Update**: Aggiorna i dati di un elemento esistente.
		- [ ] **Delete**: Elimina un elemento specifico dal database.
		- [ ] **List**: Restituisce una lista di tutti gli elementi o una selezione.
		- [ ] **Search**: (Facoltativa) Ricerca avanzata di elementi nel database.
		- [ ] **Revoke**: (Facoltativa) Invalida o revoca un elemento (usata per chiavi API o token).
		
		######  Operazioni CRUD Applicabili ai Modelli
		
		- **Create**: User, Article, Comment, Category, Tag, File/Image, Page, API Key, Token, Request Log, Endpoint, Permission, Rate Limit Rule, Settings, Document, Preferences, Task, Project, Command, Sensor Data, Script, Schedule, Log Event, Macro Script, Firmware Version, Device, Configuration
		- **Read**: User, Article, Comment, Category, Tag, File/Image, Page, API Key, Token, Request Log, Endpoint, Permission, Rate Limit Rule, Settings, Document, Preferences, Task, Project, Command, Sensor Data, Script, Schedule, Log Event, Macro Script, Firmware Version, Device, Configuration
		- **Update**: User, Article, Comment, Category, Tag, File/Image, Page, Endpoint, Permission, Rate Limit Rule, Settings, Document, Preferences, Task, Project, Command, Script, Script, Schedule, Macro Script, Firmware Version, Device, Configuration
		- **Delete**: User, Article, Comment, Category, Tag, File/Image, Page, API Key, Token, Request Log, Endpoint, Permission, Rate Limit Rule, Settings, Document, Preferences, Task, Project, Sensor Data, Script, Script, Schedule, Log Event, Macro Script, Device, Configuration
		- **List**: User, Article, Comment, Category, Tag, File/Image, Page, API Key, Token, Request Log, Endpoint, Permission, Rate Limit Rule, Settings, Document, Preferences, Task, Project, Command, Sensor Data, Schedule, Macro Script, Device
		- **Search**: Article, Comment, File/Image, Page, Document, Command,Token, Sensor Data, Script, Schedule, Log Event, Firmware Version,Device, Configuration
		- **Revoke**: API Key, Token, Command, Schedule, Firmware Version, Device, Configuration
	
	- [ ] **Memory integration**
	
	- **Per database tradizionale (PostgreSQL)**:
	    - Una libreria Rust come **`Diesel`**.
	    - Un server PostgreSQL locale o remoto. (Dettaglio che andrà sviluppato dinamicamente visto che non riguarda la possibile app)
	- **Per Solana**:
	    - La libreria **`solana-client`** in Rust.
	    - Un nodo RPC o una connessione alla rete Solana
	    
#### Passo 6: Testing dell'Integrazione
    
- Una volta stabilita la rete, procedi con test combinati per CRUD e autenticazione su server. Questo ti permette di rilevare eventuali problematiche nelle dipendenze, come ritardi di autenticazione o problemi di timeout nelle connessioni.

### System Core Initialize module
- [ ] Sviluppare la funzione `initialize_module` per ogni modulo.

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

**Author**: Kenneth Boldrini
