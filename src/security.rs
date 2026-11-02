//! Discord security suite: anti-nuke, anti-raid, heat automod, quarantine,
//! lockdown, verification, backups, cases and notes.
//!
//! Every setting lives in [`SecurityConfig`], stored per guild as one JSON
//! value. Fields marked *Premium* only take effect for guilds whose billing
//! user has the `advanced-security` entitlement; [`SecurityConfig::effective`]
//! is the single place that applies that gate, so the dashboard keeps the
//! admin's raw intent and the bot never has to know which fields are paid.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Punishment {
    /// Record it in the security log and do nothing else.
    Log,
    Timeout,
    Kick,
    Ban,
    Quarantine,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct SecurityConfig {
    /// Channel for security events. Falls back to the moderation alerts channel.
    pub log_channel_id: Option<String>,
    /// Role that strips a member's access. Created by `/setup` when absent.
    pub quarantine_role_id: Option<String>,
    /// Users immune to every security system and allowed every security
    /// command. The owner is always included.
    pub trusted_user_ids: Vec<String>,
    /// Roles immune to heat, join gate and the word/link filters, but not to
    /// anti-nuke: a compromised moderator is exactly what anti-nuke is for.
    pub immune_role_ids: Vec<String>,
    /// Roles allowed to run moderation commands besides members holding the
    /// matching Discord permission.
    pub moderator_role_ids: Vec<String>,
    pub antinuke: AntiNukeConfig,
    pub join_gate: JoinGateConfig,
    pub join_raid: JoinRaidConfig,
    pub heat: HeatConfig,
    pub auto_lockdown: AutoLockdownConfig,
    pub verification: VerificationConfig,
    pub warns: WarnsConfig,
    pub backups: BackupsConfig,
    pub mod_settings: ModSettings,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum LogKind {
    AntiNuke,
    JoinGate,
    JoinRaid,
    Automod,
    Verification,
    Moderation,
    Lockdown,
    Reports,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum NukeAction {
    ChannelCreate,
    ChannelDelete,
    RoleCreate,
    RoleDelete,
    Ban,
    Kick,
    WebhookCreate,
    WebhookDelete,
    /// Premium: member prune.
    Prune,
}

impl NukeAction {
    pub const ALL: [NukeAction; 9] = [
        Self::ChannelCreate,
        Self::ChannelDelete,
        Self::RoleCreate,
        Self::RoleDelete,
        Self::Ban,
        Self::Kick,
        Self::WebhookCreate,
        Self::WebhookDelete,
        Self::Prune,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::ChannelCreate => "Channel creation",
            Self::ChannelDelete => "Channel deletion",
            Self::RoleCreate => "Role creation",
            Self::RoleDelete => "Role deletion",
            Self::Ban => "Bans",
            Self::Kick => "Kicks",
            Self::WebhookCreate => "Webhook creation",
            Self::WebhookDelete => "Webhook deletion",
            Self::Prune => "Member prune",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct RateLimit {
    pub per_minute: u32,
    pub per_hour: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct AntiNukeConfig {
    pub enabled: bool,
    pub limits: HashMap<NukeAction, RateLimit>,
    pub punishment: Punishment,
    /// Punish anyone who removes the quarantine role from a quarantined
    /// member, or adds dangerous permissions to the quarantine role.
    pub quarantine_hold: bool,
    /// Revert and punish changes to the vanity URL.
    pub vanity_protection: bool,
    /// Revert dangerous permissions added to @everyone or roles held by
    /// most of the server.
    pub monitor_public_roles: bool,
    /// Revert dangerous permissions granted to @everyone in a channel.
    pub monitor_channel_permissions: bool,
    /// Bots may only be added by trusted users.
    pub bot_addition_filter: bool,
    /// Premium: dangerous permissions added to any role are reverted and
    /// punished unless the actor is trusted.
    pub strict_mode: bool,
    /// Premium: count member prunes against the limits.
    pub prune_detection: bool,
    pub panic: PanicConfig,
}

/// Premium. Runs when anti-nuke punishes someone: locks the server, then
/// restores it from the latest backup.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct PanicConfig {
    pub enabled: bool,
    pub lock_server: bool,
    pub auto_restore: bool,
    /// Minutes after which the lock lifts on its own; 0 keeps it until
    /// `/unlock`.
    pub auto_unlock_minutes: u32,
    /// Categories left alone by the lock, e.g. tickets.
    pub whitelisted_category_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct GateFilter {
    pub enabled: bool,
    pub action: Punishment,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct JoinGateConfig {
    pub enabled: bool,
    pub no_avatar: GateFilter,
    pub account_age: GateFilter,
    pub min_account_age_secs: u64,
    pub unverified_bots: GateFilter,
    /// Discord invite links in the username or display name.
    pub advertising_names: GateFilter,
    /// Usernames matching `username_filter` (case-insensitive, `*` wildcard).
    pub username_filter: GateFilter,
    pub username_words: Vec<String>,
    /// Usernames the username model flags.
    pub suspicious_names: GateFilter,
    /// Tell the member why they were removed before acting.
    pub dm_reason: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RaidAccountType {
    All,
    /// Only accounts younger than `young_account_days` or without an avatar.
    Suspicious,
}

/// Premium. Detects a surge of joins and acts on every account in it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct JoinRaidConfig {
    pub enabled: bool,
    pub joins: u32,
    pub window_secs: u64,
    pub account_type: RaidAccountType,
    pub young_account_days: u32,
    pub action: Punishment,
    /// Lock joins while the raid lasts.
    pub lock_joins: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct HeatConfig {
    pub enabled: bool,
    /// Heat added per factor, in percent of `max_heat`.
    pub weights: HeatWeights,
    /// Premium: heat at which a member is punished.
    pub max_heat: f64,
    /// Premium: heat lost per second.
    pub decay_per_sec: f64,
    pub first_timeout_secs: u64,
    pub max_timeout_secs: u64,
    /// Premium: each repeat offence multiplies the previous timeout.
    pub multiplier: f64,
    /// Delete the messages that built up the heat.
    pub purge_messages: bool,
    pub monitor_webhooks: bool,
    pub panic: HeatPanicConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct HeatWeights {
    pub message: f64,
    pub repeated: f64,
    pub per_emoji: f64,
    pub per_100_chars: f64,
    pub per_newline: f64,
    pub per_mention: f64,
    pub everyone_mention: f64,
    pub per_attachment: f64,
    pub invite_link: f64,
    pub malicious_link: f64,
    pub blacklisted: f64,
}

/// Premium. When several members max out their heat within the window,
/// every further offence by a recent joiner is an immediate timeout.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct HeatPanicConfig {
    pub enabled: bool,
    pub raiders: u32,
    pub window_secs: u64,
    pub duration_secs: u64,
}

/// Locks every channel when mentions from non-immune members flood in.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct AutoLockdownConfig {
    pub enabled: bool,
    pub mentions: u32,
    pub window_secs: u64,
    /// Minutes until the lock lifts on its own; 0 keeps it until `/unlock`.
    pub duration_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
/// Verification is always the Cloudflare check on supervisor.gg; these only
/// decide who has to pass it and what happens if they do not.
pub struct VerificationConfig {
    /// Verify only suspicious accounts (young or no avatar); others get the
    /// role on join.
    pub suspicious_only: bool,
    /// Seconds to finish before `fail_action`; 0 waits forever.
    pub time_limit_secs: u64,
    pub fail_action: Punishment,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct WarnsConfig {
    /// Actions taken when a member's active warn count reaches `count`.
    pub thresholds: Vec<WarnThreshold>,
    /// Days a warn stays active; 0 keeps it forever.
    pub expire_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WarnThreshold {
    pub count: u32,
    pub action: Punishment,
    #[serde(default)]
    pub timeout_secs: u64,
}

/// Premium. Automatic snapshots of channels, roles, permissions and settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct BackupsConfig {
    pub auto: bool,
    pub interval_hours: u32,
    pub keep: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ModSettings {
    /// DM the member when they are banned, kicked, timed out or warned.
    pub dm_on_action: bool,
    pub default_timeout_secs: u64,
    /// Days of messages deleted with a ban.
    pub ban_delete_days: u8,
    /// Hide which moderator acted in DMs.
    pub anonymous: bool,
    /// Text appended to ban DMs, e.g. an appeal link.
    pub ban_message: Option<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            log_channel_id: None,
            quarantine_role_id: None,
            trusted_user_ids: Vec::new(),
            immune_role_ids: Vec::new(),
            moderator_role_ids: Vec::new(),
            antinuke: Default::default(),
            join_gate: Default::default(),
            join_raid: Default::default(),
            heat: Default::default(),
            auto_lockdown: Default::default(),
            verification: Default::default(),
            warns: Default::default(),
            backups: Default::default(),
            mod_settings: Default::default(),
        }
    }
}

impl Default for AntiNukeConfig {
    fn default() -> Self {
        let limit = |per_minute, per_hour| RateLimit { per_minute, per_hour };
        Self {
            // Off until an admin opts in: punishing staff is not something a
            // server should discover by surprise.
            enabled: false,
            limits: HashMap::from([
                (NukeAction::ChannelCreate, limit(5, 20)),
                (NukeAction::ChannelDelete, limit(3, 10)),
                (NukeAction::RoleCreate, limit(5, 20)),
                (NukeAction::RoleDelete, limit(3, 10)),
                (NukeAction::Ban, limit(5, 30)),
                (NukeAction::Kick, limit(5, 30)),
                (NukeAction::WebhookCreate, limit(3, 10)),
                (NukeAction::WebhookDelete, limit(3, 10)),
                (NukeAction::Prune, limit(1, 2)),
            ]),
            punishment: Punishment::Quarantine,
            quarantine_hold: true,
            vanity_protection: true,
            monitor_public_roles: true,
            monitor_channel_permissions: true,
            bot_addition_filter: true,
            strict_mode: false,
            prune_detection: true,
            panic: Default::default(),
        }
    }
}

impl Default for PanicConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            lock_server: true,
            auto_restore: true,
            auto_unlock_minutes: 30,
            whitelisted_category_ids: Vec::new(),
        }
    }
}

impl Default for GateFilter {
    fn default() -> Self {
        Self { enabled: false, action: Punishment::Kick }
    }
}

impl Default for JoinGateConfig {
    fn default() -> Self {
        let on = |action| GateFilter { enabled: true, action };
        Self {
            enabled: false,
            no_avatar: GateFilter::default(),
            account_age: on(Punishment::Kick),
            min_account_age_secs: 3 * 24 * 60 * 60,
            unverified_bots: on(Punishment::Kick),
            advertising_names: on(Punishment::Kick),
            username_filter: GateFilter::default(),
            username_words: Vec::new(),
            suspicious_names: GateFilter { enabled: false, action: Punishment::Log },
            dm_reason: true,
        }
    }
}

impl Default for JoinRaidConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            joins: 10,
            window_secs: 60,
            account_type: RaidAccountType::All,
            young_account_days: 7,
            action: Punishment::Kick,
            lock_joins: true,
        }
    }
}

impl Default for HeatConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            weights: Default::default(),
            max_heat: 100.0,
            decay_per_sec: 2.0,
            first_timeout_secs: 60,
            max_timeout_secs: 60 * 60,
            multiplier: 2.0,
            purge_messages: true,
            monitor_webhooks: true,
            panic: Default::default(),
        }
    }
}

