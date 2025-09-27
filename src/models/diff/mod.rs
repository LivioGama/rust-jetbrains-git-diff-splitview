#![allow(dead_code)]

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
}

#[derive(Debug, Clone)]
pub struct AnchorPoint {
    pub y_left_doc: f32,
    pub y_right_doc: f32,
}

impl AnchorPoint {}

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
