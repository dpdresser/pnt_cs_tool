#[macro_use]
pub mod services;

fn main() {
    dotenv::dotenv().ok();
    let _fs_auth = services::fs_api::FactSetAuth {
        fs_url: dotenv::var("FACTSET_URI").unwrap(),
        fs_un: dotenv::var("FACTSET_UN").unwrap(),
        fs_key: dotenv::var("FACTSET_KEY").unwrap(),
    };
    let connection_string = dotenv::var("DB_CONNECTION_STRING").unwrap();
    pnt_cs_tool::connect_to_database(&connection_string);

    pnt_cs_tool::run_gui();
}
