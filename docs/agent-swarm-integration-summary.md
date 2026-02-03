# Agent-Swarm Integration Summary

## Overview

This document provides a concise summary of the integration design for connecting the unused `Agent` struct and `Tool` import from `src/agent/mod.rs` into the swarm system.

## Key Decisions

### 1. Agent as Swarm Participant: **YES**

The `Agent` struct **should** be a first-class swarm participant because:
- It already implements `Actor` and `Handler<SwarmMessage>` traits
- It provides higher-level orchestration (tool loops, session management)
- Enables consistent behavior between single-agent and swarm execution
- Eliminates duplicate code in `SwarmExecutor`

### 2. Tool Registration: **Agent-level, not Swarm-level**

Tools should be registered at the **Agent level** via `ToolRegistry` because:
- Each Agent already has its own `ToolRegistry` field
- Different swarm participants may need different tool sets
- Maintains encapsulation and flexibility
- Specialty-specific tools can be configured per-agent

### 3. Agent-SubAgent Relationship

**Agent IS-A swarm participant; SubAgent becomes configuration-only**

- `Agent` is the active execution entity with `Actor`/`Handler` traits
- `SubAgent` should be refactored to `SubAgentConfig` (pure data)
- New `SwarmParticipant` wrapper combines Agent + Config + Status

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        SwarmExecutor                            │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    Orchestrator                         │   │
│  │  - Decomposes tasks into SubTasks                       │   │
│  │  - Manages dependencies and stages                      │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              │                                  │
│              ┌───────────────┼───────────────┐                  │
│              ▼               ▼               ▼                  │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐   │
│  │  SwarmParticipant│ │  SwarmParticipant│ │  SwarmParticipant│   │
│  │  ┌───────────┐   │ │  ┌───────────┐   │ │  ┌───────────┐   │   │
│  │  │   Agent   │   │ │  │   Agent   │   │ │  │   Agent   │   │   │
│  │  │ ┌───────┐ │   │ │  │ ┌───────┐ │   │ │  │ ┌───────┐ │   │   │
│  │  │ │Tools  │ │   │ │  │ │Tools  │ │   │ │  │ │Tools  │ │   │   │
│  │  │ │Registry│ │   │ │  │ │Registry│ │   │ │  │ │Registry│ │   │   │
│  │  │ └───────┘ │   │ │  │ └───────┘ │   │ │  │ └───────┘ │   │   │
│  │  └───────────┘   │ │  └───────────┘   │ │  └───────────┘   │   │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## Integration Points

### Current State (src/agent/mod.rs)

The `Agent` struct already has:
- ✅ `Actor` trait implementation (lines 231-253)
- ✅ `Handler<SwarmMessage>` implementation (lines 256-307)
- ✅ `ToolRegistry` integration
- ✅ LLM provider integration
- ✅ Permission system
- ✅ Session management

### Required Changes

#### 1. Refactor SubAgent → SubAgentConfig

**File:** `src/swarm/subtask.rs`

```rust
// Current: SubAgent has execution state
pub struct SubAgent {
    pub id: String,
    pub name: String,
    pub specialty: String,
    pub status: SubAgentStatus,  // Remove
    pub steps: usize,            // Remove
    pub tool_calls: Vec<ToolCallRecord>, // Remove
    pub output: String,          // Remove
    // ...
}

// New: SubAgentConfig is pure configuration
pub struct SubAgentConfig {
    pub id: String,
    pub name: String,
    pub specialty: String,
    pub subtask_id: String,
    pub model: String,
    pub provider: String,
    pub system_prompt: String,
}
```

#### 2. Create SwarmParticipant

**New file:** `src/swarm/participant.rs`

```rust
pub struct SwarmParticipant {
    pub agent: Agent,
    pub config: SubAgentConfig,
    pub status: ParticipantStatus,
}

impl Actor for SwarmParticipant { /* Delegate to Agent */ }
impl Handler<SwarmMessage> for SwarmParticipant { /* Delegate to Agent */ }
```

