
//! Intereacts with the public/open section of the coinspot API.
//! No authentication is required for this section of the library.
pub mod public_api;

/// Holds all the types used by the v2 sdk.
/// Some responses have overlaping response interfaces.
/// This is reflected in types as compatible interfaces share types.
pub mod types;

/// The main entry point for the public API methods.
/// All methods are static.
pub struct CoinSpotPublic;