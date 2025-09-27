// src/config/app_config.rs
// Application configuration extracted from main.rs

// GPUI migration: eframe not needed

use gpui::{
    Bounds, Pixels, Point, Size, TitlebarOptions, WindowBackgroundAppearance, WindowBounds,
    WindowDecorations, WindowKind, WindowOptions,
};

/// Application configuration for window and runtime settings
pub struct WindowConfig;

impl WindowConfig {
    /// Get GPUI window options for window setup
    pub fn get_window_options() -> WindowOptions {
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: Point {
                    x: Pixels(0.0),
                    y: Pixels(0.0),
                },
                size: Size {
                    width: Pixels(1600.0),
                    height: Pixels(1000.0),
                },
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("JetBrains Diff Viewer".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            window_background: WindowBackgroundAppearance::Opaque,
            focus: true,
            show: true,
            kind: WindowKind::Normal,
            is_movable: true,
            display_id: None,
            window_min_size: None,
            app_id: None,
            window_decorations: Some(WindowDecorations::Server),
        }
    }

    /// Get line height from config manager (extracted from main.rs lines 173-176)
    pub fn get_line_height(config_manager: &crate::config::ConfigManager) -> f32 {
        config_manager
            .get_config()
            .fonts
            .calculated_buffer_line_height()
    }
}
