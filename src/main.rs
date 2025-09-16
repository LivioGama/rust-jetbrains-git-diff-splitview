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
mod theme;
mod ui;
mod utils;

// Re-exports for convenience
use actions::*;
use app::*;
use config::*;
use diff::*;
use file_ops::*;
use git::{GitOps, GitResult};
use models::*;
use navigation::*;
use rendering::*;
use state::*;
use sync::*;
use theme::*;
use ui::*;
use utils::*;

fn main() -> Result<(), eframe::Error> {
    println!("🚀 Starting JetBrains Diff Viewer - Modular Edition");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 1000.0])
            .with_resizable(true)
            .with_visible(true)
            .with_transparent(false)
            .with_decorations(true)
            .with_window_level(egui::WindowLevel::Normal),
        centered: true,
        ..Default::default()
    };

    println!("📊 Initializing Git operations and file operations...");
    let git_ops = GitOps::new("/Users/livio/Documents/anbiti-apps/".to_string());
    let file_ops = FileOps::with_default_config();
    println!("✅ Git operations initialized successfully");

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
            println!("✅ Got current content ({} chars)", content.len());
            content
        }
        Err(e) => {
            println!("❌ Failed to read current file: {}, using fallback", e);
            "function App() {\n  return <div>Hello World - Modified</div>;\n}\n\nexport default App;\n".to_string()
        }
    };

    // Try to get Git diff, with fallback
    println!("🔍 Getting Git diff...");
    let diff_text = match git_ops.diff_file(
        Some("cb2752b3"),
        Some("dcc2893b"),
        "apps/app/app/Providers.tsx",
    ) {
        GitResult {
            success: true,
            stdout,
            ..
        } => {
            println!("✅ Got Git diff ({} chars)", stdout.len());
            stdout
        }
        _ => {
            println!("❌ Failed to get Git diff, trying fallback...");
            match file_ops.get_git_diff() {
                Ok(diff) => {
                    println!("✅ Using fallback diff ({} chars)", diff.len());
                    diff
                }
                Err(_) => {
                    println!("❌ Fallback diff also failed, using empty diff");
                    "".to_string()
                }
            }
        }
    };

    // Create complete side-by-side display with diff highlighting
    println!("⚙️ Processing diff...");
    let (old_lines, new_lines, change_blocks) =
        create_complete_side_by_side_with_diff(&original_content, &current_content, &diff_text);

    println!(
        "✅ Diff processed - old_lines: {}, new_lines: {}, change_blocks: {}",
        old_lines.len(),
        new_lines.len(),
        change_blocks.len()
    );

    // Build enhanced data structures for better functionality
    let line_height = 18.0;
    let anchors = sync::build_anchors_from_blocks(&change_blocks, line_height);
    let mapping_segments = sync::build_mapping_segments(&anchors);

    // Initialize state manager and action handler
    let mut state_manager = StateManager::new();
    let action_handler = ActionHandler::new(git_ops);

    // Initialize the application state
    state_manager.update_state(|state| {
        state.current_file = "apps/app/app/Providers.tsx".to_string();
        state.left_lines = old_lines.clone();
        state.right_lines = new_lines.clone();
        state.change_blocks = change_blocks;
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

    eframe::run_native(
        "JetBrains Diff Viewer - Modular",
        options,
        Box::new(move |_cc| {
            println!("✅ Window creation callback called successfully");
            Ok(Box::new(DiffViewerApp::new(state_manager, action_handler)))
        }),
    )
}
