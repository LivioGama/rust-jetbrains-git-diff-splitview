// Configuration module
pub mod app_config;
pub mod editor_settings;

// Re-export for convenience
pub use app_config::*;
// Editor settings module (currently unused)
// pub use editor_settings::*;

/// Layout configuration for the diff viewer
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    pub pane_padding: f32,
    pub connector_column_width: f32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            pane_padding: 8.0,
            connector_column_width: 90.0,
        }
    }
}

/// Configuration manager for the entire application
#[derive(Debug, Clone)]
pub struct ConfigManager {
    pub layout: LayoutConfig,
}

impl ConfigManager {
    /// Create a new configuration manager with default settings
    pub fn new() -> Self {
        let config = ConfigManager {
            layout: LayoutConfig::default(),
        };

        config
    }

    /// Get reference to the layout configuration
    pub fn get_config(&self) -> &ConfigManager {
        self
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
        assert_eq!(config.layout.connector_column_width, 45.0);
        assert_eq!(config.fonts.buffer_font_size, 15.0);
        assert_eq!(config.fonts.ui_font_size, 16.0);
        assert_eq!(config.editor.language.tab_size, 4);
        assert!(!config.editor.language.hard_tabs);
        assert!(config.editor.editor.cursor_blink);
    }

    #[test]
    fn test_config_manager() {
        let manager = ConfigManager::new();
        let config = manager.get_config();
        assert_eq!(config.layout.connector_column_width, 45.0);
    }
}
