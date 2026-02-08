//! Swarm pre-flight validation
//!
//! Validates swarm execution configuration before starting to catch common issues early
//! and provide helpful error messages.

use super::{SubTask, SwarmConfig};
use crate::provider::ProviderRegistry;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Result of pre-flight validation
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Whether all checks passed
    pub is_valid: bool,
    /// List of validation issues found
    pub issues: Vec<ValidationIssue>,
    /// Estimated token usage for the swarm execution
    pub estimated_tokens: TokenEstimate,
    /// Provider availability status
    pub provider_status: ProviderStatus,
    /// Workspace/git state summary
    pub workspace_status: WorkspaceStatus,
}

/// A single validation issue
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// Severity level
    pub severity: IssueSeverity,
    /// Category of the issue
    pub category: IssueCategory,
    /// Human-readable description
    pub message: String,
    /// Suggested fix (if applicable)
    pub suggestion: Option<String>,
}

/// Severity levels for validation issues
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueSeverity {
    /// Critical - execution cannot proceed
    Error,
    /// Warning - execution can proceed but may have issues
    Warning,
    /// Info - informational only
    Info,
}

/// Categories of validation issues
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueCategory {
    /// Provider/credential issues
    Provider,
    /// Workspace/git state issues
    Workspace,
    /// Configuration issues
    Configuration,
    /// Subtask dependency issues
    Dependencies,
    /// Token usage estimation
    TokenEstimate,
}

/// Estimated token usage
#[derive(Debug, Clone, Default)]
pub struct TokenEstimate {
    /// Estimated prompt tokens
    pub prompt_tokens: usize,
    /// Estimated completion tokens
    pub completion_tokens: usize,
    /// Estimated total tokens
    pub total_tokens: usize,
    /// Whether this exceeds recommended limits
    pub exceeds_limit: bool,
    /// Context window of the selected model
    pub context_window: usize,
}

/// Provider availability status
#[derive(Debug, Clone)]
pub struct ProviderStatus {
    /// Selected provider name
    pub provider: String,
    /// Selected model name
    pub model: String,
    /// Whether the provider is available
    pub is_available: bool,
    /// Whether credentials are configured
    pub has_credentials: bool,
    /// Context window size
    pub context_window: usize,
}

/// Workspace/git state status
#[derive(Debug, Clone)]
pub struct WorkspaceStatus {
    /// Whether we're in a git repository
    pub is_git_repo: bool,
    /// Current branch name
    pub current_branch: Option<String>,
    /// Number of uncommitted changes
    pub uncommitted_changes: usize,
    /// Whether there are untracked files
    pub has_untracked_files: bool,
    /// Whether worktrees can be created
    pub can_create_worktrees: bool,
}

/// Pre-flight validator for swarm execution
pub struct SwarmValidator {
    config: SwarmConfig,
    provider: String,
    model: String,
}

impl SwarmValidator {
    /// Create a new validator
    pub fn new(config: SwarmConfig, provider: String, model: String) -> Self {
        Self {
            config,
            provider,
            model,
        }
    }

    /// Run all validation checks
    pub async fn validate(&self, subtasks: &[SubTask]) -> Result<ValidationReport> {
        let mut issues = Vec::new();

        // Validate provider
        let provider_status = self.validate_provider(&mut issues).await?;

        // Validate workspace/git state
        let workspace_status = self.validate_workspace(&mut issues)?;

        // Validate configuration
        self.validate_configuration(&mut issues);

        // Validate subtask dependencies
        self.validate_dependencies(subtasks, &mut issues);

        // Estimate token usage
        let estimated_tokens =
            self.estimate_token_usage(subtasks, &provider_status, &mut issues);

        // Determine if valid (no errors)
        let is_valid = !issues.iter().any(|i| i.severity == IssueSeverity::Error);

        Ok(ValidationReport {
            is_valid,
            issues,
            estimated_tokens,
            provider_status,
            workspace_status,
        })
    }

