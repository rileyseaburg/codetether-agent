//! Parallel execution engine for swarm operations
//!
//! Executes subtasks in parallel across multiple sub-agents,
//! respecting dependencies and optimizing for critical path.

use super::{
    DecompositionStrategy, StageStats, SwarmConfig, SwarmResult,
    orchestrator::Orchestrator,
    subtask::{SubTask, SubTaskResult, SubTaskStatus},
};
use crate::tui::swarm_view::{AgentMessageEntry, AgentToolCallDetail, SubTaskInfo, SwarmEvent};

// Re-export swarm types for convenience
pub use super::{Actor, ActorStatus, Handler, SwarmMessage};
use crate::{
    agent::Agent,
    provider::{CompletionRequest, ContentPart, FinishReason, Message, Provider, Role},
    rlm::RlmExecutor,
    swarm::{SwarmArtifact, SwarmStats},
    telemetry::SwarmTelemetryCollector,
    tool::ToolRegistry,
    worktree::{WorktreeInfo, WorktreeManager},
};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{RwLock, mpsc};
use tokio::time::{Duration, timeout};

/// Default context limit (256k tokens - conservative for most models)
const DEFAULT_CONTEXT_LIMIT: usize = 256_000;

/// Reserve tokens for the response generation
const RESPONSE_RESERVE_TOKENS: usize = 8_192;

/// Safety margin before we start truncating (90% of limit)
const TRUNCATION_THRESHOLD: f64 = 0.85;

/// Estimate token count from text (rough heuristic: ~4 chars per token)
fn estimate_tokens(text: &str) -> usize {
    // This is a rough estimate - actual tokenization varies by model
    // Most tokenizers average 3-5 chars per token for English text
    // We use 3.5 to be conservative
    (text.len() as f64 / 3.5).ceil() as usize
}

/// Estimate total tokens in a message
fn estimate_message_tokens(message: &Message) -> usize {
    let mut tokens = 4; // Role overhead

    for part in &message.content {
        tokens += match part {
            ContentPart::Text { text } => estimate_tokens(text),
            ContentPart::ToolCall {
                id,
                name,
                arguments,
            } => estimate_tokens(id) + estimate_tokens(name) + estimate_tokens(arguments) + 10,
            ContentPart::ToolResult {
                tool_call_id,
                content,
            } => estimate_tokens(tool_call_id) + estimate_tokens(content) + 6,
            ContentPart::Image { .. } | ContentPart::File { .. } => 2000, // Binary content is expensive
        };
    }

    tokens
}

/// Estimate total tokens in all messages
fn estimate_total_tokens(messages: &[Message]) -> usize {
    messages.iter().map(estimate_message_tokens).sum()
}

/// Truncate messages to fit within context limit
///
/// Strategy:
/// 1. First, aggressively truncate large tool results
/// 2. Keep system message (first) and user message (second) always
/// 3. Keep the most recent assistant + tool result pairs together
/// 4. Drop oldest middle messages in matched pairs
fn truncate_messages_to_fit(messages: &mut Vec<Message>, context_limit: usize) {
    let target_tokens =
        ((context_limit as f64) * TRUNCATION_THRESHOLD) as usize - RESPONSE_RESERVE_TOKENS;

    let current_tokens = estimate_total_tokens(messages);
    if current_tokens <= target_tokens {
        return;
    }

    tracing::warn!(
        current_tokens = current_tokens,
        target_tokens = target_tokens,
        context_limit = context_limit,
        "Context approaching limit, truncating conversation history"
    );

    // FIRST: Aggressively truncate large tool results (this is the main culprit)
    truncate_large_tool_results(messages, 2000); // Max 2k tokens per tool result

    let after_tool_truncation = estimate_total_tokens(messages);
    if after_tool_truncation <= target_tokens {
        tracing::info!(
            old_tokens = current_tokens,
            new_tokens = after_tool_truncation,
            "Truncated large tool results, context now within limits"
        );
        return;
    }

    // Minimum: keep first 2 (system + initial user) and last 4 messages
    if messages.len() <= 6 {
        tracing::warn!(
            tokens = after_tool_truncation,
            target = target_tokens,
            "Cannot truncate further - conversation too short"
        );
        return;
    }

    // Remove messages from the middle, keeping first 2 and last 4
    // But we need to remove in pairs to maintain tool_call_id references
    let keep_start = 2;
    let keep_end = 4;
    let removable_count = messages.len() - keep_start - keep_end;

    if removable_count == 0 {
        return;
    }

    // Remove all middle messages
    let removed_messages: Vec<_> = messages
        .drain(keep_start..keep_start + removable_count)
        .collect();
    let summary = summarize_removed_messages(&removed_messages);

    // Insert a summary message where we removed content
    messages.insert(
        keep_start,
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: format!(
                    "[Context truncated: {} earlier messages removed to fit context window]\n{}",
                    removed_messages.len(),
                    summary
                ),
            }],
        },
    );

    let new_tokens = estimate_total_tokens(messages);
    tracing::info!(
        removed_messages = removed_messages.len(),
        old_tokens = current_tokens,
        new_tokens = new_tokens,
        "Truncated conversation history"
    );
}

