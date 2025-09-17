// Comprehensive editor settings based on Zed IDE specifications
// From crates/editor/src/editor_settings.rs and crates/language/src/language_settings.rs

/// Editor behavior settings matching Zed's defaults
#[derive(Debug, Clone)]
pub struct ZedEditorSettings {
    // Cursor and Selection
    pub cursor_blink: bool,                           // Default true
    pub current_line_highlight: CurrentLineHighlight, // Default All
    pub selection_highlight: bool,                    // Default true
    pub rounded_selection: bool,                      // Default true
    pub relative_line_numbers: bool,                  // Default false

    // Hover and Popover
    pub hover_popover_enabled: bool,    // Default true
    pub hover_popover_delay_ms: u32,    // Default 300ms
    pub lsp_highlight_debounce_ms: u32, // Default 75ms

    // Scrolling
    pub scroll_beyond_last_line: ScrollBeyondLastLine, // Default OnePagep
    pub vertical_scroll_margin: u32,                   // Default 3 lines
    pub horizontal_scroll_margin: u32,                 // Default 5 characters
    pub scroll_sensitivity: f32,                       // Default 1.0
    pub fast_scroll_sensitivity: f32,                  // Default 4.0 (with alt/option)

    // Search
    pub search_wrap: bool,                              // Default true
    pub seed_search_query_from_cursor: SeedSearchQuery, // Default Always

    // Input and Interaction
    pub middle_click_paste: bool,                   // Default true
    pub multi_cursor_modifier: MultiCursorModifier, // Default Alt
    pub drag_and_drop_selection: bool,              // Default true
    pub drag_and_drop_delay_ms: u32,                // Default 300ms

    // Code Actions and LSP
    pub auto_signature_help: bool,             // Default false
    pub show_signature_help_after_edits: bool, // Default false
    pub go_to_definition_fallback: GoToDefinitionFallback, // Default FindAllReferences
    pub inline_code_actions: bool,             // Default true
    pub lsp_document_colors: LspDocumentColors, // Default Inlay

    // Visual
    pub minimum_contrast_for_highlights: u8, // Default 45 (APCA perceptual contrast)
}

/// Current line highlight options
#[derive(Debug, Clone, PartialEq)]
pub enum CurrentLineHighlight {
    None,
    Gutter,
    Line,
    All, // Default
}

impl CurrentLineHighlight {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "none" => Self::None,
            "gutter" => Self::Gutter,
            "line" => Self::Line,
            "all" => Self::All,
            _ => Self::All, // Default fallback
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Gutter => "gutter",
            Self::Line => "line",
            Self::All => "all",
        }
    }
}

/// Scroll beyond last line options
#[derive(Debug, Clone, PartialEq)]
pub enum ScrollBeyondLastLine {
    Off,
    OnePage, // Default
    VerticalScrollMargin,
}

impl ScrollBeyondLastLine {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "off" => Self::Off,
            "one_page" => Self::OnePage,
            "vertical_scroll_margin" => Self::VerticalScrollMargin,
            _ => Self::OnePage, // Default fallback
        }
    }
}

/// Multi-cursor modifier key
#[derive(Debug, Clone, PartialEq)]
pub enum MultiCursorModifier {
    Alt, // Default
    Cmd,
    Ctrl,
}

/// Seed search query behavior
#[derive(Debug, Clone, PartialEq)]
pub enum SeedSearchQuery {
    Never,
    Selection,
    Always, // Default
}

/// Go to definition fallback behavior
#[derive(Debug, Clone, PartialEq)]
pub enum GoToDefinitionFallback {
    None,
    FindAllReferences, // Default
}

/// LSP document colors display mode
#[derive(Debug, Clone, PartialEq)]
pub enum LspDocumentColors {
    None,
    Inlay, // Default
    Border,
    Background,
}

impl Default for ZedEditorSettings {
    fn default() -> Self {
        Self {
            // Cursor and Selection - exact Zed defaults
            cursor_blink: true,
            current_line_highlight: CurrentLineHighlight::All,
            selection_highlight: true,
            rounded_selection: true,
            relative_line_numbers: false,

            // Hover and Popover
            hover_popover_enabled: true,
            hover_popover_delay_ms: 300,
            lsp_highlight_debounce_ms: 75,

            // Scrolling
            scroll_beyond_last_line: ScrollBeyondLastLine::OnePage,
            vertical_scroll_margin: 3,
            horizontal_scroll_margin: 5,
            scroll_sensitivity: 1.0,
            fast_scroll_sensitivity: 4.0,

            // Search
            search_wrap: true,
            seed_search_query_from_cursor: SeedSearchQuery::Always,

            // Input and Interaction
            middle_click_paste: true,
            multi_cursor_modifier: MultiCursorModifier::Alt,
            drag_and_drop_selection: true,
            drag_and_drop_delay_ms: 300,

            // Code Actions and LSP
            auto_signature_help: false,
            show_signature_help_after_edits: false,
            go_to_definition_fallback: GoToDefinitionFallback::FindAllReferences,
            inline_code_actions: true,
            lsp_document_colors: LspDocumentColors::Inlay,

            // Visual
            minimum_contrast_for_highlights: 45,
        }
    }
}

