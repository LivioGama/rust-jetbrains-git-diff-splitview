use crate::git::{GitOps, GitResult};
use crate::file_ops::FileOps;
use crate::diff::create_complete_side_by_side_with_diff;
use crate::models::line::DisplayLine;
use crate::models::diff::ChangeBlock;
use crate::sync;

/// Configuration for data loading
#[derive(Debug, Clone)]
pub struct DataLoaderConfig {
    pub git_repo_path: String,
    pub file_path: String,
    pub original_commit: String,
    pub current_commit: Option<String>,
}

impl Default for DataLoaderConfig {
    fn default() -> Self {
        Self {
            git_repo_path: "/Users/livio/Documents/anbiti-apps/".to_string(),
            file_path: "apps/app/app/Providers.tsx".to_string(),
            original_commit: "cb2752b3".to_string(),
            current_commit: Some("dcc2893b".to_string()),
        }
    }
}

/// Handles all data loading operations for the diff viewer
pub struct DataLoader {
    config: DataLoaderConfig,
    git_ops: GitOps,
    file_ops: FileOps,
}

impl DataLoader {
    pub fn new(config: DataLoaderConfig) -> Self {
        let git_ops = GitOps::new(config.git_repo_path.clone());
        let file_ops = FileOps::with_default_config();

        Self {
            config,
            git_ops,
            file_ops,
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(DataLoaderConfig::default())
    }

    /// Load all data needed for the diff viewer
    pub fn load_data(&self) -> Result<DiffData, DataLoadError> {
        println!("Loading diff data...");

        // Load original content
        let original_content = self.load_original_content()?;

        // Load current content
        let current_content = self.load_current_content()?;

        // Load diff text
        let diff_text = self.load_diff_text()?;

        // Process the diff
        let (old_lines, new_lines, change_blocks) = self.process_diff(
            &original_content,
            &current_content,
            &diff_text,
        )?;

        // Build additional data structures
        let line_height = 18.0;
        let anchors = sync::build_anchors_from_blocks(&change_blocks, line_height);
        let mapping_segments = sync::build_mapping_segments(&anchors);

        Ok(DiffData {
            file_path: self.config.file_path.clone(),
            original_content,
            current_content,
            diff_text,
            old_lines,
            new_lines,
            change_blocks,
            anchors,
            mapping_segments,
        })
    }

    fn load_original_content(&self) -> Result<String, DataLoadError> {
        println!("Reading original file content from Git...");
        match self.git_ops.show_file(&self.config.original_commit, &self.config.file_path) {
            GitResult { success: true, stdout, .. } => {
                println!("✓ Got original content ({} chars)", stdout.len());
                Ok(stdout)
            }
            _ => {
                println!("✗ Failed to read original, trying fallback...");
                match self.file_ops.read_original_content() {
                    Ok(content) => {
                        println!("Using fallback content ({} chars)", content.len());
                        Ok(content)
                    }
                    Err(e) => {
                        println!("Fallback also failed ({}), using default content", e);
                        Ok("function App() {\n  return <div>Hello World</div>;\n}\n\nexport default App;\n".to_string())
                    }
                }
            }
        }
    }

    fn load_current_content(&self) -> Result<String, DataLoadError> {
        println!("Reading current file content...");
        match std::fs::read_to_string(&self.config.file_path) {
            Ok(content) => {
                println!("✓ Got current content ({} chars)", content.len());
                Ok(content)
            }
            Err(e) => {
                println!("✗ Failed to read current file: {}, using fallback", e);
                Ok("function App() {\n  return <div>Hello World - Modified</div>;\n}\n\nexport default App;\n".to_string())
            }
        }
    }

    fn load_diff_text(&self) -> Result<String, DataLoadError> {
        println!("Getting Git diff...");
        let current_commit = self.config.current_commit.as_deref().unwrap_or("HEAD");

        match self.git_ops.diff_file(
            Some(&self.config.original_commit),
            Some(current_commit),
            &self.config.file_path,
        ) {
            GitResult { success: true, stdout, .. } => {
                println!("✓ Got Git diff ({} chars)", stdout.len());
                Ok(stdout)
            }
            _ => {
                println!("✗ Failed to get Git diff, trying fallback...");
                match self.file_ops.get_git_diff() {
                    Ok(diff) => {
                        println!("Using fallback diff ({} chars)", diff.len());
                        Ok(diff)
                    }
                    Err(_) => {
                        println!("Fallback diff also failed, using empty diff");
                        Ok("".to_string())
                    }
                }
            }
        }
    }

    fn process_diff(
        &self,
        original: &str,
        current: &str,
        diff_text: &str,
    ) -> Result<(Vec<DisplayLine>, Vec<DisplayLine>, Vec<ChangeBlock>), DataLoadError> {
        println!("Processing diff...");
        let result = create_complete_side_by_side_with_diff(original, current, diff_text);
        println!(
            "✓ Diff processed - old_lines: {}, new_lines: {}, change_blocks: {}",
            result.0.len(),
            result.1.len(),
            result.2.len()
        );
        Ok(result)
    }
}

/// Represents the loaded diff data
#[derive(Debug, Clone)]
pub struct DiffData {
    pub file_path: String,
    pub original_content: String,
    pub current_content: String,
    pub diff_text: String,
    pub old_lines: Vec<DisplayLine>,
    pub new_lines: Vec<DisplayLine>,
    pub change_blocks: Vec<ChangeBlock>,
    pub anchors: Vec<crate::models::diff::AnchorPoint>,
    pub mapping_segments: Vec<crate::models::diff::MappingSegment>,
}

/// Errors that can occur during data loading
#[derive(Debug, Clone)]
pub enum DataLoadError {
    GitError(String),
    FileError(String),
    DiffProcessingError(String),
}
