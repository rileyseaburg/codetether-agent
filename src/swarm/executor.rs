//! Parallel execution engine for swarm operations
//!
//! Executes subtasks in parallel across multiple sub-agents,
//! respecting dependencies and optimizing for critical path.

use super::{
    BranchObservation, BranchRuntimeState, CacheConfig, CacheStats, CollapseController,
    CollapsePolicy, DecompositionStrategy, ExecutionMode, StageStats, SwarmCache, SwarmConfig,
    SwarmResult,
    kubernetes_executor::{
        RemoteSubtaskPayload, SWARM_SUBTASK_PAYLOAD_ENV, encode_payload, latest_probe_from_logs,
        probe_changed_files_set, result_from_logs,
    },
    orchestrator::Orchestrator,
    result_store::ResultStore,
    subtask::{SubTask, SubTaskResult, SubTaskStatus},
};
use crate::bus::{AgentBus, BusMessage};
use crate::k8s::{K8sManager, SubagentPodSpec, SubagentPodState};
use crate::tui::swarm_view::{AgentMessageEntry, AgentToolCallDetail, SubTaskInfo, SwarmEvent};

// Re-export swarm types for convenience
pub use super::SwarmMessage;
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
use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{RwLock, mpsc};
use tokio::task::AbortHandle;
use tokio::time::{Duration, MissedTickBehavior, timeout};

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
                ..
            } => estimate_tokens(id) + estimate_tokens(name) + estimate_tokens(arguments) + 10,
            ContentPart::ToolResult {
                tool_call_id,
                content,
            } => estimate_tokens(tool_call_id) + estimate_tokens(content) + 6,
            ContentPart::Image { .. } | ContentPart::File { .. } => 2000, // Binary content is expensive
            ContentPart::Thinking { text } => estimate_tokens(text),
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
            if let ContentPart::ToolCall { name, .. } = part
                && !tool_calls.contains(name)
            {
                tool_calls.push(name.clone());
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

/// Collapse controller sampling interval for branch health.
const COLLAPSE_SAMPLE_SECS: u64 = 5;
const SWARM_FALLBACK_PROMPT_ENV: &str = "CODETETHER_SWARM_FALLBACK_PROMPT";
const SWARM_FALLBACK_MODEL_ENV: &str = "CODETETHER_SWARM_FALLBACK_MODEL";
const K8S_PASSTHROUGH_ENV_VARS: &[&str] = &[
    "VAULT_ADDR",
    "VAULT_TOKEN",
    "VAULT_MOUNT",
    "VAULT_SECRETS_PATH",
    "VAULT_NAMESPACE",
    "CODETETHER_AUTH_TOKEN",
];

#[derive(Debug, Clone)]
struct ActiveK8sBranch {
    branch: String,
    started_at: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentLoopExit {
    Completed,
    MaxStepsReached,
    TimedOut,
}

/// Calculate the delay for exponential backoff
///
/// Uses the formula: min(initial_delay * multiplier^attempt, max_delay)
fn calculate_backoff_delay(
    attempt: u32,
    initial_delay_ms: u64,
    max_delay_ms: u64,
    multiplier: f64,
) -> Duration {
    let delay_ms =
        (initial_delay_ms as f64 * multiplier.powi(attempt as i32)).min(max_delay_ms as f64);
    Duration::from_millis(delay_ms as u64)
}

fn compute_resource_health(pod_state: Option<&SubagentPodState>) -> (f32, u32) {
    let Some(pod_state) = pod_state else {
        return (0.2, 1);
    };

    let reason = pod_state
        .reason
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let phase = pod_state.phase.to_ascii_lowercase();

    if reason.contains("oomkilled") {
        return (0.0, 3);
    }
    if reason.contains("imagepullbackoff") || reason.contains("errimagepull") {
        return (0.0, 3);
    }
    if reason.contains("crashloopbackoff") {
        return (0.1, 2);
    }
    if phase == "failed" {
        return (0.1, 2);
    }

    let mut score = 1.0f32;
    let mut unhealthy_signals = 0u32;

    if !pod_state.ready {
        score -= 0.2;
    }
    if !reason.is_empty() {
        score -= 0.3;
        unhealthy_signals += 1;
    }
    if pod_state.restart_count > 0 {
        score -= (pod_state.restart_count.min(3) as f32) * 0.2;
        unhealthy_signals += 1;
    }

    (score.clamp(0.0, 1.0), unhealthy_signals)
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
            let bounded_answer = truncate_single_result(&result.answer, SIMPLE_TRUNCATE_CHARS * 2);
            tracing::info!(
                tool = %tool_name,
                original_len = content.len(),
                summary_len = bounded_answer.len(),
                iterations = result.iterations,
                "RLM summarized large result"
            );

            format!(
                "[RLM Summary of {} output ({} chars → {} chars)]\n\n{}",
                tool_name,
                content.len(),
                bounded_answer.len(),
                bounded_answer
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
    /// Cache for avoiding duplicate subtask execution
    cache: Option<Arc<tokio::sync::Mutex<SwarmCache>>>,
    /// Shared result store for sub-agent result sharing
    result_store: Arc<ResultStore>,
    /// Optional agent bus for inter-agent communication
    bus: Option<Arc<AgentBus>>,
}

impl SwarmExecutor {
    /// Create a new executor
    pub fn new(config: SwarmConfig) -> Self {
        Self {
            config,
            coordinator_agent: None,
            event_tx: None,
            telemetry: Arc::new(tokio::sync::Mutex::new(SwarmTelemetryCollector::default())),
            cache: None,
            result_store: ResultStore::new_arc(),
            bus: None,
        }
    }

    /// Create a new executor with caching enabled
    pub async fn with_cache(config: SwarmConfig, cache_config: CacheConfig) -> Result<Self> {
        let cache = SwarmCache::new(cache_config).await?;
        Ok(Self {
            config,
            coordinator_agent: None,
            event_tx: None,
            telemetry: Arc::new(tokio::sync::Mutex::new(SwarmTelemetryCollector::default())),
            cache: Some(Arc::new(tokio::sync::Mutex::new(cache))),
            result_store: ResultStore::new_arc(),
            bus: None,
        })
    }

    /// Set a pre-initialized cache
    pub fn with_cache_instance(mut self, cache: Arc<tokio::sync::Mutex<SwarmCache>>) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Set an agent bus for inter-agent communication
    pub fn with_bus(mut self, bus: Arc<AgentBus>) -> Self {
        self.bus = Some(bus);
        self
    }

    /// Get the agent bus if set
    pub fn bus(&self) -> Option<&Arc<AgentBus>> {
        self.bus.as_ref()
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
        tracing::debug!(
            has_coordinator = self.coordinator_agent.is_some(),
            "Getting coordinator agent"
        );
        self.coordinator_agent.as_ref()
    }

    /// Get the shared result store
    pub fn result_store(&self) -> &Arc<ResultStore> {
        &self.result_store
    }

    /// Get cache statistics if caching is enabled
    pub async fn cache_stats(&self) -> Option<CacheStats> {
        if let Some(ref cache) = self.cache {
            let cache_guard = cache.lock().await;
            Some(cache_guard.stats().clone())
        } else {
            None
        }
    }

    /// Clear the cache if enabled
    pub async fn clear_cache(&self) -> Result<()> {
        if let Some(ref cache) = self.cache {
            let mut cache_guard = cache.lock().await;
            cache_guard.clear().await?;
        }
        Ok(())
    }

    /// Get the retry configuration
    pub fn retry_config(&self) -> (u32, u64, u64, f64) {
        (
            self.config.max_retries,
            self.config.retry_initial_delay_ms,
            self.config.retry_max_delay_ms,
            self.config.retry_backoff_multiplier,
        )
    }

    /// Check if retries are enabled
    pub fn retries_enabled(&self) -> bool {
        self.config.max_retries > 0
    }

    /// Send an event to the TUI if channel is connected (non-blocking)
    fn try_send_event(&self, event: SwarmEvent) {
        // Also emit on the agent bus if connected
        if let Some(ref bus) = self.bus {
            let handle = bus.handle("swarm-executor");
            match &event {
                SwarmEvent::Started { task, .. } => {
                    handle.send(
                        "broadcast",
                        BusMessage::AgentReady {
                            agent_id: "swarm-executor".to_string(),
                            capabilities: vec![format!("executing:{task}")],
                        },
                    );
                }
                SwarmEvent::Complete { success, .. } => {
                    let state = if *success {
                        crate::a2a::types::TaskState::Completed
                    } else {
                        crate::a2a::types::TaskState::Failed
                    };
                    handle.send_task_update("swarm", state, None);
                }
                _ => {} // Other events are TUI-specific
            }
        }

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
        let strategy_str = format!("{:?}", strategy);
        self.telemetry
            .lock()
            .await
            .start_swarm(&swarm_id, subtasks.len(), &strategy_str)
            .await;

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
                    // Also publish to shared ResultStore for richer querying
                    let tags = vec![
                        format!("stage:{stage}"),
                        format!("subtask:{}", result.subtask_id),
                    ];
                    let _ = self
                        .result_store
                        .publish(
                            &result.subtask_id,
                            &result.subagent_id,
                            &result.result,
                            tags,
                            None,
                        )
                        .await;
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
            .record_swarm_latency("total_execution", start_time.elapsed())
            .await;

        // Calculate final stats
        let stats = orchestrator.stats_mut();
        stats.execution_time_ms = start_time.elapsed().as_millis() as u64;
        stats.sequential_time_estimate_ms = all_results.iter().map(|r| r.execution_time_ms).sum();
        stats.calculate_critical_path();
        stats.calculate_speedup();

        // Aggregate results
        let success = all_results.iter().all(|r| r.success);

        // Complete telemetry collection
        let _telemetry_metrics = self.telemetry.lock().await.complete_swarm(success).await;
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
        if self.config.execution_mode == ExecutionMode::KubernetesPod {
            return self
                .execute_stage_kubernetes(orchestrator, subtasks, completed_results, swarm_id)
                .await;
        }

        let mut handles: FuturesUnordered<
            tokio::task::JoinHandle<(String, Result<SubTaskResult, anyhow::Error>)>,
        > = FuturesUnordered::new();
        let mut abort_handles: HashMap<String, AbortHandle> = HashMap::new();
        let mut task_ids: HashMap<tokio::task::Id, String> = HashMap::new();
        let mut active_worktrees: HashMap<String, WorktreeInfo> = HashMap::new();
        let mut all_worktrees: HashMap<String, WorktreeInfo> = HashMap::new();
        let mut cached_results: Vec<SubTaskResult> = Vec::new();
        let mut completed_entries: Vec<(SubTaskResult, Option<WorktreeInfo>)> = Vec::new();
        let mut kill_reasons: HashMap<String, String> = HashMap::new();
        let mut promoted_subtask_id: Option<String> = None;

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

        // Create base tool registry with provider for ralph and batch tool
        let base_tool_registry =
            ToolRegistry::with_provider_arc(Arc::clone(&provider), model.clone());
        // Filter out 'question' tool - sub-agents must be autonomous, not interactive
        // Include 'swarm_share' definition so LLMs know about it (registered per-agent below)
        let mut tool_definitions: Vec<_> = base_tool_registry
            .definitions()
            .into_iter()
            .filter(|t| t.name != "question")
            .collect();

        // Add swarm_share tool definition so LLMs know it's available
        let swarm_share_def = crate::provider::ToolDefinition {
            name: "swarm_share".to_string(),
            description: "Share results with other sub-agents in the swarm. Actions: publish \
                          (share a result), get (retrieve a result by key), query_tags (find \
                          results by tags), query_prefix (find results by key prefix), list \
                          (show all shared results)."
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["publish", "get", "query_tags", "query_prefix", "list"],
                        "description": "Action to perform"
                    },
                    "key": {
                        "type": "string",
                        "description": "Result key (for publish/get)"
                    },
                    "value": {
                        "description": "Result value to publish (any JSON value)"
                    },
                    "tags": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Tags for publish or query_tags"
                    },
                    "prefix": {
                        "type": "string",
                        "description": "Key prefix for query_prefix"
                    }
                },
                "required": ["action"]
            }),
        };
        tool_definitions.push(swarm_share_def);

        // Clone the result store for sub-agent sharing
        let result_store = Arc::clone(&self.result_store);

        // Create worktree manager if enabled
        let worktree_manager = if self.config.worktree_enabled {
            let working_dir = self.config.working_dir.clone().unwrap_or_else(|| {
                std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string())
            });

            let mgr = WorktreeManager::new(&working_dir);
            tracing::info!(
                working_dir = %working_dir,
                "Worktree isolation enabled for parallel sub-agents"
            );
            Some(Arc::new(mgr) as Arc<WorktreeManager>)
        } else {
            None
        };

        for (idx, subtask) in subtasks.into_iter().enumerate() {
            let model = model.clone();
            let _provider_name = provider_name.clone();
            let provider = Arc::clone(&provider);

            // Check cache first
            if let Some(ref cache) = self.cache {
                let mut cache_guard = cache.lock().await;
                if let Some(cached_result) = cache_guard.get(&subtask).await {
                    tracing::info!(
                        subtask_id = %subtask.id,
                        task_name = %subtask.name,
                        "Cache hit for subtask, skipping execution"
                    );
                    self.try_send_event(SwarmEvent::SubTaskUpdate {
                        id: subtask.id.clone(),
                        name: subtask.name.clone(),
                        status: SubTaskStatus::Completed,
                        agent_name: Some("cached".to_string()),
                    });
                    cached_results.push(cached_result);
                    continue;
                }
            }

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

            // Retry configuration
            let max_retries = self.config.max_retries;
            let retry_initial_delay_ms = self.config.retry_initial_delay_ms;
            let retry_max_delay_ms = self.config.retry_max_delay_ms;
            let retry_backoff_multiplier = self.config.retry_backoff_multiplier;

            // Create worktree for this sub-agent before spawning so the collapse controller
            // can monitor live branches while execution is in-flight.
            let worktree_info = if let Some(ref mgr) = worktree_manager {
                let task_slug = subtask_id.replace("-", "_");
                match mgr.create(&task_slug).await {
                    Ok(wt) => {
                        if let Err(e) = mgr.inject_workspace_stub(&wt.path) {
                            tracing::warn!(
                                subtask_id = %subtask_id,
                                error = %e,
                                "Failed to inject workspace stub into worktree"
                            );
                        }
                        tracing::info!(
                            subtask_id = %subtask_id,
                            worktree_path = %wt.path.display(),
                            worktree_branch = %wt.branch,
                            "Created worktree for sub-agent"
                        );
                        active_worktrees.insert(subtask_id.clone(), wt.clone());
                        all_worktrees.insert(subtask_id.clone(), wt.clone());
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

            let working_dir = worktree_info
                .as_ref()
                .map(|wt| wt.path.display().to_string())
                .unwrap_or_else(|| ".".to_string());
            let working_dir_path = worktree_info.as_ref().map(|wt| wt.path.clone());

            // Clone for the async block
            let tools = tool_definitions.clone();
            let _base_registry = Arc::clone(&base_tool_registry);
            let agent_result_store = Arc::clone(&result_store);
            let sem = Arc::clone(&semaphore);
            let stagger_delay = delay_ms * idx as u64; // Stagger start times
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
                let _permit = match sem.acquire().await {
                    Ok(permit) => permit,
                    Err(_) => {
                        return (
                            subtask_id.clone(),
                            Err(anyhow::anyhow!("Swarm execution cancelled")),
                        );
                    }
                };

                let _agent_start = Instant::now();

                let start = Instant::now();

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
- swarm_share: Share results with other sub-agents running in parallel
- agent: Spawn specialized helper agents when needed (smart delegation)

SMART SPAWN POLICY (mandatory):
- Any spawned agent MUST use a different model than your current model ('{}')
- Spawned model MUST be free/subscription-eligible (e.g. '*:free', openai-codex/*, github-copilot/*, gemini-web/*, local_cuda/*)
- Include `model` when calling agent.spawn

SHARING RESULTS:
Use swarm_share to collaborate with other sub-agents:
- swarm_share({{action: 'publish', key: 'my-finding', value: '...', tags: ['research']}}) to share a result
- swarm_share({{action: 'get', key: 'some-key'}}) to retrieve a result from another agent
- swarm_share({{action: 'list'}}) to see all shared results
- swarm_share({{action: 'query_tags', tags: ['research']}}) to find results by tag

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
                    model,
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
                // Create per-agent registry with SwarmShareTool for this subtask
                let mut agent_registry =
                    ToolRegistry::with_provider(Arc::clone(&provider), model.clone());
                agent_registry.register(Arc::new(crate::tool::swarm_share::SwarmShareTool::new(
                    Arc::clone(&agent_result_store),
                    subtask_id.clone(),
                )));
                let registry = Arc::new(agent_registry);

                // Execute with exponential backoff retry
                let mut attempt = 0u32;
                let mut result: Result<(String, usize, usize, AgentLoopExit), anyhow::Error> =
                    Err(anyhow::anyhow!("Not executed"));

                while attempt <= max_retries {
                    let _attempt_start = Instant::now();

                    // Run the agent loop
                    result = run_agent_loop(
                        Arc::clone(&provider),
                        &model,
                        &system_prompt,
                        &user_prompt,
                        tools.clone(),
                        Arc::clone(&registry),
                        max_steps,
                        timeout_secs,
                        event_tx.clone(),
                        subtask_id.clone(),
                        None,
                        working_dir_path.clone(),
                    )
                    .await;

                    // Check if the attempt succeeded
                    match &result {
                        Ok((_, _, _, exit_reason)) => {
                            if *exit_reason == AgentLoopExit::Completed {
                                // Success - no need to retry
                                tracing::info!(
                                    subtask_id = %subtask_id,
                                    attempt = attempt + 1,
                                    "Sub-agent completed successfully on first attempt"
                                );
                                break;
                            }
                            // Failed but not an error - check if we should retry
                            let should_retry = attempt < max_retries;
                            if should_retry {
                                let delay = calculate_backoff_delay(
                                    attempt,
                                    retry_initial_delay_ms,
                                    retry_max_delay_ms,
                                    retry_backoff_multiplier,
                                );
                                tracing::warn!(
                                    subtask_id = %subtask_id,
                                    attempt = attempt + 1,
                                    max_retries = max_retries,
                                    exit_reason = ?exit_reason,
                                    delay_ms = delay.as_millis(),
                                    "Sub-agent did not complete, retrying with backoff"
                                );
                                tokio::time::sleep(delay).await;
                            } else {
                                tracing::warn!(
                                    subtask_id = %subtask_id,
                                    attempt = attempt + 1,
                                    max_retries = max_retries,
                                    exit_reason = ?exit_reason,
                                    "Sub-agent did not complete, retries exhausted"
                                );
                            }
                        }
                        Err(e) => {
                            // Error occurred - retry with backoff if we have retries left
                            let should_retry = attempt < max_retries;
                            if should_retry {
                                let delay = calculate_backoff_delay(
                                    attempt,
                                    retry_initial_delay_ms,
                                    retry_max_delay_ms,
                                    retry_backoff_multiplier,
                                );
                                tracing::warn!(
                                    subtask_id = %subtask_id,
                                    attempt = attempt + 1,
                                    max_retries = max_retries,
                                    error = %e,
                                    delay_ms = delay.as_millis(),
                                    "Sub-agent error, retrying with backoff"
                                );
                                tokio::time::sleep(delay).await;
                            } else {
                                tracing::error!(
                                    subtask_id = %subtask_id,
                                    attempt = attempt + 1,
                                    max_retries = max_retries,
                                    error = %e,
                                    "Sub-agent error, retries exhausted"
                                );
                            }
                        }
                    }

                    attempt += 1;
                }

                let result = result;

                let task_result = match result {
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

                        // Calculate actual retry info - attempt is 0-indexed, so attempt=0 means 1 attempt, attempt=1 means 2 attempts, etc.
                        let total_attempts = attempt + 1;
                        let actual_retry_attempts = if total_attempts > 1 {
                            total_attempts - 1
                        } else {
                            0
                        };
                        let was_retried = attempt > 0;

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
                        Ok(SubTaskResult {
                            subtask_id: subtask_id.clone(),
                            subagent_id: format!("agent-{}", subtask_id),
                            success,
                            result: output,
                            steps,
                            tool_calls,
                            execution_time_ms: start.elapsed().as_millis() as u64,
                            error,
                            artifacts: Vec::new(),
                            retry_attempts: actual_retry_attempts,
                            is_retry: was_retried,
                        })
                    }
                    Err(e) => {
                        // Calculate actual retry info - attempt is 0-indexed
                        let total_attempts = attempt + 1;
                        let actual_retry_attempts = if total_attempts > 1 {
                            total_attempts - 1
                        } else {
                            0
                        };
                        let was_retried = attempt > 0;

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
                            retry_attempts: actual_retry_attempts,
                            is_retry: was_retried,
                        })
                    }
                };

                (subtask_id.clone(), task_result)
            });

            let abort_handle = handle.abort_handle();
            abort_handles.insert(subtask_id_for_handle.clone(), abort_handle);
            task_ids.insert(handle.id(), subtask_id_for_handle.clone());
            handles.push(handle);
        }

        // Deterministic collapse loop while branches are still running.
        let mut collapse_controller = if worktree_manager.is_some() && active_worktrees.len() > 1 {
            Some(CollapseController::new(CollapsePolicy::default()))
        } else {
            None
        };
        let mut collapse_tick = tokio::time::interval(Duration::from_secs(COLLAPSE_SAMPLE_SECS));
        collapse_tick.set_missed_tick_behavior(MissedTickBehavior::Skip);
        // Consume the immediate first tick so sampling starts after the first interval.
        let _ = collapse_tick.tick().await;

        while !handles.is_empty() {
            tokio::select! {
                maybe_join = handles.next() => {
                    let Some(joined) = maybe_join else {
                        continue;
                    };
                    match joined {
                        Ok((subtask_id, Ok(result))) => {
                            abort_handles.remove(&subtask_id);
                            let wt = active_worktrees.remove(&subtask_id).or_else(|| all_worktrees.get(&subtask_id).cloned());
                            completed_entries.push((result, wt));
                        }
                        Ok((subtask_id, Err(e))) => {
                            abort_handles.remove(&subtask_id);
                            active_worktrees.remove(&subtask_id);
                            let wt = all_worktrees.get(&subtask_id).cloned();
                            completed_entries.push((
                                SubTaskResult {
                                    subtask_id: subtask_id.clone(),
                                    subagent_id: format!("agent-{subtask_id}"),
                                    success: false,
                                    result: String::new(),
                                    steps: 0,
                                    tool_calls: 0,
                                    execution_time_ms: 0,
                                    error: Some(e.to_string()),
                                    artifacts: Vec::new(),
                                    retry_attempts: 0,
                                    is_retry: false,
                                },
                                wt,
                            ));
                        }
                        Err(e) => {
                            let subtask_id = task_ids
                                .remove(&e.id())
                                .unwrap_or_else(|| "unknown".to_string());
                            abort_handles.remove(&subtask_id);
                            active_worktrees.remove(&subtask_id);
                            let wt = all_worktrees.get(&subtask_id).cloned();
                            completed_entries.push((
                                SubTaskResult {
                                    subtask_id: subtask_id.clone(),
                                    subagent_id: format!("agent-{subtask_id}"),
                                    success: false,
                                    result: String::new(),
                                    steps: 0,
                                    tool_calls: 0,
                                    execution_time_ms: 0,
                                    error: Some(format!("Task join error: {e}")),
                                    artifacts: Vec::new(),
                                    retry_attempts: 0,
                                    is_retry: false,
                                },
                                wt,
                            ));
                        }
                    }
                }
                _ = collapse_tick.tick(), if collapse_controller.is_some() && !active_worktrees.is_empty() => {
                    let branches: Vec<BranchRuntimeState> = active_worktrees
                        .iter()
                        .map(|(subtask_id, wt)| BranchRuntimeState {
                            subtask_id: subtask_id.clone(),
                            branch: wt.branch.clone(),
                            worktree_path: wt.path.clone(),
                        })
                        .collect();

                    if let Some(controller) = collapse_controller.as_mut() {
                        match controller.sample(&branches) {
                            Ok(tick) => {
                                if promoted_subtask_id != tick.promoted_subtask_id {
                                    promoted_subtask_id = tick.promoted_subtask_id.clone();
                                    if let Some(ref promoted) = promoted_subtask_id {
                                        tracing::info!(
                                            subtask_id = %promoted,
                                            "Collapse controller promoted branch"
                                        );
                                        if let Some(audit) = crate::audit::try_audit_log() {
                                            audit.log_with_correlation(
                                                crate::audit::AuditCategory::Swarm,
                                                "collapse_promote_branch",
                                                crate::audit::AuditOutcome::Success,
                                                Some("collapse-controller".to_string()),
                                                Some(serde_json::json!({
                                                    "swarm_id": swarm_id,
                                                    "subtask_id": promoted,
                                                })),
                                                None,
                                                None,
                                                Some(swarm_id.to_string()),
                                                None,
                                            ).await;
                                        }
                                    }
                                }

                                for kill in tick.kills {
                                    if kill_reasons.contains_key(&kill.subtask_id) {
                                        continue;
                                    }
                                    let Some(abort_handle) = abort_handles.get(&kill.subtask_id) else {
                                        continue;
                                    };

                                    abort_handle.abort();
                                    abort_handles.remove(&kill.subtask_id);
                                    active_worktrees.remove(&kill.subtask_id);
                                    kill_reasons.insert(kill.subtask_id.clone(), kill.reason.clone());

                                    tracing::warn!(
                                        subtask_id = %kill.subtask_id,
                                        branch = %kill.branch,
                                        reason = %kill.reason,
                                        "Collapse controller killed branch"
                                    );

                                    if let Some(ref tx) = self.event_tx {
                                        let _ = tx.try_send(SwarmEvent::SubTaskUpdate {
                                            id: kill.subtask_id.clone(),
                                            name: kill.subtask_id.clone(),
                                            status: SubTaskStatus::Cancelled,
                                            agent_name: Some(format!("agent-{}", kill.subtask_id)),
                                        });
                                        let _ = tx.try_send(SwarmEvent::AgentError {
                                            subtask_id: kill.subtask_id.clone(),
                                            error: format!("Cancelled by collapse controller: {}", kill.reason),
                                        });
                                    }

                                    if let Some(audit) = crate::audit::try_audit_log() {
                                        audit.log_with_correlation(
                                            crate::audit::AuditCategory::Swarm,
                                            "collapse_kill_branch",
                                            crate::audit::AuditOutcome::Success,
                                            Some("collapse-controller".to_string()),
                                            Some(serde_json::json!({
                                                "swarm_id": swarm_id,
                                                "subtask_id": kill.subtask_id,
                                                "branch": kill.branch,
                                                "reason": kill.reason,
                                            })),
                                            None,
                                            None,
                                            Some(swarm_id.to_string()),
                                            None,
                                        ).await;
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::warn!(error = %e, "Collapse controller sampling failed");
                            }
                        }
                    }
                }
            }
        }

        // Merge in deterministic order: promoted branch first when present.
        if let Some(ref promoted) = promoted_subtask_id {
            completed_entries.sort_by_key(|(result, _)| {
                if &result.subtask_id == promoted {
                    0usize
                } else {
                    1usize
                }
            });
        }

        let mut results = cached_results;
        let auto_merge = self.config.worktree_auto_merge;

        for (mut result, worktree_info) in completed_entries {
            // Handle worktree merge/cleanup if applicable
            if let Some(wt) = worktree_info {
                if let Some(reason) = kill_reasons.get(&result.subtask_id) {
                    result.error = Some(format!("Cancelled by collapse controller: {reason}"));
                    result.result.push_str(&format!(
                        "\n\n--- Collapse Controller ---\nBranch terminated: {reason}"
                    ));
                    if let Some(ref mgr) = worktree_manager
                        && let Err(e) = mgr.cleanup(&wt.name).await
                    {
                        tracing::warn!(error = %e, "Failed to cleanup killed worktree");
                    }
                } else if result.success && auto_merge {
                    if let Some(ref mgr) = worktree_manager {
                        match mgr.merge(&wt.name).await {
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
                                if let Err(e) = mgr.cleanup(&wt.name).await {
                                    tracing::warn!(error = %e, "Failed to cleanup worktree");
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
                    tracing::info!(
                        subtask_id = %result.subtask_id,
                        worktree_path = %wt.path.display(),
                        "Keeping worktree for debugging (task failed)"
                    );
                }
            }

            // Cache successful result
            if result.success
                && let Some(ref cache_arc) = self.cache
            {
                let mut cache_guard: tokio::sync::MutexGuard<'_, SwarmCache> =
                    cache_arc.lock().await;
                let cache_subtask = SubTask::new(&result.subtask_id, &result.result);
                if let Err(e) = cache_guard.put(&cache_subtask, &result).await {
                    tracing::warn!(
                        subtask_id = %result.subtask_id,
                        error = %e,
                        "Failed to cache subtask result"
                    );
                }
            }

            results.push(result);
        }

        Ok(results)
    }

    async fn execute_stage_kubernetes(
        &self,
        orchestrator: &Orchestrator,
        subtasks: Vec<SubTask>,
        completed_results: Arc<RwLock<HashMap<String, String>>>,
        swarm_id: &str,
    ) -> Result<Vec<SubTaskResult>> {
        let k8s = K8sManager::new().await;
        if !k8s.is_available() {
            anyhow::bail!(
                "Kubernetes execution mode requested but K8s client is unavailable in this environment"
            );
        }

        let provider_name = orchestrator.provider().to_string();
        let model = orchestrator.model().to_string();
        let pod_budget = self.config.k8s_pod_budget.max(1);
        let mut pending: VecDeque<SubTask> = subtasks.into_iter().collect();
        let mut active: HashMap<String, ActiveK8sBranch> = HashMap::new();
        let mut subtask_names: HashMap<String, String> = HashMap::new();
        let mut results: Vec<SubTaskResult> = Vec::new();
        let mut kill_reasons: HashMap<String, String> = HashMap::new();
        let mut promoted_subtask_id: Option<String> = None;
        let mut collapse_controller = CollapseController::new(CollapsePolicy::default());

        let mut tick = tokio::time::interval(Duration::from_secs(COLLAPSE_SAMPLE_SECS));
        tick.set_missed_tick_behavior(MissedTickBehavior::Skip);
        let _ = tick.tick().await;

        loop {
            while active.len() < pod_budget {
                let Some(subtask) = pending.pop_front() else {
                    break;
                };

                if let Some(ref cache) = self.cache {
                    let mut cache_guard = cache.lock().await;
                    if let Some(cached_result) = cache_guard.get(&subtask).await {
                        tracing::info!(
                            subtask_id = %subtask.id,
                            task_name = %subtask.name,
                            "Cache hit for subtask, skipping Kubernetes execution"
                        );
                        self.try_send_event(SwarmEvent::SubTaskUpdate {
                            id: subtask.id.clone(),
                            name: subtask.name.clone(),
                            status: SubTaskStatus::Completed,
                            agent_name: Some("cached".to_string()),
                        });
                        results.push(cached_result);
                        continue;
                    }
                }

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

                let payload = RemoteSubtaskPayload {
                    swarm_id: swarm_id.to_string(),
                    subtask_id: subtask.id.clone(),
                    subtask_name: subtask.name.clone(),
                    specialty: subtask.specialty.clone().unwrap_or_default(),
                    instruction: subtask.instruction.clone(),
                    context: context.clone(),
                    provider: provider_name.clone(),
                    model: model.clone(),
                    max_steps: self.config.max_steps_per_subagent,
                    timeout_secs: self.config.subagent_timeout_secs,
                    working_dir: self.config.working_dir.clone(),
                    probe_interval_secs: COLLAPSE_SAMPLE_SECS,
                };
                let payload_b64 = match encode_payload(&payload) {
                    Ok(payload) => payload,
                    Err(error) => {
                        let error_text = format!("Failed to encode remote payload: {error}");
                        tracing::error!(subtask_id = %subtask.id, error = %error, "K8s payload encoding failed");
                        self.try_send_event(SwarmEvent::SubTaskUpdate {
                            id: subtask.id.clone(),
                            name: subtask.name.clone(),
                            status: SubTaskStatus::Failed,
                            agent_name: Some("k8s-encoder".to_string()),
                        });
                        self.try_send_event(SwarmEvent::AgentError {
                            subtask_id: subtask.id.clone(),
                            error: error_text.clone(),
                        });
                        results.push(SubTaskResult {
                            subtask_id: subtask.id.clone(),
                            subagent_id: format!("agent-{}", subtask.id),
                            success: false,
                            result: String::new(),
                            steps: 0,
                            tool_calls: 0,
                            execution_time_ms: 0,
                            error: Some(error_text),
                            artifacts: Vec::new(),
                            is_retry: false,
                            retry_attempts: 0,
                        });
                        continue;
                    }
                };

                let mut env_vars = HashMap::new();
                env_vars.insert(SWARM_SUBTASK_PAYLOAD_ENV.to_string(), payload_b64);
                for key in K8S_PASSTHROUGH_ENV_VARS {
                    if let Ok(value) = std::env::var(key)
                        && !value.trim().is_empty()
                    {
                        env_vars.insert((*key).to_string(), value);
                    }
                }
                let fallback_prompt = if context.trim().is_empty() {
                    format!(
                        "You are executing swarm subtask '{}'.\n\nTask:\n{}\n\n\
Return only the final subtask answer.",
                        subtask.id, subtask.instruction
                    )
                } else {
                    format!(
                        "You are executing swarm subtask '{}'.\n\nTask:\n{}\n\n\
Dependency context:\n{}\n\nReturn only the final subtask answer.",
                        subtask.id, subtask.instruction, context
                    )
                };
                env_vars.insert(SWARM_FALLBACK_PROMPT_ENV.to_string(), fallback_prompt);
                env_vars.insert(SWARM_FALLBACK_MODEL_ENV.to_string(), model.clone());

                let mut labels = HashMap::new();
                labels.insert("codetether.run/swarm-id".to_string(), swarm_id.to_string());
                labels.insert(
                    "codetether.run/stage".to_string(),
                    subtask.stage.to_string(),
                );

                let spec = SubagentPodSpec {
                    image: self.config.k8s_subagent_image.clone(),
                    env_vars,
                    labels,
                    command: Some(vec!["sh".to_string(), "-lc".to_string()]),
                    args: Some(vec![
                        format!(
                            "if codetether help swarm-subagent >/dev/null 2>&1; then \
exec codetether swarm-subagent --payload-env {payload_env}; \
else \
exec codetether run \"$${fallback_prompt_env}\" --model \"$${fallback_model_env}\"; \
fi",
                            payload_env = SWARM_SUBTASK_PAYLOAD_ENV,
                            fallback_prompt_env = SWARM_FALLBACK_PROMPT_ENV,
                            fallback_model_env = SWARM_FALLBACK_MODEL_ENV,
                        )
                        .replace("$$", "$"),
                    ]),
                };

                if let Err(error) = k8s.spawn_subagent_pod_with_spec(&subtask.id, spec).await {
                    let error_text = format!("Failed to spawn Kubernetes pod: {error}");
                    tracing::error!(subtask_id = %subtask.id, error = %error, "K8s sub-agent pod spawn failed");
                    self.try_send_event(SwarmEvent::SubTaskUpdate {
                        id: subtask.id.clone(),
                        name: subtask.name.clone(),
                        status: SubTaskStatus::Failed,
                        agent_name: Some("k8s-spawn".to_string()),
                    });
                    self.try_send_event(SwarmEvent::AgentError {
                        subtask_id: subtask.id.clone(),
                        error: error_text.clone(),
                    });
                    results.push(SubTaskResult {
                        subtask_id: subtask.id.clone(),
                        subagent_id: format!("agent-{}", subtask.id),
                        success: false,
                        result: String::new(),
                        steps: 0,
                        tool_calls: 0,
                        execution_time_ms: 0,
                        error: Some(error_text),
                        artifacts: Vec::new(),
                        is_retry: false,
                        retry_attempts: 0,
                    });
                    continue;
                }

                let branch = K8sManager::subagent_pod_name(&subtask.id);
                subtask_names.insert(subtask.id.clone(), subtask.name.clone());
                active.insert(
                    subtask.id.clone(),
                    ActiveK8sBranch {
                        branch: branch.clone(),
                        started_at: Instant::now(),
                    },
                );

                self.try_send_event(SwarmEvent::SubTaskUpdate {
                    id: subtask.id.clone(),
                    name: subtask.name.clone(),
                    status: SubTaskStatus::Running,
                    agent_name: Some(format!("k8s-{branch}")),
                });
                self.try_send_event(SwarmEvent::AgentStarted {
                    subtask_id: subtask.id.clone(),
                    agent_name: format!("k8s-{branch}"),
                    specialty: subtask
                        .specialty
                        .clone()
                        .unwrap_or_else(|| "generalist".to_string()),
                });

                tracing::info!(
                    subtask_id = %subtask.id,
                    pod = %branch,
                    "Spawned Kubernetes sub-agent pod"
                );
            }

            if pending.is_empty() && active.is_empty() {
                break;
            }

            tick.tick().await;

            let active_ids: Vec<String> = active.keys().cloned().collect();
            let mut finished_results: Vec<SubTaskResult> = Vec::new();
            for subtask_id in active_ids {
                let Some(active_state) = active.get(&subtask_id).cloned() else {
                    continue;
                };

                if active_state.started_at.elapsed()
                    > Duration::from_secs(self.config.subagent_timeout_secs)
                {
                    let reason = format!(
                        "Timed out after {}s in Kubernetes pod",
                        self.config.subagent_timeout_secs
                    );
                    kill_reasons.insert(subtask_id.clone(), reason.clone());
                    if let Err(error) = k8s.delete_subagent_pod(&subtask_id).await {
                        tracing::warn!(
                            subtask_id = %subtask_id,
                            error = %error,
                            "Failed deleting timed-out Kubernetes pod"
                        );
                    }
                    active.remove(&subtask_id);
                    finished_results.push(SubTaskResult {
                        subtask_id: subtask_id.clone(),
                        subagent_id: format!("agent-{subtask_id}"),
                        success: false,
                        result: String::new(),
                        steps: 0,
                        tool_calls: 0,
                        execution_time_ms: active_state.started_at.elapsed().as_millis() as u64,
                        error: Some(reason),
                        artifacts: Vec::new(),
                        is_retry: false,
                        retry_attempts: 0,
                    });
                    continue;
                }

                let pod_state = match k8s.get_subagent_pod_state(&subtask_id).await {
                    Ok(state) => state,
                    Err(error) => {
                        tracing::warn!(
                            subtask_id = %subtask_id,
                            error = %error,
                            "Failed to query Kubernetes pod state for sub-agent"
                        );
                        continue;
                    }
                };
                let Some(pod_state) = pod_state else {
                    active.remove(&subtask_id);
                    finished_results.push(SubTaskResult {
                        subtask_id: subtask_id.clone(),
                        subagent_id: format!("agent-{subtask_id}"),
                        success: false,
                        result: String::new(),
                        steps: 0,
                        tool_calls: 0,
                        execution_time_ms: active_state.started_at.elapsed().as_millis() as u64,
                        error: Some("Sub-agent pod disappeared".to_string()),
                        artifacts: Vec::new(),
                        is_retry: false,
                        retry_attempts: 0,
                    });
                    continue;
                };

                let phase = pod_state.phase.to_ascii_lowercase();
                let finished = pod_state.terminated || phase == "succeeded" || phase == "failed";
                if !finished {
                    continue;
                }

                let logs = k8s
                    .subagent_logs(&subtask_id, 10_000)
                    .await
                    .unwrap_or_default();
                let mut result = result_from_logs(&logs).unwrap_or_else(|| SubTaskResult {
                    subtask_id: subtask_id.clone(),
                    subagent_id: format!("agent-{subtask_id}"),
                    success: pod_state.exit_code.unwrap_or(1) == 0,
                    result: logs,
                    steps: 0,
                    tool_calls: 0,
                    execution_time_ms: active_state.started_at.elapsed().as_millis() as u64,
                    error: if pod_state.exit_code.unwrap_or(1) == 0 {
                        None
                    } else {
                        Some(
                            pod_state
                                .reason
                                .clone()
                                .unwrap_or_else(|| "Remote sub-agent failed".to_string()),
                        )
                    },
                    artifacts: Vec::new(),
                    is_retry: false,
                    retry_attempts: 0,
                });

                if let Some(reason) = kill_reasons.get(&subtask_id) {
                    result.success = false;
                    result.error = Some(format!("Cancelled by collapse controller: {reason}"));
                    result.result.push_str(&format!(
                        "\n\n--- Collapse Controller ---\nBranch terminated: {reason}"
                    ));
                }

                active.remove(&subtask_id);
                if let Err(error) = k8s.delete_subagent_pod(&subtask_id).await {
                    tracing::warn!(
                        subtask_id = %subtask_id,
                        error = %error,
                        "Failed deleting completed Kubernetes pod"
                    );
                }
                finished_results.push(result);
            }

            for result in finished_results {
                if result.success {
                    completed_results
                        .write()
                        .await
                        .insert(result.subtask_id.clone(), result.result.clone());
                }
                if result.success
                    && let Some(ref cache_arc) = self.cache
                {
                    let mut cache_guard = cache_arc.lock().await;
                    let cache_subtask = SubTask::new(&result.subtask_id, &result.result);
                    let _ = cache_guard.put(&cache_subtask, &result).await;
                }

                self.try_send_event(SwarmEvent::SubTaskUpdate {
                    id: result.subtask_id.clone(),
                    name: subtask_names
                        .get(&result.subtask_id)
                        .cloned()
                        .unwrap_or_else(|| result.subtask_id.clone()),
                    status: if result.success {
                        SubTaskStatus::Completed
                    } else {
                        SubTaskStatus::Failed
                    },
                    agent_name: Some(format!("k8s-{}", result.subtask_id)),
                });
                if let Some(ref error) = result.error {
                    self.try_send_event(SwarmEvent::AgentError {
                        subtask_id: result.subtask_id.clone(),
                        error: error.clone(),
                    });
                }
                self.try_send_event(SwarmEvent::AgentOutput {
                    subtask_id: result.subtask_id.clone(),
                    output: result.result.clone(),
                });
                self.try_send_event(SwarmEvent::AgentComplete {
                    subtask_id: result.subtask_id.clone(),
                    success: result.success,
                    steps: result.steps,
                });
                results.push(result);
            }

            if active.len() > 1 {
                let mut observations = Vec::with_capacity(active.len());
                for (subtask_id, state) in &active {
                    let pod_state = match k8s.get_subagent_pod_state(subtask_id).await {
                        Ok(state) => state,
                        Err(error) => {
                            tracing::warn!(
                                subtask_id = %subtask_id,
                                error = %error,
                                "Failed to query pod state while sampling branch observation"
                            );
                            None
                        }
                    };
                    let (resource_health_score, infra_unhealthy_signals) =
                        compute_resource_health(pod_state.as_ref());

                    let logs = k8s.subagent_logs(subtask_id, 500).await.unwrap_or_default();
                    if let Some(probe) = latest_probe_from_logs(&logs) {
                        let compile_ok = pod_state
                            .as_ref()
                            .map(|p| probe.compile_ok && !p.phase.eq_ignore_ascii_case("failed"))
                            .unwrap_or(probe.compile_ok);
                        observations.push(BranchObservation {
                            subtask_id: subtask_id.clone(),
                            branch: state.branch.clone(),
                            compile_ok,
                            changed_files: probe_changed_files_set(&probe),
                            changed_lines: probe.changed_lines,
                            resource_health_score,
                            infra_unhealthy_signals,
                        });
                        continue;
                    }
                    let compile_ok = pod_state
                        .as_ref()
                        .map(|p| !p.phase.eq_ignore_ascii_case("failed"))
                        .unwrap_or(false);
                    observations.push(BranchObservation {
                        subtask_id: subtask_id.clone(),
                        branch: state.branch.clone(),
                        compile_ok,
                        changed_files: std::collections::HashSet::new(),
                        changed_lines: 0,
                        resource_health_score,
                        infra_unhealthy_signals,
                    });
                }

                let tick = collapse_controller.sample_observations(&observations);
                if promoted_subtask_id != tick.promoted_subtask_id {
                    promoted_subtask_id = tick.promoted_subtask_id.clone();
                    if let Some(ref promoted) = promoted_subtask_id {
                        tracing::info!(subtask_id = %promoted, "Collapse controller promoted branch");
                        if let Some(audit) = crate::audit::try_audit_log() {
                            audit
                                .log_with_correlation(
                                    crate::audit::AuditCategory::Swarm,
                                    "collapse_promote_branch",
                                    crate::audit::AuditOutcome::Success,
                                    Some("collapse-controller".to_string()),
                                    Some(serde_json::json!({
                                        "swarm_id": swarm_id,
                                        "subtask_id": promoted,
                                        "execution_mode": "kubernetes_pod",
                                    })),
                                    None,
                                    None,
                                    Some(swarm_id.to_string()),
                                    None,
                                )
                                .await;
                        }
                    }
                }

                for kill in tick.kills {
                    if kill_reasons.contains_key(&kill.subtask_id) {
                        continue;
                    }
                    if !active.contains_key(&kill.subtask_id) {
                        continue;
                    }

                    if let Err(error) = k8s.delete_subagent_pod(&kill.subtask_id).await {
                        tracing::warn!(
                            subtask_id = %kill.subtask_id,
                            error = %error,
                            "Failed deleting Kubernetes pod after collapse kill"
                        );
                    }
                    kill_reasons.insert(kill.subtask_id.clone(), kill.reason.clone());
                    let elapsed_ms = active
                        .remove(&kill.subtask_id)
                        .map(|s| s.started_at.elapsed().as_millis() as u64)
                        .unwrap_or(0);

                    tracing::warn!(
                        subtask_id = %kill.subtask_id,
                        branch = %kill.branch,
                        reason = %kill.reason,
                        "Collapse controller killed Kubernetes branch"
                    );

                    if let Some(audit) = crate::audit::try_audit_log() {
                        audit
                            .log_with_correlation(
                                crate::audit::AuditCategory::Swarm,
                                "collapse_kill_branch",
                                crate::audit::AuditOutcome::Success,
                                Some("collapse-controller".to_string()),
                                Some(serde_json::json!({
                                    "swarm_id": swarm_id,
                                    "subtask_id": kill.subtask_id.clone(),
                                    "branch": kill.branch.clone(),
                                    "reason": kill.reason.clone(),
                                    "execution_mode": "kubernetes_pod",
                                })),
                                None,
                                None,
                                Some(swarm_id.to_string()),
                                None,
                            )
                            .await;
                    }

                    self.try_send_event(SwarmEvent::SubTaskUpdate {
                        id: kill.subtask_id.clone(),
                        name: subtask_names
                            .get(&kill.subtask_id)
                            .cloned()
                            .unwrap_or_else(|| kill.subtask_id.clone()),
                        status: SubTaskStatus::Cancelled,
                        agent_name: Some(format!("agent-{}", kill.subtask_id)),
                    });
                    self.try_send_event(SwarmEvent::AgentError {
                        subtask_id: kill.subtask_id.clone(),
                        error: format!("Cancelled by collapse controller: {}", kill.reason),
                    });

                    results.push(SubTaskResult {
                        subtask_id: kill.subtask_id.clone(),
                        subagent_id: format!("agent-{}", kill.subtask_id),
                        success: false,
                        result: format!(
                            "\n\n--- Collapse Controller ---\nBranch terminated: {}",
                            kill.reason
                        ),
                        steps: 0,
                        tool_calls: 0,
                        execution_time_ms: elapsed_ms,
                        error: Some(format!("Cancelled by collapse controller: {}", kill.reason)),
                        artifacts: Vec::new(),
                        is_retry: false,
                        retry_attempts: 0,
                    });
                }
            }
        }

        if let Some(ref promoted) = promoted_subtask_id {
            results.sort_by_key(|result| {
                if &result.subtask_id == promoted {
                    0usize
                } else {
                    1usize
                }
            });
        }

        if !active.is_empty() {
            let residual_ids: Vec<String> = active.keys().cloned().collect();
            for subtask_id in residual_ids {
                if let Err(error) = k8s.delete_subagent_pod(&subtask_id).await {
                    tracing::warn!(
                        subtask_id = %subtask_id,
                        error = %error,
                        "Failed deleting residual Kubernetes pod at stage end"
                    );
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
/// Resolve relative file paths in tool call arguments to use the given working directory.
///
/// When sub-agents run in worktrees, the file tools (read, write, edit, etc.) resolve
/// relative paths from the process CWD (the main repo), not the worktree. This function
/// rewrites relative paths to absolute paths within the worktree before tool execution.
fn resolve_tool_paths(
    tool_name: &str,
    args: &mut serde_json::Value,
    working_dir: &std::path::Path,
) {
    match tool_name {
        "read" | "write" | "list" | "grep" | "codesearch" => {
            if let Some(path) = args.get("path").and_then(|v| v.as_str()).map(String::from)
                && !std::path::Path::new(&path).is_absolute()
            {
                args["path"] = serde_json::json!(working_dir.join(&path).display().to_string());
            }
        }
        "edit" => {
            if let Some(path) = args
                .get("filePath")
                .and_then(|v| v.as_str())
                .map(String::from)
                && !std::path::Path::new(&path).is_absolute()
            {
                args["filePath"] = serde_json::json!(working_dir.join(&path).display().to_string());
            }
        }
        "glob" => {
            if let Some(pattern) = args
                .get("pattern")
                .and_then(|v| v.as_str())
                .map(String::from)
                && !std::path::Path::new(&pattern).is_absolute()
                && !pattern.starts_with("*")
            {
                args["pattern"] =
                    serde_json::json!(working_dir.join(&pattern).display().to_string());
            }
        }
        "multiedit" => {
            if let Some(edits) = args.get_mut("edits").and_then(|v| v.as_array_mut()) {
                for edit in edits.iter_mut() {
                    if let Some(file) = edit.get("file").and_then(|v| v.as_str()).map(String::from)
                        && !std::path::Path::new(&file).is_absolute()
                    {
                        edit["file"] =
                            serde_json::json!(working_dir.join(&file).display().to_string());
                    }
                }
            }
        }
        "patch" => {
            if let Some(path) = args.get("file").and_then(|v| v.as_str()).map(String::from)
                && !std::path::Path::new(&path).is_absolute()
            {
                args["file"] = serde_json::json!(working_dir.join(&path).display().to_string());
            }
        }
        "bash" => {
            // If bash has no cwd, set it to the working directory
            if args.get("cwd").and_then(|v| v.as_str()).is_none() {
                args["cwd"] = serde_json::json!(working_dir.display().to_string());
            }
        }
        _ => {}
    }
}

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
    bus: Option<Arc<AgentBus>>,
    working_dir: Option<std::path::PathBuf>,
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
        let mut thinking_parts = Vec::new();
        let mut tool_calls = Vec::new();

        for part in &response.message.content {
            match part {
                ContentPart::Text { text } => {
                    text_parts.push(text.clone());
                }
                ContentPart::Thinking { text } if !text.is_empty() => {
                    thinking_parts.push(text.clone());
                }
                ContentPart::ToolCall {
                    id,
                    name,
                    arguments,
                    ..
                } => {
                    tool_calls.push((id.clone(), name.clone(), arguments.clone()));
                }
                _ => {}
            }
        }

        // Publish thinking/reasoning to bus for training data capture
        if !thinking_parts.is_empty()
            && let Some(ref bus) = bus
        {
            let thinking_text = thinking_parts.join("\n");
            let handle = bus.handle(&subtask_id);
            handle.send(
                format!("agent.{subtask_id}.thinking"),
                BusMessage::AgentThinking {
                    agent_id: subtask_id.clone(),
                    thinking: thinking_text,
                    step: steps,
                },
            );
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
                let mut args: serde_json::Value =
                    serde_json::from_str(&arguments).unwrap_or_else(|e| {
                        tracing::warn!(tool = %tool_name, error = %e, raw = %arguments, "Failed to parse tool arguments");
                        serde_json::json!({})
                    });

                // Resolve relative file paths to the working directory (critical for worktree isolation)
                if let Some(ref wd) = working_dir {
                    resolve_tool_paths(&tool_name, &mut args, wd);
                }
                if let Some(args_obj) = args.as_object_mut() {
                    args_obj
                        .entry("__ct_current_model".to_string())
                        .or_insert_with(|| serde_json::json!(model));
                    args_obj
                        .entry("__ct_session_id".to_string())
                        .or_insert_with(|| serde_json::json!(subtask_id.clone()));
                    args_obj
                        .entry("__ct_agent_name".to_string())
                        .or_insert_with(|| serde_json::json!(format!("agent-{subtask_id}")));
                }

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

            // Publish full, untruncated tool output to the bus so other
            // agents and the bus log see the complete result.
            if let Some(ref bus) = bus {
                let handle = bus.handle(&subtask_id);
                handle.send(
                    format!("tools.{tool_name}"),
                    BusMessage::ToolOutputFull {
                        agent_id: subtask_id.clone(),
                        tool_name: tool_name.clone(),
                        output: result.clone(),
                        success: tool_success,
                        step: steps,
                    },
                );
            }

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
