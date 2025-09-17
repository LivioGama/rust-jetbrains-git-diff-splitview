Font and Display Settings

**From `crates/theme/src/settings.rs`:**
- **UI Font Size**: Default 16px
- **Buffer Font Size**: Default 15px
- **Buffer Line Height**:
  - `Comfortable` (default): 1.618
  - `Standard`: 1.3
  - `Custom`: Any value ≥ 1.0
- **Font Size Limits**: Min 6px, Max 100px

## Language/Buffer Settings

**From `crates/language/src/language_settings.rs`:**
- **Tab Size**: Default 4 columns
- **Hard Tabs**: Default `false` (uses spaces)
- **Preferred Line Length**: Default 80 columns
- **Soft Wrap**: Default `none`
- **Show Wrap Guides**: Default `true`
- **Format on Save**: Default `on`
- **Remove Trailing Whitespace on Save**: Default `true`
- **Ensure Final Newline on Save**: Default `true`

## Editor Behavior Settings

**From `crates/editor/src/editor_settings.rs`:**
- **Cursor Blink**: Default `true`
- **Current Line Highlight**: Default `all`
- **Selection Highlight**: Default `true`
- **Rounded Selection**: Default `true`
- **LSP Highlight Debounce**: Default 75ms
- **Hover Popover Enabled**: Default `true`
- **Hover Popover Delay**: Default 300ms
- **Scroll Beyond Last Line**: Default `one_page`
- **Vertical Scroll Margin**: Default 3 lines
- **Horizontal Scroll Margin**: Default 5 characters
- **Scroll Sensitivity**: Default 1.0
- **Fast Scroll Sensitivity**: Default 4.0 (with alt/option key)
- **Relative Line Numbers**: Default `false`
- **Search Wrap**: Default `true`
- **Middle Click Paste**: Default `true`
- **Minimum Contrast for Highlights**: Default 45 (APCA perceptual contrast)

## Additional Specifications

- **Multi-cursor Modifier**: Default `alt`
- **Seed Search Query from Cursor**: Default `always`
- **Auto Signature Help**: Default `false`
- **Show Signature Help After Edits**: Default `false`
- **Go to Definition Fallback**: Default `FindAllReferences`
- **Inline Code Actions**: Default `true`
- **Drag and Drop Selection**: Default enabled with 300ms delay
- **LSP Document Colors**: Default `inlay
