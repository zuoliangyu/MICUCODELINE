use crate::config::{
    AnsiColor, ColorConfig, IconConfig, SegmentConfig, SegmentId, TextStyleConfig,
};
use std::collections::HashMap;

pub fn model_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Model,
        enabled: true,
        icon: IconConfig {
            plain: "🤖".to_string(),
            nerd_font: "\u{e26d}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            text: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            background: Some(AnsiColor::Rgb {
                r: 42,
                g: 56,
                b: 110,
            }),
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn context_window_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::ContextWindow,
        enabled: true,
        icon: IconConfig {
            plain: "⚡️".to_string(),
            nerd_font: "\u{f49b}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            text: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            background: Some(AnsiColor::Rgb {
                r: 68,
                g: 95,
                b: 167,
            }),
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn usage_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Usage,
        enabled: true,
        icon: IconConfig {
            plain: "📊".to_string(),
            nerd_font: "\u{f0a9e}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            text: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            background: Some(AnsiColor::Rgb {
                r: 92,
                g: 124,
                b: 192,
            }),
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn cost_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Cost,
        enabled: true,
        icon: IconConfig {
            plain: "💰".to_string(),
            nerd_font: "\u{eec1}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            text: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            background: Some(AnsiColor::Rgb {
                r: 188,
                g: 121,
                b: 196,
            }),
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn session_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Session,
        enabled: true,
        icon: IconConfig {
            plain: "⏱️".to_string(),
            nerd_font: "\u{f19bb}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            text: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            background: Some(AnsiColor::Rgb {
                r: 220,
                g: 99,
                b: 134,
            }),
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn output_style_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::OutputStyle,
        enabled: false,
        icon: IconConfig {
            plain: "🔌".to_string(),
            nerd_font: "\u{f12f5}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            text: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            background: Some(AnsiColor::Rgb {
                r: 228,
                g: 134,
                b: 78,
            }),
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn cwd_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Cwd,
        enabled: true,
        icon: IconConfig {
            plain: "📁".to_string(),
            nerd_font: "\u{f024b}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            text: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            background: Some(AnsiColor::Rgb {
                r: 43,
                g: 60,
                b: 95,
            }),
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn directory_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Directory,
        enabled: true,
        icon: IconConfig {
            plain: "📁".to_string(),
            nerd_font: "\u{f024b}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            text: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            background: Some(AnsiColor::Rgb {
                r: 63,
                g: 86,
                b: 142,
            }),
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn git_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Git,
        enabled: true,
        icon: IconConfig {
            plain: "🌿".to_string(),
            nerd_font: "\u{f02a2}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            text: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            background: Some(AnsiColor::Rgb {
                r: 87,
                g: 114,
                b: 175,
            }),
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn used_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Used,
        enabled: true,
        icon: IconConfig {
            plain: "💸".to_string(),
            nerd_font: "\u{f155}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            text: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            background: Some(AnsiColor::Rgb {
                r: 184,
                g: 102,
                b: 169,
            }),
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn balance_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Balance,
        enabled: true,
        icon: IconConfig {
            plain: "💵".to_string(),
            nerd_font: "\u{f0d6}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            text: Some(AnsiColor::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),
            background: Some(AnsiColor::Rgb {
                r: 218,
                g: 159,
                b: 86,
            }),
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn branding_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Branding,
        enabled: true,
        icon: IconConfig {
            plain: "MICU".to_string(),
            nerd_font: "MICU".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 30,
                g: 30,
                b: 30,
            }),
            text: Some(AnsiColor::Rgb {
                r: 30,
                g: 30,
                b: 30,
            }),
            background: Some(AnsiColor::Rgb {
                r: 245,
                g: 213,
                b: 96,
            }),
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}
