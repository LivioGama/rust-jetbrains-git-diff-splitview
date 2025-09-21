// jetbrains_diff_step_by_step2/src/main.rs
// Main entry point for the JetBrains Diff Viewer
// Clean, modular architecture with separated concerns

// Removed unused import: use eframe::egui;

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
mod toolbar;
mod ui;
mod utils;

// Re-exports for convenience
use config::WindowConfig;

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

    let options = WindowConfig::get_window_options();

    // Initialize the application using the bootstrap
    let bootstrap = crate::core::app_bootstrap::AppBootstrap::initialize()?;

    // Run the application
    eframe::run_native(
        "JetBrains Diff Viewer - Modular",
        options,
        bootstrap.create_app_callback(),
    )
}
