use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct UserCountResponse {
    pub count: u64,
}

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: String, // Add the user's UUID
    pub discord_id: Option<String>,
    pub email: String,
    pub username: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
}