use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub discord_id: Option<String>,
    pub username: Option<String>,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub exp: usize, // expiration time
    pub iat: usize, // issued at
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackQuery {
    pub code: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletionInitiateResponse {
    pub auth_url: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletionCallbackQuery {
    pub code: String,
    pub state: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkDiscordQuery {
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthInitiateResponse {
    pub auth_url: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordCallbackQuery {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub id: String,
    pub name: String,
    pub key_preview: String,
    pub status: String,
    pub created_at: String,
    pub last_used: Option<String>,
    pub usage_count: i64,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyResponse {
    pub id: String,
    pub name: String,
    pub full_key: String,
    pub key_preview: String,
    pub status: String,
    pub created_at: String,
    pub warning: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
}