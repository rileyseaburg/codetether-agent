# CodeTether Agent TUI Module Documentation

## Overview

The `src/tui/` directory contains the Terminal User Interface implementation for CodeTether Agent, built using [Ratatui](https://github.com/ratatui/ratatui) (a Rust library for building rich terminal user interfaces) and [crossterm](https://github.com/crossterm-rs/crossterm) (cross-platform terminal manipulation).

## File Organization

```
src/tui/
├── mod.rs              # Main TUI module - entry point, app state, event loop
├── color_palette.rs    # Semantic color system for consistent theming
├── message_formatter.rs # Syntax highlighting and message formatting
├── swarm_view.rs       # Swarm mode UI for parallel sub-agent execution
├── theme.rs            # Theme configuration and serialization
├── theme_utils.rs      # Color support detection and validation
└── token_display.rs    # Token usage and cost display
```

## Public Modules and Exports

### `mod.rs` (Main Entry Point)

**Public Functions:**
- `run(project: Option<PathBuf>) -> Result<()>` - Entry point to run the TUI

**Private Types:**
- `enum MessageType` - Message classification for display
  - `Text(String)`
  - `ToolCall { name, arguments }`
  - `ToolResult { name, output }`

- `enum ViewMode` - Current TUI view
  - `Chat` - Standard chat interface
  - `Swarm` - Parallel execution view

- `struct App` - Main application state
  - Input handling, cursor position
  - Message history and scroll state
  - Agent switching (build/plan)
  - Processing state and tool tracking
  - Session management
  - Swarm mode state

- `struct ChatMessage` - Individual chat message
  - `role`, `content`, `timestamp`, `message_type`

### `message_formatter.rs`

**Public Struct:**
- `struct MessageFormatter`
  - `new(max_width: usize) -> Self`
  - `format_content(&self, content: &str, role: &str) -> Vec<Line<'static>>`

**Features:**
- Syntax highlighting using syntect
- Code block detection and formatting
- Inline markdown formatting (bold, italic, inline code)
- Line wrapping

### `swarm_view.rs`

**Public Types:**
- `enum SwarmEvent` - Events emitted during swarm execution
  - `Started { task, total_subtasks }`
  - `Decomposed { subtasks }`
  - `SubTaskUpdate { id, name, status, agent_name }`
  - `AgentStarted { subtask_id, agent_name, specialty }`
  - `AgentToolCall { subtask_id, tool_name }`
  - `AgentComplete { subtask_id, success, steps }`
  - `StageComplete { stage, completed, failed }`
  - `Complete { success, stats }`
  - `Error(String)`

- `struct SubTaskInfo` - Display information for a subtask
  - `id`, `name`, `status`, `stage`
  - `dependencies`, `agent_name`
  - `current_tool`, `steps`, `max_steps`

- `struct SwarmViewState` - State for swarm view
  - `active`, `task`, `subtasks`
  - `current_stage`, `total_stages`
  - `stats`, `error`, `scroll`, `complete`
  - `new() -> Self`
  - `handle_event(&mut self, event: SwarmEvent)`
  - `progress(&self) -> f64`
  - `status_counts(&self) -> (usize, usize, usize, usize)`

- `fn render_swarm_view(f: &mut Frame, state: &SwarmViewState, area: Rect)`

### `theme.rs`

**Public Struct:**
- `struct Theme` - Serializable theme configuration
  - Color definitions for all UI elements
  - `dark()`, `light()`, `solarized_dark()`, `solarized_light()` presets
  - `get_role_style(&self, role: &str) -> Style`

- `enum ColorDef` - Serializable color definition
  - `Named(String)`
  - `Rgb(u8, u8, u8)`
  - `Indexed(u8)`
  - `to_color() -> Color`

### `theme_utils.rs`

**Public Types:**
- `enum ColorSupport` - Terminal color capability detection
  - `Monochrome`
  - `Ansi8`
  - `Ansi256`
  - `TrueColor`
  - Methods: `supports_rgb()`, `supports_indexed()`, `supports_named()`

- `fn detect_color_support() -> ColorSupport`
- `fn validate_theme(theme: &Theme) -> Theme`

### `color_palette.rs`

**Public Struct:**
- `struct ColorPalette` - Semantic color system
  - `user_message`, `assistant_message`, `system_message`
  - `error`, `timestamp`, `border`
  - `code_block`, `tool_message`, `text`, `background`
  - `dark()`, `light()`, `solarized_dark()`, `solarized_light()`
  - `get_message_color(&self, role: &str) -> Color`

### `token_display.rs`

**Public Struct:**
- `struct TokenDisplay` - Token usage and cost tracking
  - `new() -> Self`
  - `get_context_limit(&self, model: &str) -> Option<u64>`
  - `calculate_cost_for_tokens(...) -> CostEstimate`
  - `create_status_bar(&self, theme: &Theme) -> Line<'_>`
  - `create_detailed_display(&self) -> Vec<String>`

## Key Features

### 1. Swarm Mode UI

The swarm view provides real-time visualization of parallel sub-agent execution:

**Components:**
- **Header**: Shows task name, status counts (pending, running, completed, failed)
- **Stage Progress**: Gauge showing overall completion percentage
- **Subtask List**: Scrollable list of all subtasks with status icons
  - Status icons: ○ pending, ⊘ blocked, ● running, ✓ completed, ✗ failed, ⊗ cancelled, ⏱ timed out
  - Shows agent name and current tool for running tasks
  - Step counter with progress
- **Stats Footer**: Execution time, speedup factor, tool calls, critical path length

**Visual Indicators:**
- Color-coded status (Yellow/Cyan/Green/Red)
- Real-time updates via `SwarmEvent` channel
- Automatic view switching on completion

### 2. Real-Time Tool Streaming

The TUI provides live feedback during tool execution:

**Features:**
- Animated spinner during processing
- Tool call display with name and arguments
- Tool result display with success/failure indicators
- Auto-scroll to show latest activity
- Color-coded tool messages (Magenta)

**Message Flow:**
1. `SessionEvent::Thinking` - Show "Thinking..." indicator
2. `SessionEvent::ToolCallStart` - Display tool call with 🔧 icon
3. `SessionEvent::ToolCallComplete` - Display result with ✓/✗ icon
4. `SessionEvent::TextComplete` - Display assistant response

### 3. Auto-Scroll

Automatic scrolling behavior:

**Implementation:**
- `app.scroll = usize::MAX` set on new messages
- Scroll position calculated as `min(app.scroll, max_scroll)`
- Scrollbar widget shown when content exceeds visible area
- Scroll position tracked per message view

**User Override:**
- Manual scroll keys override auto-scroll
- Returns to bottom with `Ctrl+G`

### 4. Keyboard Navigation

Comprehensive keyboard shortcuts:

**Basic Controls:**
- `Enter` - Send message
- `Tab` - Switch between build/plan agents
- `Ctrl+C` / `Ctrl+Q` - Quit
- `?` - Toggle help overlay
- `Esc` - Close help / Exit swarm view

**Scrolling:**
- `Up/Down` - Scroll messages line by line
- `PageUp/PageDown` - Scroll by page
- `Alt+j/k` - Vim-style scroll down/up
- `Alt+d/u` - Half page down/up
- `Ctrl+g` - Go to top
- `Ctrl+G` - Go to bottom

**Command History:**
- `Ctrl+R` - Search history (matches current input prefix)
- `Ctrl+Up/Down` - Navigate history

**Text Editing:**
- `Left/Right` - Move cursor
- `Home/End` - Jump to start/end
- `Backspace` - Delete backwards
- `Delete` - Delete forwards

## Architecture

### Event Loop

```rust
async fn run_app(terminal: &mut Terminal<...>) -> Result<()> {
    let mut app = App::new();
    let config = Config::load().await?;
    let theme = validate_theme(&config.load_theme());

    loop {
        // 1. Draw UI
        terminal.draw(|f| ui(f, &app, &theme))?;

        // 2. Handle async events (non-blocking)
        if let Some(rx) = &mut app.response_rx {
            if let Ok(event) = rx.try_recv() {
                app.handle_response(event);
            }
        }

        // 3. Handle user input (with timeout)
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // Handle key events...
                }
            }
        }
    }
}
```

### Message Processing

1. User submits message → `App::submit_message()`
2. Create async channel for events
3. Spawn tokio task calling `Session::prompt_with_events()`
4. Main loop receives events via channel
5. Update UI state and redraw

### Swarm Execution Flow

1. User enters `/swarm <task>`
2. Switch to `ViewMode::Swarm`
3. Create `SwarmEvent` channel
4. Spawn tokio task with `SwarmExecutor`
5. Task sends events: Started → Decomposed → AgentStarted → AgentToolCall → AgentComplete → Complete
6. `SwarmViewState::handle_event()` updates state
7. UI redraws with current state
8. On Complete, switch back to Chat view with summary

## Dependencies

- `ratatui` (0.30.0) - Terminal UI framework
- `crossterm` (0.29.0) - Cross-platform terminal control
- `syntect` (5.2.0) - Syntax highlighting
- `chrono` - Timestamp formatting
- `tokio` - Async runtime
- `serde` - Theme serialization

## Styling Conventions

Per AGENTS.md:
- User messages: Default foreground (or Cyan in some themes)
- Assistant messages: Cyan (better readability than Blue)
- System messages: Yellow
- Tool messages: Magenta
- Errors: Red
- Success: Green
- Status: Dim

Use `Stylize` helpers: `"text".cyan().bold()` instead of verbose `Style::default()` construction.
