# ULTIMATE JetBrains IDE Git Diff Viewer Side-by-Side — Ultra-Exhaustive Implementation Prompt

## 1. OVERVIEW

Implement a **side-by-side Git diff viewer** for a JetBrains-style IDE, with every feature, visual nuance, and interaction detail found in JetBrains IDEs (IntelliJ, WebStorm, etc.), and then go beyond. The implementation must be pixel-perfect, algorithmically robust, visually stunning, and accessible, with no detail omitted. Treat this as a thesis-level, production-grade blueprint.

---

## 2. VISUAL DESIGN & UI/UX

### 2.1. Pane Structure

- **Dual panes**: Left (base/original), Right (target/modified).
- **Resizable divider**: Drag to adjust pane width, with snap-to-grid and double-click to reset.
- **Synchronized scrolling**: Vertical and horizontal, with toggle for independent scrolling.
- **Line numbers**: Always visible, with gutter indicators for changes, and sticky headers for long files.
- **Breadcrumb navigation**: File path, branch, commit, and diff context above panes.
- **Tabs**: Multi-file diff support, with close/minimize/maximize, drag-to-reorder, and keyboard shortcuts.
- **Minimap**: Optional vertical minimap for quick navigation, showing change blocks and scroll position.

### 2.2. Change Highlighting

