// src/state/state_manager.rs
// State manager extracted from state/mod.rs

use crate::state::app_state::AppState;

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
