use super::{ApiConfig, BalanceData, UserData, UserResponse};
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

    pub fn get_user_data(&self) -> Result<UserData, Box<dyn std::error::Error>> {
        if !self.config.enabled || self.config.api_key.is_empty() {
            return Err("API not configured".into());
        }

        let mut request = self
            .agent
            .get(&self.config.api_url)
            .set("Authorization", &format!("Bearer {}", self.config.api_key));

        if let Some(ref user_id) = self.config.user_id {
            request = request.set("New-Api-User", user_id);
        }

        let response = request.call();

        let response = match response {
            Ok(r) => r,
            Err(ureq::Error::Status(code, resp)) => {
                // 尝试解析错误响应中的 message
                if let Ok(text) = resp.into_string() {
                    if let Ok(err_resp) = serde_json::from_str::<UserResponse>(&text) {
                        let msg = err_resp.message.unwrap_or_else(|| format!("HTTP {}", code));
                        return Err(msg.into());
                    }
                }
                return Err(format!("HTTP {}", code).into());
            }
            Err(e) => return Err(e.into()),
        };

        let resp: UserResponse = serde_json::from_str(&response.into_string()?)?;

        if !resp.success {
            return Err(resp
                .message
                .unwrap_or_else(|| "Unknown error".to_string())
                .into());
        }

        resp.data.ok_or_else(|| "No data in response".into())
    }

    pub fn get_balance(&self) -> Result<BalanceData, Box<dyn std::error::Error>> {
        let user_data = self.get_user_data()?;
        Ok(BalanceData::from_user_data(&user_data))
    }
}
