// diffsplit/src/rendering/mod.rs
// Rendering module for UI rendering logic

use crate::models::*;
use crate::theme::JetBrainsTheme;
use egui::{Color32, FontId, Pos2, Rect, Stroke};

/// Rendering context for managing rendering state
pub struct RenderContext {
    pub theme: JetBrainsTheme,
    pub line_height: f32,
    pub font_size: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

impl RenderContext {
    pub fn new(theme: JetBrainsTheme, viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            theme,
            line_height: 18.0,
            font_size: 14.0,
            viewport_width,
            viewport_height,
        }
    }

    pub fn update_viewport(&mut self, width: f32, height: f32) {
        self.viewport_width = width;
        self.viewport_height = height;
    }
}

/// Line renderer for rendering individual lines
pub struct LineRenderer {
    theme: JetBrainsTheme,
}

impl LineRenderer {
    pub fn new(theme: JetBrainsTheme) -> Self {
        Self { theme }
    }

    pub fn render_line(
        &self,
        ui: &mut egui::Ui,
        line: &DisplayLine,
        line_idx: usize,
        is_left: bool,
    ) -> Rect {
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), self.theme.line_height),
            egui::Sense::hover(),
        );

        // Draw background based on line type
        let bg_color = self.get_line_background_color(line, is_left);
        ui.painter().rect_filled(rect, 0.0, bg_color);

        // Draw line number if enabled
        if self.theme.show_line_numbers {
            self.render_line_number(ui, line_idx, rect);
        }

        // Draw line content
        self.render_line_content(ui, line, rect);

        // Draw word highlights
        self.render_word_highlights(ui, line, rect);

        // Draw selection indicator if line is selected
        if response.hovered() {
            ui.painter().rect_stroke(
                rect,
                0.0,
                Stroke::new(1.0, Color32::from_rgb(100, 150, 255)),
                egui::StrokeKind::Middle,
            );
        }

        rect
    }

    fn get_line_background_color(&self, line: &DisplayLine, is_left: bool) -> Color32 {
        match line.line_type {
            LineType::Addition => {
                if is_left {
                    self.theme.deletion_background
                } else {
                    self.theme.addition_background
                }
            }
            LineType::Deletion => {
                if is_left {
                    self.theme.deletion_background
                } else {
                    self.theme.addition_background
                }
            }
            LineType::Context => {
                if line.word_highlights.is_empty() {
                    Color32::TRANSPARENT
                } else {
                    self.theme.modification_background
                }
            }
            LineType::Empty => Color32::TRANSPARENT,
        }
    }

    fn render_line_number(&self, ui: &mut egui::Ui, line_idx: usize, rect: Rect) {
        let number_text = format!("{:4}", line_idx + 1);
        let number_rect = Rect::from_min_max(rect.min, Pos2::new(rect.min.x + 40.0, rect.max.y));

        ui.painter().text(
            number_rect.center(),
            egui::Align2::CENTER_CENTER,
            number_text,
            FontId::monospace(self.theme.font_size),
            self.theme.line_numbers,
        );
    }

    fn render_line_content(&self, ui: &mut egui::Ui, line: &DisplayLine, rect: Rect) {
        let text_color = self.get_text_color(line);
        let content_rect = if self.theme.show_line_numbers {
            Rect::from_min_max(Pos2::new(rect.min.x + 45.0, rect.min.y), rect.max)
        } else {
            rect
        };

        ui.painter().text(
            content_rect.left_center(),
            egui::Align2::LEFT_CENTER,
            &line.content,
            FontId::monospace(self.theme.font_size),
            text_color,
        );
    }

    fn render_word_highlights(&self, ui: &mut egui::Ui, line: &DisplayLine, rect: Rect) {
        if line.word_highlights.is_empty() {
            return;
        }

        let content_rect = if self.theme.show_line_numbers {
            Rect::from_min_max(Pos2::new(rect.min.x + 45.0, rect.min.y), rect.max)
        } else {
            rect
        };

        for (start, end) in &line.word_highlights {
            if *start >= line.content.len() {
                continue;
            }

            let highlight_start = *start.min(&line.content.len());
            let highlight_end = *end.min(&line.content.len());

            if highlight_start >= highlight_end {
                continue;
            }

            // Calculate highlight rectangle
            let text_before = &line.content[..highlight_start];
            let highlighted_text = &line.content[highlight_start..highlight_end];

            let before_width = ui
                .painter()
                .layout_no_wrap(
                    text_before.to_string(),
                    FontId::monospace(self.theme.font_size),
                    Color32::TRANSPARENT,
                )
                .size()
                .x;

            let highlight_width = ui
                .painter()
                .layout_no_wrap(
                    highlighted_text.to_string(),
                    FontId::monospace(self.theme.font_size),
                    Color32::TRANSPARENT,
                )
                .size()
                .x;

            let highlight_rect = Rect::from_min_max(
                Pos2::new(content_rect.min.x + before_width, content_rect.min.y),
                Pos2::new(
                    content_rect.min.x + before_width + highlight_width,
                    content_rect.max.y,
                ),
            );

            ui.painter()
                .rect_filled(highlight_rect, 2.0, self.theme.modification_foreground);
        }
    }

    fn get_text_color(&self, line: &DisplayLine) -> Color32 {
        match line.line_type {
            LineType::Addition => self.theme.addition_foreground,
            LineType::Deletion => self.theme.deletion_foreground,
            LineType::Context => {
                if line.word_highlights.is_empty() {
                    self.theme.foreground
                } else {
                    self.theme.modification_foreground
                }
            }
            LineType::Empty => self.theme.foreground,
        }
    }

    pub fn update_theme(&mut self, theme: JetBrainsTheme) {
        self.theme = theme;
    }
}

