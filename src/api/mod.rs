pub mod cache;
pub mod client;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub enabled: bool,
    pub api_key: String,
    pub api_base_url: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_key: String::new(),
            api_base_url: String::new(),
        }
    }
}

/// /api/user/self 响应中的用户数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSelfData {
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

/// /api/user/self 完整响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSelfApiResponse {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub message: String,
    pub data: Option<UserSelfData>,
}

/// /v1/dashboard/billing/subscription 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionResponse {
    #[serde(default)]
    pub object: String,
    #[serde(default)]
    pub has_payment_method: bool,
    #[serde(default)]
    pub hard_limit_usd: f64,
    #[serde(default)]
    pub soft_limit_usd: f64,
    #[serde(default)]
    pub system_hard_limit_usd: f64,
    #[serde(default)]
    pub access_until: i64,
}

/// /v1/dashboard/billing/usage 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageResponse {
    #[serde(default)]
    pub object: String,
    #[serde(default)]
    pub total_usage: f64, // 单位: 分 (cents)，实际值需除以 100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceData {
    pub display_name: String,
    pub group: String,
    pub balance: f64,
    pub used: f64,
    pub total: f64,
    pub is_unlimited: bool,
}

const UNLIMITED_THRESHOLD: i64 = 100_000_000_000;
const UNLIMITED_THRESHOLD_USD: f64 = 100_000_000.0;
const QUOTA_UNIT: f64 = 500000.0;

impl BalanceData {
    /// 从 /api/user/self 响应构造（保留 display_name 和 group）
    pub fn from_user_data(data: &UserSelfData) -> Self {
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
            total: balance + used,
            is_unlimited,
        }
    }

    /// 从 /api/user/self 响应中的原始额度计算余额（带自定义汇率）
    pub fn from_user_self(data: &UserSelfData, quota_per_unit: f64, exchange_rate: f64) -> Self {
        let remain = data.quota as f64;
        let used = data.used_quota as f64;
        let total_raw = remain + used;

        let balance = remain / quota_per_unit * exchange_rate;
        let used_display = used / quota_per_unit * exchange_rate;
        let total = total_raw / quota_per_unit * exchange_rate;

        Self {
            display_name: data.display_name.clone(),
            group: data.group.clone(),
            balance,
            used: used_display,
            total,
            is_unlimited: false,
        }
    }

    /// 从 subscription 和 usage 两个接口响应计算余额
    pub fn from_billing(subscription: &SubscriptionResponse, usage: &UsageResponse) -> Self {
        let total = subscription.hard_limit_usd;
        let used = usage.total_usage / 100.0;
        let is_unlimited = total >= UNLIMITED_THRESHOLD_USD;

        let balance = if is_unlimited { 0.0 } else { total - used };

        Self {
            display_name: String::new(),
            group: String::new(),
            balance,
            used,
            total,
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
