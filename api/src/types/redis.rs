// API-facing re-exports of the shared engine messaging types.
// This keeps the rest of the API code decoupled from the concrete crate name.
pub use exchange_types::{
    CancelOrderData,
    CreateOrderData,
    DepthPayload,
    Fill,
    GetOpenOrdersData,
    MessageFromOrderbook,
    MessageToEngine,
    OpenOrder,
    OrderCancelledPayload,
    OrderPlacedPayload,
    UpdateOrderData,
};