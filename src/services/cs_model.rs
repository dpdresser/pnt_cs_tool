// use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;

pub struct FactSetAuth {
    pub fs_url: String,
    pub fs_un: String,
    pub fs_key: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
pub enum CSModelEntryType {
    #[default]
    Debt,
    Preferred,
    NonControllingInterest,
    Cash,
    Shares,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CSModelEntry {
    pub formula: String,
    pub entry_type: CSModelEntryType,
    pub display_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CSModel {
    pub ticker: String,
    pub entries: Vec<CSModelEntry>,
}

impl CSModel {
    pub fn db_string(&self) -> String {
        let entries_json: String = serde_json::to_string(&self.entries).unwrap();

        format!(
            "INSERT INTO cs_models (ticker, entries)
            VALUES ('{}', '{}')
            ON CONFLICT (ticker)
            DO UPDATE SET entries = EXCLUDED.entries",
            self.ticker,
            entries_json,
        )
    }
}

#[derive(Default)]
pub struct AppNewCSModel {
    pub ticker: String,
    pub entries: Vec<CSModelEntry>,
    pub formula: String,
    pub entry_type: CSModelEntryType,
    pub display_name: String,
}

// pub async fn post_request(
//     request_data: Value,
//     fs_auth: &FactSetAuth,
//     client: Client,
// ) -> Result<String, reqwest::Error> {
//     let json_request = serde_json::to_string(&request_data).unwrap();
//     let request_result = client
//         .post(fs_auth.fs_url.clone())
//         .basic_auth(fs_auth.fs_un.clone(), Some(fs_auth.fs_key.clone()))
//         .body(json_request)
//         .header("Accept", "application/json")
//         .header("Content-Type", "application/json")
//         .send()
//         .await?;

//     request_result.text().await
// }

// #[derive(Serialize)]
// pub enum CSModelType {
//     PeriodDate,
//     ReleaseDate,
//     Shares,
//     Cash,
//     Debt,
//     Preferred,
//     NonControllingInterest,
// }

// #[derive(Serialize)]
// pub struct CSModel {
//     pub id: [String; 1],
//     pub formulas: Vec<String>,
//     pub types: Vec<CSModelType>,
//     pub display_names: Vec<String>,
// }

// impl CSModel {
//     pub fn new(id: [String; 1], formulas: Vec<String>, types: Vec<CSModelType>, display_names: Vec<String>) -> CSModel {
//         CSModel {
//             id,
//             formulas,
//             types,
//             display_names,
//         }
//     }
// }

// macro_rules! generate_cs_json {
//     ($cs_model:expr) => {
//         json!({
//             "data": {
//                 "ids": [$cs_model.id[0].clone() + "-US"],
//                 "formulas": $cs_model.formulas,
//                 "flatten": "Y",
//                 "displayName": $cs_model.display_names,
//             }
//         })
//     };
// }
