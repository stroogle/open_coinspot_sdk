use crate::v2::{
    CoinSpotPublic,
    PublicUtils,
    types::{
        CoinSpotResult,
        CompletedOrders,
        Market
    }
};

impl CoinSpotPublic {

    pub async fn completed_orders(market: Market<'_>) -> CoinSpotResult<CompletedOrders> {
        match market {
            Market::Coin(coin_symbol) => {
                PublicUtils::completed_orders_coin(coin_symbol)
                .await
            },
            Market::TradePair(coin_symbol, market) => {
                PublicUtils::completed_orders_coin_market(coin_symbol, market)
                .await
            }
        }
    }

}