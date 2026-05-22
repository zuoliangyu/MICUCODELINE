#!/usr/bin/env python3
"""Regenerate all 9 theme files in src/ui/themes/ to a unified powerline-arrow style.

Each segment gets:
  - icon (NerdFont + plain emoji)
  - foreground = white (or palette-specific)
  - background per segment role (model/dir/git/...) from the theme's palette

Themes share segment roles/icons but differ in palette.
"""

from pathlib import Path

THEME_DIR = Path(r"E:/work/中转站/MicuCodeLine/src/ui/themes")

# Segment definitions: (fn_name, id, plain_icon, nerd_font_icon, role_key, enabled)
# `role_key` indexes into the per-theme palette below.
SEGMENTS = [
    ("model_segment",          "Model",         "🤖", "\\u{e26d}",  "model",   True),
    ("context_window_segment", "ContextWindow", "⚡️", "\\u{f49b}",  "ctx",     True),
    ("usage_segment",          "Usage",         "📊", "\\u{f0a9e}", "usage",   True),
    ("cost_segment",           "Cost",          "💰", "\\u{eec1}",  "cost",    True),
    ("session_segment",        "Session",       "⏱️", "\\u{f19bb}", "session", True),
    ("output_style_segment",   "OutputStyle",   "🔌", "\\u{f12f5}", "style",   False),
    ("cwd_segment",            "Cwd",           "📁", "\\u{f024b}", "cwd",     True),
    ("directory_segment",      "Directory",     "📁", "\\u{f024b}", "dir",     True),
    ("git_segment",            "Git",           "🌿", "\\u{f02a2}", "git",     True),
    ("used_segment",           "Used",          "💸", "\\u{f155}",  "used",    True),
    ("balance_segment",        "Balance",       "💵", "\\u{f0d6}",  "balance", True),
    ("branding_segment",       "Branding",      "MICU", "MICU", "brand", True),
]

# Per-theme palettes. Each palette maps role -> (bg_rgb, fg_rgb).
WHITE = (245, 245, 245)
BLACK = (30, 30, 30)
GRAY  = (180, 180, 180)

