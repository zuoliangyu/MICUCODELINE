use super::{ApiConfig, BalanceData, client::ApiClient};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

const CACHE_FRESH_SECS: u64 = 300; // 5分钟

#[derive(Clone)]
struct InMemoryCacheEntry {
    key: String,
    data: BalanceData,
}

static IN_MEMORY_CACHE: OnceLock<Mutex<Option<InMemoryCacheEntry>>> = OnceLock::new();

fn get_cache_dir() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let cache_dir = home.join(".claude").join("micucodeline").join("cache");
    fs::create_dir_all(&cache_dir).ok()?;
    Some(cache_dir)
}

fn get_cache_file(cache_key: &str) -> Option<PathBuf> {
    Some(get_cache_dir()?.join(format!("balance_{}.json", cache_key)))
}

fn is_cache_fresh(path: &PathBuf) -> bool {
    let Ok(metadata) = fs::metadata(path) else {
        return false;
    };
    let Ok(modified) = metadata.modified() else {
        return false;
    };
    let Ok(elapsed) = SystemTime::now().duration_since(modified) else {
        return false;
    };
    elapsed.as_secs() < CACHE_FRESH_SECS
}

fn hash_key(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

pub fn cache_key(config: &ApiConfig) -> String {
    let mut input = String::new();
    input.push_str(&config.api_base_url);
    input.push('|');
    input.push_str(&config.api_key);
    hash_key(&input)
}

fn in_memory_cache() -> &'static Mutex<Option<InMemoryCacheEntry>> {
    IN_MEMORY_CACHE.get_or_init(|| Mutex::new(None))
}

pub fn get_in_memory_balance(cache_key: &str) -> Option<BalanceData> {
    let cache = in_memory_cache().lock().ok()?;
    match cache.as_ref() {
        Some(entry) if entry.key == cache_key => Some(entry.data.clone()),
        _ => None,
    }
}

pub fn set_in_memory_balance(cache_key: &str, data: &BalanceData) {
    if let Ok(mut cache) = in_memory_cache().lock() {
        *cache = Some(InMemoryCacheEntry {
            key: cache_key.to_string(),
            data: data.clone(),
        });
    }
}

/// 返回 (缓存数据, 是否需要刷新)
pub fn get_cached_balance(cache_key: &str) -> (Option<BalanceData>, bool) {
    let path = match get_cache_file(cache_key) {
        Some(p) => p,
        None => return (None, false),
    };

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return (None, false),
    };

    let data: Option<BalanceData> = serde_json::from_str(&content).ok();
    let needs_refresh = data.is_some() && !is_cache_fresh(&path);

    (data, needs_refresh)
}

pub fn save_cached_balance(
    cache_key: &str,
    data: &BalanceData,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = get_cache_file(cache_key) {
        fs::write(path, serde_json::to_string(data)?)?;
    }
    Ok(())
}

/// 统一的余额获取入口：内存缓存 → /api/user/self（access_token 路径）→ billing 接口 → 磁盘缓存兜底。
/// `Used` 和 `Balance` 两个段位都通过这个入口共享同一份 `BalanceData`。
pub fn fetch_balance() -> Option<BalanceData> {
    use crate::config::BalanceConfig;
    use crate::utils::credentials;

    // 自动从 Claude Code settings 读取 API Key 和 Base URL
    // 优先级：settings.local.json > settings.json > 系统环境变量
    let api_key = credentials::get_api_key()?;
    let api_base_url = credentials::get_api_base_url()?;

    let config = ApiConfig {
        enabled: true,
        api_key,
        api_base_url,
    };

    let key = cache_key(&config);
    if let Some(data) = get_in_memory_balance(&key) {
        return Some(data);
    }

    let client = ApiClient::new(config);

    let balance_config = BalanceConfig::load();
    if let Some(ref bc) = balance_config
        && let (Some(access_token), Some(user_id)) = (&bc.access_token, bc.new_api_user_id)
    {
        let quota_per_unit = bc.quota_per_unit.unwrap_or(500_000.0);
        if let Ok(data) = client.get_user_self_balance(access_token, user_id, quota_per_unit) {
            set_in_memory_balance(&key, &data);
            let _ = save_cached_balance(&key, &data);
            return Some(data);
        }
    }

    if let Ok(data) = client.get_balance() {
        set_in_memory_balance(&key, &data);
        let _ = save_cached_balance(&key, &data);
        return Some(data);
    }

    let (cached, _) = get_cached_balance(&key);
    if let Some(data) = cached {
        set_in_memory_balance(&key, &data);
        return Some(data);
    }

    None
}
