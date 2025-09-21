// src/config/app_config.rs
// Application configuration extracted from main.rs

use eframe::egui;

/// Application configuration for window and runtime settings
pub struct WindowConfig;

impl WindowConfig {
    /// Get eframe native options for window setup (extracted from main.rs lines 53-65)
    pub fn get_window_options() -> eframe::NativeOptions {
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1600.0, 1000.0])
                .with_resizable(true)
                .with_visible(true)
                .with_transparent(false)
                .with_decorations(cfg!(not(any(
                    target_os = "ios",
                    target_os = "android",
                    target_arch = "wasm32"
                ))))
                .with_window_level(egui::WindowLevel::Normal),
            centered: true,
            // Add hardware acceleration settings for better compatibility
            hardware_acceleration: eframe::HardwareAcceleration::Preferred,
            ..Default::default()
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