    /// Validate provider availability and credentials
    async fn validate_provider(
        &self,
        issues: &mut Vec<ValidationIssue>,
    ) -> Result<ProviderStatus> {
        let registry = match ProviderRegistry::from_vault().await {
            Ok(r) => r,
            Err(e) => {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Error,
                    category: IssueCategory::Provider,
                    message: format!("Failed to load provider registry: {e}"),
                    suggestion: Some(
                        "Check VAULT_ADDR and VAULT_TOKEN environment variables".to_string(),
                    ),
                });
                return Ok(ProviderStatus {
                    provider: self.provider.clone(),
                    model: self.model.clone(),
                    is_available: false,
                    has_credentials: false,
                    context_window: 0,
                });
            }
        };

        let available_providers = registry.list();
        let is_available = available_providers.contains(&self.provider.as_str());

        if !is_available {
            let available = available_providers.join(", ");
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Provider,
                message: format!(
                    "Provider '{}' is not available. Available providers: {}",
                    self.provider, available
                ),
                suggestion: Some(format!(
                    "Set credentials in Vault at secret/codetether/providers/{}",
                    self.provider
                )),
            });
        }

        // Get context window for the model
        let context_window = if let Some(provider) = registry.get(&self.provider) {
            match provider.list_models().await {
                Ok(models) => models
                    .iter()
                    .find(|m| m.id == self.model)
                    .map(|m| m.context_window)
                    .unwrap_or(128_000),
                Err(_) => 128_000,
            }
        } else {
            128_000
        };

        Ok(ProviderStatus {
            provider: self.provider.clone(),
            model: self.model.clone(),
            is_available,
            has_credentials: is_available,
            context_window,
        })
    }

    /// Validate workspace and git state
    fn validate_workspace(&self, issues: &mut Vec<ValidationIssue>) -> Result<WorkspaceStatus> {
        let current_dir = std::env::current_dir()?;

        // Check if we're in a git repository
        let is_git_repo = Path::new(".git").exists()
            || std::process::Command::new("git")
                .args(["rev-parse", "--git-dir"])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

        let current_branch = if is_git_repo {
            std::process::Command::new("git")
                .args(["rev-parse", "--abbrev-ref", "HEAD"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
        } else {
            None
        };

        // Check for uncommitted changes
        let uncommitted_changes = if is_git_repo {
            std::process::Command::new("git")
                .args(["status", "--porcelain"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.lines().filter(|l| !l.is_empty()).count())
                .unwrap_or(0)
        } else {
            0
        };

        let has_untracked_files = if is_git_repo {
            std::process::Command::new("git")
                .args(["status", "--porcelain"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.lines().any(|l| l.starts_with("??")))
                .unwrap_or(false)
        } else {
            false
        };

        // Check if worktrees can be created
        let can_create_worktrees = if is_git_repo {
            std::process::Command::new("git")
                .args(["worktree", "list"])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        } else {
            false
        };

        // Add warnings for workspace issues
        if !is_git_repo {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Workspace,
                message: "Not in a git repository. Worktree isolation will be disabled.".to_string(),
                suggestion: Some(
                    "Initialize a git repository with 'git init' or run from a git repo".to_string(),
                ),
            });
        } else if uncommitted_changes > 0 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Workspace,
                message: format!(
                    "Found {uncommitted_changes} uncommitted change(s) in the working directory"
                ),
                suggestion: Some(
                    "Consider committing or stashing changes before running swarm".to_string(),
                ),
            });
        }

        if !can_create_worktrees && is_git_repo {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Workspace,
                message: "Unable to verify worktree creation capability".to_string(),
                suggestion: Some(
                    "Ensure git worktree is available and you have proper permissions".to_string(),
                ),
            });
        }

        Ok(WorkspaceStatus {
            is_git_repo,
            current_branch,
            uncommitted_changes,
            has_untracked_files,
            can_create_worktrees,
        })
    }

    /// Validate swarm configuration
    fn validate_configuration(&self, issues: &mut Vec<ValidationIssue>) {
        // Check max_subagents
        if self.config.max_subagents == 0 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Configuration,
                message: "max_subagents must be greater than 0".to_string(),
                suggestion: Some("Set max_subagents to at least 1".to_string()),
            });
        } else if self.config.max_subagents > 100 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Configuration,
                message: format!(
                    "max_subagents ({}) is very high and may cause rate limiting issues",
                    self.config.max_subagents
                ),
                suggestion: Some("Consider reducing max_subagents to 50 or less".to_string()),
            });
        }

        // Check timeout
        if self.config.subagent_timeout_secs == 0 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Configuration,
                message: "subagent_timeout_secs must be greater than 0".to_string(),
                suggestion: Some("Set subagent_timeout_secs to at least 60".to_string()),
            });
        }

        // Check max steps
        if self.config.max_steps_per_subagent == 0 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Configuration,
                message: "max_steps_per_subagent must be greater than 0".to_string(),
                suggestion: Some("Set max_steps_per_subagent to at least 10".to_string()),
            });
        }
    }

    /// Validate subtask dependencies for cycles and missing references
    fn validate_dependencies(&self, subtasks: &[SubTask], issues: &mut Vec<ValidationIssue>) {
        let subtask_ids: HashSet<&str> = subtasks.iter().map(|s| s.id.as_str()).collect();
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

        // Build dependency graph and check for missing dependencies
        for subtask in subtasks {
            let deps: Vec<&str> = subtask
                .dependencies
                .iter()
                .map(|d| d.as_str())
                .collect();

            // Check for missing dependencies
            for dep in &deps {
                if !subtask_ids.contains(dep) {
                    issues.push(ValidationIssue {
                        severity: IssueSeverity::Error,
                        category: IssueCategory::Dependencies,
                        message: format!(
                            "Subtask '{}' depends on missing subtask '{}'",
                            subtask.name, dep
                        ),
                        suggestion: Some(format!(
                            "Create subtask with id '{}' or remove the dependency",
                            dep
                        )),
                    });
                }
            }

            graph.insert(&subtask.id, deps);
        }

        // Check for cycles using DFS
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        fn has_cycle<'a>(
            node: &'a str,
            graph: &HashMap<&'a str, Vec<&'a str>>,
            visited: &mut HashSet<&'a str>,
            rec_stack: &mut HashSet<&'a str>,
            path: &mut Vec<&'a str>,
        ) -> Option<Vec<&'a str>> {
            visited.insert(node);
            rec_stack.insert(node);
            path.push(node);

            if let Some(neighbors) = graph.get(node) {
                for &neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        if let Some(cycle) =
                            has_cycle(neighbor, graph, visited, rec_stack, path)
                        {
                            return Some(cycle);
                        }
                    } else if rec_stack.contains(neighbor) {
                        // Found a cycle - extract the cycle from path
                        let cycle_start = path.iter().position(|&n| n == neighbor).unwrap();
                        let cycle: Vec<_> = path[cycle_start..].iter().copied().collect();
                        return Some(cycle);
                    }
                }
            }

            path.pop();
            rec_stack.remove(node);
            None
        }

        for &subtask_id in subtask_ids.iter() {
            if !visited.contains(subtask_id) {
                let mut path = Vec::new();
                if let Some(cycle) =
                    has_cycle(subtask_id, &graph, &mut visited, &mut rec_stack, &mut path)
                {
                    let cycle_str = cycle.join(" -> ");
                    issues.push(ValidationIssue {
                        severity: IssueSeverity::Error,
                        category: IssueCategory::Dependencies,
                        message: format!("Circular dependency detected: {cycle_str}"),
                        suggestion: Some(
                            "Break the cycle by removing one of the dependencies".to_string(),
                        ),
                    });
                    break; // Only report first cycle
                }
            }
        }
    }

    /// Estimate token usage for the swarm execution
    fn estimate_token_usage(
        &self,
        subtasks: &[SubTask],
        provider_status: &ProviderStatus,
        issues: &mut Vec<ValidationIssue>,
    ) -> TokenEstimate {
        // Rough estimation: ~4 characters per token on average
        const CHARS_PER_TOKEN: usize = 4;

        // Base prompt overhead (system message, formatting, etc.)
        let base_prompt_tokens = 500;

        // Calculate tokens from subtask instructions
        let instruction_tokens: usize = subtasks
            .iter()
            .map(|s| s.instruction.len() / CHARS_PER_TOKEN)
            .sum();

        // Calculate tokens from context
        let context_tokens: usize = subtasks
            .iter()
            .map(|s| {
                s.context
                    .dependency_results
                    .values()
                    .map(|v| v.len() / CHARS_PER_TOKEN)
                    .sum::<usize>()
            })
            .sum();

        let prompt_tokens = base_prompt_tokens + instruction_tokens + context_tokens;

        // Estimate completion tokens (assume ~50% of prompt for responses)
        let completion_tokens = prompt_tokens / 2;
        let total_tokens = prompt_tokens + completion_tokens;

        // Check if exceeds context window
        let exceeds_limit = total_tokens > provider_status.context_window;

        if exceeds_limit {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::TokenEstimate,
                message: format!(
                    "Estimated token usage ({} tokens) exceeds model context window ({} tokens)",
                    total_tokens, provider_status.context_window
                ),
                suggestion: Some(
                    "Reduce subtask complexity or split into smaller subtasks".to_string(),
                ),
            });
        } else if total_tokens > provider_status.context_window * 8 / 10 {
            // Warning at 80% of context window
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::TokenEstimate,
                message: format!(
                    "Estimated token usage ({} tokens) is at {}% of context window",
                    total_tokens,
                    (total_tokens * 100) / provider_status.context_window
                ),
                suggestion: Some(
                    "Consider reducing subtask complexity to avoid token limits".to_string(),
                ),
            });
        }

        TokenEstimate {
            prompt_tokens,
            completion_tokens,
            total_tokens,
            exceeds_limit,
            context_window: provider_status.context_window,
        }
    }
}

