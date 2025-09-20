// jetbrains_diff_step_by_step2/src/main.rs
// Main entry point for the JetBrains Diff Viewer
// Clean, modular architecture with separated concerns

use eframe::egui;

// Module declarations
mod actions;
mod app;
mod config;
mod core;
mod diff;
mod file_ops;
mod git;
mod models;
mod navigation;
mod rendering;
mod state;
mod sync;
mod syntax;
mod theme;
mod ui;
mod utils;

// Re-exports for convenience
use actions::*;
use app::*;
use config::ConfigManager;
use diff::imara::compute_imara_diff_default;
use diff::parser::create_complete_side_by_side_with_diff;
use diff::*;
use file_ops::FileOps;
use git::{GitOps, GitResult};

use state::*;

fn main() -> Result<(), eframe::Error> {
    println!("🚀 Starting JetBrains Diff Viewer - Modular Edition");

    // Set up panic handler for better error reporting
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("💥 Application panicked: {}", panic_info);
        if let Some(location) = panic_info.location() {
            eprintln!(
                "📍 Location: {}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            );
        }
    }));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 1000.0])
            .with_resizable(true)
            .with_visible(true)
            .with_transparent(false)
            .with_decorations(true)
            .with_window_level(egui::WindowLevel::Normal),
        centered: true,
        // Add hardware acceleration settings for better compatibility
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..Default::default()
    };

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
    let line_height = config_manager
        .get_config()
        .fonts
        .calculated_buffer_line_height();
    let anchors = sync::build_anchors_from_blocks(&change_blocks, line_height);
    let mapping_segments = sync::build_mapping_segments(&anchors);

    // Git operations already initialized above

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

    // Run the application
    println!(
        "🎨 About to create window with {} old lines and {} new lines",
        old_lines.len(),
        new_lines.len()
    );
    println!("✅ Application initialization complete, creating window...");

    // Apply font configuration before creating the application
    eframe::run_native(
        "JetBrains Diff Viewer - Modular",
        options,
        Box::new(move |cc| {
            // Apply Zed font configuration to the egui context
            config_manager
                .get_font_manager()
                .apply_to_context(&cc.egui_ctx);

            // Create the application
            Ok(Box::new(DiffViewerApp::new(state_manager, action_handler)))
        }),
    )
}
