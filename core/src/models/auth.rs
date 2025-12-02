use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserInfo {
    pub user_id: String,
    pub api_key: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl UserInfo {
    pub fn system() -> Self {
        std::env::var("SYSTEM_USER_ID")
            .map(|user_id| Self {
                user_id,
                ..Default::default()
            })
            .unwrap_or_default()
    }
}

impl Default for UserInfo {
    fn default() -> Self {
        Self {
            user_id: "system".to_string(),
            api_key: None,
            email: None,
            first_name: None,
            last_name: None,
        }
    }
}
