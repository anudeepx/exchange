use std::collections::{BTreeMap, VecDeque};

use exchange_types::{
    CreateOrderData, MessageFromOrderbook, MessageToEngine, OrderPlacedPayload, Fill, OrderSide,
};
use redis::aio::ConnectionLike;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use tokio::signal;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone)]
struct Order {
    id: String,
    user_id: String,
    price: f64,
    qty: f64,
    side: OrderSide,
}

#[derive(Default)]
struct OrderBook {
    // price -> queue of orders at that price (FIFO)
    bids: BTreeMap<f64, VecDeque<Order>>, // highest first
    asks: BTreeMap<f64, VecDeque<Order>>, // lowest first
}

impl OrderBook {
    fn new() -> Self {
        Self::default()
    }

    /// Naive price-time priority matching for a single incoming order.
    fn match_order(&mut self, mut incoming: Order) -> (f64, Vec<Fill>) {
        let mut executed_qty = 0.0f64;
        let mut fills = Vec::new();

        match incoming.side {
            OrderSide::Buy => {
                // Match against best asks (lowest price)
                let mut levels: Vec<f64> = self.asks.keys().cloned().collect();
                levels.sort_by(|a, b| a.partial_cmp(b).unwrap());

                for price_level in levels {
                    if incoming.qty <= 0.0 {
                        break;
                    }
                    if price_level > incoming.price {
                        break;
                    }

                    if let Some(queue) = self.asks.get_mut(&price_level) {
                        while let Some(mut resting) = queue.pop_front() {
                            if incoming.qty <= 0.0 {
                                queue.push_front(resting);
                                break;
                            }

                            let trade_qty = incoming.qty.min(resting.qty);
                            incoming.qty -= trade_qty;
                            resting.qty -= trade_qty;
                            executed_qty += trade_qty;

                            fills.push(Fill {
                                price: price_level.to_string(),
                                qty: trade_qty,
                                trade_id: 0, // could be generated/sequenced later
                            });

                            if resting.qty > 0.0 {
                                queue.push_front(resting);
                                break;
                            }
                        }
                    }

                    if let Some(queue) = self.asks.get(&price_level) {
                        if queue.is_empty() {
                            self.asks.remove(&price_level);
                        }
                    }
                }
            }
            OrderSide::Sell => {
                // Match against best bids (highest price)
                let mut levels: Vec<f64> = self.bids.keys().cloned().collect();
                levels.sort_by(|a, b| b.partial_cmp(a).unwrap());

                for price_level in levels {
                    if incoming.qty <= 0.0 {
                        break;
                    }
                    if price_level < incoming.price {
                        break;
                    }

                    if let Some(queue) = self.bids.get_mut(&price_level) {
                        while let Some(mut resting) = queue.pop_front() {
                            if incoming.qty <= 0.0 {
                                queue.push_front(resting);
                                break;
                            }

                            let trade_qty = incoming.qty.min(resting.qty);
                            incoming.qty -= trade_qty;
                            resting.qty -= trade_qty;
                            executed_qty += trade_qty;

                            fills.push(Fill {
                                price: price_level.to_string(),
                                qty: trade_qty,
                                trade_id: 0,
                            });

                            if resting.qty > 0.0 {
                                queue.push_front(resting);
                                break;
                            }
                        }
                    }

                    if let Some(queue) = self.bids.get(&price_level) {
                        if queue.is_empty() {
                            self.bids.remove(&price_level);
                        }
                    }
                }
            }
        }

        // If there is remaining quantity, rest it in the book
        if incoming.qty > 0.0 {
            let levels = match incoming.side {
                OrderSide::Buy => &mut self.bids,
                OrderSide::Sell => &mut self.asks,
            };
            levels
                .entry(incoming.price)
                .or_insert_with(VecDeque::new)
                .push_back(incoming);
        }

        (executed_qty, fills)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageWrapper {
    client_id: String,
    user_id: String,
    message: MessageToEngine,
}

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("engine=info".parse().unwrap()))
        .init();

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into());
    let client = redis::Client::open(redis_url.clone())?;
    let pub_client = redis::Client::open(redis_url)?;

    let mut conn = client.get_async_connection().await?;
    let mut pub_conn = pub_client.get_async_connection().await?;

    tracing::info!("Engine started, waiting for messages from API via Redis…");

    let mut book = OrderBook::new();

    // Simple loop over a single Redis stream (backed by list `messages`)
    // In a more advanced design this would be a consumer group or stream.
    tokio::select! {
        _ = async {
            loop {
                // BLPOP blocks until a new message arrives.
                let (_key, payload): (String, String) = conn.blpop("messages", 0).await?;
                tracing::debug!("Received raw message: {}", payload);

                let wrapper: MessageWrapper = match serde_json::from_str(&payload) {
                    Ok(w) => w,
                    Err(e) => {
                        tracing::warn!("Failed to deserialize message wrapper: {e}");
                        continue;
                    }
                };

                let response = handle_message(&mut book, &wrapper.message, &wrapper.user_id);

                let serialized = serde_json::to_string(&response)
                    .expect("Failed to serialize engine response");

                // Publish back on the per-request channel.
                let channel = wrapper.client_id;
                tracing::debug!("Publishing response on channel {channel}: {serialized}");
                let _: () = pub_conn.publish(channel, serialized).await?;
            }
            #[allow(unreachable_code)]
            Ok::<(), redis::RedisError>(())
        } => {}

        _ = signal::ctrl_c() => {
            tracing::info!("Engine shutting down");
        }
    }

    Ok(())
}

fn handle_message(
    book: &mut OrderBook,
    msg: &MessageToEngine,
    user_id: &str,
) -> MessageFromOrderbook {
    match msg {
        MessageToEngine::CreateOrder { data } => handle_create_order(book, data, user_id),
        // For now, other message types are not implemented; a production system
        // would fully support them.
        _ => {
            tracing::warn!("Unimplemented engine message: {:?}", msg);
            MessageFromOrderbook::OpenOrders { payload: Vec::new() }
        }
    }
}

fn handle_create_order(
    book: &mut OrderBook,
    data: &CreateOrderData,
    user_id: &str,
) -> MessageFromOrderbook {
    let price: f64 = data.price.parse().unwrap_or(0.0);
    let qty: f64 = data.quantity.parse().unwrap_or(0.0);
    let order_id = uuid::Uuid::new_v4().to_string();

    let order = Order {
        id: order_id.clone(),
        user_id: user_id.to_string(),
        price,
        qty,
        side: data.side,
    };

    let (executed_qty, fills) = book.match_order(order);

    let payload = OrderPlacedPayload {
        order_id,
        executed_qty,
        fills,
    };

    MessageFromOrderbook::OrderPlaced { payload }
}

