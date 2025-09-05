use eframe::egui;
use egui::{Color32, FontId, Pos2, Rect, ScrollArea, Shape, Stroke, Vec2};

mod diff_parser;

#[derive(Debug, Clone, PartialEq)]
enum LineType {
    Context,
    Addition,
    Deletion,
    Empty,
}

#[derive(Debug, Clone)]
struct DisplayLine {
    content: String,
    line_type: LineType,
    original_line_num: Option<usize>,
    word_highlights: Vec<(usize, usize)>, // Character ranges for word-level highlighting
}

#[derive(Debug, Clone)]
struct ChangeBlock {
    line_type: LineType,
    start_line: usize,
    end_line: usize,
    left_rects: Vec<Rect>,
    right_rects: Vec<Rect>,
}

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

    eframe::run_native(
        "JetBrains Diff Viewer",
        options,
        Box::new(|_cc| Box::new(DiffViewerApp::new(old_lines, new_lines, change_blocks))),
    )
}

struct DiffViewerApp {
    old_lines: Vec<DisplayLine>,
    new_lines: Vec<DisplayLine>,
    change_blocks: Vec<ChangeBlock>,
    current_file: usize,
    left_scroll_offset: f32,
    right_scroll_offset: f32,
}

impl DiffViewerApp {
    fn new(
        old_lines: Vec<DisplayLine>,
        new_lines: Vec<DisplayLine>,
        change_blocks: Vec<ChangeBlock>,
    ) -> Self {
        Self {
            old_lines,
            new_lines,
            change_blocks,
            current_file: 0,
            left_scroll_offset: 0.0,
            right_scroll_offset: 0.0,
        }
    }
}

impl eframe::App for DiffViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // JetBrains dark theme colors
        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.window_fill = Color32::from_rgb(43, 43, 43);
        style.visuals.panel_fill = Color32::from_rgb(43, 43, 43);
        style.visuals.faint_bg_color = Color32::from_rgb(43, 43, 43);
        ctx.set_style(style);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(43, 43, 43)))
            .show(ctx, |ui| {
                {
                    let old_lines = &self.old_lines;
                    let new_lines = &self.new_lines;

                    // Create a horizontal layout for the two panes
                    ui.horizontal(|ui| {
                        let total_width = ui.available_width();
                        let pane_width = (total_width - 40.0) / 2.0; // 40px for gutter

                        // Left pane (original)
                        ui.vertical(|ui| {
                            ui.set_width(pane_width);
                            ui.set_height(ui.available_height());

                            // Header
                            ui.horizontal(|ui| {
                                ui.add_space(10.0);
                                ui.label(
                                    egui::RichText::new("Original")
                                        .font(FontId::proportional(14.0))
                                        .color(Color32::from_rgb(187, 187, 187)),
                                );
                            });

                            ui.separator();

                            // Content area with scrolling and synchronization
                            ui.allocate_ui(Vec2::new(ui.available_width(), 1200.0), |ui| {
                                let scroll_area = ScrollArea::vertical()
                                    .id_source("left_pane")
                                    .auto_shrink([false, false]);

                                let scroll_output = scroll_area.show(ui, |ui| {
                                    let mut line_rects = Vec::new();

                                    for (line_idx, line) in self.old_lines.iter().enumerate() {
                                        let rect = self.render_line(ui, line, line_idx, true);
                                        line_rects.push(rect);
                                    }

                                    // Store line positions for drawing connections
                                    ui.ctx().memory_mut(|mem| {
                                        mem.data.insert_persisted("left_rects".into(), line_rects);
                                    });
                                });

                                // Store scroll offset for synchronization
                                ui.ctx().memory_mut(|mem| {
                                    mem.data.insert_persisted(
                                        "left_scroll".into(),
                                        scroll_output.state.offset.y,
                                    );
                                });
                            });
                        });

                        // Middle gutter for connections
                        ui.vertical(|ui| {
                            ui.set_width(40.0);
                            ui.set_height(ui.available_height());

                            // Draw connection lines
                            self.draw_connection_lines(ui, &old_lines, &new_lines);
                        });

                        // Right pane (modified)
                        ui.vertical(|ui| {
                            ui.set_width(pane_width);
                            ui.set_height(ui.available_height());

                            // Header
                            ui.horizontal(|ui| {
                                ui.add_space(10.0);
                                ui.label(
                                    egui::RichText::new("Modified")
                                        .font(FontId::proportional(14.0))
                                        .color(Color32::from_rgb(187, 187, 187)),
                                );
                            });

                            ui.separator();

                            // Content area with scrolling and synchronization
                            ui.allocate_ui(Vec2::new(ui.available_width(), 1200.0), |ui| {
                                // Get left scroll offset for synchronization
                                let left_scroll: Option<f32> = ui
                                    .ctx()
                                    .memory_mut(|mem| mem.data.get_persisted("left_scroll".into()));

                                let mut scroll_area = ScrollArea::vertical()
                                    .id_source("right_pane")
                                    .auto_shrink([false, false]);

                                // Synchronize scroll position
                                if let Some(left_offset) = left_scroll {
                                    scroll_area =
                                        scroll_area.scroll_offset(Vec2::new(0.0, left_offset));
                                }

                                let scroll_output = scroll_area.show(ui, |ui| {
                                    let mut line_rects = Vec::new();

                                    for (line_idx, line) in self.new_lines.iter().enumerate() {
                                        let rect = self.render_line(ui, line, line_idx, false);
                                        line_rects.push(rect);
                                    }

                                    // Store line positions for drawing connections
                                    ui.ctx().memory_mut(|mem| {
                                        mem.data.insert_persisted("right_rects".into(), line_rects);
                                    });
                                });

                                // Store scroll offset for synchronization
                                ui.ctx().memory_mut(|mem| {
                                    mem.data.insert_persisted(
                                        "right_scroll".into(),
                                        scroll_output.state.offset.y,
                                    );
                                });
                            });
                        });
                    });
                }
            });
    }
}

