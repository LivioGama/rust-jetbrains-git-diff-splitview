// Line rendering logic for the diff viewer with Zed IDE font specifications
use egui::{Color32, Rect};

use crate::models::line::{DisplayLine, LineType};
use crate::rendering::{HighlightRenderer, TextRenderer};
use crate::theme::JetBrainsTheme;

pub struct LineRenderer {
    theme: JetBrainsTheme,
    highlight_renderer: HighlightRenderer,
    text_renderer: TextRenderer,
}

impl LineRenderer {
    pub fn new(theme: JetBrainsTheme) -> Self {
        Self {
            highlight_renderer: HighlightRenderer::new(theme.clone()),
            text_renderer: TextRenderer::new(theme.clone()),
            theme,
        }
    }

    pub fn render_line(
        &self,
        ui: &mut egui::Ui,
        line: &DisplayLine,
        _line_idx: usize,
        _is_left: bool,
    ) -> Rect {
        let line_height = self.theme.line_height();
        let available_width = ui.available_width();

        // Ensure consistent line allocation with no extra margins
        let (rect, _response) = ui.allocate_exact_size(
            egui::Vec2::new(available_width, line_height),
            egui::Sense::hover(),
        );

        // Ensure no item spacing affects positioning
        ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;

        // Use Zed-style baseline calculation for proper text alignment
        let baseline_y = rect.min.y + self.theme.baseline_offset();

        // Always fill the entire line background first to prevent white background
        ui.painter().rect_filled(rect, 0.0, self.theme.background);

        // JetBrains-style background colors for diff highlighting using theme
        let bg_color = self.theme.get_line_background(&line.line_type);

        // Draw JetBrains-style highlight for changed lines - ALWAYS draw background for changes
        if bg_color != Color32::TRANSPARENT {
            // Fill background for the entire line
            ui.painter().rect_filled(rect, 0.0, bg_color);
        }

        // Additional highlight rendering for special cases
        if line.line_type != LineType::Context {
            self.highlight_renderer
                .draw_highlight(ui, rect, &line.line_type);
        }

        // Draw 2px left indicator for changed lines (JetBrains style)
        if line.line_type != LineType::Context {
            let indicator_color = match line.line_type {
                LineType::Addition => self.theme.addition_background,
                LineType::Deletion => self.theme.deletion_background,
                LineType::Modification => self.theme.modification_background,
                _ => Color32::TRANSPARENT,
            };

            if indicator_color != Color32::TRANSPARENT {
                // Draw 2px wide colored bar on the left edge
                let indicator_rect = egui::Rect::from_min_size(
                    egui::Pos2::new(rect.min.x, rect.min.y),
                    egui::Vec2::new(2.0, rect.height()),
                );
                ui.painter()
                    .rect_filled(indicator_rect, 0.0, indicator_color);
            }
        }

        // Use TextRenderer for line number rendering
        self.text_renderer
            .render_line_number(ui, line, rect, baseline_y);

        // Use TextRenderer for content and highlighting
        self.text_renderer
            .render_content(ui, line, rect, baseline_y);
        self.text_renderer
            .render_word_highlights(ui, line, rect, baseline_y);

        // Note: Block delimiters will be drawn by the pane renderer, not per-line

        rect
    }

    fn blend_colors(
        &self,
        syntax_color: Color32,
        line_color: Color32,
        syntax_weight: f32,
    ) -> Color32 {
        let line_weight = 1.0 - syntax_weight;

        let r =
            (syntax_color.r() as f32 * syntax_weight + line_color.r() as f32 * line_weight) as u8;
        let g =
            (syntax_color.g() as f32 * syntax_weight + line_color.g() as f32 * line_weight) as u8;
        let b =
            (syntax_color.b() as f32 * syntax_weight + line_color.b() as f32 * line_weight) as u8;

        Color32::from_rgb(r, g, b)
    }

    fn get_text_color(&self, content: &str) -> Color32 {
        // Enhanced syntax highlighting with improved color detection
        let trimmed = content.trim();

        if trimmed.starts_with("//") || trimmed.starts_with("#") {
            self.theme.code_comment
        } else if trimmed.contains("fn ")
            || trimmed.contains("let ")
            || trimmed.contains("const ")
            || trimmed.contains("struct ")
            || trimmed.contains("enum ")
            || trimmed.contains("impl ")
            || trimmed.contains("pub ")
            || trimmed.contains("use ")
        {
            self.theme.code_keyword
        } else if trimmed.contains("\"") || trimmed.contains("'") {
            self.theme.code_string
        } else {
            self.theme.code_foreground
        }
    }
}
