
//! Intereacts with the public/open section of the coinspot API.
//! No authentication is required for this section of the library.
mod public_api_utils;
mod public_api;
mod account_api;
mod signature;

/// Holds all the types used by the v2 sdk.
/// Some responses have overlaping response interfaces.
/// This is reflected in types as compatible interfaces share types.
pub mod types;

/// The main entry point for the public API methods.
/// All methods are static.
pub struct CoinSpotPublic;

pub struct CoinSpotReadOnly {
    api_key: String,
    secret: String,
}

pub struct CoinSpotFullAccess {
    api_key: String,
    secret: String
}
pub struct CoinSpotAccount {
    pub read_only: CoinSpotReadOnly,
    pub full_access: CoinSpotFullAccess
}
struct PublicUtils;