impl ValidationReport {
    /// Format the report as a human-readable string
    pub fn format(&self) -> String {
        let mut output = String::new();

        // Summary
        if self.is_valid {
            output.push_str("✓ Swarm pre-flight validation passed\n\n");
        } else {
            output.push_str("✗ Swarm pre-flight validation failed\n\n");
        }

        // Provider status
        output.push_str(&format!(
            "Provider: {} ({}) - {}\n",
            self.provider_status.provider,
            self.provider_status.model,
            if self.provider_status.is_available {
                "✓ Available"
            } else {
                "✗ Unavailable"
            }
        ));

        // Workspace status
        output.push_str(&format!(
            "Workspace: {}\n",
            if self.workspace_status.is_git_repo {
                format!(
                    "✓ Git repo (branch: {})",
                    self.workspace_status
                        .current_branch
                        .as_deref()
                        .unwrap_or("unknown")
                )
            } else {
                "✗ Not a git repo".to_string()
            }
        ));

        if self.workspace_status.uncommitted_changes > 0 {
            output.push_str(&format!(
                "  ⚠ {} uncommitted change(s)\n",
                self.workspace_status.uncommitted_changes
            ));
        }

        // Token estimate
        output.push_str(&format!(
            "Token estimate: {} prompt + {} completion = {} total (context: {})\n",
            self.estimated_tokens.prompt_tokens,
            self.estimated_tokens.completion_tokens,
            self.estimated_tokens.total_tokens,
            self.estimated_tokens.context_window
        ));

        if self.estimated_tokens.exceeds_limit {
            output.push_str("  ⚠ Exceeds context window!\n");
        }

        // Issues
        if !self.issues.is_empty() {
            output.push_str("\nIssues:\n");
            for issue in &self.issues {
                let icon = match issue.severity {
                    IssueSeverity::Error => "✗",
                    IssueSeverity::Warning => "⚠",
                    IssueSeverity::Info => "ℹ",
                };
                output.push_str(&format!(
                    "  {} [{}] {}\n",
                    icon,
                    format!("{:?}", issue.category).to_lowercase(),
                    issue.message
                ));
                if let Some(ref suggestion) = issue.suggestion {
                    output.push_str(&format!("    → {suggestion}\n"));
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dependencies_no_cycle() {
        let config = SwarmConfig::default();
        let validator = SwarmValidator::new(config, "test".to_string(), "model".to_string());

        let mut issues = Vec::new();
        let subtasks = vec![
            SubTask::new("Task A", "Do A"),
            SubTask::new("Task B", "Do B").with_dependencies(vec![]),
            SubTask::new("Task C", "Do C"),
        ];

        validator.validate_dependencies(&subtasks, &mut issues);

        assert!(issues.is_empty());
    }

    #[test]
    fn test_validate_dependencies_cycle() {
        let config = SwarmConfig::default();
        let validator = SwarmValidator::new(config, "test".to_string(), "model".to_string());

        let mut subtask_a = SubTask::new("Task A", "Do A");
        let mut subtask_b = SubTask::new("Task B", "Do B");
        let mut subtask_c = SubTask::new("Task C", "Do C");

        // Create cycle: A -> B -> C -> A
        subtask_a.dependencies = vec![subtask_c.id.clone()];
        subtask_b.dependencies = vec![subtask_a.id.clone()];
        subtask_c.dependencies = vec![subtask_b.id.clone()];

        let subtasks = vec![subtask_a, subtask_b, subtask_c];
        let mut issues = Vec::new();

        validator.validate_dependencies(&subtasks, &mut issues);

        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].severity, IssueSeverity::Error);
        assert!(issues[0].message.contains("Circular dependency"));
    }

    #[test]
    fn test_validate_dependencies_missing() {
        let config = SwarmConfig::default();
        let validator = SwarmValidator::new(config, "test".to_string(), "model".to_string());

        let subtask_a = SubTask::new("Task A", "Do A")
            .with_dependencies(vec!["non-existent-id".to_string()]);

        let subtasks = vec![subtask_a];
        let mut issues = Vec::new();

        validator.validate_dependencies(&subtasks, &mut issues);

        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].severity, IssueSeverity::Error);
        assert!(issues[0].message.contains("missing subtask"));
    }

