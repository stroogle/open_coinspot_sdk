use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

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

pub enum Market<'a> {
    Coin(&'a str),
    TradePair(&'a str, &'a str)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Price {
    pub bid: String,
    pub ask: String,
    pub last: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LatestPrices {
    pub status: String,
    pub prices: HashMap<String, Price>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LatestPrice {
    pub status: String,
    pub prices: Price
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LatestActionPrice {
    pub status: String,
    pub message: String,
    pub rate: String,
    pub market: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Order {
    pub amount: f64,
    pub rate: f64,
    pub total: f64,
    pub coin: String,
    pub market: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OpenOrders {
    pub status: String,
    pub message: String,
    pub buyorders: Vec<Order>,
    pub sellorders: Vec<Order>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompleteOrder {
    pub amount: f64,
    pub rate: f64,
    pub total: f64,
    pub coin: String,
    pub market: String,
    pub solddate: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CompletedOrders {
    pub status: String,
    pub message: String,
    pub buyorders: Vec<CompleteOrder>,
    pub sellorders: Vec<CompleteOrder>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompletedOrdersSummary {
    pub status: String,
    pub message: String,
    pub orders: Vec<CompleteOrder>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StatusCheckResponse {
    pub status: String
}

