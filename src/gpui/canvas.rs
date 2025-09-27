use ab_glyph::{point, Font, FontArc, Glyph, PxScale, ScaleFont};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

pub struct Canvas<'a> {
    pub width: u32,
    pub height: u32,
    pub data: &'a mut [u8],
}

impl<'a> Canvas<'a> {
    pub fn new(width: u32, height: u32, data: &'a mut [u8]) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn clear(&mut self, color: Color) {
        for chunk in self.data.chunks_exact_mut(4) {
            chunk[0] = color.r;
            chunk[1] = color.g;
            chunk[2] = color.b;
            chunk[3] = color.a;
        }
    }

    pub fn fill_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        if w <= 0.0 || h <= 0.0 {
            return;
        }

        let x0 = x.floor().max(0.0) as i32;
        let y0 = y.floor().max(0.0) as i32;
        let x1 = (x + w).ceil().min(self.width as f32) as i32;
        let y1 = (y + h).ceil().min(self.height as f32) as i32;

        for yy in y0.max(0)..y1.max(0) {
            if yy < 0 || yy as u32 >= self.height {
                continue;
            }
            for xx in x0.max(0)..x1.max(0) {
                if xx < 0 || xx as u32 >= self.width {
                    continue;
                }
                self.blend_pixel(xx as u32, yy as u32, color, 1.0);
            }
        }
    }

    pub fn fill_triangle(&mut self, p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), color: Color) {
        let area = edge(p0, p1, p2);
        if area.abs() <= f32::EPSILON {
            return;
        }

        let min_x = p0.0.min(p1.0).min(p2.0).floor() as i32;
        let max_x = p0.0.max(p1.0).max(p2.0).ceil() as i32;
        let min_y = p0.1.min(p1.1).min(p2.1).floor() as i32;
        let max_y = p0.1.max(p1.1).max(p2.1).ceil() as i32;

        for y in min_y..max_y {
            if y < 0 || y as u32 >= self.height {
                continue;
            }
            for x in min_x..max_x {
                if x < 0 || x as u32 >= self.width {
                    continue;
                }

                let sample = (x as f32 + 0.5, y as f32 + 0.5);
                let w0 = edge(p1, p2, sample);
                let w1 = edge(p2, p0, sample);
                let w2 = edge(p0, p1, sample);

                if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0) || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0) {
                    self.blend_pixel(x as u32, y as u32, color, 1.0);
                }
            }
        }
    }

    pub fn fill_convex_quad(
        &mut self,
        p0: (f32, f32),
        p1: (f32, f32),
        p2: (f32, f32),
        p3: (f32, f32),
        color: Color,
    ) {
        self.fill_triangle(p0, p1, p2, color);
        self.fill_triangle(p0, p2, p3, color);
    }

    pub fn draw_text(
        &mut self,
        font: &FontArc,
        text: &str,
        origin: (f32, f32),
        size: f32,
        color: Color,
    ) {
        if text.is_empty() {
            return;
        }

        let scale = PxScale::from(size);
        let scaled_font = font.as_scaled(scale);
        let mut caret_x = origin.0;
        let mut baseline = origin.1 + scaled_font.ascent();
        let line_height = scaled_font.height() + scaled_font.line_gap();

        for ch in text.chars() {
            if ch == '\n' {
                caret_x = origin.0;
                baseline += line_height;
                continue;
            }

            let mut glyph = scaled_font.scaled_glyph(ch);
            glyph.position = point(caret_x, baseline);
            self.draw_glyph(font, glyph.clone(), color);
            caret_x += scaled_font.h_advance(glyph.id);
        }
    }

    fn draw_glyph(&mut self, font: &FontArc, glyph: Glyph, color: Color) {
        if let Some(outlined) = font.outline_glyph(glyph.clone()) {
            let bounds = outlined.px_bounds();
            outlined.draw(|gx, gy, coverage| {
                let px = bounds.min.x + gx as f32;
                let py = bounds.min.y + gy as f32;
                if px < 0.0 || py < 0.0 || px >= self.width as f32 || py >= self.height as f32 {
                    return;
                }

                self.blend_pixel(px as u32, py as u32, color, coverage);
            });
        }
    }

    fn blend_pixel(&mut self, x: u32, y: u32, color: Color, coverage: f32) {
        let idx = ((y * self.width + x) * 4) as usize;
        let src_a = (color.a as f32 / 255.0) * coverage;
        let inv_a = 1.0 - src_a;

        let r = color.r as f32 * src_a + self.data[idx] as f32 * inv_a;
        let g = color.g as f32 * src_a + self.data[idx + 1] as f32 * inv_a;
        let b = color.b as f32 * src_a + self.data[idx + 2] as f32 * inv_a;
        let a = 255.0 * (src_a + (self.data[idx + 3] as f32 / 255.0) * inv_a);

        self.data[idx] = r.clamp(0.0, 255.0) as u8;
        self.data[idx + 1] = g.clamp(0.0, 255.0) as u8;
        self.data[idx + 2] = b.clamp(0.0, 255.0) as u8;
        self.data[idx + 3] = a.clamp(0.0, 255.0) as u8;
    }
}

fn edge(a: (f32, f32), b: (f32, f32), p: (f32, f32)) -> f32 {
    (p.0 - a.0) * (b.1 - a.1) - (p.1 - a.1) * (b.0 - a.0)
}
