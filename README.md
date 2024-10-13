# ArxFramework

## Introduzione

**ArxFramework** è una libreria modulare e flessibile che prende forma come applicazione completa una volta configurata. È progettato per supportare lo sviluppo di diverse tipologie di applicazioni, tra cui applicazioni web, API backend, applicazioni desktop, sistemi di automazione e scripting, e sistemi embedded.

### Caratteristiche principali:

1. **Architettura Modulare**: Composto da moduli specializzati come il core system, autenticazione, CRUD, gestione file, monitoraggio e altri, ciascuno ottimizzato per il proprio dominio e facilmente configurabile.
2. **Multilingua**: Principalmente implementato in **Rust** per prestazioni e sicurezza, con integrazioni in **Python** per flessibilità, soprattutto nelle operazioni di machine learning.
3. **Versatilità**: ArxFramework si adatta alle esigenze di cinque principali tipologie di applicazioni, consentendo personalizzazioni specifiche tramite configurazione.
4. **Core System Robusto**: Un sistema centrale in **Rust** che gestisce operazioni fondamentali come la gestione della memoria, la concorrenza, e altre operazioni di base che possono essere estese.
5. **Integrazione Frontend**: Supporto per frontend dinamici utilizzando **Svelte**, permettendo di creare interfacce moderne con facilità.
6. **Capacità Avanzate**: Include moduli per **machine learning**, integrazione **blockchain**, e automazione dei task.
7. **Logging e Monitoraggio**: Sistema integrato per il **logging** e il monitoraggio delle prestazioni, fondamentale per tracciare il comportamento dell'applicazione.
8. **Gestione File e Risorse**: Un modulo dedicato alla gestione efficiente di file e risorse, configurabile in base all'applicazione in uso.
9. **Documentazione e Template**: Fornisce documentazione completa e **template** che agevolano la configurazione dei moduli e l'integrazione di modelli personalizzati.

### Ambito (SCOPE)

ArxFramework offre una base solida e flessibile per lo sviluppo di applicazioni modulari. Combinando l'efficienza di **Rust** con la versatilità di **Python** e altri strumenti moderni, il framework è progettato per essere facilmente estensibile e adattabile. Grazie alla sua natura configurabile tramite **CLI** e alla presenza di **template**, permette di creare applicazioni su misura, dalla prototipazione rapida alla costruzione di sistemi complessi e ad alte prestazioni, massimizzando il potenziale grazie all'integrazione personalizzata dei moduli.

---

**Table of contents**

