// jetbrains_diff_step_by_step2/src/toolbar.rs
// Toolbar navigation module for diff viewer

use eframe::egui;
use std::path::PathBuf;

/// Diff mode for navigation
#[derive(Debug, Clone, PartialEq)]
pub enum DiffMode {
    ProjectFile(usize), // index into file list
    DefaultDemo,        // legacy hardcoded demo diff
}

/// Toolbar actions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolbarAction {
    Previous,
    Next,
    Default,
    None,
}

/// Toolbar state
#[derive(Debug, Clone)]
pub struct ToolbarState {
    pub project_files: Vec<PathBuf>,
    pub current_mode: DiffMode,
}

impl ToolbarState {
    pub fn new(project_files: Vec<PathBuf>) -> Self {
        let current_mode = if project_files.is_empty() {
            DiffMode::DefaultDemo
        } else {
            DiffMode::ProjectFile(0)
        };

        Self {
            project_files,
            current_mode,
        }
    }

    pub fn current_index(&self) -> Option<usize> {
        match self.current_mode {
            DiffMode::ProjectFile(index) => Some(index),
            DiffMode::DefaultDemo => None,
        }
    }

    pub fn can_go_previous(&self) -> bool {
        match self.current_mode {
            DiffMode::ProjectFile(index) => index > 0,
            DiffMode::DefaultDemo => !self.project_files.is_empty(),
        }
    }

    pub fn can_go_next(&self) -> bool {
        match self.current_mode {
            DiffMode::ProjectFile(index) => index < self.project_files.len().saturating_sub(1),
            DiffMode::DefaultDemo => !self.project_files.is_empty(),
        }
    }

    pub fn go_previous(&mut self) {
        match self.current_mode {
            DiffMode::ProjectFile(ref mut index) => {
                if *index > 0 {
                    *index -= 1;
                }
            }
            DiffMode::DefaultDemo => {
                if !self.project_files.is_empty() {
                    let last_index = self.project_files.len() - 1;
                    self.current_mode = DiffMode::ProjectFile(last_index);
                }
            }
        }
    }

    pub fn go_next(&mut self) {
        match self.current_mode {
            DiffMode::ProjectFile(ref mut index) => {
                if *index < self.project_files.len().saturating_sub(1) {
                    *index += 1;
                }
            }
            DiffMode::DefaultDemo => {
                if !self.project_files.is_empty() {
                    self.current_mode = DiffMode::ProjectFile(0);
                }
            }
        }
    }

    pub fn go_to_default(&mut self) {
        self.current_mode = DiffMode::DefaultDemo;
    }

    pub fn current_file(&self) -> Option<&PathBuf> {
        match self.current_mode {
            DiffMode::ProjectFile(index) => self.project_files.get(index),
            DiffMode::DefaultDemo => None,
        }
    }
}

/// Toolbar handler for UI and keyboard input
pub struct ToolbarHandler {
    state: ToolbarState,
}

impl ToolbarHandler {
    pub fn new(project_files: Vec<PathBuf>) -> Self {
        Self {
            state: ToolbarState::new(project_files),
        }
    }

    /// Handle keyboard input and return toolbar action
    pub fn handle_input(&mut self, ctx: &egui::Context) -> ToolbarAction {
        // Ctrl+Shift+Left for Previous
        if ctx
            .input(|i| i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::ArrowLeft))
        {
            self.state.go_previous();
            ToolbarAction::Previous
        }
        // Ctrl+Shift+Right for Next
        else if ctx.input(|i| {
            i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::ArrowRight)
        }) {
            self.state.go_next();
            ToolbarAction::Next
        }
        // 'd' key for Demo
        else if ctx.input(|i| i.key_pressed(egui::Key::D)) {
            self.state.go_to_default();
            ToolbarAction::Default
        } else {
            ToolbarAction::None
        }
    }

    /// Render the toolbar UI
    pub fn render_toolbar(&mut self, ui: &mut egui::Ui) -> ToolbarAction {
        let mut action = ToolbarAction::None;

        ui.horizontal(|ui| {
            // Previous button
            let prev_enabled = self.state.can_go_previous();
            if ui
                .add_enabled(prev_enabled, egui::Button::new("⬅ Previous"))
                .clicked()
            {
                self.state.go_previous();
                action = ToolbarAction::Previous;
            }

            // Next button
            let next_enabled = self.state.can_go_next();
            if ui
                .add_enabled(next_enabled, egui::Button::new("Next ➡"))
                .clicked()
            {
                self.state.go_next();
                action = ToolbarAction::Next;
            }

            ui.separator();

            // Demo button
            if ui.button("Demo").clicked() {
                self.state.go_to_default();
                action = ToolbarAction::Default;
            }

            ui.separator();

            // Current file display
            match &self.state.current_mode {
                DiffMode::ProjectFile(index) => {
                    if let Some(file) = self.state.project_files.get(*index) {
                        ui.label(format!(
                            "File: {} ({}/{})",
                            file.display(),
                            index + 1,
                            self.state.project_files.len()
                        ));
                    }
                }
                DiffMode::DefaultDemo => {
                    ui.label("Demo Diff");
                }
            }
        });

        action
    }

    /// Get the current state
    pub fn get_state(&self) -> &ToolbarState {
        &self.state
    }

    /// Get mutable state for updates
    pub fn get_state_mut(&mut self) -> &mut ToolbarState {
        &mut self.state
    }
}

impl Default for ToolbarHandler {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_toolbar_state_creation() {
        let files = vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];
        let state = ToolbarState::new(files.clone());
        assert_eq!(state.project_files, files);
        assert_eq!(state.current_index(), Some(0));
    }

    #[test]
    fn test_toolbar_navigation() {
        let files = vec![
            PathBuf::from("file1.txt"),
            PathBuf::from("file2.txt"),
            PathBuf::from("file3.txt"),
        ];
        let mut state = ToolbarState::new(files);

        assert!(!state.can_go_previous()); // false initially
        assert!(state.can_go_next()); // true

        state.go_next();
        assert_eq!(state.current_index(), Some(1));
        assert!(state.can_go_previous());
        assert!(state.can_go_next());

        state.go_next();
        assert_eq!(state.current_index(), Some(2));
        assert!(state.can_go_previous());
        assert!(!state.can_go_next());

        state.go_previous();
        assert_eq!(state.current_index(), Some(1));
    }

    #[test]
    fn test_default_mode() {
        let files = vec![PathBuf::from("file1.txt")];
        let mut state = ToolbarState::new(files);

        state.go_to_default();
        assert_eq!(state.current_mode, DiffMode::DefaultDemo);
        assert_eq!(state.current_index(), None);
        assert!(!state.can_go_previous());
        assert!(!state.can_go_next());
    }
}
