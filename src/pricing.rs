use std::fmt::Display;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeSubscriptionInfo {
    pub tier: Tier,
    pub billing_cycle: BillingCycle,
    pub status: String,
    pub current_period_end: Option<DateTime<Utc>>,
    pub cancel_at_period_end: bool,
    pub is_active: bool,
    pub price: Option<f64>, // Price in the smallest currency unit (e.g., pence, cents)
    pub currency: Option<String>, // Currency code (e.g., "gbp", "usd")
    pub payment_method_id: Option<String>, // Default payment method ID
}

impl StripeSubscriptionInfo {
    pub fn free() -> Self {
        Self {
            tier: Tier::Free,
            billing_cycle: BillingCycle::Monthly,
            status: "inactive".to_string(),
            current_period_end: None,
            cancel_at_period_end: false,
            is_active: false,
            price: None,
            currency: None,
            payment_method_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInfo {
    pub tier: Tier,
    pub cycle: BillingCycle,
    pub price: f64,
    pub expires_at: Option<NaiveDateTime>,
    pub max_monthly_credits: i64,
    pub is_active: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditsInfoResponse {
    pub balance: i64,
    pub monthly_allocation: i64,
    pub used_this_month: i64,
    pub remaining_this_month: i64,
    pub usage_percentage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCheckoutSessionRequest {
    pub tier: Tier,
    pub billing_cycle: BillingCycle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCheckoutSessionResponse {
    pub checkout_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionUpdateResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePlanRequest {
    pub tier: Tier,
    pub billing_cycle: BillingCycle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodResponse {
    pub last4: String,
    pub brand: String,
    pub exp_month: u32,
    pub exp_year: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BillingPortalResponse {
    pub portal_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleAutoRenewalRequest {
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleAutoRenewalResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Hash, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Free,
    Basic,
    Standard,
    Premium,
}

impl Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tier_str = match self {
            Tier::Free => "Free",
            Tier::Basic => "Basic",
            Tier::Standard => "Standard",
            Tier::Premium => "Premium",
        };
        write!(f, "{}", tier_str)
    }
}

impl From<String> for Tier {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "free" => Tier::Free,
            "basic" => Tier::Basic,
            "standard" => Tier::Standard,
            "premium" => Tier::Premium,
            _ => Tier::Free, // Default to Free for invalid values
        }
    }
}

impl From<&str> for Tier {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "free" => Tier::Free,
            "basic" => Tier::Basic,
            "standard" => Tier::Standard,
            "premium" => Tier::Premium,
            _ => Tier::Free, // Default to Free for invalid values
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    Annual,
    Triennial,
}

impl BillingCycle {
    pub fn period_suffix(&self) -> &str {
        match self {
            BillingCycle::Monthly => "month",
            BillingCycle::Quarterly => "3 months",
            BillingCycle::Annual => "year",
            BillingCycle::Triennial => "3 years",
        }
    }
    pub fn get_all_cycles() -> Vec<BillingCycle> {
        vec![
            BillingCycle::Monthly,
            BillingCycle::Quarterly,
            BillingCycle::Annual,
            BillingCycle::Triennial,
        ]
    }
    pub fn stripe_interval(&self) -> &'static str {
        match self {
            BillingCycle::Monthly => "month",
            BillingCycle::Quarterly => "month",
            BillingCycle::Annual => "year",
            BillingCycle::Triennial => "year",
        }
    }

    pub fn stripe_interval_count(&self) -> u32 {
        match self {
            BillingCycle::Monthly => 1,
            BillingCycle::Quarterly => 3,
            BillingCycle::Annual => 1,
            BillingCycle::Triennial => 3,
        }
    }
}

impl Display for BillingCycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cycle_str = match self {
            BillingCycle::Monthly => "Monthly",
            BillingCycle::Quarterly => "Quarterly",
            BillingCycle::Annual => "Annual",
            BillingCycle::Triennial => "Triennial",
        };
        write!(f, "{}", cycle_str)
    }
}

impl From<String> for BillingCycle {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "monthly" => BillingCycle::Monthly,
            "quarterly" => BillingCycle::Quarterly,
            "annual" => BillingCycle::Annual,
            "triennial" => BillingCycle::Triennial,
            _ => BillingCycle::Monthly, // Default to Monthly for invalid values
        }
    }
}

impl From<&str> for BillingCycle {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "monthly" => BillingCycle::Monthly,
            "quarterly" => BillingCycle::Quarterly,
            "annual" => BillingCycle::Annual,
            "triennial" => BillingCycle::Triennial,
            _ => BillingCycle::Monthly,
        }
    }
}

impl Tier {
    pub fn all_tiers() -> Vec<Tier> {
        vec![Tier::Free, Tier::Basic, Tier::Standard, Tier::Premium]
    }
}
