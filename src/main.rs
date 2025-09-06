// Modular JetBrains Git Diff Viewer
use eframe::egui;
use egui::{Color32, FontId, Pos2, Rect, ScrollArea, Vec2};

// Module declarations
mod diff;
mod models;
mod sync;
mod theme;
mod ui;

// Re-exports for convenience
use models::*;
use sync::*;
use theme::*;
use ui::*;
// Types and data structures are now in the models module
// ScrollSync is now in the sync module
// JetBrainsTheme is now in the theme module
// Diff parsing is now in the diff module
// UI rendering is now in the ui module
// ScrollSync and mapping functions are now in the sync module

// Functions moved to sync module

// JetBrains theming system is now in the theme module
// Types are now in the models module

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 1000.0])
            .with_resizable(true),
        ..Default::default()
    };

    // Read complete files and apply diff highlighting
    let original_content = std::process::Command::new("git")
        .arg("show")
        .arg("HEAD:src/main.rs")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        .unwrap_or_else(|_| "Error reading original main.rs".to_string());

    let current_content = std::fs::read_to_string("src/main.rs")
        .unwrap_or_else(|_| "Error reading current main.rs".to_string());

    // Get git diff to identify changes
    let diff_text = std::process::Command::new("git")
        .arg("diff")
        .arg("HEAD")
        .arg("--")
        .arg("src/main.rs")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        .unwrap_or_else(|_| "".to_string());

    // Create complete side-by-side display with diff highlighting
    let (old_lines, new_lines, change_blocks) =
        create_complete_side_by_side_with_diff(&original_content, &current_content, &diff_text);

    // Build enhanced data structures for better functionality
    let line_height = 18.0;
    let anchors = sync::build_anchors_from_blocks(&change_blocks, line_height);
    let mapping_segments = sync::build_mapping_segments(&anchors);

    // Run the application
    eframe::run_native(
        "JetBrains Diff Viewer - Modular",
        options,
        Box::new(|_cc| {
            Box::new(DiffViewerApp::new(
                old_lines,
                new_lines,
                change_blocks,
                anchors,
                mapping_segments,
            ))
        }),
    )
}

struct DiffViewerApp {
    old_lines: Vec<DisplayLine>,
    new_lines: Vec<DisplayLine>,
    change_blocks: Vec<ChangeBlock>,
    anchors: Vec<AnchorPoint>,
    mapping_segments: Vec<MappingSegment>,
    connector_curves: Vec<ConnectorCurve>,
    current_file: usize,
    scroll_sync: ScrollSync,
    theme: JetBrainsTheme,
    line_renderer: LineRenderer,
    connector_renderer: ConnectorRenderer,
}

impl DiffViewerApp {
    fn new(
        old_lines: Vec<DisplayLine>,
        new_lines: Vec<DisplayLine>,
        change_blocks: Vec<ChangeBlock>,
        anchors: Vec<AnchorPoint>,
        mapping_segments: Vec<MappingSegment>,
    ) -> Self {
        let line_height = 18.0;
        let viewport_height = 800.0; // Default, will be updated dynamically
        let theme = JetBrainsTheme::dark_theme();

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
}

impl DiffViewerApp {
    fn handle_keyboard_navigation(&mut self, ctx: &egui::Context) {
        // Keyboard shortcuts for diff navigation (Part 4 specification)
        let input = ctx.input(|i| i.clone());

        // F7: Next diff block
        if input.key_pressed(egui::Key::F7) && !input.modifiers.shift {
            self.navigate_to_next_diff_block();
        }

        // Shift+F7: Previous diff block
        if input.key_pressed(egui::Key::F7) && input.modifiers.shift {
            self.navigate_to_previous_diff_block();
        }

        // Alt+Up: Previous connector
        if input.key_pressed(egui::Key::ArrowUp) && input.modifiers.alt {
            self.navigate_to_previous_connector();
        }

        // Alt+Down: Next connector
        if input.key_pressed(egui::Key::ArrowDown) && input.modifiers.alt {
            self.navigate_to_next_connector();
        }

        // Ctrl+Enter: Apply current hunk
        if input.key_pressed(egui::Key::Enter) && input.modifiers.ctrl {
            self.apply_current_hunk();
        }

        // Ctrl+Backspace: Revert current hunk
        if input.key_pressed(egui::Key::Backspace) && input.modifiers.ctrl {
            self.revert_current_hunk();
        }

        // Ctrl+Shift+S: Stage current hunk
        if input.key_pressed(egui::Key::S) && input.modifiers.ctrl && input.modifiers.shift {
            self.stage_current_hunk();
        }
    }

