// Line rendering logic for the diff viewer with Zed IDE font specifications
use egui::{Color32, FontFamily, FontId, Pos2, Rect};

use crate::models::line::{DisplayLine, LineType};
use crate::rendering::HighlightRenderer;
use crate::syntax::SyntaxHighlighter;
use crate::theme::JetBrainsTheme;

pub struct LineRenderer {
    theme: JetBrainsTheme,
    highlight_renderer: HighlightRenderer,
    syntax_highlighter: SyntaxHighlighter,
}

impl LineRenderer {
    pub fn new(theme: JetBrainsTheme) -> Self {
        Self {
            highlight_renderer: HighlightRenderer::new(theme.clone()),
            syntax_highlighter: SyntaxHighlighter::new(),
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
                FontId::new(self.theme.buffer_font_size() * 0.85, FontFamily::Monospace),
                self.theme.line_numbers,
            );
        }

        // Remove change indicators - no plus/minus signs displayed

        // Render code content with JetBrains syntax highlighting
        if !line.content.is_empty() {
            let content_start_x = self.theme.gutter_width;

            // Get syntax-highlighted tokens
            let tokens = self.syntax_highlighter.highlight_line(&line.content);

            if tokens.is_empty() {
                // Fallback to single-color text if no tokens
                let text_color = self.get_text_color(&line.content);
                let final_text_color = match line.line_type {
                    LineType::Deletion => self.theme.deletion_foreground,
                    LineType::Addition => self.theme.addition_foreground,
                    LineType::Modification => self.theme.modification_foreground,
                    _ => text_color,
                };

                let text_pos = Pos2::new(rect.min.x + content_start_x, baseline_y);
                ui.painter().text(
                    text_pos,
                    egui::Align2::LEFT_BOTTOM,
                    &line.content,
                    self.theme.buffer_font_id(),
                    final_text_color,
                );
            } else {
                // Render each token with its appropriate color
                let mut current_x = rect.min.x + content_start_x;

                for token in tokens {
                    let token_color = match line.line_type {
                        LineType::Addition => {
                            // For addition lines, blend syntax color with addition foreground
                            self.blend_colors(
                                self.syntax_highlighter
                                    .get_color_for_token(&token.token_type),
                                self.theme.addition_foreground,
                                0.7,
                            )
                        }
                        LineType::Deletion => {
                            // For deletion lines, blend syntax color with deletion foreground
                            self.blend_colors(
                                self.syntax_highlighter
                                    .get_color_for_token(&token.token_type),
                                self.theme.deletion_foreground,
                                0.7,
                            )
                        }
                        LineType::Modification => {
                            // For modification lines, blend syntax color with modification foreground
                            self.blend_colors(
                                self.syntax_highlighter
                                    .get_color_for_token(&token.token_type),
                                self.theme.modification_foreground,
                                0.7,
                            )
                        }
                        _ => {
                            // For context lines, use pure syntax highlighting
                            self.syntax_highlighter
                                .get_color_for_token(&token.token_type)
                        }
                    };

                    ui.painter().text(
                        Pos2::new(current_x, baseline_y),
                        egui::Align2::LEFT_BOTTOM,
                        &token.text,
                        self.theme.buffer_font_id(),
                        token_color,
                    );

                    // Calculate the width of the rendered text to position the next token
                    let text_width = ui
                        .painter()
                        .layout_no_wrap(
                            token.text.clone(),
                            self.theme.buffer_font_id(),
                            Color32::TRANSPARENT,
                        )
                        .size()
                        .x;

                    current_x += text_width;
                }
            }

            // Render word-level highlights for modifications
            if (line.line_type == LineType::Context || line.line_type == LineType::Modification)
                && !line.word_highlights.is_empty()
            {
                let char_count = line.content.chars().count();
                for (start, end, highlight_type) in &line.word_highlights {
                    let char_start = line.content[..*start].chars().count();
                    let char_end = line.content[..*end].chars().count();
                    if char_start < char_count && char_end <= char_count && char_start < char_end {
                        let char_width = self.theme.char_width(); // Use Zed-calculated character width
                        let highlight_start_x =
                            60.0 + (char_start as f32 * char_width) - (1.5 * char_width);
                        let highlight_width = (char_end - char_start) as f32 * char_width;

                        // Standardized positioning calculation for both panes
                        let highlight_rect = Rect::from_min_size(
                            Pos2::new(
                                rect.min.x + highlight_start_x,
                                baseline_y - line_height + 6.0,
                            ),
                            egui::Vec2::new(highlight_width, line_height - 4.0),
                        );

                        // Word-level highlight with JetBrains colors
                        let base_color = match highlight_type {
                            crate::models::line::HighlightType::Insert => {
                                self.theme.addition_foreground
                            }
                            crate::models::line::HighlightType::Delete => self.theme.color_blue_500,
                        };
                        let highlight_color = Color32::from_rgba_unmultiplied(
                            base_color.r(),
                            base_color.g(),
                            base_color.b(),
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