impl Default for HeatWeights {
    fn default() -> Self {
        // Calibrated so ordinary chat never builds up: a message is 8%, and
        // decay at 2/s clears it in 4 seconds. Ten quick messages, a wall of
        // emojis or a handful of mentions reach 100 well inside a few seconds.
        Self {
            message: 8.0,
            repeated: 20.0,
            per_emoji: 3.0,
            per_100_chars: 4.0,
            per_newline: 2.0,
            per_mention: 12.0,
            everyone_mention: 60.0,
            per_attachment: 10.0,
            invite_link: 50.0,
            malicious_link: 100.0,
            blacklisted: 100.0,
        }
    }
}

impl Default for HeatPanicConfig {
    fn default() -> Self {
        Self { enabled: false, raiders: 3, window_secs: 600, duration_secs: 1800 }
    }
}

impl Default for AutoLockdownConfig {
    fn default() -> Self {
        Self { enabled: false, mentions: 50, window_secs: 30, duration_minutes: 15 }
    }
}

impl Default for VerificationConfig {
    fn default() -> Self {
        Self {
            suspicious_only: false,
            time_limit_secs: 0,
            fail_action: Punishment::Kick,
        }
    }
}

impl Default for WarnsConfig {
    fn default() -> Self {
        Self { thresholds: Vec::new(), expire_days: 30 }
    }
}

