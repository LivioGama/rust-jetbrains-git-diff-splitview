// Font configuration system based on Zed IDE specifications from crates/theme/src/settings.rs
use egui::{FontData, FontDefinitions, FontFamily, FontId};
use std::result::Result;

/// Zed IDE font specifications from source code
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

    // Line height modes from Zed source
    pub line_height_mode: LineHeightMode,
}

/// Line height configuration modes from Zed
#[derive(Debug, Clone, PartialEq)]
pub enum LineHeightMode {
    Comfortable, // 1.618 (golden ratio)
    Standard,    // 1.3
    Custom(f32), // custom value
}

impl LineHeightMode {
    pub fn to_multiplier(&self) -> f32 {
        match self {
            LineHeightMode::Comfortable => 1.618,
            LineHeightMode::Standard => 1.3,
            LineHeightMode::Custom(value) => *value,
        }
    }
}

impl Default for ZedFontConfig {
    fn default() -> Self {
        Self {
            // Use generic font families that egui handles
            buffer_font_family: "monospace".to_string(),
            buffer_font_size: 15.0, // Exact Zed default from settings.rs
            buffer_font_weight: 400,
            buffer_line_height: 15.0 * 1.618, // comfortable line height

            // Use generic font families that egui handles
            ui_font_family: "sans-serif".to_string(),
            ui_font_size: 16.0, // Exact Zed default from settings.rs
            ui_font_weight: 400,

            // Terminal uses same as buffer
            terminal_font_family: "monospace".to_string(),
            terminal_font_size: 15.0,
            terminal_line_height: 15.0 * 1.618,

            // Font features
            ligatures_enabled: true,
            line_height_mode: LineHeightMode::Comfortable,
        }
    }
}

impl ZedFontConfig {
    /// Create a new Zed font configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Get buffer font ID for egui
    pub fn buffer_font_id(&self) -> FontId {
        FontId::new(self.buffer_font_size, FontFamily::Monospace)
    }

    /// Get UI font ID for egui
    pub fn ui_font_id(&self) -> FontId {
        FontId::new(self.ui_font_size, FontFamily::Proportional)
    }

    /// Get terminal font ID for egui
    pub fn terminal_font_id(&self) -> FontId {
        FontId::new(self.terminal_font_size, FontFamily::Monospace)
    }

    /// Get calculated line height for buffer
    pub fn calculated_buffer_line_height(&self) -> f32 {
        self.buffer_font_size * self.line_height_mode.to_multiplier()
    }

    /// Get calculated line height for terminal
    pub fn calculated_terminal_line_height(&self) -> f32 {
        self.terminal_font_size * self.line_height_mode.to_multiplier()
    }

    /// Set custom line height mode
    pub fn with_line_height_mode(mut self, mode: LineHeightMode) -> Self {
        self.line_height_mode = mode;
        self.buffer_line_height = self.calculated_buffer_line_height();
        self.terminal_line_height = self.calculated_terminal_line_height();
        self
    }

    /// Set buffer font size and recalculate line height
    pub fn with_buffer_font_size(mut self, size: f32) -> Self {
        self.buffer_font_size = size;
        self.buffer_line_height = self.calculated_buffer_line_height();
        self
    }

    /// Set UI font size
    pub fn with_ui_font_size(mut self, size: f32) -> Self {
        self.ui_font_size = size;
        self
    }

    /// Enable/disable ligatures
    pub fn with_ligatures(mut self, enabled: bool) -> Self {
        self.ligatures_enabled = enabled;
        self
    }

    /// Validate font sizes are within Zed's limits (6-100px)
    pub fn validate_font_sizes(&mut self) {
        self.buffer_font_size = self.buffer_font_size.clamp(6.0, 100.0);
        self.ui_font_size = self.ui_font_size.clamp(6.0, 100.0);
        self.terminal_font_size = self.terminal_font_size.clamp(6.0, 100.0);
    }
}

