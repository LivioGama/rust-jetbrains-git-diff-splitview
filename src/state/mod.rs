// diffsplit/src/state/mod.rs
// Application state management module

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
            imara_analysis: crate::diff::imara::ImaraDiffAnalysis {
                blocks: Vec::new(),
                line_mapping: Vec::new(),
                total_old_lines: 0,
                total_new_lines: 0,
            },
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

/// State manager for handling state transitions
pub struct StateManager {
    current_state: AppState,
    previous_states: Vec<AppState>,
    max_history_size: usize,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            current_state: AppState::new(),
            previous_states: Vec::new(),
            max_history_size: 10,
        }
    }

    pub fn get_current_state(&self) -> &AppState {
        &self.current_state
    }

    pub fn get_current_state_mut(&mut self) -> &mut AppState {
        &mut self.current_state
    }

    pub fn update_state<F>(&mut self, updater: F)
    where
        F: FnOnce(&mut AppState),
    {
        // Save current state to history before modification
        self.save_to_history();

        updater(&mut self.current_state);
    }

    pub fn save_to_history(&mut self) {
        self.previous_states.push(self.current_state.clone());

        // Keep history size within limits
        if self.previous_states.len() > self.max_history_size {
            self.previous_states.remove(0);
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.previous_states.is_empty()
    }

    pub fn undo(&mut self) -> bool {
        if let Some(previous_state) = self.previous_states.pop() {
            self.current_state = previous_state;
            true
        } else {
            false
        }
    }

    pub fn clear_history(&mut self) {
        self.previous_states.clear();
    }

    pub fn set_max_history_size(&mut self, size: usize) {
        self.max_history_size = size;

        // Trim history if needed
        while self.previous_states.len() > self.max_history_size {
            self.previous_states.remove(0);
        }
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::line::{DisplayLine, LineType};

    #[test]
    fn test_app_state_creation() {
        let state = AppState::new();
        assert!(state.is_empty());
        assert!(!state.has_changes());
    }

    #[test]
    fn test_app_state_with_lines() {
        let left_lines = vec![DisplayLine::new("test".to_string(), LineType::Context)];
        let right_lines = vec![DisplayLine::new("test".to_string(), LineType::Context)];

        let state = AppState::new().with_lines(left_lines, right_lines);
        assert!(!state.is_empty());
        assert_eq!(state.total_lines_left(), 1);
        assert_eq!(state.total_lines_right(), 1);
    }

    #[test]
    fn test_state_manager() {
        let mut manager = StateManager::new();

        // Test initial state
        assert!(!manager.can_undo());

        // Update state
        manager.update_state(|state| {
            state.current_file = "test.txt".to_string();
        });

        assert!(manager.can_undo());
        assert_eq!(manager.get_current_state().current_file, "test.txt");

        // Test undo
        let undo_success = manager.undo();
        assert!(undo_success);
        assert_eq!(manager.get_current_state().current_file, "");
        assert!(!manager.can_undo());
    }
}
