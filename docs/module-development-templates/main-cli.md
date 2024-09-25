# Main - CLI

**`main.rs`**, viene utilizzato per avviare l'intera applicazione/framework. Il ruolo del **CLI** è di configurare l'ambiente e passare i parametri al **`main`**, che a sua volta delega il resto del lavoro al **`system_core`**.

### Struttura del Processo:

1. **CLI**: Raccoglie le configurazioni (tipo di applicazione, numero di thread, ecc.) e le passa al **`main.rs`**.
    
2. **`main.rs`**: Avvia l'applicazione in base ai parametri ricevuti, passando la palla al **`system_core`**.
    
3. **`system_core`**: Inizializza tutti i moduli necessari (come `auth`, `crud`, `api_layer`, ecc.) e gestisce l'intera logica del framework.
    

### Funzionamento:

- Il **`main.rs`** gestisce il **punto di ingresso** unico e coordina tutte le operazioni iniziali.
- Il **CLI** è un'**interfaccia** che consente all'utente di scegliere come configurare e avviare il sistema.
- **`system_core`** esegue la logica interna e gestisce il ciclo di vita dell'applicazione, basandosi sulle scelte effettuate tramite il CLI.

Il CLI quindi:

1. **Sarà un binario separato** che l'utente può installare e richiamare da terminale.
2. **Gestirà la configurazione dell'applicazione** (come i parametri dell'applicazione e la selezione dei moduli) e avvierà il framework in base a tali parametri.
3. **Richiamerà il  `system core` del framework o altre funzioni importanti del framework** con le giuste variabili di configurazione, avviando il sistema nella cartella progetto.



# src/`cli.rs`
Attraverso il CLI saranno personalizzati i seguenti aspetti:

- Tipo di Applicazione.
- Il setup del .toml per l'nclusione dei moduli strettamente necessari in relazione al tipo di applicazione scelta.
- Numero di threads per il core.
- Le configurazioni di memoria come il pool size e il buffer size.


```Rust
```


# src/`main.rs`

### Sequenza chiamata moduli

```Rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inizializza il sistema di logging
    monitoring::logger::setup_logging().expect("Errore nell'inizializzazione del sistema di logging");
    
    // Ottieni la configurazione dall'utente usando il CLI
    let config = cli::parse_core_config_cli(); 
    
    // Inizializza il CoreSystem con la configurazione ricevuta
    let core_system = CoreSystem::new(config).expect("Errore nell'inizializzazione del Core System");
    
    // Esegui il core system
    core_system.run()?;
    
    Ok(())
}

```