// UI-related data structures and types - GPUI Implementation

use gpui::Hsla;

/// Represents a visual connector curve between diff blocks in left and right panes
#[derive(Debug, Clone)]
pub struct ConnectorCurve {
    /// Starting point on the left pane (x, y)
    pub start_point: (f32, f32),
    /// Ending point on the right pane (x, y)
    pub end_point: (f32, f32),
    /// First control point for S-curve (x, y)
    pub control_point1: (f32, f32),
    /// Second control point for S-curve (x, y)
    pub control_point2: (f32, f32),
    /// Color of the connector based on operation type
    pub color: Hsla,
    /// Operation type this connector represents
    pub operation: ConnectorOperation,
    /// Thickness of the connector line
    pub thickness: f32,
}

/// The type of operation this connector represents
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectorOperation {
    Insert,
    Delete,
    Modify,
}

impl ConnectorCurve {
    /// Create a new connector curve
    pub fn new(
        start_point: (f32, f32),
        end_point: (f32, f32),
        control_point1: (f32, f32),
        control_point2: (f32, f32),
        color: Hsla,
        operation: ConnectorOperation,
        thickness: f32,
    ) -> Self {
        Self {
            start_point,
            end_point,
            control_point1,
            control_point2,
            color,
            operation,
            thickness,
        }
    }

    /// Create a connector curve between two blocks that spans the full block height
    pub fn create_s_curve(
        left_start_y: f32,
        left_end_y: f32,
        right_start_y: f32,
        right_end_y: f32,
        connector_x_start: f32,
        connector_x_end: f32,
        color: Hsla,
        operation: ConnectorOperation,
        thickness: f32,
    ) -> Self {
        let start_point = (connector_x_start, (left_start_y + left_end_y) / 2.0);
        let end_point = (connector_x_end, (right_start_y + right_end_y) / 2.0);

        // Control points curve through the top and bottom edges for a full block connection
        Self::new(
            (connector_x_start, left_start_y),
            (connector_x_end, right_end_y),
            (connector_x_start, left_start_y),
            (connector_x_end, right_end_y),
            color,
            operation,
            thickness,
        )
    }
}
