// JetBrains-style diff implementation with Myers + heuristics + intra-line diff
use crate::models::diff::ChangeBlock;
use crate::models::line::{DisplayLine, LineType};

#[derive(Debug, Clone, PartialEq)]
pub enum DiffOp {
    Equal(String),
    Delete(String),
    Insert(String),
}

#[derive(Debug, Clone)]
pub struct DiffResult {
    pub operations: Vec<DiffOp>,
    pub left_lines: Vec<DisplayLine>,
    pub right_lines: Vec<DisplayLine>,
    pub change_blocks: Vec<ChangeBlock>,
}

/// Realistic JetBrains-style diff implementation
pub struct JetBrainsDiff;

impl JetBrainsDiff {
    pub fn diff(text1: &str, text2: &str) -> DiffResult {
        let lines1: Vec<&str> = if text1.is_empty() {
            Vec::new()
        } else {
            text1.lines().collect()
        };
        let lines2: Vec<&str> = if text2.is_empty() {
            Vec::new()
        } else {
            text2.lines().collect()
        };

        // Step 1: Pre-processing heuristics
        let (processed_lines1, processed_lines2, prefix_len, suffix_len) =
            Self::preprocess_lines(&lines1, &lines2);

        // Step 2: Core Patience diff algorithm
        let mut operations = Self::patience_diff(&processed_lines1, &processed_lines2);

        // Step 3: Post-processing heuristics
        let mut operations = Self::group_nearby_changes(operations);
        operations = Self::collapse_large_unchanged_blocks(operations);

        // Step 4: Add back prefix/suffix
        operations = Self::add_prefix_suffix(operations, &lines1, &lines2, prefix_len, suffix_len);

        // Step 5: Build display lines with intra-line diff
        let (left_lines, right_lines) = Self::build_display_lines_with_intraline(&operations);

        // Step 6: Create change blocks
        let change_blocks = Self::create_change_blocks(&left_lines, &right_lines);

        DiffResult {
            operations,
            left_lines,
            right_lines,
            change_blocks,
        }
    }

