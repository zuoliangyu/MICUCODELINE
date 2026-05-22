use super::{Segment, SegmentData};
use crate::api::cache;
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct UsedSegment;

impl UsedSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for UsedSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        let data = cache::fetch_balance()?;

        let mut metadata = HashMap::new();
        metadata.insert("used".to_string(), data.used.to_string());
        if let Some(ref g) = data.group_name {
            metadata.insert("group".to_string(), g.clone());
        }

        Some(SegmentData {
            primary: format!("已用:{}", data.format_used()),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Used
    }
}