/// Summarize removed messages for context preservation
fn summarize_removed_messages(messages: &[Message]) -> String {
    let mut summary = String::new();
    let mut tool_calls: Vec<String> = Vec::new();

    for msg in messages {
        for part in &msg.content {
            if let ContentPart::ToolCall { name, .. } = part {
                if !tool_calls.contains(name) {
                    tool_calls.push(name.clone());
                }
            }
        }
    }

    if !tool_calls.is_empty() {
        summary.push_str(&format!(
            "Tools used in truncated history: {}",
            tool_calls.join(", ")
        ));
    }

    summary
}

/// Truncate large tool results that exceed a reasonable size
fn truncate_large_tool_results(messages: &mut [Message], max_tokens_per_result: usize) {
    let char_limit = max_tokens_per_result * 3; // ~3 chars per token
    let mut truncated_count = 0;
    let mut saved_tokens = 0usize;

    for message in messages.iter_mut() {
        for part in message.content.iter_mut() {
            if let ContentPart::ToolResult { content, .. } = part {
                let tokens = estimate_tokens(content);
                if tokens > max_tokens_per_result {
                    let old_len = content.len();
                    *content = truncate_single_result(content, char_limit);
                    saved_tokens += tokens.saturating_sub(estimate_tokens(content));
                    if content.len() < old_len {
                        truncated_count += 1;
                    }
                }
            }
        }
    }

    if truncated_count > 0 {
        tracing::info!(
            truncated_count = truncated_count,
            saved_tokens = saved_tokens,
            max_tokens_per_result = max_tokens_per_result,
            "Truncated large tool results"
        );
    }
}

/// Truncate a single result string to a maximum character limit
fn truncate_single_result(content: &str, max_chars: usize) -> String {
    if content.len() <= max_chars {
        return content.to_string();
    }

    // Find a valid char boundary at or before max_chars
    let safe_limit = {
        let mut limit = max_chars.min(content.len());
        while limit > 0 && !content.is_char_boundary(limit) {
            limit -= 1;
        }
        limit
    };

    // Try to find a good break point (newline) near the limit
    let break_point = content[..safe_limit].rfind('\n').unwrap_or(safe_limit);

    let truncated = format!(
        "{}...\n\n[OUTPUT TRUNCATED: {} → {} chars to fit context limit]",
        &content[..break_point],
        content.len(),
        break_point
    );

    tracing::debug!(
        original_len = content.len(),
        truncated_len = truncated.len(),
        "Truncated large result"
    );

    truncated
}

/// Threshold for when to use RLM vs truncation (in characters)
const RLM_THRESHOLD_CHARS: usize = 50_000;

/// Max chars for simple truncation (below RLM threshold)
const SIMPLE_TRUNCATE_CHARS: usize = 6000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentLoopExit {
    Completed,
    MaxStepsReached,
    TimedOut,
}

/// Process a large tool result using RLM to intelligently summarize it
async fn process_large_result_with_rlm(
    content: &str,
    tool_name: &str,
    provider: Arc<dyn Provider>,
    model: &str,
) -> String {
    if content.len() <= SIMPLE_TRUNCATE_CHARS {
        return content.to_string();
    }

    // For medium-sized content, just truncate
    if content.len() <= RLM_THRESHOLD_CHARS {
        return truncate_single_result(content, SIMPLE_TRUNCATE_CHARS);
    }

    // For very large content, use RLM to intelligently summarize
    tracing::info!(
        tool = %tool_name,
        content_len = content.len(),
        "Using RLM to process large tool result"
    );

    let query = format!(
        "Summarize the key information from this {} output. \
         Focus on: errors, warnings, important findings, and actionable items. \
         Be concise but thorough.",
        tool_name
    );

    let mut executor =
        RlmExecutor::new(content.to_string(), provider, model.to_string()).with_max_iterations(3);

    match executor.analyze(&query).await {
        Ok(result) => {
            tracing::info!(
                tool = %tool_name,
                original_len = content.len(),
                summary_len = result.answer.len(),
                iterations = result.iterations,
                "RLM summarized large result"
            );

            format!(
                "[RLM Summary of {} output ({} chars → {} chars)]\n\n{}",
                tool_name,
                content.len(),
                result.answer.len(),
                result.answer
            )
        }
        Err(e) => {
            tracing::warn!(
                tool = %tool_name,
                error = %e,
                "RLM analysis failed, falling back to truncation"
            );
            truncate_single_result(content, SIMPLE_TRUNCATE_CHARS)
        }
    }
}

