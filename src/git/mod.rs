#[allow(dead_code)]
// diffsplit/src/git/mod.rs
// Git operations module
use std::process::Command;

/// Git operation result
#[derive(Debug, Clone)]
pub struct GitResult {
    pub success: bool,
    pub stdout: String,
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
            },
            Err(_e) => GitResult {
                success: false,
                stdout: String::new(),
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

    /// Get the list of changed files from git diff
    pub fn get_changed_files(
        &self,
        from_commit: Option<&str>,
        to_commit: Option<&str>,
    ) -> Vec<String> {
        let mut args = vec!["diff", "--name-only"];

        if let Some(from) = from_commit {
            args.push(from);
        }

        if let Some(to) = to_commit {
            args.push(to);
        }

        let result = self.execute_command(&args);
        if result.success {
            result.stdout.lines().map(|s| s.to_string()).collect()
        } else {
            Vec::new()
        }
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
        };

        assert!(result.success);
        assert_eq!(result.stdout, "output");
    }
}
