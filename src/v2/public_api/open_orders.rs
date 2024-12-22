use crate::v2::{
    CoinSpotPublic,
    PublicUtils,
    types::{
        CoinSpotResult,
        OpenOrders,
        Market
    }
};

impl CoinSpotPublic {

    pub async fn open_orders(market: Market<'_>) -> CoinSpotResult<OpenOrders> {
        match market {
            Market::Coin(coin_symbol) => {
                PublicUtils::open_orders_coin(coin_symbol)
                .await
            },
            Market::TradePair(coin_symbol, market) => {
                PublicUtils::open_orders_coin_market(coin_symbol, market)
                .await
            }
        }
    }

}