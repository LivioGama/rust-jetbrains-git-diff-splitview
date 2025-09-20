use imara_diff::{Algorithm, Diff, InternedInput};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct ImaraDiffBlock {
    pub left_range: Range<usize>,
    pub right_range: Range<usize>,
    pub operation: ImaraBlockOperation,
    pub semantic_similarity: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImaraBlockOperation {
    Equal,
    Insert,
    Delete,
    Modify,
}

#[derive(Debug, Clone)]
pub struct ImaraConfig {
    pub algorithm: Algorithm,
}

impl Default for ImaraConfig {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::Histogram,
        }
    }
}

impl ImaraDiffBlock {
    pub fn new(
        left_range: Range<usize>,
        right_range: Range<usize>,
        operation: ImaraBlockOperation,
    ) -> Self {
        Self {
            left_range,
            right_range,
            operation,
            semantic_similarity: None,
        }
    }

    pub fn with_similarity(mut self, similarity: f32) -> Self {
        self.semantic_similarity = Some(similarity);
        self
    }

    pub fn left_line_count(&self) -> usize {
        self.left_range.end.saturating_sub(self.left_range.start)
    }

    pub fn right_line_count(&self) -> usize {
        self.right_range.end.saturating_sub(self.right_range.start)
    }

    pub fn is_change(&self) -> bool {
        !matches!(self.operation, ImaraBlockOperation::Equal)
    }

    pub fn is_pure_insertion(&self) -> bool {
        matches!(self.operation, ImaraBlockOperation::Insert) && self.left_range.is_empty()
    }

    pub fn is_pure_deletion(&self) -> bool {
        matches!(self.operation, ImaraBlockOperation::Delete) && self.right_range.is_empty()
    }

    pub fn is_transformation(&self) -> bool {
        matches!(self.operation, ImaraBlockOperation::Modify)
    }
}

#[derive(Debug, Clone)]
pub struct ImaraDiffAnalysis {
    pub blocks: Vec<ImaraDiffBlock>,
    pub line_mapping: Vec<LineMapping>,
    pub total_old_lines: usize,
    pub total_new_lines: usize,
}

