// Connector rendering logic for diff viewer
use egui::epaint::StrokeKind;
use egui::{Color32, Pos2, Rect, Stroke};

use crate::models::types::LineType;
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
        old_lines: &[crate::models::types::DisplayLine],
        new_lines: &[crate::models::types::DisplayLine],
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

            // Find all modified blocks
            let mut left_blocks = Vec::new();
            let mut right_blocks = Vec::new();

            // Find left pane blocks
            let mut i = 0;
            while i < old_lines.len() {
                if old_lines[i].line_type != LineType::Context {
                    let start = i;
                    let change_type = old_lines[i].line_type.clone();
                    let mut end = i;
                    while end + 1 < old_lines.len() && old_lines[end + 1].line_type == change_type {
                        end += 1;
                    }
                    left_blocks.push((start, end));
                    i = end + 1;
                } else {
                    i += 1;
                }
            }

            // Find right pane blocks
            let mut j = 0;
            while j < new_lines.len() {
                if new_lines[j].line_type != LineType::Context {
                    let start = j;
                    let change_type = new_lines[j].line_type.clone();
                    let mut end = j;
                    while end + 1 < new_lines.len() && new_lines[end + 1].line_type == change_type {
                        end += 1;
                    }
                    right_blocks.push((start, end));
                    j = end + 1;
                } else {
                    j += 1;
                }
            }

            // Draw curved Bézier connectors - JetBrains style
            let max_blocks = left_blocks.len().max(right_blocks.len());
            let config = crate::models::types::ConnectorConfig::default();

            for block_idx in 0..max_blocks {
                if let (Some((left_start, left_end)), Some((right_start, right_end))) =
                    (left_blocks.get(block_idx), right_blocks.get(block_idx))
                {
                    if *left_start < left_rects.len() && *right_start < right_rects.len() {
                        let left_first_rect = &left_rects[*left_start];
                        let left_last_rect = &left_rects[*left_end];
                        let right_first_rect = &right_rects[*right_start];
                        let right_last_rect = &right_rects[*right_end];

                        // Calculate block centers for connector attachment
                        let left_center_y = (left_first_rect.min.y + left_last_rect.max.y) / 2.0;
                        let right_center_y = (right_first_rect.min.y + right_last_rect.max.y) / 2.0;

                        let left_center = Pos2::new(left_first_rect.max.x, left_center_y);
                        let right_center = Pos2::new(right_first_rect.min.x, right_center_y);

                        // Determine change type for color
                        let change_type = if old_lines[*left_start].line_type == LineType::Deletion
                            && new_lines[*right_start].line_type == LineType::Addition
                        {
                            "modification"
                        } else if old_lines[*left_start].line_type == LineType::Deletion {
                            "deletion"
                        } else {
                            "addition"
                        };

                        let connector_color = match change_type {
                            "addition" => Color32::from_rgb(76, 175, 80), // Green
                            "deletion" => Color32::from_rgb(244, 67, 54), // Red
                            "modification" => Color32::from_rgb(255, 193, 7), // Yellow
                            _ => Color32::from_rgb(100, 150, 200),
                        };

                        // Create Bézier connector curve
                        let curve = crate::models::types::ConnectorCurve::new(
                            left_center,
                            right_center,
                            &config,
                            connector_color,
                            format!("block_{}", block_idx),
                        );

                        // Calculate block height for multi-line detection
                        let left_block_height =
                            (left_end - left_start + 1) as f32 * self.theme.line_height;
                        let right_block_height =
                            (right_end - right_start + 1) as f32 * self.theme.line_height;
                        let max_block_height = left_block_height.max(right_block_height);

                        // Determine if this is a multi-line block
                        let is_multi_line =
                            (left_end - left_start) > 0 || (right_end - right_start) > 0;

                        if is_multi_line {
                            // Create larger filled region for multi-line blocks positioned at bottom/top
                            self.draw_filled_connector_region(
                                painter,
                                &curve,
                                max_block_height,
                                left_block_height,
                                right_block_height,
                                &left_rects,
                                &right_rects,
                                *left_start,
                                *left_end,
                                *right_start,
                                *right_end,
                            );
                        } else {
                            // Draw simple Bézier curve for single-line blocks
                            painter.add(egui::epaint::Shape::CubicBezier(
                                egui::epaint::CubicBezierShape {
                                    points: [
                                        curve.start,
                                        curve.control1,
                                        curve.control2,
                                        curve.end,
                                    ],
                                    closed: false,
                                    fill: Color32::TRANSPARENT,
                                    stroke: Stroke::new(
                                        curve.thickness,
                                        curve.color.gamma_multiply(0.8),
                                    )
                                    .into(),
                                },
                            ));
                        }
                    }
                }
            }
        }
    }

    fn draw_filled_connector_region(
        &self,
        painter: &egui::Painter,
        curve: &crate::models::types::ConnectorCurve,
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

            // Calculate the bounds of all rectangles in the block
            let left_min_x = left_first_rect.min.x;
            let left_max_x = left_last_rect.max.x;
            let right_min_x = right_first_rect.min.x;
            let right_max_x = right_last_rect.max.x;

            let top_y = left_first_rect.min.y.min(right_first_rect.min.y);
            let bottom_y = left_last_rect.max.y.max(right_last_rect.max.y);

            // Create rectangle covering the entire block area
            let block_rect = Rect::from_min_max(
                Pos2::new(left_max_x, top_y),
                Pos2::new(right_min_x, bottom_y),
            );

            // Fill the rectangle with semi-transparent color
            painter.add(egui::epaint::Shape::rect_filled(
                block_rect, 0.0, // No corner radius
                fill_color,
            ));

            // Draw outline around the filled area
            painter.add(egui::epaint::Shape::rect_stroke(
                block_rect,
                0.0, // No corner radius
                Stroke::new(curve.thickness * 1.5, curve.color.gamma_multiply(0.9)),
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
}
