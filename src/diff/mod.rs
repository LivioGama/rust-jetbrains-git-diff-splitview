pub mod parser;

pub use parser::*;

use crate::models::*;
use std::collections::HashSet;

pub fn create_complete_side_by_side_with_diff(
    original: &str,
    current: &str,
    diff_text: &str,
) -> (Vec<DisplayLine>, Vec<DisplayLine>, Vec<ChangeBlock>) {
    let original_lines: Vec<&str> = original.lines().collect();
    let current_lines: Vec<&str> = current.lines().collect();

    // Parse diff to identify changed sections using correct line number tracking
    let diff = parser::parse_diff(diff_text);
    let mut changed_lines = HashSet::new();
    let mut addition_lines = HashSet::new();
    let mut deletion_lines = HashSet::new();

    if !diff.files.is_empty() {
        for hunk in &diff.files[0].hunks {
            let mut old_line = hunk.old_start;
            let mut new_line = hunk.new_start;

            for line in &hunk.lines {
                match line {
                    parser::Line::Context(_) => {
                        // Context lines are unchanged, just advance line counters
                        old_line += 1;
                        new_line += 1;
                    }
                    parser::Line::Deletion(_) => {
                        // Deletion exists at old_line in original file
                        if old_line <= original_lines.len() {
                            let index = old_line as usize;
                            deletion_lines.insert(index);
                            changed_lines.insert(index);
                        }
                        old_line += 1;
                    }
                    parser::Line::Addition(_) => {
                        // Addition exists at new_line in current file
                        if new_line <= current_lines.len() {
                            let index = new_line as usize;
                            addition_lines.insert(index);
                            changed_lines.insert(index);
                        }
                        new_line += 1;
                    }
                }
            }
        }
    }

    let mut old_lines = Vec::new();
    let mut new_lines = Vec::new();

    let max_lines = original_lines.len().max(current_lines.len());

    for i in 0..max_lines {
        let orig_line = original_lines.get(i);
        let curr_line = current_lines.get(i);

        match (orig_line, curr_line) {
            (Some(o), Some(c)) => {
                let is_changed = changed_lines.contains(&i);
                let is_addition = addition_lines.contains(&i);
                let is_deletion = deletion_lines.contains(&i);

                // Determine line type based on diff analysis
                let (old_line_type, new_line_type) = if is_deletion && is_addition {
                    // This line was modified (deletion + addition at same position)
                    (LineType::Deletion, LineType::Addition)
                } else if is_deletion {
                    // Pure deletion - line only exists in old
                    (LineType::Deletion, LineType::Empty)
                } else if is_addition {
                    // Pure addition - line only exists in new
                    (LineType::Empty, LineType::Addition)
                } else if is_changed {
                    // Context line in changed region
                    (LineType::Context, LineType::Context)
                } else {
                    // Unchanged line
                    (LineType::Context, LineType::Context)
                };

                // Calculate word-level highlights for modifications
                let word_highlights = if o != c && (is_deletion || is_addition) {
                    calculate_word_diffs(o, c)
                } else {
                    Vec::new()
                };

                old_lines.push(DisplayLine {
                    content: o.to_string(),
                    line_type: old_line_type,
                    original_line_num: Some(i + 1),
                    word_highlights: word_highlights.clone(),
                });
                new_lines.push(DisplayLine {
                    content: c.to_string(),
                    line_type: new_line_type,
                    original_line_num: Some(i + 1),
                    word_highlights,
                });
            }
            (Some(o), None) => {
                old_lines.push(DisplayLine {
                    content: o.to_string(),
                    line_type: LineType::Deletion,
                    original_line_num: Some(i + 1),
                    word_highlights: Vec::new(),
                });
                new_lines.push(DisplayLine {
                    content: "".to_string(),
                    line_type: LineType::Empty,
                    original_line_num: None,
                    word_highlights: Vec::new(),
                });
            }
            (None, Some(c)) => {
                old_lines.push(DisplayLine {
                    content: "".to_string(),
                    line_type: LineType::Empty,
                    original_line_num: None,
                    word_highlights: Vec::new(),
                });
                new_lines.push(DisplayLine {
                    content: c.to_string(),
                    line_type: LineType::Addition,
                    original_line_num: Some(i + 1),
                    word_highlights: Vec::new(),
                });
            }
            (None, None) => {
                // This shouldn't happen in normal diff scenarios, but handle it gracefully
            }
        }
    }

    // Create change blocks for trapezoidal connectors
    let change_blocks = detect_change_blocks(&old_lines, &new_lines);

    (old_lines, new_lines, change_blocks)
}

