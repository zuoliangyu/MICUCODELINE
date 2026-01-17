use serde::{Deserialize, Serialize};
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
