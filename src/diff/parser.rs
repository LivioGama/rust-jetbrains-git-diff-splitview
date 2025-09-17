// Imara-diff based implementation for semantic diff analysis
use crate::models::diff::ChangeBlock;
use crate::models::line::{DisplayLine, LineType};

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

        // Mark left side lines (deletions)
        if !imara_block.left_range.is_empty() {
            for line_idx in imara_block.left_range.clone() {
                if line_idx < left_display_lines.len() {
                    left_display_lines[line_idx].line_type = LineType::Deletion;
                }
            }
        }

        // Mark right side lines (additions)
        if !imara_block.right_range.is_empty() {
            for line_idx in imara_block.right_range.clone() {
                if line_idx < right_display_lines.len() {
                    right_display_lines[line_idx].line_type = LineType::Addition;
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
