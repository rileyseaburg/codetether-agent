//! Orchestrator for decomposing tasks and coordinating sub-agents
//!
//! The orchestrator analyzes complex tasks and decomposes them into
//! parallelizable subtasks, then coordinates their execution.

use super::{
    DecompositionStrategy, SubAgent, SubTask, SubTaskResult, SubTaskStatus, SwarmConfig, SwarmStats,
};
use crate::provider::{CompletionRequest, ContentPart, Message, ProviderRegistry, Role};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The orchestrator manages task decomposition and sub-agent coordination
pub struct Orchestrator {
    /// Configuration
    config: SwarmConfig,

    /// Provider registry for AI calls
    providers: ProviderRegistry,

    /// All subtasks
    subtasks: HashMap<String, SubTask>,

    /// All sub-agents
    subagents: HashMap<String, SubAgent>,

    /// Completed subtask IDs
    completed: Vec<String>,

    /// Current model for orchestration
    model: String,

    /// Current provider
    provider: String,

    /// Stats
    stats: SwarmStats,
}

impl Orchestrator {
    /// Create a new orchestrator
    pub async fn new(config: SwarmConfig) -> Result<Self> {
        use crate::provider::parse_model_string;

        let providers = ProviderRegistry::from_vault().await?;
        let provider_list = providers.list();

        if provider_list.is_empty() {
            anyhow::bail!("No providers available for orchestration");
        }

        // Parse model from config, env var, or use default
        let model_str = config
            .model
            .clone()
            .or_else(|| std::env::var("CODETETHER_DEFAULT_MODEL").ok());

        let (provider, model) = if let Some(ref model_str) = model_str {
            let (prov, mod_id) = parse_model_string(model_str);
            let prov = prov.map(|p| if p == "zhipuai" { "zai" } else { p });
            let provider = if let Some(explicit_provider) = prov {
                if provider_list.contains(&explicit_provider) {
                    explicit_provider.to_string()
                } else {
                    anyhow::bail!(
                        "Provider '{}' selected explicitly but is unavailable. Available providers: {}",
                        explicit_provider,
                        provider_list.join(", ")
                    );
                }
            } else {
                provider_list[0].to_string()
            };
            let model = if mod_id.trim().is_empty() {
                Self::default_model_for_provider(&provider)
            } else {
                mod_id.to_string()
            };
            (provider, model)
        } else {
            // Default to GLM-5 via Z.AI for swarm.
            let provider = if provider_list.contains(&"zai") {
                "zai".to_string()
            } else if provider_list.contains(&"openrouter") {
                "openrouter".to_string()
            } else {
                provider_list[0].to_string()
            };
            let model = Self::default_model_for_provider(&provider);
            (provider, model)
        };

        tracing::info!("Orchestrator using model {} via {}", model, provider);

        Ok(Self {
            config,
            providers,
            subtasks: HashMap::new(),
            subagents: HashMap::new(),
            completed: Vec::new(),
            model,
            provider,
            stats: SwarmStats::default(),
        })
    }

    /// Get default model for a provider
    fn default_model_for_provider(provider: &str) -> String {
        match provider {
            "moonshotai" => "kimi-k2.5".to_string(),
            "anthropic" => "claude-sonnet-4-20250514".to_string(),
            "bedrock" => "us.anthropic.claude-opus-4-6-v1:0".to_string(),
            "openai" => "gpt-4o".to_string(),
            "google" => "gemini-2.5-pro".to_string(),
            "zhipuai" | "zai" => "glm-5".to_string(),
            "openrouter" => "z-ai/glm-5".to_string(),
            "novita" => "qwen/qwen3-coder-next".to_string(),
            "github-copilot" | "github-copilot-enterprise" => "gpt-5-mini".to_string(),
            _ => "glm-5".to_string(),
        }
    }

    fn prefers_temperature_one(model: &str) -> bool {
        let normalized = model.to_ascii_lowercase();
        normalized.contains("kimi-k2")
            || normalized.contains("glm-")
            || normalized.contains("minimax")
    }

