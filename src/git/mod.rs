// diffsplit/src/git/mod.rs
// Git operations module

use std::path::Path;
use std::process::Command;

/// Git operation result
#[derive(Debug, Clone)]
pub struct GitResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

/// Git repository operations
pub struct GitOps {
    repo_path: String,
}

impl GitOps {
    pub fn new(repo_path: String) -> Self {
        Self { repo_path }
    }

    pub fn with_current_dir() -> Self {
        Self::new(".".to_string())
    }

    /// Execute a git command and return the result
    fn execute_command(&self, args: &[&str]) -> GitResult {
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.repo_path)
            .output();

        match output {
            Ok(output) => GitResult {
                success: output.status.success(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                exit_code: output.status.code(),
            },
            Err(e) => GitResult {
                success: false,
                stdout: String::new(),
                stderr: format!("Failed to execute git command: {}", e),
                exit_code: None,
            },
        }
    }

    /// Get the content of a file at a specific commit
    pub fn show_file(&self, commit: &str, file_path: &str) -> GitResult {
        self.execute_command(&["show", &format!("{}:{}", commit, file_path)])
    }

    /// Get the diff between two commits for a specific file
    pub fn diff_file(
        &self,
        from_commit: Option<&str>,
        to_commit: Option<&str>,
        file_path: &str,
    ) -> GitResult {
        let mut args = vec!["diff"];

        if let Some(from) = from_commit {
            args.push(from);
        }

        if let Some(to) = to_commit {
            args.push(to);
        }

        args.push("--");
        args.push(file_path);

        self.execute_command(&args)
    }

    /// Get the current HEAD commit
    pub fn get_head_commit(&self) -> GitResult {
        self.execute_command(&["rev-parse", "HEAD"])
    }

    /// Check if the repository is clean (no uncommitted changes)
    pub fn is_clean(&self) -> GitResult {
        self.execute_command(&["status", "--porcelain"])
    }

    /// Get the status of a specific file
    pub fn file_status(&self, file_path: &str) -> GitResult {
        self.execute_command(&["status", "--porcelain", file_path])
    }

    /// Stage a file
    pub fn stage_file(&self, file_path: &str) -> GitResult {
        self.execute_command(&["add", file_path])
    }

    /// Unstage a file
    pub fn unstage_file(&self, file_path: &str) -> GitResult {
        self.execute_command(&["reset", "HEAD", file_path])
    }

    /// Apply a hunk (placeholder - would need more complex implementation)
    pub fn apply_hunk(&self, hunk_content: &str) -> GitResult {
        // This would need to create a patch and apply it
        // For now, just return an error
        GitResult {
            success: false,
            stdout: String::new(),
            stderr: "Hunk application not implemented".to_string(),
            exit_code: Some(1),
        }
    }

    /// Get repository information
    pub fn get_repo_info(&self) -> GitResult {
        self.execute_command(&["remote", "-v"])
    }

    /// Check if a file exists in the repository
    pub fn file_exists(&self, file_path: &str) -> bool {
        Path::new(&self.repo_path).join(file_path).exists()
    }

    /// Get the relative path from repo root
    pub fn get_relative_path(&self, absolute_path: &str) -> Option<String> {
        let repo_path = Path::new(&self.repo_path);
        let abs_path = Path::new(absolute_path);

        abs_path
            .strip_prefix(repo_path)
            .ok()
            .and_then(|p| p.to_str())
            .map(|s| s.to_string())
    }
}

impl Default for GitOps {
    fn default() -> Self {
        Self::with_current_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_ops_creation() {
        let git_ops = GitOps::new("/path/to/repo".to_string());
        assert_eq!(git_ops.repo_path, "/path/to/repo");
    }

    #[test]
    fn test_git_ops_default() {
        let git_ops = GitOps::default();
        assert_eq!(git_ops.repo_path, ".");
    }

    #[test]
    fn test_git_result_structure() {
        let result = GitResult {
            success: true,
            stdout: "output".to_string(),
            stderr: String::new(),
            exit_code: Some(0),
        };

        assert!(result.success);
        assert_eq!(result.stdout, "output");
        assert_eq!(result.exit_code, Some(0));
    }
}
