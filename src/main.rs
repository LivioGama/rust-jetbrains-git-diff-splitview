// diffsplit/src/main.rs
// Modular JetBrains Git Diff Viewer
use eframe::egui;

// Module declarations
mod app;
mod config;
mod diff;
mod file_ops;
mod models;
mod navigation;
mod sync;
mod theme;
mod ui;

// Re-exports for convenience
use app::*;
use diff::*;
use file_ops::*;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 1000.0])
            .with_resizable(true),
        ..Default::default()
    };

    // Read complete files and apply diff highlighting using file_ops
    let file_ops = FileOps::with_default_config();
    let file_content = file_ops.read_all_content().unwrap_or_else(|e| {
        eprintln!("Error reading files: {}", e);
        FileContent {
            original_content: "Error reading original file".to_string(),
            current_content: "Error reading current file".to_string(),
            diff_text: "".to_string(),
        }
    });

    let original_content = file_content.original_content;
    let current_content = file_content.current_content;
    let diff_text = file_content.diff_text;

    // Create complete side-by-side display with diff highlighting
    let (old_lines, new_lines, change_blocks) =
        create_complete_side_by_side_with_diff(&original_content, &current_content, &diff_text);

    // Build enhanced data structures for better functionality
    let line_height = 18.0;
    let anchors = sync::build_anchors_from_blocks(&change_blocks, line_height);
    let mapping_segments = sync::build_mapping_segments(&anchors);

    // Run the application
    eframe::run_native(
        "JetBrains Diff Viewer - Modular",
        options,
        Box::new(|_cc| {
            Ok(Box::new(DiffViewerApp::new(
                old_lines,
                new_lines,
                change_blocks,
                anchors,
                mapping_segments,
            )))
        }),
    )
}
