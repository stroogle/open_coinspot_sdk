//! Intereacts with the public/open section of the coinspot API.
//! No authentication is required for this section of the library.
pub mod public_api;

/// The main entry point for the public API methods.
/// All methods are static.
pub struct CoinSpotPublic;