use std::fs;
use std::path::{Path, PathBuf};

use ab_glyph::FontArc;
use anyhow::{Context, Result};

use crate::actions::ActionHandler;
use crate::config::{ConfigManager, LayoutConfig, WindowConfig};
use crate::core::app_bootstrap::AppBootstrap;
use crate::diff::imara::compute_imara_diff_default;
use crate::diff::parser::create_complete_side_by_side_with_diff;
use crate::git::GitOps;
use crate::gpui::{Canvas, Color, GpuiApp, GpuiEvent, KeyCode, KeyState};
use crate::models::line::{DisplayLine, LineType};
use crate::models::ConnectorKind;
use crate::navigation::{NavigationAction, NavigationState};
use crate::state::StateManager;
use crate::sync::{build_anchors_from_blocks, build_connector_curves, build_mapping_segments};
use crate::toolbar::{DiffMode, ToolbarState};

const HEADER_HEIGHT: f32 = 36.0;
const GUTTER_WIDTH: f32 = 64.0;

const COLOR_HEADER: Color = Color::rgba(32, 35, 46, 255);
const COLOR_HEADER_TEXT: Color = Color::rgba(224, 229, 243, 255);
const COLOR_INSTRUCTIONS: Color = Color::rgba(160, 170, 190, 255);
const COLOR_SELECTION: Color = Color::rgba(96, 130, 200, 90);

const HEADER_TITLE_SIZE: f32 = 18.0;
const HEADER_INSTRUCTION_SIZE: f32 = 14.0;
const INSTRUCTIONS_WIDTH: f32 = 420.0;
const TITLE_Y_OFFSET: f32 = 6.0;
const INSTRUCTION_BASELINE_OFFSET: f32 = 6.0;
const TEXT_PADDING: f32 = 8.0;
const MIN_HEADER_PADDING: f32 = 12.0;
const MIN_COLUMN_WIDTH: f32 = 100.0;
const GUTTER_FONT_SIZE: f32 = 14.0;

struct DiffPalette {
    background: Color,
    text_primary: Color,
    line_numbers: Color,
    addition_background: Color,
    deletion_background: Color,
    modification_background: Color,
    addition_ribbon: Color,
    deletion_ribbon: Color,
    modification_ribbon: Color,
    crushed_insert: Color,
    crushed_delete: Color,
}

impl DiffPalette {
    fn new() -> Self {
        Self {
            background: Color::rgba(23, 25, 33, 255),
            text_primary: Color::rgba(212, 212, 212, 255),
            line_numbers: Color::rgba(153, 153, 153, 255),
            addition_background: Color::rgba(52, 85, 52, 255),
            deletion_background: Color::rgba(85, 56, 56, 255),
            modification_background: Color::rgba(50, 66, 98, 255),
            addition_ribbon: Color::rgba(52, 85, 52, 255),
            deletion_ribbon: Color::rgba(85, 56, 56, 255),
            modification_ribbon: Color::rgba(50, 66, 98, 255),
            crushed_insert: Color::rgba(52, 85, 52, 255),
            crushed_delete: Color::rgba(85, 56, 56, 255),
        }
    }
}

pub struct DiffViewerApp {
    state_manager: StateManager,
    action_handler: ActionHandler,
    config_manager: ConfigManager,
    layout_config: LayoutConfig,
    project_files: Vec<PathBuf>,
    current_file_index: usize,
    font: FontArc,
    window_title: String,
    viewport_width: u32,
    viewport_height: u32,
    palette: DiffPalette,
    device_scale: f32,
    viewing_demo: bool,
    toolbar_state: crate::toolbar::ToolbarState,
}

