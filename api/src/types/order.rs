use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewOrder {
    pub market: String,
    pub price: String,
    pub quantity: String,
    #[serde(rename = "side")] // Rename to avoid conflict with reserved keyword
    pub side: OrderSide,
}

#[derive(Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}
