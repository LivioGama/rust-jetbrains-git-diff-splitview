# Git Diff Viewer Design Principles

## Core Layout Structure

### Two-Pane System
```rust
// Core measurements for the pane system
struct DiffViewer {
    line_height: f32 = 18.0,      // Optimal line spacing
    gutter_width: f32 = 50.0,     // Space between panes
    text_padding: f32 = 5.0,      // Text indentation
    font_size: f32 = 14.0,        // Monospace font size
}

// Pane layout calculation
let available_size = ui.available_size();
let pane_width = (available_size.x - gutter_width) / 2.0;
```

### Visual Design Elements

1. **Color Scheme**
   ```rust
   // GitHub-inspired color palette
   Addition:     RGB(63, 185, 80)   // Soft green
   Deletion:     RGB(248, 81, 73)   // Warm red
   Modification: RGB(55, 71, 82)    // Neutral blueish gray
   ```

2. **Typography**
   ```rust
   // Font configuration
   Code content:   FontId::monospace(14.0)
   Line numbers:   FontId::monospace(12.6)  // 90% of main font
   Text alignment: Align2::LEFT_CENTER
   ```

3. **Change Indicators**
   - Semi-transparent backgrounds (alpha = 40)
   - Clear visual hierarchy
   - Smooth transitions between states

## Interactive Elements

### Connector Design
```rust
struct ChangeBlock {
    change_type: ChangeType,
    left_start_idx: usize,
    left_len: usize,
    right_start_idx: usize,
    right_len: usize,
}

// Connector rendering
fn draw_connectors(&self, painter: &egui::Painter, gutter_rect: Rect) {
    // Creates trapezoid shapes between matching changes
    // Uses semi-transparent fills (alpha = 180)
    // Color-coded by change type
}
```

### Scroll Synchronization
```rust
ScrollArea::vertical()
    .id_source("left_pane")
    .auto_shrink([false, false])
    .show(ui, |ui| {
        // Content rendering with synchronized scrolling
    });
```

## Implementation Details

### 1. Change Block System
- Tracks modifications between files
- Maintains visual connection through gutter
- Supports three change types: Addition, Deletion, Modification

### 2. Layout Management
```rust
// Layout hierarchy
CentralPanel
└─ Horizontal layout
   ├─ Left pane (width = (total - gutter) / 2)
   ├─ Gutter (width = 50px)
   └─ Right pane (width = (total - gutter) / 2)
```

### 3. Responsive Design
- Dynamic width calculation
- Consistent spacing ratios
- Maintains readability at all sizes

### 4. Performance Optimizations
```rust
// State management
line_positions: HashMap<usize, f32>,  // Cache line positions
change_blocks: Vec<ChangeBlock>,      // Track changes efficiently
```

## Best Practices

1. **Visual Clarity**
   - Consistent spacing (18px line height)
   - Clear change indicators
   - Monospace font for code alignment

2. **User Experience**
   - Synchronized scrolling
   - Smooth animations
   - Clear visual feedback

3. **Code Organization**
   - Modular component structure
   - Efficient state management
   - Clear separation of concerns

## Example Usage

```rust
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1200.0, 800.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Diff Viewer Example",
        options,
        Box::new(|_cc| Box::new(DiffViewer::default())),
    )
}
```

This design system prioritizes:
- Clean, distraction-free interface
- Clear visual hierarchy
- Efficient state management
- Smooth, responsive interactions
- Maintainable, modular code structure

The implementation is inspired by modern diff viewers while maintaining its own unique characteristics for optimal user experience.