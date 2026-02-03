//! Parallel execution engine for swarm operations
//!
//! Executes subtasks in parallel across multiple sub-agents,
//! respecting dependencies and optimizing for critical path.

use super::{
    orchestrator::Orchestrator,
    subtask::{SubTask, SubTaskResult},
    DecompositionStrategy, StageStats, SwarmConfig, SwarmResult,
};

// Re-export swarm types for convenience
pub use super::{Actor, ActorStatus, Handler, SwarmMessage};
use crate::{
    agent::Agent,
    provider::{CompletionRequest, ContentPart, FinishReason, Message, Provider, Role},
    swarm::{SwarmArtifact, SwarmStats},
    tool::ToolRegistry,
};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tokio::time::{timeout, Duration};

/// The swarm executor runs subtasks in parallel
pub struct SwarmExecutor {
    config: SwarmConfig,
    /// Optional agent for handling swarm-level coordination (reserved for future use)
    coordinator_agent: Option<Arc<tokio::sync::Mutex<Agent>>>,
}

impl SwarmExecutor {
    /// Create a new executor
    pub fn new(config: SwarmConfig) -> Self {
        Self { 
            config,
            coordinator_agent: None,
        }
    }
    
    /// Set a coordinator agent for swarm-level coordination
    pub fn with_coordinator_agent(mut self, agent: Arc<tokio::sync::Mutex<Agent>>) -> Self {
        tracing::debug!("Setting coordinator agent for swarm execution");
        self.coordinator_agent = Some(agent);
        self
    }
    
    /// Get the coordinator agent if set
    pub fn coordinator_agent(&self) -> Option<&Arc<tokio::sync::Mutex<Agent>>> {
        self.coordinator_agent.as_ref()
    }
    
    /// Execute a task using the swarm
    pub async fn execute(
        &self,
        task: &str,
        strategy: DecompositionStrategy,
    ) -> Result<SwarmResult> {
        let start_time = Instant::now();
        
        // Create orchestrator
        let mut orchestrator = Orchestrator::new(self.config.clone()).await?;
        
        tracing::info!(provider_name = %orchestrator.provider(), "Starting swarm execution for task");
        
        // Decompose the task
        let subtasks = orchestrator.decompose(task, strategy).await?;
        
        if subtasks.is_empty() {
            return Ok(SwarmResult {
                success: false,
                result: String::new(),
                subtask_results: Vec::new(),
                stats: SwarmStats::default(),
                artifacts: Vec::new(),
                error: Some("No subtasks generated".to_string()),
            });
        }
        
        tracing::info!(provider_name = %orchestrator.provider(), "Task decomposed into {} subtasks", subtasks.len());
        
        // Execute stages in order
        let max_stage = subtasks.iter().map(|s| s.stage).max().unwrap_or(0);
        let mut all_results: Vec<SubTaskResult> = Vec::new();
        let artifacts: Vec<SwarmArtifact> = Vec::new();
        
        // Shared state for completed results
        let completed_results: Arc<RwLock<HashMap<String, String>>> = 
            Arc::new(RwLock::new(HashMap::new()));
        
        for stage in 0..=max_stage {
            let stage_start = Instant::now();
            
            let stage_subtasks: Vec<SubTask> = orchestrator
                .subtasks_for_stage(stage)
                .into_iter()
                .cloned()
                .collect();
            
            tracing::debug!(
                "Stage {} has {} subtasks (max_stage={})",
                stage,
                stage_subtasks.len(),
                max_stage
            );
            
            if stage_subtasks.is_empty() {
                continue;
            }
            
            tracing::info!(
                provider_name = %orchestrator.provider(),
                "Executing stage {} with {} subtasks",
                stage,
                stage_subtasks.len()
            );
            
            // Execute all subtasks in this stage in parallel
            let stage_results = self
                .execute_stage(
                    &orchestrator,
                    stage_subtasks,
                    completed_results.clone(),
                )
                .await?;
            
            // Update completed results for next stage
            {
                let mut completed = completed_results.write().await;
                for result in &stage_results {
                    completed.insert(result.subtask_id.clone(), result.result.clone());
                }
            }
            
            // Update orchestrator stats
            let stage_time = stage_start.elapsed().as_millis() as u64;
            let max_steps = stage_results.iter().map(|r| r.steps).max().unwrap_or(0);
            let total_steps: usize = stage_results.iter().map(|r| r.steps).sum();
            
            orchestrator.stats_mut().stages.push(StageStats {
                stage,
                subagent_count: stage_results.len(),
                max_steps,
                total_steps,
                execution_time_ms: stage_time,
            });
            
            // Mark subtasks as completed
            for result in &stage_results {
                orchestrator.complete_subtask(&result.subtask_id, result.clone());
            }
            
            all_results.extend(stage_results);
        }
        
        // Get provider name before mutable borrow
        let provider_name = orchestrator.provider().to_string();
        
        // Calculate final stats
        let stats = orchestrator.stats_mut();
        stats.execution_time_ms = start_time.elapsed().as_millis() as u64;
        stats.sequential_time_estimate_ms = all_results
            .iter()
            .map(|r| r.execution_time_ms)
            .sum();
        stats.calculate_critical_path();
        stats.calculate_speedup();
        
        // Aggregate results
        let success = all_results.iter().all(|r| r.success);
        let result = self.aggregate_results(&all_results).await?;
        
        tracing::info!(
            provider_name = %provider_name,
            "Swarm execution complete: {} subtasks, {:.1}x speedup",
            all_results.len(),
            stats.speedup_factor
        );
        
        Ok(SwarmResult {
            success,
            result,
            subtask_results: all_results,
            stats: orchestrator.stats().clone(),
            artifacts,
            error: None,
        })
    }
    
