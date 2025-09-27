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
    use crate::diff::imara::{compute_imara_diff_default, ImaraBlockOperation};

    // Split content into lines for lookups
    let old_lines: Vec<&str> = original.lines().collect();
    let new_lines: Vec<&str> = current.lines().collect();

    // Compute semantic blocks that describe the relationship between documents
    let imara_analysis = compute_imara_diff_default(original, current);

    // Builders for aligned panes and change navigation metadata
    let mut left_display_lines = Vec::new();
    let mut right_display_lines = Vec::new();
    let mut change_blocks = Vec::new();

    // Track how many source lines we've already materialised
    let mut left_cursor = 0usize;
    let mut right_cursor = 0usize;

    for block in &imara_analysis.blocks {
        if !block.is_change() {
            // Emit matching context before non-change blocks (if any)
            emit_context(
                &mut left_display_lines,
                &mut right_display_lines,
                &old_lines,
                &new_lines,
                &mut left_cursor,
                &mut right_cursor,
                block.left_range.start,
                block.right_range.start,
            );
            continue;
        }

        emit_context(
            &mut left_display_lines,
            &mut right_display_lines,
            &old_lines,
            &new_lines,
            &mut left_cursor,
            &mut right_cursor,
            block.left_range.start,
            block.right_range.start,
        );

        let block_start_index = left_display_lines.len();

        match block.operation {
            ImaraBlockOperation::Insert => {
                for idx in block.right_range.clone() {
                    let right_line =
                        DisplayLine::new(new_lines[idx].to_string(), LineType::Addition)
                            .with_line_number(idx + 1);

                    let left_placeholder = DisplayLine {
                        content: String::new(),
                        line_type: LineType::Addition,
                        original_line_num: None,
                        word_highlights: Vec::new(),
                    };

                    push_aligned_pair(
                        &mut left_display_lines,
                        &mut right_display_lines,
                        left_placeholder,
                        right_line,
                    );
                }

                right_cursor = block.right_range.end;
            }
            ImaraBlockOperation::Delete => {
                for idx in block.left_range.clone() {
                    let left_line =
                        DisplayLine::new(old_lines[idx].to_string(), LineType::Deletion)
                            .with_line_number(idx + 1);

                    let right_placeholder = DisplayLine {
                        content: String::new(),
                        line_type: LineType::Deletion,
                        original_line_num: None,
                        word_highlights: Vec::new(),
                    };

                    push_aligned_pair(
                        &mut left_display_lines,
                        &mut right_display_lines,
                        left_line,
                        right_placeholder,
                    );
                }

                left_cursor = block.left_range.end;
            }
            ImaraBlockOperation::Modify => {
                let left_segment = &old_lines[block.left_range.clone()];
                let right_segment = &new_lines[block.right_range.clone()];
                let max_len = left_segment.len().max(right_segment.len());

                for i in 0..max_len {
                    let left_line = left_segment.get(i).map(|line| {
                        DisplayLine::new(line.to_string(), LineType::Modification)
                            .with_line_number(block.left_range.start + i + 1)
                    });

                    let right_line = right_segment.get(i).map(|line| {
                        DisplayLine::new(line.to_string(), LineType::Modification)
                            .with_line_number(block.right_range.start + i + 1)
                    });

                    match (left_line, right_line) {
                        (Some(mut left_line), Some(mut right_line)) => {
                            let (left_highlights, right_highlights) = compute_word_highlights(
                                &old_lines[block.left_range.start + i],
                                &new_lines[block.right_range.start + i],
                            );
                            left_line.word_highlights = left_highlights;
                            right_line.word_highlights = right_highlights;
                            push_aligned_pair(
                                &mut left_display_lines,
                                &mut right_display_lines,
                                left_line,
                                right_line,
                            );
                        }
                        (Some(left_line), None) => {
                            let right_placeholder = DisplayLine {
                                content: String::new(),
                                line_type: LineType::Deletion,
                                original_line_num: None,
                                word_highlights: Vec::new(),
                            };
                            push_aligned_pair(
                                &mut left_display_lines,
                                &mut right_display_lines,
                                left_line,
                                right_placeholder,
                            );
                        }
                        (None, Some(right_line)) => {
                            let left_placeholder = DisplayLine {
                                content: String::new(),
                                line_type: LineType::Addition,
                                original_line_num: None,
                                word_highlights: Vec::new(),
                            };
                            push_aligned_pair(
                                &mut left_display_lines,
                                &mut right_display_lines,
                                left_placeholder,
                                right_line,
                            );
                        }
                        (None, None) => {}
                    }
                }

                left_cursor = block.left_range.end;
                right_cursor = block.right_range.end;
            }
        }

        let block_len = left_display_lines.len().saturating_sub(block_start_index);
        if block_len > 0 {
            let block_end_index = block_start_index + block_len - 1;
            change_blocks.push(ChangeBlock::new(block_start_index, block_end_index));
        }
    }

    // Flush any trailing context after the last block
    emit_context(
        &mut left_display_lines,
        &mut right_display_lines,
        &old_lines,
        &new_lines,
        &mut left_cursor,
        &mut right_cursor,
        old_lines.len(),
        new_lines.len(),
    );

    debug_assert_eq!(left_display_lines.len(), right_display_lines.len());

    (left_display_lines, right_display_lines, change_blocks)
}

fn push_aligned_pair(
    left_lines: &mut Vec<DisplayLine>,
    right_lines: &mut Vec<DisplayLine>,
    left: DisplayLine,
    right: DisplayLine,
) {
    left_lines.push(left);
    right_lines.push(right);
}

fn emit_context(
    left_lines: &mut Vec<DisplayLine>,
    right_lines: &mut Vec<DisplayLine>,
    old_lines: &[&str],
    new_lines: &[&str],
    left_cursor: &mut usize,
    right_cursor: &mut usize,
    target_left: usize,
    target_right: usize,
) {
    while *left_cursor < target_left || *right_cursor < target_right {
        let left_line = if *left_cursor < target_left {
            let line = DisplayLine::new(old_lines[*left_cursor].to_string(), LineType::Context)
                .with_line_number(*left_cursor + 1);
            *left_cursor += 1;
            line
        } else {
            DisplayLine {
                content: String::new(),
                line_type: LineType::Context,
                original_line_num: None,
                word_highlights: Vec::new(),
            }
        };

        let right_line = if *right_cursor < target_right {
            let line = DisplayLine::new(new_lines[*right_cursor].to_string(), LineType::Context)
                .with_line_number(*right_cursor + 1);
            *right_cursor += 1;
            line
        } else {
            DisplayLine {
                content: String::new(),
                line_type: LineType::Context,
                original_line_num: None,
                word_highlights: Vec::new(),
            }
        };

        push_aligned_pair(left_lines, right_lines, left_line, right_line);
    }
}
