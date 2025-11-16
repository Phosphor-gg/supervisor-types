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
