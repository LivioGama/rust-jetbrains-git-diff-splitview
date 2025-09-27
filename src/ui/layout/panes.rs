// src/ui/layout/panes.rs
// Pane rendering logic extracted from layout/mod.rs

use crate::config::LayoutConfig;
use crate::models::diff::MappingSegment;
use crate::models::line::DisplayLine;
use eframe::egui;
use egui::{FontId, ScrollArea, Vec2};

/// Pane rendering functionality for the layout manager
pub struct PaneRenderer {
    config: LayoutConfig,
}

impl PaneRenderer {
    pub fn new(config: LayoutConfig) -> Self {
        Self { config }
    }

    /// Render the left pane (original file)
    pub fn render_left_pane(
        &self,
        ui: &mut egui::Ui,
        old_lines: &[DisplayLine],
        scroll_sync: &mut crate::sync::ScrollSync,
        theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        pane_width: f32,
        total_height: f32,
        mapping_segments: &[MappingSegment],
        imara_analysis: &crate::diff::imara::ImaraDiffAnalysis,
    ) {
        ui.allocate_ui_with_layout(
            Vec2::new(pane_width, total_height),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                ui.style_mut().spacing.indent = 0.0;
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
                    imara_analysis,
                );
            },
        );
    }

    /// Render the right pane (modified file)
    pub fn render_right_pane(
        &self,
        ui: &mut egui::Ui,
        new_lines: &[DisplayLine],
        scroll_sync: &mut crate::sync::ScrollSync,
        theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        pane_width: f32,
        total_height: f32,
        mapping_segments: &[MappingSegment],
        imara_analysis: &crate::diff::imara::ImaraDiffAnalysis,
    ) {
        ui.allocate_ui_with_layout(
            Vec2::new(pane_width, total_height),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                ui.style_mut().spacing.indent = 0.0;
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
                    imara_analysis,
                );
            },
        );
    }

    /// Render a pane header
    pub fn render_pane_header(
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
                        theme.ui_font_size() * 1.1,
                        egui::FontFamily::Proportional,
                    ))
                    .color(theme.foreground),
            );
        });
    }

    /// Render scrollable content area
    pub fn render_scrollable_content(
        &self,
        ui: &mut egui::Ui,
        lines: &[DisplayLine],
        scroll_sync: &mut crate::sync::ScrollSync,
        _theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        is_left: bool,
        scroll_id: &str,
        scroll_memory_key: &str,
        rects_memory_key: &str,
        _mapping_segments: &[MappingSegment],
        imara_analysis: &crate::diff::imara::ImaraDiffAnalysis,
    ) {
        let available_height = ui.available_height();

        // Create scroll area for this pane
        let scroll_area = ScrollArea::vertical()
            .id_salt(scroll_id)
            .auto_shrink([false, false])
            .max_height(available_height)
            .stick_to_bottom(false);

        let scroll_area_response = scroll_area.show(ui, |ui| {
            // Store line rectangles for connector calculations
            let mut line_rects = Vec::new();
            let mut crushed_rects = Vec::new();
            let mut crushed_line_rects = Vec::new();

            ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;

            // Create a combined rendering plan that includes both normal lines and crushed blocks in proper sequence
            let mut line_idx = 0;

            // Process each imara block to determine where crushed blocks should be inserted
            for imara_block in &imara_analysis.blocks {
                // Render normal lines up to this block's position
                let target_line_end = if is_left {
                    if imara_block.is_pure_insertion() {
                        // For pure insertions in left pane, render lines up to the insertion point
                        imara_block.right_range.start.saturating_sub(
                            imara_analysis
                                .blocks
                                .iter()
                                .filter(|b| {
                                    b.right_range.end <= imara_block.right_range.start
                                        && b.is_pure_insertion()
                                })
                                .map(|b| b.right_range.len())
                                .sum::<usize>(),
                        )
                    } else {
                        imara_block.left_range.end
                    }
                } else {
                    if imara_block.is_pure_deletion() {
                        // For pure deletions in right pane, render lines up to the deletion point
                        imara_block.left_range.start.saturating_sub(
                            imara_analysis
                                .blocks
                                .iter()
                                .filter(|b| {
                                    b.left_range.end <= imara_block.left_range.start
                                        && b.is_pure_deletion()
                                })
                                .map(|b| b.left_range.len())
                                .sum::<usize>(),
                        )
                    } else {
                        imara_block.right_range.end
                    }
                };

                // Render normal lines up to the target position
                while line_idx < target_line_end.min(lines.len()) {
                    let line = &lines[line_idx];
                    let line_rect = line_renderer.render_line(ui, line, line_idx, is_left);
                    line_rects.push(line_rect);

                    // Track crushed lines for pure insertion handling
                    if line.content.starts_with("...") {
                        crushed_rects.push((line_idx, line_rect, line.content.clone()));
                    }
                    line_idx += 1;
                }

                // Insert crushed block if needed
                if (is_left
                    && imara_block.is_pure_insertion()
                    && !imara_block.right_range.is_empty())
                    || (!is_left
                        && imara_block.is_pure_deletion()
                        && !imara_block.left_range.is_empty())
                {
                    // Allocate space for the crushed block within the UI layout
                    let (crushed_rect, _response) = ui.allocate_exact_size(
                        egui::Vec2::new(ui.available_width(), 2.0),
                        egui::Sense::hover(),
                    );

                    let adjusted_rect = crushed_rect.translate(Vec2::new(0.0, 1.0));

                    let (color, block_type, block_idx) = if is_left
                        && imara_block.is_pure_insertion()
                    {
                        let base = theme.addition_background;
                        (
                            egui::Color32::from_rgba_unmultiplied(
                                base.r(),
                                base.g(),
                                base.b(),
                                base.a(),
                            ),
                            "addition",
                            imara_block.right_range.start,
                        )
                    } else {
                        let base = theme.deletion_background;
                        (
                            egui::Color32::from_rgba_unmultiplied(
                                base.r(),
                                base.g(),
                                base.b(),
                                base.a(),
                            ),
                            "deletion",
                            imara_block.left_range.start,
                        )
                    };

                    ui.painter().rect_filled(adjusted_rect, 0.0, color);

                    // Store crushed line info for connectors
                    crushed_line_rects.push((block_idx, adjusted_rect, block_type.to_string()));
                }
            }

            // Render any remaining normal lines
            while line_idx < lines.len() {
                let line = &lines[line_idx];
                let line_rect = line_renderer.render_line(ui, line, line_idx, is_left);
                line_rects.push(line_rect);

                // Track crushed lines for pure insertion handling
                if line.content.starts_with("...") {
                    crushed_rects.push((line_idx, line_rect, line.content.clone()));
                }
                line_idx += 1;
            }

            // Store crushed line positions for connectors
            let crushed_memory_key = if is_left {
                "left_crushed_rects"
            } else {
                "right_crushed_rects"
            };
            ui.ctx().memory_mut(|mem| {
                mem.data
                    .insert_persisted(crushed_memory_key.to_string().into(), crushed_line_rects);
            });

            // Store rectangles in memory for connector rendering
            ui.ctx().memory_mut(|mem| {
                mem.data
                    .insert_persisted(rects_memory_key.to_string().into(), line_rects);
                // Note: crushed_line_rects are already stored above with the correct keys
            });
        });

        // Update scroll synchronization
        let current_scroll_offset = scroll_area_response.state.offset.y;
        if is_left {
            scroll_sync.set_left_scroll(current_scroll_offset);
        } else {
            scroll_sync.set_right_scroll(current_scroll_offset);
        }

        // Store scroll position in memory
        ui.ctx().memory_mut(|mem| {
            mem.data
                .insert_persisted(scroll_memory_key.to_string().into(), current_scroll_offset);
        });
    }
}
