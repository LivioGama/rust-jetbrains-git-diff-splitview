// diffsplit/src/app/mod.rs
use gpui::{Context, Window, IntoElement, Render, Styled, ParentElement, div};
use std::path::PathBuf;

use crate::actions::*;
use crate::config::*;
use crate::core::app_bootstrap::AppBootstrap;
use crate::navigation::*;
use crate::state::*;
use crate::sync::*;
use crate::theme::*;
use crate::toolbar::ToolbarHandler;
use crate::ui::layout::LayoutManager;
use crate::ui::{ConnectorRenderer, LineRenderer};

pub struct DiffViewerApp {
    pub state_manager: StateManager,
    pub action_handler: ActionHandler,
    pub scroll_sync: ScrollSync,
    pub theme: JetBrainsTheme,
    pub line_renderer: LineRenderer,
    pub connector_renderer: ConnectorRenderer,
    pub navigation_handler: NavigationHandler,
    pub layout_manager: LayoutManager,
    pub config_manager: ConfigManager,
    pub toolbar_handler: ToolbarHandler,
}

impl DiffViewerApp {
    pub fn new(bootstrap: AppBootstrap) -> Self {
        Self::create_from_bootstrap(bootstrap)
    }

    fn create_from_bootstrap(bootstrap: AppBootstrap) -> Self {
        let AppBootstrap {
            state_manager,
            action_handler,
            project_files,
        } = bootstrap;
        eprintln!("🎨 Creating DiffViewerApp...");

        // Initialize configuration with error handling
        let config_manager = ConfigManager::new();
        eprintln!("✅ Configuration manager created successfully");

        // Initialize theme with safe defaults
        let theme = JetBrainsTheme::dark_theme();
        eprintln!("✅ Theme initialized successfully");

        let line_height = theme.line_height(); // Use Zed's calculated line height
        let viewport_height = 1000.0;

        eprintln!("📏 Using line height: {}", line_height);

        let mut app = Self {
            state_manager,
            action_handler,
            scroll_sync: ScrollSync::new(line_height, viewport_height),
            theme: theme.clone(),
            line_renderer: LineRenderer::new(theme.clone()),
            connector_renderer: ConnectorRenderer::new(theme.clone()),
            navigation_handler: NavigationHandler::new(),
            layout_manager: LayoutManager::new(config_manager.get_config().layout.clone()),
            config_manager,
            toolbar_handler: ToolbarHandler::new(project_files),
        };

        // Load first project file's diff if there are project files
        if let Some(first_file) = app.toolbar_handler.get_state().current_file().cloned() {
            app.load_file_diff(&first_file);
        }

        app
    }

    fn handle_navigation_action(&mut self, action: NavigationAction) {
        let current_block = self
            .state_manager
            .get_current_state()
            .navigation_state
            .current_block_index;
        let result = self.action_handler.execute_action(action, current_block);

        // Handle the result silently
        if !result.success {
            eprintln!("Action failed: {}", result.message);
        }
    }

    fn handle_toolbar_action(&mut self, action: crate::toolbar::ToolbarAction) {
        match action {
            crate::toolbar::ToolbarAction::Previous | crate::toolbar::ToolbarAction::Next => {
                // Load the file at the current toolbar index
                let file_path = self.toolbar_handler.get_state().current_file().cloned();
                if let Some(file_path) = file_path {
                    self.load_file_diff(&file_path);
                }
            }
            crate::toolbar::ToolbarAction::Default => {
                // Always switch to demo
                self.toolbar_handler.get_state_mut().current_mode =
                    crate::toolbar::DiffMode::DefaultDemo;
                self.load_default_demo_diff();
            }
            crate::toolbar::ToolbarAction::None => {}
        }
    }

    fn load_file_diff(&mut self, file_path: &std::path::Path) {
        use crate::diff::imara::compute_imara_diff_default;
        use crate::diff::parser::create_complete_side_by_side_with_diff;
        use crate::git::GitOps;
        use crate::sync::{build_anchors_from_blocks, build_mapping_segments};

        eprintln!("Loading diff for file: {:?}", file_path);

        let git_ops = GitOps::with_current_dir();

        // Read current content from filesystem
        let current_content = match std::fs::read_to_string(file_path) {
            Ok(content) => {
                eprintln!("✅ Read current content ({} chars)", content.len());
                content
            }
            Err(e) => {
                eprintln!("❌ Failed to read current file ({}), using fallback", e);
                "function App() {\n  return <div>Hello World</div>;\n}\n\nexport default App;\n"
                    .to_string()
            }
        };

        // Get original content from git (last committed version)
        let original_content = match git_ops.show_file("HEAD", &file_path.to_string_lossy()) {
            crate::git::GitResult {
                success: true,
                stdout,
                ..
            } => {
                eprintln!("✅ Got original content from HEAD ({} chars)", stdout.len());
                stdout
            }
            _ => {
                eprintln!("❌ Failed to read original from git, using current as original");
                current_content.clone()
            }
        };

        // Get diff from current directory
        let diff_text = match git_ops.diff_file(None, None, &file_path.to_string_lossy()) {
            crate::git::GitResult {
                success: true,
                stdout,
                ..
            } => {
                eprintln!("✅ Got diff content ({} chars)", stdout.len());
                stdout
            }
            _ => {
                eprintln!("❌ Failed to read diff, using empty diff");
                "".to_string()
            }
        };

        // Create diff analysis
        let (old_lines, new_lines, change_blocks) =
            create_complete_side_by_side_with_diff(&original_content, &current_content, &diff_text);
        let imara_analysis = compute_imara_diff_default(&original_content, &current_content);

        // Build UI data
        let line_height = crate::config::WindowConfig::get_line_height(&self.config_manager);
        let anchors = build_anchors_from_blocks(&change_blocks, line_height);
        let mapping_segments = build_mapping_segments(&anchors);

        // Update state
        self.state_manager.update_state(|state| {
            state.current_file = file_path.to_string_lossy().to_string();
            state.left_lines = old_lines;
            state.right_lines = new_lines;
            state.change_blocks = change_blocks.to_vec();
            state.imara_analysis = imara_analysis;
            state.anchors = anchors;
            state.mapping_segments = mapping_segments;
            state.reset_navigation();
            state.left_scroll_offset = 0.0;
            state.right_scroll_offset = 0.0;
        });

        // Reset scroll positions
        self.scroll_sync.set_left_scroll(0.0);
        self.scroll_sync.set_right_scroll(0.0);
    }

