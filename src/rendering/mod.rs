// diffsplit/src/rendering/mod.rs
// Rendering module for UI rendering logic

use crate::models::*;
use crate::theme::JetBrainsTheme;
use egui::epaint::{PathShape, Shape};
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

    // Enhanced connector rendering with JetBrains-style curved paths
    pub fn render_jetbrains_connector(
        &self,
        ui: &mut egui::Ui,
        left_rect: Rect,
        right_rect: Rect,
        gutter_width: f32,
    ) {
        // Step 3 — Connector anchor points
        let x0 = left_rect.right();
        let y0_top = left_rect.top() + 2.0;
        let y0_bottom = left_rect.bottom() - 2.0;

        let x1 = right_rect.left();
        let y1_top = right_rect.top() + 2.0;
        let y1_bottom = right_rect.bottom() - 2.0;

        // Step 4 — Build connector path
        let stroke = Stroke::new(1.0, Color32::from_rgba_premultiplied(51, 130, 255, 56));

        // Calculate control points for cubic Bézier curves
        let mut cp1_x = x0 + gutter_width * 0.35;
        let mut cp2_x = x1 - gutter_width * 0.35;
        let mut cp1_y = y0_top;
        let mut cp2_y = y1_top;

        // Step 5 — S-shape adjustment for vertical offset
        if y0_top != y1_top {
            let y_diff = y1_top - y0_top;
            cp1_y = y0_top + y_diff * 0.2;
            cp2_y = y1_top - y_diff * 0.2;
        }

        // Create closed path for connector shape
        let mut points = Vec::new();

        // Move to start top
        points.push(Pos2::new(x0, y0_top));

        // Top cubic Bézier curve
        let top_curve_points = self.cubic_bezier_points(
            Pos2::new(x0, y0_top),
            Pos2::new(cp1_x, cp1_y),
            Pos2::new(cp2_x, cp2_y),
            Pos2::new(x1, y1_top),
            20, // number of segments
        );
        points.extend(top_curve_points);

        // Line down right side
        points.push(Pos2::new(x1, y1_bottom));

        // Bottom cubic Bézier curve (reverse direction)
        let mut cp1_y_bottom = y0_bottom;
        let mut cp2_y_bottom = y1_bottom;
        if y0_bottom != y1_bottom {
            let y_diff = y1_bottom - y0_bottom;
            cp1_y_bottom = y0_bottom + y_diff * 0.2;
            cp2_y_bottom = y1_bottom - y_diff * 0.2;
        }

        let bottom_curve_points = self.cubic_bezier_points(
            Pos2::new(x1, y1_bottom),
            Pos2::new(cp2_x, cp2_y_bottom),
            Pos2::new(cp1_x, cp1_y_bottom),
            Pos2::new(x0, y0_bottom),
            20,
        );
        points.extend(bottom_curve_points);

        // Close path back to start
        points.push(Pos2::new(x0, y0_top));

        // Step 6 — Draw connector
        let path_shape = PathShape {
            points,
            closed: true,
            fill: Color32::from_rgba_premultiplied(73, 156, 84, 25),
            stroke: egui::epaint::PathStroke::NONE,
        };

        ui.painter().add(Shape::Path(path_shape));
    }

    // Helper function to generate cubic Bézier curve points
    fn cubic_bezier_points(
        &self,
        p0: Pos2,
        p1: Pos2,
        p2: Pos2,
        p3: Pos2,
        segments: usize,
    ) -> Vec<Pos2> {
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

            points.push(Pos2::new(x, y));
        }
        points
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
        self.connector_renderer
            .render_connections(ui, change_blocks, &left_rects, &right_rects);
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
