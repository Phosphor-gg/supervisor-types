use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

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
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceInfo {
    pub price_id: String,
    pub product_id: String,
    pub tier: Tier,
    pub billing_cycle: BillingCycle,
    pub amount: i64, // Amount in cents
    pub currency: String,
    pub payment_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattedPrice {
    pub price_id: String,
    pub amount_cents: i64,
    pub amount_display: f64, // Amount in major currency units (e.g., dollars)
    pub currency: String,
}

impl FormattedPrice {
    pub fn currency_symbol(&self) -> &str {
        match self.currency.to_uppercase().as_str() {
            "USD" => "$",
            "EUR" => "€",
            "GBP" => "£",
            "JPY" => "¥",
            "CAD" => "CA$",
            "AUD" => "A$",
            "CHF" => "CHF ",
            "CNY" => "¥",
            "INR" => "₹",
            _ => &self.currency,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPricing {
    pub tier: Tier,
    pub prices: HashMap<BillingCycle, FormattedPrice>, // billing_cycle -> price
}

impl TierPricing {
    pub fn description(&self) -> String {
        self.tier.get_description().to_string()
    }

    pub fn features(&self) -> Vec<&'static str> {
        self.tier.get_features()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingData {
    pub tiers: Vec<TierPricing>,
    pub billing_cycles: Vec<BillingCycleInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingCycleInfo {
    pub cycle: BillingCycle,
    pub display_name: String,
    pub period_suffix: String,
    pub discount_percentage: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Hash, Eq, Copy)]
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

#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize, Eq, Hash)]
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

    pub fn get_referral_multiplier(&self) -> f64 {
        match self {
            Tier::Basic => 0.05,     // 5%
            Tier::Standard => 0.075, // 7.5%
            Tier::Premium => 0.1125, // 11.25%
            _ => 0.0,                // Free tier gets no multiplier
        }
    }

    pub fn get_description(&self) -> &str {
        match self {
            Tier::Basic => "For small communities.",
            Tier::Standard => "For growing communities.",
            Tier::Premium => "For large communities & orgs.",
            _ => "Free tier with limited features.",
        }
    }

    pub fn get_features(&self) -> Vec<&'static str> {
        match self {
            Tier::Basic => vec![
                "Access to Observer Model",
                "Discord Bot and Rest API Access",
                "Context Awareness",
                "2.5 Million Credits/Month",
            ],
            Tier::Standard => vec![
                "Everything in Basic",
                "Access to Sentinel Model",
                "7.5 Million Credits/Month",
            ],
            Tier::Premium => vec![
                "Everything in Standard",
                "Access to Arbiter Model",
                "22.5 Million Credits/Month",
            ],
            _ => vec![
                "Access to Observer Model",
                "Discord Bot and Rest API Access",
                "50000 Credits/Month",
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralStats {
    pub referral_code: String,
    pub total_referrals: usize,
    pub active_referrals: usize,
    pub current_multiplier: f64,
    pub referral_breakdown: Vec<ReferralInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralInfo {
    pub user_id: String,
    pub created_at: String,
    pub tier: Tier,
    pub is_active: bool,
    pub multiplier_contribution: f64,
}