PALETTES = {
    # Default: vibrant neon gradient inspired by the reference screenshot.
    "default": {
        "model":   ((42, 56, 110),  WHITE),  # deep indigo
        "ctx":     ((68, 95, 167),  WHITE),
        "usage":   ((92, 124, 192), WHITE),
        "cost":    ((188, 121, 196),WHITE),  # pink-purple
        "session": ((220, 99, 134), WHITE),
        "style":   ((228, 134, 78), WHITE),
        "cwd":     ((43, 60, 95),   WHITE),  # second-row dark blue
        "dir":     ((63, 86, 142),  WHITE),
        "git":     ((87, 114, 175), WHITE),
        "used":    ((184, 102, 169),WHITE),
        "balance": ((218, 159, 86), WHITE),
        "brand":   ((245, 213, 96), BLACK),
    },
    # Cometix: cool cyan-purple palette.
    "cometix": {
        "model":   ((42, 64, 110), WHITE),
        "ctx":     ((58, 102, 160), WHITE),
        "usage":   ((70, 138, 192), WHITE),
        "cost":    ((128, 110, 200), WHITE),
        "session": ((172, 102, 192), WHITE),
        "style":   ((216, 100, 180), WHITE),
        "cwd":     ((38, 58, 96), WHITE),
        "dir":     ((58, 90, 142), WHITE),
        "git":     ((78, 134, 180), WHITE),
        "used":    ((148, 110, 200), WHITE),
        "balance": ((196, 124, 192), WHITE),
        "brand":   ((232, 174, 96), BLACK),
    },
    # Minimal: monochrome ramp.
    "minimal": {
        "model":   ((36, 36, 36), WHITE),
        "ctx":     ((62, 62, 62), WHITE),
        "usage":   ((88, 88, 88), WHITE),
        "cost":    ((114, 114, 114), WHITE),
        "session": ((140, 140, 140), BLACK),
        "style":   ((166, 166, 166), BLACK),
        "cwd":     ((44, 44, 44), WHITE),
        "dir":     ((78, 78, 78), WHITE),
        "git":     ((112, 112, 112), WHITE),
        "used":    ((146, 146, 146), BLACK),
        "balance": ((180, 180, 180), BLACK),
        "brand":   ((220, 220, 220), BLACK),
    },
    # Gruvbox: warm earth tones.
    "gruvbox": {
        "model":   ((40, 40, 40),   (251, 241, 199)),
        "ctx":     ((69, 133, 136), (251, 241, 199)),
        "usage":   ((104, 157, 106), (251, 241, 199)),
        "cost":    ((215, 153, 33), (40, 40, 40)),
        "session": ((184, 187, 38), (40, 40, 40)),
        "style":   ((211, 134, 155), (40, 40, 40)),
        "cwd":     ((60, 56, 54),   (251, 241, 199)),
        "dir":     ((124, 111, 100),(251, 241, 199)),
        "git":     ((177, 98, 134), (251, 241, 199)),
        "used":    ((204, 36, 29),  (251, 241, 199)),
        "balance": ((215, 153, 33), (40, 40, 40)),
        "brand":   ((250, 189, 47), (40, 40, 40)),
    },
    # Nord: cool blue/teal palette.
    "nord": {
        "model":   ((46, 52, 64),   (236, 239, 244)),
        "ctx":     ((59, 66, 82),   (236, 239, 244)),
        "usage":   ((76, 86, 106),  (236, 239, 244)),
        "cost":    ((94, 129, 172), (236, 239, 244)),
        "session": ((129, 161, 193),(236, 239, 244)),
        "style":   ((136, 192, 208),(46, 52, 64)),
        "cwd":     ((59, 66, 82),   (236, 239, 244)),
        "dir":     ((76, 86, 106),  (236, 239, 244)),
        "git":     ((143, 188, 187),(46, 52, 64)),
        "used":    ((180, 142, 173),(46, 52, 64)),
        "balance": ((163, 190, 140),(46, 52, 64)),
        "brand":   ((235, 203, 139),(46, 52, 64)),
    },
    # Powerline-dark: original dark theme intent.
    "powerline_dark": {
        "model":   ((45, 45, 45),   WHITE),
        "ctx":     ((60, 60, 60),   WHITE),
        "usage":   ((80, 80, 80),   WHITE),
        "cost":    ((113, 87, 39),  WHITE),
        "session": ((63, 95, 41),   WHITE),
        "style":   ((100, 60, 100), WHITE),
        "cwd":     ((42, 56, 90),   WHITE),
        "dir":     ((50, 70, 110),  WHITE),
        "git":     ((85, 105, 145), WHITE),
        "used":    ((140, 70, 110), WHITE),
        "balance": ((90, 130, 80),  WHITE),
        "brand":   ((200, 160, 60), BLACK),
    },
    # Powerline-light: light backgrounds, dark text.
    "powerline_light": {
        "model":   ((220, 220, 220), BLACK),
        "ctx":     ((205, 205, 230), BLACK),
        "usage":   ((190, 220, 230), BLACK),
        "cost":    ((230, 210, 170), BLACK),
        "session": ((205, 230, 200), BLACK),
        "style":   ((230, 200, 220), BLACK),
        "cwd":     ((220, 230, 240), BLACK),
        "dir":     ((205, 220, 240), BLACK),
        "git":     ((200, 220, 220), BLACK),
        "used":    ((240, 200, 215), BLACK),
        "balance": ((215, 235, 200), BLACK),
        "brand":   ((250, 220, 130), BLACK),
    },
    # Rose-pine: muted pink/lavender.
    "powerline_rose_pine": {
        "model":   ((25, 23, 36),   (224, 222, 244)),
        "ctx":     ((38, 35, 58),   (224, 222, 244)),
        "usage":   ((49, 116, 143), (224, 222, 244)),
        "cost":    ((246, 193, 119),(25, 23, 36)),
        "session": ((156, 207, 216),(25, 23, 36)),
        "style":   ((196, 167, 231),(25, 23, 36)),
        "cwd":     ((38, 35, 58),   (224, 222, 244)),
        "dir":     ((49, 47, 68),   (224, 222, 244)),
        "git":     ((110, 106, 134),(224, 222, 244)),
        "used":    ((235, 111, 146),(25, 23, 36)),
        "balance": ((156, 207, 216),(25, 23, 36)),
        "brand":   ((246, 193, 119),(25, 23, 36)),
    },
    # Tokyo Night.
    "powerline_tokyo_night": {
        "model":   ((26, 27, 38),   (192, 202, 245)),
        "ctx":     ((36, 40, 59),   (192, 202, 245)),
        "usage":   ((73, 159, 245), (26, 27, 38)),
        "cost":    ((224, 175, 104),(26, 27, 38)),
        "session": ((158, 206, 106),(26, 27, 38)),
        "style":   ((187, 154, 247),(26, 27, 38)),
        "cwd":     ((36, 40, 59),   (192, 202, 245)),
        "dir":     ((52, 59, 88),   (192, 202, 245)),
        "git":     ((125, 207, 255),(26, 27, 38)),
        "used":    ((247, 118, 142),(192, 202, 245)),
        "balance": ((158, 206, 106),(26, 27, 38)),
        "brand":   ((224, 175, 104),(26, 27, 38)),
    },
}

