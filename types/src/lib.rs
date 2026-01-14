use serde::{Deserialize, Serialize};

/// Side of an order in the book.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Messages sent from the API into the matching engine over Redis.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MessageToEngine {
    #[serde(rename = "CREATE_ORDER")]
    CreateOrder { data: CreateOrderData },

    #[serde(rename = "GET_OPEN_ORDERS")]
    GetOpenOrders { data: GetOpenOrdersData },

    #[serde(rename = "CANCEL_ORDER")]
    CancelOrder { data: CancelOrderData },

    #[serde(rename = "UPDATE_ORDER")]
    UpdateOrder { data: UpdateOrderData },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrderData {
    pub market: String,
    pub price: String,
    pub quantity: String,
    #[serde(rename = "side")]
    pub side: OrderSide,
}

/// Messages sent from the engine back to the API / WS layer.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MessageFromOrderbook {
    #[serde(rename = "DEPTH")]
    Depth { payload: DepthPayload },

    #[serde(rename = "ORDER_PLACED")]
    OrderPlaced { payload: OrderPlacedPayload },

    #[serde(rename = "ORDER_CANCELLED")]
    OrderCancelled { payload: OrderCancelledPayload },

    #[serde(rename = "OPEN_ORDERS")]
    OpenOrders { payload: Vec<OpenOrder> },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DepthPayload {
    pub market: String,
    pub bids: Vec<[String; 2]>,
    pub asks: Vec<[String; 2]>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fill {
    pub price: String,
    pub qty: f64,
    pub trade_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderPlacedPayload {
    pub order_id: String,
    pub executed_qty: f64,
    pub fills: Vec<Fill>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderCancelledPayload {
    pub order_id: String,
    pub executed_qty: f64,
    pub remaining_qty: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenOrder {
    pub order_id: String,
    pub executed_qty: f64,
    pub price: String,
    pub quantity: String,
    #[serde(rename = "side")]
    pub side: OrderSide,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetOpenOrdersData {
    pub market: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelOrderData {
    pub order_id: String,
    pub market: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateOrderData {
    pub order_id: String,
}

