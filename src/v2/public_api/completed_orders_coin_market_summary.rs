use reqwest::StatusCode;

use crate::v2::{
    CoinSpotPublic,
    types::{
        CoinSpotBadResponse,
        CoinSpotResponse,
        CoinSpotResult,
        CompletedOrdersSummary
    }
};

impl CoinSpotPublic {

    /// Used to get the latest buy price of a specific coin.
    /// CoinSpot's API also throws a 400 error for invalid markets.
    /// This 400 error will return a CoinSpotResponse::Bad response
    pub async fn completed_orders_coin_market_summary(coin_symbol: &str, market: &str) -> CoinSpotResult<CompletedOrdersSummary>{
        let url = format!("https://www.coinspot.com.au/pubapi/v2/orders/summary/completed/{}/{}", coin_symbol, market);
        
        let res = reqwest::get(
            &url
        ).await?;

        match res.status() {
            StatusCode::OK => {
                let text = res.text().await?;
                
                let json: CompletedOrdersSummary = serde_json::from_str(&text)?;
                return Ok(
                    CoinSpotResponse::Ok(json)
                )
            },
            StatusCode::BAD_REQUEST => {
                let text = res.text().await?;
                
                let json: CoinSpotBadResponse = serde_json::from_str(&text)?;
                return Ok(
                    CoinSpotResponse::Bad(json)
                )
            }
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
    async fn test_completed_orders_coin_market_summary() {
    
        let result: CoinSpotResponse<CompletedOrdersSummary>;
        result = CoinSpotPublic::completed_orders_coin_market_summary("btc", "usdt")
        .await
        .unwrap();

        match result {
            CoinSpotResponse::Ok(res) => {
                assert_eq!(res.status, "ok");
                assert_eq!(res.message, "ok");
                assert_eq!(res.orders[0].coin, "BTC");
            },
            _ => {}
        }    
    }

    #[tokio::test]
    async fn test_completed_orders_fake_coin_market() {
        let result2 = CoinSpotPublic::completed_orders_coin_market_summary("sdfsdf", "usdt")
        .await
        .unwrap();

        match result2 {
            CoinSpotResponse::Bad(res) => {
                assert_eq!(res.status, "error");
                assert_eq!(res.message, "Coin not found");
            },
            _ => assert!(false)
        }
    }

    #[tokio::test]
    async fn test_completed_orders_fake_market() {
        let result2 = CoinSpotPublic::completed_orders_coin_market_summary("btc", "usdt123")
        .await
        .unwrap();

        match result2 {
            CoinSpotResponse::Bad(res) => {
                assert_eq!(res.status, "error");
                assert_eq!(res.message, "Invalid market");
            },
            _ => assert!(false)
        }
    }
}