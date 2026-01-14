use serde::{Deserialize, Serialize};
use exchange_types::OrderSide;

#[derive(Serialize, Deserialize)]
pub struct NewOrder {
    pub market: String,   // e.g., "BTC-USD"
    pub price: String,    // String to represent decimal values precisely ex: "45000.50"
    pub quantity: String, // String to represent decimal values precisely ex: "0.005"
    #[serde(rename = "side")]
    pub side: OrderSide,
}

#[derive(Deserialize)]
pub struct OpenOrdersQuery {
    pub market: String,
}