    #[test]
    fn test_validate_configuration() {
        let mut config = SwarmConfig::default();
        config.max_subagents = 0;
        config.subagent_timeout_secs = 0;

        let validator = SwarmValidator::new(config, "test".to_string(), "model".to_string());

        let mut issues = Vec::new();
        validator.validate_configuration(&mut issues);

        assert_eq!(issues.len(), 2);
        assert!(issues.iter().any(|i| i.message.contains("max_subagents")));
        assert!(issues
            .iter()
            .any(|i| i.message.contains("subagent_timeout_secs")));
    }

    #[test]
    fn test_token_estimate() {
        let config = SwarmConfig::default();
        let validator = SwarmValidator::new(config, "test".to_string(), "model".to_string());

        let provider_status = ProviderStatus {
            provider: "test".to_string(),
            model: "model".to_string(),
            is_available: true,
            has_credentials: true,
            context_window: 1000,
        };

        // Create subtasks with large instructions to exceed limit
        let subtasks: Vec<SubTask> = (0..10)
            .map(|i| SubTask::new(&format!("Task {i}"), &"x".repeat(1000)))
            .collect();

        let mut issues = Vec::new();
        let estimate = validator.estimate_token_usage(&subtasks, &provider_status, &mut issues);

        assert!(estimate.total_tokens > 0);
        assert!(estimate.prompt_tokens > 0);
        assert!(estimate.completion_tokens > 0);
    }
}
