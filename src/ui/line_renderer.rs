// Line rendering logic for the diff viewer
use egui::{Color32, FontId, Pos2, Rect};

use crate::models::line::{DisplayLine, LineType};
use crate::rendering::HighlightRenderer;
use crate::theme::JetBrainsTheme;

pub struct LineRenderer {
    theme: JetBrainsTheme,
    highlight_renderer: HighlightRenderer,
}

impl LineRenderer {
    pub fn new(theme: JetBrainsTheme) -> Self {
        Self {
            highlight_renderer: HighlightRenderer::new(theme.clone()),
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
        let line_height = self.theme.line_height;
        let available_width = ui.available_width();

        // Ensure consistent line allocation with no extra margins
        let (rect, _response) = ui.allocate_exact_size(
            egui::Vec2::new(available_width, line_height),
            egui::Sense::hover(),
        );

        // Ensure no item spacing affects positioning
        ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;

        // Ensure identical baseline positioning for both panes
        let baseline_y = rect.min.y + line_height - 3.0;

        // Always fill the entire line background first to prevent white background
        ui.painter().rect_filled(rect, 0.0, self.theme.background);

        // JetBrains-style background colors for diff highlighting using theme
        let bg_color = self.theme.get_line_background(&line.line_type);

        // Draw JetBrains-style highlight for changed lines
        if line.line_type != LineType::Context {
            self.highlight_renderer
                .draw_highlight(ui, rect, &line.line_type);
        } else if bg_color != Color32::TRANSPARENT {
            // Fill background for the entire line for context changes
            ui.painter().rect_filled(rect, 0.0, bg_color);
        }

        // Render line number with enhanced styling (only for non-empty lines)
        if let Some(line_num) = line.original_line_num {
            // Standardized positioning calculation for both panes
            let line_num_pos = Pos2::new(rect.min.x + 30.0, baseline_y);

            ui.painter().text(
                line_num_pos,
                egui::Align2::RIGHT_BOTTOM,
                format!("{}", line_num),
                FontId::new(self.theme.font_size * 0.85, egui::FontFamily::Monospace),
                self.theme.line_numbers,
            );
        }

        // Render change indicator with improved positioning and colors
        let indicator = match line.line_type {
            LineType::Deletion => "−",
            LineType::Addition => "+",
            _ => "",
        };

        if !indicator.is_empty() {
            // Standardized positioning calculation for both panes
            let indicator_pos = Pos2::new(rect.min.x + self.theme.gutter_width - 5.0, baseline_y);

            let indicator_color = match line.line_type {
                LineType::Deletion => self.theme.deletion_gutter,
                LineType::Addition => self.theme.addition_gutter,
                _ => self.theme.line_numbers,
            };

            ui.painter().text(
                indicator_pos,
                egui::Align2::CENTER_BOTTOM,
                indicator,
                FontId::new(self.theme.font_size * 0.9, egui::FontFamily::Monospace),
                indicator_color,
            );
        }

        // Render code content with enhanced syntax highlighting and word-level diffs
        if !line.content.is_empty() {
            let content_start_x = self.theme.gutter_width;
            let text_color = self.get_text_color(&line.content);

            // Override text color for highlighted lines to ensure readability
            let final_text_color = match line.line_type {
                LineType::Deletion => self.theme.deletion_foreground,
                LineType::Addition => self.theme.addition_foreground,
                _ => text_color,
            };

            // Standardized positioning calculation for both panes
            let text_pos = Pos2::new(rect.min.x + content_start_x, baseline_y);

            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_BOTTOM,
                &line.content,
                FontId::new(self.theme.font_size, egui::FontFamily::Monospace),
                final_text_color,
            );

            // Render word-level highlights for modifications
            if line.line_type == LineType::Context && !line.word_highlights.is_empty() {
                for (start, end) in &line.word_highlights {
                    if *start < line.content.len() && *end <= line.content.len() && *start < *end {
                        let char_width = 8.0; // Approximate character width
                        let highlight_start_x = 60.0 + (*start as f32 * char_width);
                        let highlight_width = (*end - *start) as f32 * char_width;

                        // Standardized positioning calculation for both panes
                        let highlight_rect = Rect::from_min_size(
                            Pos2::new(
                                rect.min.x + highlight_start_x,
                                baseline_y - line_height + 6.0,
                            ),
                            egui::Vec2::new(highlight_width, line_height - 4.0),
                        );

                        // Word-level highlight with improved JetBrains colors
                        // Use theme-based color for all word highlights
                        let highlight_color = Color32::from_rgba_unmultiplied(
                            self.theme.color_blue_500.r(),
                            self.theme.color_blue_500.g(),
                            self.theme.color_blue_500.b(),
                            64,
                        );

                        ui.painter()
                            .rect_filled(highlight_rect, 0.0, highlight_color);
                    }
                }
            }
        }

        rect
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