impl DiffViewerApp {
    fn render_line(
        &self,
        ui: &mut egui::Ui,
        line: &DisplayLine,
        _line_idx: usize,
        _is_left: bool,
    ) -> Rect {
        let line_height = 18.0;
        let available_width = ui.available_width();
        let (rect, _response) = ui.allocate_exact_size(
            Vec2::new(available_width, line_height),
            egui::Sense::hover(),
        );

        // JetBrains-style background colors for diff highlighting
        let bg_color = match line.line_type {
            LineType::Deletion => Color32::from_rgb(255, 235, 235), // JetBrains red for deletions
            LineType::Addition => Color32::from_rgb(235, 255, 235), // JetBrains green for additions
            LineType::Context => Color32::from_rgb(43, 43, 43),     // Default dark background
            LineType::Empty => Color32::TRANSPARENT,
        };

        // Fill background for the entire line
        if bg_color != Color32::TRANSPARENT {
            ui.painter().rect_filled(rect, 0.0, bg_color);
        }

        // Render word-level highlights for modifications
        if line.line_type == LineType::Context && !line.word_highlights.is_empty() {
            for (start, end) in &line.word_highlights {
                if *start < line.content.len() && *end <= line.content.len() && *start < *end {
                    let char_width = 8.0; // Approximate character width
                    let highlight_start_x = 60.0 + (*start as f32 * char_width);
                    let highlight_width = (*end - *start) as f32 * char_width;

                    let highlight_rect = Rect::from_min_size(
                        rect.min + Vec2::new(highlight_start_x, 0.0),
                        Vec2::new(highlight_width, line_height),
                    );

                    // Word-level highlight with JetBrains blue for modifications
                    ui.painter().rect_filled(
                        highlight_rect,
                        0.0,
                        Color32::from_rgb(187, 222, 251), // JetBrains blue for modifications
                    );
                }
            }
        }

        // Render line number (only for non-empty lines)
        if let Some(line_num) = line.original_line_num {
            let line_num_rect = Rect::from_min_size(rect.min, Vec2::new(40.0, line_height));

            ui.painter().text(
                line_num_rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("{:4}", line_num),
                FontId::monospace(11.0),
                Color32::from_rgb(128, 128, 128),
            );
        }

        // Render change indicator (only for additions/deletions)
        let indicator = match line.line_type {
            LineType::Deletion => "−",
            LineType::Addition => "+",
            _ => "",
        };

        if !indicator.is_empty() {
            let indicator_rect = Rect::from_min_size(
                rect.min + Vec2::new(45.0, 0.0),
                Vec2::new(15.0, line_height),
            );

            let indicator_color = match line.line_type {
                LineType::Deletion => Color32::from_rgb(244, 67, 54), // JetBrains red
                LineType::Addition => Color32::from_rgb(76, 175, 80), // JetBrains green
                _ => Color32::WHITE,
            };

            ui.painter().text(
                indicator_rect.center(),
                egui::Align2::CENTER_CENTER,
                indicator,
                FontId::monospace(12.0),
                indicator_color,
            );
        }

        // Render code content with syntax highlighting
        if !line.content.is_empty() {
            let content_start_x = 60.0;
            let content_rect = Rect::from_min_size(
                rect.min + Vec2::new(content_start_x, 0.0),
                Vec2::new(rect.width() - content_start_x, line_height),
            );

            let text_color = self.get_text_color(&line.content, &line.line_type);

            // Override text color for highlighted lines to ensure readability
            let final_text_color = match line.line_type {
                LineType::Deletion => Color32::from_rgb(211, 47, 47), // JetBrains red text
                LineType::Addition => Color32::from_rgb(56, 142, 60), // JetBrains green text
                _ => text_color,
            };

            ui.painter().text(
                content_rect.min + Vec2::new(5.0, line_height / 2.0),
                egui::Align2::LEFT_CENTER,
                &line.content,
                FontId::monospace(12.0),
                final_text_color,
            );
        }

        rect
    }

