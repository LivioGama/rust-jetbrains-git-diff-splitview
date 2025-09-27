// jetbrains_diff_step_by_step2/src/main.rs
// Main entry point for the JetBrains Diff Viewer
// Clean, modular architecture with separated concerns

use gpui::{App, AppContext, Pixels, WindowBounds, WindowOptions};

// Module declarations
mod actions;
mod app;
mod compat; // Compatibility layer for egui -> gpui migration
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

use app::DiffViewerApp;
use core::app_bootstrap::AppBootstrap;
use config::WindowConfig;

fn main() {
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

    let bootstrap = AppBootstrap::initialize().expect("Failed to initialize app");
    let app = DiffViewerApp::new(bootstrap);

    App::new().run(move |cx| {
        cx.open_window(WindowConfig::get_window_options(), |cx| app);
    });
}
