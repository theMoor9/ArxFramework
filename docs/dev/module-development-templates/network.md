
| Module  | Primary Language | Support Language | Wrapping | Main Framework/Libraries |
| ------- | ---------------- | ---------------- | -------- | ------------------------ |
| Network | Rust<br>         | -<br>            | -        | Diesel                   |


## Multi database solution
1. **Adattabilità ai requisiti specifici**: Avendo un database diverso per ciascuna app, è possibile scegliere il sistema di gestione dati più adatto alle esigenze e alle caratteristiche di ogni singola applicazione. Questo permette di ottimizzare le prestazioni e le funzionalità in modo granulare.

2. **Indipendenza e separazione dei domini**: Mantenendo i dati di ciascuna app in un database dedicato, è garantita una maggiore indipendenza e separazione logica tra i diversi contesti applicativi.

3. **Flessibilità nello scaling**: E' possibile scalare orizzontalmente i singoli database in base alle necessità di carico di lavoro di ciascuna app, senza dover sottodimensionare o sovradimensionare un unico sistema centralizzato.

4. **Tecnologia ottimizzata per ogni app**: Scegliendo il database più adatto per ogni app (ad es. PostgreSQL per il backend, MongoDB per la webapp, SQLite per l'app desktop), é possibile sfruttare al meglio le caratteristiche e le funzionalità di ciascuna soluzione.

5. **Minore complessità di gestione**: Avendo database separati, la manutenzione, il backup, la sicurezza e altri aspetti amministrativi sono più semplici e circoscritti ad ogni singolo sistema.


## Schema

| App                    | Sistema    | Descrizione                                                                                                                                                                                           |
| ---------------------- | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Webapp                 | PostgreSQL | PostgreSQL offre un ottimo supporto per i dati relazionali e semi-strutturati. MongoDB è preferibile per esigenze di flessibilità dello schema e gestione di grandi quantità di dati non strutturati. |
| API Backend            | PostgreSQL | Entrambi i database relazionali sono ottimi per le API, offrendo robustezza, sicurezza e un ricco ecosistema di strumenti e librerie.                                                                 |
| Desktop App            | SQLite     | SQLite è leggero, serverless e facile da integrare, rendendolo una scelta ideale per applicazioni desktop con requisiti di footprint ridotto.                                                         |
| Automation e Scripting | MongoDB    | A seconda che tu abbia esigenze più orientate ai dati relazionali o non relazionali, PostgreSQL o MongoDB possono entrambi essere ottime soluzioni.                                                   |
| App Embedded           | SQLite     | SQLite è la scelta più adatta per applicazioni embedded con requisiti di footprint e semplicità di gestione.                                                                                          |

---

# Struttura

1. **`ConnectionConfig`** in `config/network_config.rs` Gestisce solo i parametri di configurazione.
2.  `network/connection_manager.rs`: Si occuperà della gestione della funzionalità. `initialize_connection`  inizializzerà la connessione usando i parametri di `ConnectionConfig`  per Diesel e mongodb rust driver .

```Rust
// network/connection_manager.rs

use diesel::prelude::*;
use log::{error, info};
use std::time::Duration;
use std::thread::sleep;

use crate::config::network_config::{ConnectionConfig, DatabaseType};

pub struct ConnectionManager {
    config: ConnectionConfig,
}

impl ConnectionManager {
    // Funzione per creare una nuova istanza del ConnectionManager con la configurazione fornita
    pub fn new(config: ConnectionConfig) -> Self {
        Self { config }
    }

    // Funzione di inizializzazione della connessione con tentativi di retry
    pub fn initialize_connection(&self) -> Result<PgConnection, diesel::ConnectionError> {
        let mut attempts = 0;
        loop {
            // Tenta di stabilire la connessione
            match self.connect() {
                Ok(connection) => {}
                Err(e) => {
	                // Aggiorna i tentativi e gestisce max_tries
                    attempts += 1;
                    if attempts >= self.config.max_retries {
	                    return Err(e)
                    }
                    sleep(self.config.retry_timeout);
                }
            }
        }
    }

    // Funzione privata per gestire la connessione a seconda del tipo di database
    fn connect(&self) -> Result<PgConnection, diesel::ConnectionError> {
        match self.config.database_type {
            DatabaseType::Postgres => {}
            DatabaseType::SQLite => {}
            DatabaseType::MongoDB => {}
        }
    }
}

```