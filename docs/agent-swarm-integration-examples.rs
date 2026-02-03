// Agent-Swarm Integration Code Examples
// This file contains runnable code examples demonstrating the integration design

// ============================================================================
// Example 1: Basic Agent as Swarm Participant
// ============================================================================

use codetether::agent::{Agent, AgentInfo, AgentMode, AgentResponse};
use codetether::swarm::{Actor, ActorStatus, Handler, SwarmMessage};
use codetether::tool::ToolRegistry;
use codetether::provider::{Provider, CompletionRequest, CompletionResponse};
use codetether::session::Session;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

/// Demonstrates how Agent already implements Actor and Handler traits
async fn example_agent_as_actor() -> Result<()> {
    // Create an agent
    let info = AgentInfo {
        name: "worker-1".to_string(),
        description: Some("A swarm worker agent".to_string()),
        mode: AgentMode::Subagent,
        native: true,
        hidden: false,
        model: Some("gpt-4o-mini".to_string()),
        temperature: Some(0.7),
        top_p: None,
        max_steps: Some(50),
    };
    
    let provider = Arc::new(MockProvider::new());
    let tools = ToolRegistry::with_defaults();
    let system_prompt = "You are a helpful assistant.".to_string();
    
    let mut agent = Agent::new(info, provider, tools, system_prompt);
    
    // Use as Actor
    println!("Agent ID: {}", agent.actor_id());
    println!("Agent Status: {}", agent.actor_status());
    
    // Initialize
    agent.initialize().await?;
    
    // Handle swarm messages
    let message = SwarmMessage::ExecuteTask {
        task_id: "task-001".to_string(),
        instruction: "Write a hello world program".to_string(),
    };
    
    let response = agent.handle(message).await?;
    println!("Response: {:?}", response);
    
    // Shutdown
    agent.shutdown().await?;
    
    Ok(())
}

// ============================================================================
// Example 2: SwarmParticipant Wrapper
// ============================================================================

use codetether::swarm::subtask::SubAgentConfig;
use serde::{Deserialize, Serialize};

/// SwarmParticipant wraps Agent to add swarm-specific state
pub struct SwarmParticipant {
    /// The underlying agent
    pub agent: Agent,
    /// Configuration for this participant
    pub config: SubAgentConfig,
    /// Current status
    pub status: ParticipantStatus,
    /// Start time
    pub started_at: Option<std::time::Instant>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticipantStatus {
    Initializing,
    Ready,
    Working,
    Completed,
    Failed,
}

impl SwarmParticipant {
    /// Create a new participant from configuration
    pub fn new(config: SubAgentConfig, agent: Agent) -> Self {
        Self {
            agent,
            config,
            status: ParticipantStatus::Initializing,
            started_at: None,
        }
    }
    
    /// Get the subtask ID this participant is handling
    pub fn subtask_id(&self) -> &str {
        &self.config.subtask_id
    }
    
    /// Get the specialty of this participant
    pub fn specialty(&self) -> &str {
        &self.config.specialty
    }
}

#[async_trait]
impl Actor for SwarmParticipant {
    fn actor_id(&self) -> &str {
        &self.config.id
    }
    
    fn actor_status(&self) -> ActorStatus {
        match self.status {
            ParticipantStatus::Initializing => ActorStatus::Initializing,
            ParticipantStatus::Ready => ActorStatus::Ready,
            ParticipantStatus::Working => ActorStatus::Busy,
            ParticipantStatus::Completed => ActorStatus::Stopped,
            ParticipantStatus::Failed => ActorStatus::Stopped,
        }
    }
    
    async fn initialize(&mut self) -> Result<()> {
        self.agent.initialize().await?;
        self.status = ParticipantStatus::Ready;
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        self.agent.shutdown().await?;
        Ok(())
    }
}

#[async_trait]
impl Handler<SwarmMessage> for SwarmParticipant {
    type Response = SwarmMessage;
    
