// jetbrains_diff_step_by_step2/src/main.rs
// Main entry point for the JetBrains Diff Viewer - GPUI Implementation
// Clean, modular architecture with separated concerns

// GPUI imports
use gpui::*;

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
use crate::config::WindowConfig;

fn main() {
    println!("🚀 Starting JetBrains Diff Viewer - GPUI Modular Edition");

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

    // Initialize the application using the bootstrap
    let bootstrap = crate::core::app_bootstrap::AppBootstrap::initialize()
        .expect("Failed to initialize application");

    // Create the app instance
    let app = bootstrap.create_app();

    // Run the GPUI application
    Application::new().run(|cx: &mut App| {
        let window_options = WindowConfig::get_window_options(cx);
        cx.open_window(window_options, |_, cx| cx.new(|_| app))
            .unwrap();
        cx.activate(true);
    });
}
