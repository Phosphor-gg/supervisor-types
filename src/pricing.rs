use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

/// Credits per 1 GBP. Based on £2.00 = 1,500,000 credits.
pub const CREDITS_PER_GBP: f64 = 750_000.0;

pub fn credits_to_gbp(credits: i64) -> f64 {
    credits as f64 / CREDITS_PER_GBP
}

pub fn format_credits_as_gbp(credits: i64) -> String {
    let gbp = credits_to_gbp(credits);
    let rounded_pence = (gbp * 100.0).round();
    if rounded_pence == rounded_pence.floor() && gbp >= 1.0 && (rounded_pence % 100.0) == 0.0 {
        format!("£{:.0}", gbp)
    } else {
        format!("£{:.2}", gbp)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBalance {
    pub used_current_period: i64,
    pub max_monthly_credits: i64,
    pub remaining_credits: i64,
    pub usage_percentage: f64,
    pub reset_date: Option<NaiveDateTime>,
    pub extra_credits: i64,
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
    #[serde(default)]
    pub subscription_id: Option<String>,
    /// Whether this customer owns the Verified lifetime plan, regardless of
    /// whether a subscription currently outranks it.
    #[serde(default)]
    pub lifetime_owned: bool,
    #[serde(default)]
    pub overdraft_enabled: bool,
    #[serde(default)]
    pub overdraft_limit: Option<i64>,
    #[serde(default)]
    pub overdraft_used: Option<i64>,
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
            subscription_id: None,
            lifetime_owned: false,
            overdraft_enabled: false,
            overdraft_limit: None,
            overdraft_used: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdraftSettingsRequest {
    pub enabled: bool,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdraftSettingsResponse {
    pub enabled: bool,
    pub limit: i64,
    pub used: i64,
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
    #[serde(default)]
    pub reset_date: Option<NaiveDateTime>,
    #[serde(default)]
    pub extra_credits: i64,
    #[serde(default)]
    pub discount_available: bool,
    #[serde(default)]
    pub discount_expires_at: Option<String>,
    /// Percentage value of the personal discount (e.g. 10 means 10% off).
    /// Only meaningful when `discount_available` is true.
    #[serde(default)]
    pub discount_percentage: Option<u32>,
    /// Whether the user has exhausted their current short-window rate limit.
    #[serde(default)]
    pub rate_limited: bool,
    /// When the current rate-limit window resets (RFC3339), if rate limited.
    #[serde(default)]
    pub rate_limit_resets_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyUsageEntry {
    pub day: String,
    pub credits_used: i64,
}

#[derive(Debug, Deserialize)]
pub struct DailyUsageQuery {
    pub days: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCheckoutSessionRequest {
    pub tier: Tier,
    pub billing_cycle: BillingCycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    /// Start a free trial instead of an immediate charge (Verified only:
    /// 3-day trial, card required, converts to the one-time Verified fee).
    #[serde(default)]
    pub trial: bool,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TierPricing {
    pub tier: Tier,
    pub prices: HashMap<BillingCycle, FormattedPrice>,
    #[serde(default)]
    pub monthly_credits: Option<i64>,
    #[serde(default)]
    pub feature_names: Vec<String>,
}

impl TierPricing {
    pub fn description(&self) -> String {
        self.tier.get_description().to_string()
    }

    pub fn features(&self) -> Vec<String> {
        if !self.feature_names.is_empty() {
            self.feature_names.clone()
        } else {
            self.tier.get_features().into_iter().map(String::from).collect()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlashSaleInfo {
    pub id: String,
    pub name: String,
    pub discount_percentage: i32,
    pub eligible_tiers: Option<Vec<Tier>>,
    pub ends_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PricingData {
    pub tiers: Vec<TierPricing>,
    pub billing_cycles: Vec<BillingCycleInfo>,
    #[serde(default)]
    pub flash_sale: Option<FlashSaleInfo>,
    #[serde(default)]
    pub free_tier_credits: Option<i64>,
}

/// The lifetime (Verified) plan: one-time purchase, no billing cycle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LifetimePlanInfo {
    pub product_id: String,
    pub price_id: String,
    pub name: String,
    /// Price in cents
    pub amount: i64,
    pub currency: String,
    pub monthly_credits: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// Lifetime plan: one-time purchase, no billing cycle. Resolved from
    /// Stripe customer metadata rather than a subscription.
    Verified,
}

impl Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tier_str = match self {
            Tier::Free => "Free",
            Tier::Basic => "Basic",
            Tier::Standard => "Standard",
            Tier::Premium => "Premium",
            Tier::Verified => "Verified",
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
            "verified" => Tier::Verified,
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
            "verified" => Tier::Verified,
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
            ],
            Tier::Standard => vec![
                "Everything in Basic",
                "Access to Sentinel Model",
                "Image Moderation",
                "Custom Bot Appearance",
                "Implicit Moderation",
            ],
            Tier::Premium => vec![
                "Everything in Standard",
                "Access to Arbiter Model",
                "Video Moderation (Coming Soon)",
                "Platform API (Earn Money)"
            ],
            _ => vec![
                "Access to Observer Model",
                "Discord Bot and Rest API Access",
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceInfo {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub created: String,
    pub hosted_invoice_url: Option<String>,
    pub invoice_pdf: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicesResponse {
    pub invoices: Vec<InvoiceInfo>,
}
