# Agent-Swarm Integration Design Document

## Executive Summary

This document outlines the integration design for connecting the unused `Agent` struct and `Tool` import from `src/agent/mod.rs` into the swarm system. The design leverages the existing `Actor` and `Handler` trait implementations already present in the `Agent` struct.

## Current State Analysis

### Existing Agent Implementation (`src/agent/mod.rs`)

The `Agent` struct already has:
- Full `Actor` trait implementation (lines 231-253)
- Full `Handler<SwarmMessage>` implementation (lines 256-307)
- Tool registry integration
- LLM provider integration
- Permission system
- Session management

### Current Swarm Architecture

The swarm system consists of:
- **Orchestrator** (`src/swarm/orchestrator.rs`): Decomposes tasks and coordinates sub-agents
- **SwarmExecutor** (`src/swarm/executor.rs`): Executes subtasks in parallel
- **SubAgent** (`src/swarm/subtask.rs`): Data structure representing a sub-agent instance
- **SubTask** (`src/swarm/subtask.rs`): Unit of work for parallel execution

### Gap Analysis

1. **Agent is NOT currently used as a swarm participant** - The `SubAgent` struct is a data container, not an active actor
2. **Tool import is unused** - The `Tool` trait is imported but not directly used (only `ToolRegistry` is used)
3. **Swarm uses raw provider calls** - `SwarmExecutor` directly calls the provider instead of using `Agent`

## Integration Design

### 1. Agent as a Swarm Participant

**Decision: YES** - The `Agent` struct should be a first-class swarm participant.

**Rationale:**
- Agent already implements `Actor` and `Handler<SwarmMessage>`
- Agent provides higher-level orchestration (tool loops, session management)
- Enables consistent behavior between single-agent and swarm execution

**Integration Points:**

```rust
// In src/swarm/executor.rs - Replace raw provider calls with Agent

pub struct SwarmExecutor {
    config: SwarmConfig,
    /// Agent factory for creating swarm-capable agents
    agent_factory: Arc<dyn Fn() -> Agent + Send + Sync>,
}

impl SwarmExecutor {
    pub async fn execute_stage_with_agents(
        &self,
        subtasks: Vec<SubTask>,
    ) -> Result<Vec<SubTaskResult>> {
        // Create Agent instances for each subtask
        let agents: Vec<Agent> = subtasks.iter()
            .map(|subtask| {
                let mut agent = (self.agent_factory)();
                agent.set_system_prompt(build_subagent_prompt(subtask));
                agent
            })
            .collect();
        
        // Execute via Actor/Handler traits
        for (agent, subtask) in agents.iter_mut().zip(&subtasks) {
            let message = SwarmMessage::ExecuteTask {
                task_id: subtask.id.clone(),
                instruction: subtask.instruction.clone(),
            };
            let response = agent.handle(message).await?;
            // Process response...
        }
    }
}
```

### 2. Tool Registration with Swarm

**Decision: Tools should be registered at the Agent level, not swarm level**

**Rationale:**
- Each Agent has its own `ToolRegistry`
- Different swarm participants may need different tool sets
- Maintains encapsulation and flexibility

**Design Pattern:**

```rust
// Agent-level tool registration (already exists)
impl Agent {
    pub fn register_tool(&mut self, tool: Arc<dyn Tool>) {
        self.tools.register(tool);
    }
    
    pub fn with_tools(mut self, tools: Vec<Arc<dyn Tool>>) -> Self {
        for tool in tools {
            self.register_tool(tool);
        }
        self
    }
}

// Swarm-level tool configuration
pub struct SwarmToolConfig {
    /// Base tools available to all agents
    pub base_tools: Vec<Arc<dyn Tool>>,
    /// Specialty-specific tool sets
    pub specialty_tools: HashMap<String, Vec<Arc<dyn Tool>>>,
}

impl SwarmExecutor {
    pub fn with_tool_config(mut self, config: SwarmToolConfig) -> Self {
        self.tool_config = Some(config);
        self
    }
}
```

### 3. Agent-SubAgent Relationship

**Decision: Agent IS-A swarm participant; SubAgent becomes a configuration/data struct**

**Refactoring Strategy:**