    fn navigate_to_next_diff_block(&mut self) {
        // Find the next diff block after current scroll position
        let current_y = self.scroll_sync.left_scroll_offset() + 300.0; // Approximate viewport center

        for block in &self.change_blocks {
            // Calculate the center of the block properly
            let block_center =
                (block.start_line as f32 + block.end_line as f32) / 2.0 * self.theme.line_height;

            if block_center > current_y {
                // Scroll to center this block
                let target_scroll = block_center - 300.0;
                self.scroll_sync.set_left_scroll(target_scroll.max(0.0));
                break;
            }
        }
    }

    fn navigate_to_previous_diff_block(&mut self) {
        // Find the previous diff block before current scroll position
        let current_y = self.scroll_sync.left_scroll_offset() + 300.0;

        for block in self.change_blocks.iter().rev() {
            // Calculate the center of the block properly
            let block_center =
                (block.start_line as f32 + block.end_line as f32) / 2.0 * self.theme.line_height;

            if block_center < current_y {
                // Scroll to center this block
                let target_scroll = block_center - 300.0;
                self.scroll_sync.set_left_scroll(target_scroll.max(0.0));
                break;
            }
        }
    }

    fn navigate_to_next_connector(&mut self) {
        self.navigate_to_next_diff_block(); // Same as diff block navigation
    }

    fn navigate_to_previous_connector(&mut self) {
        self.navigate_to_previous_diff_block(); // Same as diff block navigation
    }

    fn apply_current_hunk(&mut self) {
        // TODO: Implement hunk application logic
        // This would apply the current diff block to the working directory
        println!("Apply current hunk - not yet implemented");
    }

    fn revert_current_hunk(&mut self) {
        // TODO: Implement hunk revert logic
        // This would revert the current diff block
        println!("Revert current hunk - not yet implemented");
    }

    fn stage_current_hunk(&mut self) {
        // TODO: Implement hunk staging logic
        // This would stage the current diff block in git
        println!("Stage current hunk - not yet implemented");
    }

