// Compatibility layer for migrating from egui to gpui
// This provides basic type aliases and stub implementations

use gpui::{Hsla, rgb, rgba};

// Basic type wrappers instead of aliases to avoid orphan rule issues
#[derive(Debug, Clone, Copy)]
pub struct Color32(pub Hsla);

#[derive(Debug, Clone, Copy)]
pub struct Rect(pub gpui::Bounds<f32>);

#[derive(Debug, Clone, Copy)]  
pub struct Pos2(pub gpui::Point<f32>);

#[derive(Debug, Clone, Copy)]
pub struct Vec2(pub gpui::Size<f32>);

// Font related aliases
pub type FontId = String; // Placeholder
pub type FontFamily = String; // Placeholder  
pub type FontData = Vec<u8>;
pub type FontDefinitions = (); // Placeholder

impl Color32 {
    pub const TRANSPARENT: Color32 = Color32(gpui::transparent_black());
    pub const BLACK: Color32 = Color32(gpui::black());
    pub const WHITE: Color32 = Color32(gpui::white());
    
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color32(rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0))
    }
    
    pub fn r(&self) -> u8 {
        // Convert from HSLA to RGB approximation
        128
    }
    
    pub fn g(&self) -> u8 {
        128 
    }
    
    pub fn b(&self) -> u8 {
        128
    }
}

// Stub implementations for UI components
pub struct Ui;
pub struct Response;
pub struct Context;
pub struct Frame;
pub struct Sense;
pub struct Button;
pub struct ScrollArea;
pub struct TextStyle;
pub struct Key;

impl Ui {
    pub fn allocate_exact_size(&mut self, _size: Vec2, _sense: Sense) -> (Rect, Response) {
        (Rect::default(), Response)
    }
    
    pub fn available_width(&self) -> f32 { 800.0 }
    pub fn available_height(&self) -> f32 { 600.0 }
    
    pub fn style_mut(&mut self) -> &mut UiStyle { 
        static mut STYLE: Option<UiStyle> = None;
        unsafe { 
            if STYLE.is_none() {
                STYLE = Some(UiStyle::new());
            }
            STYLE.as_mut().unwrap()
        }
    }
    
    pub fn painter(&self) -> Painter { Painter }
    pub fn horizontal<R>(&mut self, _f: impl FnOnce(&mut Self) -> R) -> R { 
        panic!("Not implemented in gpui migration")
    }
    pub fn vertical<R>(&mut self, _f: impl FnOnce(&mut Self) -> R) -> R { 
        panic!("Not implemented in gpui migration")
    }
    pub fn allocate_ui_with_layout<R>(&mut self, _size: Vec2, _layout: Layout, _f: impl FnOnce(&mut Self) -> R) -> R {
        panic!("Not implemented in gpui migration")
    }
    pub fn allocate_response(&mut self, _size: Vec2, _sense: Sense) -> Response {
        Response
    }
    pub fn add_enabled(&mut self, _enabled: bool, _widget: Button) -> Response {
        Response
    }
    pub fn add_space(&mut self, _amount: f32) {}
    pub fn separator(&mut self) {}
    pub fn input<R>(&self, _f: impl FnOnce(&InputState) -> R) -> R {
        panic!("Not implemented in gpui migration")
    }
}

pub struct UiStyle {
    pub spacing: Spacing,
}

impl UiStyle {
    fn new() -> Self {
        UiStyle {
            spacing: Spacing { item_spacing: Vec2::ZERO }
        }
    }
}

pub struct Spacing {
    pub item_spacing: Vec2,
}

pub struct Painter;
impl Painter {
    pub fn rect_filled(&self, _rect: Rect, _rounding: f32, _color: Color32) {}
    pub fn add(&self, _shape: Shape) {}
}

pub struct InputState;
impl InputState {
    pub fn key_pressed(&self, _key: Key) -> bool { false }
}

pub struct Layout;
impl Layout {
    pub fn left_to_right(_align: Align) -> Self { Layout }
}

pub enum Align { TOP }

pub enum Shape { 
    Mesh(Mesh)
}

pub struct Mesh;

impl Mesh {
    pub fn default() -> Self { Mesh }
    pub fn add_triangle(&mut self, _a: u32, _b: u32, _c: u32) {}
}

impl Default for Mesh {
    fn default() -> Self { Mesh }
}

pub struct Vertex {
    pub pos: Pos2,
    pub uv: Pos2, 
    pub color: Color32,
}

impl Sense {
    pub fn hover() -> Self { Sense }
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2(gpui::Size { width: 0.0, height: 0.0 });
    pub fn new(width: f32, height: f32) -> Self {
        Vec2(gpui::Size { width, height })
    }
}

impl Pos2 {
    pub const ZERO: Pos2 = Pos2(gpui::Point { x: 0.0, y: 0.0 });
    pub fn new(x: f32, y: f32) -> Self {
        Pos2(gpui::Point { x, y })
    }
}

impl Default for Rect {
    fn default() -> Self {
        Rect(gpui::Bounds {
            origin: Pos2::ZERO.0,
            size: Vec2::ZERO.0,
        })
    }
}

impl Rect {
    pub fn from_min_size(origin: Pos2, size: Vec2) -> Self {
        Rect(gpui::Bounds { origin: origin.0, size: size.0 })
    }
    
    pub fn min(&self) -> Pos2 { Pos2(self.0.origin) }
    pub fn max(&self) -> Pos2 { 
        Pos2(gpui::Point { 
            x: self.0.origin.x + self.0.size.width,
            y: self.0.origin.y + self.0.size.height,
        })
    }
    
    pub fn top(&self) -> f32 { self.0.origin.y }
    pub fn bottom(&self) -> f32 { self.0.origin.y + self.0.size.height }
    pub fn height(&self) -> f32 { self.0.size.height }
    pub fn width(&self) -> f32 { self.0.size.width }
}

// Key constants
impl Key {
    pub const ArrowDown: Key = Key;
    pub const ArrowUp: Key = Key;
    pub const ArrowLeft: Key = Key;
    pub const ArrowRight: Key = Key;
    pub const Enter: Key = Key;
    pub const Backspace: Key = Key;
    pub const Space: Key = Key;
    pub const D: Key = Key;
}

impl Button {
    pub fn new(_text: &str) -> Self { Button }
}

impl TextStyle {
    pub const Body: TextStyle = TextStyle;
    pub const Monospace: TextStyle = TextStyle;
    pub const Button: TextStyle = TextStyle;
    pub const Small: TextStyle = TextStyle;
    pub const Heading: TextStyle = TextStyle;
}