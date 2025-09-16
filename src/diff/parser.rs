// diffsplit/src/diff/parser.rs
// Diff parsing module

use crate::models::diff::ChangeBlock;
use crate::models::line::LineType;

#[derive(Debug, Clone)]
pub enum Line {
    Context(String),
    Addition(String),
    Deletion(String),
}

#[derive(Debug, Clone)]
pub struct Hunk {
    pub old_start: usize,
    pub new_start: usize,
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone)]
pub struct FileDiff {
    pub filename: String,
    pub hunks: Vec<Hunk>,
}

#[derive(Debug, Clone)]
pub struct Diff {
    pub files: Vec<FileDiff>,
}

pub fn parse_diff(diff_text: &str) -> Diff {
    let mut files = Vec::new();
    let mut current_file: Option<FileDiff> = None;
    let mut current_hunk: Option<Hunk> = None;

    for line in diff_text.lines() {
        // Skip diff header lines that shouldn't be displayed
        if line.starts_with("diff --git") {
            if let Some(file) = current_file.take() {
                files.push(file);
            }
            // Extract filename from "diff --git a/file b/file"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let filename = parts[2].trim_start_matches("a/").to_string();
                current_file = Some(FileDiff {
                    filename,
                    hunks: Vec::new(),
                });
            }
        } else if line.starts_with("index ")
            || line.starts_with("--- ")
            || line.starts_with("+++ ")
            || line.trim().is_empty()
        {
            // Skip these header lines
            continue;
        } else if line.starts_with("@@") {
            if let Some(file) = current_file.as_mut() {
                if let Some(hunk) = current_hunk.take() {
                    file.hunks.push(hunk);
                }
                // Parse @@ -old_start,old_count +new_start,new_count @@
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let old_range = parts[1].trim_start_matches('-');
                    let new_range = parts[2].trim_start_matches('+');
                    let old_start: usize = old_range
                        .split(',')
                        .next()
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0);
                    let new_start: usize = new_range
                        .split(',')
                        .next()
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0);
                    current_hunk = Some(Hunk {
                        old_start,
                        new_start,
                        lines: Vec::new(),
                    });
                }
            }
        } else if let Some(hunk) = current_hunk.as_mut() {
            if line.starts_with(' ') {
                hunk.lines.push(Line::Context(line[1..].to_string()));
            } else if line.starts_with('+') {
                hunk.lines.push(Line::Addition(line[1..].to_string()));
            } else if line.starts_with('-') {
                hunk.lines.push(Line::Deletion(line[1..].to_string()));
            }
        }
    }

    if let Some(file) = current_file.take() {
        if let Some(hunk) = current_hunk.take() {
            let mut f = file;
            f.hunks.push(hunk);
            files.push(f);
        } else {
            files.push(file);
        }
    }

    Diff { files }
}

pub fn create_complete_side_by_side_with_diff(
    original: &str,
    current: &str,
    diff_text: &str,
) -> (
    Vec<crate::models::line::DisplayLine>,
    Vec<crate::models::line::DisplayLine>,
    Vec<ChangeBlock>,
) {
    let original_lines: Vec<&str> = original.lines().collect();
    let current_lines: Vec<&str> = current.lines().collect();

    let diff = parse_diff(diff_text);
    let mut old_display_lines = Vec::new();
    let mut new_display_lines = Vec::new();
    let mut change_blocks = Vec::new();

    // Process each file in the diff
    for file_diff in &diff.files {
        for hunk in &file_diff.hunks {
            // Add context lines before the hunk
            for i in 0..hunk.old_start.saturating_sub(1) {
                if i < original_lines.len() {
                    old_display_lines.push(crate::models::line::DisplayLine {
                        content: original_lines[i].to_string(),
                        line_type: crate::models::line::LineType::Context,
                        original_line_num: Some(i + 1),
                        word_highlights: Vec::new(),
                    });
                }
            }

            for i in 0..hunk.new_start.saturating_sub(1) {
                if i < current_lines.len() {
                    new_display_lines.push(crate::models::line::DisplayLine {
                        content: current_lines[i].to_string(),
                        line_type: crate::models::line::LineType::Context,
                        original_line_num: Some(i + 1),
                        word_highlights: Vec::new(),
                    });
                }
            }

            // Process hunk lines
            let mut old_line_idx = hunk.old_start;
            let mut new_line_idx = hunk.new_start;

            for line in &hunk.lines {
                match line {
                    Line::Context(content) => {
                        if old_line_idx <= original_lines.len() {
                            old_display_lines.push(crate::models::line::DisplayLine {
                                content: content.clone(),
                                line_type: crate::models::line::LineType::Context,
                                original_line_num: Some(old_line_idx),
                                word_highlights: Vec::new(),
                            });
                        }
                        if new_line_idx <= current_lines.len() {
                            new_display_lines.push(crate::models::line::DisplayLine {
                                content: content.clone(),
                                line_type: crate::models::line::LineType::Context,
                                original_line_num: Some(new_line_idx),
                                word_highlights: Vec::new(),
                            });
                        }
                        old_line_idx += 1;
                        new_line_idx += 1;
                    }
                    Line::Addition(content) => {
                        new_display_lines.push(crate::models::line::DisplayLine {
                            content: content.clone(),
                            line_type: crate::models::line::LineType::Addition,
                            original_line_num: Some(new_line_idx),
                            word_highlights: Vec::new(),
                        });
                        new_line_idx += 1;
                    }
                    Line::Deletion(content) => {
                        old_display_lines.push(crate::models::line::DisplayLine {
                            content: content.clone(),
                            line_type: crate::models::line::LineType::Deletion,
                            original_line_num: Some(old_line_idx),
                            word_highlights: Vec::new(),
                        });
                        old_line_idx += 1;
                    }
                }
            }
        }
    }

    // Create change blocks from the diff
    for file_diff in &diff.files {
        for hunk in &file_diff.hunks {
            let mut old_start = hunk.old_start;
            let mut new_start = hunk.new_start;
            let mut old_count = 0;
            let mut new_count = 0;

            for line in &hunk.lines {
                match line {
                    Line::Addition(_) => new_count += 1,
                    Line::Deletion(_) => old_count += 1,
                    Line::Context(_) => {
                        if old_count > 0 || new_count > 0 {
                            change_blocks.push(ChangeBlock {
                                line_type: if old_count > 0 {
                                    crate::models::line::LineType::Deletion
                                } else {
                                    crate::models::line::LineType::Addition
                                },
                                start_line: old_start,
                                end_line: old_start + old_count - 1,
                                left_rects: Vec::new(),
                                right_rects: Vec::new(),
                            });
                            old_start += old_count;
                            new_start += new_count;
                            old_count = 0;
                            new_count = 0;
                        }
                        old_start += 1;
                        new_start += 1;
                    }
                }
            }

            if old_count > 0 || new_count > 0 {
                change_blocks.push(ChangeBlock {
                    line_type: if old_count > 0 {
                        crate::models::line::LineType::Deletion
                    } else {
                        crate::models::line::LineType::Addition
                    },
                    start_line: old_start,
                    end_line: old_start + old_count - 1,
                    left_rects: Vec::new(),
                    right_rects: Vec::new(),
                });
            }
        }
    }

    (old_display_lines, new_display_lines, change_blocks)
}
