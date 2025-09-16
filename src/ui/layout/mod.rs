// diffsplit/src/ui/layout/mod.rs
// UI Layout module for organizing the main application interface

use eframe::egui;
use egui::{FontId, ScrollArea, Vec2};

use crate::config::LayoutConfig;

use crate::sync::*;

/// Layout manager for the diff viewer
pub struct LayoutManager {
    config: LayoutConfig,
}

impl LayoutManager {
    pub fn new(config: LayoutConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(LayoutConfig::default())
    }

    /// Render the complete application layout
    pub fn render_layout(
        &self,
        ui: &mut egui::Ui,
        old_lines: &[crate::models::DisplayLine],
        new_lines: &[crate::models::DisplayLine],
        scroll_sync: &mut crate::sync::ScrollSync,
        theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        connector_renderer: &mut crate::ui::ConnectorRenderer,
        mapping_segments: &[crate::models::MappingSegment],
    ) {
        let total_height = ui.available_height();
        let total_width = ui.available_width();
        let pane_width = (total_width - self.config.connector_column_width) / 2.0;

        // Create a horizontal layout with explicit height allocation
        ui.allocate_ui_with_layout(
            Vec2::new(total_width, total_height),
            egui::Layout::left_to_right(egui::Align::TOP),
            |ui| {
                // Left pane (original)
                self.render_left_pane(
                    ui,
                    old_lines,
                    scroll_sync,
                    theme,
                    line_renderer,
                    pane_width,
                    total_height,
                    mapping_segments,
                );

                // Middle gutter for connections
                self.render_connector_gutter(
                    ui,
                    old_lines,
                    new_lines,
                    connector_renderer,
                    total_height,
                );

                // Right pane (modified)
                self.render_right_pane(
                    ui,
                    new_lines,
                    scroll_sync,
                    theme,
                    line_renderer,
                    pane_width,
                    total_height,
                    mapping_segments,
                );
            },
        );
    }

    /// Render the left pane (original file)
    fn render_left_pane(
        &self,
        ui: &mut egui::Ui,
        old_lines: &[crate::models::DisplayLine],
        scroll_sync: &mut crate::sync::ScrollSync,
        theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        pane_width: f32,
        total_height: f32,
        mapping_segments: &[crate::models::MappingSegment],
    ) {
        ui.allocate_ui_with_layout(
            Vec2::new(pane_width, total_height),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                // Header
                self.render_pane_header(ui, "Original", theme);

                ui.separator();

                // Content area with scrolling
                self.render_scrollable_content(
                    ui,
                    old_lines,
                    scroll_sync,
                    theme,
                    line_renderer,
                    true,
                    "diff_left_scroll",
                    "left_scroll",
                    "left_rects",
                    mapping_segments,
                );
            },
        );
    }

    /// Render the right pane (modified file)
    fn render_right_pane(
        &self,
        ui: &mut egui::Ui,
        new_lines: &[crate::models::DisplayLine],
        scroll_sync: &mut crate::sync::ScrollSync,
        theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        pane_width: f32,
        total_height: f32,
        mapping_segments: &[crate::models::MappingSegment],
    ) {
        ui.allocate_ui_with_layout(
            Vec2::new(pane_width, total_height),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                // Header
                self.render_pane_header(ui, "Modified", theme);

                ui.separator();

                // Content area with scrolling
                self.render_scrollable_content(
                    ui,
                    new_lines,
                    scroll_sync,
                    theme,
                    line_renderer,
                    false,
                    "diff_right_scroll",
                    "right_scroll",
                    "right_rects",
                    mapping_segments,
                );
            },
        );
    }

    /// Render the connector gutter
    fn render_connector_gutter(
        &self,
        ui: &mut egui::Ui,
        old_lines: &[crate::models::DisplayLine],
        new_lines: &[crate::models::DisplayLine],
        connector_renderer: &mut crate::ui::ConnectorRenderer,
        total_height: f32,
    ) {
        ui.allocate_ui_with_layout(
            Vec2::new(self.config.connector_column_width, total_height),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                // Draw connection lines
                connector_renderer.draw_connection_lines(ui, old_lines, new_lines);
            },
        );
    }

    /// Render a pane header
    fn render_pane_header(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        theme: &crate::theme::JetBrainsTheme,
    ) {
        ui.horizontal(|ui| {
            ui.add_space(self.config.pane_padding);
            ui.label(
                egui::RichText::new(title)
                    .font(FontId::new(
                        theme.font_size * 1.1,
                        egui::FontFamily::Proportional,
                    ))
                    .color(theme.foreground),
            );
        });
    }

    /// Render scrollable content area
    fn render_scrollable_content(
        &self,
        ui: &mut egui::Ui,
        lines: &[crate::models::DisplayLine],
        scroll_sync: &mut crate::sync::ScrollSync,
        theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        is_left: bool,
        scroll_id: &str,
        scroll_memory_key: &str,
        rects_memory_key: &str,
        mapping_segments: &[crate::models::MappingSegment],
    ) {
        let available_height = ui.available_height();

        // Get current scroll position from memory
        let current_scroll = ui.ctx().memory_mut(|mem| {
            mem.data
                .get_persisted(scroll_memory_key.to_string().into())
                .unwrap_or(0.0)
        });

        // Calculate synchronized position
        let scroll_offset = if (is_left && scroll_sync.master_pane() == MasterPane::Right)
            || (!is_left && scroll_sync.master_pane() == MasterPane::Left)
        {
            if is_left {
                scroll_sync.left_scroll_offset()
            } else {
                scroll_sync.right_scroll_offset()
            }
        } else {
            current_scroll
        };

        let scroll_output = ScrollArea::vertical()
            .id_source(scroll_id)
            .auto_shrink([false, false])
            .max_height(available_height)
            .min_scrolled_height(available_height)
            .scroll_offset(Vec2::new(0.0, scroll_offset))
            .show(ui, |ui| {
                let mut line_rects = Vec::new();

                for (line_idx, line) in lines.iter().enumerate() {
                    let rect = line_renderer.render_line(ui, line, line_idx, is_left);
                    line_rects.push(rect);
                }

                ui.ctx().memory_mut(|mem| {
                    mem.data
                        .insert_persisted(rects_memory_key.to_string().into(), line_rects);
                });
            });

        // Update scroll sync
        if is_left {
            scroll_sync.set_left_scroll(scroll_output.state.offset.y);
            // Synchronize right pane
            scroll_sync
                .synchronize_scrolls(|y| crate::sync::map_left_to_right(y, mapping_segments));
        } else {
            scroll_sync.set_right_scroll(scroll_output.state.offset.y);
            // Synchronize left pane
            scroll_sync
                .synchronize_scrolls(|y| crate::sync::map_right_to_left(y, mapping_segments));
        }

        // Store scroll position
        ui.ctx().memory_mut(|mem| {
            mem.data.insert_persisted(
                scroll_memory_key.to_string().into(),
                scroll_output.state.offset.y,
            );
        });
    }

    /// Update layout configuration
    pub fn update_config(&mut self, config: LayoutConfig) {
        self.config = config;
    }

    /// Get current layout configuration
    pub fn get_config(&self) -> &LayoutConfig {
        &self.config
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::with_default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LayoutConfig;

    #[test]
    fn test_layout_manager_creation() {
        let manager = LayoutManager::new(LayoutConfig::default());
        assert_eq!(manager.get_config().connector_column_width, 45.0);
    }

    #[test]
    fn test_layout_manager_default() {
        let manager = LayoutManager::default();
        assert_eq!(manager.get_config().pane_padding, 10.0);
    }
}