impl Default for BackupsConfig {
    fn default() -> Self {
        Self { auto: true, interval_hours: 3, keep: 10 }
    }
}

impl Default for ModSettings {
    fn default() -> Self {
        Self {
            dm_on_action: true,
            default_timeout_secs: 60 * 60,
            ban_delete_days: 0,
            anonymous: false,
            ban_message: None,
        }
    }
}

impl SecurityConfig {
    /// The settings the bot runs with. Without `advanced-security` every
    /// Premium field is replaced by its free behaviour, which is either off or
    /// the fixed default, so a lapsed subscription keeps the saved intent and
    /// resumes it on renewal.
    pub fn effective(&self, advanced: bool) -> SecurityConfig {
        let mut c = self.clone();
        if advanced {
            return c;
        }
        let free = SecurityConfig::default();
        c.antinuke.strict_mode = false;
        c.antinuke.prune_detection = false;
        c.antinuke.panic.enabled = false;
        c.join_raid.enabled = false;
        c.heat.weights = free.heat.weights;
        c.heat.max_heat = free.heat.max_heat;
        c.heat.decay_per_sec = free.heat.decay_per_sec;
        c.heat.multiplier = 1.0;
        c.heat.panic.enabled = false;
        c.backups.auto = false;
        c
    }
}

