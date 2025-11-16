use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::user::UserProfile;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBalance {
    pub used_current_period: i64,
    pub max_monthly_credits: i64,
    pub remaining_credits: i64,
    pub usage_percentage: f64,
    pub reset_date: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditTransaction {
    pub id: String,
    pub amount: i64,
    pub transaction_type: String,
    pub model_type: Option<String>,
    pub bytes_processed: Option<i64>,
    pub description: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionInfo {
    pub tier: String,
    pub cycle: String,
    pub price: f64,
    pub expires_at: Option<NaiveDateTime>,
    pub max_monthly_credits: i64,
    pub next_refresh: Option<String>,
    pub is_active: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditsResponse {
    pub user: UserProfile,
    pub subscription: SubscriptionInfo,
    pub credits: CreditsInfo,
    pub transactions: Vec<CreditTransactionResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditsInfo {
    pub balance: i64,
    pub monthly_allocation: i64,
    pub used_this_month: i64,
    pub remaining_this_month: i64,
    pub usage_percentage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditTransactionResponse {
    pub id: String,
    pub amount: i64,
    pub transaction_type: String,
    pub model_type: Option<String>,
    pub bytes_processed: Option<i64>,
    pub description: String,
    pub created_at: String,
}