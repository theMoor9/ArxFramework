use crate::crud::models::table_scraper::{scrape};

let default_models = r".\Models\default"; 
let dev_models = r".\Models\dev"; 
if let Err(e) = scrape(default_models) || scrape(dev_models) {
    eprintln!("Errore: {}", e);  // Stampa un messaggio di errore in caso di fallimento
}