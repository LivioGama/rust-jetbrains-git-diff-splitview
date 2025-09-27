// jetbrains_diff_step_by_step2/src/toolbar.rs
// Toolbar navigation module for diff viewer

use std::path::PathBuf;

/// Diff mode for navigation
#[derive(Debug, Clone, PartialEq)]
pub enum DiffMode {
    ProjectFile(usize), // index into file list
    DefaultDemo,        // legacy hardcoded demo diff
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

    pub fn go_previous(&mut self) {
        match self.current_mode {
            DiffMode::ProjectFile(ref mut index) => {
                if self.project_files.is_empty() {
                    return;
                }
                if *index == 0 {
                    *index = self.project_files.len() - 1;
                } else {
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
                if self.project_files.is_empty() {
                    return;
                }
                *index = (*index + 1) % self.project_files.len();
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
        assert!(matches!(state.current_mode, DiffMode::ProjectFile(0)));
    }

    #[test]
    fn test_toolbar_navigation() {
        let files = vec![
            PathBuf::from("file1.txt"),
            PathBuf::from("file2.txt"),
            PathBuf::from("file3.txt"),
        ];
        let mut state = ToolbarState::new(files);

        state.go_next();
        assert!(matches!(state.current_mode, DiffMode::ProjectFile(1)));

        state.go_next();
        assert!(matches!(state.current_mode, DiffMode::ProjectFile(2)));

        state.go_previous();
        assert!(matches!(state.current_mode, DiffMode::ProjectFile(1)));
    }

    #[test]
    fn test_default_mode() {
        let files = vec![PathBuf::from("file1.txt")];
        let mut state = ToolbarState::new(files);

        state.go_to_default();
        assert_eq!(state.current_mode, DiffMode::DefaultDemo);
        state.go_next();
        assert!(matches!(state.current_mode, DiffMode::ProjectFile(0)));
    }
}
