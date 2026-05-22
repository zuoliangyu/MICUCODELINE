use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId, TranscriptEntry};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Default)]
pub struct SessionSegment;

impl SessionSegment {
    pub fn new() -> Self {
        Self
    }

    fn format_duration(ms: u64) -> String {
        if ms < 1000 {
            format!("{}ms", ms)
        } else if ms < 60_000 {
            let seconds = ms / 1000;
            format!("{}s", seconds)
        } else if ms < 3_600_000 {
            let minutes = ms / 60_000;
            let seconds = (ms % 60_000) / 1000;
            if seconds == 0 {
                format!("{}m", minutes)
            } else {
                format!("{}m{}s", minutes, seconds)
            }
        } else {
            let hours = ms / 3_600_000;
            let minutes = (ms % 3_600_000) / 60_000;
            if minutes == 0 {
                format!("{}h", hours)
            } else {
                format!("{}h{}m", hours, minutes)
            }
        }
    }

    /// 累加 transcript 中所有 assistant 消息的 input + output token，得到本次会话总 token 数。
    /// 无 transcript / 解析失败时返回 0。
    fn transcript_total_tokens<P: AsRef<Path>>(transcript_path: P) -> u64 {
        let file = match fs::File::open(transcript_path.as_ref()) {
            Ok(f) => f,
            Err(_) => return 0,
        };
        let mut total: u64 = 0;
        for line in BufReader::new(file).lines().map_while(Result::ok) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Ok(entry) = serde_json::from_str::<TranscriptEntry>(line)
                && entry.r#type.as_deref() == Some("assistant")
                && let Some(usage) = entry.message.and_then(|m| m.usage)
            {
                let n = usage.normalize();
                total += n.input_tokens as u64 + n.output_tokens as u64;
            }
        }
        total
    }

    /// 计算 tokens/second。优先用 API 耗时，缺失时退回总耗时。
    /// 任一项缺失返回 None。
    fn tokens_per_second(input: &InputData) -> Option<f64> {
        let tokens = Self::transcript_total_tokens(&input.transcript_path);
        let duration_ms = input
            .cost
            .as_ref()
            .and_then(|c| c.total_api_duration_ms.or(c.total_duration_ms))
            .unwrap_or(0);
        if tokens == 0 || duration_ms == 0 {
            return None;
        }
        Some(tokens as f64 / (duration_ms as f64 / 1000.0))
    }

    fn format_tps(tps: f64) -> String {
        if tps >= 1000.0 {
            format!("{:.1}k t/s", tps / 1000.0)
        } else {
            format!("{:.1} t/s", tps)
        }
    }
}

impl Segment for SessionSegment {
    fn collect(&self, input: &InputData) -> Option<SegmentData> {
        let cost_data = input.cost.as_ref()?;

        // 行内改动统计 (+N,-N)，沿用 cost 数据中的 Claude 写入行数。
        let changes_part = match (cost_data.total_lines_added, cost_data.total_lines_removed) {
            (Some(added), Some(removed)) => Some(format!("(+{},-{})", added, removed)),
            (Some(added), None) => Some(format!("(+{},-0)", added)),
            (None, Some(removed)) => Some(format!("(+0,-{})", removed)),
            (None, None) => None,
        };

        // 合计 tokens/second
        let tps_part = Self::tokens_per_second(input).map(|tps| format!("合计：{}", Self::format_tps(tps)));

        // 拼出主显示：`(+N,-N)  合计：X t/s`
        let primary = match (changes_part.as_deref(), tps_part.as_deref()) {
            (Some(c), Some(t)) => format!("{}  {}", c, t),
            (Some(c), None) => c.to_string(),
            (None, Some(t)) => t.to_string(),
            (None, None) => {
                // 若上述均无，退回到耗时显示（保持 EFlowCodeLine 原行为）
                cost_data.total_duration_ms.map(Self::format_duration)?
            }
        };

        // secondary 放总耗时，便于在窄屏/纯文本模式下仍能看到。
        let secondary = cost_data
            .total_duration_ms
            .map(Self::format_duration)
            .unwrap_or_default();

        let mut metadata = HashMap::new();
        if let Some(duration) = cost_data.total_duration_ms {
            metadata.insert("duration_ms".to_string(), duration.to_string());
        }
        if let Some(api_duration) = cost_data.total_api_duration_ms {
            metadata.insert("api_duration_ms".to_string(), api_duration.to_string());
        }
        if let Some(added) = cost_data.total_lines_added {
            metadata.insert("lines_added".to_string(), added.to_string());
        }
        if let Some(removed) = cost_data.total_lines_removed {
            metadata.insert("lines_removed".to_string(), removed.to_string());
        }
        if let Some(tps) = Self::tokens_per_second(input) {
            metadata.insert("tokens_per_second".to_string(), format!("{:.2}", tps));
        }

        Some(SegmentData {
            primary,
            secondary,
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Session
    }
}
