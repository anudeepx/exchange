use crate::redis::redis_manager::RedisManager;
use crate::types::order::OpenOrdersQuery;
use crate::types::redis::{CancelOrderData, GetOpenOrdersData, UpdateOrderData};
use crate::types::{
    order_types::NewOrder,
    redis_types::{CreateOrderData, MessageToEngine},
};
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

// POST /api/v1/orders/{userid}
#[post("/")]
async fn create_order(userid: web::Path<String>, order: web::Json<NewOrder>) -> impl Responder {
    let redis_manager = RedisManager::get_instance().lock().unwrap();
    let request_body = order.into_inner();

    let message = MessageToEngine::CreateOrder {
        data: CreateOrderData {
            market: request_body.market,
            price: request_body.price,
            quantity: request_body.quantity,
            side: request_body.side,
        },
    };
    match redis_manager
        .send_and_await(message, userid.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// GET /api/v1/orders
#[get("/")]
async fn get_open_orders(
    user_id: web::ReqData<String>,
    query: web::Query<OpenOrdersQuery>,
) -> impl Responder {
    let redis_manager = RedisManager::get_instance().lock().unwrap();

    let message = MessageToEngine::GetOpenOrders {
        data: GetOpenOrdersData {
            market: query.market.clone(),
        },
    };

    match redis_manager
        .send_and_await(message, user_id.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// PUT /api/v1/orders/{order_id}
#[put("/{order_id}")]
async fn update_order(path: web::Path<String>) -> impl Responder {
    let order_id = path.into_inner();
    let redis_manager = RedisManager::get_instance().lock().unwrap();
    let message = MessageToEngine::UpdateOrder {
        data: UpdateOrderData {
            order_id: order_id.clone(),
        },
    };
    match redis_manager.send_and_await(message, order_id.clone()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish()
    };
    HttpResponse::Ok().body(format!("Update order {}", order_id))
}

// DELETE /api/v1/orders/{order_id}
#[delete("/{order_id}")]
async fn cancel_order(path: web::Path<String>, query: web::Query<CancelOrderData>) -> impl Responder {
    let order_id = path.into_inner();
    let redis_manager = RedisManager::get_instance().lock().unwrap();

    let message = MessageToEngine::CancelOrder {
        data: CancelOrderData{
            order_id: query.order_id.clone(),
            market: query.market.clone(),
        },
    };

    match redis_manager.send_and_await(message, order_id.clone()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish()
    };
    HttpResponse::Ok().body(format!("Cancel order {}", order_id))
}

// GET /api/v1/orders/history
#[get("/history")]
async fn order_history() -> impl Responder {
    let redis_manager = RedisManager::get_instance().lock().unwrap();

    let message = MessageToEngine::GetOpenOrders {
        data: GetOpenOrdersData {
            market: "ALL".to_string(),
        },
    };

    match  redis_manager
        .send_and_await(message, "history_request".to_string())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish(),
        
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_order)
        .service(get_open_orders)
        .service(update_order)
        .service(cancel_order)
        .service(order_history);
}
