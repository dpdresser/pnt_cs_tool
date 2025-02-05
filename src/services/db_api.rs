use postgres::{Client, Row};
use std::error::Error;

pub fn load_cs_models(db_client: &mut Client) -> Result<Vec<Row>, Box<dyn Error>> {
    let query = "SELECT * FROM cs_models"; 
    match db_client.query(query, &[]) {
        Ok(rows) => Ok(rows),
        Err(e) => return Err(format!("Error reading from PostgreSQL database: {:?}", e).into()),
    }
}