impl DiffViewerApp {
    pub fn from_bootstrap(bootstrap: AppBootstrap) -> Result<Self> {
        let font_bytes = bootstrap
            .config_manager
            .get_font_manager()
            .buffer_font_bytes();
        let font = FontArc::try_from_slice(font_bytes).context("failed to load embedded font")?;

        let viewing_demo = bootstrap.project_files.is_empty();
        let mut app = Self {
            state_manager: bootstrap.state_manager,
            action_handler: bootstrap.action_handler,
            config_manager: bootstrap.config_manager,
            layout_config: LayoutConfig::default(),
            project_files: bootstrap.project_files,
            current_file_index: 0,
            font,
            window_title: "JetBrains Diff Viewer".to_string(),
            viewport_width: WindowConfig::default_window_descriptor().width,
            viewport_height: WindowConfig::default_window_descriptor().height,
            palette: DiffPalette::new(),
            device_scale: 1.0,
            viewing_demo,
            toolbar_state: ToolbarState::new(Vec::new()),
        };

        app.layout_config = app.config_manager.get_config().layout.clone();
        app.toolbar_state = ToolbarState::new(app.project_files.clone());

        if !app.project_files.is_empty() {
            let first = app.project_files[0].clone();
            app.viewing_demo = false;
            app.load_file_diff(&first);
        } else {
            app.load_demo_view();
        }

        Ok(app)
    }

    pub fn title(&self) -> &str {
        &self.window_title
    }

    fn device_scale(&self) -> f32 {
        self.device_scale
    }

    fn header_height(&self) -> f32 {
        HEADER_HEIGHT * self.device_scale()
    }

    fn gutter_width(&self) -> f32 {
        GUTTER_WIDTH * self.device_scale()
    }

    fn connector_column_width(&self) -> f32 {
        self.layout_config.connector_column_width * self.device_scale()
    }

    fn pane_padding(&self) -> f32 {
        self.layout_config.pane_padding * self.device_scale()
    }

    fn min_header_padding(&self) -> f32 {
        MIN_HEADER_PADDING * self.device_scale()
    }

    fn header_title_size(&self) -> f32 {
        HEADER_TITLE_SIZE * self.device_scale()
    }

    fn header_instruction_size(&self) -> f32 {
        HEADER_INSTRUCTION_SIZE * self.device_scale()
    }

    fn instructions_width(&self) -> f32 {
        INSTRUCTIONS_WIDTH * self.device_scale()
    }

    fn header_title_offset(&self) -> f32 {
        TITLE_Y_OFFSET * self.device_scale()
    }

    fn header_instruction_offset(&self) -> f32 {
        INSTRUCTION_BASELINE_OFFSET * self.device_scale()
    }

    fn text_padding(&self) -> f32 {
        TEXT_PADDING * self.device_scale()
    }

    fn min_column_width(&self) -> f32 {
        MIN_COLUMN_WIDTH * self.device_scale()
    }

    fn gutter_font_size(&self) -> f32 {
        GUTTER_FONT_SIZE * self.device_scale()
    }

    fn content_font_size(&self) -> f32 {
        self.config_manager.get_font_manager().buffer_font_size() * self.device_scale()
    }

    fn line_height(&self) -> f32 {
        WindowConfig::get_line_height(&self.config_manager) * self.device_scale()
    }

    fn navigation_state(&self) -> &NavigationState {
        &self.state_manager.get_current_state().navigation_state
    }

    fn navigation_state_mut(&mut self) -> &mut NavigationState {
        &mut self.state_manager.get_current_state_mut().navigation_state
    }

    fn scroll_offset(&self) -> f32 {
        let state = self.state_manager.get_current_state();
        state.left_scroll_offset.max(state.right_scroll_offset)
    }

    fn set_scroll_offset(&mut self, offset: f32) {
        let max = self.max_scroll_offset();
        let clamped = offset.clamp(0.0, max);
        let line_height = self.line_height();
        let available = (self.viewport_height as f32 - self.header_height()).max(0.0);
        let state = self.state_manager.get_current_state_mut();
        let left_total = state.left_lines.len() as f32 * line_height;
        let right_total = state.right_lines.len() as f32 * line_height;
        let max_left_scroll = (left_total - available).max(0.0);
        let max_right_scroll = (right_total - available).max(0.0);
        state.left_scroll_offset = clamped.min(max_left_scroll);
        state.right_scroll_offset = clamped.min(max_right_scroll);
    }

    fn max_scroll_offset(&self) -> f32 {
        let state = self.state_manager.get_current_state();
        let total_lines = state.left_lines.len().max(state.right_lines.len()) as f32;
        let total_height = total_lines * self.line_height();
        let available = (self.viewport_height as f32 - self.header_height()).max(0.0);
        if total_height <= available || available <= 0.0 {
            0.0
        } else {
            total_height - available
        }
    }

