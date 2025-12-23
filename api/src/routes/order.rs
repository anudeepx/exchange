use actix_web::{post, get, put, delete, web, HttpResponse, Responder};
use crate::types::order_types::NewOrder;

// POST /api/v1/orders
#[post("/")]
async fn create_order(
    userid: web::Path<String>,
    order: web::Json<NewOrder>,
) -> impl Responder {
    
    HttpResponse::Ok().body("Place new order")
}

// GET /api/v1/orders
#[get("/")]
async fn list_orders() -> impl Responder {
    HttpResponse::Ok().body("List user orders")
}

// GET /api/v1/orders/{order_id}
#[get("/{order_id}")]
async fn get_order(path: web::Path<String>) -> impl Responder {
    let order_id = path.into_inner();
    HttpResponse::Ok().body(format!("Get order {}", order_id))
}

// PUT /api/v1/orders/{order_id}
#[put("/{order_id}")]
async fn update_order(path: web::Path<String>) -> impl Responder {
    let order_id = path.into_inner();
    HttpResponse::Ok().body(format!("Update order {}", order_id))
}

// DELETE /api/v1/orders/{order_id}
#[delete("/{order_id}")]
async fn cancel_order(path: web::Path<String>) -> impl Responder {
    let order_id = path.into_inner();
    HttpResponse::Ok().body(format!("Cancel order {}", order_id))
}

// GET /api/v1/orders/history
#[get("/history")]
async fn order_history() -> impl Responder {
    HttpResponse::Ok().body("Order history")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_order)
        .service(list_orders)
        .service(get_order)
        .service(update_order)
        .service(cancel_order)
        .service(order_history);
}
