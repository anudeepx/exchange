use crate::types::order::CreateOrderData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MessageToEngine {
    #[serde(rename = "CREATE_ORDER")]
    CreateOrder { data: CreateOrderData },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderData {
    pub market: String,
    pub price: String,
    pub quantity: String,
    #[serde(rename = "side")]
    pub side: OrderSide,
}
