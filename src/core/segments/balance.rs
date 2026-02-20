use super::{Segment, SegmentData};
use crate::api::{ApiConfig, cache, client::ApiClient};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct BalanceSegment;

impl BalanceSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for BalanceSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        self.try_collect().ok().flatten()
    }

    fn id(&self) -> SegmentId {
        SegmentId::Balance
    }
}

impl BalanceSegment {
    fn try_collect(&self) -> Result<Option<SegmentData>, Box<dyn std::error::Error>> {
        use crate::config::BalanceConfig;
        use crate::utils::credentials;

        // 自动从 Claude Code settings 读取 API Key 和 Base URL
        let api_key = match credentials::get_api_key() {
            Some(k) => k,
            None => return Ok(None),
        };

        let api_base_url = match credentials::get_api_base_url() {
            Some(u) => u,
            None => return Ok(None),
        };

        let config = ApiConfig {
            enabled: true,
            api_key,
            api_base_url,
        };

        let cache_key = cache::cache_key(&config);
        if let Some(balance) = cache::get_in_memory_balance(&cache_key) {
            return Ok(Some(SegmentData {
                primary: balance.format_display(),
                secondary: String::new(),
                metadata: HashMap::new(),
            }));
        }

        let client = ApiClient::new(config);

        // 1. 优先：如果配置了 access_token + user_id，用它查真实余额
        //    适用于 API Key 设了无限额度的情况
        let balance_config = BalanceConfig::load();
        if let Some(ref bc) = balance_config
            && let (Some(access_token), Some(user_id)) = (&bc.access_token, bc.new_api_user_id)
        {
            let quota_per_unit = bc.quota_per_unit.unwrap_or(500_000.0);
            let exchange_rate = bc.exchange_rate.unwrap_or(7.3);
            if let Ok(balance) =
                client.get_user_self_balance(access_token, user_id, quota_per_unit, exchange_rate)
            {
                cache::set_in_memory_balance(&cache_key, &balance);
                let _ = cache::save_cached_balance(&cache_key, &balance);
                return Ok(Some(SegmentData {
                    primary: balance.format_display(),
                    secondary: String::new(),
                    metadata: HashMap::new(),
                }));
            }
        }

        // 2. 默认：用 API Key 调 /api/user/self（new-api 标准接口）
        if let Ok(balance) = client.get_user_self() {
            cache::set_in_memory_balance(&cache_key, &balance);
            let _ = cache::save_cached_balance(&cache_key, &balance);
            return Ok(Some(SegmentData {
                primary: balance.format_display(),
                secondary: String::new(),
                metadata: HashMap::new(),
            }));
        }

        // 3. 回退：billing 接口
        if let Ok(balance) = client.get_billing_balance() {
            cache::set_in_memory_balance(&cache_key, &balance);
            let _ = cache::save_cached_balance(&cache_key, &balance);
            return Ok(Some(SegmentData {
                primary: balance.format_display(),
                secondary: String::new(),
                metadata: HashMap::new(),
            }));
        }

        // 4. 最后：缓存 fallback
        let (cached, _) = cache::get_cached_balance(&cache_key);
        if let Some(balance) = cached {
            cache::set_in_memory_balance(&cache_key, &balance);
            return Ok(Some(SegmentData {
                primary: balance.format_display(),
                secondary: String::new(),
                metadata: HashMap::new(),
            }));
        }

        Ok(None)
    }
}
