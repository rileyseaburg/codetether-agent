# Session Message Management Design Document

## Overview

This document defines the design patterns for using `add_message()` and `generate_title()` in the CodeTether agent session system. It covers integration with the chat loop, context management, and persistence strategies.

---

## Current Implementation Analysis

### Existing Methods in `src/session/mod.rs`

```rust
// Adds a message to the session, updates timestamp
pub fn add_message(&mut self, message: Message)

// Generates title from first user message (only if title is None)
pub async fn generate_title(&mut self) -> Result<()>

// Regenerates title even if already set
pub async fn regenerate_title(&mut self) -> Result<()>

// Sets a custom title explicitly
pub fn set_title(&mut self, title: impl Into<String>)

// Clears title to allow regeneration
pub fn clear_title(&mut self)

// Handles context changes with optional title regeneration
pub async fn on_context_change(&mut self, regenerate_title: bool) -> Result<()>
```

### Current Usage in `prompt()` Method

The `prompt()` method currently:
1. Adds user message via `add_message()` before sending to provider
2. Calls `generate_title()` if title is None and this is the first user message
3. Adds assistant response via `add_message()` after receiving completion
4. Saves session to disk after each prompt

---

## Design Decisions

### 1. When to Add Messages

#### **User Messages**
| Scenario | When to Add | Method |
|----------|-------------|--------|
| Interactive chat (TUI) | Immediately on user input, before processing | `add_message()` |
| CLI `run` command | Before calling provider | `add_message()` via `prompt()` |
| A2A worker | On receiving task, before processing | `add_message()` |
| Tool results | After tool execution completes | `add_message()` with `Role::Tool` |
| System prompts | At session creation or context change | `add_message()` with `Role::System` |

#### **Assistant Messages**
| Scenario | When to Add | Method |
|----------|-------------|--------|
| Completion response | Immediately after receiving full response | `add_message()` |
| Streaming responses | After stream completes (accumulate then add) | `add_message()` |
| Tool call requests | When model requests tool execution | `add_message()` with tool call content |

#### **Design Principle: Eager Persistence**
- Messages should be added **before** async operations when possible
- This ensures conversation state is captured even if the operation fails
- Tool results should be added **after** successful execution

### 2. When to Generate Titles

#### **Automatic Title Generation (Recommended)**

```rust
// Trigger conditions for automatic title generation:
1. First user message is added AND title is None
2. Session is being saved for the first time AND title is None
3. Explicit user request to regenerate
```

#### **Title Generation Strategy**

| Approach | Pros | Cons | Recommendation |
|----------|------|------|----------------|
| **A. First-message truncation** (current) | Simple, fast, no extra API call | May not capture intent | ✅ **Default for MVP** |
| **B. LLM-based summarization** | Accurate, captures intent | Requires API call, adds latency | For future enhancement |
| **C. User-defined** | Perfect accuracy | Requires user input | Available as override |

#### **Title Lifecycle**

```
Session Created
      ↓
[title: None]
      ↓
First user message added
      ↓
Auto-generate title (if None) ──→ [title: "First message truncated..."]
      ↓
User can: regenerate, set custom, or clear
```

### 3. Integration with Chat Loop

#### **Current TUI Flow (Needs Update)**

```rust
// CURRENT (in tui/mod.rs) - Messages only in UI state, not persisted
app.messages.push(ChatMessage { ... });

// DESIRED - Integrate with Session
let mut session = Session::new().await?;

// On user submit:
session.add_message(Message {
    role: Role::User,
    content: vec![ContentPart::Text { text: user_input }],
});

// Auto-generate title if needed
if session.title.is_none() {
    session.generate_title().await?;
}

// Get completion
let response = session.prompt(&user_input).await?;
// prompt() internally adds assistant message and saves
```

#### **Recommended Chat Loop Architecture**