    fn handle_navigation_action(&mut self, action: NavigationAction) {
        match action {
            NavigationAction::NextDiffBlock => {
                if let Some(idx) = self.navigation_state_mut().next_block() {
                    self.goto_block(idx);
                }
            }
            NavigationAction::PreviousDiffBlock => {
                if let Some(idx) = self.navigation_state_mut().previous_block() {
                    self.goto_block(idx);
                }
            }
            NavigationAction::NextConnector => {
                if let Some(idx) = self.navigation_state_mut().next_connector() {
                    if let Some(curve) = self
                        .state_manager
                        .get_current_state()
                        .connector_curves
                        .get(idx)
                    {
                        self.scroll_to_line(curve.focus_line);
                    }
                }
            }
            NavigationAction::PreviousConnector => {
                if let Some(idx) = self.navigation_state_mut().previous_connector() {
                    if let Some(curve) = self
                        .state_manager
                        .get_current_state()
                        .connector_curves
                        .get(idx)
                    {
                        self.scroll_to_line(curve.focus_line);
                    }
                }
            }
            NavigationAction::ApplyHunk
            | NavigationAction::RevertHunk
            | NavigationAction::StageHunk => {
                let current_block = self.navigation_state().current_block_index;
                let result = self.action_handler.execute_action(action, current_block);
                if !result.success {
                    eprintln!("Action failed: {}", result.message);
                }
            }
            NavigationAction::None => {}
        }
    }

    fn load_file_diff(&mut self, file_path: &Path) {
        eprintln!("Loading diff for file: {:?}", file_path);
        let git_ops = GitOps::with_current_dir();
        let file_path_str = file_path.to_string_lossy().to_string();

        let current_content = fs::read_to_string(file_path).unwrap_or_else(|err| {
            eprintln!("❌ Failed to read current file ({}), using fallback", err);
            String::new()
        });

        let original_content = match git_ops.show_file("HEAD", &file_path_str) {
            crate::git::GitResult {
                success: true,
                stdout,
                ..
            } => stdout,
            _ => {
                eprintln!("❌ Failed to read original from git, using current as original");
                current_content.clone()
            }
        };

        let diff_text = match git_ops.diff_file(None, None, &file_path_str) {
            crate::git::GitResult {
                success: true,
                stdout,
                ..
            } => stdout,
            _ => String::new(),
        };

        let (left_lines, right_lines, change_blocks) =
            create_complete_side_by_side_with_diff(&original_content, &current_content, &diff_text);
        let imara_analysis = compute_imara_diff_default(&original_content, &current_content);
        let line_height = self.line_height();
        let anchors = build_anchors_from_blocks(&change_blocks, line_height);
        let mapping_segments = build_mapping_segments(&anchors);
        let connector_curves = build_connector_curves(&imara_analysis);

        let header_height = self.header_height();
        let viewport_height = self.viewport_height as f32;
        self.state_manager.update_state(|state| {
            state.current_file = file_path_str.clone();
            state.left_lines = left_lines;
            state.right_lines = right_lines;
            state.change_blocks = change_blocks;
            state.imara_analysis = imara_analysis;
            state.anchors = anchors;
            state.mapping_segments = mapping_segments;
            state.connector_curves = connector_curves.clone();
            state.reset_navigation();
            state.viewport_height = (viewport_height - header_height).max(0.0);
            state.left_scroll_offset = 0.0;
            state.right_scroll_offset = 0.0;
            state.update_navigation_state();
        });

        self.set_scroll_offset(0.0);
        self.viewing_demo = false;
        if !self.project_files.is_empty() {
            self.toolbar_state.current_mode = DiffMode::ProjectFile(self.current_file_index);
        }
        self.update_window_title(
            file_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or(file_path_str),
        );
    }

