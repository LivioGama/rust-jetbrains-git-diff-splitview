// src/config/app_config.rs
// Application configuration extracted from main.rs

/// Basic window descriptor for the custom GPUI runtime
#[derive(Debug, Clone, Copy)]
pub struct WindowDescriptor {
    pub width: u32,
    pub height: u32,
    pub min_width: u32,
    pub min_height: u32,
}

impl Default for WindowDescriptor {
    fn default() -> Self {
        Self {
            width: 1600,
            height: 1000,
            min_width: 960,
            min_height: 720,
        }
    }
}

/// Application configuration for window and runtime settings
pub struct WindowConfig;

impl WindowConfig {
    pub fn default_window_descriptor() -> WindowDescriptor {
        WindowDescriptor::default()
    }

    /// Get line height from config manager
    pub fn get_line_height(config_manager: &crate::config::ConfigManager) -> f32 {
        config_manager.get_font_manager().buffer_line_height()
    }
}
