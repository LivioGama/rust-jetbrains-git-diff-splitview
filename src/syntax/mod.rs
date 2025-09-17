// Syntax highlighting module for JetBrains-style code coloration
use egui::Color32;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    PlainText,
    Keyword,
    String,
    Number,
    FunctionCall,
    Comment,
    ClassName,
    Constant,
    Annotation,
    Operator,
    Punctuation,
    JsxTag,
    JsxAttribute,
    Parameter,
    Property,
}

#[derive(Debug, Clone)]
pub struct ColoredToken {
    pub text: String,
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct SyntaxHighlighter {
    pub keywords: HashMap<String, TokenType>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let mut keywords = HashMap::new();

        // Common keywords across languages
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
            "from",
            "export",
            "default",
            "async",
            "await",
            "try",
            "catch",
            "throw",
            "new",
            "this",
            "super",
            "extends",
            "implements",
            "abstract",
            "void",
            "int",
            "string",
            "bool",
            "boolean",
            "float",
            "double",
            "char",
            "true",
            "false",
            "null",
            "undefined",
            "None",
            "Some",
            "Option",
            "Result",
            "Vec",
            "HashMap",
            "String",
            "i32",
            "i64",
            "u32",
            "u64",
            "f32",
            "f64",
            "use",
            "mod",
            "pub",
            "impl",
            "trait",
            "type",
            "where",
            "match",
            "loop",
            "mut",
            "ref",
            "self",
            "Self",
            "crate",
            "extern",
            "unsafe",
            "move",
        ];

        for keyword in keyword_list {
            keywords.insert(keyword.to_string(), TokenType::Keyword);
        }

