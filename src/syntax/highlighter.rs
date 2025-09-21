// src/syntax/highlighter.rs
// Syntax highlighter extracted from syntax/mod.rs

use super::colors::JetBrainsColors;
use super::token_types::{ColoredToken, TokenType};
use egui::Color32;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SyntaxHighlighter {
    pub keywords: HashMap<String, TokenType>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let mut keywords = HashMap::new();

        // Essential keywords for common languages
        let keyword_list = vec![
            "if",
            "else",
            "for",
            "while",
            "do",
            "switch",
            "case",
            "break",
            "continue",
            "return",
            "function",
            "fn",
            "def",
            "class",
            "struct",
            "enum",
            "interface",
            "public",
            "private",
            "protected",
            "static",
            "final",
            "const",
            "let",
            "var",
            "import",
            "export",
            "from",
            "as",
            "try",
            "catch",
            "throw",
            "async",
            "await",
            "yield",
            "true",
            "false",
            "null",
            "undefined",
            "this",
            "super",
            "new",
            "delete",
            "typeof",
            "instanceof",
            "in",
            "of",
            // TypeScript/JavaScript specific
            "type",
            "namespace",
            "module",
            "declare",
            "abstract",
            "readonly",
            "never",
            "any",
            "unknown",
            "string",
            "number",
            "boolean",
            "object",
            "symbol",
            "bigint",
            // React/JSX
            "React",
            "Component",
            "useState",
            "useEffect",
            "useContext",
            "useMemo",
            "useCallback",
            "useState",
            "useReducer",
            "useRef",
            "forwardRef",
            "memo",
            "lazy",
            "Suspense",
        ];

        for keyword in keyword_list {
            keywords.insert(keyword.to_string(), TokenType::Keyword);
        }

        Self { keywords }
    }

    /// Highlight a line of text and return colored tokens
    pub fn highlight_line(&self, line: &str) -> Vec<ColoredToken> {
        if line.trim().is_empty() {
            return vec![];
        }

        // Simple tokenization based on common patterns
        let mut tokens = Vec::new();
        let mut chars = line.char_indices().peekable();
        let mut current_token = String::new();
        let mut token_start = 0;
        let mut current_type = TokenType::PlainText;

        while let Some((pos, ch)) = chars.next() {
            if current_token.is_empty() {
                token_start = pos;
            }

            match ch {
                // Handle strings
                '"' | '\'' => {
                    if !current_token.is_empty() && current_type != TokenType::String {
                        self.push_token(&mut tokens, current_token, current_type, token_start, pos);
                        current_token = String::new();
                        token_start = pos;
                    }
                    current_token.push(ch);
                    current_type = TokenType::String;

                    // Continue until closing quote
                    while let Some((_, next_ch)) = chars.next() {
                        current_token.push(next_ch);
                        if next_ch == ch && !current_token.ends_with("\\") {
                            break;
                        }
                    }
                }
                // Handle comments
                '/' if chars.peek().map(|(_, c)| *c) == Some('/') => {
                    if !current_token.is_empty() {
                        self.push_token(&mut tokens, current_token, current_type, token_start, pos);
                        token_start = pos;
                    }
                    // Rest of line is comment
                    current_token = line[pos..].to_string();
                    current_type = TokenType::Comment;
                    break;
                }
                // Handle numbers
                c if c.is_ascii_digit() => {
                    if current_type != TokenType::Number && !current_token.is_empty() {
                        self.push_token(&mut tokens, current_token, current_type, token_start, pos);
                        current_token = String::new();
                        token_start = pos;
                    }
                    current_token.push(ch);
                    current_type = TokenType::Number;
                }
                // Handle identifiers and keywords
                c if c.is_ascii_alphanumeric() || c == '_' => {
                    if current_type == TokenType::Number
                        || (current_type != TokenType::PlainText
                            && current_type != TokenType::Keyword)
                    {
                        if !current_token.is_empty() {
                            self.push_token(
                                &mut tokens,
                                current_token,
                                current_type,
                                token_start,
                                pos,
                            );
                            current_token = String::new();
                            token_start = pos;
                        }
                    }
                    current_token.push(ch);
                    current_type = if self.keywords.contains_key(&current_token) {
                        TokenType::Keyword
                    } else {
                        TokenType::PlainText
                    };
                }
                // Handle operators and punctuation
                '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' | '&' | '|' | '^' | '%' => {
                    if !current_token.is_empty() {
                        self.push_token(&mut tokens, current_token, current_type, token_start, pos);
                        current_token = String::new();
                        token_start = pos;
                    }
                    current_token.push(ch);
                    current_type = TokenType::Operator;
                }
                // Handle punctuation
                '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' | '.' | ':' => {
                    if !current_token.is_empty() {
                        self.push_token(&mut tokens, current_token, current_type, token_start, pos);
                        current_token = String::new();
                        token_start = pos;
                    }
                    current_token.push(ch);
                    current_type = TokenType::Punctuation;
                }
                // Whitespace - finalize current token
                c if c.is_whitespace() => {
                    if !current_token.is_empty() {
                        self.push_token(&mut tokens, current_token, current_type, token_start, pos);
                        current_token = String::new();
                    }
                    // Don't include whitespace in tokens
                }
                // Default case
                _ => {
                    current_token.push(ch);
                    if current_type == TokenType::Number {
                        current_type = TokenType::PlainText;
                    }
                }
            }
        }

        // Push final token
        if !current_token.is_empty() {
            let end_pos = line.len();
            self.push_token(
                &mut tokens,
                current_token,
                current_type,
                token_start,
                end_pos,
            );
        }

        tokens
    }

    fn push_token(
        &self,
        tokens: &mut Vec<ColoredToken>,
        token_text: String,
        mut token_type: TokenType,
        start: usize,
        end: usize,
    ) {
        // Check if it's a keyword after full token is built
        if token_type == TokenType::PlainText && self.keywords.contains_key(&token_text) {
            token_type = TokenType::Keyword;
        }

        tokens.push(ColoredToken::new(token_text, token_type, start, end));
    }

    /// Get color for a token type
    pub fn get_color_for_token(&self, token_type: &TokenType) -> Color32 {
        JetBrainsColors::get_color_for_token(token_type)
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}
