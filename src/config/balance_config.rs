use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceConfig {
    pub api_key: String,
    #[serde(default)]
    pub user_id: Option<String>, // 保留向后兼容，但不再使用
    /// new-api 用户 access token（从个人中心获取），用于查询用户账户余额
    /// 当 API Key 设置为无限额度时，billing 接口会返回 ∞，配置此项可改为显示用户实际余额
    #[serde(default)]
    pub access_token: Option<String>,
    /// new-api 用户 ID（从个人中心获取），配合 access_token 使用
    #[serde(default)]
    pub new_api_user_id: Option<i64>,
    /// 汇率（USD 到 CNY），默认 7.3，与 new-api 服务器配置保持一致
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    /// 每美元对应的额度单位数，默认 500000（new-api 默认值）
    #[serde(default)]
    pub quota_per_unit: Option<f64>,
}

impl BalanceConfig {
    pub fn config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".claude")
            .join("micucodeline")
            .join("balance_config.json")
    }

    pub fn load() -> Option<Self> {
        let path = Self::config_path();
        if !path.exists() {
            return None;
        }

        let content = fs::read_to_string(&path).ok()?;
        serde_json::from_str(&content).ok()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::config_path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }
}
