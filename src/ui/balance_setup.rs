use crate::config::BalanceConfig;
use std::io::{self, Write};

pub fn run_setup() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔═══════════════════════════════════════════════════════╗");
    println!("║         Balance API Configuration Setup              ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    println!("This wizard will help you configure the Balance API credentials.");
    println!("You only need to provide your API Key (sk-xxx).\n");

    // Get API Key
    print!("Enter your API Key: ");
    io::stdout().flush()?;
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    let api_key = api_key.trim().to_string();

    if api_key.is_empty() {
        return Err("API Key cannot be empty".into());
    }

    // Optional: user access token for showing real account balance
    println!("\n--- 可选：用户账户余额（解决 API Key 无限额度显示 ∞ 的问题）---");
    println!("如果你的 API Key 设置了无限额度，billing 接口会返回 ∞ 而非实际余额。");
    println!("可配置用户 Access Token 来直接查询账户余额。");
    println!("Access Token 和 User ID 可在 new-api 个人中心页面获取。");
    print!("\n是否配置用户 Access Token？(y/N): ");
    io::stdout().flush()?;
    let mut use_user_auth = String::new();
    io::stdin().read_line(&mut use_user_auth)?;

    let (access_token, new_api_user_id, exchange_rate, quota_per_unit) =
        if use_user_auth.trim().eq_ignore_ascii_case("y") {
            print!("Enter your Access Token: ");
            io::stdout().flush()?;
            let mut token = String::new();
            io::stdin().read_line(&mut token)?;
            let token = token.trim().to_string();

            print!("Enter your User ID (numeric): ");
            io::stdout().flush()?;
            let mut uid_str = String::new();
            io::stdin().read_line(&mut uid_str)?;
            let uid: i64 = uid_str.trim().parse().unwrap_or(0);

            print!("Exchange rate USD→CNY (default 7.3, press Enter to use default): ");
            io::stdout().flush()?;
            let mut rate_str = String::new();
            io::stdin().read_line(&mut rate_str)?;
            let rate: f64 = rate_str.trim().parse().unwrap_or(7.3);

            print!("Quota per unit (default 500000, press Enter to use default): ");
            io::stdout().flush()?;
            let mut qpu_str = String::new();
            io::stdin().read_line(&mut qpu_str)?;
            let qpu: f64 = qpu_str.trim().parse().unwrap_or(500_000.0);

            (
                if token.is_empty() { None } else { Some(token) },
                if uid == 0 { None } else { Some(uid) },
                Some(rate),
                Some(qpu),
            )
        } else {
            (None, None, None, None)
        };

    // Create and save config
    let config = BalanceConfig {
        api_key,
        user_id: None,
        access_token,
        new_api_user_id,
        exchange_rate,
        quota_per_unit,
    };

    println!("\n📝 Saving configuration...");
    config.save()?;

    // Enable balance segment in config.toml
    enable_balance_segment()?;

    let config_path = BalanceConfig::config_path();
    println!("✓ Configuration saved successfully!");
    println!("  Location: {}", config_path.display());
    if config.new_api_user_id.is_some() {
        println!("  Mode: User account balance (via /api/user/self)");
    } else {
        println!("  Mode: API Key billing (via /v1/dashboard/billing/...)");
    }
    println!("✓ Balance segment enabled in config.toml");
    println!("\nYou can now use the Balance segment in your statusline.\n");

    Ok(())
}

fn enable_balance_segment() -> Result<(), Box<dyn std::error::Error>> {
    use crate::config::Config;

    let mut config = Config::load()?;

    // Find and enable balance segment
    for segment in &mut config.segments {
        if segment.id == crate::config::SegmentId::Balance {
            segment.enabled = true;
            break;
        }
    }

    // Save the updated config
    config.save()?;

    Ok(())
}