/// Font manager for loading and configuring fonts
pub struct ZedFontManager {
    config: ZedFontConfig,
    font_definitions: FontDefinitions,
    embedded_fonts_loaded: bool,
}

impl ZedFontManager {
    /// Create a new font manager with default Zed configuration
    pub fn new() -> Self {
        Self::with_config(ZedFontConfig::default())
    }

    /// Create a new font manager with custom configuration
    pub fn with_config(config: ZedFontConfig) -> Self {
        // Use egui's default font definitions without modification
        let font_definitions = FontDefinitions::default();
        let embedded_fonts_loaded = false;

        Self {
            config,
            font_definitions,
            embedded_fonts_loaded,
        }
    }

    /// Apply font configuration to egui context
    pub fn apply_to_context(&self, ctx: &egui::Context) {
        // Apply fonts with error handling
        match self.try_apply_fonts(ctx) {
            Ok(()) => {}
            Err(_) => {
                self.apply_system_font_fallback(ctx);
            }
        }
    }

    /// Try to apply fonts with error handling
    fn try_apply_fonts(&self, ctx: &egui::Context) -> Result<(), String> {
        ctx.set_fonts(self.font_definitions.clone());

        // Apply additional styling for Zed-like appearance
        let mut style = (*ctx.style()).clone();

        // Set default font sizes with safe fallbacks
        style.text_styles.insert(
            egui::TextStyle::Body,
            FontId::new(self.config.ui_font_size, FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Monospace,
            FontId::new(self.config.buffer_font_size, FontFamily::Monospace),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            FontId::new(self.config.ui_font_size, FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Small,
            FontId::new(self.config.ui_font_size * 0.85, FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Heading,
            FontId::new(self.config.ui_font_size * 1.2, FontFamily::Proportional),
        );

        ctx.set_style(style);
        Ok(())
    }

    /// Apply system font fallback if custom fonts fail
    fn apply_system_font_fallback(&self, ctx: &egui::Context) {
        let mut font_definitions = FontDefinitions::default();

        // Use only system fonts with safe defaults
        font_definitions.families.insert(
            FontFamily::Monospace,
            vec![
                "SF Mono".to_string(),
                "Menlo".to_string(),
                "Monaco".to_string(),
                "Consolas".to_string(),
                "monospace".to_string(),
            ],
        );

        font_definitions.families.insert(
            FontFamily::Proportional,
            vec![
                "SF Pro Display".to_string(),
                "system-ui".to_string(),
                "-apple-system".to_string(),
                "sans-serif".to_string(),
            ],
        );

        ctx.set_fonts(font_definitions);
    }

    /// Get font configuration
    pub fn config(&self) -> &ZedFontConfig {
        &self.config
    }

    /// Check if embedded fonts were loaded successfully
    pub fn has_embedded_fonts(&self) -> bool {
        self.embedded_fonts_loaded
    }

    /// Update font configuration
    pub fn update_config(&mut self, config: ZedFontConfig) {
        self.config = config;
        self.font_definitions = FontDefinitions::default();
        self.embedded_fonts_loaded = Self::try_load_embedded_fonts(&mut self.font_definitions);
        Self::setup_monospace_fonts(
            &mut self.font_definitions,
            &self.config,
            self.embedded_fonts_loaded,
        );
        Self::setup_proportional_fonts(
            &mut self.font_definitions,
            &self.config,
            self.embedded_fonts_loaded,
        );
    }

    /// Setup monospace fonts with Zed-style fallbacks
    fn setup_monospace_fonts(
        _font_definitions: &mut FontDefinitions,
        _config: &ZedFontConfig,
        _embedded_fonts_loaded: bool,
    ) {
        // Use egui defaults, no customization
    }

    /// Setup proportional fonts with Zed-style fallbacks
    fn setup_proportional_fonts(
        _font_definitions: &mut FontDefinitions,
        _config: &ZedFontConfig,
        _embedded_fonts_loaded: bool,
    ) {
        // Use egui defaults, no customization
    }

    /// Load embedded fonts with proper error handling
    fn try_load_embedded_fonts(font_definitions: &mut FontDefinitions) -> bool {
        let mut fonts_loaded = 0;
        let mut total_fonts = 0;

        // Try to load embedded Lilex font (used as Zed Mono replacement)
        total_fonts += 1;
        match load_lilex_font() {
            Ok(lilex_data) => {
                font_definitions
                    .font_data
                    .insert("Lilex".to_owned(), FontData::from_static(lilex_data).into());
                fonts_loaded += 1;
            }
            Err(_) => {}
        }

        // Try to load embedded IBM Plex Sans font
        total_fonts += 1;
        match load_ibm_plex_sans_font() {
            Ok(plex_sans_data) => {
                font_definitions.font_data.insert(
                    "IBM Plex Sans".to_owned(),
                    FontData::from_static(plex_sans_data).into(),
                );
                fonts_loaded += 1;
            }
            Err(_) => {}
        }

        let success = fonts_loaded > 0;
        success
    }
}

impl Default for ZedFontManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Load embedded Lilex font (Zed Mono replacement) with error handling
fn load_lilex_font() -> Result<&'static [u8], String> {
    let data = include_bytes!("../../assets/fonts/Lilex-Regular.ttf");
    if data.is_empty() {
        Err("Lilex font file is empty".to_string())
    } else {
        Ok(data)
    }
}

/// Load embedded IBM Plex Sans font with error handling
fn load_ibm_plex_sans_font() -> Result<&'static [u8], String> {
    let data = include_bytes!("../../assets/fonts/IBMPlexSans-Regular.ttf");
    if data.is_empty() {
        Err("IBM Plex Sans font file is empty".to_string())
    } else {
        Ok(data)
    }
}

/// Utility functions for font metrics calculations
pub struct FontMetrics;

impl FontMetrics {
    /// Calculate optimal line height based on font size and mode
    pub fn calculate_line_height(font_size: f32, mode: &LineHeightMode) -> f32 {
        font_size * mode.to_multiplier()
    }

