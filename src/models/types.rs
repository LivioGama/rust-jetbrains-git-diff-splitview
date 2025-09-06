// Data structures and types for the diff viewer

#[derive(Debug, Clone, PartialEq)]
pub enum LineType {
    Context,
    Addition,
    Deletion,
    Empty,
}

#[derive(Debug, Clone)]
pub struct DisplayLine {
    pub content: String,
    pub line_type: LineType,
    pub original_line_num: Option<usize>,
    pub word_highlights: Vec<(usize, usize)>,
}

#[derive(Debug, Clone)]
pub struct ChangeBlock {
    pub line_type: LineType,
    pub start_line: usize,
    pub end_line: usize,
    pub left_rects: Vec<egui::Rect>,
    pub right_rects: Vec<egui::Rect>,
}

#[derive(Debug, Clone)]
pub struct AnchorPoint {
    pub y_left_doc: f32,
    pub y_right_doc: f32,
    pub weight: f32,
    pub block_id: String,
}

#[derive(Debug, Clone)]
pub struct MappingSegment {
    pub left_start: f32,
    pub left_end: f32,
    pub right_start: f32,
    pub right_end: f32,
    pub slope: f32,
    pub left_tangent: f32,
    pub right_tangent: f32,
}

#[derive(Debug, Clone)]
pub struct ConnectorConfig {
    pub column_width: f32,
    pub line_height: f32,
    pub tension_x: f32,
    pub tension_y: f32,
    pub min_control_offset: f32,
    pub max_control_offset: f32,
    pub ribbon_width: f32,
    pub opacity: f32,
}

impl Default for ConnectorConfig {
    fn default() -> Self {
        Self {
            column_width: 45.0,
            line_height: 20.0,
            tension_x: 0.35,
            tension_y: 0.10,
            min_control_offset: 6.0,
            max_control_offset: 14.0,
            ribbon_width: 3.0,
            opacity: 0.4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectorCurve {
    pub start: egui::Pos2,
    pub end: egui::Pos2,
    pub control1: egui::Pos2,
    pub control2: egui::Pos2,
    pub color: egui::Color32,
    pub thickness: f32,
    pub block_height: f32,
    pub block_id: String,
}

impl ConnectorCurve {
    pub fn new(
        left_center: egui::Pos2,
        right_center: egui::Pos2,
        config: &ConnectorConfig,
        color: egui::Color32,
        block_id: String,
    ) -> Self {
        let dy = right_center.y - left_center.y;

        let tx = (config.column_width * config.tension_x)
            .clamp(config.min_control_offset, config.max_control_offset);
        let vy = dy * config.tension_y;

        let control1_x = left_center.x + tx;
        let control2_x = right_center.x - tx;

        Self {
            start: left_center,
            end: right_center,
            control1: egui::Pos2::new(control1_x, left_center.y + vy),
            control2: egui::Pos2::new(control2_x, right_center.y - vy),
            color,
            thickness: config.ribbon_width,
            block_height: 20.0,
            block_id,
        }
    }

    pub fn evaluate(&self, t: f32) -> egui::Pos2 {
        let u = 1.0 - t;
        let u2 = u * u;
        let u3 = u2 * u;
        let t2 = t * t;
        let t3 = t2 * t;

        egui::Pos2::new(
            u3 * self.start.x
                + 3.0 * u2 * t * self.control1.x
                + 3.0 * u * t2 * self.control2.x
                + t3 * self.end.x,
            u3 * self.start.y
                + 3.0 * u2 * t * self.control1.y
                + 3.0 * u * t2 * self.control2.y
                + t3 * self.end.y,
        )
    }
}
