// diffsplit/src/app/mod.rs
use eframe::egui;
use egui::{Color32, FontId, Pos2, Rect, ScrollArea, Vec2};

use crate::models::*;
use crate::sync::*;
use crate::theme::*;
use crate::ui::*;

pub struct DiffViewerApp {
    pub old_lines: Vec<DisplayLine>,
    pub new_lines: Vec<DisplayLine>,
    pub change_blocks: Vec<ChangeBlock>,
    pub anchors: Vec<AnchorPoint>,
    pub mapping_segments: Vec<MappingSegment>,
    pub connector_curves: Vec<ConnectorCurve>,
    pub current_file: usize,
    pub scroll_sync: ScrollSync,
    pub theme: JetBrainsTheme,
    pub line_renderer: LineRenderer,
    pub connector_renderer: ConnectorRenderer,
}

impl DiffViewerApp {
    pub fn new(
        old_lines: Vec<DisplayLine>,
        new_lines: Vec<DisplayLine>,
        change_blocks: Vec<ChangeBlock>,
        anchors: Vec<AnchorPoint>,
        mapping_segments: Vec<MappingSegment>,
    ) -> Self {
        let theme = JetBrainsTheme::dark_theme();
        let line_height = 18.0;
        let viewport_height = 1000.0;

        Self {
            old_lines,
            new_lines,
            change_blocks,
            anchors,
            mapping_segments,
            connector_curves: Vec::new(),
            current_file: 0,
            scroll_sync: ScrollSync::new(line_height, viewport_height),
            theme: theme.clone(),
            line_renderer: LineRenderer::new(theme.clone()),
            connector_renderer: ConnectorRenderer::new(theme),
        }
    }

    fn handle_keyboard_navigation(&mut self, ctx: &egui::Context) {
        // Handle keyboard navigation for diff blocks
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            self.navigate_to_next_diff_block();
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            self.navigate_to_previous_diff_block();
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            self.navigate_to_next_connector();
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            self.navigate_to_previous_connector();
        } else if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.apply_current_hunk();
        } else if ctx.input(|i| i.key_pressed(egui::Key::Backspace)) {
            self.revert_current_hunk();
        } else if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.stage_current_hunk();
        }
    }

    fn navigate_to_next_diff_block(&mut self) {
        // Implementation for navigating to next diff block
    }

    fn navigate_to_previous_diff_block(&mut self) {
        // Implementation for navigating to previous diff block
    }

    fn navigate_to_next_connector(&mut self) {
        // Implementation for navigating to next connector
    }

    fn navigate_to_previous_connector(&mut self) {
        // Implementation for navigating to previous connector
    }

    fn apply_current_hunk(&mut self) {
        // Implementation for applying current hunk
    }

    fn revert_current_hunk(&mut self) {
        // Implementation for reverting current hunk
    }

    fn stage_current_hunk(&mut self) {
        // Implementation for staging current hunk
    }

    fn update_connector_curves(&mut self) {
        // Implementation for updating connector curves
    }

    fn render_line(
        &self,
        ui: &mut egui::Ui,
        line: &DisplayLine,
        line_idx: usize,
        is_left: bool,
    ) -> Rect {
        self.line_renderer.render_line(ui, line, line_idx, is_left)
    }
}

