// src/rendering/jetbrains_renderer.rs
// JetBrains renderer extracted from rendering/mod.rs

use crate::models::*;
use crate::theme::JetBrainsTheme;
use super::{RenderContext, LineRenderer, HighlightRenderer, ConnectorRenderer};

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

    // Simplified render_diff_view - main logic moved to other modules
    pub fn render_diff_view(
        &mut self,
        ui: &mut egui::Ui,
        left_lines: &[DisplayLine],
        right_lines: &[DisplayLine],
        change_blocks: &[ChangeBlock],
        _anchors: &[AnchorPoint],
        _mapping_segments: &[MappingSegment],
    ) {
        // Delegate to specialized renderers
        eprintln!(
            "🎨 JetBrainsRenderer rendering {} left lines and {} right lines",
            left_lines.len(),
            right_lines.len()
        );

        // Basic layout - actual rendering delegated to LayoutManager
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                for (idx, line) in left_lines.iter().enumerate() {
                    self.line_renderer.render_line(ui, line, idx, true);
                }
            });

            ui.vertical(|ui| {
                for (idx, line) in right_lines.iter().enumerate() {
                    self.line_renderer.render_line(ui, line, idx, false);
                }
            });
        });
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
