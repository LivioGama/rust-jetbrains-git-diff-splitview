use std::rc::Rc;

use anyhow::{anyhow, Context, Result};
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{ElementState, Event, MouseScrollDelta, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::WindowBuilder;

use super::{
    canvas::Canvas,
    event::{GpuiEvent, KeyCode, KeyState},
};
use crate::config::WindowDescriptor;

pub trait GpuiApp {
    fn title(&self) -> &str;
    fn handle_event(&mut self, event: GpuiEvent);
    fn render(&mut self, canvas: &mut Canvas);
}

pub fn run<A: GpuiApp + 'static>(mut app: A, descriptor: WindowDescriptor) -> Result<()> {
    let event_loop = EventLoop::new().context("failed to create event loop")?;

    let logical_size = LogicalSize::new(descriptor.width as f64, descriptor.height as f64);
    let window = WindowBuilder::new()
        .with_title(app.title())
        .with_inner_size(logical_size)
        .with_min_inner_size(LogicalSize::new(
            descriptor.min_width as f64,
            descriptor.min_height as f64,
        ))
        .with_resizable(true)
        .build(&event_loop)
        .context("failed to create window")?;

    let window = Rc::new(window);
    window.set_title(app.title());
    let mut buffer_size = ensure_nonzero(window.inner_size());
    let surface = SurfaceTexture::new(buffer_size.width, buffer_size.height, window.as_ref());
    let mut pixels = Pixels::new(buffer_size.width, buffer_size.height, surface)
        .context("failed to create pixel surface")?;

    let initial_scale = window.scale_factor() as f32;
    app.handle_event(GpuiEvent::Resize {
        width: buffer_size.width,
        height: buffer_size.height,
        scale_factor: initial_scale,
    });

    let window_ref = Rc::clone(&window);

    event_loop
        .run(move |event, target| {
            let window = &window_ref;
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => target.exit(),
                    WindowEvent::KeyboardInput { event, .. } => {
                        if let Some(code) = map_key(&event.logical_key) {
                            let state = if event.state == ElementState::Pressed {
                                KeyState::Pressed
                            } else {
                                KeyState::Released
                            };
                            app.handle_event(GpuiEvent::Key {
                                code,
                                state,
                                repeat: event.repeat,
                            });
                            window.set_title(app.title());
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        let lines = match delta {
                            MouseScrollDelta::LineDelta(_, y) => y,
                            MouseScrollDelta::PixelDelta(pos) => (pos.y as f32) / 40.0,
                        };
                        app.handle_event(GpuiEvent::Scroll {
                            delta_lines: -lines,
                        });
                    }
                    WindowEvent::Resized(new_size) => {
                        if new_size.width > 0 && new_size.height > 0 {
                            buffer_size = new_size;
                            if let Err(err) = pixels.resize_surface(new_size.width, new_size.height)
                            {
                                eprintln!("Failed to resize surface: {err}");
                            }
                            if let Err(err) = pixels.resize_buffer(new_size.width, new_size.height)
                            {
                                eprintln!("Failed to resize buffer: {err}");
                            }
                            let current_scale = window.scale_factor() as f32;
                            app.handle_event(GpuiEvent::Resize {
                                width: new_size.width,
                                height: new_size.height,
                                scale_factor: current_scale,
                            });
                        }
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor: new_scale,
                        ..
                    } => {
                        let adjusted = ensure_nonzero(window.inner_size());
                        buffer_size = adjusted;
                        if let Err(err) = pixels.resize_surface(adjusted.width, adjusted.height) {
                            eprintln!("Failed to resize surface: {err}");
                        }
                        if let Err(err) = pixels.resize_buffer(adjusted.width, adjusted.height) {
                            eprintln!("Failed to resize buffer: {err}");
                        }
                        let current_scale = new_scale as f32;
                        app.handle_event(GpuiEvent::Resize {
                            width: adjusted.width,
                            height: adjusted.height,
                            scale_factor: current_scale,
                        });
                    }
                    WindowEvent::RedrawRequested => {
                        let frame = pixels.frame_mut();
                        let mut canvas = Canvas::new(buffer_size.width, buffer_size.height, frame);
                        app.render(&mut canvas);
                        if let Err(err) = pixels.render() {
                            eprintln!("Render error: {err}");
                            target.exit();
                        }
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    window.request_redraw();
                }
                Event::LoopExiting => {}
                _ => {}
            }
        })
        .map_err(|err| anyhow!(err))
}

fn ensure_nonzero(size: PhysicalSize<u32>) -> PhysicalSize<u32> {
    PhysicalSize::new(size.width.max(1), size.height.max(1))
}

fn map_key(key: &Key) -> Option<KeyCode> {
    match key {
        Key::Named(NamedKey::ArrowUp) => Some(KeyCode::Up),
        Key::Named(NamedKey::ArrowDown) => Some(KeyCode::Down),
        Key::Named(NamedKey::ArrowLeft) => Some(KeyCode::Left),
        Key::Named(NamedKey::ArrowRight) => Some(KeyCode::Right),
        Key::Named(NamedKey::PageUp) => Some(KeyCode::PageUp),
        Key::Named(NamedKey::PageDown) => Some(KeyCode::PageDown),
        Key::Named(NamedKey::Home) => Some(KeyCode::Home),
        Key::Named(NamedKey::End) => Some(KeyCode::End),
        Key::Named(NamedKey::Enter) => Some(KeyCode::Enter),
        Key::Named(NamedKey::Backspace) => Some(KeyCode::Backspace),
        Key::Named(NamedKey::Space) => Some(KeyCode::Space),
        Key::Character(cow) => cow.chars().next().map(KeyCode::Character),
        _ => None,
    }
}