    fn load_default_demo_diff(&mut self) {
        use crate::diff::imara::compute_imara_diff_default;
        use crate::diff::parser::create_complete_side_by_side_with_diff;
        use crate::git::GitOps;
        use crate::sync::{build_anchors_from_blocks, build_mapping_segments};

        eprintln!("Loading default demo diff");

        let git_ops = GitOps::new("/Users/livio/Documents/anbiti-apps".to_string());
        let file_path = "apps/app/app/Providers.tsx";

        // Read current content from HEAD
        let current_content = match git_ops.show_file("HEAD~1", file_path) {
            crate::git::GitResult {
                success: true,
                stdout,
                ..
            } => {
                eprintln!("✅ Got current content from HEAD ({} chars)", stdout.len());
                stdout
            }
            _ => {
                eprintln!("❌ Failed to read current from git, using fallback");
                "import React from 'react';\nimport { BrowserRouter as Router, Routes, Route } from 'react-router-dom';\nimport { ThemeProvider } from './theme';\nimport { AuthProvider } from './auth';\nimport { NotificationProvider } from './notifications';\n\nfunction AppProviders({ children }) {\n  return (\n    <ThemeProvider>\n      <AuthProvider>\n        <NotificationProvider>\n          <Router>\n            {children}\n          </Router>\n        </NotificationProvider>\n      </AuthProvider>\n    </ThemeProvider>\n  );\n}\n\nexport default AppProviders;\n".to_string()
            }
        };

        // Read original content from HEAD~1
        let original_content = match git_ops.show_file("cb2752b3", file_path) {
            crate::git::GitResult {
                success: true,
                stdout,
                ..
            } => {
                eprintln!(
                    "✅ Got original content from HEAD~1 ({} chars)",
                    stdout.len()
                );
                stdout
            }
            _ => {
                eprintln!("❌ Failed to read original from git, using fallback");
                "function App() {\n  return <div>Hello World</div>;\n}\n\nexport default App;\n"
                    .to_string()
            }
        };

        // Get diff between HEAD and HEAD~1 (reverse direction)
        let diff_text = match git_ops.diff_file(Some("HEAD"), Some("HEAD~1"), file_path) {
            crate::git::GitResult {
                success: true,
                stdout,
                ..
            } => {
                eprintln!("✅ Got diff content ({} chars)", stdout.len());
                stdout
            }
            _ => {
                eprintln!("❌ Failed to read diff, using fallback");
                "diff --git a/apps/app/app/Providers.tsx b/apps/app/app/Providers.tsx\nindex dcc2893b..cb2752b 100644\n--- a/apps/app/app/Providers.tsx\n+++ b/apps/app/app/Providers.tsx\n@@ -1,6 +1,5 @@\n import React from 'react';\n import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';\n import { ThemeProvider } from './theme';\n import { AuthProvider } from './auth';\n-import { NotificationProvider } from './notifications';\n\n function AppProviders({ children }) {\n   return (\n@@ -7,9 +6,6 @@\n       <AuthProvider>\n-        <NotificationProvider>\n           <Router>\n             {children}\n-          </Router>\n-        </NotificationProvider>\n       </AuthProvider>\n     </ThemeProvider>\n   );\n }\n".to_string()
            }
        };

        let (old_lines, new_lines, change_blocks) =
            create_complete_side_by_side_with_diff(&original_content, &current_content, &diff_text);
        let imara_analysis = compute_imara_diff_default(&original_content, &current_content);

        let line_height = crate::config::WindowConfig::get_line_height(&self.config_manager);
        let anchors = build_anchors_from_blocks(&change_blocks, line_height);
        let mapping_segments = build_mapping_segments(&anchors);

        self.state_manager.update_state(|state| {
            state.current_file = file_path.to_string();
            state.left_lines = old_lines;
            state.right_lines = new_lines;
            state.change_blocks = change_blocks.to_vec();
            state.imara_analysis = imara_analysis;
            state.anchors = anchors;
            state.mapping_segments = mapping_segments;
            state.reset_navigation();
            state.left_scroll_offset = 0.0;
            state.right_scroll_offset = 0.0;
        });

        self.scroll_sync.set_left_scroll(0.0);
        self.scroll_sync.set_right_scroll(0.0);
    }
}

impl Render for DiffViewerApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        // Update navigation state from current state
        let current_state = self.state_manager.get_current_state();
        self.navigation_handler.update_state(
            current_state.change_blocks.len(),
            current_state.connector_curves.len(),
        );

        // TODO: Handle keyboard navigation and toolbar input in gpui
        // This will require adapting the navigation and toolbar handlers to work with gpui events

        // For now, create a basic div structure
        div()
            .bg(gpui::rgb(0x1e1e1e)) // Dark background similar to JetBrains theme
            .w_full()
            .h_full()
            .child(
                div()
                    .w_full()
                    .h_full()
                    .child("DiffViewerApp - GPUI Migration in Progress")
            )
    }
}
