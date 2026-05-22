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

    /// 通过 new-api 用户 access token 调用 /api/user/self 获取用户真实余额。
    /// 适用于 API Key 设置了无限额度、billing 接口返回 ∞ 的情况。
    pub fn get_user_self_balance(
        &self,
        access_token: &str,
        user_id: i64,
        quota_per_unit: f64,
    ) -> Result<BalanceData, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/user/self",
            self.config.api_base_url.trim_end_matches('/')
        );

        let response = self
            .agent
            .get(&url)
            .set("Authorization", &format!("Bearer {}", access_token))
            .set("New-Api-User", &user_id.to_string())
            .call()?;

        let resp: UserSelfApiResponse = serde_json::from_str(&response.into_string()?)?;

        if !resp.success {
            return Err(format!("API error: {}", resp.message).into());
        }

        let data = resp.data.ok_or("No data in response")?;
        Ok(BalanceData::from_user_self(&data, quota_per_unit))
    }

    pub fn get_balance(&self) -> Result<BalanceData, Box<dyn std::error::Error>> {
        if !self.config.enabled || self.config.api_key.is_empty() {
            return Err("API not configured".into());
        }

        let subscription = self.get_subscription()?;
        let usage = self.get_usage()?;

        Ok(BalanceData::from_billing(&subscription, &usage))
    }
}
