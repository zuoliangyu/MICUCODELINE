use super::types::Config;
use std::path::PathBuf;

/// Result of config initialization (kept for CLI compatibility).
#[derive(Debug)]
pub enum InitResult {
    /// Config was created at the given path
    Created(PathBuf),
    /// Config already existed at the given path
    AlreadyExists(PathBuf),
}

pub struct ConfigLoader;

impl ConfigLoader {
    /// Always returns the hard-coded default config — user config files are ignored.
    pub fn load() -> Config {
        Config::default()
    }
}

impl Config {
    /// Always returns the hard-coded default config.
    /// Kept as a fallible signature for backwards compatibility with the CLI.
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        Ok(Config::default())
    }

    /// Saving is intentionally a no-op now that themes are hard-coded.
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Init no longer writes anything to disk; reports as already existing.
    pub fn init() -> Result<InitResult, Box<dyn std::error::Error>> {
        Ok(InitResult::AlreadyExists(PathBuf::from(
            "<built-in: themes are hard-coded>",
        )))
    }

    /// Validate configuration — still useful for sanity-checking the built-in default.
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

        Ok(())
    }

    /// Print configuration as TOML (still handy for `--check`/`--print` in CLI).
    pub fn print(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        println!("{}", content);
        Ok(())
    }
}