impl eframe::App for DiffViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply comprehensive JetBrains theme
        self.theme.apply_to_context(ctx);

        // Handle keyboard navigation
        self.handle_keyboard_navigation(ctx);

        // Update viewport height dynamically
        let viewport_height = ctx.screen_rect().height();
        self.scroll_sync.update_viewport_height(viewport_height);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(43, 43, 43)))
            .show(ctx, |ui| {
                let total_height = ui.available_height();
                let total_width = ui.available_width();
                let pane_width = (total_width - 45.0) / 2.0; // 45px for connector column

                // Create a horizontal layout with explicit height allocation
                ui.allocate_ui_with_layout(
                    Vec2::new(total_width, total_height),
                    egui::Layout::left_to_right(egui::Align::TOP),
                    |ui| {
                        // Left pane (original) - with explicit height allocation
                        ui.allocate_ui_with_layout(
                            Vec2::new(pane_width, total_height),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                // Header
                                ui.horizontal(|ui| {
                                    ui.add_space(10.0);
                                    ui.label(
                                        egui::RichText::new("Original")
                                            .font(FontId::new(
                                                self.theme.font_size * 1.1,
                                                egui::FontFamily::Proportional,
                                            ))
                                            .color(self.theme.foreground),
                                    );
                                });

                                ui.separator();

                                // Content area with scrolling - now with proper height allocation
                                let available_height = ui.available_height();

                                // Get current scroll position from memory
                                let current_left_scroll = ui.ctx().memory_mut(|mem| {
                                    mem.data.get_persisted("left_scroll".into()).unwrap_or(0.0)
                                });

                                // Calculate synchronized position if right pane is master
                                let left_scroll_offset =
                                    if self.scroll_sync.master_pane() == MasterPane::Right {
                                        self.scroll_sync.left_scroll_offset()
                                    } else {
                                        current_left_scroll
                                    };

                                let scroll_output = ScrollArea::vertical()
                                    .id_source("diff_left_scroll")
                                    .auto_shrink([false, false])
                                    .max_height(available_height)
                                    .min_scrolled_height(available_height)
                                    .scroll_offset(Vec2::new(0.0, left_scroll_offset))
                                    .show(ui, |ui| {
                                        let mut line_rects = Vec::new();

                                        for (line_idx, line) in self.old_lines.iter().enumerate() {
                                            let rect = self.render_line(ui, line, line_idx, true);
                                            line_rects.push(rect);
                                        }

                                        ui.ctx().memory_mut(|mem| {
                                            mem.data
                                                .insert_persisted("left_rects".into(), line_rects);
                                        });
                                    });

                                self.scroll_sync
                                    .set_left_scroll(scroll_output.state.offset.y);

                                // Synchronize right pane based on left pane scroll
                                self.scroll_sync.synchronize_scrolls(|y| {
                                    map_left_to_right(y, &self.mapping_segments)
                                });

                                ui.ctx().memory_mut(|mem| {
                                    mem.data.insert_persisted(
                                        "left_scroll".into(),
                                        scroll_output.state.offset.y,
                                    );
                                });
                            },
                        );

                        // Middle gutter for connections
                        ui.allocate_ui_with_layout(
                            Vec2::new(45.0, total_height),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                // Draw connection lines
                                self.connector_renderer.draw_connection_lines(
                                    ui,
                                    &self.old_lines,
                                    &self.new_lines,
                                );
                            },
                        );

                        // Right pane (modified) - with explicit height allocation
                        ui.allocate_ui_with_layout(
                            Vec2::new(pane_width, total_height),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                // Header
                                ui.horizontal(|ui| {
                                    ui.add_space(10.0);
                                    ui.label(
                                        egui::RichText::new("Modified")
                                            .font(FontId::new(
                                                self.theme.font_size * 1.1,
                                                egui::FontFamily::Proportional,
                                            ))
                                            .color(self.theme.foreground),
                                    );
                                });

                                ui.separator();

                                // Content area with scrolling - now with proper height allocation
                                let available_height = ui.available_height();

                                // Get current scroll position from memory
                                let current_right_scroll = ui.ctx().memory_mut(|mem| {
                                    mem.data.get_persisted("right_scroll".into()).unwrap_or(0.0)
                                });

                                // Calculate synchronized position if left pane is master
                                let right_scroll_offset =
                                    if self.scroll_sync.master_pane() == MasterPane::Left {
                                        self.scroll_sync.right_scroll_offset()
                                    } else {
                                        current_right_scroll
                                    };

                                let scroll_output = ScrollArea::vertical()
                                    .id_source("diff_right_scroll")
                                    .auto_shrink([false, false])
                                    .max_height(available_height)
                                    .min_scrolled_height(available_height)
                                    .scroll_offset(Vec2::new(0.0, right_scroll_offset))
                                    .show(ui, |ui| {
                                        let mut line_rects = Vec::new();

                                        for (line_idx, line) in self.new_lines.iter().enumerate() {
                                            let rect = self.render_line(ui, line, line_idx, false);
                                            line_rects.push(rect);
                                        }

                                        ui.ctx().memory_mut(|mem| {
                                            mem.data
                                                .insert_persisted("right_rects".into(), line_rects);
                                        });
                                    });

                                self.scroll_sync
                                    .set_right_scroll(scroll_output.state.offset.y);

                                // Synchronize left pane based on right pane scroll
                                self.scroll_sync.synchronize_scrolls(|y| {
                                    map_right_to_left(y, &self.mapping_segments)
                                });

                                ui.ctx().memory_mut(|mem| {
                                    mem.data.insert_persisted(
                                        "right_scroll".into(),
                                        scroll_output.state.offset.y,
                                    );
                                });
                            },
                        );
                    },
                );
            });
    }
}
