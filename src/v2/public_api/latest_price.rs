use crate::v2::{
    CoinSpotPublic,
    PublicUtils,
    types::{
        CoinSpotResult,
        LatestPrice,
        Market
    }
};

impl CoinSpotPublic {

    pub async fn latest_price(market: Market<'_>) -> CoinSpotResult<LatestPrice> {
        match market {
            Market::Coin(coin_symbol) => {
                PublicUtils::latest_coin_price(coin_symbol)
                .await
            },
            Market::TradePair(coin_symbol, market) => {
                PublicUtils::latest_coin_price_market(coin_symbol, market)
                .await
            }
        }
    }

}