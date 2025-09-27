// Simplified font configuration inspired by Zed's defaults.

/// Zed IDE font specifications from source code
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZedFontConfig {
    // Buffer/Editor fonts - from crates/theme/src/settings.rs
    pub buffer_font_family: String,
    pub buffer_font_size: f32, // Default 15px (min 6, max 100)
    pub buffer_font_weight: u16,
    pub buffer_line_height: f32,

    // UI fonts - from crates/theme/src/settings.rs
    pub ui_font_family: String,
    pub ui_font_size: f32, // Default 16px (min 6, max 100)
    pub ui_font_weight: u16,

    // Terminal fonts
    pub terminal_font_family: String,
    pub terminal_font_size: f32,
    pub terminal_line_height: f32,

    // Font features
    pub ligatures_enabled: bool,

    // Line height modes from Zed
    pub line_height_mode: LineHeightMode,
}

/// Line height configuration modes from Zed
#[derive(Debug, Clone, PartialEq)]
pub enum LineHeightMode {
    Comfortable, // 1.618 (golden ratio)
}

impl LineHeightMode {
    pub fn to_multiplier(&self) -> f32 {
        match self {
            LineHeightMode::Comfortable => 1.618,
        }
    }
}

impl Default for ZedFontConfig {
    fn default() -> Self {
        Self {
            buffer_font_family: "Lilex".to_string(),
            buffer_font_size: 15.0,
            buffer_font_weight: 400,
            buffer_line_height: 15.0 * 1.618,
            ui_font_family: "Sans Serif".to_string(),
            ui_font_size: 16.0,
            ui_font_weight: 400,
            terminal_font_family: "Lilex".to_string(),
            terminal_font_size: 15.0,
            terminal_line_height: 15.0 * 1.618,
            ligatures_enabled: true,
            line_height_mode: LineHeightMode::Comfortable,
        }
    }
}

impl ZedFontConfig {
    pub fn calculated_buffer_line_height(&self) -> f32 {
        self.buffer_font_size * self.line_height_mode.to_multiplier()
    }

    pub fn buffer_font_size(&self) -> f32 {
        self.buffer_font_size
    }

    pub fn buffer_font_bytes(&self) -> &'static [u8] {
        include_bytes!("../../assets/fonts/Lilex-Regular.ttf")
    }
}

/// Minimal font manager that simply exposes the configuration.
#[derive(Debug, Clone)]
pub struct ZedFontManager {
    config: ZedFontConfig,
}

impl ZedFontManager {
    pub fn with_config(config: ZedFontConfig) -> Self {
        Self { config }
    }

    #[allow(dead_code)]
    pub fn config(&self) -> &ZedFontConfig {
        &self.config
    }

    pub fn buffer_font_size(&self) -> f32 {
        self.config.buffer_font_size()
    }

    pub fn buffer_line_height(&self) -> f32 {
        self.config.calculated_buffer_line_height()
    }

    pub fn buffer_font_bytes(&self) -> &'static [u8] {
        self.config.buffer_font_bytes()
    }
}

impl Default for ZedFontManager {
    fn default() -> Self {
        Self {
            config: ZedFontConfig::default(),
        }
    }
}
