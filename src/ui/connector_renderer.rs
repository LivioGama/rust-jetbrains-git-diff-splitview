// Connector rendering logic for diff viewer - GPUI Native Implementation
use gpui::Hsla;
use gpui::*;

use crate::diff::imara::ImaraDiffAnalysis;
use crate::models::line::DisplayLine;

use crate::models::ui::{ConnectorCurve, ConnectorOperation};
use crate::sync::ScrollSync;
use crate::theme::JetBrainsTheme;

// Add logging for debugging connector coordinates
macro_rules! log_connector_coords {
    ($msg:expr, $($arg:tt)*) => {
        eprintln!("[CONNECTOR] {}", format_args!($msg, $($arg)*));
    };
}

#[derive(Clone)]
pub struct ConnectorRenderer {
    theme: JetBrainsTheme,
}

impl ConnectorRenderer {
    pub fn new(theme: JetBrainsTheme) -> Self {
        Self { theme }
    }

    /// Generate div elements for a Bézier curve approximated by many thin dots along the path
    fn bezier_curve_to_divs(
        start: (f32, f32),
        control1: (f32, f32),
        control2: (f32, f32),
        end: (f32, f32),
        color: gpui::Hsla,
        segments: usize,
    ) -> Vec<gpui::AnyElement> {
        let mut elements = Vec::new();
        let thickness = 2.0; // Thin dots

        for i in 0..=segments {
            let t = i as f32 / segments as f32;

            // Cubic Bézier formula
            let t2 = t * t;
            let t3 = t2 * t;
            let mt = 1.0 - t;
            let mt2 = mt * mt;
            let mt3 = mt2 * mt;

            let x = mt3 * start.0
                + 3.0 * mt2 * t * control1.0
                + 3.0 * mt * t2 * control2.0
                + t3 * end.0;
            let y = mt3 * start.1
                + 3.0 * mt2 * t * control1.1
                + 3.0 * mt * t2 * control2.1
                + t3 * end.1;

            // Create a small dot at each point along the curve
            elements.push(
                div()
                    .absolute()
                    .left(px(x - thickness / 2.0))
                    .top(px(y - thickness / 2.0))
                    .w(px(thickness))
                    .h(px(thickness))
                    .bg(color)
                    .into_any_element(),
            );
        }

        elements
    }

    /// Calculate control points for an S-shaped connector curve
    fn calculate_s_curve_control_points(
        left_start_y: f32,
        left_end_y: f32,
        right_start_y: f32,
        right_end_y: f32,
        connector_width: f32,
    ) -> ((f32, f32), (f32, f32), (f32, f32), (f32, f32)) {
        let left_center_y = (left_start_y + left_end_y) / 2.0;
        let right_center_y = (right_start_y + right_end_y) / 2.0;

        let start_point = (0.0, left_center_y);
        let end_point = (connector_width, right_center_y);

        // Create a proper S-curve like JetBrains IDEA:
        // The curve should go out to the right, then curve back toward the center, then out to the right again
        let curve_amplitude = (left_end_y - left_start_y)
            .abs()
            .max((right_end_y - right_start_y).abs())
            * 0.3;

        // For S-curve: first control point goes outward, second comes back inward
        let control1_x = connector_width * 0.35;
        let control2_x = connector_width * 0.65;

        // Calculate Y positions to create the S-shape
        let control1_y = if left_center_y < right_center_y {
            // Going downward: first control point goes up (outward), second goes down (inward)
            left_center_y - curve_amplitude * 0.5
        } else if left_center_y > right_center_y {
            // Going upward: first control point goes down (outward), second goes up (inward)
            left_center_y + curve_amplitude * 0.5
        } else {
            // Horizontal: no curve
            left_center_y
        };

        let control2_y = if left_center_y < right_center_y {
            right_center_y + curve_amplitude * 0.5
        } else if left_center_y > right_center_y {
            right_center_y - curve_amplitude * 0.5
        } else {
            right_center_y
        };

        (
            start_point,
            (control1_x, control1_y),
            (control2_x, control2_y),
            end_point,
        )
    }

