use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGuildsRequest {
    pub user_discord_id: String,
    pub guild_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildInfo {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGuildsResponse {
    pub guilds: Vec<GuildInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub id: String,
    pub name: String,
    pub channel_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetChannelsResponse {
    pub channels: Vec<ChannelInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleInfo {
    pub id: String,
    pub name: String,
    pub color: u32,
    pub position: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRolesResponse {
    pub roles: Vec<RoleInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminInfo {
    pub discord_id: String,
    pub username: String,
    pub is_owner: bool,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAdminsResponse {
    pub admins: Vec<AdminInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerInfo {
    pub discord_id: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnerResponse {
    pub owner: OwnerInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckAccountRequest {
    pub guild_id: String,
    pub owner_discord_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckAccountResponse {
    pub has_account: bool,
    #[allow(dead_code)]
    pub has_subscription: bool,
    pub account_tier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerateRequest {
    pub guild_id: String,
    pub owner_discord_id: String,
    pub text: String,
    pub model: Option<String>,
    pub enabled_labels: Option<Vec<String>>,
    pub message_history: Option<Vec<String>>,
    #[serde(default)]
    pub include_context: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerateResponse {
    pub flagged: bool,
    pub labels: Vec<String>,
    pub scores: std::collections::HashMap<String, f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_context: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[allow(dead_code)]
    pub context_labels: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildConfig {
    pub guild_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_icon: Option<String>,
    #[serde(default = "default_moderate_all_channels")]
    pub moderate_all_channels: bool,
    pub moderated_channels: Vec<String>,
    pub enabled_labels: Vec<String>,
    #[serde(default = "default_moderate_all_roles")]
    pub moderate_all_roles: bool,
    #[serde(default = "default_role_filter_mode")]
    pub role_filter_mode: String,
    pub filtered_roles: Vec<String>,
    #[serde(rename = "action_on_flag")]
    pub actions: Vec<ModerationAction>,
    #[serde(rename = "log_channel_id")]
    pub log_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(rename = "alerts_channel_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts_channel: Option<String>,
    #[serde(default = "default_context_history_count")]
    pub context_history_count: i32,
    pub enable_context: bool,
}

fn default_moderate_all_channels() -> bool {
    true
}

fn default_moderate_all_roles() -> bool {
    true
}

fn default_role_filter_mode() -> String {
    "exclude".to_string()
}

fn default_model() -> String {
    "observer".to_string()
}

fn default_context_history_count() -> i32 {
    5
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModerationAction {
    Delete,
    Timeout,
    Warn,
}

impl Default for GuildConfig {
    fn default() -> Self {
        Self {
            guild_id: String::new(),
            guild_name: None,
            guild_icon: None,
            moderate_all_channels: true,
            moderated_channels: Vec::new(),
            enabled_labels: vec![
                "S".to_string(),
                "H".to_string(),
                "V".to_string(),
                "HR".to_string(),
                "SH".to_string(),
                "S3".to_string(),
                "SP".to_string(),
                "SE".to_string(),
            ],
            moderate_all_roles: true,
            role_filter_mode: "exclude".to_string(),
            filtered_roles: Vec::new(),
            actions: vec![ModerationAction::Delete],
            log_channel: None,
            is_active: None,
            model: "observer".to_string(),
            alerts_channel: None,
            context_history_count: 5,
            enable_context: false,
        }
    }
}

impl std::fmt::Display for ModerationAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModerationAction::Delete => write!(f, "delete"),
            ModerationAction::Timeout => write!(f, "timeout"),
            ModerationAction::Warn => write!(f, "warn"),
        }
    }
}
