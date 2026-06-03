use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct ModelSegment;

impl ModelSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for ModelSegment {
    fn collect(&self, input: &InputData) -> Option<SegmentData> {
        let mut metadata = HashMap::new();
        metadata.insert("model_id".to_string(), input.model.id.clone());
        metadata.insert("display_name".to_string(), input.model.display_name.clone());

        // Directly show the display name Claude Code already resolved for us.
        // We intentionally do NOT remap it through models.toml patterns — pattern
        // matching produced odd fallback names (e.g. a generic "Claude 1M") for
        // models that didn't match a known entry.
        Some(SegmentData {
            primary: input.model.display_name.clone(),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Model
    }
}