    /// Render connectors using GPUI elements with proper S-curves and scroll sync
    pub fn render_connectors_gpui(
        &self,
        old_lines: &[DisplayLine],
        new_lines: &[DisplayLine],
        header_offset: Pixels,
        scroll_sync: &mut ScrollSync,
        imara_analysis: &ImaraDiffAnalysis,
        left_scroll: f32,
        right_scroll: f32,
        viewport_height: Option<f32>,
        connector_width: f32,
    ) -> impl IntoElement {
        log_connector_coords!(
            "render_connectors_gpui called with viewport_height: {:?}",
            viewport_height
        );
        let line_height = self.theme.line_height();
        let vertical_padding = 4.0;
        let line_stride = line_height + vertical_padding;
        let header_offset_px = header_offset.0;

        // Create S-shaped connectors between block corners like JetBrains diff viewer
        let s_connectors: Vec<_> = imara_analysis
            .blocks
            .iter()
            .filter_map(|block| {
                // Always try to create connectors if there's any content on both sides
                if block.left_range.is_empty() && block.right_range.is_empty() {
                    return None;
                }

                log_connector_coords!("Processing block: op={:?}, left_range={:?}, right_range={:?}",
                    block.operation, block.left_range, block.right_range);

                let color = match &block.operation {
                    crate::diff::imara::ImaraBlockOperation::Insert => self.theme.addition_background.opacity(1.0),
                    crate::diff::imara::ImaraBlockOperation::Delete => self.theme.deletion_background.opacity(1.0),
                    crate::diff::imara::ImaraBlockOperation::Modify => self.theme.modification_background.opacity(1.0),
                };

                // Calculate Y positions for the block
                // These are document-relative positions, then adjusted for scroll to get viewport positions
                let left_start_y_doc = if block.left_range.is_empty() {
                    // For insertions, use the right side position
                    (block.right_range.start as f32) * line_stride + header_offset_px
                } else {
                    (block.left_range.start as f32) * line_stride + header_offset_px
                };

                let left_end_y_doc = if block.left_range.is_empty() {
                    left_start_y_doc
                        + (block.right_range.end - block.right_range.start) as f32 * line_stride
                } else {
                    (block.left_range.end as f32) * line_stride + header_offset_px
                };

                let right_start_y_doc = if block.right_range.is_empty() {
                    // For deletions, use the left side position
                    (block.left_range.start as f32) * line_stride + header_offset_px
                } else {
                    (block.right_range.start as f32) * line_stride + header_offset_px
                };

                let right_end_y_doc = if block.right_range.is_empty() {
                    right_start_y_doc
                        + (block.left_range.end - block.left_range.start) as f32 * line_stride
                } else {
                    (block.right_range.end as f32) * line_stride + header_offset_px
                };

                // Convert document positions to viewport positions by subtracting scroll offsets
                // For connectors to stay connected during scrolling, use the scroll offset of the pane
                // that contains the block. For blocks on both sides, average the positions.
                let (left_start_y, left_end_y, right_start_y, right_end_y) = if block.left_range.is_empty() {
                    // Pure insertion: follow right pane scroll
                    (
                        left_start_y_doc - right_scroll,
                        left_end_y_doc - right_scroll,
                        right_start_y_doc - right_scroll,
                        right_end_y_doc - right_scroll,
                    )
                } else if block.right_range.is_empty() {
                    // Pure deletion: follow left pane scroll
                    (
                        left_start_y_doc - left_scroll,
                        left_end_y_doc - left_scroll,
                        right_start_y_doc - left_scroll,
                        right_end_y_doc - left_scroll,
                    )
                } else {
                    // Modification on both sides: average the scroll positions for smooth connection
                    let avg_scroll = (left_scroll + right_scroll) / 2.0;
                    (
                        left_start_y_doc - avg_scroll,
                        left_end_y_doc - avg_scroll,
                        right_start_y_doc - avg_scroll,
                        right_end_y_doc - avg_scroll,
                    )
                };

                log_connector_coords!("Calculated positions: left_y=({:.1}, {:.1}), right_y=({:.1}, {:.1}), left_scroll={:.1}, right_scroll={:.1}",
                    left_start_y_doc, left_end_y_doc, right_start_y_doc, right_end_y_doc, left_scroll, right_scroll);

                // Include block operation in the data passed to canvas
                // Store both document and viewport coordinates
                Some((
                    left_start_y_doc, left_end_y_doc, right_start_y_doc, right_end_y_doc, // document coords
                    left_start_y, left_end_y, right_start_y, right_end_y, // viewport coords
                    color, block.operation.clone()
                ))
            })
            .collect();

        log_connector_coords!("Total connectors to render: {}", s_connectors.len());

        // Render S-shaped connectors using canvas for precise curves
        let viewport_height = match viewport_height {
            Some(h) if h > 0.0 => {
                log_connector_coords!("Using passed viewport height: {:.1}", h);
                h
            }
            _ => {
                log_connector_coords!(
                    "Using fallback viewport height: 600.0 (passed: {:?})",
                    viewport_height
                );
                600.0
            }
        };

        // Pre-calculate curve data to avoid lifetime issues
        let curve_data: Vec<_> = s_connectors
            .iter()
            .map(
                |&(
                    _,
                    _,
                    _,
                    _,
                    left_start_y,
                    left_end_y,
                    right_start_y,
                    right_end_y,
                    color,
                    ref operation,
                )| {
                    let (start, control1, control2, end) = Self::calculate_s_curve_control_points(
                        left_start_y,
                        left_end_y,
                        right_start_y,
                        right_end_y,
                        connector_width,
                    );
                    let thickness = match operation {
                        crate::diff::imara::ImaraBlockOperation::Insert => 8.0,
                        crate::diff::imara::ImaraBlockOperation::Delete => 8.0,
                        crate::diff::imara::ImaraBlockOperation::Modify => 7.0,
                    };
                    (start, control1, control2, end, color, thickness)
                },
            )
            .collect();

        // Create background elements
        let mut elements = Vec::new();
        for &(_, _, _, _, left_start_y, left_end_y, right_start_y, right_end_y, color, _) in
            &s_connectors
        {
            elements.push(self.create_dual_curve_connector(
                0.0,
                left_start_y,
                left_end_y,
                connector_width,
                right_start_y,
                right_end_y,
                color,
            ));
        }

        // Create curve elements with 50 segments for smoother approximation
        for &(start, control1, control2, end, color, _) in &curve_data {
            let mut curve_elements =
                Self::bezier_curve_to_divs(start, control1, control2, end, color, 50);
            elements.append(&mut curve_elements);
        }

        // Calculate document height based on maximum lines for proper connector positioning
        let max_lines = old_lines.len().max(new_lines.len());
        let document_height = max_lines as f32 * line_stride + header_offset_px;

        div()
            .w(px(connector_width))
            .h(px(document_height))
            .bg(self.theme.connector_column)
            .relative()
            .children(elements)
            .into_any_element()
    }

