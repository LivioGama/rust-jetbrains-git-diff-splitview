// JetBrains theme implementation with Zed IDE font specifications
use crate::config::{FontMetrics, LineHeightMode, ZedFontConfig, ZedFontManager, ZedSettings};
use gpui::{Hsla, rgba};

// Compatibility layer for egui types
pub type Color32 = gpui::Hsla;
pub type Stroke = f32; // Simplified for now

#[derive(Debug, Clone)]
pub struct JetBrainsTheme {
    pub color_blue_500: Color32,
    pub font_config: ZedFontConfig,
    pub editor_settings: ZedSettings,
    pub gutter_width: f32,
    pub connector_width: f32,
    pub addition_background: Color32,
    pub addition_foreground: Color32,
    pub addition_gutter: Color32,
    pub deletion_background: Color32,
    pub deletion_foreground: Color32,
    pub deletion_gutter: Color32,
    pub modification_background: Color32,
    pub modification_foreground: Color32,
    pub modification_gutter: Color32,
    pub code_foreground: Color32,
    pub code_comment: Color32,
    pub code_keyword: Color32,
    pub code_string: Color32,
    pub background: Color32,
    pub foreground: Color32,
    pub border: Color32,
    pub gutter_background: Color32,
    pub gutter_border: Color32,
    pub connector_column: Color32,
    pub line_numbers: Color32,
    pub show_line_numbers: bool,
}

impl JetBrainsTheme {
    pub fn dark_theme() -> Self {
        let font_config =
            ZedFontConfig::default().with_line_height_mode(LineHeightMode::Comfortable);
        let editor_settings = ZedSettings::default();

        Self {
            color_blue_500: rgba(33.0/255.0, 150.0/255.0, 243.0/255.0, 1.0),
            font_config,
            editor_settings,
            gutter_width: 45.0,
            connector_width: 45.0,
            addition_background: rgba(52.0/255.0, 85.0/255.0, 52.0/255.0, 1.0),
            addition_foreground: rgba(129.0/255.0, 199.0/255.0, 132.0/255.0, 1.0),
            addition_gutter: rgba(52.0/255.0, 85.0/255.0, 52.0/255.0, 1.0),
            deletion_background: rgba(85.0/255.0, 56.0/255.0, 56.0/255.0, 1.0), // Rouge foncé solide (pas de transparence)
            deletion_foreground: rgba(239.0/255.0, 154.0/255.0, 154.0/255.0, 1.0),
            deletion_gutter: rgba(113.0/255.0, 113.0/255.0, 113.0/255.0, 1.0),
            modification_background: rgba(50.0/255.0, 66.0/255.0, 98.0/255.0, 1.0),
            modification_foreground: rgba(212.0/255.0, 212.0/255.0, 212.0/255.0, 1.0), // Use normal foreground for modifications
            modification_gutter: rgba(50.0/255.0, 66.0/255.0, 98.0/255.0, 1.0),
            code_foreground: rgba(212.0/255.0, 212.0/255.0, 212.0/255.0, 1.0),
            code_comment: rgba(106.0/255.0, 153.0/255.0, 85.0/255.0, 1.0),
            code_keyword: rgba(86.0/255.0, 156.0/255.0, 214.0/255.0, 1.0),
            code_string: rgba(206.0/255.0, 145.0/255.0, 120.0/255.0, 1.0),
            background: rgba(30.0/255.0, 30.0/255.0, 30.0/255.0, 1.0),
            foreground: rgba(212.0/255.0, 212.0/255.0, 212.0/255.0, 1.0),
            border: rgba(62.0/255.0, 62.0/255.0, 62.0/255.0, 1.0),
            gutter_background: rgba(37.0/255.0, 37.0/255.0, 38.0/255.0, 1.0),
            gutter_border: rgba(62.0/255.0, 62.0/255.0, 62.0/255.0, 1.0),
            connector_column: rgba(45.0/255.0, 45.0/255.0, 45.0/255.0, 1.0),
            line_numbers: rgba(153.0/255.0, 153.0/255.0, 153.0/255.0, 1.0),
            show_line_numbers: true,
        }
    }

    // TODO: Migrate to gpui context
    // pub fn apply_to_context(&self, ctx: &egui::Context) {
        // TODO: Apply gpui font configuration
        // let font_manager = ZedFontManager::with_config(self.font_config.clone());
        // font_manager.apply_to_context(ctx);
        // 
        // let mut style = (*ctx.style()).clone();
        // style.visuals.dark_mode = self.background.r() < 128;
        // ...
        // ctx.set_style(style);
    // }

    /// Get Zed-style buffer font size
    pub fn buffer_font_size(&self) -> f32 {
        self.font_config.buffer_font_size
    }

    /// Get Zed-style UI font size
    pub fn ui_font_size(&self) -> f32 {
        self.font_config.ui_font_size
    }

    /// Get calculated line height using Zed's golden ratio
    pub fn line_height(&self) -> f32 {
        self.font_config.calculated_buffer_line_height()
    }

    // TODO: Migrate to gpui font ID
    // pub fn buffer_font_id(&self) -> gpui::FontId {
    //     self.font_config.buffer_font_id()
    // }

    // TODO: Migrate to gpui font ID
    // pub fn ui_font_id(&self) -> gpui::FontId {
    //     self.font_config.ui_font_id()
    // }

    /// Check if ligatures are enabled
    pub fn ligatures_enabled(&self) -> bool {
        self.font_config.ligatures_enabled
    }

