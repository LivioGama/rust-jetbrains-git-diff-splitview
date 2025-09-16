// diffsplit/src/app/mod.rs
use eframe::egui;
use egui::{Color32, Rect};

use crate::config::*;
use crate::models::*;
use crate::navigation::*;
use crate::sync::*;
use crate::theme::*;
use crate::ui::layout::*;
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
    pub navigation_handler: NavigationHandler,
    pub layout_manager: LayoutManager,
    pub config_manager: ConfigManager,
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
        let config_manager = ConfigManager::new();

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
            navigation_handler: NavigationHandler::new(),
            layout_manager: LayoutManager::new(config_manager.get_config().layout.clone()),
            config_manager,
        }
    }

    fn handle_navigation_action(&mut self, action: NavigationAction) {
        match action {
            NavigationAction::ApplyHunk => self.apply_current_hunk(),
            NavigationAction::RevertHunk => self.revert_current_hunk(),
            NavigationAction::StageHunk => self.stage_current_hunk(),
            _ => {} // Other actions are handled by the navigation handler
        }
    }

    fn apply_current_hunk(&mut self) {
        // Implementation for applying current hunk
        // TODO: Integrate with git operations
        println!(
            "Applying current hunk at block {}",
            self.navigation_handler.current_block_index()
        );
    }

    fn revert_current_hunk(&mut self) {
        // Implementation for reverting current hunk
        // TODO: Integrate with git operations
        println!(
            "Reverting current hunk at block {}",
            self.navigation_handler.current_block_index()
        );
    }

    fn stage_current_hunk(&mut self) {
        // Implementation for staging current hunk
        // TODO: Integrate with git operations
        println!(
            "Staging current hunk at block {}",
            self.navigation_handler.current_block_index()
        );
    }

    fn update_connector_curves(&mut self) {
        // Implementation for updating connector curves
        // TODO: Implement connector curve updates
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
        let action = self.navigation_handler.handle_input(ctx);
        self.handle_navigation_action(action);

        // Update navigation state
        self.navigation_handler
            .update_state(self.change_blocks.len(), 0); // TODO: Update connector count

        // Update viewport height dynamically
        let viewport_height = ctx.screen_rect().height();
        self.scroll_sync.update_viewport_height(viewport_height);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(43, 43, 43)))
            .show(ctx, |ui| {
                self.layout_manager.render_layout(
                    ui,
                    &self.old_lines,
                    &self.new_lines,
                    &mut self.scroll_sync,
                    &self.theme,
                    &mut self.line_renderer,
                    &mut self.connector_renderer,
                    &self.mapping_segments,
                );
            });
    }
}
