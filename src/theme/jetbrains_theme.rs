// JetBrains theme implementation with Zed IDE font specifications
use crate::config::{FontMetrics, LineHeightMode, ZedFontConfig, ZedFontManager, ZedSettings};
use egui::{Color32, Stroke};

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
            color_blue_500: Color32::from_rgb(33, 150, 243),
            font_config,
            editor_settings,
            gutter_width: 45.0,
            connector_width: 45.0,
            addition_background: Color32::from_rgb(52, 85, 52),
            addition_foreground: Color32::from_rgb(129, 199, 132),
            addition_gutter: Color32::from_rgb(52, 85, 52),
            deletion_background: Color32::from_rgb(85, 56, 56), // Rouge foncé solide (pas de transparence)
            deletion_foreground: Color32::from_rgb(239, 154, 154),
            deletion_gutter: Color32::from_rgb(113, 113, 113),
            modification_background: Color32::from_rgb(50, 66, 98),
            modification_foreground: Color32::from_rgb(212, 212, 212), // Use normal foreground for modifications
            modification_gutter: Color32::from_rgb(50, 66, 98),
            code_foreground: Color32::from_rgb(212, 212, 212),
            code_comment: Color32::from_rgb(106, 153, 85),
            code_keyword: Color32::from_rgb(86, 156, 214),
            code_string: Color32::from_rgb(206, 145, 120),
            background: Color32::from_rgb(30, 30, 30),
            foreground: Color32::from_rgb(212, 212, 212),
            border: Color32::from_rgb(62, 62, 62),
            gutter_background: Color32::from_rgb(37, 37, 38),
            gutter_border: Color32::from_rgb(62, 62, 62),
            connector_column: Color32::from_rgb(45, 45, 45),
            line_numbers: Color32::from_rgb(153, 153, 153),
            show_line_numbers: true,
        }
    }

    pub fn apply_to_context(&self, ctx: &egui::Context) {
        // Apply Zed font configuration first
        let font_manager = ZedFontManager::with_config(self.font_config.clone());
        font_manager.apply_to_context(ctx);

        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = self.background.r() < 128;
        style.visuals.window_fill = self.background;
        style.visuals.panel_fill = self.background;
        style.visuals.faint_bg_color = self.background;
        style.visuals.override_text_color = Some(self.foreground);
        style.visuals.selection.bg_fill = self.modification_background;
        style.visuals.selection.stroke = Stroke::new(1.0, self.modification_background);
        ctx.set_style(style);
    }

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

    /// Get buffer font ID for egui
    pub fn buffer_font_id(&self) -> egui::FontId {
        self.font_config.buffer_font_id()
    }

    /// Get UI font ID for egui
    pub fn ui_font_id(&self) -> egui::FontId {
        self.font_config.ui_font_id()
    }

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
            crate::models::line::LineType::Context => Color32::TRANSPARENT,
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
            color_blue_500: Color32::from_rgb(100, 150, 200),
            font_config,
            editor_settings,
            gutter_width: 40.0,
            connector_width: 40.0,
            addition_background: Color32::from_rgb(40, 60, 40),
            addition_foreground: Color32::from_rgb(100, 180, 100),
            addition_gutter: Color32::from_rgb(40, 60, 40),
            deletion_background: Color32::from_rgb(80, 50, 50), // Rouge plus visible
            deletion_foreground: Color32::from_rgb(200, 120, 120),
            deletion_gutter: Color32::from_rgb(80, 80, 80),
            modification_background: Color32::from_rgb(40, 50, 80),
            modification_foreground: Color32::from_rgb(200, 200, 200), // Use normal foreground for modifications
            modification_gutter: Color32::from_rgb(40, 50, 80),
            code_foreground: Color32::from_rgb(200, 200, 200),
            code_comment: Color32::from_rgb(100, 140, 80),
            code_keyword: Color32::from_rgb(80, 140, 200),
            code_string: Color32::from_rgb(200, 140, 100),
            background: Color32::from_rgb(40, 40, 40),
            foreground: Color32::from_rgb(200, 200, 200),
            border: Color32::from_rgb(80, 80, 80),
            gutter_background: Color32::from_rgb(50, 50, 50),
            gutter_border: Color32::from_rgb(80, 80, 80),
            connector_column: Color32::from_rgb(60, 60, 60),
            line_numbers: Color32::from_rgb(140, 140, 140),
            show_line_numbers: true,
        }
    }
}
