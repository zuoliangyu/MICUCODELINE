use crate::config::Config;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

/// Theme is locked to a single hard-coded preset; this component just shows
/// the current name plus the active separator for context.
#[derive(Default)]
pub struct ThemeSelectorComponent;

impl ThemeSelectorComponent {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, f: &mut Frame, area: Rect, config: &Config) {
        let body = format!(
            "Theme: {} (locked)\nSeparator: \"{}\"",
            config.theme, config.style.separator
        );
        let widget = Paragraph::new(body)
            .block(Block::default().borders(Borders::ALL).title("Theme"))
            .wrap(ratatui::widgets::Wrap { trim: false });
        f.render_widget(widget, area);
    }
}
