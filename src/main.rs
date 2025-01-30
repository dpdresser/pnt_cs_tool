use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct PriceResponse {
    #[serde(rename = "requestId")]
    ticker: String,
    #[serde(rename = "P_PRICE(-4D,0)")]
    price: f64,
    date: String,
}

struct PriceResponseWrapper {
    data: Vec<PriceResponse>,
}

struct FactSetAuth {
    fs_url: String,
    fs_un: String,
    fs_key: String,
}

async fn post_request(
    request_data: Value,
    fs_auth: &FactSetAuth,
    client: Client,
) -> Result<String, reqwest::Error> {
    let json_request = serde_json::to_string(&request_data).unwrap();
    let request_result = client
        .post(fs_auth.fs_url.clone())
        .basic_auth(fs_auth.fs_un.clone(), Some(fs_auth.fs_key.clone()))
        .body(json_request)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    request_result.text().await
}

#[derive(Serialize)]
struct CSModel {
    id: [String; 1],
    formulas: Vec<String>,
    display_names: Vec<String>,
}

impl CSModel {
    fn new(id: [String; 1], formulas: Vec<String>, display_names: Vec<String>) -> CSModel {
        CSModel {
            id,
            formulas,
            display_names,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let fs_auth = FactSetAuth {
        fs_url: dotenv::var("FACTSET_URI").unwrap(),
        fs_un: dotenv::var("FACTSET_UN").unwrap(),
        fs_key: dotenv::var("FACTSET_KEY").unwrap(),
    };

    let client = reqwest::Client::new();

    let mut cs_models = BTreeMap::new();

    // The following will be entered by a user
    let id = "ABCB".to_string();
    let formulas = vec!(
        "FF_ARPT_HEADER(DATA_PER,QTR,0,0,,\"BS\")".to_string(),
        "FF_ARPT_HEADER(FILE_DATE,QTR,0,0,,\"BS\")".to_string(),
        "FF_COM_SHS_OUT(QTR_R,0,0,,RF,,M)".to_string(),
        "FF_ARPT_SERIES(\"CASH AND CASH EQUIVALENTS\",QTR_R,0,0,M,\"BS\")".to_string(),
        "FF_ARPT_SERIES(\"OTHER BORROWINGS\",QTR_R,0,0,M,\"BS\")".to_string(),
        "FF_ARPT_SERIES(\"SUBORDINATED DEFERRABLE INTEREST DEBENTURES\",QTR_R,0,0,M,\"BS\")".to_string(),
    );

    let display_names = vec!(
        "earnings_period".to_string(),
        "earnings_release".to_string(),
        "shares".to_string(),
        "cash".to_string(),
        "debt_other".to_string(),
        "debt_subordinated".to_string(),
    );

    // Model generated and stored in BTreeMap
    let model = CSModel::new([id.clone()], formulas, display_names);

    cs_models.insert(id.clone(), model);

    // Need macro to generate request value based cs model fields
    // Something like: let request_date = generate_cs_model!(cs_models.get(&id));
    let request_data = json!({
        "data": {
            "ids": [id.clone().push_str("-US")],
            "formulas": cs_models.get(&id).unwrap().formulas,
            "flatten": "Y",
            "displayName": cs_models.get(&id).unwrap().display_names,
        }
    });

    let response_text = post_request(request_data, &fs_auth, client).await?;
    // Need macro to generate ModelResponseWrapper and ResponseStruct for each ticker
    // let response_wrapper: PriceResponseWrapper = serde_json::from_str(&response_text)?;
    // let response_data = response_wrapper.data;
    println!("{:?}", response_text);

    Ok(())
}