    /// Execute a single stage of subtasks in parallel (with rate limiting)
    async fn execute_stage(
        &self,
        orchestrator: &Orchestrator,
        subtasks: Vec<SubTask>,
        completed_results: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<Vec<SubTaskResult>> {
        let mut handles: Vec<tokio::task::JoinHandle<Result<SubTaskResult, anyhow::Error>>> = Vec::new();
        
        // Rate limiting: semaphore for max concurrent requests
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.config.max_concurrent_requests));
        let delay_ms = self.config.request_delay_ms;
        
        // Get provider info for tool registry
        let model = orchestrator.model().to_string();
        let provider_name = orchestrator.provider().to_string();
        let providers = orchestrator.providers();
        let provider = providers.get(&provider_name)
            .ok_or_else(|| anyhow::anyhow!("Provider {} not found", provider_name))?;
        
        tracing::info!(provider_name = %provider_name, "Selected provider for subtask execution");
        
        // Create shared tool registry with provider for ralph and batch tool
        let tool_registry = ToolRegistry::with_provider_arc(Arc::clone(&provider), model.clone());
        let tool_definitions = tool_registry.definitions();
        
        for (idx, subtask) in subtasks.into_iter().enumerate() {
            let model = model.clone();
            let _provider_name = provider_name.clone();
            let provider = Arc::clone(&provider);
            
            // Get context from dependencies
            let context = {
                let completed = completed_results.read().await;
                let mut dep_context = String::new();
                for dep_id in &subtask.dependencies {
                    if let Some(result) = completed.get(dep_id) {
                        dep_context.push_str(&format!("\n--- Result from dependency {} ---\n{}\n", dep_id, result));
                    }
                }
                dep_context
            };
            
            let instruction = subtask.instruction.clone();
            let specialty = subtask.specialty.clone().unwrap_or_default();
            let subtask_id = subtask.id.clone();
            let max_steps = self.config.max_steps_per_subagent;
            let timeout_secs = self.config.subagent_timeout_secs;
            
            // Clone for the async block
            let tools = tool_definitions.clone();
            let registry = Arc::clone(&tool_registry);
            let sem = Arc::clone(&semaphore);
            let stagger_delay = delay_ms * idx as u64;  // Stagger start times
            
            // Spawn the subtask execution with agentic tool loop
            let handle = tokio::spawn(async move {
                // Rate limiting: stagger start and acquire semaphore
                if stagger_delay > 0 {
                    tokio::time::sleep(Duration::from_millis(stagger_delay)).await;
                }
                let _permit = sem.acquire().await.expect("semaphore closed");
                
                let start = Instant::now();
                
// Build the system prompt for this sub-agent
                // Use subtask_id to create unique PRD names for parallel execution
                let prd_filename = format!("prd_{}.json", subtask_id.replace("-", "_"));
                let system_prompt = format!(
                    "You are a {} specialist sub-agent (ID: {}). You have access to tools to complete your task.

IMPORTANT: You MUST use tools to make changes. Do not just describe what to do - actually do it using the tools available.

Available tools:
- read: Read file contents
- write: Write/create files  
- edit: Edit existing files (search and replace)
- multiedit: Make multiple edits at once
- glob: Find files by pattern
- grep: Search file contents
- bash: Run shell commands
- webfetch: Fetch web pages
- prd: Generate structured PRD for complex tasks
- ralph: Run autonomous agent loop on a PRD

COMPLEX TASKS:
If your task is complex and involves multiple implementation steps, use the prd + ralph workflow:
1. Call prd({{action: 'analyze', task_description: '...'}}) to understand what's needed
2. Break down into user stories with acceptance criteria
3. Call prd({{action: 'save', prd_path: '{}', project: '...', feature: '...', stories: [...]}})
4. Call ralph({{action: 'run', prd_path: '{}'}}) to execute

NOTE: Use your unique PRD file '{}' so parallel agents don't conflict.

When done, provide a brief summary of what you accomplished.",
                    specialty,
                    subtask_id,
                    prd_filename,
                    prd_filename,
                    prd_filename
                );
                
                let user_prompt = if context.is_empty() {
                    format!("Complete this task:\n\n{}", instruction)
                } else {
                    format!(
                        "Complete this task:\n\n{}\n\nContext from prior work:\n{}",
                        instruction, context
                    )
                };
                
                // Run the agentic loop
                let result = run_agent_loop(
                    provider,
                    &model,
                    &system_prompt,
                    &user_prompt,
                    tools,
                    registry,
                    max_steps,
                    timeout_secs,
                ).await;
                
                match result {
                    Ok((output, steps, tool_calls)) => {
                        Ok(SubTaskResult {
                            subtask_id: subtask_id.clone(),
                            subagent_id: format!("agent-{}", subtask_id),
                            success: true,
                            result: output,
                            steps,
                            tool_calls,
                            execution_time_ms: start.elapsed().as_millis() as u64,
                            error: None,
                            artifacts: Vec::new(),
                        })
                    }
                    Err(e) => {
                        Ok(SubTaskResult {
                            subtask_id: subtask_id.clone(),
                            subagent_id: format!("agent-{}", subtask_id),
                            success: false,
                            result: String::new(),
                            steps: 0,
                            tool_calls: 0,
                            execution_time_ms: start.elapsed().as_millis() as u64,
                            error: Some(e.to_string()),
                            artifacts: Vec::new(),
                        })
                    }
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for all handles
        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(result)) => results.push(result),
                Ok(Err(e)) => {
                    tracing::error!(provider_name = %provider_name, "Subtask error: {}", e);
                }
                Err(e) => {
                    tracing::error!(provider_name = %provider_name, "Task join error: {}", e);
                }
            }
        }
        
        Ok(results)
    }
    
