// diffsplit/src/models/line/mod.rs
// Line-related data structures and types

#[derive(Debug, Clone, PartialEq)]
pub enum LineType {
    Context,
    Addition,
    Deletion,
    Modification,
    Empty,
}

#[derive(Debug, Clone)]
pub struct DisplayLine {
    pub content: String,
    pub line_type: LineType,
    pub original_line_num: Option<usize>,
    pub word_highlights: Vec<(usize, usize)>,
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

    pub fn with_word_highlights(mut self, highlights: Vec<(usize, usize)>) -> Self {
        self.word_highlights = highlights;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn len(&self) -> usize {
        self.content.len()
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
