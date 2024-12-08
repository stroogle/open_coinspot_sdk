use std::collections::HashMap;
use std::error::Error;
use crate::v2::CoinSpotPublic;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Price {
    bid: String,
    ask: String,
    last: String,
}

#[derive(Serialize, Deserialize)]
pub struct LatestPricesResponse {
    status: String,
    // At this stage, event though the API documentation states this is here on a successful request, it is not.
    // message: String,
    prices: HashMap<String, Price>
}

impl CoinSpotPublic {

    /// <https://www.coinspot.com.au/v2/api#latestprices>
    /// 
    /// Worth noting that even though the docs specify message: "ok" to exist on a successful request, it doesn't exist.
    pub async fn latest_prices() -> Result<LatestPricesResponse, Box<dyn Error>> {
        let res = reqwest::get("https://www.coinspot.com.au/pubapi/v2/latest").await?;

        let res_json: LatestPricesResponse = serde_json::from_str(&res.text().await?)?;

        Ok(res_json)
    }

}

mod tests {

    use super::*;

    #[tokio::test]
    async fn test_latest_prices() {
    
        let result = CoinSpotPublic::latest_prices().await.unwrap();

        assert_eq!(result.status, "ok");

        assert!(result.prices.contains_key("btc"));
        
    }
}
