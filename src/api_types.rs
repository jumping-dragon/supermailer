use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mail {
    pub pk: String,
    pub sk: i64,
    pub message_id: String,
    pub subject: String,
    pub from: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListEmailsResponse {
    pub data: Vec<Mail>,
}
