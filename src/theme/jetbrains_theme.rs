// JetBrains theme implementation
use egui::{Color32, Stroke};

#[derive(Debug, Clone)]
pub struct JetBrainsTheme {
    pub color_green_500: Color32,
    pub color_red_500: Color32,
    pub color_yellow_500: Color32,
    pub color_blue_500: Color32,
    pub gray_50: Color32,
    pub gray_100: Color32,
    pub gray_200: Color32,
    pub gray_300: Color32,
    pub gray_400: Color32,
    pub gray_500: Color32,
    pub gray_600: Color32,
    pub gray_700: Color32,
    pub gray_800: Color32,
    pub gray_900: Color32,
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
    pub scrollbar_track: Color32,
    pub scrollbar_thumb: Color32,
    pub line_numbers: Color32,
    pub line_numbers_active: Color32,
    pub show_line_numbers: bool,
}

impl JetBrainsTheme {
    pub fn dark_theme() -> Self {
        Self {
            color_green_500: Color32::from_rgb(76, 175, 80),
            color_red_500: Color32::from_rgb(244, 67, 54),
            color_yellow_500: Color32::from_rgb(255, 193, 7),
            color_blue_500: Color32::from_rgb(33, 150, 243),
            gray_50: Color32::from_rgb(250, 250, 250),
            gray_100: Color32::from_rgb(245, 245, 245),
            gray_200: Color32::from_rgb(238, 238, 238),
            gray_300: Color32::from_rgb(224, 224, 224),
            gray_400: Color32::from_rgb(189, 189, 189),
            gray_500: Color32::from_rgb(158, 158, 158),
            gray_600: Color32::from_rgb(117, 117, 117),
            gray_700: Color32::from_rgb(97, 97, 97),
            gray_800: Color32::from_rgb(66, 66, 66),
            gray_900: Color32::from_rgb(33, 33, 33),
            font_family: "JetBrains Mono".to_string(),
            font_size: 13.0,
            line_height: 18.0,
            gutter_width: 45.0,
            connector_width: 45.0,
            addition_background: Color32::from_rgba_unmultiplied(76, 175, 80, 64),
            addition_foreground: Color32::from_rgb(129, 199, 132),
            addition_gutter: Color32::from_rgb(76, 175, 80),
            deletion_background: Color32::from_rgba_unmultiplied(244, 67, 54, 64),
            deletion_foreground: Color32::from_rgb(239, 154, 154),
            deletion_gutter: Color32::from_rgb(244, 67, 54),
            modification_background: Color32::from_rgba_unmultiplied(33, 150, 243, 64),
            modification_foreground: Color32::from_rgb(255, 249, 196),
            modification_gutter: Color32::from_rgb(33, 150, 243),
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
            scrollbar_track: Color32::from_rgb(62, 62, 62),
            scrollbar_thumb: Color32::from_rgb(100, 100, 100),
            line_numbers: Color32::from_rgb(153, 153, 153),
            line_numbers_active: Color32::from_rgb(255, 255, 255),
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