    fn get_text_color(&self, content: &str, line_type: &LineType) -> Color32 {
        // Syntax highlighting
        let trimmed = content.trim();

        if trimmed.starts_with("//") || trimmed.starts_with("#") {
            Color32::from_rgb(98, 151, 85) // Comment green
        } else if trimmed.contains("fn ") || trimmed.contains("function ") {
            Color32::from_rgb(255, 198, 109) // Function yellow
        } else if content.contains("\"") {
            Color32::from_rgb(152, 118, 170) // String purple
        } else if trimmed.starts_with("use ") || trimmed.contains("import ") {
            Color32::from_rgb(204, 120, 50) // Import orange
        } else {
            match line_type {
                LineType::Deletion => Color32::from_rgb(255, 182, 182),
                LineType::Addition => Color32::from_rgb(182, 255, 182),
                _ => Color32::from_rgb(169, 183, 198), // Default text
            }
        }
    }

    fn draw_change_block_connection(
        &self,
        ui: &mut egui::Ui,
        change_type: &str,
        left_start: usize,
        left_end: usize,
        right_start: usize,
        right_end: usize,
        left_rects: &[Rect],
        right_rects: &[Rect],
    ) {
        if left_start >= left_rects.len() || right_start >= right_rects.len() {
            return;
        }

        let painter = ui.painter();

        // Get the color based on change type
        let stroke_color = match change_type {
            "addition" => Color32::from_rgb(80, 160, 80), // Green for additions
            "deletion" => Color32::from_rgb(160, 80, 80), // Red for deletions
            "modification" => Color32::from_rgb(160, 160, 80), // Yellow for modifications
            _ => Color32::from_rgb(100, 150, 200),        // Default blue
        };

        // Calculate the vertical span of the change blocks
        let left_top = left_rects[left_start].min.y;
        let left_bottom = left_rects[left_end.min(left_rects.len() - 1)].max.y;
        let right_top = right_rects[right_start].min.y;
        let right_bottom = right_rects[right_end.min(right_rects.len() - 1)].max.y;

        // Connect the top boundaries
        let start_top = Pos2::new(left_rects[left_start].max.x, left_top);
        let end_top = Pos2::new(right_rects[right_start].min.x, right_top);

        // Connect the bottom boundaries
        let start_bottom = Pos2::new(
            left_rects[left_end.min(left_rects.len() - 1)].max.x,
            left_bottom,
        );
        let end_bottom = Pos2::new(
            right_rects[right_end.min(right_rects.len() - 1)].min.x,
            right_bottom,
        );

        // Draw curved connection lines for the band
        let control_distance = 40.0;

        // Top curve
        let control1_top = Pos2::new(start_top.x + control_distance, start_top.y);
        let control2_top = Pos2::new(end_top.x - control_distance, end_top.y);

        painter.add(Shape::CubicBezier(egui::epaint::CubicBezierShape {
            points: [start_top, control1_top, control2_top, end_top],
            closed: false,
            fill: Color32::TRANSPARENT,
            stroke: Stroke::new(2.0, stroke_color),
        }));

        // Bottom curve
        let control1_bottom = Pos2::new(start_bottom.x + control_distance, start_bottom.y);
        let control2_bottom = Pos2::new(end_bottom.x - control_distance, end_bottom.y);

        painter.add(Shape::CubicBezier(egui::epaint::CubicBezierShape {
            points: [start_bottom, control1_bottom, control2_bottom, end_bottom],
            closed: false,
            fill: Color32::TRANSPARENT,
            stroke: Stroke::new(2.0, stroke_color),
        }));

        // Draw vertical connecting lines to create a band effect
        if (left_bottom - left_top).abs() > 1.0 {
            painter.add(Shape::line_segment(
                [start_top, start_bottom],
                Stroke::new(1.0, stroke_color),
            ));
        }

        if (right_bottom - right_top).abs() > 1.0 {
            painter.add(Shape::line_segment(
                [end_top, end_bottom],
                Stroke::new(1.0, stroke_color),
            ));
        }
    }

