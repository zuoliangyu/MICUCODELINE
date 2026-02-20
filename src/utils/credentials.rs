use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
struct OAuthCredentials {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "refreshToken")]
    refresh_token: Option<String>,
    #[serde(rename = "expiresAt")]
    expires_at: Option<u64>,
    scopes: Option<Vec<String>>,
    #[serde(rename = "subscriptionType")]
    subscription_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CredentialsFile {
    #[serde(rename = "claudeAiOauth")]
    claude_ai_oauth: Option<OAuthCredentials>,
}

pub fn get_oauth_token() -> Option<String> {
    if cfg!(target_os = "macos") {
        get_oauth_token_macos()
    } else {
        get_oauth_token_file()
    }
}

fn get_oauth_token_macos() -> Option<String> {
    use std::process::Command;

    let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());

    let output = Command::new("security")
        .args([
            "find-generic-password",
            "-a",
            &user,
            "-w",
            "-s",
            "Claude Code-credentials",
        ])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let json_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !json_str.is_empty() {
                if let Ok(creds_file) = serde_json::from_str::<CredentialsFile>(&json_str) {
                    return creds_file.claude_ai_oauth.map(|oauth| oauth.access_token);
                }
            }
            None
        }
        _ => {
            // Fallback to file-based credentials
            get_oauth_token_file()
        }
    }
}

fn get_oauth_token_file() -> Option<String> {
    let credentials_path = get_credentials_path()?;

    if !credentials_path.exists() {
        return None;
    }

    let content = std::fs::read_to_string(&credentials_path).ok()?;
    let creds_file: CredentialsFile = serde_json::from_str(&content).ok()?;

    creds_file.claude_ai_oauth.map(|oauth| oauth.access_token)
}

fn get_credentials_path() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    Some(home.join(".claude").join(".credentials.json"))
}

// ── Claude Code settings (API Key + Base URL) ──────────────────────────────

#[derive(Debug, Deserialize)]
struct ClaudeSettings {
    #[serde(default)]
    env: HashMap<String, String>,
}

/// 从 Claude Code settings 文件读取 env 块
/// 优先级：settings.local.json > settings.json > 系统环境变量
fn read_settings_env() -> HashMap<String, String> {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return HashMap::new(),
    };
    let claude_dir = home.join(".claude");

    let mut merged: HashMap<String, String> = HashMap::new();

    // 先读 settings.json，再读 settings.local.json（后者覆盖前者）
    for filename in &["settings.json", "settings.local.json"] {
        let path = claude_dir.join(filename);
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(s) = serde_json::from_str::<ClaudeSettings>(&content) {
                merged.extend(s.env);
            }
        }
    }

    merged
}

/// 读取用于 API 调用的 API Key
/// 优先级：settings.local.json > settings.json > 系统环境变量
pub fn get_api_key() -> Option<String> {
    let env = read_settings_env();

    // ANTHROPIC_AUTH_TOKEN 和 ANTHROPIC_API_KEY 都支持
    for key in &["ANTHROPIC_AUTH_TOKEN", "ANTHROPIC_API_KEY"] {
        if let Some(v) = env.get(*key) {
            if !v.is_empty() {
                return Some(v.clone());
            }
        }
    }

    // 系统环境变量兜底
    for key in &["ANTHROPIC_AUTH_TOKEN", "ANTHROPIC_API_KEY"] {
        if let Ok(v) = std::env::var(key) {
            if !v.is_empty() {
                return Some(v);
            }
        }
    }

    None
}

/// 读取 API Base URL（中转站地址）
/// 优先级：settings.local.json > settings.json > 系统环境变量
pub fn get_api_base_url() -> Option<String> {
    let env = read_settings_env();

    if let Some(v) = env.get("ANTHROPIC_BASE_URL") {
        if !v.is_empty() {
            return Some(v.trim_end_matches('/').to_string());
        }
    }

    if let Ok(v) = std::env::var("ANTHROPIC_BASE_URL") {
        if !v.is_empty() {
            return Some(v.trim_end_matches('/').to_string());
        }
    }

    None
}
