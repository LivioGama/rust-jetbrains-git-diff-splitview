// diffsplit/src/app/mod.rs
use eframe::egui;
use egui::{Color32, Rect};

use crate::actions::*;
use crate::config::*;
use crate::models::*;
use crate::navigation::*;
use crate::rendering::JetBrainsRenderer;
use crate::state::*;
use crate::sync::*;
use crate::theme::*;
use crate::ui::layout::*;
use crate::ui::*;

pub struct DiffViewerApp {
    pub state_manager: StateManager,
    pub action_handler: ActionHandler,
    pub scroll_sync: ScrollSync,
    pub theme: JetBrainsTheme,
    pub line_renderer: LineRenderer,
    pub connector_renderer: ConnectorRenderer,
    pub navigation_handler: NavigationHandler,
    pub layout_manager: LayoutManager,
    pub config_manager: ConfigManager,
}

impl DiffViewerApp {
    pub fn new(state_manager: StateManager, action_handler: ActionHandler) -> Self {
        let theme = JetBrainsTheme::dark_theme();
        let line_height = 18.0;
        let viewport_height = 1000.0;
        let config_manager = ConfigManager::new();

        Self {
            state_manager,
            action_handler,
            scroll_sync: ScrollSync::new(line_height, viewport_height),
            theme: theme.clone(),
            line_renderer: LineRenderer::new(theme.clone()),
            connector_renderer: ConnectorRenderer::new(theme),
            navigation_handler: NavigationHandler::new(),
            layout_manager: LayoutManager::new(config_manager.get_config().layout.clone()),
            config_manager,
        }
    }

    fn handle_navigation_action(&mut self, action: NavigationAction) {
        let current_block = self
            .state_manager
            .get_current_state()
            .navigation_state
            .current_block_index;
        let result = self.action_handler.execute_action(action, current_block);

        // Handle the result silently
        if !result.success {
            eprintln!("Action failed: {}", result.message);
        }
    }
}

impl eframe::App for DiffViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply comprehensive JetBrains theme
        self.theme.apply_to_context(ctx);

        // Handle keyboard navigation
        let action = self.navigation_handler.handle_input(ctx);
        self.handle_navigation_action(action);

        // Update navigation state from current state
        let current_state = self.state_manager.get_current_state();
        self.navigation_handler.update_state(
            current_state.change_blocks.len(),
            current_state.connector_curves.len(),
        );

        // Update viewport height dynamically
        let viewport_height = ctx.screen_rect().height();
        self.scroll_sync.update_viewport_height(viewport_height);

        // Update state with current scroll positions
        let left_scroll = self.scroll_sync.left_scroll_offset();
        let right_scroll = self.scroll_sync.right_scroll_offset();
        self.state_manager.update_state(|state| {
            state.update_scroll_offsets(left_scroll, right_scroll);
            state.set_viewport_height(viewport_height);
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(43, 43, 43)))
            .show(ctx, |ui| {
                let current_state = self.state_manager.get_current_state();
                let left_lines = current_state.left_lines.clone();
                let right_lines = current_state.right_lines.clone();

                // Hardcoded example: highlight line 1 left, lines 2-12 right with S connector
                self.render_hardcoded_example(ui, &left_lines, &right_lines);
            });
    }
}

