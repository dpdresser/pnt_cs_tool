use pnt_cs_tool::services::cs_model;

use dashmap::DashMap;
use postgres::{Client, NoTls};
use std::error::Error;
use std::sync::Arc;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables for calling FactSet API
    // and connecting to the PostgreSQL database
    dotenv::dotenv().ok();
    let _fs_auth = cs_model::FactSetAuth {
        fs_url: dotenv::var("FACTSET_URI").expect("No FACTSET_URI value found in .env"),
        fs_un: dotenv::var("FACTSET_UN").expect("No FACTSET_UN value found in .env"),
        fs_key: dotenv::var("FACTSET_KEY").expect("No FACTSET_KEY value found in .env"),
    };

    let connection_string =
        dotenv::var("DB_CONNECTION_STRING").expect("No DB_CONNECTION_STRING value found in .env");

    // Connect to PostgreSQL database
    let mut db_client = Client::connect(&connection_string, NoTls)?;

    // Initialize data containers and channels
    let cs_models: Arc<DashMap<String, cs_model::CSModel>> = Arc::new(DashMap::new());
    let cs_models_for_gui = cs_models.clone();
    let (cs_model_tx, cs_model_rx) = crossbeam_channel::unbounded::<cs_model::CSModel>();
    let cs_model_tx = Arc::new(cs_model_tx);
    let cs_model_tx_for_gui = cs_model_tx.clone();

    // Add existing CS models from the database
    pnt_cs_tool::initialize_cs_models(&mut db_client, cs_models)?;

    // Run thread to store and edit CS models
    let store_cs_models_thread = thread::spawn(move || {
        pnt_cs_tool::save_cs_models_loop(&mut db_client, cs_model_rx);
    });

    // Run the GUI
    pnt_cs_tool::run_gui(cs_models_for_gui, cs_model_tx_for_gui);

    // Clean up data channels
    drop(cs_model_tx);

    // Join spawned threads
    store_cs_models_thread.join().unwrap();

    Ok(())
}
