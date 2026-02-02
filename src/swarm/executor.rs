//! Parallel execution engine for swarm operations
//!
//! Executes subtasks in parallel across multiple sub-agents,
//! respecting dependencies and optimizing for critical path.

use super::{
    orchestrator::Orchestrator,
    subtask::{SubTask, SubTaskResult},
    DecompositionStrategy, StageStats, SwarmConfig, SwarmResult,
};
use crate::{provider::{CompletionRequest, ContentPart, Message, Role}, swarm::{SwarmArtifact, SwarmStats}};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tokio::time::{timeout, Duration};

/// The swarm executor runs subtasks in parallel
pub struct SwarmExecutor {
    config: SwarmConfig,
}

impl SwarmExecutor {
    /// Create a new executor
    pub fn new(config: SwarmConfig) -> Self {
        Self { config }
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
        
        tracing::info!("Starting swarm execution for task");
        
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
        
        tracing::info!("Task decomposed into {} subtasks", subtasks.len());
        
        // Execute stages in order
        let max_stage = subtasks.iter().map(|s| s.stage).max().unwrap_or(0);
        let mut all_results: Vec<SubTaskResult> = Vec::new();
        let mut artifacts: Vec<SwarmArtifact> = Vec::new();
        
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
            
            if stage_subtasks.is_empty() {
                continue;
            }
            
            tracing::info!(
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
    
    /// Execute a single stage of subtasks in parallel
    async fn execute_stage(
        &self,
        orchestrator: &Orchestrator,
        subtasks: Vec<SubTask>,
        completed_results: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<Vec<SubTaskResult>> {
        let mut handles: Vec<tokio::task::JoinHandle<Result<SubTaskResult, anyhow::Error>>> = Vec::new();
        
        for subtask in subtasks {
            let model = orchestrator.model().to_string();
            let provider_name = orchestrator.provider().to_string();
            let providers = orchestrator.providers();
            
            let provider = providers.get(&provider_name)
                .ok_or_else(|| anyhow::anyhow!("Provider {} not found", provider_name))?;
            
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
            
            // Build the prompt for this sub-agent
            let prompt = if context.is_empty() {
                format!(
                    "You are a {} specialist. Complete this task:\n\n{}",
                    specialty, instruction
                )
            } else {
                format!(
                    "You are a {} specialist. Complete this task:\n\n{}\n\nContext from prior work:\n{}",
                    specialty, instruction, context
                )
            };
            
            let temperature = if model.starts_with("kimi-k2") { 1.0 } else { 0.7 };
            
            let request = CompletionRequest {
                messages: vec![Message {
                    role: Role::User,
                    content: vec![ContentPart::Text { text: prompt }],
                }],
                tools: Vec::new(),
                model: model.clone(),
                temperature: Some(temperature),
                top_p: None,
                max_tokens: Some(8192),
                stop: Vec::new(),
            };
            
            // Spawn the subtask execution
            let handle = tokio::spawn({
                let start = Instant::now();
                
                async move {
                    let result = timeout(
                        Duration::from_secs(timeout_secs),
                        async {
                            // Execute the completion
                            match provider.complete(request).await {
                                Ok(response) => {
                                    let text = response.message.content
                                        .iter()
                                        .filter_map(|p| match p {
                                            ContentPart::Text { text } => Some(text.clone()),
                                            _ => None,
                                        })
                                        .collect::<Vec<_>>()
                                        .join("\n");
                                    
                                    Ok(SubTaskResult {
                                        subtask_id: subtask_id.clone(),
                                        subagent_id: format!("agent-{}", subtask_id),
                                        success: true,
                                        result: text,
                                        steps: 1,
                                        tool_calls: 0,
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
                        }
                    ).await;
                    
                    match result {
                        Ok(r) => r,
                        Err(_) => Ok(SubTaskResult {
                            subtask_id,
                            subagent_id: String::new(),
                            success: false,
                            result: String::new(),
                            steps: 0,
                            tool_calls: 0,
                            execution_time_ms: timeout_secs * 1000,
                            error: Some("Timeout".to_string()),
                            artifacts: Vec::new(),
                        }),
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
                    tracing::error!("Subtask error: {}", e);
                }
                Err(e) => {
                    tracing::error!("Task join error: {}", e);
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