    /// Aggregate results from all subtasks into a final result
    async fn aggregate_results(&self, results: &[SubTaskResult]) -> Result<String> {
        let mut aggregated = String::new();
        
        for (i, result) in results.iter().enumerate() {
            if result.success {
                aggregated.push_str(&format!(
                    "=== Subtask {} ===\n{}\n\n",
                    i + 1,
                    result.result
                ));
            } else {
                aggregated.push_str(&format!(
                    "=== Subtask {} (FAILED) ===\nError: {}\n\n",
                    i + 1,
                    result.error.as_deref().unwrap_or("Unknown error")
                ));
            }
        }
        
        Ok(aggregated)
    }
    
    /// Execute a single task without decomposition (for simple cases)
    pub async fn execute_single(&self, task: &str) -> Result<SwarmResult> {
        self.execute(task, DecompositionStrategy::None).await
    }
}

/// Builder for swarm execution
pub struct SwarmExecutorBuilder {
    config: SwarmConfig,
}

impl SwarmExecutorBuilder {
    pub fn new() -> Self {
        Self {
            config: SwarmConfig::default(),
        }
    }
    
    pub fn max_subagents(mut self, max: usize) -> Self {
        self.config.max_subagents = max;
        self
    }
    
    pub fn max_steps_per_subagent(mut self, max: usize) -> Self {
        self.config.max_steps_per_subagent = max;
        self
    }
    
    pub fn max_total_steps(mut self, max: usize) -> Self {
        self.config.max_total_steps = max;
        self
    }
    
    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.config.subagent_timeout_secs = secs;
        self
    }
    
    pub fn parallel_enabled(mut self, enabled: bool) -> Self {
        self.config.parallel_enabled = enabled;
        self
    }
    
    pub fn build(self) -> SwarmExecutor {
        SwarmExecutor::new(self.config)
    }
}

