//! Modulo per la gestione della memoria in base al tipo di applicazione.
//!
//! Questo modulo fornisce un gestore della memoria che implementa diverse strategie di allocazione
//! a seconda del tipo di applicazione. Le strategie attualmente supportate includono:
//! - `Standard`: allocazione standard, utilizza il sistema di allocazione predefinito di Rust.
//! - `PoolBased`: allocazione basata su un pool di buffer pre-allocati per migliorare le prestazioni.
//! - `CustomEmbedded`: allocazione personalizzata per applicazioni embedded con requisiti specifici.
//!
//! È possibile configurare la dimensione dei buffer e del pool utilizzando la struttura `MemoryConfig`.

use crate::config::global_config::{ApplicationType, MemoryConfig};
use crate::system_core::CoreError;
use log::{info};
use std::collections::VecDeque;

/// Enum per rappresentare le diverse strategie di allocazione della memoria.
enum AllocationStrategy {
    Standard,
    PoolBased,
    CustomEmbedded,
}

/// `MemoryManager` gestisce l'allocazione e la deallocazione della memoria in base alla strategia
/// selezionata dall'applicazione.
///
/// # Campi
/// - `allocation_strategy`: La strategia di allocazione utilizzata.
/// - `pool`: Un pool di buffer pre-allocati (usato solo nella strategia `PoolBased`).
/// - `memory_config`: Configurazione della memoria fornita dall'utente.
pub struct MemoryManager {
    allocation_strategy: AllocationStrategy,
    pool: Option<VecDeque<Box<[u8]>>>, // Pool per l'allocazione basata su pool
    memory_config: MemoryConfig,  // Configurazione della memoria
}

impl MemoryManager {
    /// Crea un nuovo gestore della memoria per l'applicazione specificata.
    ///
    /// # Parametri
    /// - `app_type`: Il tipo di applicazione (ad esempio, `WebApp`, `ApiBackend`, `DesktopApp`, ecc.).
    /// - `memory_config`: La configurazione della memoria che specifica le dimensioni dei buffer e del pool.
    ///
    /// # Ritorna
    /// Un'istanza di `MemoryManager` o un errore di tipo `CoreError` in caso di fallimento.
    pub fn new(app_type: ApplicationType, memory_config: MemoryConfig) -> Result<Self, CoreError> {
        info!("Inizializzazione del MemoryManager...");

        // Determina la strategia di allocazione in base al tipo di applicazione.
        let strategy = match app_type {
            ApplicationType::WebApp | ApplicationType::ApiBackend => AllocationStrategy::PoolBased,
            ApplicationType::DesktopApp => AllocationStrategy::Standard,
            ApplicationType::AutomationScript => AllocationStrategy::Standard,
            ApplicationType::EmbeddedSystem => AllocationStrategy::CustomEmbedded,
        };

        // Inizializza il pool solo se la strategia è `PoolBased`, utilizzando il `pool_size` configurato.
        let pool = if let AllocationStrategy::PoolBased = strategy {
            let mut pool = VecDeque::new();
            for _ in 0..memory_config.pool_size {
                pool.push_back(vec![0u8; memory_config.buffer_size].into_boxed_slice());
            }
            Some(pool)
        } else {
            None
        };

        Ok(MemoryManager { allocation_strategy: strategy, pool, memory_config })
    }

    /// Alloca memoria in base alla strategia configurata.
    ///
    /// # Parametri
    /// - `size`: La quantità di memoria da allocare in byte.
    ///
    /// # Ritorna
    /// Un buffer di memoria (`Box<[u8]>`) o un errore di tipo `CoreError` in caso di fallimento.
    ///
    /// # Nota
    /// - La strategia `Standard` alloca dinamicamente la memoria.
    /// - La strategia `PoolBased` utilizza buffer pre-allocati dal pool.
    /// - La strategia `CustomEmbedded` utilizza una configurazione fissa per i buffer.
    pub fn allocate(&mut self, size: usize) -> Result<Box<[u8]>, CoreError> {
        info!("Allocazione di {} byte di memoria...", size);
        match self.allocation_strategy {
            AllocationStrategy::Standard => {
                let buffer = vec![0u8; size].into_boxed_slice();
                Ok(buffer)
            },
            AllocationStrategy::PoolBased => {
                if let Some(ref mut pool) = self.pool {
                    if let Some(buffer) = pool.pop_front() {
                        Ok(buffer)
                    } else {
                        // Pool esaurito, alloca dinamicamente
                        let buffer = vec![0u8; size].into_boxed_slice();
                        Ok(buffer)
                    }
                } else {
                    Err(CoreError::ResourceAllocationError("Pool non disponibile".to_string()))
                }
            },
            AllocationStrategy::CustomEmbedded => {
                // Usa la dimensione configurata per i buffer negli embedded.
                let buffer = vec![0u8; self.memory_config.buffer_size].into_boxed_slice();
                Ok(buffer)
            },
        }
    }

    /// Dealloca memoria precedentemente allocata.
    ///
    /// # Parametri
    /// - `buffer`: Il buffer di memoria da deallocare.
    ///
    /// # Ritorna
    /// `Ok(())` se la deallocazione ha successo, oppure un errore di tipo `CoreError`.
    ///
    /// # Nota
    /// - Nella strategia `Standard`, Rust dealloca automaticamente la memoria.
    /// - Nella strategia `PoolBased`, il buffer viene restituito al pool.
    /// - Nella strategia `CustomEmbedded`, non è richiesta alcuna azione specifica.
    pub fn deallocate(&mut self, buffer: Box<[u8]>) -> Result<(), CoreError> {
        info!("Deallocazione della memoria...");
        match self.allocation_strategy {
            AllocationStrategy::Standard => {
                // Rust dealloca automaticamente la memoria.
                Ok(())
            },
            AllocationStrategy::PoolBased => {
                // Restituisce il buffer al pool.
                if let Some(ref mut pool) = self.pool {
                    pool.push_back(buffer);
                    Ok(())
                } else {
                    Err(CoreError::ResourceAllocationError("Pool non disponibile".to_string()))
                }
            },
            AllocationStrategy::CustomEmbedded => {
                // Gestione personalizzata per sistemi embedded.
                Ok(())
            },
        }
    }
}

/// # Aggiunta di Nuovi Modelli o Strategie di Allocazione
/// Se desideri aggiungere nuovi modelli o strategie di allocazione, segui questi passaggi:
/// 1. **Nuova Strategia**: Aggiungi una nuova variante all'enum `AllocationStrategy` per rappresentare la tua strategia.
/// 2. **Logica di Allocazione**: Aggiorna i metodi `allocate` e `deallocate` per gestire la nuova strategia.
/// 3. **Configurazione**: Assicurati di aggiungere nuove opzioni alla struttura `MemoryConfig` per configurare il comportamento della nuova strategia.
/// 4. **Testing**: Scrivi test unitari per verificare il corretto funzionamento della tua nuova strategia di allocazione.