THEME_TO_PALETTE = {
    "theme_default": "default",
    "theme_cometix": "cometix",
    "theme_minimal": "minimal",
    "theme_gruvbox": "gruvbox",
    "theme_nord": "nord",
    "theme_powerline_dark": "powerline_dark",
    "theme_powerline_light": "powerline_light",
    "theme_powerline_rose_pine": "powerline_rose_pine",
    "theme_powerline_tokyo_night": "powerline_tokyo_night",
}


def rgb_block(r, g, b):
    return (f"            Some(AnsiColor::Rgb {{\n"
            f"                r: {r},\n"
            f"                g: {g},\n"
            f"                b: {b},\n"
            f"            }})")


def make_segment_fn(fn_name, seg_id, plain_icon, nerd_icon, role, enabled, palette):
    bg, fg = palette[role]
    enabled_str = "true" if enabled else "false"

    # Branding is special: the brand name lives in the icon fields, primary is blank.
    if seg_id == "Branding":
        icon_plain = "MICU"
        icon_nerd  = "MICU"
    else:
        icon_plain = plain_icon
        icon_nerd  = nerd_icon

    return f'''pub fn {fn_name}() -> SegmentConfig {{
    SegmentConfig {{
        id: SegmentId::{seg_id},
        enabled: {enabled_str},
        icon: IconConfig {{
            plain: "{icon_plain}".to_string(),
            nerd_font: "{icon_nerd}".to_string(),
        }},
        colors: ColorConfig {{
            icon:
{rgb_block(*fg)},
            text:
{rgb_block(*fg)},
            background:
{rgb_block(*bg)},
        }},
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }}
}}
'''


def regenerate_theme(theme_name, palette_key):
    palette = PALETTES[palette_key]
    parts = [
        "use crate::config::{\n"
        "    AnsiColor, ColorConfig, IconConfig, SegmentConfig, SegmentId, TextStyleConfig,\n"
        "};\n"
        "use std::collections::HashMap;\n\n"
    ]
    for fn_name, seg_id, plain_icon, nerd_icon, role, enabled in SEGMENTS:
        parts.append(make_segment_fn(fn_name, seg_id, plain_icon, nerd_icon, role, enabled, palette))
        parts.append("\n")
    return "".join(parts).rstrip() + "\n"


for theme_file, palette_key in THEME_TO_PALETTE.items():
    path = THEME_DIR / f"{theme_file}.rs"
    content = regenerate_theme(theme_file, palette_key)
    path.write_text(content, encoding="utf-8")
    print(f"[ok] {theme_file}.rs")
