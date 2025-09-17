// diffsplit/src/config/mod.rs
// Application configuration module

/// Application configuration
#[derive(Debug, Clone, Default)]
pub struct AppConfig {
    pub layout: LayoutConfig,
}

/// Window configuration
#[derive(Debug, Clone)]
pub struct WindowConfig {}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {}
    }
}

/// Theme configuration
#[derive(Debug, Clone)]
pub struct ThemeConfig {}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {}
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
pub struct NavigationConfig {}

impl Default for NavigationConfig {
    fn default() -> Self {
        Self {}
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

    pub fn get_config(&self) -> &AppConfig {
        &self.config
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