impl DiffViewerApp {
    fn render_hardcoded_example(
        &mut self,
        ui: &mut egui::Ui,
        left_lines: &[DisplayLine],
        right_lines: &[DisplayLine],
    ) {
        let available_rect = ui.available_rect_before_wrap();
        let line_height = 18.0;

        // Create side-by-side layout with wider gutter (1.5x wider)
        let left_width = available_rect.width() / 2.0 - 50.0; // Reduced parent width
        let right_width = available_rect.width() / 2.0 - 50.0;
        let gutter_width = 75.0; // 1.5 times wider than original 50.0

        let left_rect = egui::Rect::from_min_size(
            available_rect.min,
            egui::Vec2::new(left_width, available_rect.height()),
        );

        let right_rect = egui::Rect::from_min_size(
            egui::Pos2::new(
                available_rect.min.x + left_width + gutter_width,
                available_rect.min.y,
            ),
            egui::Vec2::new(right_width, available_rect.height()),
        );

        // Render left side content
        ui.scope_builder(egui::UiBuilder::new().max_rect(left_rect), |ui| {
            egui::ScrollArea::vertical()
                .id_salt("left_scroll_area")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (i, line) in left_lines.iter().enumerate() {
                        self.line_renderer.render_line(ui, line, i, true);
                    }
                });
        });

        // Render right side content
        ui.scope_builder(egui::UiBuilder::new().max_rect(right_rect), |ui| {
            egui::ScrollArea::vertical()
                .id_salt("right_scroll_area")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (i, line) in right_lines.iter().enumerate() {
                        self.line_renderer.render_line(ui, line, i, false);
                    }
                });
        });

        // Draw editor borders
        // Right border of left editor
        ui.painter().line_segment(
            [
                egui::Pos2::new(left_rect.right(), left_rect.top()),
                egui::Pos2::new(left_rect.right(), left_rect.bottom()),
            ],
            egui::Stroke::new(1.0, egui::Color32::RED),
        );

        // Left border of right editor
        ui.painter().line_segment(
            [
                egui::Pos2::new(right_rect.left(), right_rect.top()),
                egui::Pos2::new(right_rect.left(), right_rect.bottom()),
            ],
            egui::Stroke::new(1.0, egui::Color32::RED),
        );

        // Hardcoded highlights and connector
        // Highlight line 1 (index 0) on left side
        if left_lines.len() > 0 {
            let left_highlight_rect = egui::Rect::from_min_size(
                egui::Pos2::new(left_rect.min.x, left_rect.min.y + 50.0), // Offset for scroll area
                egui::Vec2::new(left_width, line_height),
            );

            // Draw left highlight
            ui.painter().rect_filled(
                left_highlight_rect,
                0.0,
                egui::Color32::from_rgba_premultiplied(51, 130, 255, 32),
            );

            // Highlight lines 2-12 (indices 1-11) on right side
            if right_lines.len() > 11 {
                let right_highlight_rect = egui::Rect::from_min_size(
                    egui::Pos2::new(right_rect.min.x, right_rect.min.y + 50.0 + line_height), // Lines 2-12
                    egui::Vec2::new(right_width, line_height * 11.0), // 11 lines (2-12)
                );

                // Draw right highlight
                ui.painter().rect_filled(
                    right_highlight_rect,
                    0.0,
                    egui::Color32::from_rgba_premultiplied(51, 130, 255, 32),
                );

                // Draw S-shaped connector between highlights
                self.draw_s_connector(ui, left_highlight_rect, right_highlight_rect, gutter_width);
            }
        }
    }

    fn draw_s_connector(
        &self,
        ui: &mut egui::Ui,
        left_rect: egui::Rect,
        right_rect: egui::Rect,
        gutter_width: f32,
    ) {
        // Step 3 — Connector anchor points - touch the exact edges
        let x0 = left_rect.right();
        let y0_top = left_rect.top();
        let y0_bottom = left_rect.bottom();

        let x1 = right_rect.left();
        let y1_top = right_rect.top();
        let y1_bottom = right_rect.bottom();

        // Calculate control points for cubic Bézier curves
        let cp1_x = x0 + gutter_width * 0.35;
        let cp2_x = x1 - gutter_width * 0.35;

        // Step 5 — S-shape adjustment for vertical offset
        let y_diff_top = y1_top - y0_top;
        let cp1_y_top = y0_top + y_diff_top * 0.2;
        let cp2_y_top = y1_top - y_diff_top * 0.2;

        let y_diff_bottom = y1_bottom - y0_bottom;
        let cp1_y_bottom = y0_bottom + y_diff_bottom * 0.2;
        let cp2_y_bottom = y1_bottom - y_diff_bottom * 0.2;

        // Draw top curve using egui's smooth curve
        ui.painter().add(egui::epaint::Shape::CubicBezier(
            egui::epaint::CubicBezierShape {
                points: [
                    egui::Pos2::new(x0, y0_top),
                    egui::Pos2::new(cp1_x, cp1_y_top),
                    egui::Pos2::new(cp2_x, cp2_y_top),
                    egui::Pos2::new(x1, y1_top),
                ],
                closed: false,
                fill: egui::Color32::TRANSPARENT,
                stroke: egui::epaint::PathStroke::new(
                    3.0,
                    egui::Color32::from_rgba_premultiplied(51, 130, 255, 80),
                ),
            },
        ));

        // Draw bottom curve using egui's smooth curve
        ui.painter().add(egui::epaint::Shape::CubicBezier(
            egui::epaint::CubicBezierShape {
                points: [
                    egui::Pos2::new(x0, y0_bottom),
                    egui::Pos2::new(cp1_x, cp1_y_bottom),
                    egui::Pos2::new(cp2_x, cp2_y_bottom),
                    egui::Pos2::new(x1, y1_bottom),
                ],
                closed: false,
                fill: egui::Color32::TRANSPARENT,
                stroke: egui::epaint::PathStroke::new(
                    3.0,
                    egui::Color32::from_rgba_premultiplied(51, 130, 255, 80),
                ),
            },
        ));

        // Fill the area between curves for the connector body
        let mut fill_points = Vec::new();

        // Top curve points
        for i in 0..=20 {
            let t = i as f32 / 20.0;
            let u = 1.0 - t;
            let u2 = u * u;
            let u3 = u2 * u;
            let t2 = t * t;
            let t3 = t2 * t;

            let x = u3 * x0 + 3.0 * u2 * t * cp1_x + 3.0 * u * t2 * cp2_x + t3 * x1;
            let y = u3 * y0_top + 3.0 * u2 * t * cp1_y_top + 3.0 * u * t2 * cp2_y_top + t3 * y1_top;
            fill_points.push(egui::Pos2::new(x, y));
        }

        // Bottom curve points (reverse order)
        for i in (0..=20).rev() {
            let t = i as f32 / 20.0;
            let u = 1.0 - t;
            let u2 = u * u;
            let u3 = u2 * u;
            let t2 = t * t;
            let t3 = t2 * t;

            let x = u3 * x0 + 3.0 * u2 * t * cp1_x + 3.0 * u * t2 * cp2_x + t3 * x1;
            let y = u3 * y0_bottom
                + 3.0 * u2 * t * cp1_y_bottom
                + 3.0 * u * t2 * cp2_y_bottom
                + t3 * y1_bottom;
            fill_points.push(egui::Pos2::new(x, y));
        }

        // Draw filled connector body
        let fill_shape = egui::epaint::PathShape {
            points: fill_points,
            closed: true,
            fill: egui::Color32::from_rgba_premultiplied(51, 130, 255, 32),
            stroke: egui::epaint::PathStroke::NONE,
        };

        ui.painter().add(egui::epaint::Shape::Path(fill_shape));
    }
}
