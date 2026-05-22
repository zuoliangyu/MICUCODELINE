use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct BrandingSegment {
    text: String,
}

impl BrandingSegment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }
}

impl Segment for BrandingSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        Some(SegmentData {
            primary: self.text.clone(),
            secondary: String::new(),
            metadata: HashMap::new(),
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Branding
    }
}