/// The swarm executor runs subtasks in parallel
pub struct SwarmExecutor {
    config: SwarmConfig,
    /// Optional agent for handling swarm-level coordination (reserved for future use)
    coordinator_agent: Option<Arc<tokio::sync::Mutex<Agent>>>,
    /// Optional event channel for TUI real-time updates
    event_tx: Option<mpsc::Sender<SwarmEvent>>,
    /// Telemetry collector for swarm execution metrics
    telemetry: Arc<tokio::sync::Mutex<SwarmTelemetryCollector>>,
}

impl SwarmExecutor {
    /// Create a new executor
    pub fn new(config: SwarmConfig) -> Self {
        Self {
            config,
            coordinator_agent: None,
            event_tx: None,
            telemetry: Arc::new(tokio::sync::Mutex::new(SwarmTelemetryCollector::default())),
        }
    }

    /// Set an event channel for real-time TUI updates
    pub fn with_event_tx(mut self, tx: mpsc::Sender<SwarmEvent>) -> Self {
        self.event_tx = Some(tx);
        self
    }

    /// Set a coordinator agent for swarm-level coordination
    pub fn with_coordinator_agent(mut self, agent: Arc<tokio::sync::Mutex<Agent>>) -> Self {
        tracing::debug!("Setting coordinator agent for swarm execution");
        self.coordinator_agent = Some(agent);
        self
    }

    /// Set a telemetry collector for swarm execution metrics
    pub fn with_telemetry(
        mut self,
        telemetry: Arc<tokio::sync::Mutex<SwarmTelemetryCollector>>,
    ) -> Self {
        self.telemetry = telemetry;
        self
    }

    /// Get the telemetry collector as an Arc
    pub fn telemetry_arc(&self) -> Arc<tokio::sync::Mutex<SwarmTelemetryCollector>> {
        Arc::clone(&self.telemetry)
    }
    /// Get the coordinator agent if set
    pub fn coordinator_agent(&self) -> Option<&Arc<tokio::sync::Mutex<Agent>>> {
        self.coordinator_agent.as_ref()
    }

    /// Send an event to the TUI if channel is connected (non-blocking)
    fn try_send_event(&self, event: SwarmEvent) {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.try_send(event);
        }
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
            self.try_send_event(SwarmEvent::Error("No subtasks generated".to_string()));
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

        self.try_send_event(SwarmEvent::Started {
            task: task.to_string(),
            total_subtasks: subtasks.len(),
        });

        // Emit decomposition event for TUI
        self.try_send_event(SwarmEvent::Decomposed {
            subtasks: subtasks
                .iter()
                .map(|s| SubTaskInfo {
                    id: s.id.clone(),
                    name: s.name.clone(),
                    status: SubTaskStatus::Pending,
                    stage: s.stage,
                    dependencies: s.dependencies.clone(),
                    agent_name: s.specialty.clone(),
                    current_tool: None,
                    steps: 0,
                    max_steps: self.config.max_steps_per_subagent,
                    tool_call_history: Vec::new(),
                    messages: Vec::new(),
                    output: None,
                    error: None,
                })
                .collect(),
        });

        // Execute stages in order
        let max_stage = subtasks.iter().map(|s| s.stage).max().unwrap_or(0);
        let mut all_results: Vec<SubTaskResult> = Vec::new();
        let artifacts: Vec<SwarmArtifact> = Vec::new();

        // Initialize telemetry for this swarm execution
        let swarm_id = uuid::Uuid::new_v4().to_string();
        self.telemetry.lock().await.start_swarm(
            swarm_id.clone(),
            subtasks.len(),
            &format!("{:?}", strategy),
        );

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
                    &swarm_id,
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

