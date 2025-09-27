// diffsplit/src/config/mod.rs
// Application configuration module

pub mod app_config;
pub mod editor_settings;
pub mod fonts;

pub use app_config::*;
pub use editor_settings::*;
pub use fonts::*;

/// Application configuration
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub layout: LayoutConfig,
    pub fonts: ZedFontConfig,
    pub editor: ZedSettings,
}

/// Layout configuration
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    pub connector_column_width: f32,
    pub pane_padding: f32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            connector_column_width: 56.0,
            pane_padding: 10.0,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            layout: LayoutConfig::default(),
            fonts: ZedFontConfig::default(),
            editor: ZedSettings::default(),
        }
    }
}

/// Configuration manager
pub struct ConfigManager {
    config: AppConfig,
    font_manager: ZedFontManager,
}

impl ConfigManager {
    pub fn new() -> Self {
        let config = AppConfig::default();
        let font_manager = ZedFontManager::with_config(config.fonts.clone());

        Self {
            config,
            font_manager,
        }
    }

    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    pub fn get_font_manager(&self) -> &ZedFontManager {
        &self.font_manager
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
