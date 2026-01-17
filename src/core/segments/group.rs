use super::{Segment, SegmentData};
use crate::api::{cache, client::ApiClient, ApiConfig};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct GroupSegment;

impl GroupSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for GroupSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        self.try_collect().ok().flatten()
    }

    fn id(&self) -> SegmentId {
        SegmentId::Group
    }
}

impl GroupSegment {
    fn try_collect(&self) -> Result<Option<SegmentData>, Box<dyn std::error::Error>> {
        // 如果没有 BALANCE_API_KEY，直接返回 None
        let api_key = match std::env::var("BALANCE_API_KEY").ok() {
            Some(key) => key,
            None => return Ok(None),
        };

        let api_url = std::env::var("BALANCE_API_URL")
            .unwrap_or_else(|_| "https://www.openclaudecode.cn/api/user/self".to_string());

        let config = ApiConfig {
            enabled: true,
            api_key,
            api_url,
            user_id: std::env::var("BALANCE_API_USER").ok(),
        };

        let cache_key = cache::cache_key(&config);
        if let Some(balance) = cache::get_in_memory_balance(&cache_key) {
            if !balance.group.is_empty() {
                return Ok(Some(SegmentData {
                    primary: balance.group,
                    secondary: String::new(),
                    metadata: HashMap::new(),
                }));
            }
        }

        let client = ApiClient::new(config);

        // 先尝试调用 API 获取最新数据
        if let Ok(balance) = client.get_balance() {
            cache::set_in_memory_balance(&cache_key, &balance);
            let _ = cache::save_cached_balance(&cache_key, &balance);
            if !balance.group.is_empty() {
                return Ok(Some(SegmentData {
                    primary: balance.group,
                    secondary: String::new(),
                    metadata: HashMap::new(),
                }));
            }
        }

        // API 失败时，使用缓存作为 fallback
        let (cached, _) = cache::get_cached_balance(&cache_key);
        if let Some(balance) = cached {
            cache::set_in_memory_balance(&cache_key, &balance);
            if !balance.group.is_empty() {
                return Ok(Some(SegmentData {
                    primary: balance.group,
                    secondary: String::new(),
                    metadata: HashMap::new(),
                }));
            }
        }

        Ok(None)
    }
}
