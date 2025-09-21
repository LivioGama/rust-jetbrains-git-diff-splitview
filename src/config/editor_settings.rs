// Comprehensive editor settings based on Zed IDE specifications
// From crates/editor/src/editor_settings.rs and crates/language/src/language_settings.rs

/// Editor behavior settings matching Zed's defaults
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZedEditorSettings {
    pub cursor_blink: bool,
    pub vertical_scroll_margin: u32,
    pub horizontal_scroll_margin: u32,
    pub scroll_sensitivity: f32,
}

/// Current line highlight options
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum CurrentLineHighlight {
    All, // Default
}

/// Scroll beyond last line options
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ScrollBeyondLastLine {
    OnePage, // Default
}

/// Multi-cursor modifier key
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum MultiCursorModifier {
    Alt, // Default
}

/// Seed search query behavior
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum SeedSearchQuery {
    Always, // Default
}

/// Go to definition fallback behavior
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum GoToDefinitionFallback {
    FindAllReferences, // Default
}

/// LSP document colors display mode
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum LspDocumentColors {
    Inlay, // Default
}

impl Default for ZedEditorSettings {
    fn default() -> Self {
        Self {
            cursor_blink: true,
            vertical_scroll_margin: 3,
            horizontal_scroll_margin: 5,
            scroll_sensitivity: 1.0,
        }
    }
}

/// Language/Buffer settings matching Zed's defaults
/// From crates/language/src/language_settings.rs
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZedLanguageSettings {
    // Indentation
    pub tab_size: u32,   // Default 4 columns
    pub hard_tabs: bool, // Default false (uses spaces)
}

/// Soft wrap options
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum SoftWrap {
    None, // Default
}

/// Format on save options
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum FormatOnSave {
    On, // Default
}

impl Default for ZedLanguageSettings {
    fn default() -> Self {
        Self {
            tab_size: 4,
            hard_tabs: false, // Uses spaces by default
        }
    }
}

/// Complete Zed settings configuration
#[allow(dead_code)]
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

#[allow(dead_code)]
impl ZedSettings {
    /// Create new Zed settings with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Get editor settings
    pub fn editor(&self) -> &ZedEditorSettings {
        &self.editor
    }

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
