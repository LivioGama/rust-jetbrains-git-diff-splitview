// Syntax highlighting module for JetBrains-style code coloration

pub mod colors;
pub mod highlighter;
pub mod token_types;

pub use highlighter::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_highlighter_creation() {
        let highlighter = SyntaxHighlighter::new();
        assert!(!highlighter.keywords.is_empty());
    }

    #[test]
    fn test_token_highlighting() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("function test() { return true; }");
        assert!(!tokens.is_empty());

        // Should detect 'function' as keyword
        let function_token = tokens.iter().find(|t| t.text == "function");
        assert!(function_token.is_some());
        assert_eq!(function_token.unwrap().token_type, TokenType::Keyword);
    }

    #[test]
    fn test_empty_line() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_jetbrains_colors() {
        let keyword_color = JetBrainsColors::get_color_for_token(&TokenType::Keyword);
        let string_color = JetBrainsColors::get_color_for_token(&TokenType::String);

        // Colors should be different
        assert_ne!(keyword_color, string_color);
    }
}
