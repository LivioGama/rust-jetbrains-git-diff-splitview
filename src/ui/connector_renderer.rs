// Connector rendering logic for diff viewer
use egui::{Color32, Pos2, Rect};

use crate::models::line::LineType;
use crate::theme::JetBrainsTheme;

pub struct ConnectorRenderer {
    theme: JetBrainsTheme,
}

impl ConnectorRenderer {
    pub fn new(theme: JetBrainsTheme) -> Self {
        Self { theme }
    }

    pub fn draw_connection_lines(
        &self,
        _ui: &mut egui::Ui,
        _old_lines: &[crate::models::line::DisplayLine],
        _new_lines: &[crate::models::line::DisplayLine],
    ) {
        // DISABLED: All connector rendering is handled by LayoutManager.render_connectors()
        // This prevents duplicate/overlapping connector rendering
    }

    fn draw_filled_connector_region(
        &self,
        _painter: &egui::Painter,
        _curve: &crate::models::ui::ConnectorCurve,
        _block_height: f32,
        _left_block_height: f32,
        _right_block_height: f32,
        _left_rects: &[Rect],
        _right_rects: &[Rect],
        _left_start: usize,
        _left_end: usize,
        _right_start: usize,
        _right_end: usize,
    ) {
        // DISABLED: All connector rendering is handled by LayoutManager.render_connectors()
        // This prevents duplicate/overlapping connector rendering
        /*
        // Calculate semi-transparent fill color using consistent alpha method
        let fill_color = Color32::from_rgba_unmultiplied(
            curve.color.r(),
            curve.color.g(),
            curve.color.b(),
            64  // Consistent with theme alpha values
        );

        // Use actual line rectangles to calculate the filled region bounds
        if left_start < left_rects.len() && right_start < right_rects.len() {
            let left_first_rect = &left_rects[left_start];
            let left_last_rect = &left_rects[left_end.min(left_rects.len() - 1)];
            let right_first_rect = &right_rects[right_start];
            let right_last_rect = &right_rects[right_end.min(right_rects.len() - 1)];

            // Calculate precise bounds for better alignment
            let left_max_x = left_first_rect.max.x - 5.0; // Slightly inset from line edge
            let right_min_x = right_first_rect.min.x + 5.0; // Slightly inset from line edge

            // Use exact line bounds for vertical alignment
            let top_y = left_first_rect.min.y.min(right_first_rect.min.y);
            let bottom_y = left_last_rect.max.y.max(right_last_rect.max.y);

            // Create rectangle covering the connector area with precise alignment
            let block_rect = Rect::from_min_max(
                Pos2::new(left_max_x, top_y),
                Pos2::new(right_min_x, bottom_y),
            );

            // Fill the rectangle with semi-transparent color
            painter.add(egui::epaint::Shape::rect_filled(
                block_rect, 2.0, // Slight corner radius for better aesthetics
                fill_color,
            ));
        }
        */
    }

    pub fn draw_change_block_connection(
        &self,
        _ui: &mut egui::Ui,
        _change_type: &str,
        _left_start: usize,
        _left_end: usize,
        _right_start: usize,
        _right_end: usize,
        _left_rects: &[Rect],
        _right_rects: &[Rect],
    ) {
        // DISABLED: All connector rendering is handled by LayoutManager.render_connectors()
        // This prevents duplicate/overlapping connector rendering
        /*
        if left_start >= left_rects.len() || right_start >= right_rects.len() {
            return;
        }

        let painter = ui.painter();

        // Use theme-based color based on change type
        let line_type = match change_type {
            "addition" => crate::models::line::LineType::Addition,
            "deletion" => crate::models::line::LineType::Deletion,
            "modification" => crate::models::line::LineType::Modification, // Modifications use new Modification type
            _ => crate::models::line::LineType::Context,
        };
        let stroke_color = self.theme.get_connector_color(&line_type);

        // Calculate the vertical span of the change blocks
        let left_top = left_rects[left_start].min.y;
        let left_bottom = left_rects[left_end.min(left_rects.len() - 1)].max.y;
        let right_top = right_rects[right_start].min.y;
        let right_bottom = right_rects[right_end.min(right_rects.len() - 1)].max.y;

        // Connect the top boundaries
        let start_top = Pos2::new(left_rects[left_start].max.x, left_top);
        let end_top = Pos2::new(right_rects[right_start].min.x, right_top);

        // Connect the bottom boundaries
        let start_bottom = Pos2::new(
            left_rects[left_end.min(left_rects.len() - 1)].max.x,
            left_bottom,
        );
        let end_bottom = Pos2::new(
            right_rects[right_end.min(right_rects.len() - 1)].min.x,
            right_bottom,
        );

        // Draw curved connection lines for the band
        let control_distance = 40.0;

        // Top curve
        let control1_top = Pos2::new(start_top.x + control_distance, start_top.y);
        let control2_top = Pos2::new(end_top.x - control_distance, end_top.y);

        // Bottom curve
        let control1_bottom = Pos2::new(start_bottom.x + control_distance, start_bottom.y);
        let control2_bottom = Pos2::new(end_bottom.x - control_distance, end_bottom.y);

        // Collect all points to create the filled polygon path
        let mut path_points = Vec::new();

        // Add top curve points
        for i in 0..=32 {
            let t = i as f32 / 32.0;
            path_points.push(self.evaluate_cubic_bezier(
                start_top,
                control1_top,
                control2_top,
                end_top,
                t,
            ));
        }

        // Add bottom curve points (in reverse order to close the shape)
        for i in (0..=32).rev() {
            let t = i as f32 / 32.0;
            path_points.push(self.evaluate_cubic_bezier(
                start_bottom,
                control1_bottom,
                control2_bottom,
                end_bottom,
                t,
            ));
        }

        // Create and draw the filled shape as a single closed path (single layer, no stroke)
        let path_shape = egui::epaint::PathShape {
            points: path_points,
            closed: true,
            fill: stroke_color,
            stroke: egui::epaint::PathStroke::NONE,
        };
        painter.add(egui::Shape::Path(path_shape));

        // Using filled curves only - single layer
        */
    }