    /// Create a dual-curve ribbon connector that fills the area between top and bottom curves
    fn create_dual_curve_connector(
        &self,
        left_x: f32,
        left_y_start: f32,
        left_y_end: f32,
        right_x: f32,
        right_y_start: f32,
        right_y_end: f32,
        color: gpui::Hsla,
    ) -> gpui::AnyElement {
        let segments = 50;
        let control_point_offset = (right_x - left_x) * 0.35;

        // Generate top curve points
        let mut top_points = Vec::new();
        let mut bottom_points = Vec::new();

        for i in 0..=segments {
            let t = i as f32 / segments as f32;

            // Top curve (left_start to right_start)
            let top_point = self.cubic_bezier(
                (left_x, left_y_start),
                (left_x + control_point_offset, left_y_start),
                (right_x - control_point_offset, right_y_start),
                (right_x, right_y_start),
                t,
            );
            top_points.push(top_point);

            // Bottom curve (left_end to right_end)
            let bottom_point = self.cubic_bezier(
                (left_x, left_y_end),
                (left_x + control_point_offset, left_y_end),
                (right_x - control_point_offset, right_y_end),
                (right_x, right_y_end),
                t,
            );
            bottom_points.push(bottom_point);
        }

        // Create ribbon by filling between top and bottom curves
        let mut elements = Vec::new();

        for i in 0..segments {
            let top_left = top_points[i];
            let top_right = top_points[i + 1];
            let bottom_left = bottom_points[i];
            let bottom_right = bottom_points[i + 1];

            // Calculate the rectangular strip between curves
            let min_x = top_left
                .0
                .min(top_right.0)
                .min(bottom_left.0)
                .min(bottom_right.0);
            let max_x = top_left
                .0
                .max(top_right.0)
                .max(bottom_left.0)
                .max(bottom_right.0);
            let min_y = top_left
                .1
                .min(top_right.1)
                .min(bottom_left.1)
                .min(bottom_right.1);
            let max_y = top_left
                .1
                .max(top_right.1)
                .max(bottom_left.1)
                .max(bottom_right.1);

            elements.push(
                div()
                    .absolute()
                    .left(px(min_x))
                    .top(px(min_y))
                    .w(px(max_x - min_x))
                    .h(px(max_y - min_y))
                    .bg(color)
                    .into_any_element(),
            );
        }

        // Find overall bounds for container
        let all_points = [&top_points[..], &bottom_points[..]].concat();
        let min_x = all_points
            .iter()
            .map(|(x, _)| *x)
            .fold(f32::INFINITY, f32::min);
        let max_x = all_points
            .iter()
            .map(|(x, _)| *x)
            .fold(f32::NEG_INFINITY, f32::max);
        let min_y = all_points
            .iter()
            .map(|(_, y)| *y)
            .fold(f32::INFINITY, f32::min);
        let max_y = all_points
            .iter()
            .map(|(_, y)| *y)
            .fold(f32::NEG_INFINITY, f32::max);

        div()
            .absolute()
            .left(px(min_x))
            .top(px(min_y))
            .w(px(max_x - min_x))
            .h(px(max_y - min_y))
            .children(elements)
            .into_any_element()
    }

    /// Cubic Bezier curve calculation helper
    fn cubic_bezier(
        &self,
        p0: (f32, f32),
        p1: (f32, f32),
        p2: (f32, f32),
        p3: (f32, f32),
        t: f32,
    ) -> (f32, f32) {
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        let t2 = t * t;
        let t3 = t2 * t;

        let x = mt3 * p0.0 + 3.0 * mt2 * t * p1.0 + 3.0 * mt * t2 * p2.0 + t3 * p3.0;
        let y = mt3 * p0.1 + 3.0 * mt2 * t * p1.1 + 3.0 * mt * t2 * p2.1 + t3 * p3.1;

        (x, y)
    }
}
