// jetbrains_diff_step_by_step2/src/main.rs
// Main entry point for the JetBrains Diff Viewer
// Clean, modular architecture with separated concerns

mod actions;
mod app;
mod config;
mod core;
mod diff;
mod file_ops;
mod git;
mod gpui;
mod models;
mod navigation;
mod state;
mod sync;
mod toolbar;
mod utils;

use anyhow::Result;
use config::WindowConfig;

fn main() -> Result<()> {
    println!("🚀 Starting JetBrains Diff Viewer - GPUI edition");

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

    let window = WindowConfig::default_window_descriptor();
    let bootstrap = crate::core::app_bootstrap::AppBootstrap::initialize()?;
    let app = crate::app::DiffViewerApp::from_bootstrap(bootstrap)?;
    crate::gpui::runtime::run(app, window)
}
