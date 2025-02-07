use postgres::{Client, Error, Row};

pub fn load_cs_models(db_client: &mut Client) -> Result<Vec<Row>, Error> {
    let query = "SELECT * FROM cs_models";
    db_client.query(query, &[])
}

pub fn save_cs_model(db_client: &mut Client, query: &str) -> Result<u64, Error> {
    db_client.execute(query, &[])
}