/// Language/Buffer settings matching Zed's defaults
/// From crates/language/src/language_settings.rs
#[derive(Debug, Clone)]
pub struct ZedLanguageSettings {
    // Indentation
    pub tab_size: u32,   // Default 4 columns
    pub hard_tabs: bool, // Default false (uses spaces)

    // Line wrapping
    pub preferred_line_length: u32, // Default 80 columns
    pub soft_wrap: SoftWrap,        // Default None
    pub show_wrap_guides: bool,     // Default true

    // Formatting
    pub format_on_save: FormatOnSave,             // Default On
    pub remove_trailing_whitespace_on_save: bool, // Default true
    pub ensure_final_newline_on_save: bool,       // Default true
}

/// Soft wrap options
#[derive(Debug, Clone, PartialEq)]
pub enum SoftWrap {
    None, // Default
    EditorWidth,
    PreferredLineLength,
    Bounded,
}

/// Format on save options
#[derive(Debug, Clone, PartialEq)]
pub enum FormatOnSave {
    Off,
    On, // Default
    External,
    LanguageServer,
}

impl Default for ZedLanguageSettings {
    fn default() -> Self {
        Self {
            // Indentation - exact Zed defaults
            tab_size: 4,
            hard_tabs: false, // Uses spaces by default

            // Line wrapping
            preferred_line_length: 80,
            soft_wrap: SoftWrap::None,
            show_wrap_guides: true,

            // Formatting
            format_on_save: FormatOnSave::On,
            remove_trailing_whitespace_on_save: true,
            ensure_final_newline_on_save: true,
        }
    }
}

/// Complete Zed settings configuration
#[derive(Debug, Clone)]
pub struct ZedSettings {
    pub editor: ZedEditorSettings,
    pub language: ZedLanguageSettings,
}

impl Default for ZedSettings {
    fn default() -> Self {
        Self {
            editor: ZedEditorSettings::default(),
            language: ZedLanguageSettings::default(),
        }
    }
}

impl ZedSettings {
    /// Create new Zed settings with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Get editor settings
    pub fn editor(&self) -> &ZedEditorSettings {
        &self.editor
    }

    /// Get language settings
    pub fn language(&self) -> &ZedLanguageSettings {
        &self.language
    }

    /// Update editor settings
    pub fn with_editor_settings<F>(mut self, updater: F) -> Self
    where
        F: FnOnce(&mut ZedEditorSettings),
    {
        updater(&mut self.editor);
        self
    }

    /// Update language settings
    pub fn with_language_settings<F>(mut self, updater: F) -> Self
    where
        F: FnOnce(&mut ZedLanguageSettings),
    {
        updater(&mut self.language);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_settings_defaults() {
        let settings = ZedEditorSettings::default();

        assert!(settings.cursor_blink);
        assert_eq!(settings.current_line_highlight, CurrentLineHighlight::All);
        assert!(settings.selection_highlight);
        assert!(settings.rounded_selection);
        assert!(!settings.relative_line_numbers);
        assert_eq!(settings.hover_popover_delay_ms, 300);
        assert_eq!(settings.lsp_highlight_debounce_ms, 75);
        assert_eq!(settings.vertical_scroll_margin, 3);
        assert_eq!(settings.horizontal_scroll_margin, 5);
        assert_eq!(settings.scroll_sensitivity, 1.0);
        assert_eq!(settings.fast_scroll_sensitivity, 4.0);
        assert!(settings.search_wrap);
        assert!(!settings.auto_signature_help);
        assert!(!settings.show_signature_help_after_edits);
        assert!(settings.inline_code_actions);
        assert_eq!(settings.minimum_contrast_for_highlights, 45);
    }

    #[test]
    fn test_language_settings_defaults() {
        let settings = ZedLanguageSettings::default();

        assert_eq!(settings.tab_size, 4);
        assert!(!settings.hard_tabs);
        assert_eq!(settings.preferred_line_length, 80);
        assert_eq!(settings.soft_wrap, SoftWrap::None);
        assert!(settings.show_wrap_guides);
        assert_eq!(settings.format_on_save, FormatOnSave::On);
        assert!(settings.remove_trailing_whitespace_on_save);
        assert!(settings.ensure_final_newline_on_save);
    }

    #[test]
    fn test_current_line_highlight_from_str() {
        assert_eq!(
            CurrentLineHighlight::from_str("all"),
            CurrentLineHighlight::All
        );
        assert_eq!(
            CurrentLineHighlight::from_str("none"),
            CurrentLineHighlight::None
        );
        assert_eq!(
            CurrentLineHighlight::from_str("gutter"),
            CurrentLineHighlight::Gutter
        );
        assert_eq!(
            CurrentLineHighlight::from_str("line"),
            CurrentLineHighlight::Line
        );
        assert_eq!(
            CurrentLineHighlight::from_str("invalid"),
            CurrentLineHighlight::All
        );
    }

    #[test]
    fn test_zed_settings_builder() {
        let settings = ZedSettings::new()
            .with_editor_settings(|editor| {
                editor.cursor_blink = false;
                editor.vertical_scroll_margin = 5;
            })
            .with_language_settings(|language| {
                language.tab_size = 2;
                language.hard_tabs = true;
            });

        assert!(!settings.editor.cursor_blink);
        assert_eq!(settings.editor.vertical_scroll_margin, 5);
        assert_eq!(settings.language.tab_size, 2);
        assert!(settings.language.hard_tabs);
    }
}
