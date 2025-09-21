// src/core/app_bootstrap.rs
// Application bootstrap and initialization logic extracted from main.rs

use crate::actions::ActionHandler;
use crate::app::DiffViewerApp;
use crate::config::{ConfigManager, WindowConfig};
use crate::diff::imara::compute_imara_diff_default;
use crate::diff::parser::create_complete_side_by_side_with_diff;
use crate::file_ops::FileOps;
use crate::git::{GitOps, GitResult};
use crate::state::StateManager;
use crate::sync;

use std::path::PathBuf;

/// Bootstrap data for the application
pub struct AppBootstrap {
    pub state_manager: StateManager,
    pub action_handler: ActionHandler,
    pub project_files: Vec<PathBuf>,
}

impl AppBootstrap {
    /// Initialize the application with all data loading and processing
    pub fn initialize() -> Result<Self, eframe::Error> {
        println!("📊 Initializing Git operations and file operations...");
        let git_ops = GitOps::with_current_dir();
        let file_ops = FileOps::with_default_config();

        // Get list of changed files from git diff
        println!("📋 Scanning for changed files in git diff...");
        let changed_files = git_ops.get_changed_files(None, None);
        let project_files: Vec<PathBuf> = changed_files.into_iter().map(PathBuf::from).collect();
        println!("📁 Found {} changed files", project_files.len());

        // Initialize configuration manager with Zed font specifications first
        let config_manager = ConfigManager::new();

        // Initialize state manager and action handler early
        let mut state_manager = StateManager::new();
        let action_handler = ActionHandler::new();

        // Determine which file to load
        let (file_path, original_commit, current_path) = if !project_files.is_empty() {
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
            let demo_current = "import React from 'react';\nimport { BrowserRouter as Router, Routes, Route } from 'react-router-dom';\nimport { ThemeProvider } from './theme';\nimport { AuthProvider } from './auth';\nimport { NotificationProvider } from './notifications';\n\nfunction AppProviders({ children }) {\n  return (\n    <ThemeProvider>\n      <AuthProvider>\n        <NotificationProvider>\n          <Router>\n            {children}\n          </Router>\n        </NotificationProvider>\n      </AuthProvider>\n    </ThemeProvider>\n  );\n}\n\nexport default AppProviders;\n".to_string();

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
                action_handler,
                project_files,
            });
        };

        // Initialize the application state
        state_manager.update_state(|state| {
            state.current_file = "Git Diff Overview".to_string();
            state.left_lines = Vec::new();
            state.right_lines = Vec::new();
            state.change_blocks = Vec::new();
            state.imara_analysis = crate::diff::imara::ImaraDiffAnalysis {
                blocks: Vec::new(),
                line_mapping: Vec::new(),
                total_old_lines: 0,
                total_new_lines: 0,
            };
            state.anchors = Vec::new();
            state.mapping_segments = Vec::new();
        });
        println!("✅ Application initialization complete, creating window...");

        Ok(Self {
            state_manager,
            action_handler,
            project_files,
        })
    }

    /// Create the eframe application callback
    pub fn create_app_callback(
        self,
    ) -> Box<
        dyn FnOnce(
            &eframe::CreationContext<'_>,
        )
            -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>>,
    > {
        Box::new(move |cc| {
            // Apply Zed font configuration to the egui context
            let config_manager = ConfigManager::new();
            config_manager
                .get_font_manager()
                .apply_to_context(&cc.egui_ctx);

            // Create the application
            Ok(Box::new(DiffViewerApp::new(
                self.state_manager,
                self.action_handler,
                self.project_files,
            )))
        })
    }
}
