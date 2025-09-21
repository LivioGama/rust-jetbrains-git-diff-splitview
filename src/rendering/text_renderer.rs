// src/rendering/text_renderer.rs
// Text rendering logic extracted from ui/line_renderer.rs

use egui::{Color32, FontId, FontFamily, Pos2, Align2};
use crate::models::line::{DisplayLine, LineType};
use crate::syntax::SyntaxHighlighter;
use crate::theme::JetBrainsTheme;

/// Text renderer for handling complex text rendering operations
pub struct TextRenderer {
    pub theme: JetBrainsTheme,
    pub syntax_highlighter: SyntaxHighlighter,
}

impl TextRenderer {
    pub fn new(theme: JetBrainsTheme) -> Self {
        Self {
            syntax_highlighter: SyntaxHighlighter::new(),
            theme,
        }
    }

    /// Render line number with enhanced styling
    pub fn render_line_number(&self, ui: &mut egui::Ui, line: &DisplayLine, rect: egui::Rect, baseline_y: f32) {
        if let Some(line_num) = line.original_line_num {
            // Standardized positioning calculation for both panes
            let line_num_pos = Pos2::new(rect.min.x + 30.0, baseline_y);

            ui.painter().text(
                line_num_pos,
                Align2::RIGHT_BOTTOM,
                format!("{}", line_num),
                FontId::new(self.theme.buffer_font_size() * 0.85, FontFamily::Monospace),
                self.theme.line_numbers,
            );
        }
    }

    /// Render code content with JetBrains syntax highlighting
    pub fn render_content(&self, ui: &mut egui::Ui, line: &DisplayLine, rect: egui::Rect, baseline_y: f32) {
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
                    Align2::LEFT_BOTTOM,
                    &line.content,
                    self.theme.buffer_font_id(),
                    final_text_color,
                );
            } else {
                // Render with syntax highlighting token by token
                let mut current_x = rect.min.x + content_start_x;
                let mut current_char_idx = 0;

                for token in tokens {
                    // Apply diff-specific colors over syntax highlighting
                    let token_color = match line.line_type {
                        LineType::Addition => {
                            // For addition lines, blend syntax color with addition foreground
                            use crate::syntax::colors::JetBrainsColors;
                            let syntax_color = JetBrainsColors::get_color_for_token(&token.token_type);
                            self.blend_colors(syntax_color, self.theme.addition_foreground, 0.7)
                        }
                        LineType::Deletion => {
                            // For deletion lines, blend syntax color with deletion foreground
                            use crate::syntax::colors::JetBrainsColors;
                            let syntax_color = JetBrainsColors::get_color_for_token(&token.token_type);
                            self.blend_colors(syntax_color, self.theme.deletion_foreground, 0.7)
                        }
                        LineType::Modification => {
                            // For modification lines, blend syntax color with modification foreground
                            use crate::syntax::colors::JetBrainsColors;
                            let syntax_color = JetBrainsColors::get_color_for_token(&token.token_type);
                            self.blend_colors(syntax_color, self.theme.modification_foreground, 0.7)
                        }
                        _ => {
                            // For context lines, use pure syntax highlighting
                            use crate::syntax::colors::JetBrainsColors;
                            JetBrainsColors::get_color_for_token(&token.token_type)
                        }
                    };

                    // Render any whitespace before this token
                    while current_char_idx < token.start {
                        if let Some(ch) = line.content.chars().nth(current_char_idx) {
                            if ch.is_whitespace() {
                                let space_width = self.theme.char_width() * if ch == '\t' { 4.0 } else { 1.0 };
                                current_x += space_width;
                            }
                        }
                        current_char_idx += 1;
                    }

                    // Render the token
                    ui.painter().text(
                        Pos2::new(current_x, baseline_y),
                        Align2::LEFT_BOTTOM,
                        &token.text,
                        self.theme.buffer_font_id(),
                        token_color,
                    );

                    // Calculate the width of the rendered token
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
                    current_char_idx = token.end;
                }
            }
        }
    }

    /// Render word-level highlights for modifications
    pub fn render_word_highlights(&self, ui: &mut egui::Ui, line: &DisplayLine, rect: egui::Rect, baseline_y: f32) {
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

                    // Render highlight background based on type
                    let highlight_color = match highlight_type {
                        crate::models::line::HighlightType::Insert => {
                            Color32::from_rgba_unmultiplied(40, 167, 69, 80) // Vert pour insertion
                        }
                        crate::models::line::HighlightType::Delete => {
                            Color32::from_rgba_unmultiplied(33, 150, 243, 80) // Bleu pour suppression (au lieu de rouge)
                        }
                    };

                    let highlight_rect = egui::Rect::from_min_size(
                        Pos2::new(
                            rect.min.x + highlight_start_x,
                            rect.min.y + (baseline_y - rect.min.y) - self.theme.buffer_font_size(),
                        ),
                        egui::Vec2::new(highlight_width, self.theme.buffer_font_size() + 2.0),
                    );

                    ui.painter()
                        .rect_filled(highlight_rect, 2.0, highlight_color);
                }
            }
        }
    }

    /// Get text color based on content analysis
    fn get_text_color(&self, _content: &str) -> Color32 {
        self.theme.foreground
    }

    /// Blend two colors with a given weight (from original line_renderer.rs)
    fn blend_colors(&self, syntax_color: Color32, line_color: Color32, syntax_weight: f32) -> Color32 {
        let line_weight = 1.0 - syntax_weight;

        let r = (syntax_color.r() as f32 * syntax_weight + line_color.r() as f32 * line_weight) as u8;
        let g = (syntax_color.g() as f32 * syntax_weight + line_color.g() as f32 * line_weight) as u8;
        let b = (syntax_color.b() as f32 * syntax_weight + line_color.b() as f32 * line_weight) as u8;
        let a = (syntax_color.a() as f32 * syntax_weight + line_color.a() as f32 * line_weight) as u8;

        Color32::from_rgba_unmultiplied(r, g, b, a)
    }

    /// Update theme for the text renderer
    pub fn update_theme(&mut self, theme: JetBrainsTheme) {
        self.theme = theme;
    }
}