- [Type of Applications](#Type-of-Applications)
- [Functionality and Unique Technologies](#Functionality-and-Unique-Technologies)
- [Folders](#Folders)
- [Scope](#Scope)
- [HOW to USE](#HOW-to-USE)

---

# Type of Applications

1. **Applicazioni Web**: App che girano su browser, possono essere single-page applications (SPA) o app multi-pagina.
2. **API Backend**: Servizi che forniscono dati e funzioni ad altre applicazioni (mobile, web, desktop) tramite un'API REST o GraphQL.
3. **App Desktop**: Applicazioni che girano direttamente su sistemi operativi come Windows, macOS o Linux, con funzionalità offline e gestione di risorse locali.
4. **Automazione e Scripting**: Tool o script che automatizzano processi specifici, come estrazione dati, analisi o task ripetitivi.
5. **Sistemi Embedded o Performance-Critical**: App che richiedono prestazioni elevate, come sistemi IoT, microservizi o app per hardware specifici.

---

# Functionality and Unique Technologies

## Tabella della Struttura Modulare per 5 Soluzioni

| Modulo                     | Web App | API Backend | App Desktop | Automazione e Scripting | Sistemi Embedded |
| -------------------------- | ------- | ----------- | ----------- | ----------------------- | ---------------- |
| Core Sistema (Rust)        | ✓       | ✓           | ✓           | ✓                       | ✓                |
| Autenticazione e Sicurezza | ✓       | ✓           | ✓           | ○                       | ○                |
| Gestione CRUD              | ✓       | ✓           | ✓           | ○                       | ○                |
| API Layer                  | ✓       | ✓           | ○           | ○                       | ○                |
| Gestione file/risorse      | ○       | ○           | ✓           | ✓                       | ○                |
| Monitoraggio e logging     | ✓       | ✓           | ✓           | ✓                       | ✓                |
| Task Automation            | ○       | ○           | ○           | ✓                       | ○                |
| Frontend Dinamico          | ✓       | ○           | ✓           | ○                       | ○                |
| Integrazione Blockchain    | ✓       | ✓           | ✓           | ✓                       | ✓                |
| Machine Learning           | ✓       | ✓           | ✓           | ✓                       | ✓                |

Legenda:

- ✓ : Modulo tipicamente utilizzato in questa soluzione
- ○ : Modulo opzionale o utilizzato in base a requisiti specifici


### Dettagli Espansi dei Moduli per lo Sviluppo

| Modulo                     | Linguaggio Principale | Linguaggio di Supporto | Wrapping | Framework/Librerie Principali |
| -------------------------- | --------------------- | ---------------------- | -------- | ----------------------------- |
| Core Sistema               | Rust                  | -                      | -        | tokio (async runtime)         |
| Autenticazione e Sicurezza | Rust                  | Python                 | Sì       | jsonwebtoken, bcrypt          |
| Gestione CRUD              | Rust                  | -                      | -        | diesel (ORM)                  |
| API Layer                  | Rust (warp/actix-web) | Python (FastAPI)       | No       | serde (serializzazione)       |
| Gestione file/risorse      | Rust                  | -                      | -        | std::fs, tokio::fs            |
| Monitoraggio e logging     | Rust                  | -                      | -        | tracing, prometheus           |
| Task Automation            | Rust                  | Python                 | Sì       | rayon (parallelismo)          |
| Frontend Dinamico          | Svelte (TypeScript)   | -                      | -        | SvelteKit, DaisyUI(CSS)       |
| Integrazione Blockchain    | Rust                  | Python                 | Sì       | Solana web3                   |
| Machine Learning           | Python                | Rust                   | Sì       | PyTorch, scikit-learn         |


Note Aggiuntive:

- Per ogni modulo, considerare l'implementazione di test unitari e di integrazione.
- Documentare le API interne ed esterne di ciascun modulo.
- Implementare un sistema di configurazione flessibile per ciascun modulo.
- Considerare l'uso di container Docker per la standardizzazione dell'ambiente di sviluppo e deployment.

## Linguaggi di Supporto (Opzionale)
### Distinzione tra Approcci Wrapping e Non-Wrapping

1. **Approccio Wrapping** (es. Autenticazione e Sicurezza): Il codice Rust viene incapsulato in funzioni Python per un uso più agevole. Esempio Rust (con PyO3):
    
    ```Rust
	#[pyfunction] 
	fn verify_password(password: &str, hash: &str) -> PyResult<bool> {     
		// Implementazione Rust    
		Ok(true) 
		// placeholder 
	}
	```
    
    Esempio Python:
    
    ```Python
	from rust_auth import verify_password 
	
	def user_login(username, password):     
		stored_hash = get_stored_hash(username)    
		return verify_password(password, stored_hash)
	```
	    
1. **Approccio Unwrapped** (es. Task Automation): Rust e Python cooperano, con Python che orchestra e chiama funzioni Rust quando necessario. Esempio Rust (con PyO3):
    
    ```Rust
	#[pyfunction] 
	fn process_data(data: Vec<f64>) -> PyResult<Vec<f64>> {
	     // Elaborazione intensiva in Rust    
	     Ok(data.iter().map(|&x| x * 2.0).collect()) 
	}
	```
    
    Esempio Python:
    
    ```Rust
	import rust_processor 
	
	def run_pipeline():     
		data = load_data()    
		processed = rust_processor.process_data(data)    
		save_results(processed)
	```
    

- Nel primo approccio, la funzionalità Rust è "avvolta" in un'interfaccia Python.
- Nel secondo, Python utilizza direttamente le funzioni Rust come parte di un flusso di lavoro più ampio.

>Occorre applicare le metodologie in relazione alla tabella antecedente.

---

# Folders

```sh
ArxFramework/
├── core/                 # Core Sistema (Rust)
├── auth/                 # Autenticazione e Sicurezza (Rust, Python opzionale)
├── crud/                 # Gestione CRUD (Rust)
| └── models/               # Cartella dei modelli
├── api/                  # API Layer (Rust primario, Python FastAPI opzionale)
├── file_management/      # Gestione file/risorse (Rust)
├── monitoring/           # Monitoraggio e logging (Rust)
| └── logs/               # Cartella dei log
├── task_automation/      # Task Automation (Rust primario, Python secondario)
├── blockchain/           # Integrazione Blockchain (Rust)
├── frontend/             # Ospita il front end
├── ml/                   # Machine Learning (Python primario, Rust secondario)
├── docs/                 # Documentazione del framework
├── config/               # Configurazioni globali
├── src/                  # Source
├── tests/                # Test unitari 
└── README.md             # Documentazione principale del framework
```

**core/**
- `system_core.rs`: Gestione centrale del sistema
- `memory_management.rs`: Ottimizzazione della memoria

**auth/**
- `auth_core.rs`: Funzionalità core di autenticazione
- `auth_wrapper.py`: Wrapper Python per integrazioni

**crud/**
- **models/**: Cartella contenente i modelli .rs costruiti secondo procedura di default per l'integrazione 
	- `user_model.rs` : modello contente la `struct` che lo definisce e l `enum` che indica le proprietà ereditabili da `crud_ops.rs`.
	- `article_model.rs`: come detto per `user_model.rs`
	- altri modelli...
- `crud_ops.rs`: Operazioni CRUD generiche

**api/**
- `api_server.rs`: Server API principale
- `routes.rs`: Definizione dei percorsi API
- `fastapi_integration.py`: Integrazione con FastAPI

**file_management/**
- `file_ops.rs`: Operazioni su file
- `resource_manager.rs`: Gestione risorse

**monitoring/**
- **logs/**: Contiene tutti i file di log dei rispettivi moduli
- `logger.rs`: Sistema di logging
- `metrics.rs`: Raccolta e reporting metriche

**task_automation/**
- `task_core.rs`: Funzioni core per automazione
- `automation_scripts.py`: Script di automazione Python

**blockchain/**
- `blockchain_integration.rs`: Interfaccia blockchain
- `smart_contracts.rs`: Gestione smart contract

**frontend/**
Ospita il frontend
- `App.svelte`: Componente root Svelte
- `index.js`: Entry point dell'applicazione
- `components/`: Cartella componenti riutilizzabili

**ml/**
- `ml_models.py`: Implementazione modelli ML
- `data_processing.rs`: Elaborazione dati performante

**docs/**
- File di documentazione per ogni modulo

**tests/**
- Contiene i test unitari.

**config/**
- `global_config.rs`: File di configurazione per l'intero framework

**src/**
- `main.rs`: Punto d'ingresso del framework e incapsulatore del CLI.
- `lib.rs`: Qui sono riportate tutte le cartelle( *crates* ) contenenti moduli
	
---

# Guidelines

[Linee Guida Generali Moduli](development_guide_lines.md)
[Proprietà dei modelli](module_proprieties.md)

---

# Come Utilizzare ArxFramework

ArxFramework è progettato per aiutarti a creare progetti con template predefiniti per diversi tipi di applicazioni: Web App, API Backend, App Desktop, Automazione e Scripting, e Sistemi Embedded. Segui i passaggi sottostanti per installare e usare il framework in modo efficace.

#### **Passo 1: Installa lo Strumento CLI**

Per iniziare a usare ArxFramework, è necessario installare lo strumento CLI globalmente sul tuo sistema utilizzando Cargo.

```sh
cargo install arx_framework
```

Questo comando:

- Scaricherà e costruirà lo strumento CLI di ArxFramework (chiamato `arx`).
- Installerà il binario `arx` nella directory bin di Cargo (`~/.cargo/bin` su Linux/macOS o `%USERPROFILE%\.cargo\bin` su Windows).

#### **Passo 2: Verifica l'Installazione**

Dopo l'installazione, assicurati che il comando `arx` sia disponibile nel tuo terminale. Potresti dover aggiungere la directory bin di Cargo alla variabile PATH del tuo sistema se non lo è già.

Per verificare se l'installazione è avvenuta correttamente:

```sh
arx --version
```

Questo comando stamperà la versione dello strumento CLI `arx` installato.

#### **Passo 3: Inizializza un Nuovo Progetto**

Ora puoi creare un nuovo progetto eseguendo il comando `arx init`. Questo comando imposterà l'intera struttura del framework per il tipo di applicazione che scegli.

1. **Crea una nuova directory per il tuo progetto**:
    
    ```sh
	mkdir mio_nuovo_progetto 
	cd mio_nuovo_progetto
	```
    
2. **Esegui il comando `arx init`** con il tipo di applicazione (`--AppType`), dove `--AppType` può essere `webapp`, `api_backend`, `desktop_app`, `automation`, o `embedded`:
    
    ```sh
	arx init --AppType webapp
	```
    
    Questo comando genererà la struttura del progetto su misura per il tipo di applicazione specificato.
    

#### **Passo 4: Struttura del Progetto**

Una volta inizializzato il progetto, avrai una struttura di directory preconfigurata basata sul tipo di applicazione scelto. Ad esempio, per una web app, la struttura potrebbe essere simile a questa:

```sh
ArxFramework/
├── core/                 # Core Sistema (Rust)
├── auth/                 # Autenticazione e Sicurezza (Rust, Python opzionale)
├── crud/                 # Gestione CRUD (Rust)
| └── models/               # Cartella dei modelli
├── api/                  # API Layer (Rust primario, Python FastAPI opzionale)
├── file_management/      # Gestione file/risorse (Rust)
├── monitoring/           # Monitoraggio e logging (Rust)
| └── logs/               # Cartella dei log
├── task_automation/      # Task Automation (Rust primario, Python secondario)
├── blockchain/           # Integrazione Blockchain (Rust)
├── frontend/             # Ospita il front end
├── ml/                   # Machine Learning (Python primario, Rust secondario)
├── docs/                 # Documentazione del framework
├── config/               # Configurazioni globali
├── src/                  # Source
├── tests/                # Test unitari 
└── README.md             # Documentazione principale del framework
```

Ogni cartella contiene moduli specifici per aiutarti a iniziare lo sviluppo della tua applicazione. 

#### **Passo 5: Configura e Personalizza**

1. **Aggiorna il `Cargo.toml`**: A seconda del progetto, potresti voler personalizzare le dipendenze o le feature all'interno del file `Cargo.toml`.
    
2. **Modifica o Aggiungi Funzionalità**:
    
    - Puoi modificare o estendere i modelli e i componenti situati nella directory `modules/` per adattarli alle specifiche esigenze della tua applicazione.
    - Per qualsiasi personalizzazione, segui la struttura documentata all'interno di ciascun modulo.
    
3. **Argomenti della Linea di Comando**: Lo strumento CLI ti consente di passare vari argomenti per personalizzare l'inizializzazione del progetto. Consulta la documentazione del CLI (`arx --help`) per esplorare le opzioni disponibili.
    

#### **Passo 6: Compila ed Esegui**

Una volta configurato e personalizzato il tuo progetto, puoi compilarlo ed eseguirlo utilizzando Cargo:

```sh
cargo build cargo run
```

Questo comando compilerà il progetto ed eseguirà l'applicazione in base alla configurazione specificata durante l'inizializzazione.

#### **Passo 7: Aggiungi Altre Funzionalità o Modifica i Componenti**

- Puoi personalizzare ulteriormente il framework aggiungendo nuovi modelli o estendendo quelli esistenti nelle cartelle `crud/` o `core/`.
- Utilizza le macro fornite all'interno del file `crud_ops.rs` per implementare funzionalità CRUD aggiuntive per i nuovi modelli.

#### **Passo 8: Sviluppo Continuo**

- Usa un sistema di controllo di versione come Git per gestire le modifiche al progetto.
- Durante lo sviluppo, continua a personalizzare i moduli generati in base alle esigenze della tua applicazione.

---

### Esempio di Workflow

1. **Installa il CLI**:
        
    ```sh
	cargo install arx_framework
	```
    
2. **Crea una nuova directory e inizializza una web app**:
    
    ```sh
	mkdir mia_webapp
	cd mia_webapp 
	arx init --AppType webapp
	```
    
3. **Compila ed esegui la tua nuova web app**:
    
    ```sh
	cargo build cargo run
	```
    

Questo farà partire la tua applicazione web con la struttura preconfigurata di ArxFramework.

---

Questa procedura fornisce un'esperienza semplificata per gli sviluppatori che usano ArxFramework per creare rapidamente progetti, sfruttando la potenza di Rust e una configurazione modulare e flessibile.

---
	
**Autore:** Kenneth Boldrini