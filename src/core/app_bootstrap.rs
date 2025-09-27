// src/core/app_bootstrap.rs
// Application bootstrap and initialization logic extracted from main.rs

use anyhow::Result;
use std::path::PathBuf;

use crate::actions::ActionHandler;
use crate::config::{ConfigManager, WindowConfig};
use crate::file_ops::FileOps;
use crate::git::GitOps;
use crate::state::StateManager;
use crate::sync;

/// Bootstrap data for the application
pub struct AppBootstrap {
    pub state_manager: StateManager,
    pub action_handler: ActionHandler,
    pub project_files: Vec<PathBuf>,
    pub config_manager: ConfigManager,
}

impl AppBootstrap {
    /// Initialize the application with all data loading and processing
    pub fn initialize() -> Result<Self> {
        println!("📊 Initializing Git operations and file operations...");
        let git_ops = GitOps::with_current_dir();
        let _file_ops = FileOps::with_default_config();

        println!("📋 Scanning for changed files in git diff...");
        let changed_files = git_ops.get_changed_files(None, None);
        let project_files: Vec<PathBuf> = changed_files.into_iter().map(PathBuf::from).collect();
        println!("📁 Found {} changed files", project_files.len());

        let config_manager = ConfigManager::new();
        let mut state_manager = StateManager::new();
        let action_handler = ActionHandler::new();

        if project_files.is_empty() {
            println!("⚠️ No changed files found, using default demo");
            load_demo_diff(&mut state_manager, &config_manager);
        } else {
            state_manager.update_state(|state| {
                state.current_file = "Git Diff Overview".to_string();
                state.left_lines = Vec::new();
                state.right_lines = Vec::new();
                state.change_blocks = Vec::new();
                state.imara_analysis = crate::diff::imara::ImaraDiffAnalysis { blocks: Vec::new() };
                state.anchors = Vec::new();
                state.mapping_segments = Vec::new();
                state.update_navigation_state();
            });
        }

        println!("✅ Application initialization complete, creating window...");

        Ok(Self {
            state_manager,
            action_handler,
            project_files,
            config_manager,
        })
    }
}

fn load_demo_diff(state_manager: &mut StateManager, config_manager: &ConfigManager) {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let left_path = manifest_dir.join("examples/ProvidersOld.tsx");
    let right_path = manifest_dir.join("examples/ProvidersNew.tsx");

    let fallback_original =
        "function App() {\n  return <div>Hello World</div>;\n}\n\nexport default App;\n";
    let fallback_current = "import React from 'react';\nimport { BrowserRouter as Router, Routes, Route } from 'react-router-dom';\nimport { ThemeProvider } from './theme';\nimport { AuthProvider } from './auth';\nimport { NotificationProvider } from './notifications';\n\nfunction AppProviders({ children }) {\n  return (\n    <ThemeProvider>\n      <AuthProvider>\n        <NotificationProvider>\n          <Router>\n            {children}\n          </Router>\n        </NotificationProvider>\n      </AuthProvider>\n    </ThemeProvider>\n  );\n}\n\nexport default AppProviders;\n";

    let demo_original = std::fs::read_to_string(&left_path).unwrap_or_else(|err| {
        eprintln!(
            "⚠️ Failed to read demo original from {:?}: {} — using fallback",
            left_path, err
        );
        fallback_original.to_string()
    });

    let demo_current = std::fs::read_to_string(&right_path).unwrap_or_else(|err| {
        eprintln!(
            "⚠️ Failed to read demo current from {:?}: {} — using fallback",
            right_path, err
        );
        fallback_current.to_string()
    });

    let demo_file = format!(
        "{} → {}",
        left_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("ProvidersOld.tsx"),
        right_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("ProvidersNew.tsx"),
    );

    let (left_lines, right_lines, change_blocks) =
        crate::diff::parser::create_complete_side_by_side_with_diff(
            &demo_original,
            &demo_current,
            "",
        );
    let imara_analysis =
        crate::diff::imara::compute_imara_diff_default(&demo_original, &demo_current);
    let connector_curves = sync::build_connector_curves(&imara_analysis);

    state_manager.update_state(move |state| {
        state.current_file = demo_file;
        state.left_lines = left_lines;
        state.right_lines = right_lines;
        state.change_blocks = change_blocks;
        state.imara_analysis = imara_analysis;
        let line_height = WindowConfig::get_line_height(config_manager);
        let anchors = sync::build_anchors_from_blocks(&state.change_blocks, line_height);
        let mapping_segments = sync::build_mapping_segments(&anchors);
        state.anchors = anchors;
        state.mapping_segments = mapping_segments;
        state.connector_curves = connector_curves;
        state.reset_navigation();
        state.left_scroll_offset = 0.0;
        state.right_scroll_offset = 0.0;
        state.update_navigation_state();
    });
}
