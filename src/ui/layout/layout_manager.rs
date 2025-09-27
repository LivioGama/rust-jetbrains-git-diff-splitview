// src/ui/layout/layout_manager.rs
// Main layout manager extracted from layout/mod.rs

use super::{gutter::GutterRenderer, panes::PaneRenderer};
use crate::config::LayoutConfig;
use crate::models::diff::MappingSegment;
use crate::models::line::DisplayLine;
use crate::ui::ConnectorRenderer;
use eframe::egui;
use egui::{Pos2, Rect, Vec2};

/// Layout manager for the diff viewer
pub struct LayoutManager {
    config: LayoutConfig,
    connector_renderer: ConnectorRenderer,
    pane_renderer: PaneRenderer,
    gutter_renderer: GutterRenderer,
}

impl LayoutManager {
    pub fn new(config: LayoutConfig) -> Self {
        let theme = crate::theme::JetBrainsTheme::dark_theme();
        Self {
            connector_renderer: ConnectorRenderer::new(theme),
            pane_renderer: PaneRenderer::new(config.clone()),
            gutter_renderer: GutterRenderer::new(config.clone()),
            config,
        }
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
        _connector_renderer: &mut crate::ui::ConnectorRenderer,
        mapping_segments: &[MappingSegment],
        imara_analysis: &crate::diff::imara::ImaraDiffAnalysis,
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
                self.pane_renderer.render_left_pane(
                    ui,
                    old_lines,
                    scroll_sync,
                    theme,
                    line_renderer,
                    pane_width,
                    total_height,
                    mapping_segments,
                    imara_analysis,
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
                self.pane_renderer.render_right_pane(
                    ui,
                    new_lines,
                    scroll_sync,
                    theme,
                    line_renderer,
                    pane_width,
                    total_height,
                    mapping_segments,
                    imara_analysis,
                );
            },
        );