#[derive(Debug, Clone)]
pub struct LineMapping {
    pub old_line: Option<usize>,
    pub new_line: Option<usize>,
    pub operation: LineOperation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineOperation {
    Same,
    Replace,
    Insert,
    Delete,
}

fn calculate_semantic_similarity(old_lines: &[&str], new_lines: &[&str]) -> f32 {
    if old_lines.is_empty() || new_lines.is_empty() {
        return 0.0;
    }

    let old_text = old_lines.join("\n");
    let new_text = new_lines.join("\n");

    if old_text == new_text {
        return 100.0;
    }

    // Simple similarity calculation based on common characters
    let old_chars: std::collections::HashSet<char> = old_text.chars().collect();
    let new_chars: std::collections::HashSet<char> = new_text.chars().collect();

    let intersection = old_chars.intersection(&new_chars).count();
    let union = old_chars.union(&new_chars).count();

    if union == 0 {
        0.0
    } else {
        (intersection as f32 / union as f32) * 100.0
    }
}

pub fn compute_imara_diff(
    old_content: &str,
    new_content: &str,
    config: &ImaraConfig,
) -> ImaraDiffAnalysis {
    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();

    let input = InternedInput::new(old_content, new_content);
    let mut diff = Diff::compute(config.algorithm, &input);
    diff.postprocess_lines(&input);

    let mut blocks = Vec::new();
    let mut line_mapping = Vec::new();

    // Process hunks to build blocks and line mappings
    for hunk in diff.hunks() {
        let old_range = hunk.before.start as usize..hunk.before.end as usize;
        let new_range = hunk.after.start as usize..hunk.after.end as usize;

        let operation = if old_range.is_empty() {
            ImaraBlockOperation::Insert
        } else if new_range.is_empty() {
            ImaraBlockOperation::Delete
        } else {
            ImaraBlockOperation::Modify
        };

        // Calculate semantic similarity for the hunk
        let old_hunk_lines: Vec<&str> = if old_range.is_empty() {
            Vec::new()
        } else {
            old_lines[old_range.clone()].to_vec()
        };

        let new_hunk_lines: Vec<&str> = if new_range.is_empty() {
            Vec::new()
        } else {
            new_lines[new_range.clone()].to_vec()
        };

        let similarity = calculate_semantic_similarity(&old_hunk_lines, &new_hunk_lines);

        let block = ImaraDiffBlock::new(old_range.clone(), new_range.clone(), operation)
            .with_similarity(similarity);

        blocks.push(block);

        // Add line mappings for this hunk
        match (old_range.is_empty(), new_range.is_empty()) {
            (true, false) => {
                // Pure insertion
                for new_line in new_range {
                    line_mapping.push(LineMapping {
                        old_line: None,
                        new_line: Some(new_line),
                        operation: LineOperation::Insert,
                    });
                }
            }
            (false, true) => {
                // Pure deletion
                for old_line in old_range {
                    line_mapping.push(LineMapping {
                        old_line: Some(old_line),
                        new_line: None,
                        operation: LineOperation::Delete,
                    });
                }
            }
            (false, false) => {
                // Modification - map lines one-to-one where possible
                let old_len = old_range.len();
                let new_len = new_range.len();
                let min_len = old_len.min(new_len);

                // Map overlapping lines as replacements
                for i in 0..min_len {
                    line_mapping.push(LineMapping {
                        old_line: Some(old_range.start + i),
                        new_line: Some(new_range.start + i),
                        operation: LineOperation::Replace,
                    });
                }

                // Handle extra old lines as deletions
                for i in min_len..old_len {
                    line_mapping.push(LineMapping {
                        old_line: Some(old_range.start + i),
                        new_line: None,
                        operation: LineOperation::Delete,
                    });
                }

                // Handle extra new lines as insertions
                for i in min_len..new_len {
                    line_mapping.push(LineMapping {
                        old_line: None,
                        new_line: Some(new_range.start + i),
                        operation: LineOperation::Insert,
                    });
                }
            }
            _ => {} // Should not happen
        }
    }

    // Add equal sections to line mapping
    let mut old_idx = 0;
    let mut new_idx = 0;

    // Sort line mappings by old line number to insert equal sections
    line_mapping.sort_by_key(|m| m.old_line.unwrap_or(usize::MAX));

    let mut final_mapping = Vec::new();

    for mapping in &line_mapping {
        // Add equal lines before this mapping
        while let (Some(old_line), Some(new_line)) = (mapping.old_line, mapping.new_line) {
            if old_idx < old_line && new_idx < new_line {
                final_mapping.push(LineMapping {
                    old_line: Some(old_idx),
                    new_line: Some(new_idx),
                    operation: LineOperation::Same,
                });
                old_idx += 1;
                new_idx += 1;
            } else {
                break;
            }
        }

        final_mapping.push(mapping.clone());

        // Update indices
        if let Some(old_line) = mapping.old_line {
            old_idx = old_idx.max(old_line + 1);
        }
        if let Some(new_line) = mapping.new_line {
            new_idx = new_idx.max(new_line + 1);
        }
    }

    // Add remaining equal lines
    while old_idx < old_lines.len() && new_idx < new_lines.len() {
        final_mapping.push(LineMapping {
            old_line: Some(old_idx),
            new_line: Some(new_idx),
            operation: LineOperation::Same,
        });
        old_idx += 1;
        new_idx += 1;
    }

    ImaraDiffAnalysis {
        total_old_lines: old_lines.len(),
        total_new_lines: new_lines.len(),
        blocks,
        line_mapping: final_mapping,
    }
}

pub fn compute_imara_diff_default(old_content: &str, new_content: &str) -> ImaraDiffAnalysis {
    compute_imara_diff(old_content, new_content, &ImaraConfig::default())
}

pub fn print_imara_analysis(analysis: &ImaraDiffAnalysis) {
    println!("🔗 Block Mapping Analysis: Histogram Algorithm");
    println!();
    println!("📊 Hunk Overview");

    let total_hunks = analysis.blocks.len();
    println!(
        "Total hunks: {} | Old: {} lines | New: {} lines",
        total_hunks, analysis.total_old_lines, analysis.total_new_lines
    );
    println!();

    let mut hunk_counter = 1;
    for block in &analysis.blocks {
        println!("═══ HUNK {} CONNECTION ═══", hunk_counter);
        println!(
            "Mapping: Lines {}-{} → Lines {}-{}",
            block.left_range.start + 1,
            block.left_range.end,
            block.right_range.start + 1,
            block.right_range.end
        );

        let block_type = if block.is_pure_insertion() {
            "PURE INSERTION (no old block)"
        } else if block.is_pure_deletion() {
            "PURE DELETION (no new block)"
        } else {
            "TRANSFORMATION (old → new)"
        };
        println!("Type: {}", block_type);
        println!();

        if !block.is_pure_insertion() {
            println!(
                "🔴 OLD BLOCK (Lines {}-{}):",
                block.left_range.start + 1,
                block.left_range.end
            );
            println!(
                "    (old content lines {}-{})",
                block.left_range.start + 1,
                block.left_range.end
            );
            println!();
        }

        if !block.is_pure_deletion() {
            println!(
                "🟢 NEW BLOCK (Lines {}-{}):",
                block.right_range.start + 1,
                block.right_range.end
            );
            println!(
                "    (new content lines {}-{})",
                block.right_range.start + 1,
                block.right_range.end
            );
            println!();
        }

        println!("🧠 Connection Analysis:");
        if let Some(similarity) = block.semantic_similarity {
            println!("  📊 Semantic similarity: {:.1}%", similarity);
            let connection_strength = if similarity >= 70.0 {
                "STRONG CONNECTION"
            } else if similarity >= 30.0 {
                "MODERATE CONNECTION"
            } else {
                "WEAK CONNECTION"
            };
            println!("    └─  {}", connection_strength);
        }
        println!();

        hunk_counter += 1;
    }

    println!();
}

// Compatibility functions for existing codebase
pub fn convert_to_legacy_blocks(
    analysis: &ImaraDiffAnalysis,
) -> Vec<crate::models::diff::ChangeBlock> {
    analysis
        .blocks
        .iter()
        .filter(|block| block.is_change())
        .flat_map(|block| {
            let mut change_blocks = Vec::new();

            // Create change blocks for left side (deletions)
            if !block.left_range.is_empty() {
                change_blocks.push(crate::models::diff::ChangeBlock::new(
                    block.left_range.start,
                    block
                        .left_range
                        .end
                        .saturating_sub(1)
                        .max(block.left_range.start),
                ));
            }

            // Create change blocks for right side (additions)
            if !block.right_range.is_empty() {
                change_blocks.push(crate::models::diff::ChangeBlock::new(
                    block.right_range.start,
                    block
                        .right_range
                        .end
                        .saturating_sub(1)
                        .max(block.right_range.start),
                ));
            }

            change_blocks
        })
        .collect()
}
