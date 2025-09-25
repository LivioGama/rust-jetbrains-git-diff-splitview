use gpui::*;

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

/// Navigation handler for managing navigation state
pub struct NavigationHandler {
    state: NavigationState,
    current_block_index: usize,
}

impl NavigationHandler {
    pub fn new() -> Self {
        Self {
            state: NavigationState::default(),
            current_block_index: 0,
        }
    }

    pub fn get_state(&self) -> &NavigationState {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut NavigationState {
        &mut self.state
    }

    pub fn update_state(&mut self, total_blocks: usize, total_connectors: usize) {
        self.state.total_blocks = total_blocks;
        self.state.total_connectors = total_connectors;
    }

    pub fn current_block_index(&self) -> usize {
        self.current_block_index
    }

    pub fn navigate_to_next_diff_block(&mut self) {
        if self.state.total_blocks > 0 {
            self.current_block_index = (self.current_block_index + 1) % self.state.total_blocks;
        }
    }

    pub fn navigate_to_previous_diff_block(&mut self) {
        if self.state.total_blocks > 0 {
            if self.current_block_index == 0 {
                self.current_block_index = self.state.total_blocks - 1;
            } else {
                self.current_block_index -= 1;
            }
        }
    }

    pub fn handle_input(&self, _cx: &gpui::App) -> NavigationAction {
        // Simplified input handling - just return None for now
        NavigationAction::None
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
