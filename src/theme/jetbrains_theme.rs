// JetBrains theme implementation
use egui::{Color32, Stroke};

#[derive(Debug, Clone)]
pub struct JetBrainsTheme {
    pub color_blue_500: Color32,
    pub font_family: String,
    pub font_size: f32,
    pub line_height: f32,
    pub gutter_width: f32,
    pub connector_width: f32,
    pub addition_background: Color32,
    pub addition_foreground: Color32,
    pub addition_gutter: Color32,
    pub deletion_background: Color32,
    pub deletion_foreground: Color32,
    pub deletion_gutter: Color32,
    pub modification_background: Color32,
    pub modification_foreground: Color32,
    pub modification_gutter: Color32,
    pub code_foreground: Color32,
    pub code_comment: Color32,
    pub code_keyword: Color32,
    pub code_string: Color32,
    pub background: Color32,
    pub foreground: Color32,
    pub border: Color32,
    pub gutter_background: Color32,
    pub gutter_border: Color32,
    pub connector_column: Color32,
    pub line_numbers: Color32,
    pub show_line_numbers: bool,
}

impl JetBrainsTheme {
    pub fn dark_theme() -> Self {
        Self {
            color_blue_500: Color32::from_rgb(33, 150, 243),
            font_family: "JetBrains Mono".to_string(),
            font_size: 13.0,
            line_height: 18.0,
            gutter_width: 45.0,
            connector_width: 45.0,
            addition_background: Color32::from_rgb(52, 85, 52),
            addition_foreground: Color32::from_rgb(129, 199, 132),
            addition_gutter: Color32::from_rgb(52, 85, 52),
            deletion_background: Color32::from_rgba_unmultiplied(113, 113, 113, 64),
            deletion_foreground: Color32::from_rgb(239, 154, 154),
            deletion_gutter: Color32::from_rgb(113, 113, 113),
            modification_background: Color32::from_rgb(50, 66, 98),
            modification_foreground: Color32::from_rgb(255, 249, 196),
            modification_gutter: Color32::from_rgb(50, 66, 98),
            code_foreground: Color32::from_rgb(212, 212, 212),
            code_comment: Color32::from_rgb(106, 153, 85),
            code_keyword: Color32::from_rgb(86, 156, 214),
            code_string: Color32::from_rgb(206, 145, 120),
            background: Color32::from_rgb(30, 30, 30),
            foreground: Color32::from_rgb(212, 212, 212),
            border: Color32::from_rgb(62, 62, 62),
            gutter_background: Color32::from_rgb(37, 37, 38),
            gutter_border: Color32::from_rgb(62, 62, 62),
            connector_column: Color32::from_rgb(45, 45, 45),
            line_numbers: Color32::from_rgb(153, 153, 153),
            show_line_numbers: true,
        }
    }

    pub fn apply_to_context(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = self.background.r() < 128;
        style.visuals.window_fill = self.background;
        style.visuals.panel_fill = self.background;
        style.visuals.faint_bg_color = self.background;
        style.visuals.override_text_color = Some(self.foreground);
        style.visuals.selection.bg_fill = self.modification_background;
        style.visuals.selection.stroke = Stroke::new(1.0, self.modification_background);
        ctx.set_style(style);
    }

    pub fn get_connector_color(&self, line_type: &crate::models::line::LineType) -> Color32 {
        match line_type {
            crate::models::line::LineType::Addition => self.addition_background,
            crate::models::line::LineType::Deletion => self.deletion_background,
            _ => self.modification_background,
        }
    }

    pub fn get_line_background(&self, line_type: &crate::models::line::LineType) -> Color32 {
        match line_type {
            crate::models::line::LineType::Addition => self.addition_background,
            crate::models::line::LineType::Deletion => self.deletion_background,
            crate::models::line::LineType::Context => Color32::TRANSPARENT,
            crate::models::line::LineType::Empty => Color32::TRANSPARENT,
        }
    }
}
