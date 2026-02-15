//! Ralph types - PRD and state structures

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A user story in the PRD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStory {
    /// Unique identifier (e.g., "US-001")
    pub id: String,

    /// Short title
    pub title: String,

    /// Full description
    pub description: String,

    /// Acceptance criteria
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,

    /// Whether this story passes all tests
    #[serde(default)]
    pub passes: bool,

    /// Story priority (1=highest)
    #[serde(default = "default_priority")]
    pub priority: u8,

    /// Dependencies on other story IDs
    #[serde(default, alias = "dependencies")]
    pub depends_on: Vec<String>,

    /// Estimated complexity (1-5)
    #[serde(default = "default_complexity")]
    pub complexity: u8,
}

fn default_priority() -> u8 {
    1
}
fn default_complexity() -> u8 {
    3
}

/// The full PRD structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prd {
    /// Project name
    pub project: String,

    /// Feature being implemented
    pub feature: String,

    /// Git branch name for this PRD
    #[serde(default)]
    pub branch_name: String,

    /// Version of the PRD format
    #[serde(default = "default_version")]
    pub version: String,

    /// User stories to implement
    #[serde(default)]
    pub user_stories: Vec<UserStory>,

    /// Technical requirements
    #[serde(default)]
    pub technical_requirements: Vec<String>,

    /// Quality checks to run
    #[serde(default)]
    pub quality_checks: QualityChecks,

    /// Created timestamp
    #[serde(default)]
    pub created_at: String,

    /// Last updated timestamp
    #[serde(default)]
    pub updated_at: String,
}

fn default_version() -> String {
    "1.0".to_string()
}

/// Quality checks configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QualityChecks {
    /// Command to run type checking
    #[serde(default)]
    pub typecheck: Option<String>,

    /// Command to run tests
    #[serde(default)]
    pub test: Option<String>,

    /// Command to run linting
    #[serde(default)]
    pub lint: Option<String>,

    /// Command to run build
    #[serde(default)]
    pub build: Option<String>,
}

impl Prd {
    /// Load a PRD from a JSON file
    pub async fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let prd: Prd = serde_json::from_str(&content)?;
        Ok(prd)
    }

    /// Save the PRD to a JSON file
    pub async fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }

    /// Get the next story to work on (not passed, dependencies met)
    pub fn next_story(&self) -> Option<&UserStory> {
        self.user_stories
            .iter()
            .filter(|s| !s.passes)
            .filter(|s| self.dependencies_met(&s.depends_on))
            .min_by_key(|s| (s.priority, s.complexity))
    }

    /// Check if all dependencies are met (all passed)
    fn dependencies_met(&self, deps: &[String]) -> bool {
        deps.iter().all(|dep_id| {
            self.user_stories
                .iter()
                .find(|s| s.id == *dep_id)
                .map(|s| s.passes)
                .unwrap_or(true)
        })
    }

    /// Get count of passed stories
    pub fn passed_count(&self) -> usize {
        self.user_stories.iter().filter(|s| s.passes).count()
    }

    /// Check if all stories are complete
    pub fn is_complete(&self) -> bool {
        self.user_stories.iter().all(|s| s.passes)
    }

    /// Mark a story as passed
    pub fn mark_passed(&mut self, story_id: &str) {
        if let Some(story) = self.user_stories.iter_mut().find(|s| s.id == *story_id) {
            story.passes = true;
        }
    }

    /// Get all stories ready to be worked on (not passed, dependencies met)
    pub fn ready_stories(&self) -> Vec<&UserStory> {
        self.user_stories
            .iter()
            .filter(|s| !s.passes)
            .filter(|s| self.dependencies_met(&s.depends_on))
            .collect()
    }

    /// Group stories into parallel execution stages based on dependencies
    /// Returns a Vec of stages, where each stage is a Vec of stories that can run in parallel
    pub fn stages(&self) -> Vec<Vec<&UserStory>> {
        let mut stages: Vec<Vec<&UserStory>> = Vec::new();
        let mut completed: std::collections::HashSet<String> = self
            .user_stories
            .iter()
            .filter(|s| s.passes)
            .map(|s| s.id.clone())
            .collect();

        let mut remaining: Vec<&UserStory> =
            self.user_stories.iter().filter(|s| !s.passes).collect();

        while !remaining.is_empty() {
            // Find all stories whose dependencies are met
            let (ready, not_ready): (Vec<_>, Vec<_>) = remaining
                .into_iter()
                .partition(|s| s.depends_on.iter().all(|dep| completed.contains(dep)));

            if ready.is_empty() {
                // Circular dependency or missing deps - just take remaining
                if !not_ready.is_empty() {
                    stages.push(not_ready);
                }
                break;
            }

            // Mark these as "will be completed" for next iteration
            for story in &ready {
                completed.insert(story.id.clone());
            }

            stages.push(ready);
            remaining = not_ready;
        }

        stages
    }
}

