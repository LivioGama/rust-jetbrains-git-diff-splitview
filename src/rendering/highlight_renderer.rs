// src/rendering/highlight_renderer.rs
// Highlight renderer extracted from rendering/mod.rs

use crate::theme::JetBrainsTheme;
use egui::{Color32, Rect};

/// Highlight renderer for JetBrains-style highlights
pub struct HighlightRenderer {
    theme: JetBrainsTheme,
}

impl HighlightRenderer {
    pub fn new(theme: JetBrainsTheme) -> Self {
        Self { theme }
    }

    // Step 2 — Draw highlights based on line type
    pub fn draw_highlight(
        &self,
        ui: &mut egui::Ui,
        rect: Rect,
        line_type: &crate::models::line::LineType,
    ) {
        // Use theme-based highlight color based on line type
        let highlight_color = match line_type {
            crate::models::line::LineType::Addition => {
                // Green background for additions
                self.theme.addition_background
            }
            crate::models::line::LineType::Deletion => {
                // Gray/red background for deletions
                self.theme.deletion_background
            }
            crate::models::line::LineType::Context => {
                // Use modification background for context lines with changes
                self.theme.modification_background
            }
            crate::models::line::LineType::Modification => {
                // Blue background for modifications
                self.theme.modification_background
            }
        };

        if highlight_color != Color32::TRANSPARENT {
            ui.painter().rect_filled(rect, 0.0, highlight_color);
        }
    }
}
