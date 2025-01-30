use dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;

type UnitResult = Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> UnitResult {
    dotenv::dotenv().ok();

    let mut request_headers = HeaderMap::new();
    request_headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());
    request_headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());

    let request_data = json!({
        "data": {
            "ids": ["SPY-US"],
            "formulas": ["P_PRICE(-4D,0)"],
            "calendar": "NAY",
            "flatten": "Y"
        }
    });

    let json_request = serde_json::to_string(&request_data).unwrap();
    println!("JSON Request: {}", json_request);

    let client = reqwest::Client::new();
    let res = client.post(dotenv::var("FACTSET_URI").unwrap())
        .basic_auth(dotenv::var("FACTSET_UN").unwrap(), Some(dotenv::var("FACTSET_KEY").unwrap()))
        .body(json_request)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    println!("{:?}", res.text().await.unwrap());

    Ok(())
}
