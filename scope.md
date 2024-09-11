# Scope

## Type of Applications:

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


## Dettagli Espansi dei Moduli per lo Sviluppo

| Modulo                     | Linguaggio Principale | Linguaggio di Supporto | Wrapping | Framework/Librerie Principali | Considerazioni per lo Sviluppo                              |
| -------------------------- | --------------------- | ---------------------- | -------- | ----------------------------- | ----------------------------------------------------------- |
| Core Sistema               | Rust                  | -                      | -        | tokio (async runtime)         | Ottimizzazione per concorrenza e gestione risorse           |
| Autenticazione e Sicurezza | Rust                  | Python                 | Sì       | jsonwebtoken, bcrypt          | Implementare OAuth2, gestione sessioni                      |
| Gestione CRUD              | Rust                  | -                      | -        | diesel (ORM)                  | Design pattern Repository, astrazione del database          |
| API Layer                  | Rust (warp/actix-web) | Python (FastAPI)       | No       | serde (serializzazione)       | API versioning, documentazione con OpenAPI                  |
| Gestione file/risorse      | Rust                  | -                      | -        | std::fs, tokio::fs            | Gestione concorrente di I/O, caching                        |
| Monitoraggio e logging     | Rust                  | -                      | -        | tracing, prometheus           | Implementare trace distribuiti, metriche personalizzate     |
| Task Automation            | Rust                  | Python                 | Sì       | rayon (parallelismo)          | Progettare per scalabilità, gestione errori robusti         |
| Frontend Dinamico          | Svelte (TypeScript)   | -                      | -        | SvelteKit, TailwindCSS        | State management, SSR, ottimizzazione bundle                |
| Integrazione Blockchain    | Rust                  | -                      | -        | web3                          | Gestione delle transazioni asincrone, sicurezza             |
| Machine Learning           | Python                | Rust                   | Sì       | PyTorch, scikit-learn         | Ottimizzazione modelli, integrazione con Rust per inferenza |

Note Aggiuntive:

- Per ogni modulo, considerare l'implementazione di test unitari e di integrazione.
- Documentare le API interne ed esterne di ciascun modulo.
- Implementare un sistema di configurazione flessibile per ciascun modulo.
- Considerare l'uso di container Docker per la standardizzazione dell'ambiente di sviluppo e deployment.

## Distinzione tra Approcci Wrapping e Non-Wrapping

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
	    
1. **Approccio Non-Wrapping** (es. Task Automation): Rust e Python cooperano, con Python che orchestra e chiama funzioni Rust quando necessario. Esempio Rust (con PyO3):
    
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

# Folder

```sh
ArxFramework/
├── core/                 # Core Sistema (Rust)
├── auth/                 # Autenticazione e Sicurezza (Rust, Python opzionale)
├── crud/                 # Gestione CRUD (Rust)
├── api/                  # API Layer (Rust primario, Python FastAPI opzionale)
├── file_management/      # Gestione file/risorse (Rust)
├── monitoring/           # Monitoraggio e logging (Rust)
├── task_automation/      # Task Automation (Rust primario, Python secondario)
├── blockchain/           # Integrazione Blockchain (Rust)
├── frontend/             # Frontend dinamico (Svelte - TypeScript/JavaScript)
├── ml/                   # Machine Learning (Python primario, Rust secondario)
├── templates/            # Template per le 5 tipologie di applicazioni
├── docs/                 # Documentazione del framework
├── config/               # Configurazioni globali
├── scripts/              # Script di utilità per build, deploy, etc.
└── README.md             # Documentazione principale del framework
```

- **core/**
    - `system_core.rs`: Gestione centrale del sistema
    - `memory_management.rs`: Ottimizzazione della memoria
- **auth/**
    - `auth_core.rs`: Funzionalità core di autenticazione
    - `auth_wrapper.py`: Wrapper Python per integrazioni
- **crud/**
    - `crud_operations.rs`: Operazioni CRUD generiche
    - `models.rs`: Definizioni dei modelli dati
- **api/**
    - `api_server.rs`: Server API principale
    - `routes.rs`: Definizione dei percorsi API
    - `fastapi_integration.py`: Integrazione con FastAPI
- **file_management/**
    - `file_ops.rs`: Operazioni su file
    - `resource_manager.rs`: Gestione risorse
- **monitoring/**
    - `logger.rs`: Sistema di logging
    - `metrics.rs`: Raccolta e reporting metriche
- **task_automation/**
    - `task_core.rs`: Funzioni core per automazione
    - `automation_scripts.py`: Script di automazione Python
- **blockchain/**
    - `blockchain_integration.rs`: Interfaccia blockchain
    - `smart_contracts.rs`: Gestione smart contract
- **frontend/**
    - `App.svelte`: Componente root Svelte
    - `main.js`: Entry point dell'applicazione
    - `components/`: Cartella componenti riutilizzabili
- **ml/**
    - `ml_models.py`: Implementazione modelli ML
    - `data_processing.rs`: Elaborazione dati performante
- **templates/**
    - Vari file template per diverse tipologie di applicazioni
- **docs/**
    - File di documentazione per ogni modulo
- **config/**
    - File di configurazione per l'intero framework
- **scripts/**
    - Script per build, deploy e altri task di sviluppo

---

# Scope

>Ogni modulo ha il compito di processare task in base ad un input restituendo un output di status di avvenuta operazione al fine di avere uno script collettore delle operazioni.

## 1. Core Sistema
>Pertinente per: Tutte le applicazioni

Questo file contiene le strutture e le funzioni principali per l'inizializzazione e il controllo del core del sistema. Definisce i tipi di applicazione supportati, gestisce la configurazione globale, implementa il logging di base e fornisce un'interfaccia per le operazioni di sistema che si adattano al tipo di applicazione in uso. È il punto di entrata principale per l'interazione con il core del framework.

[Tameplate](ArxFramework/tameplates/core-system.md)
[Standard Procedure](./standard-procedures/core-system.md)

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
	
**Author:** Kenneth Boldrini