    async fn handle(&mut self, message: SwarmMessage) -> Result<Self::Response> {
        self.status = ParticipantStatus::Working;
        self.started_at = Some(std::time::Instant::now());
        
        // Delegate to the underlying Agent
        let response = self.agent.handle(message).await?;
        
        // Update status based on response
        self.status = match &response {
            SwarmMessage::TaskCompleted { .. } => ParticipantStatus::Completed,
            SwarmMessage::TaskFailed { .. } => ParticipantStatus::Failed,
            _ => ParticipantStatus::Ready,
        };
        
        Ok(response)
    }
}

// ============================================================================
// Example 3: Agent Factory Pattern
// ============================================================================

/// Factory for creating agents with specific configurations
pub trait AgentFactory: Send + Sync {
    fn create_agent(&self, specialty: &str, subtask_id: &str) -> Agent;
}

/// Default implementation using closures
pub struct ClosureAgentFactory {
    creator: Box<dyn Fn(&str, &str) -> Agent + Send + Sync>,
}

impl ClosureAgentFactory {
    pub fn new<F>(creator: F) -> Self
    where
        F: Fn(&str, &str) -> Agent + Send + Sync + 'static,
    {
        Self {
            creator: Box::new(creator),
        }
    }
}

impl AgentFactory for ClosureAgentFactory {
    fn create_agent(&self, specialty: &str, subtask_id: &str) -> Agent {
        (self.creator)(specialty, subtask_id)
    }
}

/// Example: Creating a factory with specialty-specific prompts
fn create_specialty_factory(base_provider: Arc<dyn Provider>) -> impl AgentFactory {
    ClosureAgentFactory::new(move |specialty, subtask_id| {
        let system_prompt = match specialty {
            "Coder" => format!(
                "You are a Coder specialist (ID: {}). Write clean, well-tested code.",
                subtask_id
            ),
            "Reviewer" => format!(
                "You are a Reviewer specialist (ID: {}). Review code for quality and bugs.",
                subtask_id
            ),
            "Tester" => format!(
                "You are a Tester specialist (ID: {}). Write comprehensive tests.",
                subtask_id
            ),
            _ => format!(
                "You are a general specialist (ID: {}). Complete the assigned task.",
                subtask_id
            ),
        };
        
        let info = AgentInfo {
            name: format!("{}-{}", specialty.to_lowercase(), subtask_id[..8].to_string()),
            description: Some(format!("{} specialist agent", specialty)),
            mode: AgentMode::Subagent,
            native: true,
            hidden: true,
            model: None,
            temperature: Some(0.7),
            top_p: None,
            max_steps: Some(50),
        };
        
        let tools = ToolRegistry::with_defaults();
        
        Agent::new(info, Arc::clone(&base_provider), tools, system_prompt)
    })
}

// ============================================================================
// Example 4: Tool Configuration for Swarm
// ============================================================================

use std::collections::HashMap;
use codetether::tool::{Tool, bash::BashTool, webfetch::WebFetchTool};

/// Configuration for tools available to swarm participants
pub struct SwarmToolConfig {
    /// Base tools available to all agents
    pub base_tools: Vec<Arc<dyn Tool>>,
    /// Specialty-specific tool sets
    pub specialty_tools: HashMap<String, Vec<Arc<dyn Tool>>>,
}

impl SwarmToolConfig {
    /// Create a new empty configuration
    pub fn new() -> Self {
        Self {
            base_tools: Vec::new(),
            specialty_tools: HashMap::new(),
        }
    }
    
    /// Add base tools
    pub fn with_base_tools(mut self, tools: Vec<Arc<dyn Tool>>) -> Self {
        self.base_tools = tools;
        self
    }
    
    /// Add specialty-specific tools
    pub fn with_specialty_tools(
        mut self,
        specialty: impl Into<String>,
        tools: Vec<Arc<dyn Tool>>,
    ) -> Self {
        self.specialty_tools.insert(specialty.into(), tools);
        self
    }
    
    /// Get all tools for a specialty (base + specialty-specific)
    pub fn get_tools_for_specialty(&self, specialty: &str) -> Vec<Arc<dyn Tool>> {
        let mut tools = self.base_tools.clone();
        if let Some(specialty_tools) = self.specialty_tools.get(specialty) {
            tools.extend(specialty_tools.clone());
        }
        tools
    }
}

/// Example: Creating a tool configuration
fn create_dev_tool_config() -> SwarmToolConfig {
    use codetether::tool::{
        file::ReadTool,
        file::WriteTool,
        edit::EditTool,
        search::GrepTool,
    };
    
    SwarmToolConfig::new()
        .with_base_tools(vec![
            Arc::new(ReadTool::new()),
            Arc::new(WriteTool::new()),
            Arc::new(EditTool::new()),
            Arc::new(GrepTool::new()),
        ])
        .with_specialty_tools("DevOps", vec![
            Arc::new(BashTool::new()),
            // Arc::new(DockerTool::new()), // if available
        ])
        .with_specialty_tools("Researcher", vec![
            Arc::new(WebFetchTool::new()),
            Arc::new(WebSearchTool::new()),
        ])
}

// ============================================================================
// Example 5: Updated SwarmExecutor with Agent Integration
// ============================================================================

use codetether::swarm::{SwarmConfig, SwarmResult, DecompositionStrategy};
use codetether::swarm::orchestrator::Orchestrator;
use codetether::swarm::subtask::{SubTask, SubTaskResult};

/// Updated SwarmExecutor that uses Agent for execution
pub struct AgentSwarmExecutor {
    config: SwarmConfig,
    agent_factory: Arc<dyn AgentFactory>,
    tool_config: Option<SwarmToolConfig>,
}

impl AgentSwarmExecutor {
    pub fn new(config: SwarmConfig, agent_factory: Arc<dyn AgentFactory>) -> Self {
        Self {
            config,
            agent_factory,
            tool_config: None,
        }
    }
    
