// src/rendering/text_renderer.rs
// Text rendering logic for GPUI - Native Implementation

use crate::models::line::{DisplayLine, LineType};
use crate::syntax::SyntaxHighlighter;
use crate::theme::JetBrainsTheme;
use gpui::*;

/// Text renderer for handling complex text rendering operations
#[derive(Clone)]
pub struct TextRenderer {
    syntax_highlighter: SyntaxHighlighter,
}

impl TextRenderer {
    pub fn new() -> Self {
        Self {
            syntax_highlighter: SyntaxHighlighter::new(),
        }
    }

    /// Render syntax-highlighted line content using GPUI
    pub fn render_syntax_highlighted_line_gpui(
        &self,
        line: &DisplayLine,
        theme: &JetBrainsTheme,
    ) -> impl gpui::IntoElement {
        let tokens = self.syntax_highlighter.highlight_line(&line.content);

        if tokens.is_empty() {
            // No tokens, render as plain text with proper font styling
            div()
                .text_sm()
                .text_color(self.get_text_color_gpui(&line.content, theme))
                .font_family(".SF Mono, Consolas, 'Liberation Mono', Menlo, monospace")
                .child(line.content.clone())
        } else {
            // Render tokens with syntax highlighting and preserved whitespace
            div()
                .flex()
                .flex_row()
                .font_family(".SF Mono, Consolas, 'Liberation Mono', Menlo, monospace")
                .text_sm()
                .children(
                    tokens
                        .iter()
                        .enumerate()
                        .map(|(i, token)| {
                            let token_color = self.get_token_color_gpui(
                                &token.token_type,
                                &line.line_type,
                                theme,
                            );

                            // Check if we need to add whitespace before this token
                            let needs_space = if i > 0 {
                                let prev_token = &tokens[i - 1];
                                // Add space if there's a gap between tokens in the original text
                                token.start > prev_token.end
                            } else {
                                false
                            };

                            // Create elements: optional space + token
                            let mut elements = Vec::new();

                            if needs_space {
                                let prev_token = &tokens[i - 1];
                                // Add space - use the actual whitespace from original text
                                let space_text =
                                    line.content[prev_token.end..token.start].to_string();
                                elements.push(div().child(space_text).into_any_element());
                            }

                            elements.push(
                                div()
                                    .text_color(token_color)
                                    .child(token.text.clone())
                                    .into_any_element(),
                            );

                            div()
                                .flex()
                                .flex_row()
                                .children(elements)
                                .into_any_element()
                        })
                        .collect::<Vec<_>>(),
                )
        }
    }

    /// Get text color for GPUI rendering with enhanced syntax detection
    fn get_text_color_gpui(&self, content: &str, theme: &JetBrainsTheme) -> gpui::Hsla {
        let trimmed = content.trim();

        if trimmed.starts_with("//") || trimmed.starts_with("#") {
            theme.code_comment
        } else if trimmed.contains("fn ")
            || trimmed.contains("let ")
            || trimmed.contains("const ")
            || trimmed.contains("struct ")
            || trimmed.contains("enum ")
            || trimmed.contains("impl ")
            || trimmed.contains("pub ")
            || trimmed.contains("use ")
            || trimmed.contains("import ")
            || trimmed.contains("export ")
            || trimmed.contains("function ")
        {
            theme.code_keyword
        } else if trimmed.contains("\"") || trimmed.contains("'") {
            theme.code_string
        } else {
            theme.code_foreground
        }
    }

    /// Get color for a specific token with line type blending
    fn get_token_color_gpui(
        &self,
        token_type: &crate::syntax::token_types::TokenType,
        line_type: &LineType,
        theme: &JetBrainsTheme,
    ) -> gpui::Hsla {
        use crate::syntax::colors::JetBrainsColors;

        let base_color = JetBrainsColors::get_color_for_token(token_type);

        // Blend with line type colors for diff highlighting
        match line_type {
            LineType::Addition => {
                self.blend_colors_gpui(base_color, theme.addition_foreground, 0.7)
            }
            LineType::Deletion => {
                self.blend_colors_gpui(base_color, theme.deletion_foreground, 0.7)
            }
            LineType::Modification => {
                self.blend_colors_gpui(base_color, theme.modification_foreground, 0.7)
            }
            LineType::Context => base_color,
        }
    }

    /// Blend two GPUI colors with a given weight
    fn blend_colors_gpui(
        &self,
        syntax_color: gpui::Hsla,
        line_color: gpui::Hsla,
        syntax_weight: f32,
    ) -> gpui::Hsla {
        let line_weight = 1.0 - syntax_weight;

        // Convert to linear RGB for blending
        let syntax_rgba = syntax_color.to_rgb();
        let line_rgba = line_color.to_rgb();

        let r = syntax_rgba.r * syntax_weight + line_rgba.r * line_weight;
        let g = syntax_rgba.g * syntax_weight + line_rgba.g * line_weight;
        let b = syntax_rgba.b * syntax_weight + line_rgba.b * line_weight;
        let a = syntax_rgba.a * syntax_weight + line_rgba.a * line_weight;

        // Create RGBA color from blended components
        gpui::Rgba { r, g, b, a }.into()
    }
}
