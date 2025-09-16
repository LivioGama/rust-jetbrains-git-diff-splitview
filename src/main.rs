// diffsplit/src/main.rs
// Modular JetBrains Git Diff Viewer
use eframe::egui;

// Module declarations
mod actions;
mod app;
mod config;
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
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 1000.0])
            .with_resizable(true),
        ..Default::default()
    };

    // Initialize Git operations and file operations
    let git_ops = GitOps::with_current_dir();
    let file_ops = FileOps::with_default_config();

    // Read original file content from Git
    let original_content =
        match git_ops.show_file("HEAD", "apps/reflecta/app/api/completion/route.ts") {
            GitResult {
                success: true,
                stdout,
                ..
            } => stdout,
            _ => {
                eprintln!("Warning: Could not read original file from Git, using fallback");
                file_ops
                    .read_original_content()
                    .unwrap_or_else(|_| "Error reading original file".to_string())
            }
        };

    // Read current file content
    let current_content = file_ops
        .read_current_content()
        .unwrap_or_else(|_| "Error reading current file".to_string());

    // Get Git diff
    let diff_text = match git_ops.diff_file(
        None,
        Some("HEAD"),
        "apps/reflecta/app/api/completion/route.ts",
    ) {
        GitResult {
            success: true,
            stdout,
            ..
        } => stdout,
        _ => {
            eprintln!("Warning: Could not get Git diff, using fallback");
            file_ops.get_git_diff().unwrap_or_else(|_| "".to_string())
        }
    };

    // Create complete side-by-side display with diff highlighting
    let (old_lines, new_lines, change_blocks) =
        create_complete_side_by_side_with_diff(&original_content, &current_content, &diff_text);

    // Build enhanced data structures for better functionality
    let line_height = 18.0;
    let anchors = sync::build_anchors_from_blocks(&change_blocks, line_height);
    let mapping_segments = sync::build_mapping_segments(&anchors);

    // Initialize state manager and action handler
    let mut state_manager = StateManager::new();
    let action_handler = ActionHandler::new(git_ops);

    // Initialize the application state
    state_manager.update_state(|state| {
        state.current_file = "apps/reflecta/app/api/completion/route.ts".to_string();
        state.left_lines = old_lines;
        state.right_lines = new_lines;
        state.change_blocks = change_blocks;
        state.anchors = anchors;
        state.mapping_segments = mapping_segments;
    });

    // Run the application
    eframe::run_native(
        "JetBrains Diff Viewer - Modular",
        options,
        Box::new(move |_cc| Ok(Box::new(DiffViewerApp::new(state_manager, action_handler)))),
    )
}
