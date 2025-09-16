// Connector rendering logic for diff viewer
use egui::epaint::{PathShape, Shape};
use egui::{Color32, Pos2, Rect, Stroke};

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
        ui: &mut egui::Ui,
        old_lines: &[crate::models::line::DisplayLine],
        new_lines: &[crate::models::line::DisplayLine],
    ) {
        // Get stored rectangle positions
        let left_rects: Option<Vec<Rect>> = ui
            .ctx()
            .memory_mut(|mem| mem.data.get_persisted("left_rects".into()));
        let right_rects: Option<Vec<Rect>> = ui
            .ctx()
            .memory_mut(|mem| mem.data.get_persisted("right_rects".into()));

        if let (Some(left_rects), Some(right_rects)) = (left_rects, right_rects) {
            let painter = ui.painter();

            // JetBrains approach: Draw connectors based on logical change relationships
            self.draw_jetbrains_connectors(
                painter,
                old_lines,
                new_lines,
                &left_rects,
                &right_rects,
            );
        }
    }

    fn draw_filled_connector_region(
        &self,
        painter: &egui::Painter,
        curve: &crate::models::ui::ConnectorCurve,
        block_height: f32,
        left_block_height: f32,
        right_block_height: f32,
        left_rects: &[Rect],
        right_rects: &[Rect],
        left_start: usize,
        left_end: usize,
        right_start: usize,
        right_end: usize,
    ) {
        // Calculate semi-transparent fill color
        let fill_color = curve.color.gamma_multiply(0.35); // 35% opacity for good visibility

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
    }

    pub fn draw_change_block_connection(
        &self,
        ui: &mut egui::Ui,
        change_type: &str,
        left_start: usize,
        left_end: usize,
        right_start: usize,
        right_end: usize,
        left_rects: &[Rect],
        right_rects: &[Rect],
    ) {
        if left_start >= left_rects.len() || right_start >= right_rects.len() {
            return;
        }

        let painter = ui.painter();

        // Use unified blue color for all connectors
        let stroke_color = Color32::from_rgb(33, 150, 243);

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

        // Create and draw the filled shape without any stroke
        let filled_shape =
            Shape::convex_polygon(path_points, stroke_color, Stroke::new(0.0, stroke_color));
        painter.add(filled_shape);

        // Using filled curves only - no additional stroke lines needed
    }

    /// JetBrains-style connector rendering for independent line arrays
    fn draw_jetbrains_connectors(
        &self,
        painter: &egui::Painter,
        old_lines: &[crate::models::line::DisplayLine],
        new_lines: &[crate::models::line::DisplayLine],
        left_rects: &[egui::Rect],
        right_rects: &[egui::Rect],
    ) {
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
                    Color32::from_rgb(33, 150, 243), // Unified blue
                    end - start > 0,           // Multi-line
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
                    Color32::from_rgb(33, 150, 243), // Unified blue
                    end - start > 0,           // Multi-line
                );
            }
        }
    }

    fn draw_single_connector(
        &self,
        painter: &egui::Painter,
        start_point: Pos2,
        end_point: Pos2,
        config: &crate::models::ui::ConnectorConfig,
        color: Color32,
        is_multi_line: bool,
    ) {
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

        let filled_shape = Shape::convex_polygon(connector_points, color, Stroke::new(0.0, color));
        painter.add(filled_shape);
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
