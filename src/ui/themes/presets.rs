// The theme is now a single hard-coded preset (the powerline-arrow style designed
// for new-api / MicuCode). Theme switching, file-based themes, and the TUI theme
// selector are all gone — `ThemePresets::get_default()` is the only path.

use crate::config::{Config, StyleConfig, StyleMode};

use super::theme_default;

pub struct ThemePresets;

impl ThemePresets {
    /// Returns the only configured theme.
    /// Kept as `get_theme(&str)` for compatibility with code paths that still
    /// pass a name through; the argument is ignored on purpose.
    pub fn get_theme(_theme_name: &str) -> Config {
        Self::get_default()
    }

    pub fn get_default() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "\u{e0b0}".to_string(),
            },
            segments: vec![
                theme_default::model_segment(),
                theme_default::context_window_segment(),
                theme_default::usage_segment(),
                theme_default::cost_segment(),
                theme_default::session_segment(),
                theme_default::output_style_segment(),
                // -- newline injected before Cwd by the renderer --
                theme_default::cwd_segment(),
                theme_default::directory_segment(),
                theme_default::git_segment(),
                theme_default::used_segment(),
                theme_default::balance_segment(),
                // -- newline injected before Branding by the renderer --
                theme_default::branding_segment(),
            ],
            theme: "default".to_string(),
        }
    }
}