        Self { keywords }
    }

    pub fn highlight_line(&self, line: &str) -> Vec<ColoredToken> {
        let mut tokens = Vec::new();
        let mut current_pos = 0;
        let chars: Vec<char> = line.chars().collect();

        while current_pos < chars.len() {
            // Handle whitespace by creating whitespace tokens
            if chars[current_pos].is_whitespace() {
                let start = current_pos;
                let mut end = current_pos;

                // Collect consecutive whitespace characters
                while end < chars.len() && chars[end].is_whitespace() {
                    end += 1;
                }

                let text = chars[start..end].iter().collect::<String>();
                tokens.push(ColoredToken {
                    text,
                    token_type: TokenType::PlainText,
                    start,
                    end,
                });

                current_pos = end;
                continue;
            }

            // Check for comments
            if current_pos < chars.len() - 1 {
                if chars[current_pos] == '/' && chars[current_pos + 1] == '/' {
                    // Line comment
                    let start = current_pos;
                    let text = chars[current_pos..].iter().collect::<String>();
                    tokens.push(ColoredToken {
                        text,
                        token_type: TokenType::Comment,
                        start,
                        end: chars.len(),
                    });
                    break;
                }

                if chars[current_pos] == '/' && chars[current_pos + 1] == '*' {
                    // Block comment start
                    let start = current_pos;
                    let mut end = current_pos + 2;

                    // Find end of block comment or end of line
                    while end < chars.len() - 1 {
                        if chars[end] == '*' && chars[end + 1] == '/' {
                            end += 2;
                            break;
                        }
                        end += 1;
                    }

                    let text = chars[current_pos..end].iter().collect::<String>();
                    tokens.push(ColoredToken {
                        text,
                        token_type: TokenType::Comment,
                        start,
                        end,
                    });
                    current_pos = end;
                    continue;
                }
            }

            // Check for strings
            if chars[current_pos] == '"' || chars[current_pos] == '\'' {
                let quote_char = chars[current_pos];
                let start = current_pos;
                let mut end = current_pos + 1;
                let mut escaped = false;

                while end < chars.len() {
                    if escaped {
                        escaped = false;
                    } else if chars[end] == '\\' {
                        escaped = true;
                    } else if chars[end] == quote_char {
                        end += 1;
                        break;
                    }
                    end += 1;
                }

                let text = chars[current_pos..end].iter().collect::<String>();

                // Check if this is a directive like 'use client' or 'use server'
                let token_type = if text == "'use client'"
                    || text == "'use server'"
                    || text == "\"use client\""
                    || text == "\"use server\""
                {
                    TokenType::Keyword
                } else {
                    TokenType::String
                };

                tokens.push(ColoredToken {
                    text,
                    token_type,
                    start,
                    end,
                });
                current_pos = end;
                continue;
            }

            // Check for numbers
            if chars[current_pos].is_ascii_digit() {
                let start = current_pos;
                let mut end = current_pos;
                let mut has_dot = false;

                while end < chars.len() {
                    if chars[end].is_ascii_digit() {
                        end += 1;
                    } else if chars[end] == '.' && !has_dot {
                        has_dot = true;
                        end += 1;
                    } else if chars[end] == 'f' || chars[end] == 'd' || chars[end] == 'L' {
                        end += 1;
                        break;
                    } else {
                        break;
                    }
                }

                let text = chars[current_pos..end].iter().collect::<String>();
                tokens.push(ColoredToken {
                    text,
                    token_type: TokenType::Number,
                    start,
                    end,
                });
                current_pos = end;
                continue;
            }

            // Check for JSX tags
            if chars[current_pos] == '<' {
                let start = current_pos;
                let mut end = current_pos + 1;
                let mut _is_closing_tag = false;

                // Handle closing tags
                if end < chars.len() && chars[end] == '/' {
                    _is_closing_tag = true;
                    end += 1;
                }

                // Collect tag name
                while end < chars.len()
                    && (chars[end].is_alphanumeric() || chars[end] == '_' || chars[end] == '-')
                {
                    end += 1;
                }

                // Include closing >
                while end < chars.len() && chars[end] != '>' {
                    end += 1;
                }
                if end < chars.len() && chars[end] == '>' {
                    end += 1;
                }

                let text = chars[current_pos..end].iter().collect::<String>();

                // Only treat as JSX if it looks like a valid tag
                if text.len() > 2 && text.ends_with('>') {
                    tokens.push(ColoredToken {
                        text,
                        token_type: TokenType::JsxTag,
                        start,
                        end,
                    });
                    current_pos = end;
                    continue;
                }
            }

            // Check for identifiers and keywords
            if chars[current_pos].is_alphabetic() || chars[current_pos] == '_' {
                let start = current_pos;
                let mut end = current_pos;

                while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
                    end += 1;
                }

                let text = chars[current_pos..end].iter().collect::<String>();

                // Look ahead to determine context
                let mut next_non_space = end;
                while next_non_space < chars.len() && chars[next_non_space].is_whitespace() {
                    next_non_space += 1;
                }

                // Look back to see if we're in function parameters
                let mut prev_non_space = start;
                while prev_non_space > 0 && chars[prev_non_space - 1].is_whitespace() {
                    prev_non_space -= 1;
                }

                let token_type = if next_non_space < chars.len() && chars[next_non_space] == '(' {
                    TokenType::FunctionCall
                } else if next_non_space < chars.len() && chars[next_non_space] == '=' {
                    // Check if we're in JSX (look for JSX context)
                    let line_str = chars.iter().collect::<String>();
                    if line_str.contains('<') && line_str.contains('>') {
                        TokenType::JsxAttribute
                    } else {
                        TokenType::Parameter
                    }
                } else if prev_non_space > 0
                    && (chars[prev_non_space - 1] == '(' || chars[prev_non_space - 1] == ',')
                {
                    TokenType::Parameter
                } else if next_non_space < chars.len() && chars[next_non_space] == ':' {
                    TokenType::Property
                } else if self.keywords.contains_key(&text) {
                    TokenType::Keyword
                } else if text.chars().all(|c| c.is_uppercase() || c == '_') && text.len() > 1 {
                    TokenType::Constant
                } else if text.chars().next().unwrap().is_uppercase() {
                    TokenType::ClassName
                } else {
                    TokenType::PlainText
                };

                tokens.push(ColoredToken {
                    text,
                    token_type,
                    start,
                    end,
                });
                current_pos = end;
                continue;
            }

            // Check for annotations/decorators
            if chars[current_pos] == '@' {
                let start = current_pos;
                let mut end = current_pos + 1;

                while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
                    end += 1;
                }

                let text = chars[current_pos..end].iter().collect::<String>();
                tokens.push(ColoredToken {
                    text,
                    token_type: TokenType::Annotation,
                    start,
                    end,
                });
                current_pos = end;
                continue;
            }

            // Check for operators and punctuation
            let operator_chars = "+-*/=<>!&|^%~.,;:()[]{}";
            if operator_chars.contains(chars[current_pos]) {
                let start = current_pos;
                let mut end = current_pos + 1;

                // Handle multi-character operators
                if current_pos < chars.len() - 1 {
                    let two_char = format!("{}{}", chars[current_pos], chars[current_pos + 1]);
                    if matches!(
                        two_char.as_str(),
                        "==" | "!="
                            | "<="
                            | ">="
                            | "&&"
                            | "||"
                            | "++"
                            | "--"
                            | "=>"
                            | "->"
                            | "::"
                            | "<<"
                            | ">>"
                            | "**"
                    ) {
                        end = current_pos + 2;
                    }
                }

                let text = chars[current_pos..end].iter().collect::<String>();
                let token_type = if "+-*/=<>!&|^%~".contains(chars[current_pos]) {
                    TokenType::Operator
                } else {
                    TokenType::Punctuation
                };

                tokens.push(ColoredToken {
                    text,
                    token_type,
                    start,
                    end,
                });
                current_pos = end;
                continue;
            }

            // Default: single character as plain text
            let text = chars[current_pos].to_string();
            tokens.push(ColoredToken {
                text,
                token_type: TokenType::PlainText,
                start: current_pos,
                end: current_pos + 1,
            });
            current_pos += 1;
        }

        tokens
    }

    pub fn get_color_for_token(&self, token_type: &TokenType) -> Color32 {
        match token_type {
            TokenType::PlainText => Color32::from_rgb(0xBC, 0xBE, 0xC4), // DEFAULT_IDENTIFIER
            TokenType::Keyword => Color32::from_rgb(0xCF, 0x8E, 0x6D),   // DEFAULT_KEYWORD
            TokenType::String => Color32::from_rgb(0x6A, 0xAB, 0x73),    // DEFAULT_STRING
            TokenType::Number => Color32::from_rgb(0x2A, 0xAC, 0xB8),    // DEFAULT_NUMBER
            TokenType::FunctionCall => Color32::from_rgb(0x56, 0xA8, 0xF5), // DEFAULT_FUNCTION_DECLARATION
            TokenType::Comment => Color32::from_rgb(0x7A, 0x7E, 0x85),      // DEFAULT_LINE_COMMENT
            TokenType::ClassName => Color32::from_rgb(0xBC, 0xBE, 0xC4), // DEFAULT_CLASS_REFERENCE
            TokenType::Constant => Color32::from_rgb(0xC7, 0x7D, 0xBB),  // DEFAULT_CONSTANT
            TokenType::Annotation => Color32::from_rgb(0xB3, 0xAE, 0x60), // DEFAULT_METADATA
            TokenType::Operator => Color32::from_rgb(0xBC, 0xBE, 0xC4),  // DEFAULT_OPERATION_SIGN
            TokenType::Punctuation => Color32::from_rgb(0x6F, 0x73, 0x7A), // More muted color for brackets/punctuation
            TokenType::JsxTag => Color32::from_rgb(0x9C, 0x9C, 0xFF), // JS.JSX_CLIENT_COMPONENT
            TokenType::JsxAttribute => Color32::from_rgb(0x56, 0xC1, 0xD6), // KOTLIN_NAMED_ARGUMENT - cyan for JSX attributes
            TokenType::Parameter => Color32::from_rgb(0xBC, 0xBE, 0xC4),    // DEFAULT_IDENTIFIER
            TokenType::Property => Color32::from_rgb(0xC7, 0x7D, 0xBB), // DEFAULT_INSTANCE_FIELD
        }
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_highlighting() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("const myFunction = () => {");

        assert!(!tokens.is_empty());
        assert!(tokens
            .iter()
            .any(|t| t.token_type == TokenType::Keyword && t.text == "const"));
    }

    #[test]
    fn test_string_highlighting() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("let message = \"Hello, World!\";");

        assert!(tokens
            .iter()
            .any(|t| t.token_type == TokenType::String && t.text == "\"Hello, World!\""));
    }

    #[test]
    fn test_comment_highlighting() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("// This is a comment");

        assert!(tokens.iter().any(|t| t.token_type == TokenType::Comment));
    }

    #[test]
    fn test_number_highlighting() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("let count = 42;");

        assert!(tokens
            .iter()
            .any(|t| t.token_type == TokenType::Number && t.text == "42"));
    }

    #[test]
    fn test_function_call_highlighting() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("console.log(\"test\");");

        assert!(tokens
            .iter()
            .any(|t| t.token_type == TokenType::FunctionCall));
    }
}
