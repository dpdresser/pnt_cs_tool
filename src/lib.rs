pub mod cs_gui;
pub mod services;
use services::cs_model::{CSModel, CSModelEntry};
use services::db_api;

use crossbeam_channel::{Receiver, Sender};
use dashmap::DashMap;
use postgres::Client;
use std::error::Error;
use std::sync::Arc;

pub fn initialize_cs_models(
    db_client: &mut Client,
    cs_models: Arc<DashMap<String, CSModel>>,
) -> Result<(), Box<dyn Error>> {
    let query_result = services::db_api::load_cs_models(db_client)?;

    for row in query_result {
        let ticker: String = row.get(0);
        let entries_json: String = row.get(1);

        let entries: Vec<CSModelEntry> = serde_json::from_str(&entries_json)?;

        let cs_model = CSModel { ticker, entries };

        cs_models.insert(cs_model.ticker.clone(), cs_model);
    }

    Ok(())
}

pub fn save_cs_models_loop(db_client: &mut Client, cs_model_rx: Receiver<CSModel>) {
    for model in cs_model_rx.iter() {
        let query = model.db_string();
        println!("{}", &query);
        match db_api::save_cs_model(db_client, &query) {
            Ok(rows_inserted) => {
                println!("Saved rows: {}", rows_inserted);
            }
            Err(e) => {
                eprintln!("Failed to save to database: {:?}", e);
            }
        };
    }
}

pub fn run_gui(cs_models: Arc<DashMap<String, CSModel>>, cs_model_tx: Arc<Sender<CSModel>>) {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "PNT CS Tool",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(cs_gui::app::MyEguiApp::new(
                cc,
                cs_models,
                cs_model_tx,
            )))
        }),
    );
}
