// diffsplit/src/ui/layout/mod.rs
// UI Layout module for organizing the main application interface

use eframe::egui;
use egui::{FontId, Pos2, ScrollArea, Vec2};

use crate::config::LayoutConfig;
use crate::models::diff::MappingSegment;
use crate::models::line::DisplayLine;

use crate::sync::*;

/// Layout manager for the diff viewer
pub struct LayoutManager {
    config: LayoutConfig,
}

impl LayoutManager {
    pub fn new(config: LayoutConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(LayoutConfig::default())
    }

    /// Render the complete application layout
    pub fn render_layout(
        &self,
        ui: &mut egui::Ui,
        old_lines: &[DisplayLine],
        new_lines: &[DisplayLine],
        scroll_sync: &mut crate::sync::ScrollSync,
        theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        connector_renderer: &mut crate::ui::ConnectorRenderer,
        mapping_segments: &[MappingSegment],
    ) {
        let total_height = ui.available_height();
        let total_width = ui.available_width();
        let pane_width = (total_width - self.config.connector_column_width) / 2.0;

        // Create a horizontal layout with explicit height allocation and no spacing
        ui.allocate_ui_with_layout(
            Vec2::new(total_width, total_height),
            egui::Layout::left_to_right(egui::Align::TOP),
            |ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                // Left pane (original)
                self.render_left_pane(
                    ui,
                    old_lines,
                    scroll_sync,
                    theme,
                    line_renderer,
                    pane_width,
                    total_height,
                    mapping_segments,
                );

                // Middle gutter background (connectors will be drawn later)
                let gutter_rect = ui
                    .allocate_response(
                        egui::Vec2::new(self.config.connector_column_width, total_height),
                        egui::Sense::hover(),
                    )
                    .rect;
                ui.painter()
                    .rect_filled(gutter_rect, 0.0, theme.connector_column);

                // Right pane (modified)
                self.render_right_pane(
                    ui,
                    new_lines,
                    scroll_sync,
                    theme,
                    line_renderer,
                    pane_width,
                    total_height,
                    mapping_segments,
                );
            },
        );