```rust
// SubAgent becomes SubAgentConfig - pure data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAgentConfig {
    pub id: String,
    pub name: String,
    pub specialty: String,
    pub subtask_id: String,
    pub model: String,
    pub provider: String,
    pub system_prompt: String,
}

// Agent is the active execution entity
pub struct SwarmParticipant {
    /// The actual agent doing the work
    pub agent: Agent,
    /// Configuration for this participant
    pub config: SubAgentConfig,
    /// Runtime state
    pub status: ParticipantStatus,
}

impl Actor for SwarmParticipant {
    fn actor_id(&self) -> &str {
        &self.config.id
    }
    
    fn actor_status(&self) -> ActorStatus {
        match self.status {
            ParticipantStatus::Working => ActorStatus::Busy,
            ParticipantStatus::Completed => ActorStatus::Stopped,
            _ => ActorStatus::Ready,
        }
    }
    // ...
}

impl Handler<SwarmMessage> for SwarmParticipant {
    type Response = SwarmMessage;
    
    async fn handle(&mut self, message: SwarmMessage) -> Result<Self::Response> {
        // Delegate to the underlying Agent
        self.agent.handle(message).await
    }
}
```

### 4. Integration Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        SwarmExecutor                            │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    Orchestrator                         │   │
│  │  - Decomposes tasks into SubTasks                       │   │
│  │  - Manages dependencies and stages                      │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              │                                  │
│                              ▼                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              Agent Factory (per subtask)                │   │
│  │  - Creates Agent with appropriate tools                 │   │
│  │  - Configures system prompts                            │   │
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
│  │  │ ┌───────┐ │   │ │  │ ┌───────┐ │   │ │  │ ┌───────┐ │   │   │
│  │  │ │Session│ │   │ │  │ │Session│ │   │ │  │ │Session│ │   │   │
│  │  │ └───────┘ │   │ │  │ └───────┘ │   │ │  │ └───────┘ │   │   │
│  │  └───────────┘   │ │  └───────────┘   │ │  └───────────┘   │   │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## Implementation Plan

### Phase 1: Refactor SubAgent to SubAgentConfig

1. Rename `SubAgent` to `SubAgentConfig` in `src/swarm/subtask.rs`
2. Remove execution-related fields (status, steps, tool_calls, output)
3. Keep configuration fields (id, name, specialty, model, provider)

### Phase 2: Create SwarmParticipant Wrapper

1. Create new `SwarmParticipant` struct in `src/swarm/participant.rs`
2. Implement `Actor` trait (delegating to Agent)
3. Implement `Handler<SwarmMessage>` trait (delegating to Agent)
4. Add status tracking fields

### Phase 3: Update SwarmExecutor

