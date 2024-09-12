# Standard Procedure

>Procedura Standard d'Uso per `system_core.rs` e `memory_management.rs`:

---

## Layer 1 Code Base `main.rs`

**Inizializzazione**:

- Importare i moduli necessari:

```Rust
use crate::core::{system_core, memory_management};
```
    
- Creare la configurazione del core:
	
> La variabile `config` ospiterà gli elementi necessari a tutti i moduli in maniera comune.
    
```Rust
let config = system_core::CoreConfig {     
	app_type: system_core::ApplicationType::WebApp, 
	// O altro tipo di applicazione    
	max_threads: 4,    
	log_level: logger::LogLevel::Info, // Modulo logger *
	/*
	... Altri tipi di configurazioni per altri moduli
	*/
};
```
	
`log_level: logger::LogLevel::Info,` [Modulo Logger](ArxFramework/module-development-tamplates/logging.md) 
	
- Inizializzare il CoreSystem:
	
```Rust
let core_system = system_core::CoreSystem::new(config).expect("Failed to initialize core system");
```
	
Utilizzo del Core System:
- Logging:
	
```Rust
core_system.log(system_core::LogLevel::Info, "Application started");
```
	
- Esecuzione di operazioni di sistema:	
	
```Rust
let operation = system_core::SystemOperation::SomeOperation; core_system.perform_operation(operation).expect("Operation failed");
```
	
Gestione della Memoria:
	
- Allocazione di memoria:
	
```Rust
let size = 1024; // dimensione in byte 
let memory_ptr = core_system.memory_manager.allocate(size).expect(
	"Memory allocation failed"
);
```
	
- Deallocazione di memoria:
	
```Rust
core_system.memory_manager.deallocate(memory_ptr).expect("Memory deallocation failed");
```
	
Gestione degli Errori:
- Utilizzare il tipo `Result<T, CoreError>` per gestire gli errori:
	
```Rust
match core_system.perform_operation(operation) {     
	Ok(_) => println!("Operation successful"),    
	Err(e) => eprintln!("Operation failed: {:?}", e), 
}
```
        
Chiusura del Sistema:
- Chiamare il metodo di shutdown quando l'applicazione termina:
	
```Rust
core_system.shutdown();
```

Questa procedura standard può essere utilizzata come base per integrare il Core Sistema in qualsiasi tipo di applicazione supportata. Gli sviluppatori possono seguire questi passaggi, adattando solo il tipo di applicazione e le operazioni specifiche necessarie per il loro contesto.

La procedura garantisce che:

- Il sistema venga inizializzato correttamente
- Le risorse di memoria siano gestite in modo appropriato
- Il logging e la gestione degli errori siano utilizzati in modo coerente
- Il sistema venga chiuso correttamente al termine dell'applicazione

Questa standardizzazione facilita l'uso coerente del Core Sistema attraverso diverse tipologie di applicazioni, mantenendo al contempo la flessibilità necessaria per adattarsi a requisiti specifici.

---
