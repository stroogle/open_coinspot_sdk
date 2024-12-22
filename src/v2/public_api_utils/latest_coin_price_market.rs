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

    /// Used to get the latest prices of a specific coin, on a specific market.
    /// CoinSpot's API also throws a 404 error for invalid markets, this 404 does not return the error response you might expect.
    /// Therefore, neither does this sdk.
    pub async fn latest_coin_price_market(coin_symbol: &str, market: &str) -> CoinSpotResult<LatestPrice>{
        let url = format!("https://www.coinspot.com.au/pubapi/v2/latest/{}/{}", coin_symbol, market);
        
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
    async fn test_latest_coin_market_price() {
    
        let result = PublicUtils::latest_coin_price_market("btc", "usdt").await.unwrap();

        match result {
            CoinSpotResponse::Ok(res) => {
                assert_eq!(res.status, "ok");
            },
            _ => {}
        }    
    }

    #[tokio::test]
    #[should_panic]
    async fn test_latest_coin_market_price_panic() {
        let _result2 = PublicUtils::latest_coin_price_market("xrp", "usdt")
        .await
        .unwrap();
    }
}