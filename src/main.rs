use reqwest::{Client, Response};
use serde::Deserialize;
use serde_json::{json, Value};

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

struct FactSetAuth {
    fs_url: String,
    fs_un: String,
    fs_key: String,
}

async fn post_request(
    request_data: Value,
    fs_auth: &FactSetAuth,
    client: Client,
) -> Result<Response, reqwest::Error> {
    let json_request = serde_json::to_string(&request_data).unwrap();
    client
        .post(fs_auth.fs_url.clone())
        .basic_auth(fs_auth.fs_un.clone(), Some(fs_auth.fs_key.clone()))
        .body(json_request)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await
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

    let request_data = json!({
        "data": {
            "ids": ["SPY-US"],
            "formulas": ["P_PRICE(-4D,0)"],
            "calendar": "NAY",
            "flatten": "Y"
        }
    });

    let res = post_request(request_data, &fs_auth, client).await?;
    let response_text = res.text().await.unwrap();

    let response_wrapper: ResponseWrapper = serde_json::from_str(&response_text)?;
    let response_data = response_wrapper.data;
    println!("{:?}", response_data);

    Ok(())
}
