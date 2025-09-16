pub mod diff;
pub mod line;
pub mod ui;

// Re-exports for convenience - avoid glob imports to prevent ambiguity
pub use diff::{AnchorPoint, ChangeBlock, DiffAnalysis, MappingSegment};
pub use line::{DisplayLine, LineType};
pub use ui::{ConnectorConfig, ConnectorCurve, RenderState};