/// Ralph execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RalphState {
    /// The PRD being worked on
    pub prd: Prd,

    /// Current iteration number
    pub current_iteration: usize,

    /// Maximum allowed iterations
    pub max_iterations: usize,

    /// Current status
    pub status: RalphStatus,

    /// Progress log entries
    #[serde(default)]
    pub progress_log: Vec<ProgressEntry>,

    /// Path to the PRD file
    pub prd_path: PathBuf,

    /// Working directory
    pub working_dir: PathBuf,
}

/// Ralph execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RalphStatus {
    Pending,
    Running,
    Completed,
    MaxIterations,
    Stopped,
    QualityFailed,
}

/// A progress log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEntry {
    /// Story ID being worked on
    pub story_id: String,

    /// Iteration number
    pub iteration: usize,

    /// Status of this attempt
    pub status: String,

    /// What was learned
    #[serde(default)]
    pub learnings: Vec<String>,

    /// Files changed
    #[serde(default)]
    pub files_changed: Vec<String>,

    /// Timestamp
    pub timestamp: String,
}

/// Ralph configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RalphConfig {
    /// Path to prd.json
    #[serde(default = "default_prd_path")]
    pub prd_path: String,

    /// Maximum iterations
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,

    /// Path to progress.txt
    #[serde(default = "default_progress_path")]
    pub progress_path: String,

    /// Whether to auto-commit changes
    #[serde(default = "default_auto_commit")]
    pub auto_commit: bool,

    /// Whether to run quality checks
    #[serde(default = "default_quality_checks_enabled")]
    pub quality_checks_enabled: bool,

    /// Model to use for iterations
    #[serde(default)]
    pub model: Option<String>,

    /// Whether to use RLM for progress compression
    #[serde(default)]
    pub use_rlm: bool,

    /// Enable parallel story execution
    #[serde(default = "default_parallel_enabled")]
    pub parallel_enabled: bool,

    /// Maximum concurrent stories to execute
    #[serde(default = "default_max_concurrent_stories")]
    pub max_concurrent_stories: usize,

    /// Use worktree isolation for parallel execution
    #[serde(default = "default_worktree_enabled")]
    pub worktree_enabled: bool,

    /// Timeout in seconds per step for story sub-agents (resets on each step)
    #[serde(default = "default_story_timeout_secs")]
    pub story_timeout_secs: u64,

    /// Maximum tool call steps per story sub-agent
    /// Increase this for complex stories that need more iterations
    #[serde(default = "default_max_steps_per_story")]
    pub max_steps_per_story: usize,

    /// Timeout in seconds per step for conflict resolution sub-agents
    #[serde(default = "default_conflict_timeout_secs")]
    pub conflict_timeout_secs: u64,

    /// Enable per-story relay teams (multi-agent collaboration per story)
    #[serde(default)]
    pub relay_enabled: bool,

    /// Maximum agents per relay team (2-8)
    #[serde(default = "default_relay_max_agents")]
    pub relay_max_agents: usize,

    /// Maximum relay rounds per story
    #[serde(default = "default_relay_max_rounds")]
    pub relay_max_rounds: usize,
}

fn default_prd_path() -> String {
    "prd.json".to_string()
}
fn default_max_iterations() -> usize {
    10
}
fn default_progress_path() -> String {
    "progress.txt".to_string()
}
fn default_auto_commit() -> bool {
    false
}
fn default_quality_checks_enabled() -> bool {
    true
}
fn default_parallel_enabled() -> bool {
    true
}
fn default_max_concurrent_stories() -> usize {
    100
}
fn default_worktree_enabled() -> bool {
    true
}
fn default_story_timeout_secs() -> u64 {
    300
}
fn default_max_steps_per_story() -> usize {
    30
}
fn default_conflict_timeout_secs() -> u64 {
    120
}
fn default_relay_max_agents() -> usize {
    8
}
fn default_relay_max_rounds() -> usize {
    3
}

impl Default for RalphConfig {
    fn default() -> Self {
        Self {
            prd_path: default_prd_path(),
            max_iterations: default_max_iterations(),
            progress_path: default_progress_path(),
            auto_commit: default_auto_commit(),
            quality_checks_enabled: default_quality_checks_enabled(),
            model: None,
            use_rlm: true,
            parallel_enabled: default_parallel_enabled(),
            max_concurrent_stories: default_max_concurrent_stories(),
            worktree_enabled: default_worktree_enabled(),
            story_timeout_secs: default_story_timeout_secs(),
            max_steps_per_story: default_max_steps_per_story(),
            conflict_timeout_secs: default_conflict_timeout_secs(),
            relay_enabled: true,
            relay_max_agents: default_relay_max_agents(),
            relay_max_rounds: default_relay_max_rounds(),
        }
    }
}
