// src/core/app_bootstrap.rs
// Application bootstrap and initialization logic extracted from main.rs

use crate::actions::ActionHandler;
use crate::app::DiffViewerApp;
use crate::config::{ConfigManager, WindowConfig};

use crate::file_ops::FileOps;
use crate::git::GitOps;
use crate::state::StateManager;
use crate::sync;

use std::{collections::HashSet, path::PathBuf};

/// Bootstrap data for the application
pub struct AppBootstrap {
    pub state_manager: StateManager,
    pub project_files: Vec<PathBuf>,
}

impl AppBootstrap {
    /// Initialize the application with all data loading and processing
    pub fn initialize() -> Result<Self, Box<dyn std::error::Error>> {
        println!("📊 Initializing Git operations and file operations...");
        let git_ops = GitOps::with_current_dir();
        let _file_ops = FileOps::with_default_config();

        // Get list of changed files from git diff
        println!("📋 Scanning for changed files in git diff...");
        let changed_files = git_ops.get_changed_files(None, None);
        let mut seen = HashSet::new();
        let project_files: Vec<PathBuf> = changed_files
            .into_iter()
            .filter_map(|path| {
                let trimmed = path.trim();
                if trimmed.is_empty() {
                    return None;
                }
                let cleaned = trimmed.trim_matches(|c: char| c == '"' || c.is_whitespace());
                if cleaned.is_empty() {
                    return None;
                }
                let path_buf = PathBuf::from(cleaned);
                let file_name = path_buf
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name.to_ascii_lowercase());
                if matches!(file_name.as_deref(), Some(".ds_store") | Some("ds_store")) {
                    return None;
                }
                Some(path_buf)
            })
            .filter(|path| seen.insert(path.clone()))
            .collect();
        println!("📁 Found {} changed files", project_files.len());

        // Initialize configuration manager with Zed font specifications first
        let config_manager = ConfigManager::new();

        // Initialize state manager early
        let mut state_manager = StateManager::new();

        // Determine which file to load
        let (_file_path, _original_commit, _current_path) = if !project_files.is_empty() {
            let first_file = &project_files[0];
            let file_path_str = first_file.to_string_lossy().to_string();
            println!("🎯 Loading first changed file: {}", file_path_str);
            (file_path_str, None::<&str>, first_file.clone())
        } else {
            println!("⚠️ No changed files found, using default demo");
            // Use hardcoded demo content directly
            let demo_file = "demo.tsx";
            let demo_original =
                "function App() {\n  return <div>Hello World</div>;\n}\n\nexport default App;\n"
                    .to_string();
            let demo_current = "import React from 'react';\nimport { BrowserRouter as Router, Routes, Route } from 'react-router-dom';\nimport { ThemeProvider } from './theme';\nimport { AuthProvider } from './auth';\nimport { NotificationProvider } from './notifications';\n\nfunction AppProviders({ children }) {\n  return (\n    <ThemeProvider>\n      <AuthProvider>\n        <NotificationProvider>\n          <Router>\n            {children}\n          </Router>\n        </NotificationProvider>\n      </AuthProvider>\n    </ThemeProvider>\n  );\n}\n\nexport default AppProviders;\n\n// Additional test lines to ensure scrolling works\nfunction TestComponent() {\n  return (\n    <div>\n      <h1>Test Line 1</h1>\n      <p>This is test content to ensure scrolling works properly.</p>\n      <h2>Test Line 2</h2>\n      <p>More test content for scrolling verification.</p>\n      <h3>Test Line 3</h3>\n      <p>Even more content to test the scroll functionality.</p>\n      <h4>Test Line 4</h4>\n      <p>Final test content to ensure we have enough lines.</p>\n    </div>\n  );\n}\n".to_string();

            // Skip file reading and use demo content directly
            state_manager.update_state(|state| {
                state.current_file = demo_file.to_string();
                state.left_lines = crate::diff::parser::create_complete_side_by_side_with_diff(
                    &demo_original,
                    &demo_original,
                    "",
                )
                .0;
                state.right_lines = crate::diff::parser::create_complete_side_by_side_with_diff(
                    &demo_original,
                    &demo_current,
                    "",
                )
                .1;
                let (_, _, change_blocks) =
                    crate::diff::parser::create_complete_side_by_side_with_diff(
                        &demo_original,
                        &demo_current,
                        "",
                    );
                state.change_blocks = change_blocks;
                state.imara_analysis =
                    crate::diff::imara::compute_imara_diff_default(&demo_original, &demo_current);
                let line_height = WindowConfig::get_line_height(&config_manager);
                let anchors = sync::build_anchors_from_blocks(&state.change_blocks, line_height);
                let mapping_segments = sync::build_mapping_segments(&anchors);
                state.anchors = anchors;
                state.mapping_segments = mapping_segments;
            });

            println!("🎨 About to create window with demo content");
            println!("✅ Application initialization complete, creating window...");

            return Ok(Self {
                state_manager,
                project_files,
            });
        };

        // Initialize the application state
        state_manager.update_state(|state| {
            state.current_file = "Git Diff Overview".to_string();
            state.left_lines = Vec::new();
            state.right_lines = Vec::new();
            state.change_blocks = Vec::new();
            state.imara_analysis = crate::diff::imara::ImaraDiffAnalysis { blocks: Vec::new() };
            state.anchors = Vec::new();
            state.mapping_segments = Vec::new();
        });
        println!("✅ Application initialization complete, creating window...");

        Ok(Self {
            state_manager,
            project_files,
        })
    }

    /// Create the GPUI application instance
    pub fn create_app(self) -> DiffViewerApp {
        DiffViewerApp::new(self.state_manager, ActionHandler::new(), self.project_files)
    }
}
