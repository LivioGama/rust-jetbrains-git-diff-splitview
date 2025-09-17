pub mod diff;
pub mod line;
pub mod ui;

// Re-exports for convenience - avoid glob imports to prevent ambiguity
pub use diff::{AnchorPoint, ChangeBlock, MappingSegment};
pub use line::{DisplayLine, LineType};
pub use ui::ConnectorCurve;
