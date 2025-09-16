// Diff generation using the similar crate for proper N:1 and 1:N line mappings

use crate::models::types::{ChangeType, DiffView, Line, LineState, NewChangeBlock, WordDiff};
use similar::{ChangeTag, TextDiff};

pub fn generate_diff_view(old_text: &str, new_text: &str) -> DiffView {
    let mut left_pane = Vec::new();
    let mut right_pane = Vec::new();
    let mut change_blocks = Vec::new();
    let diff = TextDiff::from_lines(old_text, new_text);

    for op in diff.ops() {
        match op.tag() {
            ChangeTag::Equal => {
                for line in diff.slice(op.clone()).iter_slices().next().unwrap().1 {
                    left_pane.push(Line {
                        content: line.to_string(),
                        state: LineState::Unchanged,
                    });
                    right_pane.push(Line {
                        content: line.to_string(),
                        state: LineState::Unchanged,
                    });
                }
            }
            ChangeTag::Delete => {
                let start_idx = left_pane.len();
                let mut len = 0;
                for line in diff.slice(op.clone()).iter_slices().next().unwrap().1 {
                    left_pane.push(Line {
                        content: line.to_string(),
                        state: LineState::Deleted,
                    });
                    right_pane.push(Line {
                        content: "".to_string(),
                        state: LineState::Placeholder,
                    });
                    len += 1;
                }
                change_blocks.push(NewChangeBlock {
                    change_type: ChangeType::Deletion,
                    left_start_idx: start_idx,
                    left_len: len,
                    right_start_idx: start_idx,
                    right_len: len,
                });
            }
            ChangeTag::Insert => {
                let start_idx = left_pane.len();
                let mut len = 0;
                for line in diff.slice(op.clone()).iter_slices().next().unwrap().1 {
                    left_pane.push(Line {
                        content: "".to_string(),
                        state: LineState::Placeholder,
                    });
                    right_pane.push(Line {
                        content: line.to_string(),
                        state: LineState::Added,
                    });
                    len += 1;
                }
                change_blocks.push(NewChangeBlock {
                    change_type: ChangeType::Addition,
                    left_start_idx: start_idx,
                    left_len: len,
                    right_start_idx: start_idx,
                    right_len: len,
                });
            }
            ChangeTag::Replace => {
                let (old_slice, new_slice) = diff.slice(op.clone()).iter_slices().next().unwrap();
                let start_idx = left_pane.len();
                let max_len = old_slice.len().max(new_slice.len());

                for i in 0..max_len {
                    let old_line_opt = old_slice.get(i);
                    let new_line_opt = new_slice.get(i);

                    let word_diffs = if let (Some(old_l), Some(new_l)) = (old_line_opt, new_line_opt)
                    {
                        let word_diff = TextDiff::from_words(old_l, new_l);
                        word_diff
                            .ops()
                            .iter()
                            .flat_map(|op| {
                                word_diff.slice(op.clone()).iter_slices().next().unwrap().1
                                    .iter()
                                    .map(move |word| match op.tag() {
                                        ChangeTag::Equal => WordDiff::Same(word.to_string()),
                                        ChangeTag::Delete => WordDiff::Deleted(word.to_string()),
                                        ChangeTag::Insert => WordDiff::Added(word.to_string()),
                                        ChangeTag::Replace => WordDiff::Added(word.to_string()),
                                    })
                            })
                            .collect()
                    } else {
                        vec![]
                    };

                    match (old_line_opt, new_line_opt) {
                        (Some(old_l), Some(_)) => {
                            left_pane.push(Line {
                                content: old_l.to_string(),
                                state: LineState::Modified {
                                    word_diffs: word_diffs.clone(),
                                },
                            });
                            right_pane.push(Line {
                                content: new_slice[i].to_string(),
                                state: LineState::Modified { word_diffs },
                            });
                        }
                        (Some(old_l), None) => {
                            left_pane.push(Line {
                                content: old_l.to_string(),
                                state: LineState::Deleted,
                            });
                            right_pane.push(Line {
                                content: "".to_string(),
                                state: LineState::Placeholder,
                            });
                        }
                        (None, Some(new_l)) => {
                            left_pane.push(Line {
                                content: "".to_string(),
                                state: LineState::Placeholder,
                            });
                            right_pane.push(Line {
                                content: new_l.to_string(),
                                state: LineState::Added,
                            });
                        }
                        (None, None) => unreachable!(),
                    }
                }
                change_blocks.push(NewChangeBlock {
                    change_type: ChangeType::Modification,
                    left_start_idx: start_idx,
                    left_len: old_slice.len(),
                    right_start_idx: start_idx,
                    right_len: new_slice.len(),
                });
            }
        }
    }

    DiffView {
        left_pane,
        right_pane,
        change_blocks,
    }
}
