// Connector rendering logic for diff viewer
use egui::epaint::StrokeKind;
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

            // Draw subtle outline around the filled area for definition
            painter.add(egui::epaint::Shape::rect_stroke(
                block_rect,
                2.0,                                               // Matching corner radius
                Stroke::new(1.0, curve.color.gamma_multiply(0.6)), // Thinner, more subtle outline
                StrokeKind::Middle,
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

        // Get the color based on change type
        let stroke_color = match change_type {
            "addition" => Color32::from_rgb(80, 160, 80), // Green for additions
            "deletion" => Color32::from_rgb(160, 80, 80), // Red for deletions
            "modification" => Color32::from_rgb(160, 160, 80), // Yellow for modifications
            _ => Color32::from_rgb(100, 150, 200),        // Default blue
        };

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

        painter.add(egui::epaint::Shape::CubicBezier(
            egui::epaint::CubicBezierShape {
                points: [start_top, control1_top, control2_top, end_top],
                closed: false,
                fill: Color32::TRANSPARENT,
                stroke: Stroke::new(2.0, stroke_color).into(),
            },
        ));

        // Bottom curve
        let control1_bottom = Pos2::new(start_bottom.x + control_distance, start_bottom.y);
        let control2_bottom = Pos2::new(end_bottom.x - control_distance, end_bottom.y);

        painter.add(egui::epaint::Shape::CubicBezier(
            egui::epaint::CubicBezierShape {
                points: [start_bottom, control1_bottom, control2_bottom, end_bottom],
                closed: false,
                fill: Color32::TRANSPARENT,
                stroke: Stroke::new(2.0, stroke_color).into(),
            },
        ));

        // Draw vertical connecting lines to create a band effect
        if (left_bottom - left_top).abs() > 1.0 {
            painter.add(egui::epaint::Shape::line_segment(
                [start_top, start_bottom],
                Stroke::new(1.0, stroke_color),
            ));
        }

        if (right_bottom - right_top).abs() > 1.0 {
            painter.add(egui::epaint::Shape::line_segment(
                [end_top, end_bottom],
                Stroke::new(1.0, stroke_color),
            ));
        }
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
                    Color32::from_rgb(244, 67, 54), // Red for deletions
                    end - start > 0,                // Multi-line
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
                    Color32::from_rgb(76, 175, 80), // Green for additions
                    end - start > 0,                // Multi-line
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

        painter.add(egui::epaint::Shape::CubicBezier(
            egui::epaint::CubicBezierShape {
                points: [curve.start, curve.control1, curve.control2, curve.end],
                closed: false,
                fill: Color32::TRANSPARENT,
                stroke: Stroke::new(thickness, color.gamma_multiply(0.7)).into(),
            },
        ));
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
}
