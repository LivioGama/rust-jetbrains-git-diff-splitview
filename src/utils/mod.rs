// diffsplit/src/utils/mod.rs
// Common utilities module
#![allow(dead_code)]

use std::fs;
use std::path::Path;

/// String utilities
pub mod string_utils {
    /// Truncate a string to a maximum length with ellipsis
    pub fn truncate_with_ellipsis(text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            text.to_string()
        } else if max_len <= 3 {
            "...".chars().take(max_len).collect()
        } else {
            format!("{}...", &text[..max_len - 3])
        }
    }

    /// Split text into lines, handling different line endings
    pub fn split_lines(text: &str) -> Vec<&str> {
        text.split('\n')
            .map(|line| line.trim_end_matches('\r'))
            .collect()
    }

    /// Count the number of lines in text
    pub fn count_lines(text: &str) -> usize {
        text.lines().count()
    }

    /// Find the longest line in a text
    pub fn longest_line(text: &str) -> Option<&str> {
        text.lines().max_by_key(|line| line.len())
    }
}

/// File system utilities
pub mod fs_utils {
    use super::*;

    /// Check if a path exists and is a file
    pub fn is_file(path: &str) -> bool {
        Path::new(path).is_file()
    }

    /// Check if a path exists and is a directory
    pub fn is_directory(path: &str) -> bool {
        Path::new(path).is_dir()
    }

    /// Get file extension from path
    pub fn get_extension(path: &str) -> Option<&str> {
        Path::new(path).extension()?.to_str()
    }

    /// Get filename from path
    pub fn get_filename(path: &str) -> Option<&str> {
        Path::new(path).file_name()?.to_str()
    }

    /// Read file content safely
    pub fn read_file_safe(path: &str) -> Result<String, String> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(e) => Err(format!("Failed to read file '{}': {}", path, e)),
        }
    }

    /// Check if file exists
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }
}

/// Math utilities
pub mod math_utils {
    /// Clamp a value between min and max
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    /// Linear interpolation between two values
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t.clamp(0.0, 1.0)
    }

    /// Map a value from one range to another
    pub fn map_range(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
        if (from_max - from_min).abs() < f32::EPSILON {
            to_min
        } else {
            let normalized = (value - from_min) / (from_max - from_min);
            to_min + normalized * (to_max - to_min)
        }
    }

    /// Calculate distance between two 2D points
    pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        let dx = x2 - x1;
        let dy = y2 - y1;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Time utilities
pub mod time_utils {
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Get current timestamp in milliseconds
    pub fn current_timestamp_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Get current timestamp in seconds
    pub fn current_timestamp_s() -> u64 {
        current_timestamp_ms() / 1000
    }

    /// Format duration in milliseconds to human readable string
    pub fn format_duration_ms(duration_ms: u64) -> String {
        if duration_ms < 1000 {
            format!("{}ms", duration_ms)
        } else if duration_ms < 60000 {
            format!("{:.1}s", duration_ms as f32 / 1000.0)
        } else {
            let minutes = duration_ms / 60000;
            let seconds = (duration_ms % 60000) / 1000;
            format!("{}m {}s", minutes, seconds)
        }
    }
}

/// Collection utilities
pub mod collection_utils {
    /// Find the index of the first element that satisfies the predicate
    pub fn find_index<T, F>(collection: &[T], predicate: F) -> Option<usize>
    where
        F: Fn(&T) -> bool,
    {
        collection.iter().position(predicate)
    }

    /// Check if collection contains element by predicate
    pub fn contains_by<T, F>(collection: &[T], predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        collection.iter().any(predicate)
    }

    /// Get element at index safely
    pub fn get_at<T: Clone>(collection: &[T], index: usize) -> Option<T> {
        collection.get(index).cloned()
    }

    /// Split collection into chunks of specified size
    pub fn chunked<T: Clone>(collection: &[T], chunk_size: usize) -> Vec<Vec<T>> {
        collection
            .chunks(chunk_size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}

/// Validation utilities
pub mod validation_utils {
    /// Validate that a string is not empty
    pub fn is_not_empty(text: &str) -> Result<(), String> {
        if text.trim().is_empty() {
            Err("String cannot be empty".to_string())
        } else {
            Ok(())
        }
    }

    /// Validate that a number is within a range
    pub fn in_range(value: f32, min: f32, max: f32) -> Result<(), String> {
        if value < min {
            Err(format!("Value {} is below minimum {}", value, min))
        } else if value > max {
            Err(format!("Value {} is above maximum {}", value, max))
        } else {
            Ok(())
        }
    }

    /// Validate that a file path exists
    pub fn file_exists(path: &str) -> Result<(), String> {
        if std::path::Path::new(path).exists() {
            Ok(())
        } else {
            Err(format!("File does not exist: {}", path))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_truncate() {
        assert_eq!(string_utils::truncate_with_ellipsis("hello", 10), "hello");
        assert_eq!(
            string_utils::truncate_with_ellipsis("hello world", 8),
            "hello..."
        );
    }

    #[test]
    fn test_math_clamp() {
        assert_eq!(math_utils::clamp(5, 0, 10), 5);
        assert_eq!(math_utils::clamp(-5, 0, 10), 0);
        assert_eq!(math_utils::clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_math_lerp() {
        assert_eq!(math_utils::lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(math_utils::lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(math_utils::lerp(0.0, 10.0, 1.0), 10.0);
    }

    #[test]
    fn test_collection_find_index() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(collection_utils::find_index(&numbers, |&x| x == 3), Some(2));
        assert_eq!(collection_utils::find_index(&numbers, |&x| x == 6), None);
    }

    #[test]
    fn test_validation() {
        assert!(validation_utils::is_not_empty("hello").is_ok());
        assert!(validation_utils::is_not_empty("").is_err());
        assert!(validation_utils::is_not_empty("   ").is_err());
    }
}