    fn load_demo_view(&mut self) {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let left_path = manifest_dir.join("examples/ProvidersOld.tsx");
        let right_path = manifest_dir.join("examples/ProvidersNew.tsx");

        let fallback_original =
            "function App() {\n  return <div>Hello World</div>;\n}\n\nexport default App;\n";
        let fallback_current = "import React from 'react';\nimport { BrowserRouter as Router, Routes, Route } from 'react-router-dom';\nimport { ThemeProvider } from './theme';\nimport { AuthProvider } from './auth';\nimport { NotificationProvider } from './notifications';\n\nfunction AppProviders({ children }) {\n  return (\n    <ThemeProvider>\n      <AuthProvider>\n        <NotificationProvider>\n          <Router>\n            {children}\n          </Router>\n        </NotificationProvider>\n      </AuthProvider>\n    </ThemeProvider>\n  );\n}\n\nexport default AppProviders;\n";

        let demo_original = fs::read_to_string(&left_path).unwrap_or_else(|err| {
            eprintln!(
                "⚠️ Failed to read demo original from {:?}: {} — using fallback",
                left_path, err
            );
            fallback_original.to_string()
        });

        let demo_current = fs::read_to_string(&right_path).unwrap_or_else(|err| {
            eprintln!(
                "⚠️ Failed to read demo current from {:?}: {} — using fallback",
                right_path, err
            );
            fallback_current.to_string()
        });

        let demo_label = format!(
            "{} → {}",
            left_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("ProvidersOld.tsx"),
            right_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("ProvidersNew.tsx"),
        );

        let (left_lines, right_lines, change_blocks) =
            create_complete_side_by_side_with_diff(&demo_original, &demo_current, "");
        let imara_analysis = compute_imara_diff_default(&demo_original, &demo_current);
        let connector_curves = build_connector_curves(&imara_analysis);
        let line_height = self.line_height();
        let anchors = build_anchors_from_blocks(&change_blocks, line_height);
        let mapping_segments = build_mapping_segments(&anchors);
        let viewport_height = self.viewport_height as f32;
        let header_height = self.header_height();

        self.state_manager.update_state(|state| {
            state.current_file = demo_label.clone();
            state.left_lines = left_lines;
            state.right_lines = right_lines;
            state.change_blocks = change_blocks;
            state.imara_analysis = imara_analysis.clone();
            state.anchors = anchors.clone();
            state.mapping_segments = mapping_segments.clone();
            state.connector_curves = connector_curves.clone();
            state.reset_navigation();
            state.viewport_height = (viewport_height - header_height).max(0.0);
            state.left_scroll_offset = 0.0;
            state.right_scroll_offset = 0.0;
            state.update_navigation_state();
        });

        self.set_scroll_offset(0.0);
        self.viewing_demo = true;
        self.toolbar_state.go_to_default();
        self.update_window_title(demo_label);
    }

    fn refresh_layout_metrics(&mut self) {
        let line_height = self.line_height();
        let viewport_height = self.viewport_height as f32;
        let header_height = self.header_height();
        self.state_manager.update_state(|state| {
            let anchors = build_anchors_from_blocks(&state.change_blocks, line_height);
            let mapping_segments = build_mapping_segments(&anchors);
            state.anchors = anchors;
            state.mapping_segments = mapping_segments;
            state.viewport_height = (viewport_height - header_height).max(0.0);
            state.update_navigation_state();
        });
    }

    fn update_window_title(&mut self, file_label: String) {
        let suffix = if self.viewing_demo {
            format!("Demo • {}", file_label)
        } else if !self.project_files.is_empty() {
            let total = self.project_files.len();
            let index = (self.current_file_index % total) + 1;
            format!("[{}/{}] {}", index, total, file_label)
        } else {
            file_label
        };
        self.window_title = format!("JetBrains Diff Viewer - {}", suffix);
    }

    fn scroll_lines(&mut self, lines: f32) {
        let line_height = self.line_height();
        let target = self.scroll_offset() + lines * line_height;
        self.set_scroll_offset(target);
    }

    fn scroll_pages(&mut self, pages: f32) {
        let line_height = self.line_height();
        let page = (self.viewport_height as f32 - self.header_height()).max(line_height);
        let target = self.scroll_offset() + pages * page;
        self.set_scroll_offset(target);
    }

