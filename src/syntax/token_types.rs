// src/syntax/token_types.rs
// Token types and structures extracted from syntax/mod.rs

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl ColoredToken {
    pub fn new(text: String, token_type: TokenType, start: usize, end: usize) -> Self {
        Self {
            text,
            token_type,
            start,
            end,
        }
    }

    pub fn plain_text(text: String, start: usize, end: usize) -> Self {
        Self::new(text, TokenType::PlainText, start, end)
    }

    pub fn keyword(text: String, start: usize, end: usize) -> Self {
        Self::new(text, TokenType::Keyword, start, end)
    }

    pub fn string_literal(text: String, start: usize, end: usize) -> Self {
        Self::new(text, TokenType::String, start, end)
    }

    pub fn comment(text: String, start: usize, end: usize) -> Self {
        Self::new(text, TokenType::Comment, start, end)
    }
}
