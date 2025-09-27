#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyState {
    Pressed,
    Released,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyCode {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
    Enter,
    Backspace,
    Space,
    Character(char),
}

#[derive(Clone, Debug)]
pub enum GpuiEvent {
    Key {
        code: KeyCode,
        state: KeyState,
        repeat: bool,
    },
    Scroll {
        delta_lines: f32,
    },
    Resize {
        width: u32,
        height: u32,
        scale_factor: f32,
    },
}