    fn goto_block(&mut self, index: usize) {
        if let Some(block) = self
            .state_manager
            .get_current_state()
            .change_blocks
            .get(index)
            .cloned()
        {
            let target = block.start_line as f32 * self.line_height();
            self.set_scroll_offset(target);
        }
    }

    fn scroll_to_line(&mut self, line_index: usize) {
        let target = line_index as f32 * self.line_height();
        self.set_scroll_offset(target);
    }

    fn next_file(&mut self) {
        if self.project_files.is_empty() {
            if !self.viewing_demo {
                self.load_demo_view();
            }
            return;
        }
        self.toolbar_state.go_next();
        match self.toolbar_state.current_mode {
            DiffMode::ProjectFile(index) => {
                self.current_file_index = index;
                if let Some(path) = self.project_files.get(index).cloned() {
                    self.load_file_diff(&path);
                }
            }
            DiffMode::DefaultDemo => self.load_demo_view(),
        }
    }

    fn previous_file(&mut self) {
        if self.project_files.is_empty() {
            if !self.viewing_demo {
                self.load_demo_view();
            }
            return;
        }
        self.toolbar_state.go_previous();
        match self.toolbar_state.current_mode {
            DiffMode::ProjectFile(index) => {
                self.current_file_index = index;
                if let Some(path) = self.project_files.get(index).cloned() {
                    self.load_file_diff(&path);
                }
            }
            DiffMode::DefaultDemo => self.load_demo_view(),
        }
    }

    fn handle_character_key(&mut self, ch: char) {
        match ch.to_ascii_lowercase() {
            '[' => self.previous_file(),
            ']' => self.next_file(),
            'j' => self.handle_navigation_action(NavigationAction::NextDiffBlock),
            'k' => self.handle_navigation_action(NavigationAction::PreviousDiffBlock),
            'n' => self.handle_navigation_action(NavigationAction::NextConnector),
            'p' => self.handle_navigation_action(NavigationAction::PreviousConnector),
            'a' => self.handle_navigation_action(NavigationAction::ApplyHunk),
            'r' => self.handle_navigation_action(NavigationAction::RevertHunk),
            's' => self.handle_navigation_action(NavigationAction::StageHunk),
            'd' => self.load_demo_view(),
            _ => self.handle_navigation_action(NavigationAction::None),
        }
    }

    fn draw_header(&self, canvas: &mut Canvas) {
        let width = canvas.width as f32;
        let header_height = self.header_height();
        canvas.fill_rect(0.0, 0.0, width, header_height, COLOR_HEADER);

        let padding = self.pane_padding().max(self.min_header_padding());
        let title_y = self.header_title_offset();
        canvas.draw_text(
            &self.font,
            &self.window_title,
            (padding, title_y),
            self.header_title_size(),
            COLOR_HEADER_TEXT,
        );
        let instructions = self.header_instructions_text();
        let inst_x = (width - padding - self.instructions_width()).max(padding);
        let instructions_y = title_y + self.header_title_size() - self.header_instruction_offset();
        canvas.draw_text(
            &self.font,
            instructions.as_str(),
            (inst_x, instructions_y),
            self.header_instruction_size(),
            COLOR_INSTRUCTIONS,
        );
    }

    fn header_instructions_text(&self) -> String {
        let base = "↑/↓ scroll  •  PgUp/PgDn jump  •  [/] change file  •  J/K next diff  •  D demo";
        let total = self.toolbar_state.project_files.len();
        if total == 0 {
            base.to_string()
        } else if self.viewing_demo {
            format!("{}  •  Viewing demo", base)
        } else {
            let index = (self.current_file_index % total) + 1;
            format!("{}  •  File {}/{}", base, index, total)
        }
    }

