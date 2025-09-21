// Imara-diff based implementation for semantic diff analysis
use crate::models::diff::ChangeBlock;
use crate::models::line::{DisplayLine, HighlightType, LineType};
use dissimilar;

fn compute_word_highlights(
    old_line: &str,
    new_line: &str,
) -> (
    Vec<(usize, usize, HighlightType)>,
    Vec<(usize, usize, HighlightType)>,
) {
    let chunks = dissimilar::diff(old_line, new_line);
    let mut left_highlights = Vec::new();
    let mut right_highlights = Vec::new();
    let mut left_pos = 0;
    let mut right_pos = 0;

    for chunk in chunks {
        match chunk {
            dissimilar::Chunk::Equal(s) => {
                let char_count = s.chars().count();
                left_pos += char_count;
                right_pos += char_count;
            }
            dissimilar::Chunk::Delete(s) => {
                let start = left_pos;
                let char_count = s.chars().count();
                left_pos += char_count;
                left_highlights.push((start, left_pos, HighlightType::Delete));
            }
            dissimilar::Chunk::Insert(s) => {
                let start = right_pos;
                let char_count = s.chars().count();
                right_pos += char_count;
                right_highlights.push((start, right_pos, HighlightType::Insert));
            }
        }
    }
    (left_highlights, right_highlights)
}

/// Main entry point using imara-diff semantic analysis with histogram algorithm
pub fn create_complete_side_by_side_with_diff(
    original: &str,
    current: &str,
    _diff_text: &str, // Ignored - we compute our own diff with imara
) -> (Vec<DisplayLine>, Vec<DisplayLine>, Vec<ChangeBlock>) {
    use crate::diff::imara::compute_imara_diff_default;

    // Split content into lines
    let old_lines: Vec<&str> = original.lines().collect();
    let new_lines: Vec<&str> = current.lines().collect();

    // Compute imara diff analysis using histogram algorithm
    let imara_analysis = compute_imara_diff_default(original, current);

    // Create display lines with default context type
    let mut left_display_lines: Vec<DisplayLine> = old_lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            DisplayLine::new(line.to_string(), LineType::Context).with_line_number(i + 1)
        })
        .collect();

    let mut right_display_lines: Vec<DisplayLine> = new_lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            DisplayLine::new(line.to_string(), LineType::Context).with_line_number(i + 1)
        })
        .collect();

    // Apply imara diff analysis to mark changed lines
    for imara_block in &imara_analysis.blocks {
        if !imara_block.is_change() {
            continue;
        }

        // Handle different block operations
        match imara_block.operation {
            crate::diff::imara::ImaraBlockOperation::Modify => {
                // For Modify blocks, use Modification type to get blue background
                if !imara_block.left_range.is_empty() {
                    for line_idx in imara_block.left_range.clone() {
                        if line_idx < left_display_lines.len() {
                            left_display_lines[line_idx].line_type = LineType::Modification;
                        }
                    }
                }

                if !imara_block.right_range.is_empty() {
                    for line_idx in imara_block.right_range.clone() {
                        if line_idx < right_display_lines.len() {
                            right_display_lines[line_idx].line_type = LineType::Modification;
                        }
                    }
                }

                // Add word-level highlights for modified lines
                for i in 0..imara_block
                    .left_range
                    .len()
                    .min(imara_block.right_range.len())
                {
                    let left_idx = imara_block.left_range.start + i;
                    let right_idx = imara_block.right_range.start + i;
                    if left_idx < left_display_lines.len() && right_idx < right_display_lines.len()
                    {
                        let old_line = &old_lines[left_idx];
                        let new_line = &new_lines[right_idx];
                        let (left_highlights, right_highlights) =
                            compute_word_highlights(old_line, new_line);
                        left_display_lines[left_idx].word_highlights = left_highlights;
                        right_display_lines[right_idx].word_highlights = right_highlights;
                    }
                }
            }
            crate::diff::imara::ImaraBlockOperation::Insert => {
                // Pure insertion: mark right side as Addition
                if !imara_block.right_range.is_empty() {
                    for line_idx in imara_block.right_range.clone() {
                        if line_idx < right_display_lines.len() {
                            right_display_lines[line_idx].line_type = LineType::Addition;
                        }
                    }
                }
            }
            crate::diff::imara::ImaraBlockOperation::Delete => {
                // Pure deletion: mark left side as Deletion
                if !imara_block.left_range.is_empty() {
                    for line_idx in imara_block.left_range.clone() {
                        if line_idx < left_display_lines.len() {
                            left_display_lines[line_idx].line_type = LineType::Deletion;
                        }
                    }
                }
            }
        }
    }

    // Create change blocks from imara-diff semantic blocks
    let mut change_blocks = Vec::new();
    for imara_block in &imara_analysis.blocks {
        if !imara_block.is_change() {
            continue;
        }

        // Create change blocks for left side (deletions)
        if !imara_block.left_range.is_empty() {
            change_blocks.push(ChangeBlock::new(
                imara_block.left_range.start,
                imara_block
                    .left_range
                    .end
                    .saturating_sub(1)
                    .max(imara_block.left_range.start),
            ));
        }

        // Create change blocks for right side (additions)
        if !imara_block.right_range.is_empty() {
            change_blocks.push(ChangeBlock::new(
                imara_block.right_range.start,
                imara_block
                    .right_range
                    .end
                    .saturating_sub(1)
                    .max(imara_block.right_range.start),
            ));
        }
    }

    (left_display_lines, right_display_lines, change_blocks)
}
