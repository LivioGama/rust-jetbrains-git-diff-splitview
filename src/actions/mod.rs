// diffsplit/src/actions/mod.rs
// User actions module for handling application commands

use crate::git::GitOps;
use crate::navigation::NavigationAction;

/// Action result
#[derive(Debug, Clone)]
pub struct ActionResult {
    pub success: bool,
    pub message: String,
    pub details: Option<String>,
}

/// Action handler for processing user commands
pub struct ActionHandler {
    git_ops: GitOps,
}

impl ActionHandler {
    pub fn new(git_ops: GitOps) -> Self {
        Self { git_ops }
    }

    /// Execute an action based on the navigation action
    pub fn execute_action(&self, action: NavigationAction, block_index: usize) -> ActionResult {
        match action {
            NavigationAction::ApplyHunk => self.apply_hunk(block_index),
            NavigationAction::RevertHunk => self.revert_hunk(block_index),
            NavigationAction::StageHunk => self.stage_hunk(block_index),
            _ => ActionResult {
                success: true,
                message: "No action required".to_string(),
                details: None,
            },
        }
    }

    /// Apply a hunk at the specified block index
    fn apply_hunk(&self, block_index: usize) -> ActionResult {
        // TODO: Implement hunk application logic
        ActionResult {
            success: false,
            message: format!("Hunk application not implemented for block {}", block_index),
            details: Some("This feature requires integration with git apply".to_string()),
        }
    }

    /// Revert a hunk at the specified block index
    fn revert_hunk(&self, block_index: usize) -> ActionResult {
        // TODO: Implement hunk reversion logic
        ActionResult {
            success: false,
            message: format!("Hunk reversion not implemented for block {}", block_index),
            details: Some("This feature requires integration with git apply -R".to_string()),
        }
    }

    /// Stage a hunk at the specified block index
    fn stage_hunk(&self, block_index: usize) -> ActionResult {
        // TODO: Implement hunk staging logic
        ActionResult {
            success: false,
            message: format!("Hunk staging not implemented for block {}", block_index),
            details: Some("This feature requires integration with git add -p".to_string()),
        }
    }

    /// Get the status of the current file
    pub fn get_file_status(&self, file_path: &str) -> ActionResult {
        let result = self.git_ops.file_status(file_path);

        ActionResult {
            success: result.success,
            message: if result.success {
                if result.stdout.is_empty() {
                    "File is clean".to_string()
                } else {
                    format!("File status: {}", result.stdout.trim())
                }
            } else {
                format!("Failed to get file status: {}", result.stderr)
            },
            details: if result.success && !result.stdout.is_empty() {
                Some(result.stdout)
            } else {
                None
            },
        }
    }

    /// Check if the repository is clean
    pub fn check_repo_clean(&self) -> ActionResult {
        let result = self.git_ops.is_clean();

        ActionResult {
            success: result.success,
            message: if result.success {
                if result.stdout.is_empty() {
                    "Repository is clean".to_string()
                } else {
                    "Repository has uncommitted changes".to_string()
                }
            } else {
                format!("Failed to check repository status: {}", result.stderr)
            },
            details: if result.success && !result.stdout.is_empty() {
                Some(result.stdout)
            } else {
                None
            },
        }
    }
}

impl Default for ActionResult {
    fn default() -> Self {
        Self {
            success: false,
            message: "No action performed".to_string(),
            details: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::GitOps;

    #[test]
    fn test_action_handler_creation() {
        let git_ops = GitOps::default();
        let handler = ActionHandler::new(git_ops);
        // Test passes if handler is created successfully
    }

    #[test]
    fn test_action_result_default() {
        let result = ActionResult::default();
        assert!(!result.success);
        assert_eq!(result.message, "No action performed");
        assert!(result.details.is_none());
    }

    #[test]
    fn test_execute_no_action() {
        let git_ops = GitOps::default();
        let handler = ActionHandler::new(git_ops);
        let result = handler.execute_action(NavigationAction::None, 0);

        assert!(result.success);
        assert_eq!(result.message, "No action required");
    }
}
