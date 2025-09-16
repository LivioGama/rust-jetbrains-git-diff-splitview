// diffsplit/src/config/mod.rs
// Application configuration module

use eframe::egui;
use std::collections::HashMap;

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub window: WindowConfig,
    pub theme: ThemeConfig,
    pub layout: LayoutConfig,
    pub navigation: NavigationConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            theme: ThemeConfig::default(),
            layout: LayoutConfig::default(),
            navigation: NavigationConfig::default(),
        }
    }
}

/// Window configuration
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub initial_size: (f32, f32),
    pub resizable: bool,
    pub min_size: Option<(f32, f32)>,
    pub max_size: Option<(f32, f32)>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "JetBrains Diff Viewer - Modular".to_string(),
            initial_size: (1600.0, 1000.0),
            resizable: true,
            min_size: Some((800.0, 600.0)),
            max_size: None,
        }
    }
}

/// Theme configuration
#[derive(Debug, Clone)]
pub struct ThemeConfig {
    pub dark_mode: bool,
    pub font_size: f32,
    pub line_height: f32,
    pub font_family: String,
    pub custom_colors: HashMap<String, egui::Color32>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        let mut custom_colors = HashMap::new();
        custom_colors.insert("background".to_string(), egui::Color32::from_rgb(43, 43, 43));
        custom_colors.insert("foreground".to_string(), egui::Color32::from_rgb(255, 255, 255));
        custom_colors.insert("connector_column".to_string(), egui::Color32::from_rgb(60, 60, 60));

        Self {
            dark_mode: true,
            font_size: 14.0,
            line_height: 18.0,
            font_family: "JetBrains Mono".to_string(),
            custom_colors,
        }
    }
}

/// Layout configuration
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    pub connector_column_width: f32,
    pub pane_padding: f32,
    pub header_height: f32,
    pub scroll_area_padding: f32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            connector_column_width: 45.0,
            pane_padding: 10.0,
            header_height: 30.0,
            scroll_area_padding: 5.0,
        }
    }
}

/// Navigation configuration
#[derive(Debug, Clone)]
pub struct NavigationConfig {
    pub key_bindings: HashMap<String, String>,
    pub scroll_sensitivity: f32,
    pub auto_scroll: bool,
}

impl Default for NavigationConfig {
    fn default() -> Self {
        let mut key_bindings = HashMap::new();
        key_bindings.insert("next_diff".to_string(), "ArrowDown".to_string());
        key_bindings.insert("prev_diff".to_string(), "ArrowUp".to_string());
        key_bindings.insert("next_connector".to_string(), "ArrowRight".to_string());
        key_bindings.insert("prev_connector".to_string(), "ArrowLeft".to_string());
        key_bindings.insert("apply_hunk".to_string(), "Enter".to_string());
        key_bindings.insert("revert_hunk".to_string(), "Backspace".to_string());
        key_bindings.insert("stage_hunk".to_string(), "Space".to_string());

        Self {
            key_bindings,
            scroll_sensitivity: 1.0,
            auto_scroll: true,
        }
    }
}

/// Configuration manager
pub struct ConfigManager {
    config: AppConfig,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            config: AppConfig::default(),
        }
    }

    pub fn with_config(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    pub fn get_config_mut(&mut self) -> &mut AppConfig {
        &mut self.config
    }

    pub fn update_config<F>(&mut self, updater: F)
    where
        F: FnOnce(&mut AppConfig),
    {
        updater(&mut self.config);
    }

    /// Load configuration from a file (placeholder for future implementation)
    pub fn load_from_file(_path: &str) -> Result<Self, String> {
        // TODO: Implement configuration file loading
        Err("Configuration file loading not implemented yet".to_string())
    }

    /// Save configuration to a file (placeholder for future implementation)
    pub fn save_to_file(&self, _path: &str) -> Result<(), String> {
        // TODO: Implement configuration file saving
        Err("Configuration file saving not implemented yet".to_string())
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.window.title, "JetBrains Diff Viewer - Modular");
        assert_eq!(config.window.initial_size, (1600.0, 1000.0));
        assert!(config.theme.dark_mode);
    }

    #[test]
    fn test_config_manager() {
        let manager = ConfigManager::new();
        let config = manager.get_config();
        assert_eq!(config.layout.connector_column_width, 45.0);
    }

    #[test]
    fn test_config_update() {
        let mut manager = ConfigManager::new();
        manager.update_config(|config| {
            config.window.title = "Updated Title".to_string();
        });

        assert_eq!(manager.get_config().window.title, "Updated Title");
    }
}
