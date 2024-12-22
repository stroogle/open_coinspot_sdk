use reqwest::StatusCode;
use crate::v2::{
    PublicUtils,
    types::{
        CoinSpotResponse,
        CoinSpotResult,
        LatestPrice
    }
};

impl PublicUtils {

    /// Used to get the latest prices of a specific coin.
    /// CoinSpot's API does not handle invalid coins, neither does this sdk.
    /// This method will throw a serde_json parse error in the event of an invalid coin input. 
    /// 
    pub async fn latest_coin_price(coin_symbol: &str) -> CoinSpotResult<LatestPrice>{
        let url = format!("https://www.coinspot.com.au/pubapi/v2/latest/{}", coin_symbol);
        
        let res = reqwest::get(
            &url
        ).await?;

        match res.status() {
            StatusCode::OK => {
                let text = res.text().await?;
                
                let json: LatestPrice = serde_json::from_str(&text)?;
                return Ok(
                    CoinSpotResponse::Ok(json)
                )
            },
            _ => {
                return Err(format!("CoinSpot API never expects status: {:?}", res.status()).into())
            }
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_latest_coin_price() {
    
        let result = PublicUtils::latest_coin_price("xrp").await.unwrap();

        match result {
            CoinSpotResponse::Ok(res) => {
                assert_eq!(res.status, "ok");
            },
            _ => {}
        }

        
    }
}