pub mod parser;

// Re-export main functions and types from parser
pub use parser::{create_complete_side_by_side_with_diff, DiffOp, DiffResult, JetBrainsDiff};

use crate::models::*;
use std::collections::HashSet;

/// Calculate word-level differences between two strings
pub fn calculate_word_diffs(original: &str, current: &str) -> Vec<(usize, usize)> {
    if original == current {
        return Vec::new();
    }

    let mut highlights = Vec::new();
    let orig_words: Vec<&str> = original.split_whitespace().collect();
    let curr_words: Vec<&str> = current.split_whitespace().collect();

    if orig_words.is_empty() {
        return highlights;
    }

    // Simple word-based diff using position tracking
    let mut orig_pos = 0;
    let mut curr_pos = 0;

    for (i, orig_word) in orig_words.iter().enumerate() {
        // Find the actual position of this word in the original string
        if let Some(word_start) = original[orig_pos..].find(orig_word) {
            let actual_start = orig_pos + word_start;
            let actual_end = actual_start + orig_word.len();

            // Check if this word exists in the corresponding position in current
            let word_differs = if i < curr_words.len() {
                curr_words[i] != *orig_word
            } else {
                true // Word doesn't exist in current version
            };

            if word_differs {
                highlights.push((actual_start, actual_end));
            }

            orig_pos = actual_end;
        }
    }

    highlights
}

/// Detect change blocks from display lines (legacy function for compatibility)
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

            // Determine the primary block type
            let mut has_deletions = false;
            let mut has_additions = false;

            // Check left pane for deletions
            for idx in start..=end {
                if idx < old_lines.len() && old_lines[idx].line_type == LineType::Deletion {
                    has_deletions = true;
                }
            }

            // Check right pane for additions
            for idx in start..=end {
                if idx < new_lines.len() && new_lines[idx].line_type == LineType::Addition {
                    has_additions = true;
                }
            }

            // Determine block type with priority: modifications > deletions > additions
            let block_type = if has_deletions && has_additions {
                LineType::Deletion // Modifications are represented as deletions with corresponding additions
            } else if has_deletions {
                LineType::Deletion
            } else if has_additions {
                LineType::Addition
            } else {
                LineType::Context
            };

            blocks.push(ChangeBlock {
                line_type: block_type,
                start_line: start,
                end_line: end,
                left_rects: Vec::new(),
                right_rects: Vec::new(),
            });

            i = end + 1;
        } else {
            i += 1;
        }
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_diffs() {
        let original = "hello world test";
        let current = "hello universe test";
        let diffs = calculate_word_diffs(original, current);

        // Should highlight "world" since it changed to "universe"
        assert!(!diffs.is_empty());
    }

    #[test]
    fn test_no_word_diffs() {
        let original = "hello world";
        let current = "hello world";
        let diffs = calculate_word_diffs(original, current);

        assert!(diffs.is_empty());
    }

    #[test]
    fn test_create_side_by_side_no_diff() {
        let original = "line1\nline2\nline3";
        let current = "line1\nline2\nline3";
        let diff = "";

        let (old_lines, new_lines, blocks) =
            create_complete_side_by_side_with_diff(original, current, diff);

        assert_eq!(old_lines.len(), 3);
        assert_eq!(new_lines.len(), 3);
        assert_eq!(blocks.len(), 0);

        // All lines should be context
        for line in &old_lines {
            assert_eq!(line.line_type, LineType::Context);
        }
        for line in &new_lines {
            assert_eq!(line.line_type, LineType::Context);
        }
    }
}
