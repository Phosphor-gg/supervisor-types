use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct UserCountResponse {
    pub count: u64,
}