```rust
pub struct ChatLoop {
    session: Session,
    provider: Arc<dyn Provider>,
    message_callback: Option<Box<dyn Fn(&Message)>>,
}

impl ChatLoop {
    pub async fn run(&mut self) -> Result<()> {
        loop {
            // 1. Get user input
            let input = self.get_input().await?;
            
            // 2. Add user message immediately (eager persistence)
            self.session.add_message(Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: input.clone() }],
            });
            
            // 3. Generate title if first message
            if self.session.title.is_none() {
                self.session.generate_title().await?;
            }
            
            // 4. Stream response
            let mut stream = self.provider.complete_stream(...).await?;
            let mut assistant_content = String::new();
            
            while let Some(chunk) = stream.next().await {
                match chunk {
                    StreamChunk::Text(text) => {
                        assistant_content.push_str(&text);
                        self.on_stream_text(&text).await?;
                    }
                    StreamChunk::ToolCallStart { id, name } => {
                        // Handle tool call
                    }
                    // ...
                }
            }
            
            // 5. Add assistant message after stream completes
            self.session.add_message(Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text { text: assistant_content }],
            });
            
            // 6. Persist session
            self.session.save().await?;
        }
    }
}
```

### 4. Context Management Integration

#### **Context Change Scenarios**

| Change Type | Action | Title Handling |
|-------------|--------|----------------|
| Directory change | Update `metadata.directory` | Optional: `regenerate_title()` |
| Model change | Update `metadata.model` | No title change |
| Agent switch | Update `agent` field | Optional: `regenerate_title()` |
| New session | Create new `Session` | Auto-generate on first message |

#### **Context Preservation Rules**

```rust
impl Session {
    /// Switch to a new context while preserving conversation history
    pub async fn switch_context(&mut self, new_directory: PathBuf) -> Result<()> {
        // Add system message about context change
        self.add_message(Message {
            role: Role::System,
            content: vec![ContentPart::Text { 
                text: format!("Context changed to: {}", new_directory.display()) 
            }],
        });
        
        // Update metadata
        self.metadata.directory = Some(new_directory);
        self.updated_at = Utc::now();
        
        // Optionally regenerate title if context significantly changed
        // self.regenerate_title().await?;
        
        self.save().await?;
        Ok(())
    }
}
```

### 5. Persistence Strategy

#### **Save Triggers**

| Event | Should Save | Priority |
|-------|-------------|----------|
| User message added | Yes | High |
| Assistant response complete | Yes | High |
| Tool result added | Yes | Medium |
| Title generated/changed | Yes | Medium |
| Context change | Yes | Medium |
| Session metadata update | Yes | Low |

#### **Batching Considerations**

For high-frequency operations, consider:

```rust
pub struct Session {
    // ... existing fields ...
    #[serde(skip)]
    pending_save: bool,
    #[serde(skip)]
    last_save: DateTime<Utc>,
}

impl Session {
    /// Debounced save for batching
    pub async fn save_debounced(&mut self) -> Result<()> {
        self.pending_save = true;
        
        // Save immediately if last save was > 5 seconds ago
        if Utc::now().signed_duration_since(self.last_save).num_seconds() > 5 {
            self.save().await?;
        }
        Ok(())
    }
    
    /// Force immediate save
    pub async fn save(&self) -> Result<()> {
        // ... existing save logic ...
    }
}
```

---

## Implementation Recommendations

### 1. Update TUI to Use Session

**File: `src/tui/mod.rs`**

Replace the in-memory `ChatMessage` vector with proper `Session` integration:

```rust
struct App {
    input: String,
    cursor_position: usize,
    session: Session,  // Replace messages: Vec<ChatMessage>
    scroll: usize,
    show_help: bool,
}

impl App {
    async fn submit_message(&mut self) -> Result<()> {
        if self.input.is_empty() {
            return Ok(());
        }

        let message = std::mem::take(&mut self.input);
        self.cursor_position = 0;

        // Use session.prompt() which handles:
        // - Adding user message
        // - Generating title if needed
        // - Getting completion
        // - Adding assistant message
        // - Saving session
        let result = self.session.prompt(&message).await?;
        
        // Display result in UI
        self.display_response(&result.text);
        
        Ok(())
    }
}
```

### 2. Add Streaming Support to Session

**File: `src/session/mod.rs`**

