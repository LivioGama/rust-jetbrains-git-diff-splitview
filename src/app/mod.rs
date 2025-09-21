// diffsplit/src/app/mod.rs
use eframe::egui;

use crate::actions::*;
use crate::config::*;
use crate::navigation::*;
use crate::state::*;
use crate::sync::*;
use crate::theme::*;
use crate::ui::layout::LayoutManager;
use crate::ui::{LineRenderer, ConnectorRenderer};

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
        eprintln!("🎨 Creating DiffViewerApp...");

        // Initialize configuration with error handling
        let config_manager = ConfigManager::new();
        eprintln!("✅ Configuration manager created successfully");

        // Initialize theme with safe defaults
        let theme = JetBrainsTheme::dark_theme();
        eprintln!("✅ Theme initialized successfully");

        let line_height = theme.line_height(); // Use Zed's calculated line height
        let viewport_height = 1000.0;

        eprintln!("📏 Using line height: {}", line_height);

        Self {
            state_manager,
            action_handler,
            scroll_sync: ScrollSync::new(line_height, viewport_height),
            theme: theme.clone(),
            line_renderer: LineRenderer::new(theme.clone()),
            connector_renderer: ConnectorRenderer::new(theme.clone()),
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
        // Apply Zed font configuration and JetBrains theme
        self.config_manager.get_font_manager().apply_to_context(ctx);
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
            .frame(egui::Frame::new().fill(self.theme.background))
            .show(ctx, |ui| {
                let current_state = self.state_manager.get_current_state();
                let left_lines = current_state.left_lines.clone();
                let right_lines = current_state.right_lines.clone();
                let mapping_segments = current_state.mapping_segments.clone();
                let imara_analysis = current_state.imara_analysis.clone();

                // Use the proper layout manager with improved connector rendering
                self.layout_manager.render_layout(
                    ui,
                    &left_lines,
                    &right_lines,
                    &mut self.scroll_sync,
                    &self.theme,
                    &mut self.line_renderer,
                    &mut self.connector_renderer,
                    &mapping_segments,
                    &imara_analysis,
                );
            });
    }
}
