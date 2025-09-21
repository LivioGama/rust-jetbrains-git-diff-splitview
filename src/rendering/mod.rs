// diffsplit/src/rendering/mod.rs
// Rendering module for UI rendering logic

pub mod highlight_renderer;
pub mod jetbrains_renderer;
pub mod render_context;
pub mod text_renderer;

pub use highlight_renderer::*;
pub use jetbrains_renderer::*;
pub use render_context::*;
pub use text_renderer::*;

use crate::models::*;
use crate::syntax::SyntaxHighlighter;
use crate::theme::JetBrainsTheme;
use egui::{Color32, Pos2, Rect, Stroke};

// Type aliases for compatibility
type Color = Color32;

/// Line renderer for rendering individual lines (delegated to ui/LineRenderer)
pub struct LineRenderer {
    theme: JetBrainsTheme,
    syntax_highlighter: SyntaxHighlighter,
}

impl LineRenderer {
    pub fn new(theme: JetBrainsTheme) -> Self {
        Self {
            syntax_highlighter: SyntaxHighlighter::new(),
            theme,
        }
    }

    pub fn render_line(
        &self,
        ui: &mut egui::Ui,
        line: &DisplayLine,
        line_idx: usize,
        is_left: bool,
    ) -> Rect {
        // Delegate to ui::LineRenderer for actual implementation
        let ui_line_renderer = crate::ui::LineRenderer::new(self.theme.clone());
        ui_line_renderer.render_line(ui, line, line_idx, is_left)
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
        _anchors: &[AnchorPoint],
        _left_rects: &[Rect],
        _right_rects: &[Rect],
        _line_height: f32,
        _top_y: f32,
    ) {
        // Simplified connector rendering - main logic moved to ui/layout modules
        eprintln!("🔗 ConnectorRenderer rendering {} change blocks", change_blocks.len());
    }

    pub fn update_theme(&mut self, theme: JetBrainsTheme) {
        self.theme = theme;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_context_creation() {
        let theme = JetBrainsTheme::dark_theme();
        let _context = RenderContext::new(theme, 800.0, 600.0);
        // Test passes if context is created successfully
    }

    #[test]
    fn test_line_renderer_creation() {
        let theme = JetBrainsTheme::dark_theme();
        let _renderer = LineRenderer::new(theme);
        // Test passes if renderer is created successfully
    }

    #[test]
    fn test_connector_renderer_creation() {
        let theme = JetBrainsTheme::dark_theme();
        let _renderer = ConnectorRenderer::new(theme);
        // Test passes if renderer is created successfully
    }
}