            // Emit stage complete event
            let stage_completed = stage_results.iter().filter(|r| r.success).count();
            let stage_failed = stage_results.iter().filter(|r| !r.success).count();
            self.try_send_event(SwarmEvent::StageComplete {
                stage,
                completed: stage_completed,
                failed: stage_failed,
            });

            all_results.extend(stage_results);
        }

        // Get provider name before mutable borrow
        let provider_name = orchestrator.provider().to_string();

        // Record overall execution latency
        self.telemetry
            .lock()
            .await
            .record_swarm_latency("total_execution", start_time.elapsed());

        // Calculate final stats
        let stats = orchestrator.stats_mut();
        stats.execution_time_ms = start_time.elapsed().as_millis() as u64;
        stats.sequential_time_estimate_ms = all_results.iter().map(|r| r.execution_time_ms).sum();
        stats.calculate_critical_path();
        stats.calculate_speedup();

        // Aggregate results
        let success = all_results.iter().all(|r| r.success);

        // Complete telemetry collection
        let _telemetry_metrics = self.telemetry.lock().await.complete_swarm(success);
        let result = self.aggregate_results(&all_results).await?;

        tracing::info!(
            provider_name = %provider_name,
            "Swarm execution complete: {} subtasks, {:.1}x speedup",
            all_results.len(),
            stats.speedup_factor
        );

        let final_stats = orchestrator.stats().clone();
        self.try_send_event(SwarmEvent::Complete {
            success,
            stats: final_stats.clone(),
        });

        Ok(SwarmResult {
            success,
            result,
            subtask_results: all_results,
            stats: final_stats,
            artifacts,
            error: None,
        })
    }

    /// Execute a single stage of subtasks in parallel (with rate limiting and worktree isolation)
    async fn execute_stage(
        &self,
        orchestrator: &Orchestrator,
        subtasks: Vec<SubTask>,
        completed_results: Arc<RwLock<HashMap<String, String>>>,
        swarm_id: &str,
    ) -> Result<Vec<SubTaskResult>> {
        let mut handles: Vec<(
            String,
            tokio::task::JoinHandle<Result<(SubTaskResult, Option<WorktreeInfo>), anyhow::Error>>,
        )> = Vec::new();

        // Rate limiting: semaphore for max concurrent requests
        let semaphore = Arc::new(tokio::sync::Semaphore::new(
            self.config.max_concurrent_requests,
        ));
        let delay_ms = self.config.request_delay_ms;

        // Get provider info for tool registry
        let model = orchestrator.model().to_string();
        let provider_name = orchestrator.provider().to_string();
        let providers = orchestrator.providers();
        let provider = providers
            .get(&provider_name)
            .ok_or_else(|| anyhow::anyhow!("Provider {} not found", provider_name))?;

        tracing::info!(provider_name = %provider_name, "Selected provider for subtask execution");

        // Create shared tool registry with provider for ralph and batch tool
        let tool_registry = ToolRegistry::with_provider_arc(Arc::clone(&provider), model.clone());
        // Filter out 'question' tool - sub-agents must be autonomous, not interactive
        let tool_definitions: Vec<_> = tool_registry
            .definitions()
            .into_iter()
            .filter(|t| t.name != "question")
            .collect();

        // Create worktree manager if enabled
        let worktree_manager = if self.config.worktree_enabled {
            let working_dir = self.config.working_dir.clone().unwrap_or_else(|| {
                std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string())
            });

            match WorktreeManager::new(&working_dir) {
                Ok(mgr) => {
                    tracing::info!(
                        working_dir = %working_dir,
                        "Worktree isolation enabled for parallel sub-agents"
                    );
                    Some(Arc::new(mgr) as Arc<WorktreeManager>)
                }
                Err(e) => {
                    tracing::warn!(
                        error = %e,
                        "Failed to create worktree manager, falling back to shared directory"
                    );
                    None
                }
            }
        } else {
            None
        };

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
                        dep_context.push_str(&format!(
                            "\n--- Result from dependency {} ---\n{}\n",
                            dep_id, result
                        ));
                    }
                }
                dep_context
            };

            let instruction = subtask.instruction.clone();
            let subtask_name = subtask.name.clone();
            let specialty = subtask.specialty.clone().unwrap_or_default();
            let subtask_id = subtask.id.clone();
            let subtask_id_for_handle = subtask_id.clone();
            let max_steps = self.config.max_steps_per_subagent;
            let timeout_secs = self.config.subagent_timeout_secs;

            // Clone for the async block
            let tools = tool_definitions.clone();
            let registry = Arc::clone(&tool_registry);
            let sem = Arc::clone(&semaphore);
            let stagger_delay = delay_ms * idx as u64; // Stagger start times
            let worktree_mgr = worktree_manager.clone();
            let event_tx = self.event_tx.clone();

            // Generate sub-agent execution ID
            let subagent_id = format!("agent-{}", uuid::Uuid::new_v4());

            // Log telemetry for this sub-agent
            tracing::debug!(subagent_id = %subagent_id, swarm_id = %swarm_id, subtask = %subtask_id, specialty = %specialty, "Starting sub-agent");

            // Spawn the subtask execution with agentic tool loop
            let handle = tokio::spawn(async move {
                // Rate limiting: stagger start and acquire semaphore
                if stagger_delay > 0 {
                    tokio::time::sleep(Duration::from_millis(stagger_delay)).await;
                }
                let _permit = sem
                    .acquire()
                    .await
                    .map_err(|_| anyhow::anyhow!("Swarm execution cancelled"))?;

                let _agent_start = Instant::now();

                let start = Instant::now();

                // Create worktree for this sub-agent if enabled
                let worktree_info = if let Some(ref mgr) = worktree_mgr {
                    let task_slug = subtask_id.replace("-", "_");
                    match mgr.create(&task_slug) {
                        Ok(wt) => {
                            tracing::info!(
                                subtask_id = %subtask_id,
                                worktree_path = %wt.path.display(),
                                worktree_branch = %wt.branch,
                                "Created worktree for sub-agent"
                            );
                            Some(wt)
                        }
                        Err(e) => {
                            tracing::warn!(
                                subtask_id = %subtask_id,
                                error = %e,
                                "Failed to create worktree, using shared directory"
                            );
                            None
                        }
                    }
                } else {
                    None
                };

                // Determine working directory
                let working_dir = worktree_info
                    .as_ref()
                    .map(|wt| wt.path.display().to_string())
                    .unwrap_or_else(|| ".".to_string());

                // Load AGENTS.md from working directory
                let working_path = std::path::Path::new(&working_dir);
                let agents_md_content = crate::agent::builtin::load_agents_md(working_path)
                    .map(|(content, _)| {
                        format!("\n\nPROJECT INSTRUCTIONS (from AGENTS.md):\n{content}")
                    })
                    .unwrap_or_default();

                // Build the system prompt for this sub-agent
                let prd_filename = format!("prd_{}.json", subtask_id.replace("-", "_"));
                let system_prompt = format!(
                    "You are a {} specialist sub-agent (ID: {}). You have access to tools to complete your task.

WORKING DIRECTORY: {}
All file operations should be relative to this directory.

IMPORTANT: You MUST use tools to make changes. Do not just describe what to do - actually do it using the tools available.

Available tools:
- read: Read file contents
- write: Write/create files  
- edit: Edit existing files (search and replace)
- multiedit: Make multiple edits at once
- glob: Find files by pattern
- grep: Search file contents
- bash: Run shell commands (use cwd: \"{}\" parameter)
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

When done, provide a brief summary of what you accomplished.{agents_md_content}",
                    specialty,
                    subtask_id,
                    working_dir,
                    working_dir,
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

                // Emit AgentStarted event
                if let Some(ref tx) = event_tx {
                    let _ = tx.try_send(SwarmEvent::SubTaskUpdate {
                        id: subtask_id.clone(),
                        name: subtask_name.clone(),
                        status: SubTaskStatus::Running,
                        agent_name: Some(format!("agent-{}", subtask_id)),
                    });
                    let _ = tx.try_send(SwarmEvent::AgentStarted {
                        subtask_id: subtask_id.clone(),
                        agent_name: format!("agent-{}", subtask_id),
                        specialty: specialty.clone(),
                    });
                }

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
                    event_tx.clone(),
                    subtask_id.clone(),
                )
                .await;

                match result {
                    Ok((output, steps, tool_calls, exit_reason)) => {
                        let (success, status, error) = match exit_reason {
                            AgentLoopExit::Completed => (true, SubTaskStatus::Completed, None),
                            AgentLoopExit::MaxStepsReached => (
                                false,
                                SubTaskStatus::Failed,
                                Some(format!("Sub-agent hit max steps ({max_steps})")),
                            ),
                            AgentLoopExit::TimedOut => (
                                false,
                                SubTaskStatus::TimedOut,
                                Some(format!("Sub-agent timed out after {timeout_secs}s")),
                            ),
                        };

                        // Emit completion events
                        if let Some(ref tx) = event_tx {
                            let _ = tx.try_send(SwarmEvent::SubTaskUpdate {
                                id: subtask_id.clone(),
                                name: subtask_name.clone(),
                                status,
                                agent_name: Some(format!("agent-{}", subtask_id)),
                            });
                            if let Some(ref message) = error {
                                let _ = tx.try_send(SwarmEvent::AgentError {
                                    subtask_id: subtask_id.clone(),
                                    error: message.clone(),
                                });
                            }
                            let _ = tx.try_send(SwarmEvent::AgentOutput {
                                subtask_id: subtask_id.clone(),
                                output: output.clone(),
                            });
                            let _ = tx.try_send(SwarmEvent::AgentComplete {
                                subtask_id: subtask_id.clone(),
                                success,
                                steps,
                            });
                        }
                        Ok((
                            SubTaskResult {
                                subtask_id: subtask_id.clone(),
                                subagent_id: format!("agent-{}", subtask_id),
                                success,
                                result: output,
                                steps,
                                tool_calls,
                                execution_time_ms: start.elapsed().as_millis() as u64,
                                error,
                                artifacts: Vec::new(),
                            },
                            worktree_info,
                        ))
                    }
                    Err(e) => {
                        // Emit error events
                        if let Some(ref tx) = event_tx {
                            let _ = tx.try_send(SwarmEvent::SubTaskUpdate {
                                id: subtask_id.clone(),
                                name: subtask_name.clone(),
                                status: SubTaskStatus::Failed,
                                agent_name: Some(format!("agent-{}", subtask_id)),
                            });
                            let _ = tx.try_send(SwarmEvent::AgentError {
                                subtask_id: subtask_id.clone(),
                                error: e.to_string(),
                            });
                            let _ = tx.try_send(SwarmEvent::AgentComplete {
                                subtask_id: subtask_id.clone(),
                                success: false,
                                steps: 0,
                            });
                        }
                        Ok((
                            SubTaskResult {
                                subtask_id: subtask_id.clone(),
                                subagent_id: format!("agent-{}", subtask_id),
                                success: false,
                                result: String::new(),
                                steps: 0,
                                tool_calls: 0,
                                execution_time_ms: start.elapsed().as_millis() as u64,
                                error: Some(e.to_string()),
                                artifacts: Vec::new(),
                            },
                            worktree_info,
                        ))
                    }
                }
            });

            handles.push((subtask_id_for_handle, handle));
        }

        // Wait for all handles and handle worktree merging
        let mut results = Vec::new();
        let auto_merge = self.config.worktree_auto_merge;

        for (subtask_id, handle) in handles {
            match handle.await {
                Ok(Ok((mut result, worktree_info))) => {
                    // Handle worktree merge if applicable
                    if let Some(wt) = worktree_info {
                        if result.success && auto_merge {
                            if let Some(ref mgr) = worktree_manager {
                                match mgr.merge(&wt) {
                                    Ok(merge_result) => {
                                        if merge_result.success {
                                            tracing::info!(
                                                subtask_id = %result.subtask_id,
                                                files_changed = merge_result.files_changed,
                                                "Merged worktree changes successfully"
                                            );
                                            result.result.push_str(&format!(
                                                "\n\n--- Merge Result ---\n{}",
                                                merge_result.summary
                                            ));
                                        } else if merge_result.aborted {
                                            // Merge was aborted due to non-conflict failure
                                            tracing::warn!(
                                                subtask_id = %result.subtask_id,
                                                summary = %merge_result.summary,
                                                "Merge was aborted"
                                            );
                                            result.result.push_str(&format!(
                                                "\n\n--- Merge Aborted ---\n{}",
                                                merge_result.summary
                                            ));
                                        } else {
                                            tracing::warn!(
                                                subtask_id = %result.subtask_id,
                                                conflicts = ?merge_result.conflicts,
                                                "Merge had conflicts"
                                            );
                                            result.result.push_str(&format!(
                                                "\n\n--- Merge Conflicts ---\n{}",
                                                merge_result.summary
                                            ));
                                        }

                                        // Cleanup worktree after merge
                                        if let Err(e) = mgr.cleanup(&wt) {
                                            tracing::warn!(
                                                error = %e,
                                                "Failed to cleanup worktree"
                                            );
                                        }
                                    }
                                    Err(e) => {
                                        tracing::error!(
                                            subtask_id = %result.subtask_id,
                                            error = %e,
                                            "Failed to merge worktree"
                                        );
                                    }
                                }
                            }
                        } else if !result.success {
                            // Keep worktree for debugging on failure
                            tracing::info!(
                                subtask_id = %result.subtask_id,
                                worktree_path = %wt.path.display(),
                                "Keeping worktree for debugging (task failed)"
                            );
                        }
                    }

                    results.push(result);
                }
                Ok(Err(e)) => {
                    tracing::error!(provider_name = %provider_name, "Subtask error: {}", e);
                    if let Some(ref tx) = self.event_tx {
                        let _ = tx.try_send(SwarmEvent::SubTaskUpdate {
                            id: subtask_id.clone(),
                            name: subtask_id.clone(),
                            status: SubTaskStatus::Failed,
                            agent_name: Some(format!("agent-{}", subtask_id)),
                        });
                        let _ = tx.try_send(SwarmEvent::AgentError {
                            subtask_id: subtask_id.clone(),
                            error: e.to_string(),
                        });
                        let _ = tx.try_send(SwarmEvent::AgentComplete {
                            subtask_id: subtask_id.clone(),
                            success: false,
                            steps: 0,
                        });
                    }
                    results.push(SubTaskResult {
                        subtask_id: subtask_id.clone(),
                        subagent_id: format!("agent-{}", subtask_id),
                        success: false,
                        result: String::new(),
                        steps: 0,
                        tool_calls: 0,
                        execution_time_ms: 0,
                        error: Some(e.to_string()),
                        artifacts: Vec::new(),
                    });
                }
                Err(e) => {
                    tracing::error!(provider_name = %provider_name, "Task join error: {}", e);
                    if let Some(ref tx) = self.event_tx {
                        let _ = tx.try_send(SwarmEvent::SubTaskUpdate {
                            id: subtask_id.clone(),
                            name: subtask_id.clone(),
                            status: SubTaskStatus::Failed,
                            agent_name: Some(format!("agent-{}", subtask_id)),
                        });
                        let _ = tx.try_send(SwarmEvent::AgentError {
                            subtask_id: subtask_id.clone(),
                            error: format!("Task join error: {}", e),
                        });
                        let _ = tx.try_send(SwarmEvent::AgentComplete {
                            subtask_id: subtask_id.clone(),
                            success: false,
                            steps: 0,
                        });
                    }
                    results.push(SubTaskResult {
                        subtask_id: subtask_id.clone(),
                        subagent_id: format!("agent-{}", subtask_id),
                        success: false,
                        result: String::new(),
                        steps: 0,
                        tool_calls: 0,
                        execution_time_ms: 0,
                        error: Some(format!("Task join error: {}", e)),
                        artifacts: Vec::new(),
                    });
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
                aggregated.push_str(&format!("=== Subtask {} ===\n{}\n\n", i + 1, result.result));
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
/// Run an agentic loop with tools - reusable for Ralph and swarm sub-agents
pub async fn run_agent_loop(
    provider: Arc<dyn Provider>,
    model: &str,
    system_prompt: &str,
    user_prompt: &str,
    tools: Vec<crate::provider::ToolDefinition>,
    registry: Arc<ToolRegistry>,
    max_steps: usize,
    timeout_secs: u64,
    event_tx: Option<mpsc::Sender<SwarmEvent>>,
    subtask_id: String,
) -> Result<(String, usize, usize, AgentLoopExit)> {
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
            content: vec![ContentPart::Text {
                text: system_prompt.to_string(),
            }],
        },
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: user_prompt.to_string(),
            }],
        },
    ];

    let mut steps = 0;
    let mut total_tool_calls = 0;
    let mut final_output = String::new();

    let mut deadline = Instant::now() + Duration::from_secs(timeout_secs);

    let exit_reason = loop {
        if steps >= max_steps {
            tracing::warn!(max_steps = max_steps, "Sub-agent reached max steps limit");
            break AgentLoopExit::MaxStepsReached;
        }

        if Instant::now() > deadline {
            tracing::warn!(timeout_secs = timeout_secs, "Sub-agent timed out");
            break AgentLoopExit::TimedOut;
        }

        steps += 1;
        tracing::info!(step = steps, "Sub-agent step starting");

        // Check context size and truncate if approaching limit
        truncate_messages_to_fit(&mut messages, DEFAULT_CONTEXT_LIMIT);

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
        let response = timeout(Duration::from_secs(120), provider.complete(request)).await??;
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
                ContentPart::ToolCall {
                    id,
                    name,
                    arguments,
                } => {
                    tool_calls.push((id.clone(), name.clone(), arguments.clone()));
                }
                _ => {}
            }
        }

        // Log assistant output
        if !text_parts.is_empty() {
            let step_output = text_parts.join("\n");
            if !final_output.is_empty() {
                final_output.push('\n');
            }
            final_output.push_str(&step_output);
            tracing::info!(
                step = steps,
                output_len = final_output.len(),
                "Sub-agent text output"
            );
            tracing::debug!(step = steps, output = %final_output, "Sub-agent full output");

            // Emit assistant message event for TUI detail view
            if let Some(ref tx) = event_tx {
                let preview = if step_output.len() > 500 {
                    let mut end = 500;
                    while end > 0 && !step_output.is_char_boundary(end) {
                        end -= 1;
                    }
                    format!("{}...", &step_output[..end])
                } else {
                    step_output.clone()
                };
                let _ = tx.try_send(SwarmEvent::AgentMessage {
                    subtask_id: subtask_id.clone(),
                    entry: AgentMessageEntry {
                        role: "assistant".to_string(),
                        content: preview,
                        is_tool_call: false,
                    },
                });
            }
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
            break AgentLoopExit::Completed;
        }

        // Execute tool calls
        let mut tool_results = Vec::new();

        for (call_id, tool_name, arguments) in tool_calls {
            total_tool_calls += 1;

            // Emit tool call event
            if let Some(ref tx) = event_tx {
                let _ = tx.try_send(SwarmEvent::AgentToolCall {
                    subtask_id: subtask_id.clone(),
                    tool_name: tool_name.clone(),
                });
            }

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
            let mut tool_success = true;
            let result = if let Some(tool) = registry.get(&tool_name) {
                // Parse arguments as JSON
                let args: serde_json::Value =
                    serde_json::from_str(&arguments).unwrap_or_else(|e| {
                        tracing::warn!(tool = %tool_name, error = %e, raw = %arguments, "Failed to parse tool arguments");
                        serde_json::json!({})
                    });

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
                            tool_success = false;
                            tracing::warn!(
                                tool = %tool_name,
                                error = %r.output,
                                "Tool returned error"
                            );
                            format!("Tool error: {}", r.output)
                        }
                    }
                    Err(e) => {
                        tool_success = false;
                        tracing::error!(
                            tool = %tool_name,
                            error = %e,
                            "Tool execution failed"
                        );
                        format!("Tool execution failed: {}", e)
                    }
                }
            } else {
                tool_success = false;
                tracing::error!(tool = %tool_name, "Unknown tool requested");
                format!("Unknown tool: {}", tool_name)
            };

            // Emit detailed tool call event
            if let Some(ref tx) = event_tx {
                let input_preview = if arguments.len() > 200 {
                    let mut end = 200;
                    while end > 0 && !arguments.is_char_boundary(end) {
                        end -= 1;
                    }
                    format!("{}...", &arguments[..end])
                } else {
                    arguments.clone()
                };
                let output_preview = if result.len() > 500 {
                    let mut end = 500;
                    while end > 0 && !result.is_char_boundary(end) {
                        end -= 1;
                    }
                    format!("{}...", &result[..end])
                } else {
                    result.clone()
                };
                let _ = tx.try_send(SwarmEvent::AgentToolCallDetail {
                    subtask_id: subtask_id.clone(),
                    detail: AgentToolCallDetail {
                        tool_name: tool_name.clone(),
                        input_preview,
                        output_preview,
                        success: tool_success,
                    },
                });
            }

            tracing::debug!(
                tool = %tool_name,
                result_len = result.len(),
                "Tool result"
            );

            // Process large results with RLM or truncate smaller ones
            let result = if result.len() > RLM_THRESHOLD_CHARS {
                // Use RLM for very large results
                process_large_result_with_rlm(&result, &tool_name, Arc::clone(&provider), model)
                    .await
            } else {
                // Simple truncation for medium results
                truncate_single_result(&result, SIMPLE_TRUNCATE_CHARS)
            };

            tool_results.push((call_id, tool_name, result));
        }

        // Add tool results to conversation
        for (call_id, _tool_name, result) in tool_results {
            messages.push(Message {
                role: Role::Tool,
                content: vec![ContentPart::ToolResult {
                    tool_call_id: call_id,
                    content: result,
                }],
            });
        }

        // Reset deadline after each successful step — agent is making progress
        deadline = Instant::now() + Duration::from_secs(timeout_secs);
    };

    Ok((final_output, steps, total_tool_calls, exit_reason))
}
