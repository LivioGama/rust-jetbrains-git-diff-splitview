// src/rendering/render_context.rs
// Rendering context extracted from rendering/mod.rs

use crate::theme::JetBrainsTheme;

/// Rendering context for managing rendering state
pub struct RenderContext {
    pub theme: JetBrainsTheme,
    pub line_height: f32,
    pub font_size: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

impl RenderContext {
    pub fn new(theme: JetBrainsTheme, viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            theme,
            line_height: 18.0,
            font_size: 14.0,
            viewport_width,
            viewport_height,
        }
    }

    pub fn update_viewport(&mut self, width: f32, height: f32) {
        self.viewport_width = width;
        self.viewport_height = height;
    }
}
