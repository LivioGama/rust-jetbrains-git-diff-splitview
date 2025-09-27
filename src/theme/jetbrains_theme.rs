// JetBrains theme implementation with Zed IDE font specifications
use crate::config::{FontMetrics, LineHeightMode, ZedFontConfig, ZedFontManager, ZedSettings};
use egui::Color32;
pub type Stroke = f32; // Simplified for now

#[derive(Debug, Clone)]
pub struct JetBrainsTheme {
    pub color_blue_500: egui::Color32,
    pub font_config: ZedFontConfig,
    pub editor_settings: ZedSettings,
    pub gutter_width: f32,
    pub connector_width: f32,
    pub addition_background: egui::Color32,
    pub addition_foreground: egui::Color32,
    pub addition_gutter: egui::Color32,
    pub deletion_background: egui::Color32,
    pub deletion_foreground: egui::Color32,
    pub deletion_gutter: egui::Color32,
    pub modification_background: egui::Color32,
    pub modification_foreground: egui::Color32,
    pub modification_gutter: egui::Color32,
    pub code_foreground: egui::Color32,
    pub code_comment: egui::Color32,
    pub code_keyword: egui::Color32,
    pub code_string: egui::Color32,
    pub background: egui::Color32,
    pub foreground: egui::Color32,
    pub border: egui::Color32,
    pub gutter_background: egui::Color32,
    pub gutter_border: egui::Color32,
    pub connector_column: egui::Color32,
    pub line_numbers: egui::Color32,
    pub show_line_numbers: bool,
}

impl JetBrainsTheme {
    pub fn dark_theme() -> Self {
        let font_config =
            ZedFontConfig::default().with_line_height_mode(LineHeightMode::Comfortable);
        let editor_settings = ZedSettings::default();

        Self {
            color_blue_500: egui::Color32::from_rgb(33, 150, 243),
            font_config,
            editor_settings,
            gutter_width: 45.0,
            connector_width: 45.0,
            addition_background: egui::Color32::from_rgb(52, 85, 52),
            addition_foreground: egui::Color32::from_rgb(129, 199, 132),
            addition_gutter: egui::Color32::from_rgb(76, 175, 80),
            deletion_background: egui::Color32::from_rgb(85, 52, 52),
            deletion_foreground: egui::Color32::from_rgb(199, 129, 129),
            deletion_gutter: egui::Color32::from_rgb(244, 67, 54),
            modification_background: egui::Color32::from_rgb(52, 52, 85),
            modification_foreground: egui::Color32::from_rgb(129, 129, 199),
            modification_gutter: egui::Color32::from_rgb(33, 150, 243),
            code_foreground: egui::Color32::from_rgb(169, 183, 198),
            code_comment: egui::Color32::from_rgb(128, 128, 128),
            code_keyword: egui::Color32::from_rgb(204, 120, 50),
            code_string: egui::Color32::from_rgb(106, 135, 89),
            background: egui::Color32::from_rgb(43, 43, 43),
            foreground: egui::Color32::from_rgb(169, 183, 198),
            border: egui::Color32::from_rgb(73, 73, 73),
            gutter_background: egui::Color32::from_rgb(33, 33, 33),
            gutter_border: egui::Color32::from_rgb(73, 73, 73),
            connector_column: egui::Color32::from_rgb(43, 43, 43),
            line_numbers: egui::Color32::from_rgb(128, 128, 128),
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
            crate::models::line::LineType::Context => egui::Color32::TRANSPARENT,
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
            color_blue_500: egui::Color32::from_rgb(100, 150, 200),
            font_config,
            editor_settings,
            gutter_width: 40.0,
            connector_width: 40.0,
            addition_background: egui::Color32::from_rgb(40, 60, 40),
            addition_foreground: egui::Color32::from_rgb(100, 180, 100),
            addition_gutter: egui::Color32::from_rgb(76, 175, 80),
            deletion_background: egui::Color32::from_rgb(80, 50, 50),
            deletion_foreground: egui::Color32::from_rgb(200, 120, 120),
            deletion_gutter: egui::Color32::from_rgb(244, 67, 54),
            modification_background: egui::Color32::from_rgb(40, 50, 80),
            modification_foreground: egui::Color32::from_rgb(200, 200, 200),
            modification_gutter: egui::Color32::from_rgb(33, 150, 243),
            code_foreground: egui::Color32::from_rgb(0, 0, 0),
            code_comment: egui::Color32::from_rgb(128, 128, 128),
            code_keyword: egui::Color32::from_rgb(0, 0, 255),
            code_string: egui::Color32::from_rgb(0, 128, 0),
            background: egui::Color32::from_rgb(255, 255, 255),
            foreground: egui::Color32::from_rgb(0, 0, 0),
            border: egui::Color32::from_rgb(200, 200, 200),
            gutter_background: egui::Color32::from_rgb(240, 240, 240),
            gutter_border: egui::Color32::from_rgb(200, 200, 200),
            connector_column: egui::Color32::from_rgb(240, 240, 240),
            line_numbers: egui::Color32::from_rgb(128, 128, 128),
            show_line_numbers: true,
        }
    }
}
