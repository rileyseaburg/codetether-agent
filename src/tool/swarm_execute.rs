//! Swarm Execute Tool - Parallel task execution across multiple agents
//!
//! This tool enables LLM-driven parallel execution of tasks across multiple
//! sub-agents in a swarm pattern, with configurable concurrency and aggregation.

use super::{Tool, ToolResult};
use crate::provider::{parse_model_string, ProviderRegistry};
use crate::swarm::executor::run_agent_loop;
use crate::tool::ToolRegistry;
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;

pub struct SwarmExecuteTool;

impl SwarmExecuteTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SwarmExecuteTool {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize)]
struct Params {
    /// Array of tasks to execute in parallel
    tasks: Vec<TaskInput>,
    /// Maximum number of concurrent agents (default: 5)
    #[serde(default = "default_concurrency")]
    concurrency_limit: usize,
    /// How to aggregate results: "all" (all must succeed), "first_error" (stop on first error), "best_effort" (collect all, report failures)
    #[serde(default = "default_aggregation")]
    aggregation_strategy: String,
    /// Model to use for sub-agents (provider/model format, e.g., "anthropic/claude-sonnet-4-20250514")
    #[serde(default)]
    model: Option<String>,
    /// Maximum steps per sub-agent
    #[serde(default = "default_max_steps")]
    max_steps: usize,
    /// Timeout per sub-agent in seconds
    #[serde(default = "default_timeout")]
    timeout_secs: u64,
}

fn default_concurrency() -> usize {
    5
}

fn default_aggregation() -> String {
    "best_effort".to_string()
}

fn default_max_steps() -> usize {
    50
}

fn default_timeout() -> u64 {
    300
}

#[derive(Deserialize, Clone)]
struct TaskInput {
    /// Unique identifier for this task
    id: Option<String>,
    /// Human-readable name
    name: String,
    /// The instruction for the sub-agent
    instruction: String,
    /// Optional specialty for the sub-agent (e.g., "Code Writer", "Researcher")
    #[serde(default)]
    specialty: Option<String>,
}

#[derive(serde::Serialize)]
struct TaskResult {
    task_id: String,
    task_name: String,
    success: bool,
    output: String,
    error: Option<String>,
    steps: usize,
    tool_calls: usize,
}

#[async_trait]
impl Tool for SwarmExecuteTool {
    fn id(&self) -> &str {
        "swarm_execute"
    }

    fn name(&self) -> &str {
        "Swarm Execute"
    }

