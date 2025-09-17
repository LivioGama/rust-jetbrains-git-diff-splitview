// diffsplit/src/file_ops/mod.rs
// File operations and Git integration module

use std::fs;
use std::process::Command;

/// Configuration for file operations
#[derive(Debug, Clone)]
pub struct FileConfig {
    pub git_repo_path: String,
    pub file_path: String,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            git_repo_path: "/Users/livio/Documents/anbiti-apps".to_string(),
            file_path: "apps/app/app/Providers.tsx".to_string(),
        }
    }
}

/// Result of reading file content with git information
#[derive(Debug, Clone)]
pub struct FileContent {
    pub original_content: String,
    pub current_content: String,
    pub diff_text: String,
}

/// File operations handler
pub struct FileOps {
    config: FileConfig,
}

impl FileOps {
    pub fn new(config: FileConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(FileConfig::default())
    }

    /// Read the original file content from git HEAD
    pub fn read_original_content(&self) -> Result<String, String> {
        let command = format!(
            "cd {} && git show HEAD:{}",
            self.config.git_repo_path, self.config.file_path
        );

        match Command::new("sh").arg("-c").arg(&command).output() {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Err(format!(
                        "Git command failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
            Err(e) => Err(format!("Failed to execute git command: {}", e)),
        }
    }

    /// Read the current file content from the filesystem
    pub fn read_current_content(&self) -> Result<String, String> {
        let full_path = format!("{}/{}", self.config.git_repo_path, self.config.file_path);

        match fs::read_to_string(&full_path) {
            Ok(content) => Ok(content),
            Err(e) => Err(format!("Failed to read file {}: {}", full_path, e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_config_default() {
        let config = FileConfig::default();
        assert!(!config.git_repo_path.is_empty());
        assert!(!config.file_path.is_empty());
    }

    #[test]
    fn test_file_ops_creation() {
        let config = FileConfig::default();
        let file_ops = FileOps::new(config);
        assert_eq!(
            file_ops.get_config().git_repo_path,
            "/Users/livio/Documents/anbiti-apps"
        );
    }
}
