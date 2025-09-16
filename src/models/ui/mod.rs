// diffsplit/src/models/ui/mod.rs
// UI-related data structures and types

use egui::{Color32, Pos2};

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
            tension_x: 0.25, // Reduced for straighter, better-aligned connectors
            tension_y: 0.05, // Reduced for less vertical curve distortion
            min_control_offset: 8.0, // Increased for smoother curves
            max_control_offset: 18.0, // Increased for better curve shape
            ribbon_width: 2.5, // Slightly thinner for cleaner appearance
            opacity: 0.35,   // Slightly reduced for better background blend
        }
    }
}

impl ConnectorConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_column_width(mut self, width: f32) -> Self {
        self.column_width = width;
        self
    }

    pub fn with_ribbon_width(mut self, width: f32) -> Self {
        self.ribbon_width = width;
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }
}

#[derive(Debug, Clone)]
pub struct ConnectorCurve {
    pub start: Pos2,
    pub end: Pos2,
    pub control1: Pos2,
    pub control2: Pos2,
    pub color: Color32,
    pub thickness: f32,
    pub block_height: f32,
    pub block_id: String,
}

impl ConnectorCurve {
    pub fn new(
        left_center: Pos2,
        right_center: Pos2,
        config: &ConnectorConfig,
        color: Color32,
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
            control1: Pos2::new(control1_x, left_center.y + vy),
            control2: Pos2::new(control2_x, right_center.y - vy),
            color,
            thickness: config.ribbon_width,
            block_height: 20.0,
            block_id,
        }
    }

    pub fn with_block_height(mut self, height: f32) -> Self {
        self.block_height = height;
        self
    }

    pub fn evaluate(&self, t: f32) -> Pos2 {
        let u = 1.0 - t;
        let u2 = u * u;
        let u3 = u2 * u;
        let t2 = t * t;
        let t3 = t2 * t;

        Pos2::new(
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

    pub fn length(&self) -> f32 {
        // Approximate curve length using a few sample points
        let mut length = 0.0;
        let mut prev_point = self.evaluate(0.0);

        for i in 1..=10 {
            let t = i as f32 / 10.0;
            let current_point = self.evaluate(t);
            length += (current_point - prev_point).length();
            prev_point = current_point;
        }

        length
    }
}

#[derive(Debug, Clone)]
pub struct RenderState {
    pub left_scroll_offset: f32,
    pub right_scroll_offset: f32,
    pub viewport_height: f32,
    pub total_left_height: f32,
    pub total_right_height: f32,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            left_scroll_offset: 0.0,
            right_scroll_offset: 0.0,
            viewport_height: 1000.0,
            total_left_height: 0.0,
            total_right_height: 0.0,
        }
    }
}

impl RenderState {
    pub fn new(viewport_height: f32) -> Self {
        Self {
            viewport_height,
            ..Default::default()
        }
    }

    pub fn update_scroll(&mut self, left_offset: f32, right_offset: f32) {
        self.left_scroll_offset = left_offset;
        self.right_scroll_offset = right_offset;
    }

    pub fn update_dimensions(&mut self, left_height: f32, right_height: f32) {
        self.total_left_height = left_height;
        self.total_right_height = right_height;
    }
}
