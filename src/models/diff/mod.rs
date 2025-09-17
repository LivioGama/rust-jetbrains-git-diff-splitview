// diffsplit/src/models/diff/mod.rs
// Diff-related data structures and types

#[derive(Debug, Clone)]
pub struct ChangeBlock {
    pub start_line: usize,
    pub end_line: usize,
}

impl ChangeBlock {
    pub fn new(start_line: usize, end_line: usize) -> Self {
        Self {
            start_line,
            end_line,
        }
    }

    pub fn len(&self) -> usize {
        self.end_line.saturating_sub(self.start_line) + 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains_line(&self, line_idx: usize) -> bool {
        line_idx >= self.start_line && line_idx <= self.end_line
    }
}

#[derive(Debug, Clone)]
pub struct AnchorPoint {
    pub y_left_doc: f32,
    pub y_right_doc: f32,
}

impl AnchorPoint {
    pub fn new(y_left: f32, y_right: f32) -> Self {
        Self {
            y_left_doc: y_left,
            y_right_doc: y_right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MappingSegment {
    pub left_start: f32,
    pub left_end: f32,
    pub right_start: f32,
    pub right_end: f32,
    pub slope: f32,
    pub left_tangent: f32,
    pub right_tangent: f32,
}

impl MappingSegment {
    pub fn new(left_start: f32, left_end: f32, right_start: f32, right_end: f32) -> Self {
        let slope = if (left_end - left_start).abs() > f32::EPSILON {
            (right_end - right_start) / (left_end - left_start)
        } else {
            0.0
        };

        Self {
            left_start,
            left_end,
            right_start,
            right_end,
            slope,
            left_tangent: 0.0,
            right_tangent: 0.0,
        }
    }

    pub fn map_left_to_right(&self, left_y: f32) -> f32 {
        if left_y < self.left_start {
            self.right_start
        } else if left_y > self.left_end {
            self.right_end
        } else {
            self.right_start + self.slope * (left_y - self.left_start)
        }
    }

    pub fn map_right_to_left(&self, right_y: f32) -> f32 {
        if right_y < self.right_start {
            self.left_start
        } else if right_y > self.right_end {
            self.left_end
        } else if self.slope.abs() > f32::EPSILON {
            self.left_start + (right_y - self.right_start) / self.slope
        } else {
            self.left_start
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiffAnalysis {
    pub change_blocks: Vec<ChangeBlock>,
    pub anchors: Vec<AnchorPoint>,
    pub mapping_segments: Vec<MappingSegment>,
}

impl DiffAnalysis {
    pub fn new() -> Self {
        Self {
            change_blocks: Vec::new(),
            anchors: Vec::new(),
            mapping_segments: Vec::new(),
        }
    }

    pub fn with_change_blocks(mut self, blocks: Vec<ChangeBlock>) -> Self {
        self.change_blocks = blocks;
        self
    }

    pub fn with_anchors(mut self, anchors: Vec<AnchorPoint>) -> Self {
        self.anchors = anchors;
        self
    }

    pub fn with_mapping_segments(mut self, segments: Vec<MappingSegment>) -> Self {
        self.mapping_segments = segments;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.change_blocks.is_empty()
    }

    pub fn total_changes(&self) -> usize {
        self.change_blocks.len()
    }
}

impl Default for DiffAnalysis {
    fn default() -> Self {
        Self::new()
    }
}
