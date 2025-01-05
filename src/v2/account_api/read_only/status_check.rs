use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::v2::{
    signature, types::{
        CoinSpotResult,
        StatusCheckResponse,
        CoinSpotBadResponse,
        CoinSpotResponse
    },
    CoinSpotReadOnly
};

#[derive(Serialize, Deserialize)]
struct StatusCheckRequest {
    nonce: i32
}

impl CoinSpotReadOnly {

    /// Used to get the latest buy price of a specific coin.
    /// CoinSpot's API also throws a 400 error for invalid markets.
    /// This 400 error will return a CoinSpotResponse::Bad response
    pub async fn status_check(&self, nonce: i32) -> CoinSpotResult<StatusCheckResponse>{
        let url = String::from("https://www.coinspot.com.au/api/v2/ro/status");

        let client = reqwest::Client::new();

        let body = StatusCheckRequest {
            nonce
        };

        let request_body: String = serde_json::to_string(&body)?;

        let res = client.post(&url)
        .header("Content-Type", "application/json")
        .header("key", &self.api_key)
        .header("sign", signature::sign(&request_body, &self.secret).unwrap())
        .body(request_body.clone())
        .send()
        .await?;

        match res.status() {
            StatusCode::OK => {
                let text = res.text().await?;

                let json: StatusCheckResponse = serde_json::from_str(&text)?;

                Ok(
                    CoinSpotResponse::Ok(json)
                )
            },
            StatusCode::BAD_REQUEST => {
                let text = res.text().await?;

                let json: CoinSpotBadResponse = serde_json::from_str(&text)?;

                Ok(
                    CoinSpotResponse::Bad(json)
                )
            },
            StatusCode::UNAUTHORIZED => {
                let text = res.text().await?;

                let json: CoinSpotBadResponse = serde_json::from_str(&text)?;

                Ok(
                    CoinSpotResponse::Bad(json)
                )
            }
            _ => Err(format!("CoinSpot API never expects status: {:?}", res.status()).into())
        }

        // println!("{}", res.text().await.unwrap())
    }

}

#[cfg(test)]
mod tests {

    use std::env;
    use dotenv;
    use crate::v2::CoinSpotAccount;

    use super::*;

    #[tokio::test]
    async fn test_read_only_status_check() {

        dotenv::dotenv().ok();

        let api_key = env::var("COINSPOT_API_KEY").unwrap();
        let secret = env::var("COINSPOT_SECRET").unwrap();

        let coin = CoinSpotAccount::new(&api_key, &secret);
        let res = coin.read_only.status_check(5).await.unwrap();

        match res {
            CoinSpotResponse::Ok(x) => {
                assert_eq!(x.status, "ok")
            },
            _ => {}
        }
    }
}