    fn update_connector_curves(&mut self) {
        let config = ConnectorConfig::default();
        let mut curves = Vec::new();

        for (i, block) in self.change_blocks.iter().enumerate() {
            // Calculate block centers using proper coordinates with better height calculation
            let block_height =
                (block.end_line - block.start_line + 1) as f32 * self.theme.line_height;

            let left_start_y = block.start_line as f32 * self.theme.line_height;
            let left_center_y = left_start_y + block_height / 2.0;

            let right_start_y = block.start_line as f32 * self.theme.line_height;
            let right_center_y = right_start_y + block_height / 2.0;

            let left_center = Pos2::new(0.0, left_center_y);
            let right_center = Pos2::new(45.0, right_center_y);

            let color = self.theme.get_connector_color(&block.line_type);

            curves.push(ConnectorCurve::new(
                left_center,
                right_center,
                &config,
                color,
                format!("block_{}", i),
            ));
        }

        self.connector_curves = curves;
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

                                let left_scroll_offset = if self.scroll_sync.master_pane()
                                    == MasterPane::Right
                                {
                                    self.scroll_sync.left_scroll_offset()
                                } else {
                                    ui.ctx().memory_mut(|mem| {
                                        mem.data.get_persisted("left_scroll".into()).unwrap_or(0.0)
                                    })
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

                                let right_scroll_offset = if self.scroll_sync.master_pane()
                                    == MasterPane::Left
                                {
                                    self.scroll_sync.right_scroll_offset()
                                } else {
                                    ui.ctx().memory_mut(|mem| {
                                        mem.data.get_persisted("right_scroll".into()).unwrap_or(0.0)
                                    })
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

impl DiffViewerApp {
    // Line rendering is now handled by the LineRenderer module
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

fn create_complete_side_by_side_with_diff(
    original: &str,
    current: &str,
    diff_text: &str,
) -> (Vec<DisplayLine>, Vec<DisplayLine>, Vec<ChangeBlock>) {
    let original_lines: Vec<&str> = original.lines().collect();
    let current_lines: Vec<&str> = current.lines().collect();

    // Parse diff to identify changed sections
    let diff = diff::parser::parse_diff(diff_text);
    let mut changed_lines = std::collections::HashSet::new();

    if !diff.files.is_empty() {
        for hunk in &diff.files[0].hunks {
            for line in &hunk.lines {
                match line {
                    diff::parser::Line::Deletion(content) => {
                        // Mark this line as changed in original
                        for (i, orig_line) in original_lines.iter().enumerate() {
                            if orig_line.trim() == content.trim() {
                                changed_lines.insert(i);
                                break;
                            }
                        }
                    }
                    diff::parser::Line::Addition(content) => {
                        // Mark this line as changed in current
                        for (i, curr_line) in current_lines.iter().enumerate() {
                            if curr_line.trim() == content.trim() {
                                changed_lines.insert(i);
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let mut old_lines = Vec::new();
    let mut new_lines = Vec::new();

    let max_lines = original_lines.len().max(current_lines.len());

    for i in 0..max_lines {
        let orig_line = original_lines.get(i);
        let curr_line = current_lines.get(i);

        match (orig_line, curr_line) {
            (Some(o), Some(c)) => {
                let is_changed = changed_lines.contains(&i);

                // Calculate word-level highlights for modifications
                let word_highlights = if is_changed && o != c {
                    calculate_word_diffs(o, c)
                } else {
                    Vec::new()
                };

                old_lines.push(DisplayLine {
                    content: o.to_string(),
                    line_type: if is_changed && o != c {
                        LineType::Deletion
                    } else {
                        LineType::Context
                    },
                    original_line_num: Some(i + 1),
                    word_highlights: word_highlights.clone(),
                });
                new_lines.push(DisplayLine {
                    content: c.to_string(),
                    line_type: if is_changed && o != c {
                        LineType::Addition
                    } else {
                        LineType::Context
                    },
                    original_line_num: Some(i + 1),
                    word_highlights,
                });
            }
            (Some(o), None) => {
                old_lines.push(DisplayLine {
                    content: o.to_string(),
                    line_type: LineType::Deletion,
                    original_line_num: Some(i + 1),
                    word_highlights: Vec::new(),
                });
                new_lines.push(DisplayLine {
                    content: "".to_string(),
                    line_type: LineType::Empty,
                    original_line_num: None,
                    word_highlights: Vec::new(),
                });
            }
            (None, Some(c)) => {
                old_lines.push(DisplayLine {
                    content: "".to_string(),
                    line_type: LineType::Empty,
                    original_line_num: None,
                    word_highlights: Vec::new(),
                });
                new_lines.push(DisplayLine {
                    content: c.to_string(),
                    line_type: LineType::Addition,
                    original_line_num: Some(i + 1),
                    word_highlights: Vec::new(),
                });
            }
            _ => {}
        }
    }

    // Create change blocks for trapezoidal connectors
    let change_blocks = detect_change_blocks(&old_lines, &new_lines);

    (old_lines, new_lines, change_blocks)
}

fn calculate_word_diffs(original: &str, current: &str) -> Vec<(usize, usize)> {
    let mut highlights = Vec::new();

    if original.is_empty() || current.is_empty() {
        return highlights;
    }

    // Improved word-based diff using simple LCS approach
    let orig_words: Vec<&str> = original.split_whitespace().collect();
    let curr_words: Vec<&str> = current.split_whitespace().collect();

    // Find differences using a simple approach
    let mut orig_idx = 0;
    let mut curr_idx = 0;

    while orig_idx < orig_words.len() && curr_idx < curr_words.len() {
        if orig_words[orig_idx] == curr_words[curr_idx] {
            // Words match, move both indices
            orig_idx += 1;
            curr_idx += 1;
        } else {
            // Words differ, mark the original word as changed
            let word_start = original
                .split_whitespace()
                .take(orig_idx)
                .map(|w| w.len() + 1)
                .sum::<usize>();

            let word_len = orig_words[orig_idx].len();
            highlights.push((word_start, word_start + word_len));

            // Try to find this word later in current
            let mut found = false;
            for j in curr_idx..curr_words.len() {
                if orig_words[orig_idx] == curr_words[j] {
                    curr_idx = j + 1;
                    found = true;
                    break;
                }
            }

            if !found {
                curr_idx += 1;
            }
            orig_idx += 1;
        }
    }

    // Handle remaining words in original
    while orig_idx < orig_words.len() {
        let word_start = original
            .split_whitespace()
            .take(orig_idx)
            .map(|w| w.len() + 1)
            .sum::<usize>();

        let word_len = orig_words[orig_idx].len();
        highlights.push((word_start, word_start + word_len));
        orig_idx += 1;
    }

    highlights
}

fn detect_change_blocks(old_lines: &[DisplayLine], new_lines: &[DisplayLine]) -> Vec<ChangeBlock> {
    let mut blocks = Vec::new();
    let mut i = 0;
    let max_lines = old_lines.len().max(new_lines.len());

    while i < max_lines {
        // Check if this line has any changes in either pane
        let old_has_change = i < old_lines.len() && old_lines[i].line_type != LineType::Context;
        let new_has_change = i < new_lines.len() && new_lines[i].line_type != LineType::Context;

        if old_has_change || new_has_change {
            let start = i;
            let mut end = i;

            // Find the end of this contiguous change block
            while end + 1 < max_lines {
                let next_old_has_change =
                    end + 1 < old_lines.len() && old_lines[end + 1].line_type != LineType::Context;
                let next_new_has_change =
                    end + 1 < new_lines.len() && new_lines[end + 1].line_type != LineType::Context;

                if !next_old_has_change && !next_new_has_change {
                    break;
                }
                end += 1;
            }

            // Calculate actual line ranges for left and right panes
            let left_start = start.min(old_lines.len().saturating_sub(1));
            let left_end = end.min(old_lines.len().saturating_sub(1));
            let right_start = start.min(new_lines.len().saturating_sub(1));
            let right_end = end.min(new_lines.len().saturating_sub(1));

            // Determine the primary block type with better logic
            let mut has_deletions = false;
            let mut has_additions = false;
            let mut has_modifications = false;

            // Check left pane for deletions
            for idx in left_start..=left_end {
                if idx < old_lines.len() && old_lines[idx].line_type == LineType::Deletion {
                    has_deletions = true;
                }
            }

            // Check right pane for additions
            for idx in right_start..=right_end {
                if idx < new_lines.len() && new_lines[idx].line_type == LineType::Addition {
                    has_additions = true;
                }
            }

            // Check for modifications (context lines with word highlights)
            for idx in left_start..=left_end {
                if idx < old_lines.len() && !old_lines[idx].word_highlights.is_empty() {
                    has_modifications = true;
                }
            }

            // Determine block type with priority: deletions > additions > modifications
            let block_type = if has_deletions {
                LineType::Deletion
            } else if has_additions {
                LineType::Addition
            } else if has_modifications {
                LineType::Context // Modifications are marked as Context with highlights
            } else {
                LineType::Context
            };

            blocks.push(ChangeBlock {
                line_type: block_type,
                start_line: left_start,
                end_line: left_end,
                left_rects: Vec::new(),
                right_rects: Vec::new(),
            });

            i = end + 1;
        } else {
            i += 1;
        }
    }

    // Post-process blocks to merge adjacent blocks of the same type
    let mut merged_blocks = Vec::new();
    let mut current_block: Option<ChangeBlock> = None;

    for block in blocks {
        if let Some(ref mut curr) = current_block {
            // Check if we can merge with the current block
            let can_merge =
                curr.line_type == block.line_type && curr.end_line + 1 >= block.start_line;

            if can_merge {
                // Merge the blocks
                curr.end_line = curr.end_line.max(block.end_line);
            } else {
                // Can't merge, push current and start new
                merged_blocks.push(current_block.take().unwrap());
                current_block = Some(block);
            }
        } else {
            current_block = Some(block);
        }
    }

    // Push the last block
    if let Some(block) = current_block {
        merged_blocks.push(block);
    }

    merged_blocks
}
