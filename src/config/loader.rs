use super::types::Config;
use std::fs;
use std::path::PathBuf;

/// Result of config initialization.
#[derive(Debug)]
pub enum InitResult {
    /// Config was created at the given path
    Created(PathBuf),
    /// Config already existed at the given path
    AlreadyExists(PathBuf),
}

pub struct ConfigLoader;

impl ConfigLoader {
    /// Infallible load helper used by the statusline render path.
    /// Falls back to the built-in default if the user's config is missing or unreadable.
    pub fn load() -> Config {
        Config::load().unwrap_or_else(|_| Config::default())
    }
}

impl Config {
    /// `~/.claude/micucodeline/config.toml` (fallback to a relative path if no home dir).
    pub fn get_config_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".claude")
                .join("micucodeline")
                .join("config.toml")
        } else {
            PathBuf::from(".claude/micucodeline/config.toml")
        }
    }

    /// Load configuration from `~/.claude/micucodeline/config.toml`.
    ///
    /// - If the file does not exist, returns the built-in default.
    /// - If the file exists, parse it and call `enforce_locks` so the three
    ///   brand segments (Balance / Used / Branding) are always present + enabled.
    /// - Parse errors propagate so the user sees a clear failure rather than
    ///   silently losing their customization.
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&config_path)?;
        let mut config: Config = toml::from_str(&content)?;
        config.enforce_locks();
        Ok(config)
    }

    /// Save configuration to `~/.claude/micucodeline/config.toml`.
    /// Locks are enforced before writing so the saved file always has the
    /// three brand segments enabled.
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write a copy with locks enforced — never mutate the caller.
        let mut to_write = self.clone();
        to_write.enforce_locks();

        let content = toml::to_string_pretty(&to_write)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    /// Initialize config: write the default config to disk if absent.
    pub fn init() -> Result<InitResult, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        if config_path.exists() {
            return Ok(InitResult::AlreadyExists(config_path));
        }

        let default_config = Config::default();
        default_config.save()?;
        Ok(InitResult::Created(config_path))
    }

    /// Validate configuration.
    pub fn check(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.segments.is_empty() {
            return Err("No segments configured".into());
        }

        let mut seen_ids = std::collections::HashSet::new();
        for segment in &self.segments {
            if !seen_ids.insert(segment.id) {
                return Err(format!("Duplicate segment ID: {:?}", segment.id).into());
            }
        }

        // Locked segments must be present and enabled (post-load this is guaranteed
        // by `enforce_locks`, but `check` may be called on a hand-built config too).
        for &locked_id in super::types::LOCKED_SEGMENTS {
            match self.segments.iter().find(|s| s.id == locked_id) {
                None => {
                    return Err(format!(
                        "Required segment {:?} is missing — Balance / Used / Branding must always be present",
                        locked_id
                    )
                    .into());
                }
                Some(seg) if !seg.enabled => {
                    return Err(format!(
                        "Required segment {:?} cannot be disabled",
                        locked_id
                    )
                    .into());
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Print configuration as TOML.
    pub fn print(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        println!("{}", content);
        Ok(())
    }
}
