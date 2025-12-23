use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};

#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::orders)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub price: f64,
    pub total_quantity: f64,
    pub filled_quantity: f64,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::trades)]
pub struct Trade {
    pub id: Uuid,
    pub is_buyer: bool,
    pub price: String,
    pub quote_quantity: String,
    pub created_at: NaiveDateTime,
}