impl Default for SwarmExecutorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Run the agentic loop for a sub-agent with tool execution
#[allow(clippy::too_many_arguments)]
async fn run_agent_loop(
    provider: Arc<dyn Provider>,
    model: &str,
    system_prompt: &str,
    user_prompt: &str,
    tools: Vec<crate::provider::ToolDefinition>,
    registry: Arc<ToolRegistry>,
    max_steps: usize,
    timeout_secs: u64,
) -> Result<(String, usize, usize)> {
    // Let the provider handle temperature - K2 models need 0.6 when thinking is disabled
    let temperature = 0.7;
    
    tracing::info!(
        model = %model,
        max_steps = max_steps,
        timeout_secs = timeout_secs,
        "Sub-agent starting agentic loop"
    );
    tracing::debug!(system_prompt = %system_prompt, "Sub-agent system prompt");
    tracing::debug!(user_prompt = %user_prompt, "Sub-agent user prompt");
    
    // Initialize conversation with system and user messages
    let mut messages = vec![
        Message {
            role: Role::System,
            content: vec![ContentPart::Text { text: system_prompt.to_string() }],
        },
        Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: user_prompt.to_string() }],
        },
    ];
    
    let mut steps = 0;
    let mut total_tool_calls = 0;
    let mut final_output = String::new();
    
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    
    loop {
        if steps >= max_steps {
            tracing::warn!(max_steps = max_steps, "Sub-agent reached max steps limit");
            break;
        }
        
        if Instant::now() > deadline {
            tracing::warn!(timeout_secs = timeout_secs, "Sub-agent timed out");
            break;
        }
        
        steps += 1;
        tracing::info!(step = steps, "Sub-agent step starting");
        
        let request = CompletionRequest {
            messages: messages.clone(),
            tools: tools.clone(),
            model: model.to_string(),
            temperature: Some(temperature),
            top_p: None,
            max_tokens: Some(8192),
            stop: Vec::new(),
        };
        
        let step_start = Instant::now();
        let response = timeout(
            Duration::from_secs(120),
            provider.complete(request),
        ).await??;
        let step_duration = step_start.elapsed();
        
        tracing::info!(
            step = steps,
            duration_ms = step_duration.as_millis() as u64,
            finish_reason = ?response.finish_reason,
            prompt_tokens = response.usage.prompt_tokens,
            completion_tokens = response.usage.completion_tokens,
            "Sub-agent step completed LLM call"
        );
        
        // Extract text from response
        let mut text_parts = Vec::new();
        let mut tool_calls = Vec::new();
        
        for part in &response.message.content {
            match part {
                ContentPart::Text { text } => {
                    text_parts.push(text.clone());
                }
                ContentPart::ToolCall { id, name, arguments } => {
                    tool_calls.push((id.clone(), name.clone(), arguments.clone()));
                }
                _ => {}
            }
        }
        
        // Log assistant output
        if !text_parts.is_empty() {
            final_output = text_parts.join("\n");
            tracing::info!(
                step = steps,
                output_len = final_output.len(),
                "Sub-agent text output"
            );
            tracing::debug!(step = steps, output = %final_output, "Sub-agent full output");
        }
        
        // Log tool calls
        if !tool_calls.is_empty() {
            tracing::info!(
                step = steps,
                num_tool_calls = tool_calls.len(),
                tools = ?tool_calls.iter().map(|(_, name, _)| name.as_str()).collect::<Vec<_>>(),
                "Sub-agent requesting tool calls"
            );
        }
        
        // Add assistant message to history
        messages.push(response.message.clone());
        
        // If no tool calls or stop, we're done
        if response.finish_reason != FinishReason::ToolCalls || tool_calls.is_empty() {
            tracing::info!(
                steps = steps, 
                total_tool_calls = total_tool_calls,
                "Sub-agent finished"
            );
            break;
        }
        
        // Execute tool calls
        let mut tool_results = Vec::new();
        
        for (call_id, tool_name, arguments) in tool_calls {
            total_tool_calls += 1;
            
            tracing::info!(
                step = steps,
                tool_call_id = %call_id,
                tool = %tool_name,
                "Executing tool"
            );
            tracing::debug!(
                tool = %tool_name,
                arguments = %arguments,
                "Tool call arguments"
            );
            
            let tool_start = Instant::now();
            let result = if let Some(tool) = registry.get(&tool_name) {
                // Parse arguments as JSON
                let args: serde_json::Value = serde_json::from_str(&arguments)
                    .unwrap_or_else(|_| serde_json::json!({}));
                
                match tool.execute(args).await {
                    Ok(r) => {
                        if r.success {
                            tracing::info!(
                                tool = %tool_name,
                                duration_ms = tool_start.elapsed().as_millis() as u64,
                                success = true,
                                "Tool execution completed"
                            );
                            r.output
                        } else {
                            tracing::warn!(
                                tool = %tool_name,
                                error = %r.output,
                                "Tool returned error"
                            );
                            format!("Tool error: {}", r.output)
                        }
                    }
                    Err(e) => {
                        tracing::error!(
                            tool = %tool_name,
                            error = %e,
                            "Tool execution failed"
                        );
                        format!("Tool execution failed: {}", e)
                    }
                }
            } else {
                tracing::error!(tool = %tool_name, "Unknown tool requested");
                format!("Unknown tool: {}", tool_name)
            };
            
            tracing::debug!(
                tool = %tool_name,
                result_len = result.len(),
                "Tool result"
            );
            
            tool_results.push((call_id, result));
        }
        
        // Add tool results to conversation
        for (call_id, result) in tool_results {
            messages.push(Message {
                role: Role::Tool,
                content: vec![ContentPart::ToolResult {
                    tool_call_id: call_id,
                    content: result,
                }],
            });
        }
    }
    
    Ok((final_output, steps, total_tool_calls))
}
