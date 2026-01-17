pub mod cache;
pub mod client;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub enabled: bool,
    pub api_key: String,
    pub api_url: String,
    pub user_id: Option<String>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_key: String::new(),
            api_url: "https://www.openclaudecode.cn/api/user/self".to_string(),
            user_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub success: bool,
    pub data: Option<UserData>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserData {
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub group: String,
    #[serde(default)]
    pub quota: i64,
    #[serde(default)]
    pub used_quota: i64,
    #[serde(default)]
    pub request_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceData {
    pub display_name: String,
    pub group: String,
    pub balance: f64,
    pub used: f64,
    pub is_unlimited: bool,
}

const UNLIMITED_THRESHOLD: i64 = 100_000_000_000;
const QUOTA_UNIT: f64 = 500000.0;

impl BalanceData {
    pub fn from_user_data(data: &UserData) -> Self {
        let is_unlimited = data.quota >= UNLIMITED_THRESHOLD;

        let balance = if is_unlimited {
            0.0
        } else {
            data.quota as f64 / QUOTA_UNIT
        };

        let used = data.used_quota as f64 / QUOTA_UNIT;

        Self {
            display_name: data.display_name.clone(),
            group: data.group.clone(),
            balance,
            used,
            is_unlimited,
        }
    }

    pub fn format_display(&self) -> String {
        if self.is_unlimited {
            "∞".to_string()
        } else {
            format!("¥{:.2}", self.balance)
        }
    }
}
