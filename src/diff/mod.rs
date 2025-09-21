pub mod imara;
pub mod parser;

// Re-export main functions and types from parser (now imara-based)
pub use parser::create_complete_side_by_side_with_diff;
// Re-export main functions and types from imara
pub use imara::compute_imara_diff_default;
