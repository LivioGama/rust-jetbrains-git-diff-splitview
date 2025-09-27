// Scroll synchronization for diff viewer panes
#![allow(dead_code)]

use crate::models::diff::{AnchorPoint, ChangeBlock, MappingSegment};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MasterPane {
    None,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct ScrollSync {
    master_pane: MasterPane,
    left_scroll_offset: f32,
    right_scroll_offset: f32,
    viewport_height: f32,
    line_height: f32,
    // Cache for frequent calculations
    cached_half_viewport: f32,
}

impl ScrollSync {
    pub fn new(line_height: f32, viewport_height: f32) -> Self {
        Self {
            master_pane: MasterPane::None,
            left_scroll_offset: 0.0,
            right_scroll_offset: 0.0,
            viewport_height,
            line_height,
            cached_half_viewport: viewport_height / 2.0,
        }
    }

    pub fn set_left_scroll(&mut self, offset: f32) {
        self.left_scroll_offset = offset;
        self.master_pane = MasterPane::Left;
    }

    pub fn set_right_scroll(&mut self, offset: f32) {
        self.right_scroll_offset = offset;
        self.master_pane = MasterPane::Right;
    }

    pub fn left_scroll_offset(&self) -> f32 {
        self.left_scroll_offset
    }

    pub fn right_scroll_offset(&self) -> f32 {
        self.right_scroll_offset
    }

    pub fn synchronize_scrolls(&mut self, mapping_function: impl Fn(f32) -> f32) {
        // Use cached half viewport calculation for better performance
        match self.master_pane {
            MasterPane::Left => {
                let left_center = self.left_scroll_offset + self.cached_half_viewport;
                let right_target_center = mapping_function(left_center);
                self.right_scroll_offset = right_target_center - self.cached_half_viewport;
            }
            MasterPane::Right => {
                let right_center = self.right_scroll_offset + self.cached_half_viewport;
                let left_target_center = mapping_function(right_center);
                self.left_scroll_offset = left_target_center - self.cached_half_viewport;
            }
            MasterPane::None => {}
        }
    }

    pub fn update_viewport_height(&mut self, height: f32) {
        self.viewport_height = height;
        // Update cache when viewport height changes
        self.cached_half_viewport = height / 2.0;
    }

    pub fn master_pane(&self) -> MasterPane {
        self.master_pane
    }

    pub fn is_synchronizing(&self) -> bool {
        self.master_pane != MasterPane::None
    }
}

// Mapping functions for scroll synchronization
pub fn map_left_to_right(y: f32, segments: &[MappingSegment]) -> f32 {
    if segments.is_empty() {
        return y;
    }

    // Find the segment containing y
    for segment in segments {
        if y >= segment.left_start && y <= segment.left_end {
            let t = (y - segment.left_start) / (segment.left_end - segment.left_start);
            return segment.right_start + t * (segment.right_end - segment.right_start);
        }
    }

    // Extrapolate if outside segments
    if y < segments[0].left_start {
        segments[0].right_start + (y - segments[0].left_start) * segments[0].slope
    } else {
        let last = &segments[segments.len() - 1];
        last.right_end + (y - last.left_end) * last.slope
    }
}

pub fn map_right_to_left(y: f32, segments: &[MappingSegment]) -> f32 {
    // Create reverse mapping by swapping left/right in segments
    let reverse_segments: Vec<MappingSegment> = segments
        .iter()
        .map(|s| MappingSegment {
            left_start: s.right_start,
            left_end: s.right_end,
            right_start: s.left_start,
            right_end: s.left_end,
            slope: 1.0 / s.slope.max(0.001), // Avoid division by zero
            left_tangent: s.right_tangent,
            right_tangent: s.left_tangent,
        })
        .collect();

    map_left_to_right(y, &reverse_segments)
}

// Build anchor points from change blocks for scroll synchronization
pub fn build_anchors_from_blocks(blocks: &[ChangeBlock], line_height: f32) -> Vec<AnchorPoint> {
    let mut anchors = Vec::new();

    // Add sentinel anchor at top
    anchors.push(AnchorPoint {
        y_left_doc: 0.0,
        y_right_doc: 0.0,
    });

    for (_i, block) in blocks.iter().enumerate() {
        // Calculate block center using the unified line range
        let block_lines = (block.end_line - block.start_line + 1) as f32;
        let block_start_y = block.start_line as f32 * line_height;
        let block_center_y = block_start_y + (block_lines * line_height) / 2.0;

        // Use block size for weight calculation
        let _max_lines = block_lines;

        anchors.push(AnchorPoint {
            y_left_doc: block_center_y,
            y_right_doc: block_center_y,
        });
    }

    // Add sentinel anchor at bottom with proper calculation
    // Add sentinel anchor at bottom with improved positioning
    let last_y = if let Some(last_block) = blocks.last() {
        (last_block.end_line + 1) as f32 * line_height
    } else {
        1000.0
    };

    anchors.push(AnchorPoint {
        y_left_doc: last_y,
        y_right_doc: last_y,
    });

    anchors
}

pub fn build_mapping_segments(anchors: &[AnchorPoint]) -> Vec<MappingSegment> {
    let mut segments = Vec::new();

    for i in 0..anchors.len().saturating_sub(1) {
        let left_start = anchors[i].y_left_doc;
        let left_end = anchors[i + 1].y_left_doc;
        let right_start = anchors[i].y_right_doc;
        let right_end = anchors[i + 1].y_right_doc;

        let slope = if left_end != left_start {
            (right_end - right_start) / (left_end - left_start)
        } else {
            1.0
        };

        segments.push(MappingSegment {
            left_start,
            left_end,
            right_start,
            right_end,
            slope,
            left_tangent: slope,
            right_tangent: slope,
        });
    }

    segments
}