    fn draw_connection_lines(
        &self,
        ui: &mut egui::Ui,
        old_lines: &[DisplayLine],
        new_lines: &[DisplayLine],
    ) {
        // Get stored rectangle positions
        let left_rects: Option<Vec<Rect>> = ui
            .ctx()
            .memory_mut(|mem| mem.data.get_persisted("left_rects".into()));

        let right_rects: Option<Vec<Rect>> = ui
            .ctx()
            .memory_mut(|mem| mem.data.get_persisted("right_rects".into()));

        if let (Some(left_rects), Some(right_rects)) = (left_rects, right_rects) {
            let painter = ui.painter();

            // Find all modified blocks
            let mut left_blocks = Vec::new();
            let mut right_blocks = Vec::new();

            // Find left pane blocks
            let mut i = 0;
            while i < old_lines.len() {
                if old_lines[i].line_type == LineType::Deletion {
                    let start = i;
                    let mut end = i;
                    while end + 1 < old_lines.len()
                        && old_lines[end + 1].line_type == LineType::Deletion
                    {
                        end += 1;
                    }
                    left_blocks.push((start, end));
                    i = end + 1;
                } else {
                    i += 1;
                }
            }

            // Find right pane blocks
            let mut j = 0;
            while j < new_lines.len() {
                if new_lines[j].line_type == LineType::Addition {
                    let start = j;
                    let mut end = j;
                    while end + 1 < new_lines.len()
                        && new_lines[end + 1].line_type == LineType::Addition
                    {
                        end += 1;
                    }
                    right_blocks.push((start, end));
                    j = end + 1;
                } else {
                    j += 1;
                }
            }

            // Draw sharp trapezoidal connector blocks - JetBrains style
            let max_blocks = left_blocks.len().max(right_blocks.len());
            for block_idx in 0..max_blocks {
                if let (Some((left_start, left_end)), Some((right_start, right_end))) =
                    (left_blocks.get(block_idx), right_blocks.get(block_idx))
                {
                    if *left_start < left_rects.len() && *right_start < right_rects.len() {
                        let left_first_rect = &left_rects[*left_start];
                        let left_last_rect = &left_rects[*left_end];
                        let right_first_rect = &right_rects[*right_start];
                        let right_last_rect = &right_rects[*right_end];

                        // Determine change type for color
                        let has_deletion = old_lines[*left_start].line_type == LineType::Deletion;
                        let has_addition = new_lines[*right_start].line_type == LineType::Addition;

                        let connector_color = if has_deletion && has_addition {
                            Color32::from_rgb(187, 222, 251) // JetBrains blue for modifications
                        } else if has_deletion {
                            Color32::from_rgb(255, 235, 235) // JetBrains red for deletions
                        } else {
                            Color32::from_rgb(235, 255, 235) // JetBrains green for additions
                        };

                        // Create precise trapezoid vertices
                        let trapezoid_points = [
                            Pos2::new(left_first_rect.max.x, left_first_rect.min.y), // Top-left of left block
                            Pos2::new(left_last_rect.max.x, left_last_rect.max.y), // Bottom-left of left block
                            Pos2::new(right_last_rect.min.x, right_last_rect.max.y), // Bottom-right of right block
                            Pos2::new(right_first_rect.min.x, right_first_rect.min.y), // Top-right of right block
                        ];

                        // Draw filled trapezoid with semi-transparent fill
                        painter.add(Shape::convex_polygon(
                            trapezoid_points.to_vec(),
                            connector_color.gamma_multiply(0.7), // Semi-transparent fill
                            Stroke::NONE,                        // No border for clean look
                        ));
                    }
                }
            }
        }
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
    let diff = diff_parser::parse_diff(diff_text);
    let mut changed_lines = std::collections::HashSet::new();

    if !diff.files.is_empty() {
        for hunk in &diff.files[0].hunks {
            for line in &hunk.lines {
                match line {
                    diff_parser::Line::Deletion(content) => {
                        // Mark this line as changed in original
                        for (i, orig_line) in original_lines.iter().enumerate() {
                            if orig_line.trim() == content.trim() {
                                changed_lines.insert(i);
                                break;
                            }
                        }
                    }
                    diff_parser::Line::Addition(content) => {
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
    // Simple word-level diff - highlight different words
    let orig_words: Vec<&str> = original.split_whitespace().collect();
    let curr_words: Vec<&str> = current.split_whitespace().collect();

    let mut highlights = Vec::new();
    let mut orig_pos = 0;

    for (i, orig_word) in orig_words.iter().enumerate() {
        if i < curr_words.len() && *orig_word != curr_words[i] {
            // Find the position of this word in the original string
            if let Some(start) = original[orig_pos..].find(orig_word) {
                let actual_start = orig_pos + start;
                highlights.push((actual_start, actual_start + orig_word.len()));
            }
        }
        orig_pos += orig_word.len() + 1; // +1 for space
    }

    highlights
}

fn detect_change_blocks(old_lines: &[DisplayLine], new_lines: &[DisplayLine]) -> Vec<ChangeBlock> {
    let mut blocks = Vec::new();
    let mut i = 0;

    while i < old_lines.len() {
        // Check if this line has any changes
        let has_change = old_lines[i].line_type != LineType::Context
            || new_lines
                .get(i)
                .map_or(false, |l| l.line_type != LineType::Context);

        if has_change {
            let start = i;
            let mut end = i;

            // Find the end of this contiguous change block
            while end + 1 < old_lines.len() {
                let next_has_change = old_lines[end + 1].line_type != LineType::Context
                    || new_lines
                        .get(end + 1)
                        .map_or(false, |l| l.line_type != LineType::Context);

                if !next_has_change {
                    break;
                }
                end += 1;
            }

            // Determine the primary block type (deletions take precedence)
            let block_type =
                if (start..=end).any(|idx| old_lines[idx].line_type == LineType::Deletion) {
                    LineType::Deletion
                } else if (start..=end).any(|idx| {
                    new_lines
                        .get(idx)
                        .map_or(false, |l| l.line_type == LineType::Addition)
                }) {
                    LineType::Addition
                } else {
                    LineType::Context
                };

            blocks.push(ChangeBlock {
                line_type: block_type,
                start_line: start,
                end_line: end,
                left_rects: Vec::new(),
                right_rects: Vec::new(),
            });

            i = end + 1;
        } else {
            i += 1;
        }
    }

    blocks
}