#### 3. Update SwarmExecutor

**File:** `src/swarm/executor.rs`

```rust
pub struct SwarmExecutor {
    config: SwarmConfig,
    agent_factory: Arc<dyn AgentFactory>,  // Add
    tool_config: Option<SwarmToolConfig>,  // Add
}

// Replace raw provider calls with:
let agent = self.agent_factory.create_agent(&specialty, &subtask.id);
let participant = SwarmParticipant::new(config, agent);
let response = participant.handle(message).await?;
```

## Usage Patterns

### Pattern 1: Basic Swarm Execution

```rust
use codetether::swarm::{SwarmExecutor, SwarmConfig, DecompositionStrategy};
use codetether::agent::{Agent, AgentInfo, AgentMode};

let executor = SwarmExecutor::new(SwarmConfig::default())
    .with_agent_factory(Arc::new(|specialty, id| {
        Agent::new(
            AgentInfo { name: id.to_string(), mode: AgentMode::Subagent, .. },
            provider,
            ToolRegistry::with_defaults(),
            format!("You are a {} specialist.", specialty)
        )
    }));

let result = executor.execute(
    "Implement a REST API",
    DecompositionStrategy::ByDomain
).await?;
```

### Pattern 2: Specialty-Specific Tools

```rust
use codetether::swarm::SwarmToolConfig;

let tool_config = SwarmToolConfig::new()
    .with_base_tools(vec![read_tool, write_tool])
    .with_specialty_tools("DevOps", vec![bash_tool, docker_tool])
    .with_specialty_tools("Researcher", vec![webfetch_tool, websearch_tool]);

let executor = SwarmExecutor::new(config)
    .with_tool_config(tool_config);
```

### Pattern 3: Direct Agent Participation

```rust
use codetether::swarm::{Actor, Handler, SwarmMessage};

let mut agent = create_agent();
agent.initialize().await?;

let response = agent.handle(SwarmMessage::ExecuteTask {
    task_id: "task-123".to_string(),
    instruction: "Refactor module".to_string(),
}).await?;

agent.shutdown().await?;
```

## Benefits

1. **Code Reuse**: Single `Agent` implementation used everywhere
2. **Consistency**: Same tool execution, session management, and LLM interaction
3. **Testability**: Can test Agent logic independently of swarm
4. **Flexibility**: Easy to create specialized agents for different roles
5. **Type Safety**: Leverages Rust's type system for message handling

## Migration Path

1. **Phase 1**: Create `SwarmParticipant` wrapper (backward compatible)
2. **Phase 2**: Add `AgentFactory` to `SwarmExecutor`
3. **Phase 3**: Refactor `SubAgent` → `SubAgentConfig`
4. **Phase 4**: Update `execute_stage` to use `Agent::handle()`
5. **Phase 5**: Remove duplicate agent loop logic from `SwarmExecutor`

## Files Modified

| File | Changes |
|------|---------|
| `src/swarm/subtask.rs` | Refactor `SubAgent` to `SubAgentConfig` |
| `src/swarm/participant.rs` | New file: `SwarmParticipant` struct |
| `src/swarm/executor.rs` | Add `AgentFactory`, use `Agent::handle()` |
| `src/swarm/mod.rs` | Export new types |

## Open Questions

1. Should agents share a tool registry or have isolated registries?
2. How should agent state be persisted across swarm stages?
3. Should we support dynamic tool registration during execution?

## Conclusion

The `Agent` struct is already well-designed for swarm participation. The integration primarily involves:

1. Creating a `SwarmParticipant` wrapper
2. Adding an `AgentFactory` to `SwarmExecutor`
3. Refactoring `SubAgent` to be configuration-only
4. Using `Agent::handle()` for message processing

This design unifies the agent and swarm systems while maintaining flexibility and extensibility.
