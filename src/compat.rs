// Compatibility layer for migrating from egui to gpui
// This provides basic type aliases and stub implementations

use egui;

// Basic type aliases
pub type Color32 = egui::Color32;
pub type Rect = egui::Rect;
pub type Pos2 = egui::Pos2;
pub type Vec2 = egui::Vec2;

// Font related aliases
pub type FontId = egui::FontId;
pub type FontFamily = egui::FontFamily;
pub type FontData = egui::FontData;
pub type FontDefinitions = egui::FontDefinitions;

// Text styling
pub type RichText = egui::RichText;
pub type TextStyle = egui::TextStyle;

// Color32 methods are available on egui::Color32

// UI components aliases
pub type Ui = egui::Ui;
pub type Response = egui::Response;
pub type Context = egui::Context;
pub type Frame = egui::Frame;
pub type Sense = egui::Sense;

pub type ScrollArea = egui::ScrollArea;
pub type Key = egui::Key;

// Additional types
pub type Painter = egui::Painter;
pub type Shape = egui::Shape;
pub type Stroke = egui::Stroke;
pub type Layout = egui::Layout;
pub type Align = egui::Align;
pub type Align2 = egui::Align2;
pub type Mesh = egui::epaint::Mesh;
pub type Vertex = egui::epaint::Vertex;
pub type PathShape = egui::epaint::PathShape;

// Input state alias
pub type InputState = egui::InputState;
pub type Modifiers = egui::Modifiers;
