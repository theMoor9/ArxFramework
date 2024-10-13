# Main - CLI

**`main.rs`**, viene utilizzato per avviare l'intera applicazione/framework. Il ruolo del **CLI** è di configurare l'ambiente e passare i parametri al **`main`**, che a sua volta configura il resto dei moduli.


### Relazione tra `cli.rs` e `main.rs`

- **`cli.rs`**: Questo file si occupa di gestire l'input dell'utente, ovvero l'input proveniente dalla riga di comando (`arx init --AppType webapp`). Quando esegui il comando `arx init --AppType webapp`, il `cli.rs` interpreta e analizza le opzioni fornite dall'utente (in questo caso `webapp` come tipo di applicazione), e passa l'informazione all'`applicazione` attraverso il suo output.
    
- **`main.rs`**: Questo file è il cuore dell'applicazione, che usa le informazioni fornite da `cli.rs` per configurare il sistema. In sostanza, `main.rs` prende i valori che `cli.rs` ha raccolto dall'utente e, in base a questi, esegue la logica necessaria per inizializzare il sistema e configurare correttamente l'applicazione. In particolare, `main.rs` utilizza la configurazione passata da `cli.rs` per avviare il `CoreSystem`.

### Processo di Creazione del Progetto con il CLI

Immagina di eseguire il comando `arx init --AppType webapp` da terminale. Qui avviene una serie di azioni concatenate:

1. **Parsing degli Argomenti CLI in `cli.rs`**:
    
    - Quando scrivi `arx init --AppType webapp`, il comando viene passato a `cli.rs`, che lo analizza (parsing) e determina che si tratta di un'inizializzazione per un tipo di applicazione `webapp`.
    - `cli.rs` cattura questo input (`webapp`) e lo passa sotto forma di struttura (oggetto `CoreConfig`) al `main.rs`.
    
2. **Configurazione dell'Applicazione in `main.rs`**:
    
    - Una volta che `cli.rs` ha passato le informazioni necessarie al `main.rs`, quest'ultimo crea un'istanza del `CoreSystem` utilizzando le configurazioni fornite.
    - Queste configurazioni vengono usate per settare correttamente l'applicazione (`webapp`, `api_backend`, ecc.).
    
3. **Generazione della Struttura del Progetto**:
    
    - Una delle funzioni di `main.rs` (o di uno specifico modulo del framework) provvederà a generare i file necessari per la struttura del progetto in base al tipo di applicazione selezionato. Ad esempio, per una `WebApp`, vengono generati file specifici come `routes.rs`, `api_server.rs`, e così via.
    
4. **Inizializzazione e Avvio**:
    
    - Una volta generata la struttura del progetto, il sistema si avvia con i parametri configurati, lanciando eventualmente un server (se si tratta di un'API o WebApp) o configurando i moduli per altre tipologie di applicazioni come `desktop` o `embedded`.