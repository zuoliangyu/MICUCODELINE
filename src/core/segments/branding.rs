use super::{Segment, SegmentData};
use crate::config::{BalanceConfig, InputData, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct BrandingSegment;

impl BrandingSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for BrandingSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        // 只在有 balance_config.json 时显示（表示使用 OpenClaudeCode）
        if BalanceConfig::load().is_none() {
            return None;
        }

        Some(SegmentData {
            primary: String::new(),
            secondary: String::new(),
            metadata: HashMap::new(),
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Branding
    }
}
