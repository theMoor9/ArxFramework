//! File `models.rs`
//! 
//! Questo file contiene i modelli base per le diverse applicazioni supportate da ArxFramework.
//! I modelli sono definiti in base alle specifiche di ciascuna applicazione (WebApp, API Backend, Desktop, ecc.).
//! Puoi estendere questi modelli o aggiungerne di nuovi seguendo alcune semplici linee guida per garantire la compatibilità con il sistema.

use chrono::{DateTime, Utc};

// ----------------------------
// Modelli Base per WebApp
// ----------------------------

#[cfg(feature = "webapp")]
#[derive(Debug, Clone)]
/// Modello per l'utente in una WebApp.
/// Può essere esteso con nuovi campi come `biometric_data` o ruoli aggiuntivi.
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub role: UserRole,
    pub biometric_data: Option<String>, // Può essere usato per autenticazione biometrica
}

#[cfg(feature = "webapp")]
#[derive(Debug, Clone)]
/// Ruoli disponibili per l'utente in una WebApp.
pub enum UserRole {
    Admin,
    Editor,
    Viewer,
}

#[cfg(feature = "webapp")]
#[derive(Debug, Clone)]
/// Modello per articoli in una WebApp.
/// Gli articoli possono avere titoli, contenuti, autore e tag.
pub struct Article {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub author_id: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[cfg(feature = "webapp")]
#[derive(Debug, Clone)]
/// Modello per i commenti sugli articoli.
/// Ogni commento è associato a un articolo e un autore.
pub struct Comment {
    pub id: u64,
    pub article_id: u64,
    pub author_id: u64,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[cfg(feature = "webapp")]
#[derive(Debug, Clone)]
/// Modello per la gestione dei file caricati (upload) in una WebApp.
/// Può essere esteso con altri metadati (es. tipo MIME).
pub struct FileUpload {
    pub id: u64,
    pub file_name: String,
    pub file_size: u64,
    pub uploaded_by: u64,
    pub upload_time: DateTime<Utc>,
}

// ----------------------------
// Modelli Base per API Backend
// ----------------------------

#[cfg(feature = "api_backend")]
#[derive(Debug, Clone)]
/// Modello per l'utente in un'API Backend.
/// Include il riferimento a una chiave API associata.
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub api_key_id: Option<u64>,
}

#[cfg(feature = "api_backend")]
#[derive(Debug, Clone)]
/// Modello per le chiavi API.
pub struct ApiKey {
    pub id: u64,
    pub key: String,
    pub user_id: u64,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "api_backend")]
#[derive(Debug, Clone)]
/// Modello per gli endpoint API.
/// Ogni endpoint ha un percorso e un metodo associato.
pub struct Endpoint {
    pub id: u64,
    pub path: String,
    pub method: String,
    pub description: Option<String>,
    pub active: bool,
}

#[cfg(feature = "api_backend")]
#[derive(Debug, Clone)]
/// Modello per la gestione dei log delle richieste API.
pub struct LogEntry {
    pub id: u64,
    pub endpoint_id: u64,
    pub status_code: u16,
    pub request_time: DateTime<Utc>,
    pub response_time: DateTime<Utc>,
    pub request_body: String,
    pub response_body: String,
}

// ----------------------------
// Modelli Base per Desktop App
// ----------------------------

#[cfg(feature = "desktop")]
#[derive(Debug, Clone)]
/// Modello per l'utente in un'applicazione desktop.
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub role: UserRole,
}

#[cfg(feature = "desktop")]
#[derive(Debug, Clone)]
/// Ruoli disponibili per l'utente in un'app desktop.
pub enum UserRole {
    Admin,
    User,
}

#[cfg(feature = "desktop")]
#[derive(Debug, Clone)]
/// Modello per risorse file in un'app desktop.
pub struct FileResource {
    pub id: u64,
    pub file_name: String,
    pub file_path: String,
    pub file_size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(feature = "desktop")]
#[derive(Debug, Clone)]
/// Modello per la gestione delle impostazioni in un'app desktop.
/// Può essere esteso per includere impostazioni aggiuntive come temi, scorciatoie, ecc.
pub struct Settings {
    pub id: u64,
    pub theme: String,
    pub language: String,
    pub notifications_enabled: bool,
}

// ----------------------------
// Modelli Base per Sistemi Embedded
// ----------------------------

#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
/// Modello per dispositivi embedded.
/// Può essere esteso con informazioni aggiuntive come configurazioni specifiche.
pub struct Device {
    pub id: u64,
    pub name: String,
    pub firmware_version: String,
    pub ip_address: Option<String>,
    pub last_seen: DateTime<Utc>,
}

#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
/// Modello per dati dei sensori nei sistemi embedded.
/// Ogni record di dati è associato a un dispositivo.
pub struct SensorData {
    pub id: u64,
    pub device_id: u64,
    pub sensor_type: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
}

#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
/// Modello per il firmware di un dispositivo.
/// Può essere esteso per includere informazioni aggiuntive come changelog dettagliati.
pub struct Firmware {
    pub id: u64,
    pub version: String,
    pub release_date: DateTime<Utc>,
    pub changelog: String,
}

#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
/// Modello per comandi inviati a dispositivi embedded.
pub struct Command {
    pub id: u64,
    pub device_id: u64,
    pub command: String,
    pub issued_at: DateTime<Utc>,
    pub status: CommandStatus,
}

#[cfg(feature = "embedded")]
#[derive(Debug, Clone)]
/// Stato dei comandi nei sistemi embedded.
pub enum CommandStatus {
    Pending,
    Executed,
    Failed,
}

// ----------------------------
// Modelli Base per Automazione e Scripting
// ----------------------------

#[cfg(feature = "automation")]
#[derive(Debug, Clone)]
/// Modello per script di automazione.
pub struct Script {
    pub id: u64,
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(feature = "automation")]
#[derive(Debug, Clone)]
/// Modello per la gestione dei task di automazione.
pub struct Task {
    pub id: u64,
    pub script_id: u64,
    pub schedule: String,
    pub last_run: DateTime<Utc>,
    pub next_run: DateTime<Utc>,
}

#[cfg(feature = "automation")]
#[derive(Debug, Clone)]
/// Modello per i log delle esecuzioni di script.
pub struct ExecutionLog {
    pub id: u64,
    pub script_id: u64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: String,
    pub output: String,
}

/// # Aggiunta di Nuovi Modelli
/// 
/// Per aggiungere nuovi modelli:
/// 
/// 1. Definisci una nuova `struct` con i campi necessari.
/// 2. Implementa i `trait` derivabili come `Debug`, `Clone`, e qualsiasi altro `trait` necessario.
/// 3. Usa i `cfg(feature = "...")` per abilitare il modello solo per specifiche applicazioni.
/// 4. Se necessario, crea nuove funzioni o moduli per la gestione del nuovo modello.
/// 
/// Nota: Assicurati di mantenere coerenza nei nomi dei campi e tipi per evitare conflitti.
/// 
/// # Estensione dei Modelli Esistenti
/// 
/// Per estendere i modelli esistenti:
/// 
/// 1. Aggiungi nuovi campi alla `struct` desiderata.
/// 2. Documenta il nuovo campo e spiega come viene utilizzato.
/// 3. Usa i `cfg(feature = "...")` per abilitare il campo solo per specifiche applicazioni, se applicabile.
/// 4. Aggiorna eventuali funzioni che utilizzano il modello per includere la logica aggiuntiva.