    /// JetBrains-style connector rendering for independent line arrays
    fn draw_jetbrains_connectors(
        &self,
        _ui: &mut egui::Ui,
        _old_lines: &[crate::models::line::DisplayLine],
        _new_lines: &[crate::models::line::DisplayLine],
    ) {
        // DISABLED: All connector rendering is handled by LayoutManager.render_connectors()
        // This prevents duplicate/overlapping connector rendering
        /*
        let config = crate::models::ui::ConnectorConfig::default();

        // Find deletion blocks on left side
        let deletion_blocks = self.find_change_blocks(old_lines, LineType::Deletion);

        // Find addition blocks on right side
        let addition_blocks = self.find_change_blocks(new_lines, LineType::Addition);

        // Draw deletion connectors (red, pointing from left to middle)
        for (start, end) in deletion_blocks {
            if start < left_rects.len() && end < left_rects.len() {
                let start_rect = &left_rects[start];
                let end_rect = &left_rects[end];

                let center_y = (start_rect.min.y + end_rect.max.y) / 2.0;
                let left_point = Pos2::new(start_rect.max.x, center_y);
                let right_point = Pos2::new(left_point.x + 30.0, center_y);

                self.draw_single_connector(
                    painter,
                    left_point,
                    right_point,
                    &config,
                    self.theme.get_connector_color(&LineType::Deletion),
                    end - start > 0, // Multi-line
                );
            }
        }

        // Draw addition connectors (green, pointing from middle to right)
        for (start, end) in addition_blocks {
            if start < right_rects.len() && end < right_rects.len() {
                let start_rect = &right_rects[start];
                let end_rect = &right_rects[end];

                let center_y = (start_rect.min.y + end_rect.max.y) / 2.0;
                let right_point = Pos2::new(start_rect.min.x, center_y);
                let left_point = Pos2::new(right_point.x - 30.0, center_y);

                self.draw_single_connector(
                    painter,
                    left_point,
                    right_point,
                    &config,
                    self.theme.get_connector_color(&LineType::Addition),
                    end - start > 0, // Multi-line
                );
            }
        }
        */
    }

    fn draw_single_connector(
        &self,
        _painter: &egui::Painter,
        _start_point: Pos2,
        _end_point: Pos2,
        _color: Color32,
        _is_multi_line: bool,
    ) {
        // DISABLED: All connector rendering is handled by LayoutManager.render_connectors()
        // This prevents duplicate/overlapping connector rendering
        /*
        let curve = crate::models::ui::ConnectorCurve::new(
            start_point,
            end_point,
            config,
            color,
            "connector".to_string(),
        );

        let thickness = if is_multi_line {
            config.ribbon_width * 1.2
        } else {
            config.ribbon_width * 0.8
        };

        // For single connector, create a simple curved line using convex polygon
        let mut path_points = Vec::new();

        // Generate curve points
        for i in 0..=20 {
            let t = i as f32 / 20.0;
            path_points.push(self.evaluate_cubic_bezier(
                curve.start,
                curve.control1,
                curve.control2,
                curve.end,
                t,
            ));
        }

        // Create a complete connector by adding points slightly offset for thickness
        let mut connector_points = path_points.clone();
        for point in path_points.iter().rev() {
            connector_points.push(Pos2::new(point.x, point.y + thickness));
        }

        // Render as a single closed path (single layer), no stroke
        let path_shape = egui::epaint::PathShape {
            points: connector_points,
            closed: true,
            fill: color,
            stroke: egui::epaint::PathStroke::NONE,
        };
        painter.add(egui::Shape::Path(path_shape));
        */
    }

    /// Find change blocks of a specific type in a line array
    fn find_change_blocks(
        &self,
        lines: &[crate::models::line::DisplayLine],
        line_type: LineType,
    ) -> Vec<(usize, usize)> {
        let mut blocks = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            if lines[i].line_type == line_type {
                let start = i;
                let mut end = i;

                // Find consecutive lines of the same change type
                while end + 1 < lines.len() && lines[end + 1].line_type == line_type {
                    end += 1;
                }

                blocks.push((start, end));
                i = end + 1;
            } else {
                i += 1;
            }
        }

        blocks
    }

    fn evaluate_cubic_bezier(&self, p0: Pos2, p1: Pos2, p2: Pos2, p3: Pos2, t: f32) -> Pos2 {
        let u = 1.0 - t;
        let tt = t * t;
        let uu = u * u;
        let uuu = uu * u;
        let ttt = tt * t;

        Pos2::new(
            uuu * p0.x + 3.0 * uu * t * p1.x + 3.0 * u * tt * p2.x + ttt * p3.x,
            uuu * p0.y + 3.0 * uu * t * p1.y + 3.0 * u * tt * p2.y + ttt * p3.y,
        )
    }
}
