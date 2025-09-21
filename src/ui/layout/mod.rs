// diffsplit/src/ui/layout/mod.rs
// UI Layout module for organizing the main application interface

pub mod gutter;
pub mod layout_manager;
pub mod panes;

pub use layout_manager::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LayoutConfig;

    #[test]
    fn test_layout_manager_creation() {
        let _manager = LayoutManager::new(LayoutConfig::default());
        // Test passes if manager is created successfully
    }

    #[test]
    fn test_layout_manager_default() {
        let _manager = LayoutManager::default();
        // Test passes if manager is created successfully
    }
}