    /// Decompose a complex task into subtasks
    pub async fn decompose(
        &mut self,
        task: &str,
        strategy: DecompositionStrategy,
    ) -> Result<Vec<SubTask>> {
        if strategy == DecompositionStrategy::None {
            // Single task, no decomposition
            let subtask = SubTask::new("Main Task", task);
            self.subtasks.insert(subtask.id.clone(), subtask.clone());
            return Ok(vec![subtask]);
        }

        // Use AI to decompose the task
        let decomposition_prompt = self.build_decomposition_prompt(task, strategy);

        let provider = self
            .providers
            .get(&self.provider)
            .ok_or_else(|| anyhow::anyhow!("Provider {} not found", self.provider))?;

        let temperature = if Self::prefers_temperature_one(&self.model) {
            1.0
        } else {
            0.7
        };

        let request = CompletionRequest {
            messages: vec![Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: decomposition_prompt,
                }],
            }],
            tools: Vec::new(),
            model: self.model.clone(),
            temperature: Some(temperature),
            top_p: None,
            max_tokens: Some(8192),
            stop: Vec::new(),
        };

        let response = provider.complete(request).await?;

        // Parse the decomposition response
        let text = response
            .message
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Text { text } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");

        tracing::debug!("Decomposition response: {}", text);

        if text.trim().is_empty() {
            // Fallback to single task if decomposition fails
            tracing::warn!("Empty decomposition response, falling back to single task");
            let subtask = SubTask::new("Main Task", task);
            self.subtasks.insert(subtask.id.clone(), subtask.clone());
            return Ok(vec![subtask]);
        }

        let subtasks = self.parse_decomposition(&text)?;

        // Store subtasks
        for subtask in &subtasks {
            self.subtasks.insert(subtask.id.clone(), subtask.clone());
        }

        // Assign stages based on dependencies
        self.assign_stages();

        tracing::info!(
            "Decomposed task into {} subtasks across {} stages",
            subtasks.len(),
            self.max_stage() + 1
        );

        Ok(subtasks)
    }

    /// Build the decomposition prompt
    fn build_decomposition_prompt(&self, task: &str, strategy: DecompositionStrategy) -> String {
        let strategy_instruction = match strategy {
            DecompositionStrategy::Automatic => {
                "Analyze the task and determine the optimal way to decompose it into parallel subtasks."
            }
            DecompositionStrategy::ByDomain => {
                "Decompose the task by domain expertise (e.g., research, coding, analysis, verification)."
            }
            DecompositionStrategy::ByData => {
                "Decompose the task by data partition (e.g., different files, sections, or datasets)."
            }
            DecompositionStrategy::ByStage => {
                "Decompose the task by workflow stages (e.g., gather, process, synthesize)."
            }
            DecompositionStrategy::None => unreachable!(),
        };

        format!(
            r#"You are a task orchestrator. Your job is to decompose complex tasks into parallelizable subtasks.

TASK: {task}

STRATEGY: {strategy_instruction}

CONSTRAINTS:
- Maximum {max_subtasks} subtasks
- Each subtask should be independently executable
- Identify dependencies between subtasks (which must complete before others can start)
- Assign a specialty/role to each subtask

OUTPUT FORMAT (JSON):
```json
{{
  "subtasks": [
    {{
      "name": "Subtask Name",
      "instruction": "Detailed instruction for this subtask",
      "specialty": "Role/specialty (e.g., Researcher, Coder, Analyst)",
      "dependencies": ["id-of-dependency-1"],
      "priority": 1
    }}
  ]
}}
```

Decompose the task now:"#,
            task = task,
            strategy_instruction = strategy_instruction,
            max_subtasks = self.config.max_subagents,
        )
    }

    /// Parse the decomposition response
    fn parse_decomposition(&self, response: &str) -> Result<Vec<SubTask>> {
        // Try to extract JSON from the response
        let json_str = if let Some(start) = response.find("```json") {
            let start = start + 7;
            if let Some(end) = response[start..].find("```") {
                &response[start..start + end]
            } else {
                response
            }
        } else if let Some(start) = response.find('{') {
            if let Some(end) = response.rfind('}') {
                &response[start..=end]
            } else {
                response
            }
        } else {
            response
        };

        #[derive(Deserialize)]
        struct DecompositionResponse {
            subtasks: Vec<SubTaskDef>,
        }

        #[derive(Deserialize)]
        struct SubTaskDef {
            name: String,
            instruction: String,
            specialty: Option<String>,
            #[serde(default)]
            dependencies: Vec<String>,
            #[serde(default)]
            priority: i32,
        }

        let parsed: DecompositionResponse = serde_json::from_str(json_str.trim())
            .map_err(|e| anyhow::anyhow!("Failed to parse decomposition: {}", e))?;

        // Create SubTask objects with proper IDs
        let mut subtasks = Vec::new();
        let mut name_to_id: HashMap<String, String> = HashMap::new();

        // First pass: create subtasks and map names to IDs
        for def in &parsed.subtasks {
            let subtask = SubTask::new(&def.name, &def.instruction).with_priority(def.priority);

            let subtask = if let Some(ref specialty) = def.specialty {
                subtask.with_specialty(specialty)
            } else {
                subtask
            };

            name_to_id.insert(def.name.clone(), subtask.id.clone());
            subtasks.push((subtask, def.dependencies.clone()));
        }

        // Second pass: resolve dependencies
        let result: Vec<SubTask> = subtasks
            .into_iter()
            .map(|(mut subtask, deps)| {
                let resolved_deps: Vec<String> = deps
                    .iter()
                    .filter_map(|dep| name_to_id.get(dep).cloned())
                    .collect();
                subtask.dependencies = resolved_deps;
                subtask
            })
            .collect();

        Ok(result)
    }

    /// Assign stages to subtasks based on dependencies
    fn assign_stages(&mut self) {
        let mut changed = true;

        while changed {
            changed = false;

            // First collect all updates needed
            let updates: Vec<(String, usize)> = self
                .subtasks
                .iter()
                .filter_map(|(id, subtask)| {
                    if subtask.dependencies.is_empty() {
                        if subtask.stage != 0 {
                            Some((id.clone(), 0))
                        } else {
                            None
                        }
                    } else {
                        let max_dep_stage = subtask
                            .dependencies
                            .iter()
                            .filter_map(|dep_id| self.subtasks.get(dep_id))
                            .map(|dep| dep.stage)
                            .max()
                            .unwrap_or(0);

                        let new_stage = max_dep_stage + 1;
                        if subtask.stage != new_stage {
                            Some((id.clone(), new_stage))
                        } else {
                            None
                        }
                    }
                })
                .collect();

            // Then apply updates
            for (id, new_stage) in updates {
                if let Some(subtask) = self.subtasks.get_mut(&id) {
                    subtask.stage = new_stage;
                    changed = true;
                }
            }
        }
    }

    /// Get maximum stage number
    fn max_stage(&self) -> usize {
        self.subtasks.values().map(|s| s.stage).max().unwrap_or(0)
    }

    /// Get subtasks ready to execute (dependencies satisfied)
    pub fn ready_subtasks(&self) -> Vec<&SubTask> {
        self.subtasks
            .values()
            .filter(|s| s.status == SubTaskStatus::Pending && s.can_run(&self.completed))
            .collect()
    }

    /// Get subtasks for a specific stage
    pub fn subtasks_for_stage(&self, stage: usize) -> Vec<&SubTask> {
        self.subtasks
            .values()
            .filter(|s| s.stage == stage)
            .collect()
    }

    /// Create a sub-agent for a subtask
    pub fn create_subagent(&mut self, subtask: &SubTask) -> SubAgent {
        let specialty = subtask
            .specialty
            .clone()
            .unwrap_or_else(|| "General".to_string());
        let name = format!("{} Agent", specialty);

        let subagent = SubAgent::new(name, specialty, &subtask.id, &self.model, &self.provider);

        self.subagents.insert(subagent.id.clone(), subagent.clone());
        self.stats.subagents_spawned += 1;

        subagent
    }

    /// Mark a subtask as completed
    pub fn complete_subtask(&mut self, subtask_id: &str, result: SubTaskResult) {
        if let Some(subtask) = self.subtasks.get_mut(subtask_id) {
            subtask.complete(result.success);

            if result.success {
                self.completed.push(subtask_id.to_string());
                self.stats.subagents_completed += 1;
            } else {
                self.stats.subagents_failed += 1;
            }

            self.stats.total_tool_calls += result.tool_calls;
        }
    }

    /// Get all subtasks
    pub fn all_subtasks(&self) -> Vec<&SubTask> {
        self.subtasks.values().collect()
    }

    /// Get statistics
    pub fn stats(&self) -> &SwarmStats {
        &self.stats
    }

    /// Get mutable statistics
    pub fn stats_mut(&mut self) -> &mut SwarmStats {
        &mut self.stats
    }

    /// Check if all subtasks are complete
    pub fn is_complete(&self) -> bool {
        self.subtasks.values().all(|s| {
            matches!(
                s.status,
                SubTaskStatus::Completed | SubTaskStatus::Failed | SubTaskStatus::Cancelled
            )
        })
    }

    /// Get the provider registry
    pub fn providers(&self) -> &ProviderRegistry {
        &self.providers
    }

    /// Get current model
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Get current provider
    pub fn provider(&self) -> &str {
        &self.provider
    }
}

/// Message from sub-agent to orchestrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubAgentMessage {
    /// Progress update
    Progress {
        subagent_id: String,
        subtask_id: String,
        steps: usize,
        status: String,
    },

    /// Tool call made
    ToolCall {
        subagent_id: String,
        tool_name: String,
        success: bool,
    },

    /// Subtask completed
    Completed {
        subagent_id: String,
        result: SubTaskResult,
    },

    /// Request for resources
    ResourceRequest {
        subagent_id: String,
        resource_type: String,
        resource_id: String,
    },
}

/// Message from orchestrator to sub-agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrchestratorMessage {
    /// Start execution
    Start { subtask: Box<SubTask> },

    /// Provide resource
    Resource {
        resource_type: String,
        resource_id: String,
        content: String,
    },

    /// Terminate execution
    Terminate { reason: String },

    /// Context update (from completed dependency)
    ContextUpdate {
        dependency_id: String,
        result: String,
    },
}
