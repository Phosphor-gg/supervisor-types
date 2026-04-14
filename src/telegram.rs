use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use crate::moderate::{ModerationLabel, ModerationModel};
use crate::discord::LinkFilterMode;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum TelegramModerationAction {
    Delete,
    Warn,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelegramChatConfig {
    #[serde(default = "default_true")]
    pub is_active: bool,
    pub enabled_labels: HashSet<ModerationLabel>,
    pub actions: HashSet<TelegramModerationAction>,
    #[serde(default = "default_model")]
    pub model: ModerationModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts_chat_id: Option<String>,
    #[serde(default)]
    pub enable_context: bool,
    #[serde(default = "default_context_count")]
    pub context_history_count: i32,
    #[serde(default = "default_true")]
    pub enable_link_filter: bool,
    #[serde(default = "default_link_filter_mode")]
    pub link_filter_mode: LinkFilterMode,
    #[serde(default)]
    pub custom_link_filters: Vec<String>,
    #[serde(default = "default_true")]
    pub block_telegram_invite_links: bool,
}

fn default_true() -> bool {
    true
}

fn default_model() -> ModerationModel {
    ModerationModel::Auto
}

fn default_context_count() -> i32 {
    5
}

fn default_link_filter_mode() -> LinkFilterMode {
    LinkFilterMode::Blacklist
}

impl Default for TelegramChatConfig {
    fn default() -> Self {
        Self {
            is_active: true,
            enabled_labels: HashSet::from([
                ModerationLabel::H,
                ModerationLabel::HR,
                ModerationLabel::I,
                ModerationLabel::S,
                ModerationLabel::SU,
                ModerationLabel::S2,
                ModerationLabel::SE,
                ModerationLabel::V,
                ModerationLabel::SH,
                ModerationLabel::SP,
                ModerationLabel::PM,
                ModerationLabel::SI,
                ModerationLabel::IL,
            ]),
            actions: HashSet::from([TelegramModerationAction::Delete]),
            model: ModerationModel::Auto,
            alerts_chat_id: None,
            enable_context: false,
            context_history_count: 5,
            enable_link_filter: true,
            link_filter_mode: LinkFilterMode::Blacklist,
            custom_link_filters: Vec::new(),
            block_telegram_invite_links: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelegramChatContext {
    pub chat_config: TelegramChatConfig,
    pub admin_user_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramModerateRequest {
    pub chat_info: TelegramChatInfo,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelegramChatInfo {
    pub id: String,
    pub title: String,
    pub admins: HashMap<String, TelegramUserInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelegramUserInfo {
    pub user_id: String,
    pub username: Option<String>,
    pub is_owner: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramAuthData {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub photo_url: Option<String>,
    pub auth_date: i64,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramLinkStatus {
    pub linked: bool,
    pub telegram_id: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramIntegrationData {
    pub chat_id: String,
    pub config: TelegramChatConfig,
    pub admin_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncAdminsRequest {
    pub chat_id: String,
    pub chat_title: Option<String>,
    pub admins: Vec<TelegramAdminEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramAdminEntry {
    pub user_id: String,
    pub username: Option<String>,
    pub is_owner: bool,
}
