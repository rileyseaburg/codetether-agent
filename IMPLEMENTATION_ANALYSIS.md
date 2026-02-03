# CodeTether Agent - Core Implementation Analysis

## Phase 1: Core Functionality (P0)

Based on my analysis of the codebase, here's what needs to be implemented for the three core priorities:

---

## 1. TUI Session Integration (CRITICAL - Currently Non-Functional)

### Current State
The TUI (`src/tui/mod.rs`) is essentially a mock UI that doesn't actually process messages:

```rust
// Line 98-103 in src/tui/mod.rs:
// TODO: Actually process the message with the agent
self.messages.push(ChatMessage {
    role: "assistant".to_string(),
    content: format!("[Agent processing not yet implemented]\n\nYou said: {}", message),
    ...
});
```

### What's Missing

#### A. Session Integration
- **No Session creation/loading**: TUI doesn't create or load a Session
- **No Provider integration**: TUI doesn't connect to LLM providers
- **No message persistence**: Messages aren't saved to disk
- **No agent selection**: Tab switches between "build" and "plan" but doesn't actually change agents

#### B. Real-time Streaming
- TUI needs to support streaming responses from the LLM
- Currently just shows a static mock response
- Need to handle partial responses and tool execution updates

#### C. Async Message Processing
- The TUI event loop is synchronous
- Need to spawn async tasks for message processing
- Need to handle concurrent operations (user typing while agent is responding)

### Implementation Plan

1. **Integrate Session into TUI App state**:
   ```rust
   struct App {
       session: Option<Session>,
       provider_registry: Option<ProviderRegistry>,
       processing: bool,
       // ... existing fields
   }
   ```

2. **Add async message processing**:
   - Spawn tokio task when user submits message
   - Stream responses back to TUI via channels
   - Update UI incrementally as responses arrive

3. **Connect agent switching to actual agents**:
   - Load AgentRegistry
   - Switch between build/plan agents properly
   - Update system prompt based on selected agent

4. **Add session persistence**:
   - Save session after each message exchange
   - Support continuing previous sessions

---

## 2. Agent-Swarm Integration (PARTIALLY IMPLEMENTED)

### Current State
The swarm system exists and is functional (`src/swarm/`), but the integration with the Agent system is incomplete.

### What's Working
- ✅ SwarmExecutor can decompose tasks and run sub-agents in parallel
- ✅ Orchestrator manages task decomposition
- ✅ Sub-agents execute with tool access
- ✅ Worktree isolation for parallel execution
- ✅ Rate limiting and context management

### What's Missing

#### A. Agent as Swarm Coordinator
The `Agent` struct implements `Actor` and `Handler<SwarmMessage>` traits (lines 243-328 in `src/agent/mod.rs`), but:
- No actual swarm execution in the agent's `execute()` method
- Agent doesn't use swarm for complex tasks
- No automatic task decomposition based on complexity

#### B. Swarm Tool Integration
- No `swarm` tool in the ToolRegistry for agents to spawn sub-agents
- Agents can't delegate to swarm programmatically

#### C. Session-Swarm Bridge
- Sessions don't have a method to execute via swarm
- No way to choose between single-agent and swarm execution

### Implementation Plan

1. **Add swarm execution option to Agent**:
   ```rust
   pub async fn execute_with_swarm(
       &self,
       session: &mut Session,
       prompt: &str,
       config: SwarmConfig,
   ) -> Result<AgentResponse>
   ```

2. **Create swarm tool**:
   - Add `swarm` tool to ToolRegistry
   - Allows agents to spawn parallel sub-agents for complex tasks
   - Integrates with existing swarm infrastructure

3. **Auto-decomposition**:
   - Agent should analyze task complexity
   - Automatically use swarm for tasks that benefit from parallelization
   - Configurable threshold for automatic swarm usage

---

## 3. Telemetry Collection (NOT IMPLEMENTED)

### Current State
- No telemetry/metrics module exists
- Only basic tracing logs
- No structured metrics collection

### What's Missing

#### A. Metrics Collection
- Request/response latency
- Token usage (input/output)
- Tool execution success/failure rates
- Session duration and message counts
- Provider performance comparison
- Swarm execution statistics

#### B. Success Tracking
- Task completion rates
- Error categorization
- User satisfaction indicators
- Cost tracking per session/task

#### C. Export/Storage
- Local metrics storage
- Optional external export (Prometheus, etc.)
- Privacy-compliant data collection

### Implementation Plan

1. **Create telemetry module** (`src/telemetry/`):
   ```rust
   pub struct Telemetry {
       events: Vec<TelemetryEvent>,
       metrics: MetricsStore,
   }
   
   pub struct TelemetryEvent {
       timestamp: DateTime<Utc>,
       event_type: EventType,
       duration_ms: u64,
       success: bool,
       metadata: HashMap<String, Value>,
   }
   ```

2. **Instrument key operations**:
   - Session::prompt()
   - Agent::execute()
   - Tool::execute()
   - SwarmExecutor::execute()
   - Provider::complete()

3. **Add CLI commands**:
   - `codetether telemetry stats` - Show usage statistics
   - `codetether telemetry export` - Export metrics
   - `codetether telemetry clear` - Clear local data

---

## Implementation Priority

### P0 (Critical - Blocks Basic Usage)
1. **TUI Session Integration** - TUI is currently non-functional
   - Integrate Session into TUI
   - Add async message processing
   - Connect to providers

### P1 (High - Significant Value)
2. **Telemetry Collection** - Needed for understanding usage and improving
   - Basic metrics collection
   - Success/failure tracking
   - Local storage

3. **Agent-Swarm Integration** - Enables parallel execution
   - Add swarm tool
   - Auto-decomposition
   - Session-swarm bridge

### P2 (Medium - Nice to Have)
- Advanced telemetry features
- External metrics export
- Detailed performance analytics

---

## Files to Modify

### For TUI Integration:
- `src/tui/mod.rs` - Major rewrite needed
- `src/session/mod.rs` - May need streaming support
- `src/main.rs` - TUI command handler

### For Swarm Integration:
- `src/agent/mod.rs` - Add swarm execution methods
- `src/tool/mod.rs` - Add swarm tool
- `src/tool/swarm.rs` - New file for swarm tool

### For Telemetry:
- `src/telemetry/mod.rs` - New module
- `src/lib.rs` - Add telemetry module
- `src/session/mod.rs` - Add telemetry hooks
- `src/agent/mod.rs` - Add telemetry hooks
- `src/tool/mod.rs` - Add telemetry hooks
- `src/swarm/executor.rs` - Add telemetry hooks
- `src/provider/` - Add telemetry hooks

---

## Quick Wins

1. **Fix TUI immediately** - The mock response makes the TUI appear broken
2. **Add basic telemetry** - Start collecting simple metrics (duration, success/failure)
3. **Create swarm tool** - Simple wrapper around existing SwarmExecutor
