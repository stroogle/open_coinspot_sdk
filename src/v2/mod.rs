
//! Intereacts with the public/open section of the coinspot API.
//! No authentication is required for this section of the library.
pub mod public_api;

use std::error::Error;
use serde::{Deserialize, Serialize};

/// CoinSpot's api documentation says that any bad response will fall into this format.
#[derive(Serialize, Deserialize, Debug)]
pub struct CoinSpotBadResponse {
    pub status: String,
    pub message: String,
}

/// This enum encompasses the expected responses from the CoinSpot api. It uses a generic type enable easier pattern matching.
pub enum CoinSpotResponse<T> {
    Ok(T),
    Bad(CoinSpotBadResponse)
}

/// This is an alias of Rust's Result type, the generic passed in will map to the Ok type in CoinSpotResponse.
pub type CoinSpotResult<T> = Result<CoinSpotResponse<T>, Box<dyn Error>>;

/// The main entry point for the public API methods.
/// All methods are static.
pub struct CoinSpotPublic;