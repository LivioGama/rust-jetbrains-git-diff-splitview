// jetbrains_diff_step_by_step2/src/main.rs
// Main entry point for the JetBrains Diff Viewer
// Clean, modular architecture with separated concerns

use gpui::{App, AppContext, WindowOptions, WindowBounds, Pixels};

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

fn main() {
    println!("🚀 Starting JetBrains Diff Viewer - Modular Edition (GPUI)");

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

    App::new().run(|cx: &mut AppContext| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(gpui::Bounds {
                origin: gpui::Point { 
                    x: Pixels(0.0), 
                    y: Pixels(0.0) 
                },
                size: gpui::Size {
                    width: Pixels(1600.0),
                    height: Pixels(1000.0),
                },
            })),
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("JetBrains Diff Viewer - Modular".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            window_background: gpui::WindowBackgroundAppearance::Opaque,
            focus: true,
            show: true,
            kind: gpui::WindowKind::Normal,
            is_movable: true,
            is_resizable: true,
            is_minimizable: true,
            display_id: None,
            window_min_size: None,
            app_id: None,
            tabbing_identifier: None,
            window_decorations: Some(gpui::WindowDecorations::Server),
        };

        cx.open_window(window_options, |cx| {
            let bootstrap = crate::core::app_bootstrap::AppBootstrap::initialize()
                .expect("Failed to initialize application");
            
            DiffViewerApp::new(bootstrap)
        })
        .expect("Failed to create window");
    });
}
