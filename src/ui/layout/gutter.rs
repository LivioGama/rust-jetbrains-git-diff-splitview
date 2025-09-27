// src/ui/layout/gutter.rs
// Gutter rendering logic extracted from layout/mod.rs

use eframe::egui;
use egui::Vec2;
use crate::config::LayoutConfig;
use crate::models::line::DisplayLine;

/// Gutter rendering functionality for the layout manager
pub struct GutterRenderer {
    config: LayoutConfig,
}

impl GutterRenderer {
    pub fn new(config: LayoutConfig) -> Self {
        Self { config }
    }

    /// Render connector gutter between the two panes
    pub fn render_connector_gutter(
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
                    let _full_rect = ui
                        .allocate_response(
                            Vec2::new(self.config.connector_column_width, total_height),
                            egui::Sense::hover(),
                        )
                        .rect;

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

                    // Draw connector ribbons between corresponding hunks
                    let line_height = theme.line_height();
                    for (left_hunk, right_hunk) in left_hunks.iter().zip(right_hunks.iter()) {
                        let left_start_y = left_hunk.0 as f32 * line_height;
                        let left_end_y = (left_hunk.1 + 1) as f32 * line_height;
                        let right_start_y = right_hunk.0 as f32 * line_height;
                        let right_end_y = (right_hunk.1 + 1) as f32 * line_height;

                        // Calculate connector geometry
                        let connector_start_x = 0.0;
                        let connector_end_x = self.config.connector_column_width;

                        // Use a simple polygon for the connector
                        let points = vec![
                            egui::Pos2::new(connector_start_x, left_start_y),
                            egui::Pos2::new(connector_start_x, left_end_y),
                            egui::Pos2::new(connector_end_x, right_end_y),
                            egui::Pos2::new(connector_end_x, right_start_y),
                        ];

                        let sample_left = old_lines
                            .get(left_hunk.0)
                            .map(|line| line.line_type.clone());
                        let sample_right = new_lines
                            .get(right_hunk.0)
                            .map(|line| line.line_type.clone());

                        let connector_color = match (sample_left, sample_right) {
                            (_, Some(crate::models::line::LineType::Addition))
                            | (Some(crate::models::line::LineType::Addition), _)
                            | (None, Some(crate::models::line::LineType::Addition)) => {
                                let base = theme.addition_background;
                                egui::Color32::from_rgba_unmultiplied(
                                    base.r(),
                                    base.g(),
                                    base.b(),
                                    base.a(),
                                )
                            }
                            (Some(crate::models::line::LineType::Deletion), _)
                            | (_, Some(crate::models::line::LineType::Deletion)) => {
                                let base = theme.deletion_background;
                                egui::Color32::from_rgba_unmultiplied(
                                    base.r(),
                                    base.g(),
                                    base.b(),
                                    base.a(),
                                )
                            }
                            (Some(crate::models::line::LineType::Modification), _)
                            | (_, Some(crate::models::line::LineType::Modification)) => {
                                let base = theme.modification_background;
                                egui::Color32::from_rgba_unmultiplied(
                                    base.r(),
                                    base.g(),
                                    base.b(),
                                    base.a(),
                                )
                            }
                            _ => {
                                let base = theme.modification_background;
                                egui::Color32::from_rgba_unmultiplied(
                                    base.r(),
                                    base.g(),
                                    base.b(),
                                    base.a(),
                                )
                            }
                        };

                        // Draw the filled polygon
                        ui.painter().add(egui::epaint::Shape::convex_polygon(
                            points,
                            connector_color,
                            egui::Stroke::NONE,
                        ));
                    }
                },
            );
        }
    }
}
