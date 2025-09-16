// diffsplit/src/rendering/mod.rs
// Rendering module for UI rendering logic

use crate::models::*;
use crate::sync;
use crate::theme::JetBrainsTheme;
use egui::epaint::{PathShape, Shape};
use egui::{Color32, FontId, Pos2, Rect, Stroke};

// Type aliases for the draw_connector function
type Color = Color32;

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

/// Highlight renderer for JetBrains-style highlights
pub struct HighlightRenderer {
    theme: JetBrainsTheme,
}

impl HighlightRenderer {
    pub fn new(theme: JetBrainsTheme) -> Self {
        Self { theme }
    }

    // Step 2 — Draw highlights
    pub fn draw_highlight(&self, ui: &mut egui::Ui, rect: Rect) {
        // Fill highlight with semi-transparent green - no borders
        ui.painter()
            .rect_filled(rect, 0.0, Color32::from_rgba_premultiplied(73, 156, 84, 25));
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
        anchors: &[crate::models::AnchorPoint],
        left_rects: &[Rect],
        right_rects: &[Rect],
        line_height: f32,
        top_y: f32,
    ) {
        let x1 = left_rects[0].max.x;
        let x2 = right_rects[0].min.x;
        for (block_idx, block) in change_blocks.iter().enumerate() {
            let anchor = anchors
                .iter()
                .find(|a| a.block_id == format!("block_{}", block_idx))
                .unwrap();
            self.render_connection_block(ui, block, x1, x2, anchor, line_height, top_y);
        }
    }

    fn render_connection_block(
        &self,
        ui: &mut egui::Ui,
        block: &ChangeBlock,
        x1: f32,
        x2: f32,
        anchor: &crate::models::AnchorPoint,
        line_height: f32,
        top_y: f32,
    ) {
        // Calculate block positions based on line indices
        let block_start_y = top_y + (block.start_line as f32 * line_height);
        let block_end_y = top_y + ((block.end_line + 1) as f32 * line_height);

        // Attach connectors to block edges:
        // Left side: right edge of left block (x1 is right edge of left pane)
        // Right side: left edge of right block (x2 is left edge of right pane)
        let y1_start = block_start_y + 2.0; // Top edge of left block
        let y1_end = block_end_y - 2.0; // Bottom edge of left block
        let y2_start = block_start_y + 2.0; // Top edge of right block
        let y2_end = block_end_y - 2.0; // Bottom edge of right block

        // Get the connector color
        let color = self.get_connector_color(block);

        // Draw the S-shaped connector from right edge of left block to left edge of right block
        self.draw_connector(x1, y1_start, y1_end, x2, y2_start, y2_end, color, ui);
    }

    fn get_connector_color(&self, block: &ChangeBlock) -> Color32 {
        match block.line_type {
            LineType::Addition => Color32::from_rgba_unmultiplied(76, 175, 80, 120), // Green with opacity
            LineType::Deletion => Color32::from_rgba_unmultiplied(244, 67, 54, 120), // Red with opacity
            LineType::Context => Color32::from_rgba_unmultiplied(255, 193, 7, 120), // Yellow with opacity
            LineType::Empty => Color32::from_rgba_unmultiplied(100, 150, 200, 120), // Blue with opacity
        }
    }

    pub fn update_theme(&mut self, theme: JetBrainsTheme) {
        self.theme = theme;
    }
    pub fn draw_connector(
        &self,
        x1: f32,
        y1_start: f32,
        y1_end: f32,
        x2: f32,
        y2_start: f32,
        y2_end: f32,
        color: Color,
        canvas: &mut egui::Ui,
    ) -> Vec<(egui::Pos2, egui::Pos2, egui::Pos2, egui::Pos2)> {
        let cp1_x = x1 + (x2 - x1) * 0.35;
        let cp2_x = x2 - (x2 - x1) * 0.35;

        // Adjust control points for S-shape
        let y_diff_top = y2_start - y1_start;
        let cp1_y = y1_start + y_diff_top * 0.2;
        let cp2_y = y2_start - y_diff_top * 0.2;

        let y_diff_bottom = y2_end - y1_end;
        let cp1_y_bottom = y1_end + y_diff_bottom * 0.2;
        let cp2_y_bottom = y2_end - y_diff_bottom * 0.2;

        // Create the path by interpolating Bezier curves
        let mut points = Vec::new();

        // Top curve
        let top_curve_points = self.cubic_bezier_points(
            egui::Pos2::new(x1, y1_start),
            egui::Pos2::new(cp1_x, cp1_y),
            egui::Pos2::new(cp2_x, cp2_y),
            egui::Pos2::new(x2, y2_start),
            20,
        );
        points.extend(top_curve_points);

        // Right side
        points.push(egui::Pos2::new(x2, y2_end));

        // Bottom curve
        let bottom_curve_points = self.cubic_bezier_points(
            egui::Pos2::new(x2, y2_end),
            egui::Pos2::new(cp2_x, cp2_y_bottom),
            egui::Pos2::new(cp1_x, cp1_y_bottom),
            egui::Pos2::new(x1, y1_end),
            20,
        );
        points.extend(bottom_curve_points);

        // Close the path
        points.push(egui::Pos2::new(x1, y1_start));

        // Create the path shape with fill only (no stroke for seamless connection)
        let path_shape = PathShape {
            points,
            closed: true,
            fill: color,
            stroke: egui::epaint::PathStroke::NONE,
        };

        // Draw the connector
        canvas.painter().add(Shape::Path(path_shape));

        // Return the Bezier segments
        vec![
            (
                egui::Pos2::new(x1, y1_start),
                egui::Pos2::new(cp1_x, cp1_y),
                egui::Pos2::new(cp2_x, cp2_y),
                egui::Pos2::new(x2, y2_start),
            ),
            (
                egui::Pos2::new(x2, y2_end),
                egui::Pos2::new(cp2_x, cp2_y_bottom),
                egui::Pos2::new(cp1_x, cp1_y_bottom),
                egui::Pos2::new(x1, y1_end),
            ),
        ]
    }

