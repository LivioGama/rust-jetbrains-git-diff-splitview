// Pane rendering logic - GPUI Native Implementation
use super::connectors::ConnectorState;
use crate::config::LayoutConfig;
use crate::models::diff::MappingSegment;
use crate::models::line::DisplayLine;
use crate::sync::{map_left_to_right, map_right_to_left};
use gpui::ScrollDelta;
use gpui::*;
use std::sync::Arc;

/// Pane rendering functionality for the layout manager
pub struct PaneRenderer {
    config: LayoutConfig,
}

impl PaneRenderer {
    pub fn new(config: LayoutConfig) -> Self {
        Self { config }
    }

    /// Render a pane using GPUI elements
    pub fn render_pane_gpui(
        &self,
        title: &str,
        lines: &[DisplayLine],
        scroll_sync: &mut crate::sync::ScrollSync,
        theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        _pane_width: Pixels,
        mapping_segments: &[MappingSegment],
        _imara_analysis: &crate::diff::imara::ImaraDiffAnalysis,
        is_left: bool,
        _left_scroll: f32,
        _right_scroll: f32,
        connector_state: &mut ConnectorState,
    ) -> impl IntoElement {
        let title = title.to_string();

        div()
            .flex()
            .flex_col()
            .h_full() // Take full height of parent
            .w_full() // Take full width of parent
            .children(vec![
                // Header
                div()
                    .flex()
                    .px(px(self.config.pane_padding))
                    .py(px(4.0))
                    .child(div().text_sm().text_color(theme.foreground).child(title))
                    .into_any_element(),
                // Separator
                div().h(px(1.0)).bg(rgb(0x3c3c3c)).into_any_element(),
                // Content area with scrolling container - takes remaining space
                div()
                    .flex_1()
                    .h_0() // Let flex determine height but start from 0
                    .min_h_0() // Allow shrinking if needed
                    .overflow_hidden() // Ensure proper containment
                    .child(self.render_scrollable_content_gpui(
                        lines,
                        theme,
                        line_renderer,
                        is_left,
                        scroll_sync,
                        connector_state,
                        mapping_segments,
                    ))
                    .into_any_element(),
            ])
    }