    /// Calculate baseline offset for text rendering
    pub fn baseline_offset(&self) -> f32 {
        FontMetrics::calculate_baseline_offset(self.line_height(), self.buffer_font_size())
    }

    /// Get character width approximation for monospace text
    pub fn char_width(&self) -> f32 {
        FontMetrics::approximate_char_width(self.buffer_font_size())
    }

    /// Get Zed editor settings
    pub fn editor_settings(&self) -> &ZedSettings {
        &self.editor_settings
    }

    /// Check if cursor should blink based on Zed settings
    pub fn cursor_should_blink(&self) -> bool {
        self.editor_settings.editor().cursor_blink
    }

    /// Get vertical scroll margin from Zed settings
    pub fn vertical_scroll_margin(&self) -> u32 {
        self.editor_settings.editor().vertical_scroll_margin
    }

    /// Get horizontal scroll margin from Zed settings
    pub fn horizontal_scroll_margin(&self) -> u32 {
        self.editor_settings.editor().horizontal_scroll_margin
    }

    /// Get scroll sensitivity from Zed settings
    pub fn scroll_sensitivity(&self) -> f32 {
        self.editor_settings.editor().scroll_sensitivity
    }

    /// Get tab size from Zed language settings
    pub fn tab_size(&self) -> u32 {
        self.editor_settings.language.tab_size
    }

    /// Check if hard tabs should be used
    pub fn use_hard_tabs(&self) -> bool {
        self.editor_settings.language.hard_tabs
    }

    pub fn get_connector_color(&self, line_type: &crate::models::line::LineType) -> Color32 {
        match line_type {
            crate::models::line::LineType::Addition => self.addition_background,
            crate::models::line::LineType::Deletion => self.deletion_background,
            crate::models::line::LineType::Modification => self.modification_background,
            _ => self.modification_background,
        }
    }

    pub fn get_line_background(&self, line_type: &crate::models::line::LineType) -> Color32 {
        match line_type {
            crate::models::line::LineType::Addition => self.addition_background,
            crate::models::line::LineType::Deletion => self.deletion_background,
            crate::models::line::LineType::Modification => self.modification_background,
            crate::models::line::LineType::Context => rgba(0.0, 0.0, 0.0, 0.0),
        }
    }

    /// Create a safe default theme that won't panic
    pub fn safe_default() -> Self {
        // Use minimal, safe configuration with system fonts
        let font_config = ZedFontConfig {
            buffer_font_family: "monospace".to_string(),
            buffer_font_size: 14.0,
            buffer_font_weight: 400,
            buffer_line_height: 14.0 * 1.3, // Standard line height
            ui_font_family: "sans-serif".to_string(),
            ui_font_size: 14.0,
            ui_font_weight: 400,
            terminal_font_family: "monospace".to_string(),
            terminal_font_size: 14.0,
            terminal_line_height: 14.0 * 1.3,
            ligatures_enabled: false,
            line_height_mode: LineHeightMode::Comfortable,
        };

        let editor_settings = ZedSettings::default();

        Self {
            color_blue_500: rgba(100.0/255.0, 150.0/255.0, 200.0/255.0, 1.0),
            font_config,
            editor_settings,
            gutter_width: 40.0,
            connector_width: 40.0,
            addition_background: rgba(40.0/255.0, 60.0/255.0, 40.0/255.0, 1.0),
            addition_foreground: rgba(100.0/255.0, 180.0/255.0, 100.0/255.0, 1.0),
            addition_gutter: rgba(40.0/255.0, 60.0/255.0, 40.0/255.0, 1.0),
            deletion_background: rgba(80.0/255.0, 50.0/255.0, 50.0/255.0, 1.0), // Rouge plus visible
            deletion_foreground: rgba(200.0/255.0, 120.0/255.0, 120.0/255.0, 1.0),
            deletion_gutter: rgba(80.0/255.0, 80.0/255.0, 80.0/255.0, 1.0),
            modification_background: rgba(40.0/255.0, 50.0/255.0, 80.0/255.0, 1.0),
            modification_foreground: rgba(200.0/255.0, 200.0/255.0, 200.0/255.0, 1.0), // Use normal foreground for modifications
            modification_gutter: rgba(40.0/255.0, 50.0/255.0, 80.0/255.0, 1.0),
            code_foreground: rgba(200.0/255.0, 200.0/255.0, 200.0/255.0, 1.0),
            code_comment: rgba(100.0/255.0, 140.0/255.0, 80.0/255.0, 1.0),
            code_keyword: rgba(80.0/255.0, 140.0/255.0, 200.0/255.0, 1.0),
            code_string: rgba(200.0/255.0, 140.0/255.0, 100.0/255.0, 1.0),
            background: rgba(40.0/255.0, 40.0/255.0, 40.0/255.0, 1.0),
            foreground: rgba(200.0/255.0, 200.0/255.0, 200.0/255.0, 1.0),
            border: rgba(80.0/255.0, 80.0/255.0, 80.0/255.0, 1.0),
            gutter_background: rgba(50.0/255.0, 50.0/255.0, 50.0/255.0, 1.0),
            gutter_border: rgba(80.0/255.0, 80.0/255.0, 80.0/255.0, 1.0),
            connector_column: rgba(60.0/255.0, 60.0/255.0, 60.0/255.0, 1.0),
            line_numbers: rgba(140.0/255.0, 140.0/255.0, 140.0/255.0, 1.0),
            show_line_numbers: true,
        }
    }
}
