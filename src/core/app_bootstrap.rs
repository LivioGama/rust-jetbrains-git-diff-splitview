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

/// Bootstrap data for the application
pub struct AppBootstrap {
    pub state_manager: StateManager,
    pub action_handler: ActionHandler,
}

impl AppBootstrap {
    /// Initialize the application with all data loading and processing
    pub fn initialize() -> Result<Self, eframe::Error> {
        println!("📊 Initializing Git operations and file operations...");
        let git_ops = GitOps::new("/Users/livio/Documents/anbiti-apps/".to_string());
        let file_ops = FileOps::with_default_config();

        // Initialize configuration manager with Zed font specifications first
        let config_manager = ConfigManager::new();

        // Try to read original file content from Git, with fallback
        println!("📖 Reading original file content from Git...");
        let original_content = match git_ops.show_file("cb2752b3", "apps/app/app/Providers.tsx") {
            GitResult {
                success: true,
                stdout,
                ..
            } => {
                println!("✅ Got original content ({} chars)", stdout.len());
                stdout
            }
            _ => {
                println!("❌ Failed to read original, trying fallback...");
                match file_ops.read_original_content() {
                    Ok(content) => {
                        println!("✅ Using fallback content ({} chars)", content.len());
                        content
                    }
                    Err(e) => {
                        println!("❌ Fallback also failed ({}), using default content", e);
                        "function App() {\n  return <div>Hello World</div>;\n}\n\nexport default App;\n"
                            .to_string()
                    }
                }
            }
        };

        // Try to read current file content, with fallback
        println!("📖 Reading current file content...");
        let current_content = match std::fs::read_to_string(
            "/Users/livio/Documents/anbiti-apps/apps/app/app/Providers.tsx",
        ) {
            Ok(content) => {
                println!("✅ Read current content ({} chars)", content.len());
                content
            }
            Err(e) => {
                println!("❌ Failed to read current file ({}), using fallback", e);
                match file_ops.read_current_content() {
                    Ok(content) => {
                        println!("✅ Using fallback content ({} chars)", content.len());
                        content
                    }
                    Err(e2) => {
                        println!("❌ Fallback also failed ({}), using default", e2);
                        "function App() {\n  return <div>Hello World</div>;\n}\n\nexport default App;\n"
                            .to_string()
                    }
                }
            }
        };

        // Try to get diff from Git, with fallback
        println!("📖 Reading diff content...");
        let diff_text = match git_ops.diff_file(
            Some("cb2752b3"),
            Some("HEAD"),
            "apps/app/app/Providers.tsx",
        ) {
            GitResult {
                success: true,
                stdout,
                ..
            } => {
                println!("✅ Got diff content ({} chars)", stdout.len());
                stdout
            }
            _ => {
                println!("❌ Failed to read diff, using default");
                "diff --git a/apps/app/app/Providers.tsx b/apps/app/app/Providers.tsx\nindex cb2752b..dcc2893b 100644\n--- a/apps/app/app/Providers.tsx\n+++ b/apps/app/app/Providers.tsx\n@@ -1,5 +1,6 @@\n import React from 'react';\n import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';\n import { ThemeProvider } from './theme';\n import { AuthProvider } from './auth';\n+import { NotificationProvider } from './notifications';\n\n function AppProviders({ children }) {\n   return (\n@@ -7,6 +8,9 @@\n       <AuthProvider>\n+        <NotificationProvider>\n           <Router>\n             {children}\n+          </Router>\n+        </NotificationProvider>\n       </AuthProvider>\n     </ThemeProvider>\n   );\n }\n"
                    .to_string()
            }
        };

        // Create side-by-side display with imara-diff semantic analysis
        let (old_lines, new_lines, change_blocks) =
            create_complete_side_by_side_with_diff(&original_content, &current_content, &diff_text);

        // Generate imara-diff analysis for semantic blocks
        let imara_analysis = compute_imara_diff_default(&original_content, &current_content);

        println!("🎯 Imara-diff semantic blocks:");
        for (i, block) in imara_analysis.blocks.iter().enumerate() {
            if block.is_change() {
                println!(
                    "   📦 Block {}: left {}–{}, right {}–{}, op: {:?}, similarity: {:?}",
                    i + 1,
                    block.left_range.start,
                    block.left_range.end,
                    block.right_range.start,
                    block.right_range.end,
                    block.operation,
                    block.semantic_similarity
                );
            }
        }

        // Build enhanced data structures for better functionality
        // Use Zed's golden ratio line height (1.618)
        let line_height = WindowConfig::get_line_height(&config_manager);
        let anchors = sync::build_anchors_from_blocks(&change_blocks, line_height);
        let mapping_segments = sync::build_mapping_segments(&anchors);

        // Initialize state manager and action handler
        let mut state_manager = StateManager::new();
        let action_handler = ActionHandler::new();

        // Initialize the application state
        state_manager.update_state(|state| {
            state.current_file = "apps/app/app/Providers.tsx".to_string();
            state.left_lines = old_lines.clone();
            state.right_lines = new_lines.clone();
            state.change_blocks = change_blocks;
            state.imara_analysis = imara_analysis.clone();
            state.anchors = anchors;
            state.mapping_segments = mapping_segments;
        });

        println!(
            "🎨 About to create window with {} old lines and {} new lines",
            old_lines.len(),
            new_lines.len()
        );
        println!("✅ Application initialization complete, creating window...");

        Ok(Self {
            state_manager,
            action_handler,
        })
    }

    /// Create the eframe application callback
    pub fn create_app_callback(
        self,
    ) -> Box<dyn FnOnce(&eframe::CreationContext<'_>) -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>>> {
        Box::new(move |cc| {
            // Apply Zed font configuration to the egui context
            let config_manager = ConfigManager::new();
            config_manager
                .get_font_manager()
                .apply_to_context(&cc.egui_ctx);

            // Create the application
            Ok(Box::new(DiffViewerApp::new(self.state_manager, self.action_handler)))
        })
    }
}
