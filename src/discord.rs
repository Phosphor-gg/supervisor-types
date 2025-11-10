use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetGuildsRequest {
    pub user_discord_id: String,
    pub guild_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct GuildInfo {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetGuildsResponse {
    pub guilds: Vec<GuildInfo>,
}

#[derive(Debug, Serialize)]
pub struct ChannelInfo {
    pub id: String,
    pub name: String,
    pub channel_type: String,
}

#[derive(Debug, Serialize)]
pub struct GetChannelsResponse {
    pub channels: Vec<ChannelInfo>,
}

#[derive(Debug, Serialize)]
pub struct RoleInfo {
    pub id: String,
    pub name: String,
    pub color: u32,
    pub position: u16,
}

#[derive(Debug, Serialize)]
pub struct GetRolesResponse {
    pub roles: Vec<RoleInfo>,
}

#[derive(Debug, Serialize)]
pub struct AdminInfo {
    pub discord_id: String,
    pub username: String,
    pub is_owner: bool,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetAdminsResponse {
    pub admins: Vec<AdminInfo>,
}

#[derive(Debug, Serialize)]
pub struct OwnerInfo {
    pub discord_id: String,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct GetOwnerResponse {
    pub owner: OwnerInfo,
}