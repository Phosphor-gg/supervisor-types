use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationResponse {
    pub id: String,
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub created_at: String,
    pub is_read: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BroadcastNotificationRequest {
    pub title: String,
    pub message: String,
    #[serde(default = "default_notification_type")]
    pub notification_type: String,
}

fn default_notification_type() -> String {
    "announcement".to_string()
}