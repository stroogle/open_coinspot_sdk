use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::v2::{CoinSpotPublic, CoinSpotResponse, CoinSpotResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct Prices {
    pub bid: String,
    pub ask: String,
    pub last: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LatestCoinMarketPriceResponse {
    pub status: String,
    pub prices: Prices
}

impl CoinSpotPublic {

    /// Used to get the latest prices of a specific coin.
    /// CoinSpot's API does not handle invalid coins, neither does this sdk.
    /// This method will throw a serde_json parse error in the event of an invalid coin input. 
    /// 
    pub async fn latest_coin_price_market(coin_symbol: &str, market: &str) -> CoinSpotResult<LatestCoinMarketPriceResponse>{
        let url = format!("https://www.coinspot.com.au/pubapi/v2/latest/{}/{}", coin_symbol, market);
        println!("{:?}", &url);
        let res = reqwest::get(
            &url
        ).await?;

        match res.status() {
            StatusCode::OK => {
                let text = res.text().await?;
                println!("{:?}", &text);
                let json: LatestCoinMarketPriceResponse = serde_json::from_str(&text)?;
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

mod tests {

    use super::*;

    #[tokio::test]
    async fn test_latest_coin_market_price() {
    
        let result = CoinSpotPublic::latest_coin_price_market("btc", "usdt").await.unwrap();

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
        let _result2 = CoinSpotPublic::latest_coin_price_market("xrp", "usdt")
        .await
        .unwrap();
    }
}