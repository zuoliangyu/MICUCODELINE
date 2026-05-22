#!/usr/bin/env python3
"""One-shot patch script for theme_*.rs.
Re-runnable: leaves files in target shape regardless of starting state."""

import re
from pathlib import Path

THEME_DIR = Path(r"E:/work/中转站/MicuCodeLine/src/ui/themes")

# 18 files share the same shape: <header>, several segment fns, then `pub fn branding_segment`.
# We:
#   1) ensure used_segment defaults to enabled:true with new icon.
#   2) inject cwd_segment if absent.

USED_ICON_BLOCK = (
    "        id: SegmentId::Used,\n"
    "        enabled: true,\n"
    "        icon: IconConfig {\n"
    '            plain: "已用\U0001f4b8".to_string(),\n'
    '            nerd_font: "\\u{f155}".to_string(),\n'
    "        },\n"
)

CWD_FN = """pub fn cwd_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Cwd,
        enabled: true,
        icon: IconConfig {
            plain: "\U0001f4c1".to_string(),
            nerd_font: "\\u{f024b}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 11 }),
            text: Some(AnsiColor::Color16 { c16: 10 }),
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

"""

USED_BLOCK_RE = re.compile(
    r"        id: SegmentId::Used,\n"
    r"        enabled: (?:true|false),\n"
    r"        icon: IconConfig \{\n"
    r"            plain: \"[^\"]*\"\.to_string\(\),\n"
    r"            nerd_font: \"[^\"]*\"\.to_string\(\),\n"
    r"        \},\n"
)

for f in sorted(THEME_DIR.glob("theme_*.rs")):
    src = f.read_text(encoding="utf-8")

    # 1) Patch used_segment block.
    new_src, n = USED_BLOCK_RE.subn(lambda _m: USED_ICON_BLOCK, src, count=1)
    if n == 0:
        print(f"[warn] {f.name}: used_segment block did not match")
    src = new_src

    # 2) Inject cwd_segment if missing.
    if "fn cwd_segment" not in src:
        src = src.replace("pub fn branding_segment", CWD_FN + "pub fn branding_segment", 1)

    f.write_text(src, encoding="utf-8")
    print(f"[ok] {f.name}")
