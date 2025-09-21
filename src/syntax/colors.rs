// src/syntax/colors.rs
// JetBrains color scheme for syntax highlighting

use egui::Color32;
use super::token_types::TokenType;

/// JetBrains color scheme for syntax highlighting
pub struct JetBrainsColors;

impl JetBrainsColors {
    /// Get color for a specific token type using JetBrains IntelliJ color scheme
    pub fn get_color_for_token(token_type: &TokenType) -> Color32 {
        match token_type {
            TokenType::PlainText => Color32::from_rgb(169, 183, 198),     // Light gray
            TokenType::Keyword => Color32::from_rgb(204, 120, 50),        // Orange
            TokenType::String => Color32::from_rgb(106, 135, 89),         // Green
            TokenType::Number => Color32::from_rgb(104, 151, 187),        // Light blue
            TokenType::FunctionCall => Color32::from_rgb(120, 150, 220),  // Light Blue (like VS Code)
            TokenType::Comment => Color32::from_rgb(128, 128, 128),       // Gray
            TokenType::ClassName => Color32::from_rgb(204, 120, 50),      // Orange (like keywords)
            TokenType::Constant => Color32::from_rgb(154, 110, 58),       // Brown
            TokenType::Annotation => Color32::from_rgb(169, 183, 198),    // Light gray (like punctuation)
            TokenType::Operator => Color32::from_rgb(169, 183, 198),      // Light gray
            TokenType::Punctuation => Color32::from_rgb(169, 183, 198),   // Light gray
            TokenType::JsxTag => Color32::from_rgb(204, 120, 50),         // Orange
            TokenType::JsxAttribute => Color32::from_rgb(104, 151, 187),  // Light blue (like numbers)
            TokenType::Parameter => Color32::from_rgb(152, 118, 170),     // Purple
            TokenType::Property => Color32::from_rgb(152, 118, 170),      // Purple
        }
    }

    /// Get all supported token types
    pub fn supported_token_types() -> Vec<TokenType> {
        vec![
            TokenType::PlainText,
            TokenType::Keyword,
            TokenType::String,
            TokenType::Number,
            TokenType::FunctionCall,
            TokenType::Comment,
            TokenType::ClassName,
            TokenType::Constant,
            TokenType::Annotation,
            TokenType::Operator,
            TokenType::Punctuation,
            TokenType::JsxTag,
            TokenType::JsxAttribute,
            TokenType::Parameter,
            TokenType::Property,
        ]
    }

    /// Check if a token type should be bold
    pub fn is_bold(token_type: &TokenType) -> bool {
        matches!(token_type, TokenType::Keyword | TokenType::ClassName)
    }

    /// Check if a token type should be italic
    pub fn is_italic(token_type: &TokenType) -> bool {
        matches!(token_type, TokenType::Comment | TokenType::Annotation)
    }
}