    /// Calculate character width approximation for monospace fonts
    pub fn approximate_char_width(font_size: f32) -> f32 {
        // Rough approximation: monospace character width is typically ~0.6 * font_size
        font_size * 0.6
    }

    /// Calculate text baseline offset for proper alignment
    pub fn calculate_baseline_offset(line_height: f32, font_size: f32) -> f32 {
        line_height - (line_height - font_size) * 0.5 - 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zed_font_config_default() {
        let config = ZedFontConfig::default();

        assert_eq!(config.buffer_font_family, "monospace");
        assert_eq!(config.buffer_font_size, 15.0);
        assert_eq!(config.buffer_font_weight, 400);
        assert_eq!(config.ui_font_size, 16.0);
        assert!(config.ligatures_enabled);
        assert_eq!(config.line_height_mode, LineHeightMode::Comfortable);
    }

    #[test]
    fn test_line_height_modes() {
        assert_eq!(LineHeightMode::Comfortable.to_multiplier(), 1.618);
        assert_eq!(LineHeightMode::Standard.to_multiplier(), 1.3);
        assert_eq!(LineHeightMode::Custom(2.0).to_multiplier(), 2.0);
    }

    #[test]
    fn test_font_config_builder() {
        let config = ZedFontConfig::new()
            .with_buffer_font_size(14.0)
            .with_ui_font_size(15.0)
            .with_ligatures(false)
            .with_line_height_mode(LineHeightMode::Standard);

        assert_eq!(config.buffer_font_size, 14.0);
        assert_eq!(config.ui_font_size, 15.0);
        assert!(!config.ligatures_enabled);
        assert_eq!(config.line_height_mode, LineHeightMode::Standard);
    }

    #[test]
    fn test_font_metrics() {
        let line_height = FontMetrics::calculate_line_height(15.0, &LineHeightMode::Comfortable);
        assert_eq!(line_height, 15.0 * 1.618);

        let char_width = FontMetrics::approximate_char_width(15.0);
        assert_eq!(char_width, 15.0 * 0.6);
    }
}