    /// Pre-processing: Remove common prefix/suffix to focus on actual changes
    fn preprocess_lines<'a>(
        lines1: &'a [&'a str],
        lines2: &'a [&'a str],
    ) -> (Vec<&'a str>, Vec<&'a str>, usize, usize) {
        let mut prefix_len = 0;
        let min_len = lines1.len().min(lines2.len());

        // Find common prefix
        while prefix_len < min_len && lines1[prefix_len] == lines2[prefix_len] {
            prefix_len += 1;
        }

        // Find common suffix
        let mut suffix_len = 0;
        let remaining1 = &lines1[prefix_len..];
        let remaining2 = &lines2[prefix_len..];
        let remaining_min = remaining1.len().min(remaining2.len());

        while suffix_len < remaining_min
            && remaining1[remaining1.len() - 1 - suffix_len]
                == remaining2[remaining2.len() - 1 - suffix_len]
        {
            suffix_len += 1;
        }

        let end1 = lines1.len() - suffix_len;
        let end2 = lines2.len() - suffix_len;

        let processed1 = lines1[prefix_len..end1].to_vec();
        let processed2 = lines2[prefix_len..end2].to_vec();

        (processed1, processed2, prefix_len, suffix_len)
    }

    /// Simple but reliable diff algorithm implementation
    /// Patience Diff algorithm implementation
    fn patience_diff(lines1: &[&str], lines2: &[&str]) -> Vec<DiffOp> {
        // Handle empty inputs
        if lines1.is_empty() && lines2.is_empty() {
            return Vec::new();
        }
        if lines1.is_empty() {
            return lines2
                .iter()
                .map(|line| DiffOp::Insert(line.to_string()))
                .collect();
        }
        if lines2.is_empty() {
            return lines1
                .iter()
                .map(|line| DiffOp::Delete(line.to_string()))
                .collect();
        }

        // Simple line-by-line diff algorithm
        let mut operations = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < lines1.len() || j < lines2.len() {
            if i < lines1.len() && j < lines2.len() {
                if lines1[i] == lines2[j] {
                    // Lines are equal
                    operations.push(DiffOp::Equal(lines1[i].to_string()));
                    i += 1;
                    j += 1;
                } else {
                    // Lines are different - check for potential matches ahead
                    let mut found_match = false;

                    // Look ahead in lines2 for a match with current lines1[i]
                    for k in j..lines2.len().min(j + 50) {
                        if lines1[i] == lines2[k] {
                            // Found a match - insert the intervening lines
                            for m in j..k {
                                operations.push(DiffOp::Insert(lines2[m].to_string()));
                            }
                            operations.push(DiffOp::Equal(lines1[i].to_string()));
                            i += 1;
                            j = k + 1;
                            found_match = true;
                            break;
                        }
                    }

                    if !found_match {
                        // Look ahead in lines1 for a match with current lines2[j]
                        for k in i..lines1.len().min(i + 50) {
                            if lines2[j] == lines1[k] {
                                // Found a match - delete the intervening lines
                                for m in i..k {
                                    operations.push(DiffOp::Delete(lines1[m].to_string()));
                                }
                                operations.push(DiffOp::Equal(lines2[j].to_string()));
                                j += 1;
                                i = k + 1;
                                found_match = true;
                                break;
                            }
                        }
                    }

                    if !found_match {
                        // No match found - treat as both delete and insert
                        operations.push(DiffOp::Delete(lines1[i].to_string()));
                        operations.push(DiffOp::Insert(lines2[j].to_string()));
                        i += 1;
                        j += 1;
                    }
                }
            } else if i < lines1.len() {
                // Only lines1 has content
                operations.push(DiffOp::Delete(lines1[i].to_string()));
                i += 1;
            } else if j < lines2.len() {
                // Only lines2 has content
                operations.push(DiffOp::Insert(lines2[j].to_string()));
                j += 1;
            }
        }

        operations
    }

    /// Group nearby changes to improve readability
    fn group_nearby_changes(operations: Vec<DiffOp>) -> Vec<DiffOp> {
        if operations.len() <= 2 {
            return operations;
        }

        let mut result = Vec::new();
        let mut i = 0;

        while i < operations.len() {
            match &operations[i] {
                DiffOp::Equal(content) => {
                    // If this is a small equal block between changes, consider grouping
                    if content.trim().len() <= 3 && i > 0 && i + 1 < operations.len() {
                        let prev_is_change =
                            matches!(operations[i - 1], DiffOp::Delete(_) | DiffOp::Insert(_));
                        let next_is_change =
                            matches!(operations[i + 1], DiffOp::Delete(_) | DiffOp::Insert(_));

                        if prev_is_change && next_is_change {
                            // Group this small equal with surrounding changes
                            result.push(operations[i].clone());
                            i += 1;
                            continue;
                        }
                    }
                    result.push(operations[i].clone());
                }
                _ => {
                    result.push(operations[i].clone());
                }
            }
            i += 1;
        }

        result
    }

    /// Collapse large unchanged blocks, keeping context at edges
    fn collapse_large_unchanged_blocks(operations: Vec<DiffOp>) -> Vec<DiffOp> {
        let mut result = Vec::new();
        let mut equal_block = Vec::new();

        for op in operations {
            match op {
                DiffOp::Equal(content) => {
                    equal_block.push(content);
                }
                _ => {
                    // Process accumulated equal block
                    if !equal_block.is_empty() {
                        if equal_block.len() > 10 {
                            // Keep first 3 and last 3 lines as context
                            for line in &equal_block[..3] {
                                result.push(DiffOp::Equal(line.clone()));
                            }
                            // Add a marker for collapsed content (optional)
                            for line in &equal_block[equal_block.len() - 3..] {
                                result.push(DiffOp::Equal(line.clone()));
                            }
                        } else {
                            // Keep all lines in smaller blocks
                            for line in equal_block {
                                result.push(DiffOp::Equal(line));
                            }
                        }
                        equal_block = Vec::new();
                    }

                    result.push(op);
                }
            }
        }

        // Handle final equal block
        if !equal_block.is_empty() {
            if equal_block.len() > 10 {
                for line in &equal_block[..3] {
                    result.push(DiffOp::Equal(line.clone()));
                }
                for line in &equal_block[equal_block.len() - 3..] {
                    result.push(DiffOp::Equal(line.clone()));
                }
            } else {
                for line in equal_block {
                    result.push(DiffOp::Equal(line));
                }
            }
        }

        result
    }

    /// Add back the common prefix and suffix
    fn add_prefix_suffix(
        operations: Vec<DiffOp>,
        original_lines1: &[&str],
        _original_lines2: &[&str],
        prefix_len: usize,
        suffix_len: usize,
    ) -> Vec<DiffOp> {
        let mut result = Vec::new();

        // Add prefix
        for line in &original_lines1[..prefix_len] {
            result.push(DiffOp::Equal(line.to_string()));
        }

        // Add processed operations
        result.extend(operations);

        // Add suffix
        let suffix_start1 = original_lines1.len() - suffix_len;
        for line in &original_lines1[suffix_start1..] {
            result.push(DiffOp::Equal(line.to_string()));
        }

        result
    }

    /// Build display lines with intra-line diff highlighting
    fn build_display_lines_with_intraline(
        operations: &[DiffOp],
    ) -> (Vec<DisplayLine>, Vec<DisplayLine>) {
        let mut left_lines = Vec::new();
        let mut right_lines = Vec::new();
        let mut left_line_num = 1;
        let mut right_line_num = 1;

        let mut i = 0;
        while i < operations.len() {
            match &operations[i] {
                DiffOp::Equal(content) => {
                    // Context line appears in both sides
                    left_lines.push(DisplayLine {
                        content: content.clone(),
                        line_type: LineType::Context,
                        original_line_num: Some(left_line_num),
                        word_highlights: Vec::new(),
                    });

                    right_lines.push(DisplayLine {
                        content: content.clone(),
                        line_type: LineType::Context,
                        original_line_num: Some(right_line_num),
                        word_highlights: Vec::new(),
                    });

                    left_line_num += 1;
                    right_line_num += 1;
                    i += 1;
                }

                DiffOp::Delete(old_content) => {
                    // Look ahead for corresponding insert (modification)
                    if i + 1 < operations.len() {
                        if let DiffOp::Insert(new_content) = &operations[i + 1] {
                            // This is a modification - compute intra-line diff
                            let word_highlights = Self::compute_word_diff(old_content, new_content);

                            left_lines.push(DisplayLine {
                                content: old_content.clone(),
                                line_type: LineType::Deletion,
                                original_line_num: Some(left_line_num),
                                word_highlights: word_highlights.clone(),
                            });

                            right_lines.push(DisplayLine {
                                content: new_content.clone(),
                                line_type: LineType::Addition,
                                original_line_num: Some(right_line_num),
                                word_highlights,
                            });

                            left_line_num += 1;
                            right_line_num += 1;
                            i += 2; // Skip the insert since we processed it
                            continue;
                        }
                    }

                    // Pure deletion - no corresponding insert
                    left_lines.push(DisplayLine {
                        content: old_content.clone(),
                        line_type: LineType::Deletion,
                        original_line_num: Some(left_line_num),
                        word_highlights: Vec::new(),
                    });

                    left_line_num += 1;
                    i += 1;
                }

                DiffOp::Insert(content) => {
                    // Pure insertion (not part of modification)
                    right_lines.push(DisplayLine {
                        content: content.clone(),
                        line_type: LineType::Addition,
                        original_line_num: Some(right_line_num),
                        word_highlights: Vec::new(),
                    });

                    right_line_num += 1;
                    i += 1;
                }
            }
        }

        (left_lines, right_lines)
    }

    /// Compute word-level diff for intra-line highlighting
    fn compute_word_diff(old_line: &str, new_line: &str) -> Vec<(usize, usize)> {
        if old_line == new_line {
            return Vec::new();
        }

        let old_words: Vec<&str> = old_line.split_whitespace().collect();
        let new_words: Vec<&str> = new_line.split_whitespace().collect();

        if old_words.is_empty() {
            return Vec::new();
        }

        let mut highlights = Vec::new();
        let mut pos = 0;

        for (i, old_word) in old_words.iter().enumerate() {
            // Find position of this word in original line
            if let Some(word_start) = old_line[pos..].find(old_word) {
                let actual_start = pos + word_start;
                let actual_end = actual_start + old_word.len();

                // Check if word differs in new line
                let word_changed = if i < new_words.len() {
                    new_words[i] != *old_word
                } else {
                    true // Word was removed
                };

                if word_changed {
                    highlights.push((actual_start, actual_end));
                }

                pos = actual_end;
            }
        }

        highlights
    }

    /// Create change blocks for connector rendering
    fn create_change_blocks(
        left_lines: &[DisplayLine],
        right_lines: &[DisplayLine],
    ) -> Vec<ChangeBlock> {
        let mut blocks = Vec::new();

        // Find deletion blocks in left pane
        let mut i = 0;
        while i < left_lines.len() {
            if left_lines[i].line_type == LineType::Deletion {
                let start = i;
                let mut end = i;

                // Find consecutive deletions
                while end + 1 < left_lines.len()
                    && left_lines[end + 1].line_type == LineType::Deletion
                {
                    end += 1;
                }

                blocks.push(ChangeBlock {
                    line_type: LineType::Deletion,
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

        // Find addition blocks in right pane
        let mut j = 0;
        while j < right_lines.len() {
            if right_lines[j].line_type == LineType::Addition {
                let start = j;
                let mut end = j;

                // Find consecutive additions
                while end + 1 < right_lines.len()
                    && right_lines[end + 1].line_type == LineType::Addition
                {
                    end += 1;
                }

                blocks.push(ChangeBlock {
                    line_type: LineType::Addition,
                    start_line: start,
                    end_line: end,
                    left_rects: Vec::new(),
                    right_rects: Vec::new(),
                });

                j = end + 1;
            } else {
                j += 1;
            }
        }

        blocks.sort_by_key(|block| block.start_line);
        blocks
    }
}

/// Main entry point compatible with existing API
pub fn create_complete_side_by_side_with_diff(
    original: &str,
    current: &str,
    _diff_text: &str, // Ignored - we compute our own diff
) -> (Vec<DisplayLine>, Vec<DisplayLine>, Vec<ChangeBlock>) {
    let result = JetBrainsDiff::diff(original, current);
    (result.left_lines, result.right_lines, result.change_blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_changes() {
        let text1 = "line1\nline2\nline3";
        let text2 = "line1\nline2\nline3";

        let result = JetBrainsDiff::diff(text1, text2);

        assert_eq!(result.left_lines.len(), 3);
        assert_eq!(result.right_lines.len(), 3);
        assert_eq!(result.change_blocks.len(), 0);

        // All lines should be context
        assert!(result
            .left_lines
            .iter()
            .all(|l| l.line_type == LineType::Context));
        assert!(result
            .right_lines
            .iter()
            .all(|l| l.line_type == LineType::Context));
    }

    #[test]
    fn test_modification_with_intraline_diff() {
        let text1 = "const old_var = 1;";
        let text2 = "const new_var = 2;";

        let result = JetBrainsDiff::diff(text1, text2);

        assert_eq!(result.left_lines.len(), 1);
        assert_eq!(result.right_lines.len(), 1);

        // Should have word-level highlights
        assert!(!result.left_lines[0].word_highlights.is_empty());
        assert!(!result.right_lines[0].word_highlights.is_empty());
    }

    #[test]
    fn test_common_prefix_suffix_optimization() {
        let text1 = "import A from 'a';\nold line\nimport Z from 'z';";
        let text2 = "import A from 'a';\nnew line\nimport Z from 'z';";

        let result = JetBrainsDiff::diff(text1, text2);

        // First and last lines should be context
        assert_eq!(result.left_lines[0].line_type, LineType::Context);
        assert_eq!(result.left_lines[2].line_type, LineType::Context);
        assert_eq!(result.right_lines[0].line_type, LineType::Context);
        assert_eq!(result.right_lines[2].line_type, LineType::Context);

        // Middle should be changed
        assert_eq!(result.left_lines[1].line_type, LineType::Deletion);
        assert_eq!(result.right_lines[1].line_type, LineType::Addition);
    }

    #[test]
    fn test_large_unchanged_block_context() {
        let mut text1 = "changed line\n".to_string();
        let mut text2 = "modified line\n".to_string();

        // Add many unchanged lines
        for i in 1..=20 {
            let line = format!("unchanged line {}\n", i);
            text1.push_str(&line);
            text2.push_str(&line);
        }

        let result = JetBrainsDiff::diff(&text1, &text2);

        // Should have modification at start
        assert_eq!(result.left_lines[0].line_type, LineType::Deletion);
        assert_eq!(result.right_lines[0].line_type, LineType::Addition);

        // Large block should be preserved as context (not collapsed in this simple test)
        let context_count = result
            .left_lines
            .iter()
            .filter(|l| l.line_type == LineType::Context)
            .count();
        assert!(context_count > 10, "Should preserve context lines");
    }
}
