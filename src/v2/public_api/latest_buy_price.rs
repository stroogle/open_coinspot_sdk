use crate::v2::{
    CoinSpotPublic,
    PublicUtils,
    types::{
        CoinSpotResult,
        LatestActionPrice,
        Market
    }
};

impl CoinSpotPublic {

    pub async fn latest_buy_price(market: Market<'_>) -> CoinSpotResult<LatestActionPrice> {
        match market {
            Market::Coin(coin_symbol) => {
                PublicUtils::latest_buy_price(coin_symbol)
                .await
            },
            Market::TradePair(coin_symbol, market) => {
                PublicUtils::latest_buy_price_market(coin_symbol, market)
                .await
            }
        }
    }

}