pub fn calculate_word_diffs(original: &str, current: &str) -> Vec<(usize, usize)> {
    let mut highlights = Vec::new();

    if original.is_empty() || current.is_empty() {
        return highlights;
    }

    // Improved word-based diff using simple LCS approach
    let orig_words: Vec<&str> = original.split_whitespace().collect();
    let curr_words: Vec<&str> = current.split_whitespace().collect();

    // Find differences using a simple approach
    let mut orig_idx = 0;
    let mut curr_idx = 0;

    while orig_idx < orig_words.len() && curr_idx < curr_words.len() {
        if orig_words[orig_idx] == curr_words[curr_idx] {
            // Words match, move both indices
            orig_idx += 1;
            curr_idx += 1;
        } else {
            // Words differ, mark the original word as changed
            let word_start = original
                .split_whitespace()
                .take(orig_idx)
                .map(|w| w.len() + 1)
                .sum::<usize>();

            let word_len = orig_words[orig_idx].len();
            highlights.push((word_start, word_start + word_len));

            // Try to find this word later in current
            let mut found = false;
            for j in curr_idx..curr_words.len() {
                if orig_words[orig_idx] == curr_words[j] {
                    curr_idx = j + 1;
                    found = true;
                    break;
                }
            }

            if !found {
                curr_idx += 1;
            }
            orig_idx += 1;
        }
    }

    // Handle remaining words in original
    while orig_idx < orig_words.len() {
        let word_start = original
            .split_whitespace()
            .take(orig_idx)
            .map(|w| w.len() + 1)
            .sum::<usize>();

        let word_len = orig_words[orig_idx].len();
        highlights.push((word_start, word_start + word_len));
        orig_idx += 1;
    }

    highlights
}

pub fn detect_change_blocks(
    old_lines: &[DisplayLine],
    new_lines: &[DisplayLine],
) -> Vec<ChangeBlock> {
    let mut blocks = Vec::new();
    let mut i = 0;
    let max_lines = old_lines.len().max(new_lines.len());

    while i < max_lines {
        // Check if this line has any changes in either pane
        let old_has_change = i < old_lines.len() && old_lines[i].line_type != LineType::Context;
        let new_has_change = i < new_lines.len() && new_lines[i].line_type != LineType::Context;

        if old_has_change || new_has_change {
            let start = i;
            let mut end = i;

            // Find the end of this contiguous change block
            while end + 1 < max_lines {
                let next_old_has_change =
                    end + 1 < old_lines.len() && old_lines[end + 1].line_type != LineType::Context;
                let next_new_has_change =
                    end + 1 < new_lines.len() && new_lines[end + 1].line_type != LineType::Context;

                if !next_old_has_change && !next_new_has_change {
                    break;
                }
                end += 1;
            }

            // Calculate actual line ranges for left and right panes
            let left_start = start.min(old_lines.len().saturating_sub(1));
            let left_end = end.min(old_lines.len().saturating_sub(1));
            let right_start = start.min(new_lines.len().saturating_sub(1));
            let right_end = end.min(new_lines.len().saturating_sub(1));

            // Determine the primary block type with better logic
            let mut has_deletions = false;
            let mut has_additions = false;
            let mut has_modifications = false;

            // Check left pane for deletions
            for idx in left_start..=left_end {
                if idx < old_lines.len() && old_lines[idx].line_type == LineType::Deletion {
                    has_deletions = true;
                }
            }

            // Check right pane for additions
            for idx in right_start..=right_end {
                if idx < new_lines.len() && new_lines[idx].line_type == LineType::Addition {
                    has_additions = true;
                }
            }

            // Check for modifications (context lines with word highlights)
            for idx in left_start..=left_end {
                if idx < old_lines.len() && !old_lines[idx].word_highlights.is_empty() {
                    has_modifications = true;
                }
            }

            // Determine block type with priority: deletions > additions > modifications
            let block_type = if has_deletions {
                LineType::Deletion
            } else if has_additions {
                LineType::Addition
            } else if has_modifications {
                LineType::Context // Modifications are marked as Context with highlights
            } else {
                LineType::Context
            };

            blocks.push(ChangeBlock {
                line_type: block_type,
                start_line: left_start,
                end_line: left_end,
                left_rects: Vec::new(),
                right_rects: Vec::new(),
            });

            i = end + 1;
        } else {
            i += 1;
        }
    }

    // Post-process blocks to merge adjacent blocks of the same type
    let mut merged_blocks = Vec::new();
    let mut current_block: Option<ChangeBlock> = None;

    for block in blocks {
        if let Some(ref mut curr) = current_block {
            // Check if we can merge with the current block
            let can_merge =
                curr.line_type == block.line_type && curr.end_line + 1 >= block.start_line;

            if can_merge {
                // Merge the blocks
                curr.end_line = curr.end_line.max(block.end_line);
            } else {
                // Can't merge, push current and start new
                merged_blocks.push(current_block.take().unwrap());
                current_block = Some(block);
            }
        } else {
            current_block = Some(block);
        }
    }

    // Push the last block
    if let Some(block) = current_block {
        merged_blocks.push(block);
    }

    merged_blocks
}