    fn draw_diff_columns(&self, canvas: &mut Canvas) {
        let width = canvas.width as f32;
        let height = canvas.height as f32;
        let header_height = self.header_height();
        let available_height = height - header_height;
        if available_height <= 0.0 {
            return;
        }

        let gutter_width = self.gutter_width();
        let column_padding = self.pane_padding();
        let connector_spacing = self.connector_column_width();

        let column_width = ((width - (2.0 * (gutter_width + column_padding)) - connector_spacing)
            / 2.0)
            .max(self.min_column_width());
        let left_x = column_padding;
        let right_x = left_x + gutter_width + column_width + connector_spacing;

        let line_height = self.line_height();
        let scroll_offset = self.scroll_offset();

        let state = self.state_manager.get_current_state();
        let active_block = state
            .change_blocks
            .get(state.navigation_state.current_block_index)
            .cloned();

        let font_size = self.content_font_size();

        let left_total_height = state.left_lines.len() as f32 * line_height;
        let right_total_height = state.right_lines.len() as f32 * line_height;
        let max_left_scroll = (left_total_height - available_height).max(0.0);
        let max_right_scroll = (right_total_height - available_height).max(0.0);
        let left_scroll = scroll_offset.min(max_left_scroll);
        let right_scroll = scroll_offset.min(max_right_scroll);

        self.draw_column_lines(
            canvas,
            &state.left_lines,
            left_x,
            gutter_width,
            column_width,
            font_size,
            left_scroll,
            available_height,
            line_height,
            height,
            &active_block,
        );

        self.draw_column_lines(
            canvas,
            &state.right_lines,
            right_x,
            gutter_width,
            column_width,
            font_size,
            right_scroll,
            available_height,
            line_height,
            height,
            &active_block,
        );

        self.draw_connectors(
            canvas,
            state,
            left_x,
            column_width,
            connector_spacing,
            line_height,
            left_scroll,
            right_scroll,
            available_height,
        );
    }

    fn draw_column_lines(
        &self,
        canvas: &mut Canvas,
        lines: &[DisplayLine],
        column_x: f32,
        gutter_width: f32,
        column_width: f32,
        font_size: f32,
        scroll_offset: f32,
        available_height: f32,
        line_height: f32,
        canvas_height: f32,
        active_block: &Option<crate::models::diff::ChangeBlock>,
    ) {
        if lines.is_empty() {
            return;
        }

        let first_line = (scroll_offset / line_height).floor().max(0.0) as usize;
        let offset_within_line = scroll_offset - first_line as f32 * line_height;
        let lines_per_view = (available_height / line_height).ceil() as usize + 2;

        for row in 0..lines_per_view {
            let line_index = first_line + row;
            let y = self.header_height() + (row as f32 * line_height) - offset_within_line;
            if y > canvas_height {
                break;
            }

            if let Some(line) = lines.get(line_index) {
                self.draw_line(
                    canvas,
                    line,
                    line_index,
                    column_x,
                    gutter_width,
                    column_width,
                    font_size,
                    y,
                    active_block,
                );
            }
        }
    }

    fn draw_line(
        &self,
        canvas: &mut Canvas,
        line: &DisplayLine,
        line_index: usize,
        column_x: f32,
        gutter_width: f32,
        column_width: f32,
        font_size: f32,
        y: f32,
        active_block: &Option<crate::models::diff::ChangeBlock>,
    ) {
        let base_color = match line.line_type {
            LineType::Context => self.palette.background,
            LineType::Addition => self.palette.addition_background,
            LineType::Deletion => self.palette.deletion_background,
            LineType::Modification => self.palette.modification_background,
        };
        canvas.fill_rect(
            column_x,
            y,
            gutter_width + column_width,
            self.line_height(),
            base_color,
        );

        if let Some(block) = active_block {
            if line_index >= block.start_line && line_index <= block.end_line {
                let highlight = self.highlight_color(&line.line_type);
                canvas.fill_rect(
                    column_x,
                    y,
                    gutter_width + column_width,
                    self.line_height(),
                    highlight,
                );
            }
        }

        let number = line
            .original_line_num
            .map(|n| n.to_string())
            .unwrap_or_else(|| "".to_string());
        let gutter_color = self.palette.line_numbers;
        let padding = self.text_padding();
        let gutter_font_size = self.gutter_font_size();
        canvas.draw_text(
            &self.font,
            &number,
            (column_x + padding, y + self.text_baseline_offset()),
            gutter_font_size,
            gutter_color,
        );

        let text = line.content.replace('\t', "    ");
        let text_color = self.palette.text_primary;
        canvas.draw_text(
            &self.font,
            &text,
            (
                column_x + gutter_width + padding,
                y + self.text_baseline_offset(),
            ),
            font_size,
            text_color,
        );
    }

