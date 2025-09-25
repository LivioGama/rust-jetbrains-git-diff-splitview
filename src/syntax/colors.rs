// src/syntax/colors.rs
// JetBrains color scheme for syntax highlighting - GPUI Native

use super::token_types::TokenType;
use gpui::*;

/// JetBrains color scheme for syntax highlighting
pub struct JetBrainsColors;

impl JetBrainsColors {
    /// Get color for a specific token type using JetBrains IntelliJ color scheme
    pub fn get_color_for_token(token_type: &TokenType) -> gpui::Hsla {
        match token_type {
            TokenType::PlainText => hsla(0.0, 0.0, 0.76, 1.0), // Light gray
            TokenType::Keyword => hsla(30.0 / 360.0, 0.7, 0.6, 1.0), // Orange
            TokenType::String => hsla(90.0 / 360.0, 0.5, 0.6, 1.0), // Green
            TokenType::Number => hsla(210.0 / 360.0, 0.6, 0.7, 1.0), // Light blue
            TokenType::FunctionCall => hsla(210.0 / 360.0, 0.8, 0.75, 1.0), // Light Blue
            TokenType::Comment => hsla(0.0, 0.0, 0.5, 1.0),    // Gray
            TokenType::ClassName => hsla(30.0 / 360.0, 0.7, 0.6, 1.0), // Orange (like keywords)
            TokenType::Constant => hsla(35.0 / 360.0, 0.6, 0.55, 1.0), // Brown
            TokenType::Annotation => hsla(0.0, 0.0, 0.76, 1.0), // Light gray (like punctuation)
            TokenType::Operator => hsla(0.0, 0.0, 0.76, 1.0),  // Light gray
            TokenType::Punctuation => hsla(0.0, 0.0, 0.76, 1.0), // Light gray
            TokenType::JsxTag => hsla(30.0 / 360.0, 0.7, 0.6, 1.0), // Orange
            TokenType::JsxAttribute => hsla(210.0 / 360.0, 0.6, 0.7, 1.0), // Light blue
            TokenType::Parameter => hsla(280.0 / 360.0, 0.6, 0.7, 1.0), // Purple
            TokenType::Property => hsla(280.0 / 360.0, 0.6, 0.7, 1.0), // Purple
        }
    }

    /// Get default text color
    pub fn default_text_color() -> gpui::Hsla {
        hsla(0.0, 0.0, 0.87, 1.0) // Light gray for dark theme
    }

    /// Get comment color
    pub fn comment_color() -> gpui::Hsla {
        hsla(0.0, 0.0, 0.5, 1.0) // Medium gray
    }

    /// Get keyword color
    pub fn keyword_color() -> gpui::Hsla {
        hsla(30.0 / 360.0, 0.7, 0.6, 1.0) // Orange
    }

    /// Get string color  
    pub fn string_color() -> gpui::Hsla {
        hsla(90.0 / 360.0, 0.5, 0.6, 1.0) // Green
    }

    /// Get number color
    pub fn number_color() -> gpui::Hsla {
        hsla(210.0 / 360.0, 0.6, 0.7, 1.0) // Light blue
    }
}