    // Helper function to generate cubic Bézier curve points
    fn cubic_bezier_points(
        &self,
        p0: egui::Pos2,
        p1: egui::Pos2,
        p2: egui::Pos2,
        p3: egui::Pos2,
        segments: usize,
    ) -> Vec<egui::Pos2> {
        let mut points = Vec::new();
        for i in 1..=segments {
            let t = i as f32 / segments as f32;
            let u = 1.0 - t;
            let u2 = u * u;
            let u3 = u2 * u;
            let t2 = t * t;
            let t3 = t2 * t;

            let x = u3 * p0.x + 3.0 * u2 * t * p1.x + 3.0 * u * t2 * p2.x + t3 * p3.x;
            let y = u3 * p0.y + 3.0 * u2 * t * p1.y + 3.0 * u * t2 * p2.y + t3 * p3.y;

            points.push(egui::Pos2::new(x, y));
        }
        points
    }
}

/// Enhanced rendering system for JetBrains-style diff viewer
pub struct JetBrainsRenderer {
    pub line_renderer: LineRenderer,
    pub highlight_renderer: HighlightRenderer,
    pub connector_renderer: ConnectorRenderer,
}

impl JetBrainsRenderer {
    pub fn new(theme: JetBrainsTheme) -> Self {
        Self {
            line_renderer: LineRenderer::new(theme.clone()),
            highlight_renderer: HighlightRenderer::new(theme.clone()),
            connector_renderer: ConnectorRenderer::new(theme),
        }
    }

    // Step 8 — Paint order: text first, highlights second, then connectors on top
    pub fn render_diff_view(
        &self,
        ui: &mut egui::Ui,
        left_lines: &[DisplayLine],
        right_lines: &[DisplayLine],
        change_blocks: &[ChangeBlock],
        editor_rect: Rect,
        line_height: f32,
    ) {
        let top_y = 50.0;
        let anchors = sync::build_anchors_from_blocks(change_blocks, line_height);
        let mut left_rects = Vec::new();
        let mut right_rects = Vec::new();

        // Create side-by-side layout
        let left_editor_rect = Rect::from_min_size(
            editor_rect.min,
            egui::Vec2::new(editor_rect.width() / 2.0, editor_rect.height()),
        );
        let right_editor_rect = Rect::from_min_size(
            Pos2::new(editor_rect.center().x, editor_rect.top()),
            egui::Vec2::new(editor_rect.width() / 2.0, editor_rect.height()),
        );

        // First pass: render text content for left side
        ui.allocate_ui_at_rect(left_editor_rect, |ui| {
            for (i, line) in left_lines.iter().enumerate() {
                let rect = self.line_renderer.render_line(ui, line, i, true);
                left_rects.push(rect);
            }
        });

        // First pass: render text content for right side
        ui.allocate_ui_at_rect(right_editor_rect, |ui| {
            for (i, line) in right_lines.iter().enumerate() {
                let rect = self.line_renderer.render_line(ui, line, i, false);
                right_rects.push(rect);
            }
        });

        // Second pass: render highlights on top of text
        for (i, _line) in left_lines.iter().enumerate() {
            if let Some(rect) = left_rects.get(i) {
                if self.should_highlight_line(i, change_blocks) {
                    self.highlight_renderer.draw_highlight(ui, *rect);
                }
            }
        }

        for (i, _line) in right_lines.iter().enumerate() {
            if let Some(rect) = right_rects.get(i) {
                if self.should_highlight_line(i, change_blocks) {
                    self.highlight_renderer.draw_highlight(ui, *rect);
                }
            }
        }

        // Third pass: render connectors on top of everything
        self.connector_renderer.render_connections(
            ui,
            change_blocks,
            &anchors,
            &left_rects,
            &right_rects,
            line_height,
            top_y,
        );
    }

    fn should_highlight_line(&self, line_index: usize, change_blocks: &[ChangeBlock]) -> bool {
        change_blocks
            .iter()
            .any(|block| line_index >= block.start_line && line_index <= block.end_line)
    }

    pub fn update_theme(&mut self, theme: JetBrainsTheme) {
        self.line_renderer.update_theme(theme.clone());
        self.highlight_renderer.update_theme(theme.clone());
        self.connector_renderer.update_theme(theme);
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
