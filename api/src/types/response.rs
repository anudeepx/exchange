use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseMessage {
    pub message: String,
    pub status_code: u16,
    pub token: Option<String>,
    pub user_id: Option<uuid::Uuid>,
}