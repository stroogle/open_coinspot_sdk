use crate::v2::{
    CoinSpotPublic,
    PublicUtils,
    types::{
        CoinSpotResult,
        LatestPrices,
    }
};

impl CoinSpotPublic {

    pub async fn latest_prices() -> CoinSpotResult<LatestPrices> {
        PublicUtils::latest_prices().await
    }

}