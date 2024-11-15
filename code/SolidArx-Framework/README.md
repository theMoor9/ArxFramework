# ArxFramework

## Introduction
ArxFramework è un framework modulare e flessibile progettato per supportare lo sviluppo di diverse tipologie di applicazioni, tra cui applicazioni web, API backend, applicazioni desktop, sistemi di automazione e scripting, e sistemi embedded.

Caratteristiche principali:

1. Architettura Modulare: Composto da moduli specializzati come core system, autenticazione, CRUD, gestione file, monitoraggio e altri, ciascuno ottimizzato per il proprio dominio.
2. Multilingua: Principalmente implementato in Rust per prestazioni e sicurezza, con integrazioni Python per flessibilità e machine learning.
3. Versatilità: Supporta cinque tipi principali di applicazioni, adattandosi alle esigenze specifiche di ciascuna.
4. Core System Robusto: Un sistema centrale in Rust che gestisce operazioni fondamentali come la gestione della memoria e la concorrenza.
5. CLI Configurabile: Un'interfaccia a linea di comando per una facile configurazione e inizializzazione del sistema.
6. Integrazione Frontend: Supporto per frontend dinamici utilizzando Svelte.
7. Capacità Avanzate: Include moduli per machine learning, integrazione blockchain e automazione dei task.
8. Logging e Monitoraggio: Sistema integrato per il logging e il monitoraggio delle prestazioni.
9. Gestione File e Risorse: Modulo dedicato per operazioni efficienti su file e risorse.
10. Documentazione e Template: Include documentazione completa e template per diversi tipi di applicazioni.

ArxFramework mira a fornire una base solida e flessibile per lo sviluppo di applicazioni, combinando l'efficienza di Rust con la versatilità di Python e altri strumenti moderni. È progettato per essere facilmente estensibile e adattabile a diverse esigenze di progetto, dalla prototipazione rapida allo sviluppo di sistemi complessi e ad alte prestazioni.

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
| Integrazione Blockchain    | ○       | ○           | ○           | ○                       | ○                |
| Machine Learning           | ○       | ○           | ○           | ○                       | ○                |

Legenda:

- ✓ : Modulo tipicamente utilizzato in questa soluzione
- ○ : Modulo opzionale o utilizzato in base a requisiti specifici

### CLI setup
Il cli sarà responsabile  della configurazione del tipo di applicazione tramite la variabile di config nel main. Da cui dipenderanno i seguenti moduli:

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

## Linguaggi di Supporto
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
├── src/ 
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
- `crud_operations.rs`: Operazioni CRUD generiche
- `models.rs`: Definizioni dei modelli dati

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
- `global_config.rs`File di configurazione per l'intero framework

**src/**
- `main.rs` Il main è costruito in gran parte dagli elementi  delle standard procedures riportate nella sezione [Scope](#Scope)
- `cli.rs` [Link al tameplate del CLI](./main-integration-procedures/cli.md)
- `lib.rs` Qui sono riportate tutte le cartelle( *crates* ) contenenti moduli
	
---

# Scope

>Ogni modulo ha il compito di processare task in base ad un input restituendo un output di status di avvenuta operazione al fine di avere uno script collettore delle operazioni.

## 1. Core Sistema
>Pertinente per: Tutte le applicazioni

**CAMBIARE**
Questo file contiene le strutture e le funzioni principali per l'inizializzazione e il controllo del core del sistema. Definisce i tipi di applicazione supportati, gestisce la configurazione globale, implementa il logging di base e fornisce un'interfaccia per le operazioni di sistema che si adattano al tipo di applicazione in uso. È il punto di entrata principale per l'interazione con il core del framework.

[Development](ArxFramework/module-development-templates/core-system.md)

[Customization]()

## 2. Autenticazione e Sicurezza 2 Versioni
>Pertinente per: Web App, API Backend, App Desktop


## 3. Gestione CRUD 2 Versioni
>Pertinente per: Web App, API Backend, App Desktop


## 4. API Layer 3 Versioni

>Pertinente per: Web App, API Backend, App Desktop, Sistemi Embedded


## 5. Gestione File/Risorse 3 Versioni

>Pertinente per: Tutte le applicazioni

## 6. Monitoraggio e Logging 2 Versioni

>Pertinente per: Tutte le applicazioni


## 7. Task Automation 2 Versioni

>Pertinente per: Automazione e Scripting, Sistemi Embedded


## 8. Integrazione Blockchain 2 Versioni

>Pertinente per: Web App, API Backend, App Desktop


## 9. Frontend Dinamico 2 Versioni

>Pertinente per: Web App, App Desktop


## 10. Machine Learning 3 Versioni

>Pertinente per: Web App, API Backend, Automazione e Scripting, Sistemi Embedded


---

# HOW to USE

1. Tramite il cli si configura la stuttura base per una delle applicazioni disponibili
2. 

---
	
# Author
Kenneth Boldrini

# License
This repository is licensed. See the [LICENSE](./LICENSE) file for more details.
