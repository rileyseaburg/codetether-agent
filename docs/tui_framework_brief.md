# TUI Framework Technical Brief

## Framework Overview

**CodeTether Agent** uses **Ratatui** (v0.30.0) with **Crossterm** (v0.29.0) as its TUI framework.

- **Ratatui**: A Rust library for building fast, lightweight terminal user interfaces
- **Crossterm**: Cross-platform terminal manipulation library for input/output handling
- **Architecture**: Immediate-mode rendering with intermediate buffers

---

## 1. Message Display Components Available

### Current Implementation
The project currently uses a `List` widget for message display:

```rust
use ratatui::widgets::{List, ListItem};

let messages: Vec<ListItem> = app
    .messages
    .iter()
    .map(|m| {
        let header = Line::from(vec![
            Span::styled(format!("[{}] ", m.timestamp), Style::default().fg(Color::DarkGray)),
            Span::styled(&m.role, style.add_modifier(Modifier::BOLD)),
        ]);
        let content = Line::from(Span::raw(&m.content));
        ListItem::new(vec![header, content, Line::from("")])
    })
    .collect();
```

### Available Widgets for Message Display

| Widget | Best For | Key Features |
|--------|----------|--------------|
| **`List`** | Simple chat messages | Selection support, highlight styles, scrollable |
| **`Paragraph`** | Rich text content | Text wrapping, alignment, scroll offset, multi-line |
| **`Table`** | Structured data | Columns, headers, footers, cell selection |
| **`Block`** | Container framing | Borders, titles, border styles |

### Recommended Components for Chat Display

#### Option A: Enhanced List (Current + Improvements)
```rust
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::style::{Style, Color, Modifier};

// With stateful selection for message navigation
let mut state = ListState::default();
let list = List::new(items)
    .block(Block::bordered().title("Messages"))
    .highlight_style(Style::new().bg(Color::DarkGray))
    .highlight_symbol("▶ ")
    .scroll_padding(3);  // Keep context around selected item
```

#### Option B: Paragraph with Scroll
```rust
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::text::Text;

let paragraph = Paragraph::new(text)
    .block(Block::bordered())
    .wrap(Wrap { trim: false })  // Preserve whitespace for code
    .scroll((scroll_y, 0));      // Vertical scrolling
```

#### Option C: Hybrid Approach (Recommended)
Combine `List` for message list with `Paragraph` for individual message content:
- Use `List` with `ListItem` containing styled headers
- Each `ListItem` can contain multiple `Line`s for rich formatting
- Use `Scrollbar` widget alongside for visual feedback

---

## 2. Input Handling Mechanisms

### Current Implementation
Uses `crossterm::event` for input handling:

```rust
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

if event::poll(std::time::Duration::from_millis(100))? {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                return Ok(());  // Quit
            }
            KeyCode::Enter => app.submit_message(),
            KeyCode::Char(c) => app.input.insert(app.cursor_position, c),
            KeyCode::Backspace => { /* handle delete */ },
            // ... more handlers
        }
    }
}
```

### Available Input Types

| Event Type | Source | Use Case |
|------------|--------|----------|
| `KeyEvent` | Keyboard | Shortcuts, text input, navigation |
| `MouseEvent` | Mouse | Clicking, scrolling, selection |
| `ResizeEvent` | Terminal | Responsive layout adjustments |

### Key Modifiers Supported
- `KeyModifiers::CONTROL` - Ctrl key
- `KeyModifiers::SHIFT` - Shift key  
- `KeyModifiers::ALT` - Alt/Option key
- Combinations supported (e.g., `Ctrl+Shift+C`)

### Current Key Bindings

| Key | Action |
|-----|--------|
| `Ctrl+C` / `Ctrl+Q` | Quit |
| `?` | Toggle help overlay |
| `Tab` | Switch agent (build/plan) |
| `Enter` | Submit message |
| `↑/↓` | Scroll messages (1 line) |
| `PgUp/PgDn` | Scroll messages (10 lines) |
| `←/→` | Move cursor in input |
| `Home/End` | Jump to input start/end |
| `Backspace/Delete` | Edit input |

### Recommendations for Enhanced Input

1. **Mouse Support**: Enable for clicking messages, scrolling
   ```rust
   execute!(stdout, EnableMouseCapture)?;
   ```

2. **Paste Support**: Handle bracketed paste for multi-line input

3. **History Navigation**: Add `↑`/`↓` for command history when input is empty

---

## 3. Scrollable View Capabilities

### Current Implementation
Simple manual scroll tracking:
```rust
struct App {
    scroll: usize,  // Manual scroll offset
    // ...
}

// Scroll handling
KeyCode::Up => app.scroll = app.scroll.saturating_sub(1),
KeyCode::Down => app.scroll = app.scroll.saturating_add(1),
KeyCode::PageUp => app.scroll = app.scroll.saturating_sub(10),
KeyCode::PageDown => app.scroll = app.scroll.saturating_add(10),
```

### Native Scroll Support

#### Paragraph Scroll
```rust
// Built-in scroll offset (y, x)
let paragraph = Paragraph::new(long_text)
    .scroll((scroll_y, scroll_x));
```

#### List with Scrollbar (Stateful)
```rust
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};

// List handles scrolling automatically with state
let mut list_state = ListState::default();
list_state.select(Some(selected_index));

// Visual scrollbar
let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
    .begin_symbol(Some("↑"))
    .end_symbol(Some("↓"));
    
let mut scrollbar_state = ScrollbarState::new(total_items)
    .position(selected_index);
```