        // Now render connectors after both panes are rendered - use original working method
        self.render_connectors(
            ui,
            &old_lines,
            &new_lines,
            pane_width,
            scroll_sync,
            imara_analysis,
        );
    }

    /// Render connectors between panes (restored from original working version)
    fn render_connectors(
        &self,
        ui: &mut egui::Ui,
        _old_lines: &[DisplayLine],
        _new_lines: &[DisplayLine],
        pane_width: f32,
        _scroll_sync: &crate::sync::ScrollSync,
        imara_analysis: &crate::diff::imara::ImaraDiffAnalysis,
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
            let _gutter_x_end = gutter_x_start + self.config.connector_column_width;

            // Use imara-diff semantic blocks for connector mapping
            let mut connectors = Vec::new();

            for imara_block in &imara_analysis.blocks {
                if !imara_block.is_change() {
                    continue;
                }

                // Use the semantic ranges from imara-diff for connector mapping
                let left_start = imara_block.left_range.start;
                let left_end = imara_block.left_range.end.saturating_sub(1);
                let right_start = imara_block.right_range.start;
                let right_end = imara_block.right_range.end.saturating_sub(1);

                // Only create connectors for blocks that have both left and right ranges
                if !imara_block.left_range.is_empty() && !imara_block.right_range.is_empty() {
                    connectors.push((
                        left_start,
                        left_end,
                        right_start,
                        right_end,
                        &imara_block.operation,
                    ));
                }
            }

            // Get stored crushed line positions
            let left_crushed_rects: Option<Vec<(usize, egui::Rect, String)>> =
                ui.ctx().memory_mut(|mem| {
                    mem.data
                        .get_persisted("left_crushed_rects".to_string().into())
                });
            let right_crushed_rects: Option<Vec<(usize, egui::Rect, String)>> =
                ui.ctx().memory_mut(|mem| {
                    mem.data
                        .get_persisted("right_crushed_rects".to_string().into())
                });

            // Handle pure insertion blocks - connect to actual crushed lines in left pane
            if let Some(left_crushed) = &left_crushed_rects {
                for imara_block in &imara_analysis.blocks {
                    if imara_block.is_pure_insertion() && !imara_block.right_range.is_empty() {
                        let right_start = imara_block.right_range.start;
                        let right_end = imara_block.right_range.end.saturating_sub(1);

                        // Find corresponding crushed line
                        if let Some((_, crushed_rect, _)) =
                            left_crushed.iter().find(|(idx, _, _)| *idx == right_start)
                        {
                            if let (Some(right_start_rect), Some(right_end_rect)) =
                                (right_rects.get(right_start), right_rects.get(right_end))
                            {
                                // Connect from the actual crushed line position to the right block
                                let crushed_line_y = crushed_rect.min.y;
                                let right_top_y = right_start_rect.top();
                                let right_bottom_y = right_end_rect.bottom();

                                // Use the same palette color as addition blocks
                                let base = theme.addition_background;
                                let addition_color =
                                    egui::Color32::from_rgba_unmultiplied(base.r(), base.g(), base.b(), base.a());

                                // Draw connector from the crushed line to the actual right block
                                self.draw_connector(
                                    ui,
                                    gutter_x_start,
                                    self.config.connector_column_width,
                                    crushed_line_y,
                                    crushed_line_y + 2.0,
                                    right_top_y,
                                    right_bottom_y,
                                    addition_color,
                                );
                            }
                        }
                    }
                }
            }

            // Handle pure deletion blocks - connect to actual crushed lines in right pane
            if let Some(right_crushed) = &right_crushed_rects {
                for imara_block in &imara_analysis.blocks {
                    if imara_block.is_pure_deletion() && !imara_block.left_range.is_empty() {
                        let left_start = imara_block.left_range.start;
                        let left_end = imara_block.left_range.end.saturating_sub(1);

                        // Find corresponding crushed line
                        if let Some((_, crushed_rect, _)) =
                            right_crushed.iter().find(|(idx, _, _)| *idx == left_start)
                        {
                            if let (Some(left_start_rect), Some(left_end_rect)) =
                                (left_rects.get(left_start), left_rects.get(left_end))
                            {
                                // Connect from the left block to the actual crushed line position
                                let left_top_y = left_start_rect.top();
                                let left_bottom_y = left_end_rect.bottom();
                                let crushed_line_y = crushed_rect.min.y;

                                let base = theme.deletion_background;
                                let deletion_color =
                                    egui::Color32::from_rgba_unmultiplied(base.r(), base.g(), base.b(), base.a());

                                // Draw connector from the left block to the crushed line
                                self.draw_connector(
                                    ui,
                                    gutter_x_start,
                                    self.config.connector_column_width,
                                    left_top_y,
                                    left_bottom_y,
                                    crushed_line_y,
                                    crushed_line_y + 2.0,
                                    deletion_color,
                                );
                            }
                        }
                    }
                }
            }

            // Draw connectors for each hunk pair
            for (left_start, left_end, right_start, right_end, operation) in connectors {
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

                    // Determine color based on Imara block operation (prioritize over line types)
                    let color = match operation {
                        crate::diff::imara::ImaraBlockOperation::Modify => {
                            let base = theme.modification_background;
                            egui::Color32::from_rgba_unmultiplied(
                                base.r(),
                                base.g(),
                                base.b(),
                                base.a(),
                            )
                        }
                        crate::diff::imara::ImaraBlockOperation::Insert => {
                            let base = theme.addition_background;
                            egui::Color32::from_rgba_unmultiplied(
                                base.r(),
                                base.g(),
                                base.b(),
                                base.a(),
                            )
                        }
                        crate::diff::imara::ImaraBlockOperation::Delete => {
                            let base = theme.deletion_background;
                            egui::Color32::from_rgba_unmultiplied(
                                base.r(),
                                base.g(),
                                base.b(),
                                base.a(),
                            )
                        }
                    };

                    // Draw connector between the two blocks using the middle gutter span
                    self.draw_connector(
                        ui,
                        gutter_x_start,
                        self.config.connector_column_width,
                        left_y_start,
                        left_y_end,
                        right_y_start,
                        right_y_end,
                        color,
                    );
                }
            }
        }
    }

    /// Draw connector using an S-shaped ribbon confined to the connector gutter
    fn draw_connector(
        &self,
        ui: &mut egui::Ui,
        gutter_x_start: f32,
        gutter_width: f32,
        y1_start: f32,
        y1_end: f32,
        y2_start: f32,
        y2_end: f32,
        color: egui::Color32,
    ) {
        use egui::{epaint::Mesh, epaint::Vertex, Pos2};

        let segments = 32; // Use high resolution for a perfectly smooth curve.
        let mut top_points = Vec::with_capacity(segments + 1);
        let mut bottom_points = Vec::with_capacity(segments + 1);

        let x1 = gutter_x_start;
        let x2 = gutter_x_start + gutter_width;

        // Match JetBrains curvature by using 30% of the gutter width as control offset
        let control_point_offset = gutter_width * 0.30;

        // 1. Generate the points for the top and bottom curves.
        for i in 0..=segments {
            let t = i as f32 / segments as f32;

            // Top curve (left to right)
            let top_start = Pos2::new(x1, y1_start);
            let top_end = Pos2::new(x2, y2_start);
            let top_ctrl1 = Pos2::new(top_start.x + control_point_offset, top_start.y);
            let top_ctrl2 = Pos2::new(top_end.x - control_point_offset, top_end.y);
            top_points.push(self.cubic_bezier(top_start, top_ctrl1, top_ctrl2, top_end, t));

            // Bottom curve (left to right)
            let bottom_start = Pos2::new(x1, y1_end);
            let bottom_end = Pos2::new(x2, y2_end);
            let bottom_ctrl1 = Pos2::new(bottom_start.x + control_point_offset, bottom_start.y);
            let bottom_ctrl2 = Pos2::new(bottom_end.x - control_point_offset, bottom_end.y);
            bottom_points.push(self.cubic_bezier(
                bottom_start,
                bottom_ctrl1,
                bottom_ctrl2,
                bottom_end,
                t,
            ));
        }

        // 2. Build the mesh using a triangle strip.
        // This gives us direct control over rendering and avoids the PathShape artifacts.
        let mut mesh = Mesh::default();
        for i in 0..segments {
            let top_left = top_points[i];
            let top_right = top_points[i + 1];
            let bottom_left = bottom_points[i];
            let bottom_right = bottom_points[i + 1];

            // Create a quad from two triangles.
            let top_left_idx = mesh.vertices.len() as u32;
            mesh.vertices.push(Vertex {
                pos: top_left,
                uv: Pos2::ZERO,
                color,
            });
            let top_right_idx = mesh.vertices.len() as u32;
            mesh.vertices.push(Vertex {
                pos: top_right,
                uv: Pos2::ZERO,
                color,
            });
            let bottom_left_idx = mesh.vertices.len() as u32;
            mesh.vertices.push(Vertex {
                pos: bottom_left,
                uv: Pos2::ZERO,
                color,
            });
            let bottom_right_idx = mesh.vertices.len() as u32;
            mesh.vertices.push(Vertex {
                pos: bottom_right,
                uv: Pos2::ZERO,
                color,
            });

            // Triangle 1: Top-left, top-right, bottom-left
            mesh.add_triangle(top_left_idx, top_right_idx, bottom_left_idx);
            // Triangle 2: Top-right, bottom-right, bottom-left
            mesh.add_triangle(top_right_idx, bottom_right_idx, bottom_left_idx);
        }

        // 3. Add the custom mesh to the painter.
        ui.painter().add(egui::Shape::Mesh(mesh.into()));
    }

    /// Simple, reliable cubic bezier calculation.
    fn cubic_bezier(&self, p0: Pos2, p1: Pos2, p2: Pos2, p3: Pos2, t: f32) -> Pos2 {
        let u = 1.0 - t;
        let u2 = u * u;
        let u3 = u2 * u;
        let t2 = t * t;
        let t3 = t2 * t;
        Pos2 {
            x: u3 * p0.x + 3.0 * u2 * t * p1.x + 3.0 * u * t2 * p2.x + t3 * p3.x,
            y: u3 * p0.y + 3.0 * u2 * t * p1.y + 3.0 * u * t2 * p2.y + t3 * p3.y,
        }
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::with_default_config()
    }
}
