// diffsplit/src/navigation/mod.rs
// Keyboard navigation module for diff viewer

use eframe::egui;

/// Navigation actions that can be performed
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavigationAction {
    NextDiffBlock,
    PreviousDiffBlock,
    NextConnector,
    PreviousConnector,
    ApplyHunk,
    RevertHunk,
    StageHunk,
    None,
}

/// Navigation state
#[derive(Debug, Clone, Default)]
pub struct NavigationState {
    pub current_block_index: usize,
    pub total_blocks: usize,
    pub current_connector_index: usize,
    pub total_connectors: usize,
}

/// Navigation handler for keyboard input
pub struct NavigationHandler {
    state: NavigationState,
}

impl NavigationHandler {
    pub fn new() -> Self {
        Self {
            state: NavigationState::default(),
        }
    }

    /// Handle keyboard input and return the appropriate navigation action
    pub fn handle_input(&mut self, ctx: &egui::Context) -> NavigationAction {
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            self.navigate_to_next_diff_block();
            NavigationAction::NextDiffBlock
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            self.navigate_to_previous_diff_block();
            NavigationAction::PreviousDiffBlock
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            self.navigate_to_next_connector();
            NavigationAction::NextConnector
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            self.navigate_to_previous_connector();
            NavigationAction::PreviousConnector
        } else if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            NavigationAction::ApplyHunk
        } else if ctx.input(|i| i.key_pressed(egui::Key::Backspace)) {
            NavigationAction::RevertHunk
        } else if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            NavigationAction::StageHunk
        } else {
            NavigationAction::None
        }
    }

    /// Update the navigation state with current data
    pub fn update_state(&mut self, total_blocks: usize, total_connectors: usize) {
        self.state.total_blocks = total_blocks;
        self.state.total_connectors = total_connectors;

        // Ensure current indices are within bounds
        if self.state.current_block_index >= total_blocks && total_blocks > 0 {
            self.state.current_block_index = total_blocks - 1;
        }

        if self.state.current_connector_index >= total_connectors && total_connectors > 0 {
            self.state.current_connector_index = total_connectors - 1;
        }
    }

    /// Get the current navigation state
    pub fn get_state(&self) -> &NavigationState {
        &self.state
    }

    /// Get the current block index
    pub fn current_block_index(&self) -> usize {
        self.state.current_block_index
    }

    /// Get the current connector index
    pub fn current_connector_index(&self) -> usize {
        self.state.current_connector_index
    }

    /// Set the current block index
    pub fn set_current_block_index(&mut self, index: usize) {
        if index < self.state.total_blocks {
            self.state.current_block_index = index;
        }
    }

    /// Set the current connector index
    pub fn set_current_connector_index(&mut self, index: usize) {
        if index < self.state.total_connectors {
            self.state.current_connector_index = index;
        }
    }

    fn navigate_to_next_diff_block(&mut self) {
        if self.state.total_blocks > 0 {
            self.state.current_block_index =
                (self.state.current_block_index + 1) % self.state.total_blocks;
        }
    }

    fn navigate_to_previous_diff_block(&mut self) {
        if self.state.total_blocks > 0 {
            self.state.current_block_index = if self.state.current_block_index == 0 {
                self.state.total_blocks - 1
            } else {
                self.state.current_block_index - 1
            };
        }
    }

    fn navigate_to_next_connector(&mut self) {
        if self.state.total_connectors > 0 {
            self.state.current_connector_index =
                (self.state.current_connector_index + 1) % self.state.total_connectors;
        }
    }

    fn navigate_to_previous_connector(&mut self) {
        if self.state.total_connectors > 0 {
            self.state.current_connector_index = if self.state.current_connector_index == 0 {
                self.state.total_connectors - 1
            } else {
                self.state.current_connector_index - 1
            };
        }
    }
}

impl Default for NavigationHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_handler_creation() {
        let handler = NavigationHandler::new();
        let state = handler.get_state();
        assert_eq!(state.current_block_index, 0);
        assert_eq!(state.total_blocks, 0);
    }

    #[test]
    fn test_navigation_state_update() {
        let mut handler = NavigationHandler::new();
        handler.update_state(5, 3);

        let state = handler.get_state();
        assert_eq!(state.total_blocks, 5);
        assert_eq!(state.total_connectors, 3);
    }

    #[test]
    fn test_block_navigation() {
        let mut handler = NavigationHandler::new();
        handler.update_state(3, 0);

        // Test next navigation
        handler.navigate_to_next_diff_block();
        assert_eq!(handler.current_block_index(), 1);

        handler.navigate_to_next_diff_block();
        assert_eq!(handler.current_block_index(), 2);

        // Test wrap around
        handler.navigate_to_next_diff_block();
        assert_eq!(handler.current_block_index(), 0);

        // Test previous navigation
        handler.navigate_to_previous_diff_block();
        assert_eq!(handler.current_block_index(), 2);
    }
}
