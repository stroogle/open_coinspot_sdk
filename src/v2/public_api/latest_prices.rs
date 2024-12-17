use std::collections::HashMap;
use crate::v2::{CoinSpotBadResponse, CoinSpotPublic, CoinSpotResponse, CoinSpotResult};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    pub bid: String,
    pub ask: String,
    pub last: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LatestPricesResponse {
    pub status: String,
    // At this stage, event though the API documentation states this is here on a successful request, it is not.
    // message: String,
    pub prices: HashMap<String, Price>
}

impl CoinSpotPublic {

    /// <https://www.coinspot.com.au/v2/api#latestprices>
    /// 
    /// Worth noting that even though the docs specify message: "ok" to exist on a successful request, it doesn't exist.
    pub async fn latest_prices() -> CoinSpotResult<LatestPricesResponse> {
        let res = reqwest::get("https://www.coinspot.com.au/pubapi/v2/latest").await?;

        match res.status() {
            StatusCode::OK => {
                let res_json: LatestPricesResponse = serde_json::from_str(&res.text().await?)?;
                return Ok(
                    CoinSpotResponse::Ok(res_json)
                )
            },
            StatusCode::BAD_REQUEST => {
                let res_json: CoinSpotBadResponse = serde_json::from_str(&res.text().await?)?;
                return Ok(
                    CoinSpotResponse::Bad(res_json)
                )
            },
            _ => return Err(format!("Failed to consider the status {:?}", res.status()).into())
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_latest_prices() {
    
        let result: CoinSpotResponse<LatestPricesResponse> = CoinSpotPublic::latest_prices().await.unwrap();

        match result {
            CoinSpotResponse::Ok(res) => {
                assert_eq!(res.status, "ok");
                assert!(res.prices.contains_key("btc"));
            },
            _ => {}
        }

        
    }
}