/// Whether a guild runs the Premium tier, for the bot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityTierResponse {
    pub advanced: bool,
}

// ── Cases ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CaseKind {
    Ban,
    Unban,
    Kick,
    Timeout,
    Untimeout,
    Warn,
    Quarantine,
    Unquarantine,
    Automod,
    JoinGate,
    JoinRaid,
    Nuke,
}

impl CaseKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Ban => "Ban",
            Self::Unban => "Unban",
            Self::Kick => "Kick",
            Self::Timeout => "Timeout",
            Self::Untimeout => "Timeout removed",
            Self::Warn => "Warn",
            Self::Quarantine => "Quarantine",
            Self::Unquarantine => "Unquarantine",
            Self::Automod => "Automod",
            Self::JoinGate => "Join gate",
            Self::JoinRaid => "Join raid",
            Self::Nuke => "Nuke attempt",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewCase {
    pub kind: CaseKind,
    pub target_id: String,
    /// Moderator, or the bot's own id for automatic actions.
    pub moderator_id: String,
    pub reason: Option<String>,
    pub duration_secs: Option<i64>,
    /// Kind-specific detail, e.g. what a nuke attempt deleted.
    #[serde(default)]
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Case {
    /// Per-guild sequence number shown to moderators.
    pub number: i64,
    pub kind: CaseKind,
    pub target_id: String,
    pub moderator_id: String,
    pub reason: Option<String>,
    pub duration_secs: Option<i64>,
    pub active: bool,
    #[serde(default)]
    pub details: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CaseQuery {
    pub target_id: Option<String>,
    pub moderator_id: Option<String>,
    pub kind: Option<CaseKind>,
    pub active_only: bool,
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CaseUpdate {
    pub reason: Option<String>,
    pub active: Option<bool>,
}

// ── Notes ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewNote {
    pub user_id: String,
    pub author_id: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: i64,
    pub user_id: String,
    pub author_id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

// ── Backups ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackupSummary {
    pub id: String,
    pub automatic: bool,
    pub channels: u32,
    pub roles: u32,
    pub created_at: DateTime<Utc>,
}

/// A snapshot of everything a backup restores. Messages and members' role
/// assignments are not included, as with Discord's own templates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuildSnapshot {
    pub name: String,
    pub verification_level: u8,
    pub explicit_content_filter: u8,
    pub default_notifications: u8,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: u32,
    pub system_channel_id: Option<String>,
    pub roles: Vec<RoleSnapshot>,
    pub channels: Vec<ChannelSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleSnapshot {
    pub id: String,
    pub name: String,
    pub color: u32,
    pub hoist: bool,
    pub mentionable: bool,
    pub permissions: u64,
    pub position: u16,
    pub managed: bool,
    pub everyone: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelSnapshot {
    pub id: String,
    pub name: String,
    pub kind: u8,
    pub position: u16,
    pub parent_id: Option<String>,
    pub topic: Option<String>,
    pub nsfw: bool,
    pub rate_limit_per_user: u16,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u32>,
    pub overwrites: Vec<OverwriteSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverwriteSnapshot {
    /// Role or member id.
    pub id: String,
    pub role: bool,
    pub allow: u64,
    pub deny: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewBackup {
    pub automatic: bool,
    pub snapshot: GuildSnapshot,
    /// Oldest automatic backups beyond this many are deleted.
    pub keep: u32,
}
