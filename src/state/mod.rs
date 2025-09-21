// diffsplit/src/state/mod.rs
// Application state management module

pub mod app_state;
pub mod state_manager;

pub use state_manager::*;

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
