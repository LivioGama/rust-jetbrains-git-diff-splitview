#[allow(dead_code)]
// diffsplit/src/models/line/mod.rs
// Line-related data structures and types
#[derive(Debug, Clone, PartialEq)]
pub enum HighlightType {
    Insert,
    Delete,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineType {
    Context,
    Addition,
    Deletion,
    Modification,
}

#[derive(Debug, Clone)]
pub struct DisplayLine {
    pub content: String,
    pub line_type: LineType,
    pub original_line_num: Option<usize>,
    pub word_highlights: Vec<(usize, usize, HighlightType)>,
}

impl DisplayLine {
    pub fn new(content: String, line_type: LineType) -> Self {
        Self {
            content,
            line_type,
            original_line_num: None,
            word_highlights: Vec::new(),
        }
    }

    pub fn with_line_number(mut self, line_num: usize) -> Self {
        self.original_line_num = Some(line_num);
        self
    }
}

impl Default for DisplayLine {
    fn default() -> Self {
        Self {
            content: String::new(),
            line_type: LineType::Context,
            original_line_num: None,
            word_highlights: Vec::new(),
        }
    }
}
