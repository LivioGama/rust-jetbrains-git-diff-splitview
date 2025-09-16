pub mod parser;

// Re-export main functions and types from parser
pub use parser::create_complete_side_by_side_with_diff;

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
