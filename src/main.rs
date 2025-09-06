// diffsplit/src/main.rs
// Modular JetBrains Git Diff Viewer
use eframe::egui;

// Module declarations
mod app;
mod diff;
mod models;
mod sync;
mod theme;
mod ui;

// Re-exports for convenience
use app::*;
use diff::*;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 1000.0])
            .with_resizable(true),
        ..Default::default()
    };

    // Read complete files and apply diff highlighting
    let original_content = std::process::Command::new("git")
        .arg("show")
        .arg("HEAD:src/main.rs")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        .unwrap_or_else(|_| "Error reading original main.rs".to_string());

    let current_content = std::fs::read_to_string("src/main.rs")
        .unwrap_or_else(|_| "Error reading current main.rs".to_string());

    // Get git diff to identify changes
    let diff_text = std::process::Command::new("git")
        .arg("diff")
        .arg("HEAD")
        .arg("--")
        .arg("src/main.rs")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        .unwrap_or_else(|_| "".to_string());

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
            Box::new(DiffViewerApp::new(
                old_lines,
                new_lines,
                change_blocks,
                anchors,
                mapping_segments,
            ))
        }),
    )
}
