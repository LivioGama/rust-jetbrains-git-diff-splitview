// Line rendering logic for the diff viewer - GPUI Native Implementation

use crate::models::line::{DisplayLine, LineType};
use crate::rendering::TextRenderer;
use crate::theme::JetBrainsTheme;
use gpui::*;

#[derive(Clone)]
pub struct LineRenderer {
    text_renderer: TextRenderer,
}

impl LineRenderer {
    pub fn new() -> Self {
        Self {
            text_renderer: TextRenderer::new(),
        }
    }

    /// Render a line using GPUI elements
    pub fn render_line_gpui(
        &self,
        line: &DisplayLine,
        line_idx: usize,
        _is_left: bool,
        theme: &JetBrainsTheme,
    ) -> impl IntoElement {
        let line_height = theme.line_height();
        let bg_color = theme.get_line_background(&line.line_type);
        let line_number = line_idx + 1;

        // Line indicator color
        let indicator_color = if line.line_type != LineType::Context {
            match line.line_type {
                LineType::Addition => theme.addition_background,
                LineType::Deletion => theme.deletion_background,
                LineType::Modification => theme.modification_background,
                _ => hsla(0.0, 0.0, 0.0, 0.0),
            }
        } else {
            hsla(0.0, 0.0, 0.0, 0.0)
        };

        let _content = line.content.clone();
        let _line_type = line.line_type.clone();

        div()
            .flex()
            .flex_row()
            .min_h(px(line_height))
            .bg(bg_color)
            .border_l_2()
            .border_color(indicator_color)
            .children([
                // Line number (right-aligned, fixed width)
                div()
                    .flex_none()
                    .w(px(50.0))
                    .px(px(8.0))
                    .py(px(2.0))
                    .text_sm()
                    .text_color(theme.line_numbers)
                    .font_family(".SF Mono, Consolas, 'Liberation Mono', Menlo, monospace")
                    .text_align(gpui::TextAlign::Right)
                    .child(format!("{}", line_number))
                    .into_any_element(),
                // Content area with syntax highlighting
                div()
                    .flex_1()
                    .px(px(8.0))
                    .py(px(2.0))
                    .text_sm()
                    .font_family(".SF Mono, Consolas, 'Liberation Mono', Menlo, monospace")
                    .child(
                        self.text_renderer
                            .render_syntax_highlighted_line_gpui(line, theme),
                    )
                    .into_any_element(),
            ])
    }
}
