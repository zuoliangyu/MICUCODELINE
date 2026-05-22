use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

const MAX_SEGMENTS: usize = 5;

#[derive(Default)]
pub struct CwdSegment;

impl CwdSegment {
    pub fn new() -> Self {
        Self
    }

    /// 把 $HOME 前缀缩成 `~`，并对超长路径做中间省略。
    /// 同时统一保留原始分隔符（避免把 Windows 反斜杠强转成正斜杠）。
    fn format_path(path: &str) -> String {
        let mut display = Self::replace_home(path);
        let sep = Self::detect_separator(&display);

        let parts: Vec<&str> = display.split(sep).filter(|s| !s.is_empty()).collect();

        // 如果原路径以分隔符开头（Unix 绝对路径或 UNC），保留它。
        let leading = if display.starts_with(sep) {
            sep.to_string()
        } else {
            String::new()
        };

        if parts.len() > MAX_SEGMENTS {
            // 保留首段（如 `~` 或盘符），尾部 2 段，中间 `...`
            let head = parts.first().copied().unwrap_or("");
            let tail = &parts[parts.len() - 2..];
            let sep_s = sep.to_string();
            display = format!(
                "{}{}{}...{}{}",
                leading,
                head,
                sep_s,
                sep_s,
                tail.join(&sep_s),
            );
        }

        display
    }

    fn replace_home(path: &str) -> String {
        let Some(home) = dirs::home_dir() else {
            return path.to_string();
        };
        let home_str = home.to_string_lossy().to_string();
        if path.starts_with(&home_str) {
            let rest = &path[home_str.len()..];
            // 把残留的前导分隔符吃掉，避免出现 `~\` 后接 `\`
            let rest = rest.trim_start_matches(['/', '\\']);
            if rest.is_empty() {
                "~".to_string()
            } else {
                let sep = Self::detect_separator(&home_str);
                format!("~{}{}", sep, rest)
            }
        } else {
            path.to_string()
        }
    }

    fn detect_separator(path: &str) -> char {
        if path.contains('\\') { '\\' } else { '/' }
    }
}

impl Segment for CwdSegment {
    fn collect(&self, input: &InputData) -> Option<SegmentData> {
        let display = Self::format_path(&input.workspace.current_dir);

        let mut metadata = HashMap::new();
        metadata.insert("full_path".to_string(), input.workspace.current_dir.clone());

        Some(SegmentData {
            primary: display,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Cwd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_path_is_collapsed() {
        let p = r"E:\work\a\b\c\d\e\MicuCodeLine";
        let out = CwdSegment::format_path(p);
        assert!(out.contains("..."), "expected ellipsis in {}", out);
        assert!(
            out.ends_with(r"e\MicuCodeLine"),
            "tail not preserved: {}",
            out
        );
    }

    #[test]
    fn short_path_is_intact() {
        let p = r"E:\work\micu";
        assert_eq!(CwdSegment::format_path(p), p);
    }
}