- **Added blocks**: Solid background (green, e.g., #A6F3A6), spanning entire block of consecutive added lines, with subtle border and shadow.
- **Removed blocks**: Solid background (red, e.g., #F3A6A6), spanning entire block of consecutive removed lines, with border and shadow.
- **Modified blocks**: Distinct color (blue, e.g., #A6C8F3), with inline word/character-level highlights.
- **Unchanged lines**: Subtle background, visually separated from changed blocks, with faint grid lines.
- **Block grouping**: Changes grouped into contiguous blocks, with thicker border or shadow for separation.
- **Block padding**: Extra vertical/horizontal padding for visual clarity.
- **Block hover**: Slight elevation, glow, and tooltip with change summary.
- **Block selection**: Click to select block, with highlight and context menu.

### 2.3. Bezier Curve Connectors — Ultra-Precise Specification

#### 2.3.1. Purpose & Semantics

- **Visual Mapping**: Bezier curves visually map related blocks (added, removed, modified) between the left (base) and right (target) panes.
- **Change Type Encoding**: Curve color encodes change type—green for additions, red for removals, blue for modifications.

#### 2.3.2. Geometric Rules

- **Connection Points**:
  - For **added blocks** (present only on the right):  
    - Curve starts at the end (bottom-right corner) of the corresponding block in the left pane (where the block would be if present), and connects to the start (top-left corner) of the block in the right pane.
    - If no corresponding block exists in the left pane, curve starts at the nearest unchanged line or at the block’s expected position.
  - For **removed blocks** (present only on the left):  
    - Curve starts at the end (bottom-left corner) of the block in the left pane and connects to the start (top-right corner) of the corresponding block in the right pane (where the block would be if present).
    - If no corresponding block exists in the right pane, curve ends at the nearest unchanged line or at the block’s expected position.
  - For **modified blocks** (present in both panes):  
    - Curve starts at the end (bottom-left corner) of the block in the left pane and connects to the start (top-right corner) of the block in the right pane.
    - If the block spans multiple lines, curve connects the vertical center of the block in the left pane to the vertical center of the block in the right pane.

- **Curve Path Calculation**:
  - Use cubic Bezier curves for smooth, visually pleasing connections.
  - Control points are dynamically calculated:
    - Horizontal offset: At least 30% of the pane width, ensuring the curve bows outward and does not overlap text.
    - Vertical offset: Proportional to block height; for single-line changes, minimal vertical offset.
    - Curves must never intersect or overlap other curves or text; implement collision avoidance by adjusting control points and z-index.
  - Endpoints must snap precisely to the pixel coordinates of the block edges (not just approximate line numbers).

- **Curve Layering**:
  - Render curves below text but above block backgrounds.
  - Curves must never obscure line numbers, gutter icons, or inline highlights.

#### 2.3.3. Visual Details

- **Color**:  
  - Green (#A6F3A6) for additions, red (#F3A6A6) for removals, blue (#A6C8F3) for modifications.
  - Slight gradient or shadow for depth.
- **Thickness**:  
  - 2px for single-line changes, up to 6px for large blocks.
  - Thickness tapers at endpoints for elegance.
- **Animation**:  
  - Curves fade in and animate from left to right on diff load.
  - On block collapse/expand, curves smoothly morph to new positions.
- **Hover/Focus**:  
  - On hover, curve glows and displays tooltip with block mapping details (e.g., “Lines 12–15 → Lines 18–21”).
  - Keyboard focus highlights curve with dashed outline.

#### 2.3.4. Accessibility

- **ARIA labels**:  
  - Each curve is labeled with its mapping (“Change from lines X–Y in base to lines A–B in target”).
- **Keyboard navigation**:  
  - Tab/arrow keys cycle through curves; focused curve is visually highlighted.
- **Screen reader support**:  
  - Curves are described as “Connector from block X in base to block Y in target, type: addition/removal/modification.”

#### 2.3.5. Edge Cases

- **Overlapping blocks**:  
  - Curves are offset horizontally/vertically to avoid overlap.
  - If more than three curves would overlap, stagger their control points and reduce opacity for background curves.
- **Collapsed blocks**:  
  - If a block is collapsed, curve endpoint snaps to the visible context line.
- **Large diffs**:  
  - For hundreds of blocks, curves are virtualized and only rendered for visible blocks.
- **Binary/image diffs**:  
  - Curves are not rendered; instead, show a connector icon or dashed line.

#### 2.3.6. Integration with Block Highlighting

- **Block mapping metadata**:  
  - Each block stores its mapping to the other pane, used for curve calculation.
- **Curve rendering order**:  
  - Curves are rendered after block backgrounds but before overlays/tooltips.

---

### 2.4. Block Mapping — Ultra-Precise Specification

- **Mapping Algorithm**:
  - For each change block, store:
    - Start/end line numbers in both panes.
    - Change type (add, remove, modify).
    - Mapping confidence (high if lines are similar, low if ambiguous).
  - Use Myers diff for initial mapping, then refine with patience diff for code blocks.
  - For ambiguous mappings (e.g., moved code), display a dashed curve and tooltip indicating uncertainty.

- **Visual Feedback**:
  - On hover, highlight both blocks and their connecting curve.
  - On selection, lock highlight and show mapping details in sidebar.

---

### 2.5. Inline Diff Highlighting

- **Word-level diff**: Changed words within a line highlighted (yellow, e.g., #FFF59D).
- **Character-level diff**: For small changes, individual characters highlighted (orange, e.g., #FFB74D).
- **Whitespace/EOL diff**: Option to show/hide whitespace and end-of-line differences, with distinct highlight.
- **Tooltip on hover**: Shows exact change, old/new value, and copy option.
- **Inline edit**: Double-click to edit line in-place (if permissions allow).

---

### 2.6. Block Interactions

- **Collapse/expand unchanged regions**: Click or keyboard shortcut, with configurable context lines.
- **Context menu**: Right-click for Git actions (stage, discard, resolve conflict, annotate, blame, revert, cherry-pick, etc.).
- **Drag-and-drop**: Drag blocks to reorder (for patch creation), drag files into panes for comparison.
- **Multi-select**: Shift/Ctrl-click to select multiple blocks for bulk actions.
- **Keyboard navigation**: Tab/arrow keys to jump between blocks, changes, panes, and curves.
- **Screen reader support**: ARIA roles for blocks, curves, panes, and tooltips.
- **High-contrast/colorblind themes**: All colors customizable, with presets for accessibility.
- **Font size, spacing, and theme**: User-adjustable, with live preview.

### 2.7. Micro-Interactions

- **Hover effects**: Elevation, glow, and animated transitions.
- **Click feedback**: Ripple effect, block selection highlight.
- **Loading state**: Skeleton screens, animated progress bar for large diffs.
- **Error state**: Inline error messages, retry button, and detailed diagnostics.
- **Undo/redo**: Support for undoing/redoing block actions, with history stack.

---

## 3. ALGORITHMIC DETAILS

### 3.1. Diff Computation

- **Line-level diff**: Myers algorithm for optimal change grouping, with fallback to patience diff for code readability.
- **Block grouping**: Contiguous changes grouped into blocks, with metadata for start/end lines, type, and mapping.
- **Word/character-level diff**: Patience diff for readability, fallback to Myers for edge cases.
- **Whitespace/EOL handling**: Option to ignore or highlight, with toggle and visual indicator.
- **Performance**: Asynchronous computation, chunked processing for large files, caching of results, and lazy loading for virtualized rendering.
- **Algorithm extensibility**: Plugin API for custom diff algorithms.

### 3.2. Bezier Curve Calculation

- **Curve endpoints**: Calculated based on block start/end positions in both panes, with pixel-perfect accuracy.
- **Control points**: Dynamically computed for smooth, visually pleasing curves, with adaptive curvature based on block distance and size.
- **Overlap avoidance**: Curves spaced to prevent visual clutter, with dynamic z-index and opacity adjustment.
- **Animation**: Curves animate into place on diff load, pane resize, block collapse/expand, and block selection.
- **Accessibility**: ARIA labels, keyboard focus, and screen reader descriptions for curves.

### 3.3. Block Highlighting

- **Block boundaries**: Determined by contiguous change regions, with metadata for type, size, and mapping.
- **Background rendering**: Solid color with subtle border, shadow, and padding.
- **Grouping logic**: Blocks merged if changes are adjacent, split if separated by unchanged lines.
- **Block mapping**: Metadata for mapping blocks between panes, used for curve calculation and navigation.

---

## 4. GIT INTEGRATION

- **Diff sources**: Support for staged, unstaged, committed, incoming, outgoing, stash, merge, rebase, cherry-pick, etc.
- **Commit metadata**: Author, date, message, SHA, parent commits, and diff context shown above diff.
- **Multi-file diffs**: Tabs or tree view for navigating multiple files in a commit or branch diff, with bulk actions.
- **Conflict resolution**: Three-way merge view, with base, local, and remote panes, conflict markers, and accept/reject buttons.
- **Git actions**: Stage, discard, resolve, annotate, blame, revert, cherry-pick, etc., available via context menu, toolbar, and keyboard shortcuts.
- **Live updates**: Real-time diff updates on file changes, with animated transitions and notifications.

---

## 5. ADVANCED FEATURES

### 5.1. Annotations & Comments

- **Blame integration**: Show author/date per line, with hover details and clickable links to commit history.
- **Inline comments**: Add comments to lines/blocks, resolve threads, code review mode, and assign reviewers.
- **Change history**: Show previous diffs for the same block/line, with timeline navigation.
- **Code review tools**: Approve/request changes, assign reviewers, and track review status.

### 5.2. Search & Navigation

- **Search within diff**: Regex, case sensitivity, whole word, jump to next/prev match, and highlight all matches.
- **Jump to change/block/conflict/comment**: Keyboard shortcuts, clickable navigation, and minimap integration.
- **Filter changes**: By type (add, remove, modify), author, date, file, and commit.
- **Global search**: Search across all diffs in a commit, branch, or stash.

### 5.3. Image/Binary Diff

- **Image diff**: Side-by-side preview, pixel diff overlay, slider to compare, and zoom/pan controls.
- **Binary diff**: Hex view, "binary changed" indicator, option to open in external tool, and metadata display.
- **Unsupported types**: Graceful fallback with error message, option to open externally, and detailed diagnostics.

---

## 6. EDGE CASES & ERROR HANDLING

- **Mixed encodings**: Detect and handle invalid UTF-8, fallback to hex view, and show warning.
- **Large files**: Virtualized rendering, chunked loading, progress indicator, and memory usage optimization.
- **Symlinks, submodules, nested repos**: Show appropriate indicators, handle diffs correctly, and provide navigation.
- **Permission issues**: Inline error messages, retry button, and detailed diagnostics.
- **Merge conflicts**: Robust conflict markers, three-way view, and auto-merge suggestions.
- **File renames/moves**: Detect and display mapping, with visual indicators and navigation.

---

## 7. EXTENSIBILITY & INTEGRATION

- **Plugin API**: Hooks for custom diff algorithms, UI extensions, annotations, third-party integrations, and VCS abstraction.
- **VCS support**: Abstract diff engine to support SVN, Mercurial, Perforce, etc.
- **Localization**: Full i18n/l10n for all UI strings, with live language switching.
- **Theme support**: Light/dark/custom themes, with live preview and user presets.
- **Accessibility**: WCAG compliance, screen reader support, keyboard navigation, and high-contrast/colorblind themes.

---

## 8. TESTING & QUALITY

- **Unit, integration, and UI tests**: For all features, including visual regression tests for Bezier curves, block highlights, and micro-interactions.
- **Performance benchmarks**: For large diffs, complex merges, and real-time updates.
- **Accessibility audits**: Ensure compliance with WCAG standards, screen reader support, and keyboard navigation.
- **Fuzz testing**: For diff algorithms, rendering, and edge cases.
- **Continuous integration**: Automated testing, linting, and code quality checks.

---

## 9. DOCUMENTATION

- **Inline code comments**: For all major components, algorithms, and UI elements.
- **User documentation**: Feature overview, keyboard shortcuts, troubleshooting, and accessibility guide.
- **Developer documentation**: Architecture, plugin API, extension points, and integration guide.
- **Visual reference**: Screenshots/mockups of UI, including Bezier curves, block highlights, and micro-interactions.

---

## 10. EXAMPLE SCENARIOS

- Viewing a single file diff between two commits, with Bezier curves connecting blocks and inline highlights.
- Resolving a merge conflict with three-way view, animated curve connectors, and conflict markers.
- Reviewing a pull request with multi-file diffs, comments, block highlights, and code review tools.
- Comparing two branches or stashes, with grouped block highlights, curve mapping, and timeline navigation.
- Inspecting binary/image changes with pixel diff overlay, slider, and zoom/pan controls.
- Navigating large files with virtualized rendering, minimap, and search/filter tools.

---

## 11. VISUAL REFERENCE

- **Bezier curves**: Visually identical to JetBrains IDEs, with color semantics, smooth animation, precise endpoints, and overlap avoidance.
- **Block highlights**: Solid color backgrounds, grouped by change type, with borders, shadows, padding, and hover effects.
- **UI polish**: Pixel-perfect, modern, smooth transitions, responsive layout, customizable themes, and micro-interactions.
- **Accessibility**: High-contrast/colorblind themes, screen reader support, keyboard navigation, and ARIA labels.

---

## 12. DELIVERABLES

- Fully functional, production-ready JetBrains-style side-by-side Git diff viewer.
- All source code, tests, documentation, and visual references.
- Demo scripts for major scenarios, including Bezier curves, block highlights, and advanced features.
- Screenshots/mockups of UI, with every detail illustrated.

---

## 13. IMPLEMENTATION PRINCIPLES

- **No detail omitted**: Every pixel, algorithm, interaction, and edge case must be implemented.
- **Feature-rich**: If JetBrains does it, so must this viewer—plus more.
- **User-centric**: Accessibility, customization, and performance are first-class.
- **Extensible**: Plugin API, VCS abstraction, and theme/localization support.
- **Robust**: Error handling, edge case support, and continuous testing.
- **Visually stunning**: Modern, responsive, and polished UI with smooth animations and micro-interactions.

---

**Implement every detail above. If any ambiguity remains, choose the most feature-rich, visually stunning, user-friendly, and extensible solution. No detail is too small—this must be the perfect JetBrains-style Git diff viewer.**