    pub fn with_tool_config(mut self, config: SwarmToolConfig) -> Self {
        self.tool_config = Some(config);
        self
    }
    
    /// Execute a task using the swarm with Agent-based participants
    pub async fn execute(
        &self,
        task: &str,
        strategy: DecompositionStrategy,
    ) -> Result<SwarmResult> {
        // Create orchestrator and decompose task
        let mut orchestrator = Orchestrator::new(self.config.clone()).await?;
        let subtasks = orchestrator.decompose(task, strategy).await?;
        
        // Execute each stage
        let mut all_results = Vec::new();
        let max_stage = subtasks.iter().map(|s| s.stage).max().unwrap_or(0);
        
        for stage in 0..=max_stage {
            let stage_subtasks: Vec<SubTask> = subtasks
                .iter()
                .filter(|s| s.stage == stage)
                .cloned()
                .collect();
            
            if stage_subtasks.is_empty() {
                continue;
            }
            
            // Execute stage with Agent-based participants
            let stage_results = self.execute_stage_with_agents(stage_subtasks).await?;
            all_results.extend(stage_results);
        }
        
        // Aggregate and return results
        Ok(SwarmResult {
            success: all_results.iter().all(|r| r.success),
            result: aggregate_results(&all_results),
            subtask_results: all_results,
            stats: SwarmStats::default(), // Simplified
            artifacts: Vec::new(),
            error: None,
        })
    }
    
    /// Execute a stage using Agent-based SwarmParticipants
    async fn execute_stage_with_agents(
        &self,
        subtasks: Vec<SubTask>,
    ) -> Result<Vec<SubTaskResult>> {
        let mut participants = Vec::new();
        let mut handles = Vec::new();
        
        // Create participants for each subtask
        for subtask in subtasks {
            let specialty = subtask.specialty.clone().unwrap_or_else(|| "General".to_string());
            
            // Create agent using factory
            let agent = self.agent_factory.create_agent(&specialty, &subtask.id);
            
            // Apply tool configuration if present
            if let Some(ref tool_config) = self.tool_config {
                let tools = tool_config.get_tools_for_specialty(&specialty);
                // Agent would need a method to set tools
                // agent.set_tools(tools);
            }
            
            // Create config
            let config = SubAgentConfig {
                id: format!("agent-{}", subtask.id),
                name: format!("{} Agent", specialty),
                specialty,
                subtask_id: subtask.id.clone(),
                model: "gpt-4o-mini".to_string(),
                provider: "openai".to_string(),
            };
            
            let participant = SwarmParticipant::new(config, agent);
            participants.push((participant, subtask));
        }
        
        // Execute in parallel
        for (mut participant, subtask) in participants {
            let handle = tokio::spawn(async move {
                // Initialize
                if let Err(e) = participant.initialize().await {
                    return SubTaskResult {
                        subtask_id: subtask.id,
                        subagent_id: participant.actor_id().to_string(),
                        success: false,
                        result: String::new(),
                        steps: 0,
                        tool_calls: 0,
                        execution_time_ms: 0,
                        error: Some(format!("Initialization failed: {}", e)),
                        artifacts: Vec::new(),
                    };
                }
                
                // Execute task
                let message = SwarmMessage::ExecuteTask {
                    task_id: subtask.id.clone(),
                    instruction: subtask.instruction.clone(),
                };
                
                let start = std::time::Instant::now();
                let result = match participant.handle(message).await {
                    Ok(SwarmMessage::TaskCompleted { result, .. }) => SubTaskResult {
                        subtask_id: subtask.id,
                        subagent_id: participant.actor_id().to_string(),
                        success: true,
                        result,
                        steps: 0, // Would track from agent
                        tool_calls: 0, // Would track from agent
                        execution_time_ms: start.elapsed().as_millis() as u64,
                        error: None,
                        artifacts: Vec::new(),
                    },
                    Ok(SwarmMessage::TaskFailed { error, .. }) => SubTaskResult {
                        subtask_id: subtask.id,
                        subagent_id: participant.actor_id().to_string(),
                        success: false,
                        result: String::new(),
                        steps: 0,
                        tool_calls: 0,
                        execution_time_ms: start.elapsed().as_millis() as u64,
                        error: Some(error),
                        artifacts: Vec::new(),
                    },
                    Err(e) => SubTaskResult {
                        subtask_id: subtask.id,
                        subagent_id: participant.actor_id().to_string(),
                        success: false,
                        result: String::new(),
                        steps: 0,
                        tool_calls: 0,
                        execution_time_ms: start.elapsed().as_millis() as u64,
                        error: Some(e.to_string()),
                        artifacts: Vec::new(),
                    },
                    _ => SubTaskResult {
                        subtask_id: subtask.id,
                        subagent_id: participant.actor_id().to_string(),
                        success: false,
                        result: String::new(),
                        steps: 0,
                        tool_calls: 0,
                        execution_time_ms: start.elapsed().as_millis() as u64,
                        error: Some("Unexpected response type".to_string()),
                        artifacts: Vec::new(),
                    },
                };
                
                // Shutdown
                let _ = participant.shutdown().await;
                
                result
            });
            
            handles.push(handle);
        }
        
        // Collect results
        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => {
                    eprintln!("Task join error: {}", e);
                }
            }
        }
        
