use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewOrder {
    pub id: String,
    pub quantity: u32,
    pub price: f64,
}