use reqwest::StatusCode;
use crate::v2::{
    CoinSpotPublic,
    types::{
        CoinSpotResponse,
        CoinSpotResult,
        LatestPrice
    }
};

impl CoinSpotPublic {

    /// Used to get the latest prices of a specific coin.
    /// CoinSpot's API does not handle invalid coins, neither does this sdk.
    /// This method will throw a serde_json parse error in the event of an invalid coin input. 
    /// 
    pub async fn latest_coin_price(coin_symbol: &str) -> CoinSpotResult<LatestPrice>{
        let url = format!("https://www.coinspot.com.au/pubapi/v2/latest/{}", coin_symbol);
        println!("{:?}", &url);
        let res = reqwest::get(
            &url
        ).await?;

        match res.status() {
            StatusCode::OK => {
                let text = res.text().await?;
                println!("{:?}", &text);
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
    
        let result = CoinSpotPublic::latest_coin_price("xrp").await.unwrap();

        match result {
            CoinSpotResponse::Ok(res) => {
                assert_eq!(res.status, "ok");
            },
            _ => {}
        }

        
    }
}