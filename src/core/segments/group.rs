use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};

#[derive(Default)]
pub struct GroupSegment;

impl GroupSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for GroupSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        // billing API 不返回 group 信息，该 segment 暂时不可用
        // 如需 group 信息可后续通过其他接口获取
        None
    }

    fn id(&self) -> SegmentId {
        SegmentId::Group
    }
}