    fn text_baseline_offset(&self) -> f32 {
        let line_height = self.line_height();
        let text_height = self.content_font_size();
        let base = ((line_height - text_height) / 2.0).max(0.0);
        (base - 0.5 * self.device_scale()).max(0.0)
    }

    fn draw_connectors(
        &self,
        canvas: &mut Canvas,
        state: &crate::state::app_state::AppState,
        left_x: f32,
        column_width: f32,
        connector_width: f32,
        line_height: f32,
        left_scroll: f32,
        right_scroll: f32,
        available_height: f32,
    ) {
        if state.connector_curves.is_empty() {
            return;
        }

        let gutter_start = left_x + self.gutter_width() + column_width;
        let gutter_end = gutter_start + connector_width;
        let viewport_top = self.header_height();
        let viewport_bottom = viewport_top + available_height;

        let crushed_insert_color = self.palette.crushed_insert;
        let crushed_delete_color = self.palette.crushed_delete;

        for curve in &state.connector_curves {
            let (left_top, left_bottom) = if curve.left_crushed {
                let center = self.header_height() + (curve.left_start as f32 * line_height)
                    - left_scroll
                    + 1.0;
                (center - 1.0, center + 1.0)
            } else {
                let top =
                    self.header_height() + (curve.left_start as f32 * line_height) - left_scroll;
                let bottom = self.header_height() + ((curve.left_end + 1) as f32 * line_height)
                    - left_scroll;
                (top - 0.5, bottom)
            };
            let (right_top, right_bottom) = if curve.right_crushed {
                let center = self.header_height() + (curve.right_start as f32 * line_height)
                    - right_scroll
                    + 1.0;
                (center - 1.0, center + 1.0)
            } else {
                let top =
                    self.header_height() + (curve.right_start as f32 * line_height) - right_scroll;
                let bottom = self.header_height() + ((curve.right_end + 1) as f32 * line_height)
                    - right_scroll;
                (top - 0.5, bottom)
            };

            let min_visible = left_top.min(right_top);
            let max_visible = left_bottom.max(right_bottom);
            if max_visible < viewport_top || min_visible > viewport_bottom {
                continue;
            }

            if curve.left_crushed {
                let bar_top = (left_top + left_bottom) * 0.5 - 1.0;
                if bar_top + 2.0 >= viewport_top && bar_top <= viewport_bottom {
                    let bar_width = self.gutter_width() + column_width;
                    canvas.fill_rect(left_x, bar_top, bar_width, 2.0, crushed_insert_color);
                }
            }

            if curve.right_crushed {
                let bar_top = (right_top + right_bottom) * 0.5 - 1.0;
                if bar_top + 2.0 >= viewport_top && bar_top <= viewport_bottom {
                    let bar_width = self.gutter_width() + column_width;
                    let bar_x = left_x + self.gutter_width() + column_width + connector_width;
                    canvas.fill_rect(bar_x, bar_top, bar_width, 2.0, crushed_delete_color);
                }
            }

            let color = self.connector_color(curve.kind);
            self.draw_connector_ribbon(
                canvas,
                gutter_start,
                gutter_end,
                left_top,
                left_bottom,
                right_top,
                right_bottom,
                color,
            );
        }
    }

    fn draw_connector_ribbon(
        &self,
        canvas: &mut Canvas,
        gutter_start: f32,
        gutter_end: f32,
        left_top: f32,
        left_bottom: f32,
        right_top: f32,
        right_bottom: f32,
        color: Color,
    ) {
        let segments = 28;
        let control_offset = (gutter_end - gutter_start) * 0.30;
        let mut top_points = Vec::with_capacity(segments + 1);
        let mut bottom_points = Vec::with_capacity(segments + 1);

        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            top_points.push(Self::cubic_bezier(
                (gutter_start, left_top),
                (gutter_start + control_offset, left_top),
                (gutter_end - control_offset, right_top),
                (gutter_end, right_top),
                t,
            ));
            bottom_points.push(Self::cubic_bezier(
                (gutter_start, left_bottom),
                (gutter_start + control_offset, left_bottom),
                (gutter_end - control_offset, right_bottom),
                (gutter_end, right_bottom),
                t,
            ));
        }

