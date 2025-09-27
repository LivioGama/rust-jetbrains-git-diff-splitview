// Toolbar implementation for GPUI - Simplified
use gpui::*;
use std::path::PathBuf;

/// Toolbar actions that can be performed
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolbarAction {
    Previous,
    Next,
    Default,
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum DiffMode {
    ProjectFile,
    DefaultDemo,
}

pub struct ToolbarState {
    pub current_file_index: usize,
    pub project_files: Vec<PathBuf>,
    pub current_mode: DiffMode,
}

impl ToolbarState {
    pub fn new(project_files: Vec<PathBuf>) -> Self {
        let filtered_files: Vec<PathBuf> = project_files
            .into_iter()
            .filter(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| {
                        let lower = name.to_ascii_lowercase();
                        lower != ".ds_store" && lower != "ds_store"
                    })
                    .unwrap_or(true)
            })
            .collect();

        let current_mode = if filtered_files.is_empty() {
            DiffMode::DefaultDemo
        } else {
            DiffMode::ProjectFile
        };

        Self {
            current_file_index: 0,
            project_files: filtered_files,
            current_mode,
        }
    }

    pub fn can_go_previous(&self) -> bool {
        matches!(self.current_mode, DiffMode::ProjectFile) && self.current_file_index > 0
    }

    pub fn can_go_next(&self) -> bool {
        matches!(self.current_mode, DiffMode::ProjectFile)
            && self.current_file_index < self.project_files.len().saturating_sub(1)
    }

    pub fn current_file(&self) -> Option<&PathBuf> {
        match self.current_mode {
            DiffMode::ProjectFile => self.project_files.get(self.current_file_index),
            DiffMode::DefaultDemo => None,
        }
    }

    pub fn go_previous(&mut self) {
        if self.can_go_previous() {
            self.current_file_index -= 1;
        }
    }

    pub fn go_next(&mut self) {
        if self.can_go_next() {
            self.current_file_index += 1;
        }
    }

    pub fn go_to_default(&mut self) {
        self.current_mode = DiffMode::DefaultDemo;
    }
}

pub struct ToolbarHandler {
    state: ToolbarState,
}

impl ToolbarHandler {
    pub fn new(project_files: Vec<PathBuf>) -> Self {
        Self {
            state: ToolbarState::new(project_files),
        }
    }

    pub fn get_state(&self) -> &ToolbarState {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut ToolbarState {
        &mut self.state
    }

    pub fn handle_input(&self, _cx: &gpui::App) -> ToolbarAction {
        // Simplified input handling - just return None for now
        ToolbarAction::None
    }

    pub fn render_toolbar(&self, _cx: &gpui::App) -> ToolbarAction {
        // Simplified toolbar rendering - just return None for now
        ToolbarAction::None
    }
}

impl Default for ToolbarHandler {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