1. Add `agent_factory` field to `SwarmExecutor`
2. Modify `execute_stage` to create `SwarmParticipant` instances
3. Use `Agent::handle()` instead of raw provider calls
4. Remove duplicate agent loop logic (use Agent's built-in loop)

### Phase 4: Tool Registration Integration

1. Create `SwarmToolConfig` for per-specialty tool sets
2. Update Agent factory to apply tool configuration
3. Ensure all swarm participants have consistent base tools

## Code Examples

### Example 1: Creating a Swarm with Custom Agents

```rust
use codetether::swarm::{SwarmExecutor, SwarmConfig, DecompositionStrategy};
use codetether::agent::{Agent, AgentInfo, AgentMode};
use codetether::tool::ToolRegistry;
use std::sync::Arc;

// Create a custom agent factory
let agent_factory = move || {
    let info = AgentInfo {
        name: "swarm-worker".to_string(),
        description: Some("Parallel task worker".to_string()),
        mode: AgentMode::Subagent,
        native: true,
        hidden: true,
        model: Some("gpt-4o-mini".to_string()),
        temperature: Some(0.7),
        top_p: None,
        max_steps: Some(50),
    };
    
    let provider = Arc::new(create_provider());
    let tools = ToolRegistry::with_defaults();
    let system_prompt = "You are a specialized sub-agent...".to_string();
    
    Agent::new(info, provider, tools, system_prompt)
};

// Create executor with factory
let executor = SwarmExecutor::new(SwarmConfig::default())
    .with_agent_factory(Arc::new(agent_factory));

// Execute task
let result = executor.execute(
    "Implement a REST API with authentication",
    DecompositionStrategy::ByDomain
).await?;
```

### Example 2: Specialty-Specific Tool Configuration

```rust
use codetether::swarm::SwarmToolConfig;
use codetether::tool::{bash::BashTool, webfetch::WebFetchTool};

let tool_config = SwarmToolConfig {
    // All agents get these tools
    base_tools: vec![
        Arc::new(ReadTool::new()),
        Arc::new(WriteTool::new()),
        Arc::new(GrepTool::new()),
    ],
    // Specialty-specific tools
    specialty_tools: {
        let mut map = HashMap::new();
        map.insert("DevOps".to_string(), vec![
            Arc::new(BashTool::new()),
            Arc::new(DockerTool::new()),
        ]);
        map.insert("Researcher".to_string(), vec![
            Arc::new(WebFetchTool::new()),
            Arc::new(WebSearchTool::new()),
        ]);
        map
    },
};

let executor = SwarmExecutor::new(config)
    .with_tool_config(tool_config);
```

### Example 3: Direct Agent Participation in Swarm

```rust
use codetether::swarm::{Actor, Handler, SwarmMessage};
use codetether::agent::Agent;

// Agent can participate directly in swarm coordination
let mut agent = create_specialized_agent();

// Initialize as actor
agent.initialize().await?;

// Handle swarm messages
let message = SwarmMessage::ExecuteTask {
    task_id: "task-123".to_string(),
    instruction: "Refactor the authentication module".to_string(),
};

let response = agent.handle(message).await?;
match response {
    SwarmMessage::TaskCompleted { task_id, result } => {
        println!("Task {} completed: {}", task_id, result);
    }
    SwarmMessage::TaskFailed { task_id, error } => {
        eprintln!("Task {} failed: {}", task_id, error);
    }
    _ => {}
}

// Shutdown gracefully
agent.shutdown().await?;
```

### Example 4: Tool Access from Agent

```rust
use codetether::tool::Tool;
use codetether::agent::Agent;

// Agent provides access to its tool registry
impl Agent {
    /// Execute a tool by name (used by Handler implementation)
    pub async fn execute_tool(&self, name: &str, arguments: &str) -> ToolResult {
        match self.tools.get(name) {
            Some(tool) => {
                let args: serde_json::Value = match serde_json::from_str(arguments) {
                    Ok(v) => v,
                    Err(e) => {
                        return ToolResult::error(format!("Failed to parse arguments: {}", e));
                    }
                };
                
                match tool.execute(args).await {
                    Ok(result) => result,
                    Err(e) => ToolResult::error(format!("Tool execution failed: {}", e)),
                }
            }
            None => ToolResult::error(format!("Unknown tool: {}", name)),
        }
    }
    
    /// Check if agent has a specific tool
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.get(name).is_some()
    }
    
    /// Get tool descriptions for LLM
    pub fn tool_definitions(&self) -> Vec<ToolDefinition> {
        self.tools.definitions()
    }
}
```

## Benefits of This Design

1. **Consistency**: Single `Agent` implementation used everywhere
2. **Testability**: Can test Agent logic independently of swarm
3. **Flexibility**: Easy to create specialized agents for different swarm roles
4. **Type Safety**: Leverages Rust's type system for message handling
5. **Extensibility**: New swarm message types just need new Handler implementations

## Migration Path

1. **Backward Compatibility**: Keep existing `SubAgent` as deprecated alias
2. **Gradual Adoption**: SwarmExecutor can use either old or new pattern
3. **Feature Flag**: Add `#[cfg(feature = "agent-swarm")]` for new behavior

## Open Questions

1. Should we support dynamic tool registration during swarm execution?
2. How should agent state be persisted across swarm stages?
3. Should agents share a tool registry or have isolated registries?

## Conclusion

The `Agent` struct is already well-designed for swarm participation with its `Actor` and `Handler` implementations. The integration primarily involves:

1. Refactoring `SubAgent` to be configuration-only
2. Creating a `SwarmParticipant` wrapper
3. Updating `SwarmExecutor` to use `Agent::handle()`
4. Adding tool configuration at the swarm level

This design unifies the agent and swarm systems while maintaining flexibility and extensibility.