        // Now render connectors after both panes are rendered
        self.render_connectors(ui, old_lines, new_lines, pane_width);
    }

    /// Render connectors between panes
    fn render_connectors(
        &self,
        ui: &mut egui::Ui,
        old_lines: &[DisplayLine],
        new_lines: &[DisplayLine],
        pane_width: f32,
    ) {
        // Get stored rectangle positions
        let left_rects: Option<Vec<egui::Rect>> = ui
            .ctx()
            .memory_mut(|mem| mem.data.get_persisted("left_rects".into()));
        let right_rects: Option<Vec<egui::Rect>> = ui
            .ctx()
            .memory_mut(|mem| mem.data.get_persisted("right_rects".into()));

        if let (Some(left_rects), Some(right_rects)) = (left_rects, right_rects) {
            // Calculate gutter position
            let gutter_x_start = pane_width;
            let gutter_x_end = gutter_x_start + self.config.connector_column_width;

            // Find change hunks (contiguous blocks of any changes) on both sides
            let mut left_hunks = Vec::new();
            let mut current_left: Option<(usize, usize)> = None;

            for (i, line) in old_lines.iter().enumerate() {
                if line.line_type != crate::models::line::LineType::Context {
                    match current_left.as_mut() {
                        Some((_, ref mut end)) => {
                            *end = i;
                        }
                        None => {
                            current_left = Some((i, i));
                        }
                    }
                } else if let Some(hunk) = current_left.take() {
                    left_hunks.push(hunk);
                }
            }
            if let Some(hunk) = current_left.take() {
                left_hunks.push(hunk);
            }

            let mut right_hunks = Vec::new();
            let mut current_right: Option<(usize, usize)> = None;

            for (i, line) in new_lines.iter().enumerate() {
                if line.line_type != crate::models::line::LineType::Context {
                    match current_right.as_mut() {
                        Some((_, ref mut end)) => {
                            *end = i;
                        }
                        None => {
                            current_right = Some((i, i));
                        }
                    }
                } else if let Some(hunk) = current_right.take() {
                    right_hunks.push(hunk);
                }
            }
            if let Some(hunk) = current_right.take() {
                right_hunks.push(hunk);
            }

            // Pair up corresponding hunks
            let hunk_pairs = std::cmp::min(left_hunks.len(), right_hunks.len());
            let mut connectors = Vec::new();

            for i in 0..hunk_pairs {
                let (left_start, left_end) = left_hunks[i];
                let (right_start, right_end) = right_hunks[i];
                connectors.push((left_start, left_end, right_start, right_end));
            }

            // Draw connectors for each hunk pair
            for (left_start, left_end, right_start, right_end) in connectors {
                if let (
                    Some(left_start_rect),
                    Some(left_end_rect),
                    Some(right_start_rect),
                    Some(right_end_rect),
                ) = (
                    left_rects.get(left_start),
                    left_rects.get(left_end),
                    right_rects.get(right_start),
                    right_rects.get(right_end),
                ) {
                    // Use actual rendered positions - align with line rectangles
                    let left_y_start = left_start_rect.top();
                    let left_y_end = left_end_rect.bottom();
                    let right_y_start = right_start_rect.top();
                    let right_y_end = right_end_rect.bottom();

                    // Connector coordinates - extend into panes for seamless connection
                    let x1 = gutter_x_start - 1.0; // Extend into left pane
                    let x2 = gutter_x_end + 1.0; // Extend into right pane

                    // Use theme addition color (green with transparency)
                    let color = egui::Color32::from_rgba_unmultiplied(76, 175, 80, 100);

                    // Draw the S-shaped connector linking the two hunks
                    self.draw_connector(
                        ui,
                        x1,
                        left_y_start,
                        left_y_end,
                        x2,
                        right_y_start,
                        right_y_end,
                        color,
                    );
                }
            }
        }
    }

    /// Draw a single connector using the rendering connector
    fn draw_connector(
        &self,
        ui: &mut egui::Ui,
        x1: f32,
        y1_start: f32,
        y1_end: f32,
        x2: f32,
        y2_start: f32,
        y2_end: f32,
        color: egui::Color32,
    ) {
        use egui::{epaint::PathShape, Pos2, Shape};

        let cp1_x = x1 + (x2 - x1) * 0.4;
        let cp2_x = x2 - (x2 - x1) * 0.4;

        // Adjust control points for better alignment with line blocks
        let y_diff_top = y2_start - y1_start;
        let cp1_y = y1_start + y_diff_top * 0.15;
        let cp2_y = y2_start - y_diff_top * 0.15;

        let y_diff_bottom = y2_end - y1_end;
        let cp1_y_bottom = y1_end + y_diff_bottom * 0.15;
        let cp2_y_bottom = y2_end - y_diff_bottom * 0.15;

        // Create the path by interpolating Bezier curves
        let mut points = Vec::new();

        // Top curve
        let top_curve_points = self.cubic_bezier_points(
            Pos2::new(x1, y1_start),
            Pos2::new(cp1_x, cp1_y),
            Pos2::new(cp2_x, cp2_y),
            Pos2::new(x2, y2_start),
            20,
        );
        points.extend(top_curve_points);

        // Right side
        points.push(Pos2::new(x2, y2_end));

        // Bottom curve
        let bottom_curve_points = self.cubic_bezier_points(
            Pos2::new(x2, y2_end),
            Pos2::new(cp2_x, cp2_y_bottom),
            Pos2::new(cp1_x, cp1_y_bottom),
            Pos2::new(x1, y1_end),
            20,
        );
        points.extend(bottom_curve_points);

        // Close the path
        points.push(Pos2::new(x1, y1_start));

        // Use mesh approach to completely eliminate any border artifacts
        let mut mesh = egui::epaint::Mesh::default();

        // Make color slightly transparent for smooth blending
        let fill_color = {
            let [r, g, b, _a] = color.to_array();
            egui::Color32::from_rgba_unmultiplied(r, g, b, 180)
        };

        // Create center point for triangulation
        let center =
            points.iter().fold(Pos2::ZERO, |acc, p| acc + p.to_vec2()) / points.len() as f32;

        // Add center vertex
        mesh.vertices.push(egui::epaint::Vertex {
            pos: center,
            uv: egui::epaint::WHITE_UV,
            color: fill_color,
        });

        // Add edge vertices
        for point in &points {
            mesh.vertices.push(egui::epaint::Vertex {
                pos: *point,
                uv: egui::epaint::WHITE_UV,
                color: fill_color,
            });
        }

        // Create triangles from center to edges
        for i in 0..points.len() {
            let next_i = (i + 1) % points.len();
            mesh.indices
                .extend_from_slice(&[0, (i + 1) as u32, (next_i + 1) as u32]);
        }

        // Draw the mesh
        ui.painter()
            .add(egui::Shape::Mesh(std::sync::Arc::new(mesh)));
    }

    /// Generate points for a cubic Bezier curve
    fn cubic_bezier_points(
        &self,
        p0: Pos2,
        p1: Pos2,
        p2: Pos2,
        p3: Pos2,
        segments: usize,
    ) -> Vec<Pos2> {
        let mut points = Vec::new();
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let x = (1.0 - t).powi(3) * p0.x
                + 3.0 * (1.0 - t).powi(2) * t * p1.x
                + 3.0 * (1.0 - t) * t.powi(2) * p2.x
                + t.powi(3) * p3.x;
            let y = (1.0 - t).powi(3) * p0.y
                + 3.0 * (1.0 - t).powi(2) * t * p1.y
                + 3.0 * (1.0 - t) * t.powi(2) * p2.y
                + t.powi(3) * p3.y;
            points.push(Pos2::new(x, y));
        }
        points
    }

    /// Render the left pane (original file)
    fn render_left_pane(
        &self,
        ui: &mut egui::Ui,
        old_lines: &[DisplayLine],
        scroll_sync: &mut crate::sync::ScrollSync,
        theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        pane_width: f32,
        total_height: f32,
        mapping_segments: &[MappingSegment],
    ) {
        ui.allocate_ui_with_layout(
            Vec2::new(pane_width, total_height),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                ui.style_mut().spacing.indent = 0.0;
                // Header
                self.render_pane_header(ui, "Original", theme);

                ui.separator();

                // Content area with scrolling
                self.render_scrollable_content(
                    ui,
                    old_lines,
                    scroll_sync,
                    theme,
                    line_renderer,
                    true,
                    "diff_left_scroll",
                    "left_scroll",
                    "left_rects",
                    mapping_segments,
                );
            },
        );
    }

    /// Render the right pane (modified file)
    fn render_right_pane(
        &self,
        ui: &mut egui::Ui,
        new_lines: &[DisplayLine],
        scroll_sync: &mut crate::sync::ScrollSync,
        theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        pane_width: f32,
        total_height: f32,
        mapping_segments: &[MappingSegment],
    ) {
        ui.allocate_ui_with_layout(
            Vec2::new(pane_width, total_height),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                ui.style_mut().spacing.indent = 0.0;
                // Header
                self.render_pane_header(ui, "Modified", theme);

                ui.separator();

                // Content area with scrolling
                self.render_scrollable_content(
                    ui,
                    new_lines,
                    scroll_sync,
                    theme,
                    line_renderer,
                    false,
                    "diff_right_scroll",
                    "right_scroll",
                    "right_rects",
                    mapping_segments,
                );
            },
        );
    }

    /// Render the connector gutter
    fn render_connector_gutter(
        &self,
        ui: &mut egui::Ui,
        old_lines: &[DisplayLine],
        new_lines: &[DisplayLine],
        connector_renderer: &mut crate::ui::ConnectorRenderer,
        theme: &crate::theme::JetBrainsTheme,
        total_height: f32,
        pane_width: f32,
        scroll_sync: &crate::sync::ScrollSync,
    ) {
        // Create a frame with proper background color
        let frame = egui::Frame::new()
            .fill(theme.connector_column)
            .inner_margin(egui::Margin::ZERO)
            .outer_margin(egui::Margin::ZERO);

        frame.show(ui, |ui| {
            ui.allocate_ui_with_layout(
                Vec2::new(self.config.connector_column_width, total_height),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    // Ensure the full height is used and background is filled
                    let full_rect = ui
                        .allocate_response(
                            Vec2::new(self.config.connector_column_width, total_height),
                            egui::Sense::hover(),
                        )
                        .rect;

                    // Fill the entire connector area with theme background
                    ui.painter()
                        .rect_filled(full_rect, 0.0, theme.connector_column);

                    // Find change hunks (contiguous blocks of any changes) on both sides
                    // This matches JetBrains behavior where connectors link corresponding hunks

                    // Find change hunks on left side
                    let mut left_hunks = Vec::new();
                    let mut current_left: Option<(usize, usize)> = None;

                    for (i, line) in old_lines.iter().enumerate() {
                        if line.line_type != crate::models::line::LineType::Context {
                            match current_left.as_mut() {
                                Some((_, ref mut end)) => {
                                    *end = i;
                                }
                                None => {
                                    current_left = Some((i, i));
                                }
                            }
                        } else if let Some(hunk) = current_left.take() {
                            left_hunks.push(hunk);
                        }
                    }
                    if let Some(hunk) = current_left.take() {
                        left_hunks.push(hunk);
                    }

                    // Find change hunks on right side
                    let mut right_hunks = Vec::new();
                    let mut current_right: Option<(usize, usize)> = None;

                    for (i, line) in new_lines.iter().enumerate() {
                        if line.line_type != crate::models::line::LineType::Context {
                            match current_right.as_mut() {
                                Some((_, ref mut end)) => {
                                    *end = i;
                                }
                                None => {
                                    current_right = Some((i, i));
                                }
                            }
                        } else if let Some(hunk) = current_right.take() {
                            right_hunks.push(hunk);
                        }
                    }
                    if let Some(hunk) = current_right.take() {
                        right_hunks.push(hunk);
                    }

                    // Pair up corresponding hunks - this is the key to JetBrains behavior
                    let hunk_pairs = std::cmp::min(left_hunks.len(), right_hunks.len());
                    let mut connectors = Vec::new();

                    for i in 0..hunk_pairs {
                        let (left_start, left_end) = left_hunks[i];
                        let (right_start, right_end) = right_hunks[i];
                        connectors.push((left_start, left_end, right_start, right_end));
                    }

                    // Create connector renderer and draw connectors between paired hunks
                    let connector_renderer =
                        crate::rendering::ConnectorRenderer::new(theme.clone());
                    let line_height = 18.0;
                    // Account for header height (header + separator)
                    let header_height = theme.font_size * 1.1 + 20.0;

                    // Get actual rendered rectangle positions from memory
                    let left_rects: Option<Vec<egui::Rect>> = ui
                        .ctx()
                        .memory_mut(|mem| mem.data.get_persisted("left_rects".into()));
                    let right_rects: Option<Vec<egui::Rect>> = ui
                        .ctx()
                        .memory_mut(|mem| mem.data.get_persisted("right_rects".into()));

                    // Only draw connectors if we have actual rendered positions
                    if let (Some(left_rects), Some(right_rects)) = (left_rects, right_rects) {
                        // Draw connectors for each hunk pair using actual rendered positions
                        for (left_start, left_end, right_start, right_end) in connectors {
                            // Get actual rendered positions from stored rectangles
                            if let (
                                Some(left_start_rect),
                                Some(left_end_rect),
                                Some(right_start_rect),
                                Some(right_end_rect),
                            ) = (
                                left_rects.get(left_start),
                                left_rects.get(left_end),
                                right_rects.get(right_start),
                                right_rects.get(right_end),
                            ) {
                                // Use actual rendered positions - align with line rectangles
                                let y1_start = left_start_rect.top();
                                let y1_end = left_end_rect.bottom();
                                let y2_start = right_start_rect.top();
                                let y2_end = right_end_rect.bottom();

                                // Only draw connector if at least part of it is visible in the gutter area
                                let connector_top = y1_start.min(y2_start);
                                let connector_bottom = y1_end.max(y2_end);
                                let visible_area_top = full_rect.top() + header_height;
                                let visible_area_bottom = full_rect.bottom();

                                // Check if connector overlaps with visible area
                                if connector_bottom >= visible_area_top
                                    && connector_top <= visible_area_bottom
                                {
                                    // Connector coordinates - extend into panes for seamless connection
                                    let x1 = full_rect.left() - 1.0; // Extend into left pane
                                    let x2 = full_rect.right() + 1.0; // Extend into right pane

                                    // Use theme addition color (green with transparency)
                                    let color =
                                        egui::Color32::from_rgba_unmultiplied(76, 175, 80, 100);

                                    // Draw the S-shaped connector linking the two hunks
                                    self.draw_connector(
                                        ui, x1, y1_start, y1_end, x2, y2_start, y2_end, color,
                                    );
                                }
                            }
                        }
                    }
                },
            );
        });
    }

    /// Render a pane header
    fn render_pane_header(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        theme: &crate::theme::JetBrainsTheme,
    ) {
        ui.horizontal(|ui| {
            ui.add_space(self.config.pane_padding);
            ui.label(
                egui::RichText::new(title)
                    .font(FontId::new(
                        theme.font_size * 1.1,
                        egui::FontFamily::Proportional,
                    ))
                    .color(theme.foreground),
            );
        });
    }

    /// Render scrollable content area
    fn render_scrollable_content(
        &self,
        ui: &mut egui::Ui,
        lines: &[DisplayLine],
        scroll_sync: &mut crate::sync::ScrollSync,
        theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        is_left: bool,
        scroll_id: &str,
        scroll_memory_key: &str,
        rects_memory_key: &str,
        mapping_segments: &[MappingSegment],
    ) {
        let available_height = ui.available_height();

        // Get current scroll position from memory
        let current_scroll = ui.ctx().memory_mut(|mem| {
            mem.data
                .get_persisted(scroll_memory_key.to_string().into())
                .unwrap_or(0.0)
        });

        // Calculate synchronized position
        let scroll_offset = if (is_left && scroll_sync.master_pane() == MasterPane::Right)
            || (!is_left && scroll_sync.master_pane() == MasterPane::Left)
        {
            if is_left {
                scroll_sync.left_scroll_offset()
            } else {
                scroll_sync.right_scroll_offset()
            }
        } else {
            current_scroll
        };

        let scroll_output = ScrollArea::vertical()
            .id_source(scroll_id)
            .auto_shrink([false, false])
            .max_height(available_height)
            .min_scrolled_height(available_height)
            .scroll_offset(Vec2::new(0.0, scroll_offset))
            .show(ui, |ui| {
                let mut line_rects = Vec::new();

                for (line_idx, line) in lines.iter().enumerate() {
                    let rect = line_renderer.render_line(ui, line, line_idx, is_left);
                    line_rects.push(rect);
                }

                ui.ctx().memory_mut(|mem| {
                    mem.data
                        .insert_persisted(rects_memory_key.to_string().into(), line_rects);
                });
            });

        // Update scroll sync
        if is_left {
            scroll_sync.set_left_scroll(scroll_output.state.offset.y);
            // Synchronize right pane
            scroll_sync
                .synchronize_scrolls(|y| crate::sync::map_left_to_right(y, mapping_segments));
        } else {
            scroll_sync.set_right_scroll(scroll_output.state.offset.y);
            // Synchronize left pane
            scroll_sync
                .synchronize_scrolls(|y| crate::sync::map_right_to_left(y, mapping_segments));
        }

        // Store scroll position
        ui.ctx().memory_mut(|mem| {
            mem.data.insert_persisted(
                scroll_memory_key.to_string().into(),
                scroll_output.state.offset.y,
            );
        });
    }

    /// Update layout configuration
    pub fn update_config(&mut self, config: LayoutConfig) {
        self.config = config;
    }

    /// Get current layout configuration
    pub fn get_config(&self) -> &LayoutConfig {
        &self.config
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::with_default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LayoutConfig;

    #[test]
    fn test_layout_manager_creation() {
        let manager = LayoutManager::new(LayoutConfig::default());
        assert_eq!(manager.get_config().connector_column_width, 45.0);
    }

    #[test]
    fn test_layout_manager_default() {
        let manager = LayoutManager::default();
        assert_eq!(manager.get_config().pane_padding, 10.0);
    }
}
