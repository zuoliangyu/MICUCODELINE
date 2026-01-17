use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceConfig {
    pub api_key: String,
    pub user_id: String,
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

    pub fn api_url() -> String {
        "https://www.openclaudecode.cn/api/user/self".to_string()
    }
}