```rust
impl Session {
    /// Stream a prompt response with callback for UI updates
    pub async fn prompt_streaming<F>(
        &mut self,
        message: &str,
        mut on_chunk: F,
    ) -> Result<SessionResult>
    where
        F: FnMut(&str),
    {
        // Add user message
        self.add_message(Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: message.to_string() }],
        });

        // Generate title if needed
        if self.title.is_none() {
            self.generate_title().await?;
        }

        // Get streaming completion
        let request = self.build_completion_request().await?;
        let stream = self.provider.complete_stream(request).await?;
        
        let mut assistant_text = String::new();
        
        while let Some(chunk) = stream.next().await {
            match chunk {
                StreamChunk::Text(text) => {
                    assistant_text.push_str(&text);
                    on_chunk(&text);
                }
                StreamChunk::Done { usage } => {
                    self.usage.add(&usage);
                }
                StreamChunk::Error(e) => return Err(anyhow::anyhow!(e)),
                _ => {}
            }
        }

        // Add assistant message
        self.add_message(Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text { text: assistant_text.clone() }],
        });

        // Save session
        self.save().await?;

        Ok(SessionResult {
            text: assistant_text,
            session_id: self.id.clone(),
        })
    }
}
```

### 3. Add Session Management Commands

**File: `src/cli/mod.rs`**

Add CLI commands for session management:

```rust
#[derive(Subcommand, Debug)]
pub enum Command {
    // ... existing commands ...
    
    /// Session management
    Session(SessionArgs),
}

#[derive(Parser, Debug)]
pub struct SessionArgs {
    #[command(subcommand)]
    pub action: SessionAction,
}

#[derive(Subcommand, Debug)]
pub enum SessionAction {
    /// List all sessions
    List,
    /// Show session details
    Show { id: String },
    /// Delete a session
    Delete { id: String },
    /// Set session title
    SetTitle { id: String, title: String },
    /// Regenerate session title
    RegenerateTitle { id: String },
}
```

### 4. Title Generation Enhancement

**File: `src/session/mod.rs`**

```rust
impl Session {
    /// Generate title using LLM for better accuracy
    pub async fn generate_title_with_llm(&mut self, provider: &dyn Provider) -> Result<()> {
        if self.title.is_some() {
            return Ok(());
        }

        let first_message = self.messages.iter()
            .find(|m| m.role == Role::User)
            .map(|m| m.content_text())
            .unwrap_or_default();

        if first_message.is_empty() {
            return Ok(());
        }

        // Use LLM to generate concise title
        let prompt = format!(
            "Generate a concise 3-5 word title for this conversation starter:\n{}\n\nTitle:",
            first_message
        );
        
        let title = provider.quick_complete(&prompt).await?;
        self.title = Some(title.trim().to_string());
        self.updated_at = Utc::now();
        
        Ok(())
    }
}
```

---

## Summary

### Key Design Principles

1. **Eager Persistence**: Add messages before async operations
2. **Automatic Title Generation**: Generate on first user message, allow override
3. **Consistent Save Points**: Save after every complete turn (user + assistant)
4. **Context Awareness**: Track context changes with system messages
5. **User Control**: Always allow explicit title setting and regeneration

### Implementation Priority

| Priority | Task | Files |
|----------|------|-------|
| P0 | Integrate Session into TUI | `src/tui/mod.rs` |
| P0 | Add streaming support to Session | `src/session/mod.rs` |
| P1 | Add session management CLI | `src/cli/mod.rs`, `src/cli/session.rs` |
| P1 | Add LLM-based title generation | `src/session/mod.rs` |
| P2 | Add debounced save optimization | `src/session/mod.rs` |
| P2 | Add context switch handling | `src/session/mod.rs` |

### API Usage Patterns

```rust
// Pattern 1: Simple prompt (current)
let mut session = Session::new().await?;
let result = session.prompt("Hello").await?;  // Handles everything internally

// Pattern 2: Manual message management
session.add_message(user_message);
session.generate_title().await?;  // Optional: auto-called in prompt()
let response = provider.complete(request).await?;
session.add_message(response.message);
session.save().await?;

// Pattern 3: Streaming with callbacks
session.prompt_streaming("Hello", |chunk| {
    print!("{}", chunk);
}).await?;

// Pattern 4: Context change
session.switch_context(new_dir).await?;
// Or manually:
session.on_context_change(true).await?;  // true = regenerate title
```
