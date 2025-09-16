use std::collections::HashMap;

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub window: WindowConfig,
    pub theme: ThemeConfig,
    pub layout: LayoutConfig,
    pub navigation: NavigationConfig,
    pub git: GitConfig,
    pub file: FileConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            theme: ThemeConfig::default(),
            layout: LayoutConfig::default(),
            navigation: NavigationConfig::default(),
            git: GitConfig::default(),
            file: FileConfig::default(),
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
        Self {
            dark_mode: true,
            font_size: 12.0,
            line_height: 18.0,
            font_family: "Monospace".to_string(),
            custom_colors: HashMap::new(),
        }
    }
}

/// Layout configuration
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    pub header_height: f32,
    pub scroll_area_padding: f32,
    pub gutter_width: f32,
    pub pane_padding: f32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            header_height: 40.0,
            scroll_area_padding: 5.0,
            gutter_width: 40.0,
            pane_padding: 8.0,
        }
    }
}

/// Navigation configuration
#[derive(Debug, Clone)]
pub struct NavigationConfig {
    pub key_bindings: HashMap<String, String>,
    pub scroll_sensitivity: f32,
    pub auto_scroll: bool,
    pub diff_lookahead: usize,
}

impl Default for NavigationConfig {
    fn default() -> Self {
        let mut key_bindings = HashMap::new();
        key_bindings.insert("next_change".to_string(), "ArrowDown".to_string());
        key_bindings.insert("prev_change".to_string(), "ArrowUp".to_string());

        Self {
            key_bindings,
            scroll_sensitivity: 1.0,
            auto_scroll: true,
            diff_lookahead: 50,
        }
    }
}

/// Git configuration
#[derive(Debug, Clone)]
pub struct GitConfig {
    pub repo_path: String,
    pub original_commit: String,
    pub current_commit: Option<String>,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            repo_path: "/Users/livio/Documents/anbiti-apps/".to_string(),
            original_commit: "cb2752b3".to_string(),
            current_commit: Some("dcc2893b".to_string()),
        }
    }
}

/// File configuration
#[derive(Debug, Clone)]
pub struct FileConfig {
    pub file_path: String,
    pub fallback_content: String,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            file_path: "apps/app/app/Providers.tsx".to_string(),
            fallback_content: "function App() {\n  return <div>Hello World</div>;\n}\n\nexport default App;\n".to_string(),
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

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut AppConfig {
        &mut self.config
    }

    pub fn load_from_file(_path: &str) -> Result<Self, String> {
        Err("Not implemented".to_string())
    }

    pub fn save_to_file(&self, _path: &str) -> Result<(), String> {
        Err("Not implemented".to_string())
    }

    pub fn update_config<F>(&mut self, updater: F)
    where
        F: FnOnce(&mut AppConfig),
    {
        updater(&mut self.config);
    }
}
