// src/state/app_state.rs
// Application state structure extracted from state/mod.rs

use crate::models::*;
use crate::navigation::NavigationState;

/// Application state
#[derive(Debug, Clone)]
pub struct AppState {
    pub current_file: String,
    pub left_lines: Vec<DisplayLine>,
    pub right_lines: Vec<DisplayLine>,
    pub change_blocks: Vec<ChangeBlock>,
    pub imara_analysis: crate::diff::imara::ImaraDiffAnalysis,
    pub anchors: Vec<AnchorPoint>,
    pub mapping_segments: Vec<MappingSegment>,
    pub connector_curves: Vec<ConnectorCurve>,
    pub navigation_state: NavigationState,
    pub viewport_height: f32,
    pub left_scroll_offset: f32,
    pub right_scroll_offset: f32,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_file: String::new(),
            left_lines: Vec::new(),
            right_lines: Vec::new(),
            change_blocks: Vec::new(),
            imara_analysis: crate::diff::imara::ImaraDiffAnalysis { blocks: Vec::new() },
            anchors: Vec::new(),
            mapping_segments: Vec::new(),
            connector_curves: Vec::new(),
            navigation_state: NavigationState::default(),
            viewport_height: 1000.0,
            left_scroll_offset: 0.0,
            right_scroll_offset: 0.0,
        }
    }

    pub fn with_file(mut self, file_path: String) -> Self {
        self.current_file = file_path;
        self
    }

    pub fn with_lines(mut self, left: Vec<DisplayLine>, right: Vec<DisplayLine>) -> Self {
        self.left_lines = left;
        self.right_lines = right;
        self
    }

    pub fn with_diff_analysis(
        mut self,
        change_blocks: Vec<ChangeBlock>,
        anchors: Vec<AnchorPoint>,
        mapping_segments: Vec<MappingSegment>,
    ) -> Self {
        self.change_blocks = change_blocks;
        self.anchors = anchors;
        self.mapping_segments = mapping_segments;
        self.update_navigation_state();
        self
    }

    pub fn update_navigation_state(&mut self) {
        self.navigation_state.total_blocks = self.change_blocks.len();
        self.navigation_state.total_connectors = self.connector_curves.len();

        // Ensure current indices are within bounds
        if self.navigation_state.current_block_index >= self.change_blocks.len()
            && !self.change_blocks.is_empty()
        {
            self.navigation_state.current_block_index = self.change_blocks.len() - 1;
        }

        if self.navigation_state.current_connector_index >= self.connector_curves.len()
            && !self.connector_curves.is_empty()
        {
            self.navigation_state.current_connector_index = self.connector_curves.len() - 1;
        }
    }

    pub fn set_viewport_height(&mut self, height: f32) {
        self.viewport_height = height;
    }

    pub fn update_scroll_offsets(&mut self, left: f32, right: f32) {
        self.left_scroll_offset = left;
        self.right_scroll_offset = right;
    }

    pub fn get_current_block(&self) -> Option<&ChangeBlock> {
        self.change_blocks
            .get(self.navigation_state.current_block_index)
    }

    pub fn get_current_connector(&self) -> Option<&ConnectorCurve> {
        self.connector_curves
            .get(self.navigation_state.current_connector_index)
    }

    pub fn navigate_to_block(&mut self, index: usize) {
        if index < self.change_blocks.len() {
            self.navigation_state.current_block_index = index;
        }
    }

    pub fn navigate_to_connector(&mut self, index: usize) {
        if index < self.connector_curves.len() {
            self.navigation_state.current_connector_index = index;
        }
    }

    pub fn next_block(&mut self) {
        if !self.change_blocks.is_empty() {
            self.navigation_state.current_block_index =
                (self.navigation_state.current_block_index + 1) % self.change_blocks.len();
        }
    }

    pub fn previous_block(&mut self) {
        if !self.change_blocks.is_empty() {
            self.navigation_state.current_block_index =
                if self.navigation_state.current_block_index == 0 {
                    self.change_blocks.len() - 1
                } else {
                    self.navigation_state.current_block_index - 1
                };
        }
    }

    pub fn is_empty(&self) -> bool {
        self.left_lines.is_empty() && self.right_lines.is_empty()
    }

    pub fn has_changes(&self) -> bool {
        !self.change_blocks.is_empty()
    }

    pub fn total_lines_left(&self) -> usize {
        self.left_lines.len()
    }

    pub fn total_lines_right(&self) -> usize {
        self.right_lines.len()
    }

    pub fn total_changes(&self) -> usize {
        self.change_blocks.len()
    }

    pub fn reset_navigation(&mut self) {
        self.navigation_state.current_block_index = 0;
        self.navigation_state.current_connector_index = 0;
    }

    pub fn clear(&mut self) {
        self.current_file.clear();
        self.left_lines.clear();
        self.right_lines.clear();
        self.change_blocks.clear();
        self.anchors.clear();
        self.mapping_segments.clear();
        self.connector_curves.clear();
        self.reset_navigation();
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
