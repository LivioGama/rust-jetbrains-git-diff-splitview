// Rectangle storage and connector state management for GPUI implementation

use gpui::*;

/// Storage for rectangle positions used by connector rendering
#[derive(Debug, Clone, Default)]
pub struct ConnectorState {
    /// Rectangle positions for left pane lines
    pub left_rects: Vec<Bounds<Pixels>>,
    /// Rectangle positions for right pane lines
    pub right_rects: Vec<Bounds<Pixels>>,
    /// Crushed line rectangles in left pane for pure insertions (line_index, rect, content)
    pub left_crushed_rects: Vec<(usize, Bounds<Pixels>, String)>,
    /// Crushed line rectangles in right pane for pure deletions (line_index, rect, content)
    pub right_crushed_rects: Vec<(usize, Bounds<Pixels>, String)>,
    /// Viewport height for connector rendering
    pub viewport_height: f32,
    /// Left pane scroll offset
    pub left_scroll_offset: f32,
    /// Right pane scroll offset
    pub right_scroll_offset: f32,
}

impl ConnectorState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Store left pane rectangle positions
    pub fn store_left_rects(&mut self, rects: Vec<Bounds<Pixels>>) {
        self.left_rects = rects;
    }

    /// Store right pane rectangle positions
    pub fn store_right_rects(&mut self, rects: Vec<Bounds<Pixels>>) {
        self.right_rects = rects;
    }

    /// Store left pane crushed line rectangles (for pure insertions)
    pub fn store_left_crushed_rects(
        &mut self,
        crushed_rects: Vec<(usize, Bounds<Pixels>, String)>,
    ) {
        self.left_crushed_rects = crushed_rects;
    }

    /// Store right pane crushed line rectangles (for pure deletions)
    pub fn store_right_crushed_rects(
        &mut self,
        crushed_rects: Vec<(usize, Bounds<Pixels>, String)>,
    ) {
        self.right_crushed_rects = crushed_rects;
    }

    /// Update viewport and scroll information
    pub fn update_viewport(&mut self, viewport_height: f32, left_scroll: f32, right_scroll: f32) {
        self.viewport_height = viewport_height;
        self.left_scroll_offset = left_scroll;
        self.right_scroll_offset = right_scroll;
    }

    /// Get rectangle for a specific line in left pane
    pub fn get_left_rect(&self, line_index: usize) -> Option<&Bounds<Pixels>> {
        self.left_rects.get(line_index)
    }

    /// Get rectangle for a specific line in right pane
    pub fn get_right_rect(&self, line_index: usize) -> Option<&Bounds<Pixels>> {
        self.right_rects.get(line_index)
    }

    /// Find crushed line rectangle by line index in left pane
    pub fn find_left_crushed_rect(
        &self,
        line_index: usize,
    ) -> Option<&(usize, Bounds<Pixels>, String)> {
        self.left_crushed_rects
            .iter()
            .find(|(idx, _, _)| *idx == line_index)
    }

    /// Find crushed line rectangle by line index in right pane
    pub fn find_right_crushed_rect(
        &self,
        line_index: usize,
    ) -> Option<&(usize, Bounds<Pixels>, String)> {
        self.right_crushed_rects
            .iter()
            .find(|(idx, _, _)| *idx == line_index)
    }

    /// Check if we have valid rectangle data for connector rendering
    pub fn has_valid_rects(&self) -> bool {
        !self.left_rects.is_empty() && !self.right_rects.is_empty()
    }

    /// Clear all stored rectangle data
    pub fn clear(&mut self) {
        self.left_rects.clear();
        self.right_rects.clear();
        self.left_crushed_rects.clear();
        self.right_crushed_rects.clear();
        self.viewport_height = 0.0;
        self.left_scroll_offset = 0.0;
        self.right_scroll_offset = 0.0;
    }
}

/// Connector rendering utilities
pub struct ConnectorUtils;

impl ConnectorUtils {
    /// Calculate connector coordinates with scroll offset adjustments
    pub fn calculate_connector_coords(
        left_start_rect: &Bounds<Pixels>,
        left_end_rect: &Bounds<Pixels>,
        right_start_rect: &Bounds<Pixels>,
        right_end_rect: &Bounds<Pixels>,
        left_scroll_offset: f32,
        right_scroll_offset: f32,
    ) -> (f32, f32, f32, f32) {
        let left_y_start = left_start_rect.origin.y.0 - left_scroll_offset;
        let left_y_end =
            left_end_rect.origin.y.0 + left_end_rect.size.height.0 - left_scroll_offset;
        let right_y_start = right_start_rect.origin.y.0 - right_scroll_offset;
        let right_y_end =
            right_end_rect.origin.y.0 + right_end_rect.size.height.0 - right_scroll_offset;

        (left_y_start, left_y_end, right_y_start, right_y_end)
    }

    pub fn estimate_scale_factors(
        left_rects: &[Bounds<Pixels>],
        right_rects: &[Bounds<Pixels>],
        line_height: f32,
    ) -> Option<(f32, f32)> {
        if line_height <= 0.0 {
            return None;
        }

        let left_rect = left_rects.first()?;
        let right_rect = right_rects.first()?;

        Some((
            left_rect.size.height.0 / line_height,
            right_rect.size.height.0 / line_height,
        ))
    }
}
