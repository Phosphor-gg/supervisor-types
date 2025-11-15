use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildsInfoRequest {
    pub discord_id: String,
    pub admin_guild_ids: Vec<String>,
    pub guild_admin_ids: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildInfoRequest {
    pub discord_id: String,
    pub guild_admin_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildInfo {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub icon: Option<String>,
    pub channels: HashMap<String, ChannelInfo>,
    pub roles: HashMap<String, RoleInfo>,
    pub admins: HashMap<String, UserInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildsInfoResponse {
    pub guilds: Vec<GuildInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChannelInfo {
    pub id: String,
    pub name: String,
    pub channel_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetChannelsResponse {
    pub channels: Vec<ChannelInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AdminInfo {
    pub user_info: UserInfo,
    pub subscription_tier: String,
    pub has_account: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AdminConfig {
    pub is_opted_in: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AdminData {
    pub admin_info: AdminInfo,
    pub admin_config: AdminConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiscordStatusResponse {
    pub discord_id: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user UUID
    pub email: String,
    pub discord_id: Option<String>,
    pub username: Option<String>,    // Discord username
    pub global_name: Option<String>, // Discord display name
    pub avatar: Option<String>,      // Discord avatar hash
    pub exp: usize,                  // expiration time
    pub iat: usize,                  // issued at
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserInfo {
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
pub struct GetOwnerResponse {
    pub owner: UserInfo,
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
pub struct DiscordModerateRequest {
    pub guild_info: GuildInfo,
    pub text: String,
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
pub struct DiscordDataResponse {
    pub discord_data: HashMap<String, DiscordData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscordData {
    pub guild_info: GuildInfo,
    pub guild_config: GuildConfig,
    pub admin_data: HashMap<String, AdminData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuildConfig {
    #[serde(default = "default_moderate_all_channels")]
    pub moderate_all_channels: bool,
    pub moderated_channels: HashMap<String, ChannelInfo>,
    pub enabled_labels: HashSet<ModerationLabel>,
    #[serde(default = "default_moderate_all_roles")]
    pub moderate_all_roles: bool,
    #[serde(default = "default_role_filter_mode")]
    pub role_filter_mode: String,
    pub filtered_roles: HashMap<String, RoleInfo>,
    pub actions: HashSet<ModerationAction>,
    #[serde(default = "default_is_active")]
    pub is_active: bool,
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

fn default_is_active() -> bool {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ModerationAction {
    Delete,
    Timeout,
    Warn,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModerationLabel {
    S,
    H,
    V,
    HR,
    SH,
    S3,
    SP,
    SE,
    T,
}

impl Default for GuildConfig {
    fn default() -> Self {
        Self {
            moderate_all_channels: true,
            moderated_channels: HashMap::new(),
            enabled_labels: HashSet::from([
                ModerationLabel::S,
                ModerationLabel::H,
                ModerationLabel::V,
                ModerationLabel::HR,
                ModerationLabel::SH,
                ModerationLabel::S3,
                ModerationLabel::SP,
                ModerationLabel::SE,
            ]),
            moderate_all_roles: true,
            role_filter_mode: "exclude".to_string(),
            filtered_roles: HashMap::new(),
            actions: HashSet::from([ModerationAction::Delete]),
            is_active: true,
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
