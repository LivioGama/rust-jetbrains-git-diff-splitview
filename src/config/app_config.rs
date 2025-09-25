// src/config/app_config.rs
// Application configuration for window and runtime settings

use gpui::*;

/// Application configuration for window and runtime settings
pub struct WindowConfig;

impl WindowConfig {
    /// Get GPUI window options for window setup
    pub fn get_window_options(cx: &gpui::App) -> gpui::WindowOptions {
        let bounds =
            gpui::Bounds::centered(None, gpui::size(gpui::px(1400.0), gpui::px(900.0)), cx);

        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("JetBrains Diff Viewer - GPUI".into()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    /// Get line height from config manager
    pub fn get_line_height(_config_manager: &crate::config::ConfigManager) -> f32 {
        // Use default line height for GPUI
        18.0 // Default monospace line height
    }
}
