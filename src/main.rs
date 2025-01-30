use serde::Deserialize;
use serde_json::json;

type UnitResult = Result<(), Box<dyn std::error::Error>>;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct PriceResponse {
    #[serde(rename = "requestId")]
    ticker: String,
    #[serde(rename = "P_PRICE(-4D,0)")]
    price: f64,
    date: String,
}

#[derive(Debug, Deserialize)]
struct ResponseWrapper {
    data: Vec<PriceResponse>,
}

#[tokio::main]
async fn main() -> UnitResult {
    dotenv::dotenv().ok();

    let request_data = json!({
        "data": {
            "ids": ["SPY-US"],
            "formulas": ["P_PRICE(-4D,0)"],
            "calendar": "NAY",
            "flatten": "Y"
        }
    });

    let json_request = serde_json::to_string(&request_data).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post(dotenv::var("FACTSET_URI").unwrap())
        .basic_auth(
            dotenv::var("FACTSET_UN").unwrap(),
            Some(dotenv::var("FACTSET_KEY").unwrap()),
        )
        .body(json_request)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let response_text = res.text().await.unwrap();

    let response_wrapper: ResponseWrapper = serde_json::from_str(&response_text)?;
    let response_data = response_wrapper.data;
    println!("{:?}", response_data);

    Ok(())
}
