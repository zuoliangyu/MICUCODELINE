use super::{Segment, SegmentData};
use crate::api::{cache, client::ApiClient, ApiConfig};
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
        // 从配置文件读取 Balance API 配置
        use crate::config::BalanceConfig;

        let balance_config = match BalanceConfig::load() {
            Some(config) => config,
            None => return Ok(None), // 配置文件不存在，返回 None
        };

        let config = ApiConfig {
            enabled: true,
            api_key: balance_config.api_key,
            api_url: BalanceConfig::api_url(),
            user_id: Some(balance_config.user_id),
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

        // 先尝试调用 API 获取最新数据
        if let Ok(balance) = client.get_balance() {
            cache::set_in_memory_balance(&cache_key, &balance);
            let _ = cache::save_cached_balance(&cache_key, &balance);
            return Ok(Some(SegmentData {
                primary: balance.format_display(),
                secondary: String::new(),
                metadata: HashMap::new(),
            }));
        }

        // API 失败时，使用缓存作为 fallback
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