/// Connector renderer for rendering connection lines
pub struct ConnectorRenderer {
    theme: JetBrainsTheme,
}

impl ConnectorRenderer {
    pub fn new(theme: JetBrainsTheme) -> Self {
        Self { theme }
    }

    pub fn render_connections(
        &self,
        ui: &mut egui::Ui,
        change_blocks: &[ChangeBlock],
        left_rects: &[Rect],
        right_rects: &[Rect],
    ) {
        for (block_idx, block) in change_blocks.iter().enumerate() {
            if let (Some(left_rect), Some(right_rect)) = (
                left_rects.get(block.start_line),
                right_rects.get(block.start_line),
            ) {
                self.render_connection_block(ui, block, *left_rect, *right_rect, block_idx);
            }
        }
    }

    fn render_connection_block(
        &self,
        ui: &mut egui::Ui,
        block: &ChangeBlock,
        left_rect: Rect,
        right_rect: Rect,
        block_idx: usize,
    ) {
        let left_center = left_rect.center();
        let right_center = right_rect.center();

        // Create connector curve
        let config = ConnectorConfig::default();
        let curve = ConnectorCurve::new(
            left_center,
            right_center,
            &config,
            self.get_connector_color(block),
            format!("block_{}", block_idx),
        );

        // Render the curve
        ui.painter().add(egui::epaint::Shape::CubicBezier(
            egui::epaint::CubicBezierShape {
                points: [curve.start, curve.control1, curve.control2, curve.end],
                closed: false,
                fill: Color32::TRANSPARENT,
                stroke: Stroke::new(curve.thickness, curve.color).into(),
            },
        ));
    }

    fn get_connector_color(&self, block: &ChangeBlock) -> Color32 {
        match block.line_type {
            LineType::Addition => self.theme.addition_foreground,
            LineType::Deletion => self.theme.deletion_foreground,
            LineType::Context => self.theme.modification_foreground,
            LineType::Empty => self.theme.foreground,
        }
    }

    pub fn update_theme(&mut self, theme: JetBrainsTheme) {
        self.theme = theme;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::JetBrainsTheme;

    #[test]
    fn test_render_context_creation() {
        let theme = JetBrainsTheme::dark_theme();
        let context = RenderContext::new(theme, 800.0, 600.0);
        assert_eq!(context.viewport_width, 800.0);
        assert_eq!(context.viewport_height, 600.0);
    }

    #[test]
    fn test_line_renderer_creation() {
        let theme = JetBrainsTheme::dark_theme();
        let renderer = LineRenderer::new(theme);
        // Test passes if renderer is created successfully
    }

    #[test]
    fn test_connector_renderer_creation() {
        let theme = JetBrainsTheme::dark_theme();
        let renderer = ConnectorRenderer::new(theme);
        // Test passes if renderer is created successfully
    }
}
