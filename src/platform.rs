use serde::{Deserialize, Serialize};

use crate::pricing::{BillingCycle, Tier};

// Registration

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformRegistrationRequest {
    pub name: String,
    pub redirect_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_primary_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_accent_color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformRegistrationResponse {
    pub platform_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub stripe_onboarding_url: Option<String>,
}

// Token exchange (OAuth2 client_credentials)

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformTokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

// User provisioning

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvisionUserRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvisionUserResponse {
    pub user_id: String,
    pub email: String,
    pub is_new_account: bool,
    pub is_newly_linked: bool,
}

// Checkout

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformCheckoutRequest {
    pub user_email: String,
    pub tier: Tier,
    pub billing_cycle: BillingCycle,
    pub success_url: String,
    pub cancel_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformCheckoutResponse {
    pub checkout_url: String,
}

// User info

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformUserInfo {
    pub user_id: String,
    pub email: String,
    pub linked_at: String,
    pub authorized: bool,
    pub has_active_subscription: bool,
    pub tier: Tier,
}

// Stripe Connect

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeConnectStatusResponse {
    pub account_id: Option<String>,
    pub onboarding_complete: bool,
    pub charges_enabled: bool,
}

// Delegated moderation

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformModerationRequest {
    pub user_email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::moderate::ModerationModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_labels: Option<Vec<crate::moderate::ModerationLabel>>,
    #[serde(default)]
    pub include_context: bool,
}

// Consent / Authorization

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizePlatformPageData {
    pub platform_id: String,
    pub platform_name: String,
    pub logo_url: Option<String>,
    pub description: Option<String>,
    pub theme_primary_color: Option<String>,
    pub theme_accent_color: Option<String>,
    pub redirect_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizePlatformResponse {
    pub redirect_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmAuthorizationRequest {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmAuthorizationResponse {
    pub user_id: String,
    pub email: String,
}

// Dashboard: authorized apps

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizedAppInfo {
    pub platform_id: String,
    pub platform_name: String,
    pub logo_url: Option<String>,
    pub authorized_at: Option<String>,
}