        for i in 0..segments {
            let top_left = top_points[i];
            let top_right = top_points[i + 1];
            let bottom_right = bottom_points[i + 1];
            let bottom_left = bottom_points[i];

            canvas.fill_convex_quad(top_left, top_right, bottom_right, bottom_left, color);
        }
    }

    fn connector_color(&self, kind: ConnectorKind) -> Color {
        match kind {
            ConnectorKind::Modify => self.palette.modification_ribbon,
            ConnectorKind::Insert => self.palette.addition_ribbon,
            ConnectorKind::Delete => self.palette.deletion_ribbon,
        }
    }

    fn highlight_color(&self, line_type: &LineType) -> Color {
        match line_type {
            LineType::Addition => {
                let base = self.palette.addition_background;
                Color::rgba(base.r, base.g, base.b, 220)
            }
            LineType::Deletion => {
                let base = self.palette.deletion_background;
                Color::rgba(base.r, base.g, base.b, 220)
            }
            LineType::Modification => {
                let base = self.palette.modification_background;
                Color::rgba(base.r, base.g, base.b, 220)
            }
            LineType::Context => COLOR_SELECTION,
        }
    }

    fn cubic_bezier(
        p0: (f32, f32),
        p1: (f32, f32),
        p2: (f32, f32),
        p3: (f32, f32),
        t: f32,
    ) -> (f32, f32) {
        let u = 1.0 - t;
        let tt = t * t;
        let uu = u * u;
        let uuu = uu * u;
        let ttt = tt * t;

        (
            uuu * p0.0 + 3.0 * uu * t * p1.0 + 3.0 * u * tt * p2.0 + ttt * p3.0,
            uuu * p0.1 + 3.0 * uu * t * p1.1 + 3.0 * u * tt * p2.1 + ttt * p3.1,
        )
    }
}

impl GpuiApp for DiffViewerApp {
    fn title(&self) -> &str {
        self.title()
    }

    fn handle_event(&mut self, event: GpuiEvent) {
        match event {
            GpuiEvent::Key {
                code,
                state,
                repeat,
            } => {
                if state == KeyState::Pressed && !repeat {
                    match code {
                        KeyCode::Up => self.scroll_lines(-1.0),
                        KeyCode::Down => self.scroll_lines(1.0),
                        KeyCode::PageUp => self.scroll_pages(-1.0),
                        KeyCode::PageDown => self.scroll_pages(1.0),
                        KeyCode::Home => self.set_scroll_offset(0.0),
                        KeyCode::End => self.set_scroll_offset(self.max_scroll_offset()),
                        KeyCode::Enter => {
                            self.handle_navigation_action(NavigationAction::ApplyHunk)
                        }
                        KeyCode::Backspace => {
                            self.handle_navigation_action(NavigationAction::RevertHunk)
                        }
                        KeyCode::Space => {
                            self.handle_navigation_action(NavigationAction::StageHunk)
                        }
                        KeyCode::Character(ch) => self.handle_character_key(ch),
                        KeyCode::Left => self.previous_file(),
                        KeyCode::Right => self.next_file(),
                    }
                }
            }
            GpuiEvent::Scroll { delta_lines } => {
                if delta_lines.abs() > f32::EPSILON {
                    self.scroll_lines(delta_lines);
                }
            }
            GpuiEvent::Resize {
                width,
                height,
                scale_factor,
            } => {
                self.viewport_width = width;
                self.viewport_height = height;
                self.device_scale = scale_factor.max(0.5);
                let viewport = (height as f32 - self.header_height()).max(0.0);
                {
                    let state = self.state_manager.get_current_state_mut();
                    state.viewport_height = viewport;
                }
                self.refresh_layout_metrics();
                let current_offset = self.scroll_offset();
                self.set_scroll_offset(current_offset);
            }
        }
    }

    fn render(&mut self, canvas: &mut Canvas) {
        canvas.clear(self.palette.background);
        self.draw_header(canvas);
        self.draw_diff_columns(canvas);
    }
}
