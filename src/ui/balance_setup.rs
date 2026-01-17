use crate::config::BalanceConfig;
use std::io::{self, Write};

pub fn run_setup() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔═══════════════════════════════════════════════════════╗");
    println!("║         Balance API Configuration Setup              ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    println!("This wizard will help you configure the Balance API credentials.");
    println!("You need to provide your API Key and User ID.\n");

    // Get API Key
    print!("Enter your API Key: ");
    io::stdout().flush()?;
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    let api_key = api_key.trim().to_string();

    if api_key.is_empty() {
        return Err("API Key cannot be empty".into());
    }

    // Get User ID
    print!("Enter your User ID: ");
    io::stdout().flush()?;
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id)?;
    let user_id = user_id.trim().to_string();

    if user_id.is_empty() {
        return Err("User ID cannot be empty".into());
    }

    // Create and save config
    let config = BalanceConfig { api_key, user_id };

    println!("\n📝 Saving configuration...");
    config.save()?;

    // Enable balance segment in config.toml
    enable_balance_segment()?;

    let config_path = BalanceConfig::config_path();
    println!("✓ Configuration saved successfully!");
    println!("  Location: {}", config_path.display());
    println!("\n  API URL: {}", BalanceConfig::api_url());
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