        Ok(results)
    }
}

fn aggregate_results(results: &[SubTaskResult]) -> String {
    results
        .iter()
        .map(|r| format!("=== {} ===\n{}", r.subtask_id, r.result))
        .collect::<Vec<_>>()
        .join("\n\n")
}

// ============================================================================
// Example 6: Direct Tool Usage from Agent
// ============================================================================

use codetether::tool::ToolResult;
use serde_json::Value;

/// Demonstrates how Agent provides access to tools
async fn example_agent_tool_usage(agent: &Agent) -> Result<()> {
    // Check if agent has a specific tool
    if agent.has_tool("read") {
        println!("Agent has read tool");
    }
    
    // List available tools
    let tools = agent.list_tools();
    println!("Available tools: {:?}", tools);
    
    // Execute a tool through the agent
    let result = agent.execute_tool("read", r#"{"path": "src/main.rs"}"#).await;
    println!("Tool result: {:?}", result);
    
    // Get tool definitions for LLM
    let definitions = agent.tool_definitions();
    println!("Tool definitions count: {}", definitions.len());
    
    Ok(())
}

// ============================================================================
// Example 7: Complete End-to-End Usage
// ============================================================================

async fn example_complete_workflow() -> Result<()> {
    use codetether::swarm::{SwarmConfig, DecompositionStrategy};
    
    // 1. Create provider
    let provider = Arc::new(MockProvider::new());
    
    // 2. Create agent factory
    let factory = Arc::new(create_specialty_factory(Arc::clone(&provider)));
    
    // 3. Create tool configuration
    let tool_config = create_dev_tool_config();
    
    // 4. Create executor
    let executor = AgentSwarmExecutor::new(
        SwarmConfig::default(),
        factory
    ).with_tool_config(tool_config);
    
    // 5. Execute task
    let result = executor.execute(
        "Implement a REST API with authentication and user management",
        DecompositionStrategy::ByDomain
    ).await?;
    
    // 6. Process results
    println!("Success: {}", result.success);
    println!("Result:\n{}", result.result);
    
    for subtask_result in &result.subtask_results {
        println!(
            "Subtask {}: {}",
            subtask_result.subtask_id,
            if subtask_result.success { "✓" } else { "✗" }
        );
    }
    
    Ok(())
}

// ============================================================================
// Mock Provider for Examples
// ============================================================================

struct MockProvider;

impl MockProvider {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Provider for MockProvider {
    async fn complete(&self, _request: CompletionRequest) -> Result<CompletionResponse> {
        // Mock implementation
        unimplemented!("Mock provider for examples only")
    }
    
    async fn health_check(&self) -> Result<bool> {
        Ok(true)
    }
}

// ============================================================================
// Main
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    println!("Agent-Swarm Integration Examples");
    println!("=================================\n");
    
    // Run examples
    example_agent_as_actor().await?;
    
    println!("\nExamples completed successfully!");
    
    Ok(())
}
