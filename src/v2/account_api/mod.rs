use super::{CoinSpotAccount, CoinSpotFullAccess, CoinSpotReadOnly};

mod read_only;

impl CoinSpotReadOnly {
    pub fn new(api_key: &str, secret: & str) -> Self {
        CoinSpotReadOnly {
            api_key: String::from(api_key),
            secret: String::from(secret)
        }
    }
}

impl CoinSpotFullAccess {
    pub fn new(api_key: &str, secret: & str) -> Self {
        CoinSpotFullAccess {
            api_key: String::from(api_key),
            secret: String::from(secret)
        }
    }
}

impl CoinSpotAccount {
    pub fn new(api_key: &str, secret: & str) -> Self {
        CoinSpotAccount {
            read_only: CoinSpotReadOnly::new(api_key, secret),
            full_access: CoinSpotFullAccess::new(api_key, secret),
        }
    }
}

