use crate::v2::{
    CoinSpotPublic,
    PublicUtils,
    types::{
        CoinSpotResult,
        CompletedOrdersSummary,
        Market
    }
};

impl CoinSpotPublic {

    pub async fn completed_orders_summary(market: Market<'_>) -> CoinSpotResult<CompletedOrdersSummary> {
        match market {
            Market::Coin(coin_symbol) => {
                PublicUtils::completed_orders_coin_summary(coin_symbol)
                .await
            },
            Market::TradePair(coin_symbol, market) => {
                PublicUtils::completed_orders_coin_market_summary(coin_symbol, market)
                .await
            }
        }
    }

}