### Scrollbar Widget

```rust
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};

// Vertical scrollbar
let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
    .begin_symbol(Some("↑"))
    .end_symbol(Some("↓"))
    .track_symbol(Some("│"))
    .thumb_symbol(Some("█"));

// State management
let mut state = ScrollbarState::new(content_length)
    .position(current_position);
```

### Recommendations

1. **Use `ListState`** for message list - provides built-in selection and scrolling
2. **Add `Scrollbar` widget** for visual scroll position feedback
3. **Auto-scroll to bottom** on new messages (with user override)
4. **Smooth scrolling** with mouse wheel support

---

## 4. Color/Styling Support

### Current Implementation
```rust
let style = match m.role.as_str() {
    "user" => Style::default().fg(Color::Green),
    "assistant" => Style::default().fg(Color::Blue),
    "system" => Style::default().fg(Color::Yellow),
    _ => Style::default(),
};
```

### Available Colors

#### ANSI Colors (16 standard colors)
```rust
use ratatui::style::Color;

// Standard colors
Color::Black, Color::Red, Color::Green, Color::Yellow,
Color::Blue, Color::Magenta, Color::Cyan, Color::Gray,

// Bright variants
Color::DarkGray, Color::LightRed, Color::LightGreen, Color::LightYellow,
Color::LightBlue, Color::LightMagenta, Color::LightCyan, Color::White,
```

#### Extended Colors
```rust
// 256 indexed colors
Color::Indexed(42)

// True color (24-bit RGB)
Color::Rgb(255, 100, 50)

// From hex (via palette feature)
Color::from_u32(0xFF6432)
```

### Text Modifiers
```rust
use ratatui::style::Modifier;

Modifier::BOLD
Modifier::DIM
Modifier::ITALIC
Modifier::UNDERLINED
Modifier::SLOW_BLINK
Modifier::RAPID_BLINK
Modifier::REVERSED
Modifier::HIDDEN
Modifier::CROSSED_OUT
```

### Style Composition
```rust
use ratatui::style::{Style, Color, Modifier};

// Method chaining
let style = Style::new()
    .fg(Color::Green)
    .bg(Color::Black)
    .add_modifier(Modifier::BOLD);

// Stylize trait shorthand
use ratatui::style::Stylize;
let style = "text".green().on_black().bold();
```

### Recommended Color Scheme for Chat

```rust
// Message type colors
const USER_MSG: Color = Color::Green;
const ASSISTANT_MSG: Color = Color::Cyan;      // More readable than Blue
const SYSTEM_MSG: Color = Color::Yellow;
const ERROR_MSG: Color = Color::LightRed;
const CODE_BLOCK: Color = Color::LightBlue;

// UI colors
const BORDER_ACTIVE: Color = Color::Cyan;
const BORDER_INACTIVE: Color = Color::DarkGray;
const TIMESTAMP: Color = Color::DarkGray;
const HIGHLIGHT: Color = Color::DarkGray;  // Selection background
```

### Advanced Styling Features

#### Border Styling
```rust
use ratatui::widgets::{Block, Borders, BorderType};

Block::bordered()
    .border_style(Style::new().fg(Color::Cyan))
    .border_type(BorderType::Rounded)  // Or: Plain, Double, Thick
```

#### Gradient-like Effects
```rust
// Use different shades for visual hierarchy
Line::from(vec![
    Span::styled("Header", Style::new().fg(Color::White).bold()),
    Span::styled("  •  ", Style::new().fg(Color::DarkGray)),
    Span::styled("subtext", Style::new().fg(Color::Gray)),
])
```

---

## Component Recommendations Summary

### For Chat Display

| Component | Recommendation | Rationale |
|-----------|---------------|-----------|
| **Message List** | `List` with `ListState` | Native selection, scrolling, highlighting |
| **Message Content** | `ListItem` with multiple `Line`s | Rich per-message formatting |
| **Scrollbar** | `Scrollbar` widget | Visual position feedback |
| **Input Area** | `Paragraph` with cursor | Multi-line input support |
| **Help Overlay** | `Clear` + `Paragraph` | Modal popup capability |

### Implementation Pattern

```rust
// Recommended architecture
pub struct ChatApp {
    messages: Vec<ChatMessage>,
    input: String,
    cursor_position: usize,
    
    // State for scrollable widgets
    message_state: ListState,
    scrollbar_state: ScrollbarState,
    
    // Scroll tracking
    scroll_offset: usize,
    auto_scroll: bool,
}

pub struct ChatMessage {
    role: MessageRole,  // User, Assistant, System, Error
    content: String,
    timestamp: DateTime<Local>,
    style: MessageStyle,  // Code block, quote, etc.
}
```

### Dependencies to Consider

Current `Cargo.toml` has:
```toml
ratatui = "0.30.0"
crossterm = "0.29.0"
```

Optional additions:
- `tui-textarea` - Enhanced multi-line text input
- `tui-input` - Input handling helpers
- `ansi-to-tui` - Convert ANSI codes to ratatui spans

---

## References

- [Ratatui Documentation](https://ratatui.rs/)
- [Ratatui API Docs](https://docs.rs/ratatui/0.30.0/)
- [Crossterm Documentation](https://docs.rs/crossterm/0.29.0/)
- [Ratatui Examples](https://github.com/ratatui/ratatui/tree/main/examples)
