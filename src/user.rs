use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct UserCountResponse {
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub id: String, // Add the user's UUID
    pub discord_id: Option<String>,
    pub email: String,
    pub username: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardOverviewResponse {
    pub user: UserProfile,
    pub stats: DashboardStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub discord_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_api_calls: u64,
    pub api_calls_this_month: u64,
    pub active_api_keys: u32,
    pub subscription_usage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyReferralCodeRequest {
    pub code: String,
}