// diffsplit/src/rendering/mod.rs
// Rendering module for UI rendering logic

pub mod highlight_renderer;

pub mod text_renderer;

pub use highlight_renderer::*;

pub use text_renderer::*;

use crate::syntax::SyntaxHighlighter;
use crate::theme::JetBrainsTheme;
use egui::Color32;

// Type aliases for compatibility
#[allow(dead_code)]
type Color = Color32;

/// Line renderer for rendering individual lines (delegated to ui/LineRenderer)
#[allow(dead_code)]
pub struct LineRenderer {
    theme: JetBrainsTheme,
    syntax_highlighter: SyntaxHighlighter,
}

impl LineRenderer {}

// Connector renderer for rendering connection lines
#[allow(dead_code)]
pub struct ConnectorRenderer {
    theme: JetBrainsTheme,
}

impl ConnectorRenderer {}

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
