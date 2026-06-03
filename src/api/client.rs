use super::{ApiConfig, BalanceData, SubscriptionResponse, UsageResponse, UserSelfApiResponse};
use std::time::Duration;

const TIMEOUT_SECS: u64 = 5;

pub struct ApiClient {
    config: ApiConfig,
    agent: ureq::Agent,
}

impl ApiClient {
    pub fn new(config: ApiConfig) -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(TIMEOUT_SECS))
            .build();
        Self { config, agent }
    }

    /// 用环境里的 Base URL + 当前 API Key 调用 /api/user/self 获取用户真实余额。
    /// 这是 new-api 中转站最常用的余额查询方式，只需 API Key 即可。
    pub fn get_user_self(
        &self,
        quota_per_unit: f64,
    ) -> Result<BalanceData, Box<dyn std::error::Error>> {
        if !self.config.enabled || self.config.api_key.is_empty() {
            return Err("API not configured".into());
        }

        let url = format!(
            "{}/api/user/self",
            self.config.api_base_url.trim_end_matches('/')
        );

        let response = self
            .agent
            .get(&url)
            .set("Authorization", &format!("Bearer {}", self.config.api_key))
            .call()?;

        let resp: UserSelfApiResponse = serde_json::from_str(&response.into_string()?)?;

        if !resp.success {
            return Err(format!("API error: {}", resp.message).into());
        }

        let data = resp.data.ok_or("No data in response")?;
        Ok(BalanceData::from_user_self(&data, quota_per_unit))
    }

    fn get_subscription(&self) -> Result<SubscriptionResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/dashboard/billing/subscription",
            self.config.api_base_url.trim_end_matches('/')
        );

        let response = self
            .agent
            .get(&url)
            .set("Authorization", &format!("Bearer {}", self.config.api_key))
            .call()?;

        let resp: SubscriptionResponse = serde_json::from_str(&response.into_string()?)?;
        Ok(resp)
    }

    fn get_usage(&self) -> Result<UsageResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/dashboard/billing/usage",
            self.config.api_base_url.trim_end_matches('/')
        );

        let response = self
            .agent
            .get(&url)
            .set("Authorization", &format!("Bearer {}", self.config.api_key))
            .call()?;

        let resp: UsageResponse = serde_json::from_str(&response.into_string()?)?;
        Ok(resp)
    }

    /// 用环境里的 Base URL + API Key 通过标准 billing 接口（subscription + usage）查余额。
    /// 适用于 /api/user/self 需要 session token、API Key 无法直接访问的中转站（如 micuapi）。
    pub fn get_balance(&self) -> Result<BalanceData, Box<dyn std::error::Error>> {
        if !self.config.enabled || self.config.api_key.is_empty() {
            return Err("API not configured".into());
        }

        let subscription = self.get_subscription()?;
        let usage = self.get_usage()?;

        Ok(BalanceData::from_billing(&subscription, &usage))
    }
}
