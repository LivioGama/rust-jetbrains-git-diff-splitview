// JetBrains theme implementation for diff viewer - GPUI Implementation

#[derive(Debug, Clone)]
pub struct JetBrainsTheme {
    // Background colors
    pub background: gpui::Hsla,
    pub connector_column: gpui::Hsla,

    // Text colors
    pub foreground: gpui::Hsla,
    pub code_foreground: gpui::Hsla,
    pub code_comment: gpui::Hsla,
    pub code_keyword: gpui::Hsla,
    pub code_string: gpui::Hsla,
    pub line_numbers: gpui::Hsla,

    // Diff colors
    pub addition_background: gpui::Hsla,
    pub addition_foreground: gpui::Hsla,
    pub deletion_background: gpui::Hsla,
    pub deletion_foreground: gpui::Hsla,
    pub modification_background: gpui::Hsla,
    pub modification_foreground: gpui::Hsla,

    // UI colors
    pub selection: gpui::Hsla,
    pub cursor: gpui::Hsla,

    // Font settings (GPUI native)
    buffer_font_size: f32,
    ui_font_size: f32,
    line_height_multiplier: f32,
}

impl JetBrainsTheme {
    /// Create a dark theme matching JetBrains IDEs
    pub fn dark_theme() -> Self {
        Self {
            // Dark background theme
            background: gpui::hsla(0.0, 0.0, 0.12, 1.0), // Dark gray background
            connector_column: gpui::hsla(0.0, 0.0, 0.15, 1.0), // Darker background for connector column

            // Text colors
            foreground: gpui::hsla(0.0, 0.0, 0.87, 1.0), // Light gray text
            code_foreground: gpui::hsla(0.0, 0.0, 0.87, 1.0), // Light gray for code
            code_comment: gpui::hsla(0.0, 0.0, 0.5, 1.0), // Medium gray for comments
            code_keyword: gpui::hsla(30.0 / 360.0, 0.7, 0.6, 1.0), // Orange for keywords
            code_string: gpui::hsla(90.0 / 360.0, 0.5, 0.6, 1.0), // Green for strings
            line_numbers: gpui::hsla(0.0, 0.0, 0.4, 1.0), // Darker gray for line numbers

            // Diff highlighting colors
            addition_background: gpui::hsla(120.0 / 360.0, 0.6, 0.25, 0.3), // Green with transparency
            addition_foreground: gpui::hsla(120.0 / 360.0, 0.8, 0.7, 1.0),  // Bright green
            deletion_background: gpui::hsla(0.0 / 360.0, 0.6, 0.25, 0.3),   // Red with transparency
            deletion_foreground: gpui::hsla(0.0 / 360.0, 0.8, 0.7, 1.0),    // Bright red
            modification_background: gpui::hsla(210.0 / 360.0, 0.6, 0.25, 0.3), // Blue with transparency
            modification_foreground: gpui::hsla(210.0 / 360.0, 0.8, 0.7, 1.0),  // Bright blue

            // UI colors
            selection: gpui::hsla(210.0 / 360.0, 0.8, 0.4, 0.4), // Blue selection
            cursor: gpui::hsla(0.0, 0.0, 1.0, 1.0),              // White cursor

            // Font configuration
            buffer_font_size: 14.0,
            ui_font_size: 12.0,
            line_height_multiplier: 1.6,
        }
    }

    /// Get line background color for diff highlighting
    pub fn get_line_background(&self, line_type: &crate::models::line::LineType) -> gpui::Hsla {
        use crate::models::line::LineType;
        match line_type {
            LineType::Addition => self.addition_background,
            LineType::Deletion => self.deletion_background,
            LineType::Modification => self.modification_background,
            LineType::Context => gpui::hsla(0.0, 0.0, 0.0, 0.0), // Transparent for context lines
        }
    }

    /// Get the buffer font size
    pub fn buffer_font_size(&self) -> f32 {
        self.buffer_font_size
    }

    /// Get the UI font size
    pub fn ui_font_size(&self) -> f32 {
        self.ui_font_size
    }

    /// Calculate line height based on font size and multiplier
    pub fn line_height(&self) -> f32 {
        self.buffer_font_size * self.line_height_multiplier
    }

    /// Calculate baseline offset for text alignment
    pub fn baseline_offset(&self) -> f32 {
        self.line_height() * 0.8
    }

    /// Get character width (monospace assumption)
    pub fn char_width(&self) -> f32 {
        self.buffer_font_size * 0.6 // Approximate monospace character width
    }

    /// Get gutter width for line numbers
    pub fn gutter_width(&self) -> f32 {
        50.0 // Fixed gutter width in pixels
    }

    /// Update font sizes
    pub fn with_font_sizes(mut self, buffer_font_size: f32, ui_font_size: f32) -> Self {
        self.buffer_font_size = buffer_font_size;
        self.ui_font_size = ui_font_size;
        self
    }

    /// Create a light theme variant
    pub fn light_theme() -> Self {
        Self {
            // Light background theme
            background: gpui::hsla(0.0, 0.0, 0.98, 1.0), // Very light gray background
            connector_column: gpui::hsla(0.0, 0.0, 0.92, 1.0), // Slightly darker for connector column

            // Text colors (inverted from dark theme)
            foreground: gpui::hsla(0.0, 0.0, 0.13, 1.0), // Dark gray text
            code_foreground: gpui::hsla(0.0, 0.0, 0.13, 1.0), // Dark gray for code
            code_comment: gpui::hsla(0.0, 0.0, 0.5, 1.0), // Medium gray for comments
            code_keyword: gpui::hsla(30.0 / 360.0, 0.7, 0.4, 1.0), // Darker orange for keywords
            code_string: gpui::hsla(90.0 / 360.0, 0.5, 0.4, 1.0), // Darker green for strings
            line_numbers: gpui::hsla(0.0, 0.0, 0.6, 1.0), // Medium gray for line numbers

            // Diff highlighting colors (lighter variants)
            addition_background: gpui::hsla(120.0 / 360.0, 0.6, 0.85, 0.5), // Light green with transparency
            addition_foreground: gpui::hsla(120.0 / 360.0, 0.8, 0.3, 1.0),  // Dark green
            deletion_background: gpui::hsla(0.0 / 360.0, 0.6, 0.85, 0.5), // Light red with transparency
            deletion_foreground: gpui::hsla(0.0 / 360.0, 0.8, 0.3, 1.0),  // Dark red
            modification_background: gpui::hsla(210.0 / 360.0, 0.6, 0.85, 0.5), // Light blue with transparency
            modification_foreground: gpui::hsla(210.0 / 360.0, 0.8, 0.3, 1.0),  // Dark blue

            // UI colors
            selection: gpui::hsla(210.0 / 360.0, 0.8, 0.6, 0.4), // Blue selection
            cursor: gpui::hsla(0.0, 0.0, 0.0, 1.0),              // Black cursor

            // Font configuration
            buffer_font_size: 14.0,
            ui_font_size: 12.0,
            line_height_multiplier: 1.6,
        }
    }
}