    /// Render scrollable content area using GPUI elements with synchronization
    pub fn render_scrollable_content_gpui(
        &self,
        lines: &[DisplayLine],
        theme: &crate::theme::JetBrainsTheme,
        line_renderer: &mut crate::ui::LineRenderer,
        is_left: bool,
        scroll_sync: &mut crate::sync::ScrollSync,
        connector_state: &mut ConnectorState,
        mapping_segments: &[MappingSegment],
    ) -> impl IntoElement {
        let scroll_id = if is_left {
            "left_pane_scroll"
        } else {
            "right_pane_scroll"
        };
        let pane_label = if is_left { "left" } else { "right" };
        let current_scroll_offset = if is_left {
            scroll_sync.get_left_scroll()
        } else {
            scroll_sync.get_right_scroll()
        };
        if is_left {
            connector_state.left_scroll_offset = current_scroll_offset;
        } else {
            connector_state.right_scroll_offset = current_scroll_offset;
        }
        eprintln!(
            "[DEBUG] render_scrollable_content_gpui pane={} scroll_offset={:.1}",
            pane_label, current_scroll_offset
        );
        let theme_clone = theme.clone();
        let is_left_clone = is_left;
        let scroll_sync_ptr = scroll_sync as *mut crate::sync::ScrollSync;

        let line_height = theme.line_height();
        let content_height = lines.len() as f32 * line_height;
        let max_scroll_offset = (content_height - line_height).max(0.0);

        let mapping_segments_arc = Arc::new(mapping_segments.to_vec());
        let has_mapping_segments = !mapping_segments_arc.is_empty();
        let mapping_segments_for_closure = Arc::clone(&mapping_segments_arc);

        div()
            .id(scroll_id)
            .h_full()
            .w_full()
            .bg(theme.background)
            .overflow_y_scroll()
            .on_scroll_wheel(move |event, _phase, _cx| {
                if scroll_sync_ptr.is_null() {
                    return;
                }
                let delta = match event.delta {
                    ScrollDelta::Pixels(delta) => delta.y.0,
                    ScrollDelta::Lines(delta) => delta.y * line_height,
                };
                if delta == 0.0 {
                    return;
                }
                let scroll_sync_ref = unsafe { &mut *scroll_sync_ptr };
                if is_left_clone {
                    let current = scroll_sync_ref.get_left_scroll();
                    let next = (current + delta).clamp(0.0, max_scroll_offset);
                    if scroll_sync_ref.update_left_scroll(next) {
                        if has_mapping_segments {
                            let segments = mapping_segments_for_closure.clone();
                            scroll_sync_ref.synchronize_scrolls(move |y| {
                                map_left_to_right(y, segments.as_slice())
                            });
                        } else {
                            scroll_sync_ref.synchronize_scrolls(|y| y);
                        }
                    }
                } else {
                    let current = scroll_sync_ref.get_right_scroll();
                    let next = (current + delta).clamp(0.0, max_scroll_offset);
                    if scroll_sync_ref.update_right_scroll(next) {
                        if has_mapping_segments {
                            let segments = mapping_segments_for_closure.clone();
                            scroll_sync_ref.synchronize_scrolls(move |y| {
                                map_right_to_left(y, segments.as_slice())
                            });
                        } else {
                            scroll_sync_ref.synchronize_scrolls(|y| y);
                        }
                    }
                }
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .min_w_0()
                    .h(px(content_height))
                    .relative()
                    .child({
                        self.calculate_and_store_rectangles(lines, theme, is_left, connector_state);
                        let stored_rect_count = if is_left {
                            connector_state.left_rects.len()
                        } else {
                            connector_state.right_rects.len()
                        };
                        eprintln!(
                            "[DEBUG] calculate_and_store_rectangles pane={} stored_rectangles={}",
                            pane_label, stored_rect_count
                        );

                        div().flex().flex_col().w_full().children(
                            lines
                                .iter()
                                .enumerate()
                                .map(|(idx, line)| {
                                    line_renderer
                                        .render_line_gpui(line, idx, is_left_clone, &theme_clone)
                                        .into_any_element()
                                })
                                .collect::<Vec<_>>(),
                        )
                    }),
            )
    }

    /// Calculate line rectangle positions and store them in ConnectorState for connector rendering
    fn calculate_and_store_rectangles(
        &self,
        lines: &[DisplayLine],
        theme: &crate::theme::JetBrainsTheme,
        is_left: bool,
        connector_state: &mut ConnectorState,
    ) {
        let line_height = theme.line_height();
        let vertical_padding = 4.0;
        let line_stride = line_height + vertical_padding;
        let pane_label = if is_left { "left" } else { "right" };
        eprintln!(
            "[DEBUG] calculate_and_store_rectangles pane={} line_height={:.1} line_stride={:.1}",
            pane_label, line_height, line_stride
        );
        let pane_padding = self.config.pane_padding;

        // Calculate rectangles for each line
        let mut rectangles = Vec::new();
        let mut crushed_rectangles = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            // Y position should be relative to content area (scroll container)
            // Connectors align with content, not absolute page position
            let y_position = idx as f32 * line_stride;
            let line_width = 400.0; // Default line width - will be adjusted by layout

            // Rectangle position relative to content area for connector alignment
            let rect = gpui::Bounds::new(
                gpui::point(px(pane_padding), px(y_position)),
                gpui::size(px(line_width), px(line_stride)),
            );

            rectangles.push(rect);

            // Handle crushed lines for large pure insertions/deletions
            // This is a simplified version - in practice, crushed lines would be identified
            // by analyzing the diff blocks and determining which lines to "crush"
            if self.should_create_crushed_line(line, idx) {
                let crushed_rect = gpui::Bounds::new(
                    gpui::point(px(pane_padding), px(y_position)),
                    gpui::size(px(20.0), px(2.0)), // Small crushed indicator
                );
                crushed_rectangles.push((idx, crushed_rect, line.content.clone()));
            }
        }

        // Store rectangles in connector state
        if is_left {
            connector_state.store_left_rects(rectangles.clone());
            connector_state.store_left_crushed_rects(crushed_rectangles);
        } else {
            connector_state.store_right_rects(rectangles.clone());
            connector_state.store_right_crushed_rects(crushed_rectangles);
        }
    }

    /// Determine if a line should have a crushed representation
    /// This is a placeholder implementation - real logic would analyze diff blocks
    fn should_create_crushed_line(&self, _line: &DisplayLine, _idx: usize) -> bool {
        // For now, don't create crushed lines - this would be implemented
        // based on imara diff analysis to identify large pure insertions/deletions
        false
    }
}