    fn description(&self) -> &str {
        "Execute multiple tasks in parallel across multiple sub-agents. \
         Each task runs independently in its own agent context. \
         Returns aggregated results from all swarm participants. \
         Handles partial failures gracefully based on aggregation strategy."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "tasks": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "string",
                                "description": "Unique identifier for this task (auto-generated if not provided)"
                            },
                            "name": {
                                "type": "string",
                                "description": "Human-readable name for this task"
                            },
                            "instruction": {
                                "type": "string",
                                "description": "The instruction for the sub-agent to execute"
                            },
                            "specialty": {
                                "type": "string",
                                "description": "Optional specialty for the sub-agent (e.g., 'Code Writer', 'Researcher', 'Tester')"
                            }
                        },
                        "required": ["name", "instruction"]
                    },
                    "description": "Array of tasks to execute in parallel"
                },
                "concurrency_limit": {
                    "type": "integer",
                    "description": "Maximum number of concurrent agents (default: 5)",
                    "default": 5
                },
                "aggregation_strategy": {
                    "type": "string",
                    "enum": ["all", "first_error", "best_effort"],
                    "description": "How to aggregate results: 'all' (all must succeed), 'first_error' (stop on first error), 'best_effort' (collect all, report failures)",
                    "default": "best_effort"
                },
                "model": {
                    "type": "string",
                    "description": "Model to use for sub-agents (provider/model format, e.g., 'anthropic/claude-sonnet-4-20250514'). Defaults to configured default."
                },
                "max_steps": {
                    "type": "integer",
                    "description": "Maximum steps per sub-agent (default: 50)",
                    "default": 50
                },
                "timeout_secs": {
                    "type": "integer",
                    "description": "Timeout per sub-agent in seconds (default: 300)",
                    "default": 300
                }
            },
            "required": ["tasks"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        if p.tasks.is_empty() {
            return Ok(ToolResult::error("At least one task is required"));
        }

        let concurrency = p.concurrency_limit.min(20).max(1);

        tracing::info!(
            task_count = p.tasks.len(),
            concurrency = concurrency,
            strategy = %p.aggregation_strategy,
            "Starting swarm execution"
        );

        // Get provider registry from Vault
        let providers = ProviderRegistry::from_vault().await.context("Failed to load providers")?;
        let provider_list = providers.list();

        if provider_list.is_empty() {
            return Ok(ToolResult::error("No providers available for swarm execution"));
        }

        // Determine model to use
        let (provider_name, model_name) = if let Some(ref model_str) = p.model {
            let (prov, mod_id) = parse_model_string(model_str);
            let prov = prov.map(|p| if p == "zhipuai" { "zai" } else { p });
            (
                prov.filter(|p| provider_list.contains(p))
                    .unwrap_or(provider_list[0])
                    .to_string(),
                mod_id.to_string(),
            )
        } else {
            // Default to GLM-5 via Z.AI for swarm
            let provider = if provider_list.contains(&"zai") {
                "zai".to_string()
            } else if provider_list.contains(&"openrouter") {
                "openrouter".to_string()
            } else {
                provider_list[0].to_string()
            };
            let model = "glm-5".to_string();
            (provider, model)
        };

        let provider = providers
            .get(&provider_name)
            .context("Failed to get provider")?;

        tracing::info!(provider = %provider_name, model = %model_name, "Using provider for swarm");

        // Get tool definitions (filtered for sub-agents)
        let tools = Self::get_subagent_tools();

        // System prompt for sub-agents
        let system_prompt = r#"You are a sub-agent in a swarm execution context.
Your role is to execute the given task independently and report your results.
Focus on completing your specific task efficiently.
Use available tools to accomplish your goal.
When done, provide a clear summary of what you accomplished.
Share any intermediate results using the swarm_share tool so other agents can benefit."#;

        // Execute tasks concurrently using semaphore for rate limiting
        let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
        let mut join_handles = Vec::new();

        for task_input in p.tasks.clone() {
            let semaphore = semaphore.clone();
            let provider = provider.clone();
            let tools = tools.clone();
            let system_prompt = system_prompt.to_string();
            let task_id = task_input
                .id
                .clone()
                .unwrap_or_else(|| format!("task_{}", uuid::Uuid::new_v4()));
            let model_name = model_name.clone();
            let max_steps = p.max_steps;
            let timeout_secs = p.timeout_secs;

            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();

                let user_prompt = format!(
                    "Task: {}\n\nInstruction: {}",
                    task_input.name, task_input.instruction
                );

                let (output, steps, tool_calls, exit) = run_agent_loop(
                    provider,
                    &model_name,
                    &system_prompt,
                    &user_prompt,
                    tools,
                    ToolRegistry::new(),
                    max_steps,
                    timeout_secs,
                    None,
                    task_id.clone(),
                    None,
                )
                .await?;

                let success = matches!(exit, crate::swarm::executor::AgentLoopExit::Finished)
                    || matches!(exit, crate::swarm::executor::AgentLoopExit::MaxStepsReached);

                Ok(TaskResult {
                    task_id,
                    task_name: task_input.name,
                    success,
                    output,
                    error: if success { None } else { Some(format!("{:?}", exit)) },
                    steps,
                    tool_calls,
                })
            });

            join_handles.push(handle);
        }

        // Wait for all tasks to complete
        let mut results: Vec<TaskResult> = Vec::new();
        let mut failures = 0;

        for handle in join_handles {
            match handle.await {
                Ok(Ok(result)) => {
                    if !result.success {
                        failures += 1;

                        // Handle aggregation strategies
                        match p.aggregation_strategy.as_str() {
                            "all" => {
                                // Return immediately on first failure
                                return Ok(ToolResult::success(json!({
                                    "status": "failed",
                                    "failed_task": result.task_name,
                                    "error": result.error,
                                    "results": [result],
                                    "summary": {
                                        "total": 1,
                                        "success": 0,
                                        "failures": 1
                                    }
                                }).to_string()));
                            }
                            "first_error" => {
                                return Ok(ToolResult::success(json!({
                                    "status": "error",
                                    "error": result.error,
                                    "failed_task": result.task_name,
                                    "completed_tasks": results.len(),
                                    "results": results,
                                }).to_string()));
                            }
                            _ => {} // "best_effort" - continue collecting
                        }
                    }
                    results.push(result);
                }
                Ok(Err(e)) => {
                    failures += 1;
                    tracing::error!(error = %e, "Task execution failed");
                }
                Err(e) => {
                    failures += 1;
                    tracing::error!(error = %e, "Task join failed");
                }
            }
        }

        // Build aggregation response
        let total = results.len();
        let successes = results.iter().filter(|r| r.success).count();

        let response = if failures == 0 {
            json!({
                "status": "success",
                "results": results,
                "summary": {
                    "total": total,
                    "success": successes,
                    "failures": failures
                }
            })
        } else {
            match p.aggregation_strategy.as_str() {
                "all" => json!({
                    "status": "partial_failure",
                    "results": results,
                    "summary": {
                        "total": total,
                        "success": successes,
                        "failures": failures
                    }
                }),
                "first_error" => json!({
                    "status": "error",
                    "results": results,
                    "summary": {
                        "total": total,
                        "success": successes,
                        "failures": failures
                    }
                }),
                _ => json!({
                    "status": "partial_success",
                    "results": results,
                    "summary": {
                        "total": total,
                        "success": successes,
                        "failures": failures
                    }
                }),
            }
        };

        Ok(ToolResult::success(response.to_string()))
    }
}

impl SwarmExecuteTool {
    /// Get tool definitions suitable for sub-agents
    fn get_subagent_tools() -> Vec<crate::provider::ToolDefinition> {
        // Filter out interactive/blocking tools that don't work well for sub-agents
        let registry = ToolRegistry::new();
        registry
            .definitions()
            .into_iter()
            .filter(|t| {
                !matches!(
                    t.name.as_str(),
                    "question"
                        | "confirm_edit"
                        | "confirm_multiedit"
                        | "plan_enter"
                        | "plan_exit"
                        | "swarm_execute"
                        | "agent"
                )
            })
            .collect()
    }
}
