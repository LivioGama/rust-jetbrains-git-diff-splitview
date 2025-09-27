// Layout manager for diff viewer - GPUI Implementation with mesh-based connector rendering
use crate::config::LayoutConfig;
use crate::diff::imara::{ImaraBlockOperation, ImaraDiffAnalysis};
use crate::models::diff::MappingSegment;
use crate::models::line::DisplayLine;
use crate::sync::ScrollSync;
use crate::theme::JetBrainsTheme;
use gpui::*;

use super::connectors::{ConnectorState, ConnectorUtils};
use crate::ui::ConnectorRenderer;

/// Layout manager for the diff viewer
pub struct LayoutManager {
    config: LayoutConfig,
    pane_renderer: super::panes::PaneRenderer,
}

impl LayoutManager {
    pub fn new(config: LayoutConfig) -> Self {
        Self {
            pane_renderer: super::panes::PaneRenderer::new(config.clone()),
            config,
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(LayoutConfig::default())
    }

    /// Render the main layout using GPUI with two panes and connectors
    pub fn render_layout(
        &self,
        old_lines: &[DisplayLine],
        new_lines: &[DisplayLine],
        scroll_sync: &mut crate::sync::ScrollSync,
        theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        mapping_segments: &[MappingSegment],
        imara_analysis: &crate::diff::imara::ImaraDiffAnalysis,
        viewport_height: f32,
        left_scroll: f32,
        right_scroll: f32,
    ) -> impl IntoElement {
        let connector_width = px(self.config.connector_column_width);

        // Create connector state for rectangle storage and management
        let mut connector_state = ConnectorState::new();
        connector_state.update_viewport(viewport_height, left_scroll, right_scroll);
        eprintln!(
            "[DEBUG] render_layout viewport_height={:.1}, left_scroll={:.1}, right_scroll={:.1}",
            viewport_height, left_scroll, right_scroll
        );

        // Render left pane and store its rectangles
        let left_pane = self.pane_renderer.render_pane_gpui(
            "Original",
            old_lines,
            scroll_sync,
            theme,
            line_renderer,
            px(0.0),
            mapping_segments,
            imara_analysis,
            true,
            left_scroll,
            right_scroll,
            &mut connector_state,
        );

        // Render right pane and store its rectangles
        let right_pane = self.pane_renderer.render_pane_gpui(
            "Modified",
            new_lines,
            scroll_sync,
            theme,
            line_renderer,
            px(0.0),
            mapping_segments,
            imara_analysis,
            false,
            left_scroll,
            right_scroll,
            &mut connector_state,
        );

        // Create connector renderer with stored rectangle data
        let connector_gutter =
            self.create_connector_gutter(&connector_state, theme, imara_analysis, connector_width);

        div()
            .h_full()
            .w_full()
            .flex()
            .flex_row()
            .relative()
            .children(vec![
                // Left pane (original) - 50% width
                div()
                    .flex_1()
                    .h_full()
                    .min_w_0()
                    .min_h_0()
                    .child(left_pane)
                    .into_any_element(),
                // Middle gutter with connectors - fixed width
                connector_gutter,
                // Right pane (modified) - 50% width
                div()
                    .flex_1()
                    .h_full()
                    .min_w_0()
                    .min_h_0()
                    .child(right_pane)
                    .into_any_element(),
            ])
    }

    /// Create connector gutter with proper state-based rendering
    fn create_connector_gutter(
        &self,
        connector_state: &ConnectorState,
        theme: &JetBrainsTheme,
        imara_analysis: &ImaraDiffAnalysis,
        connector_width: Pixels,
    ) -> gpui::AnyElement {
        // Only render connectors if we have valid rectangle data
        if !connector_state.has_valid_rects() {
            return div()
                .flex_none()
                .w(connector_width)
                .h_full()
                .min_h(px(400.0))
                .relative()
                .overflow_hidden()
                .bg(theme.connector_column)
                .into_any_element();
        }

        // Create connector elements using stored rectangle data
        let connector_elements =
            self.generate_connector_elements(connector_state, theme, imara_analysis);

        let header_spacer_height = 31.0;

        div()
            .flex_none()
            .w(connector_width)
            .h_full()
            .min_h(px(400.0))
            .flex()
            .flex_col()
            .relative()
            .overflow_hidden()
            .bg(theme.connector_column)
            .children(vec![
                div()
                    .flex_none()
                    .w_full()
                    .h(px(header_spacer_height))
                    .bg(theme.connector_column)
                    .into_any_element(),
                div()
                    .flex_1()
                    .min_h_0()
                    .relative()
                    .overflow_hidden()
                    .children(connector_elements)
                    .into_any_element(),
            ])
            .into_any_element()
    }

    /// Generate connector elements from stored rectangle data
    fn generate_connector_elements(
        &self,
        connector_state: &ConnectorState,
        theme: &JetBrainsTheme,
        imara_analysis: &ImaraDiffAnalysis,
    ) -> Vec<gpui::AnyElement> {
        eprintln!(
            "[DEBUG] generate_connector_elements called with {} blocks",
            imara_analysis.blocks.len()
        );
        eprintln!(
            "[DEBUG] connector_state has valid rects: {}",
            connector_state.has_valid_rects()
        );
        eprintln!(
            "[DEBUG] left rects: {}, right rects: {}",
            connector_state.left_rects.len(),
            connector_state.right_rects.len()
        );

        let mut elements = Vec::new();
        let connector_width = self.config.connector_column_width as f32;
        let left_x_end = 0.0;
        let right_x_start = connector_width;

        eprintln!(
            "[DEBUG] connector_width: {}, left_x_end: {}, right_x_start: {}",
            connector_width, left_x_end, right_x_start
        );

        // Process each imara diff block
        for (block_idx, block) in imara_analysis.blocks.iter().enumerate() {
            eprintln!(
                "[DEBUG] Processing block {}: left_range={:?}, right_range={:?}",
                block_idx, block.left_range, block.right_range
            );

            if block.left_range.is_empty() && block.right_range.is_empty() {
                eprintln!("[DEBUG] Block {} skipped - both ranges empty", block_idx);
                continue;
            }

            let color = match &block.operation {
                ImaraBlockOperation::Insert => theme.addition_background.opacity(0.9),
                ImaraBlockOperation::Delete => theme.deletion_background.opacity(0.9),
                ImaraBlockOperation::Modify => theme.modification_background.opacity(0.9),
            };

            eprintln!("[DEBUG] Block {} color: {:?}", block_idx, color);

            // Handle different types of connectors
            if block.is_pure_insertion() {
                eprintln!("[DEBUG] Block {} is pure insertion", block_idx);
            } else {
                eprintln!(
                    "[DEBUG] Block {} is regular - creating full-width connector",
                    block_idx
                );
                // Create full-width connector for regular blocks
                if let Some(connector_element) = self.create_full_width_connector(
                    connector_state,
                    block,
                    left_x_end,
                    right_x_start,
                    color,
                ) {
                    eprintln!(
                        "[DEBUG] Block {} connector element created successfully",
                        block_idx
                    );
                    elements.push(connector_element);
                } else {
                    eprintln!(
                        "[DEBUG] Block {} connector element creation failed",
                        block_idx
                    );
                }
            }
        }

        eprintln!(
            "[DEBUG] generate_connector_elements returning {} elements (left_scroll_offset={:.1}, right_scroll_offset={:.1})",
            elements.len(),
            connector_state.left_scroll_offset,
            connector_state.right_scroll_offset
        );

        elements
    }

    /// Create a full-width connector element that spans the entire gutter
    fn create_full_width_connector(
        &self,
        connector_state: &ConnectorState,
        block: &crate::diff::imara::ImaraDiffBlock,
        left_x: f32,
        right_x: f32,
        color: gpui::Hsla,
    ) -> Option<gpui::AnyElement> {
        let default_bounds = gpui::Bounds::default();
        let left_start_rect = connector_state
            .get_left_rect(block.left_range.start)
            .unwrap_or(&default_bounds);
        let left_end_rect = connector_state
            .get_left_rect(block.left_range.end.saturating_sub(1))
            .unwrap_or(left_start_rect);
        let right_start_rect = connector_state
            .get_right_rect(block.right_range.start)
            .unwrap_or(&default_bounds);
        let right_end_rect = connector_state
            .get_right_rect(block.right_range.end.saturating_sub(1))
            .unwrap_or(right_start_rect);

        let (mut left_y_start, mut left_y_end, mut right_y_start, mut right_y_end) =
            ConnectorUtils::calculate_connector_coords(
                left_start_rect,
                left_end_rect,
                right_start_rect,
                right_end_rect,
                connector_state.left_scroll_offset,
                connector_state.right_scroll_offset,
            );

        eprintln!(
            "[DEBUG] Rect bounds -> left_start: {:?}, left_end: {:?}, right_start: {:?}, right_end: {:?}",
            left_start_rect,
            left_end_rect,
            right_start_rect,
            right_end_rect
        );

        let left_block_height = left_y_end - left_y_start;
        let right_block_height = right_y_end - right_y_start;

        eprintln!(
            "[DEBUG] Block heights -> left={:.1}, right={:.1}, delta={:.1}",
            left_block_height,
            right_block_height,
            (right_block_height - left_block_height).abs()
        );

        eprintln!(
            "[DEBUG] Raw connector coords: left=({:.1}, {:.1}), right=({:.1}, {:.1}), scroll_offsets=({:.1}, {:.1})",
            left_y_start,
            left_y_end,
            right_y_start,
            right_y_end,
            connector_state.left_scroll_offset,
            connector_state.right_scroll_offset
        );

        let adjusted_left_y_end = left_y_end + 1.0;
        let adjusted_right_y_end = right_y_end + 1.0;

        eprintln!(
            "[DEBUG] Final connector coords: left=({:.1}, {:.1}), right=({:.1}, {:.1})",
            left_y_start, adjusted_left_y_end, right_y_start, adjusted_right_y_end
        );

        Some(self.create_dual_curve_connector(
            left_x,
            left_y_start,
            adjusted_left_y_end,
            right_x,
            right_y_start,
            adjusted_right_y_end,
            color,
        ))
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
        let segments = 32;
        let control_point_offset = (right_x - left_x) * 0.35;

        // Use original coordinates without scaling

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
