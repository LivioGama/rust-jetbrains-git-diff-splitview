// diffsplit/src/ui/layout/mod.rs
// UI Layout module for organizing the main application interface

use eframe::egui;
use egui::{FontId, Pos2, ScrollArea, Vec2};

use crate::config::LayoutConfig;
use crate::diff::imara::ImaraDiffBlock;
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
        self.render_connectors(ui, old_lines, new_lines, pane_width, imara_analysis);
    }

    /// Render connectors between panes
    fn render_connectors(
        &self,
        ui: &mut egui::Ui,
        old_lines: &[DisplayLine],
        new_lines: &[DisplayLine],
        pane_width: f32,
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
                    connectors.push((left_start, left_end, right_start, right_end));
                }
            }

            // Draw crushed lines for pure insertion blocks (added lines with no left block)
            for imara_block in &imara_analysis.blocks {
                if imara_block.is_pure_insertion() && !imara_block.right_range.is_empty() {
                    let right_start = imara_block.right_range.start;
                    let right_end = imara_block.right_range.end.saturating_sub(1);

                    if let (Some(right_start_rect), Some(right_end_rect)) =
                        (right_rects.get(right_start), right_rects.get(right_end))
                    {
                        // Create a crushed line on the left side (same Y coordinates for top and bottom)
                        let left_x_start = if left_rects.is_empty() {
                            gutter_x_start - 100.0 // If no left content, start from a reasonable position
                        } else {
                            // Find the appropriate left position based on context
                            let left_line_for_insertion = if right_start > 0 {
                                // Try to find the last left line before this insertion
                                left_rects.len().saturating_sub(1)
                            } else {
                                0
                            };

                            left_rects
                                .get(
                                    left_line_for_insertion.min(left_rects.len().saturating_sub(1)),
                                )
                                .map(|rect| rect.min.x)
                                .unwrap_or(gutter_x_start - 100.0)
                        };

                        let left_x_end = gutter_x_start;
                        let line_top_y = right_start_rect.top();
                        let line_bottom_y = line_top_y + 2.0; // JetBrains-style 2px height

                        // Use green color for additions with transparency
                        let addition_color = egui::Color32::from_rgba_unmultiplied(76, 175, 80, 64);

                        // Draw a thin line on the left (2px height at the top)
                        self.draw_connector(
                            ui,
                            left_x_start,
                            line_top_y,
                            line_bottom_y,
                            left_x_end,
                            line_top_y,
                            line_bottom_y,
                            addition_color,
                        );

                        // Draw connector from the thin line to the actual right block
                        let right_x_start = right_start_rect.min.x;
                        let right_top_y = right_start_rect.top();
                        let right_bottom_y = right_end_rect.bottom();

                        self.draw_connector(
                            ui,
                            left_x_end,
                            line_top_y,
                            line_bottom_y,
                            right_x_start,
                            right_top_y,
                            right_bottom_y,
                            addition_color,
                        );
                    }
                }
            }

            // Draw crushed lines for pure deletion blocks (deleted lines with no right block)
            for imara_block in &imara_analysis.blocks {
                if imara_block.is_pure_deletion() && !imara_block.left_range.is_empty() {
                    let left_start = imara_block.left_range.start;
                    let left_end = imara_block.left_range.end.saturating_sub(1);

                    if let (Some(left_start_rect), Some(left_end_rect)) =
                        (left_rects.get(left_start), left_rects.get(left_end))
                    {
                        // Create a crushed line on the right side (same Y coordinates for top and bottom)
                        let right_x_start = gutter_x_start + self.config.connector_column_width;
                        let right_x_end = if right_rects.is_empty() {
                            right_x_start + 100.0 // If no right content, extend to a reasonable position
                        } else {
                            // Find the appropriate right position based on context
                            right_rects
                                .get(0)
                                .map(|rect| rect.max.x)
                                .unwrap_or(right_x_start + 100.0)
                        };

                        let line_top_y = left_start_rect.top();
                        let line_bottom_y = line_top_y + 2.0; // JetBrains-style 2px height

                        // Use red color for deletions with transparency
                        let deletion_color = egui::Color32::from_rgba_unmultiplied(244, 67, 54, 64);

                        // Draw connector from the left block to the thin line
                        let left_x_end = left_start_rect.max.x;
                        let left_top_y = left_start_rect.top();
                        let left_bottom_y = left_end_rect.bottom();

                        self.draw_connector(
                            ui,
                            left_x_end,
                            left_top_y,
                            left_bottom_y,
                            right_x_start,
                            line_top_y,
                            line_bottom_y,
                            deletion_color,
                        );

                        // Draw a thin line on the right (2px height at the top)
                        self.draw_connector(
                            ui,
                            right_x_start,
                            line_top_y,
                            line_bottom_y,
                            right_x_end,
                            line_top_y,
                            line_bottom_y,
                            deletion_color,
                        );
                    }
                }
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

                    // Clean connector coordinates - use actual content boundaries for seamless connection
                    let x1 = left_start_rect.max.x; // Exact right edge of left content
                    let x2 = right_start_rect.min.x; // Exact left edge of right content

                    // Determine color based on change type
                    let left_line_type = old_lines.get(left_start).map(|l| &l.line_type);
                    let right_line_type = new_lines.get(right_start).map(|l| &l.line_type);

                    let color = match (left_line_type, right_line_type) {
                        (
                            Some(crate::models::line::LineType::Deletion),
                            Some(crate::models::line::LineType::Addition),
                        ) => {
                            // Modification: blue
                            egui::Color32::from_rgba_unmultiplied(33, 150, 243, 64)
                        }
                        (Some(crate::models::line::LineType::Deletion), _) => {
                            // Deletion: red
                            egui::Color32::from_rgba_unmultiplied(244, 67, 54, 64)
                        }
                        (_, Some(crate::models::line::LineType::Addition)) => {
                            // Addition: green
                            egui::Color32::from_rgba_unmultiplied(76, 175, 80, 64)
                        }
                        _ => {
                            // Default: blue for context changes
                            egui::Color32::from_rgba_unmultiplied(33, 150, 243, 64)
                        }
                    };

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

    /// JetBrains-quality connector using a custom triangle mesh to ensure perfect, artifact-free rendering.
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
        use egui::{epaint::Mesh, epaint::Vertex, Pos2};

        let segments = 32; // Use high resolution for a perfectly smooth curve.
        let mut top_points = Vec::with_capacity(segments + 1);
        let mut bottom_points = Vec::with_capacity(segments + 1);

        let control_point_offset = (x2 - x1) * 0.35;

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
        _connector_renderer: &mut crate::ui::ConnectorRenderer,
        theme: &crate::theme::JetBrainsTheme,
        total_height: f32,
        _pane_width: f32,
        _scroll_sync: &crate::sync::ScrollSync,
    ) {
        // No frame needed - background already handled by main layout
        {
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

                    // Background already filled by main layout - no duplicate fill needed

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

                    // Connector rendering is handled by render_connectors() method only
                    // No additional connector rendering needed here
                    let _line_height = 18.0;
                    // Account for header height (header + separator)
                    let header_height = theme.buffer_font_size() * 1.1 + 20.0;

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
                                let _x1 = full_rect.left() - 1.0; // Extend into left pane
                                let _x2 = full_rect.right() + 1.0; // Extend into right pane

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
                                    let _x1 = full_rect.left() - 1.0; // Extend into left pane
                                    let _x2 = full_rect.right() + 1.0; // Extend into right pane

                                    // Determine color based on change type
                                    let left_line_type =
                                        old_lines.get(left_start).map(|l| &l.line_type);
                                    let right_line_type =
                                        new_lines.get(right_start).map(|l| &l.line_type);
                                    let _color = match (left_line_type, right_line_type) {
                                        (
                                            Some(crate::models::line::LineType::Deletion),
                                            Some(crate::models::line::LineType::Addition),
                                        ) => {
                                            // Modification: blue
                                            egui::Color32::from_rgba_unmultiplied(33, 150, 243, 64)
                                        }
                                        (Some(crate::models::line::LineType::Deletion), _) => {
                                            // Deletion: red
                                            egui::Color32::from_rgba_unmultiplied(244, 67, 54, 64)
                                        }
                                        (_, Some(crate::models::line::LineType::Addition)) => {
                                            // Addition: green
                                            egui::Color32::from_rgba_unmultiplied(76, 175, 80, 64)
                                        }
                                        _ => {
                                            // Default: blue for context changes
                                            egui::Color32::from_rgba_unmultiplied(33, 150, 243, 64)
                                        }
                                    };

                                    // Draw the S-shaped connector linking the two hunks
                                    // DISABLED: Draw the S-shaped connector linking the two hunks
                                    // (Preventing duplicate connector rendering - main connectors handled elsewhere)
                                    // self.draw_connector(
                                    //     ui, x1, y1_start, y1_end, x2, y2_start, y2_end, color,
                                    // );
                                }
                            }
                        }
                    }
                },
            );
        }
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
                        theme.ui_font_size() * 1.1,
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
        _theme: &crate::theme::JetBrainsTheme,
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
