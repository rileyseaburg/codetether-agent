# Ralph - Autonomous PRD-Driven Agent Loop

## Overview

Ralph is an autonomous AI agent loop that implements the "ralph pattern" (named after Geoffrey Huntley's approach: `while :; do cat PROMPT.md | llm; done`). It iterates through PRD (Product Requirements Document) user stories, running quality gates after each iteration, and uses RLM (Recursive Language Model) for progress compression when context gets too large.

## Architecture

```
src/ralph/
├── mod.rs           # Module exports
├── types.rs         # PRD structures and state types
└── ralph_loop.rs    # Core autonomous execution loop
```

## Public API

### Types (`types.rs`)

#### `UserStory`
Represents a single user story in the PRD.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStory {
    pub id: String,                          // Unique identifier (e.g., "US-001")
    pub title: String,                       // Short title
    pub description: String,                 // Full description
    pub acceptance_criteria: Vec<String>,    // Acceptance criteria
    pub passes: bool,                        // Whether this story passes all tests
    pub priority: u8,                        // Story priority (1=highest)
    pub depends_on: Vec<String>,             // Dependencies on other story IDs
    pub complexity: u8,                      // Estimated complexity (1-5)
}
```

#### `Prd`
The full PRD structure.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prd {
    pub project: String,                     // Project name
    pub feature: String,                     // Feature being implemented
    pub branch_name: String,                 // Git branch name for this PRD
    pub version: String,                     // Version of the PRD format
    pub user_stories: Vec<UserStory>,        // User stories to implement
    pub technical_requirements: Vec<String>, // Technical requirements
    pub quality_checks: QualityChecks,       // Quality checks to run
    pub created_at: String,                  // Created timestamp
    pub updated_at: String,                  // Last updated timestamp
}
```

**Methods on `Prd`:**
- `async fn load(path: &PathBuf) -> anyhow::Result<Self>` - Load a PRD from a JSON file
- `async fn save(&self, path: &PathBuf) -> anyhow::Result<()>` - Save the PRD to a JSON file
- `fn next_story(&self) -> Option<&UserStory>` - Get the next story to work on (not passed, dependencies met)
- `fn passed_count(&self) -> usize` - Get count of passed stories
- `fn is_complete(&self) -> bool` - Check if all stories are complete
- `fn mark_passed(&mut self, story_id: &str)` - Mark a story as passed
- `fn ready_stories(&self) -> Vec<&UserStory>` - Get all stories ready to be worked on
- `fn stages(&self) -> Vec<Vec<&UserStory>>` - Group stories into parallel execution stages based on dependencies

#### `QualityChecks`
Configuration for quality gates.

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QualityChecks {
    pub typecheck: Option<String>,  // Command to run type checking
    pub test: Option<String>,       // Command to run tests
    pub lint: Option<String>,       // Command to run linting
    pub build: Option<String>,      // Command to run build
}
```

#### `RalphState`
Ralph execution state.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RalphState {
    pub prd: Prd,                          // The PRD being worked on
    pub current_iteration: usize,          // Current iteration number
    pub max_iterations: usize,             // Maximum allowed iterations
    pub status: RalphStatus,               // Current status
    pub progress_log: Vec<ProgressEntry>,  // Progress log entries
    pub prd_path: PathBuf,                 // Path to the PRD file
    pub working_dir: PathBuf,              // Working directory
}
```

#### `RalphStatus`
Execution status enum.

```rust
pub enum RalphStatus {
    Pending,
    Running,
    Completed,
    MaxIterations,
    Stopped,
    QualityFailed,
}
```

#### `ProgressEntry`
A progress log entry.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEntry {
    pub story_id: String,       // Story ID being worked on
    pub iteration: usize,       // Iteration number
    pub status: String,         // Status of this attempt
    pub learnings: Vec<String>, // What was learned
    pub files_changed: Vec<String>, // Files changed
    pub timestamp: String,      // Timestamp
}
```

#### `RalphConfig`
Configuration for the Ralph loop.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RalphConfig {
    pub prd_path: String,                  // Path to prd.json
    pub max_iterations: usize,             // Maximum iterations (default: 10)
    pub progress_path: String,             // Path to progress.txt
    pub auto_commit: bool,                 // Whether to auto-commit changes
    pub quality_checks_enabled: bool,      // Whether to run quality checks
    pub model: Option<String>,             // Model to use for iterations
    pub use_rlm: bool,                     // Whether to use RLM for progress compression
    pub parallel_enabled: bool,            // Enable parallel story execution
    pub max_concurrent_stories: usize,     // Maximum concurrent stories (default: 3)
    pub worktree_enabled: bool,            // Use worktree isolation for parallel execution
}
```

### Core Loop (`ralph_loop.rs`)

#### `RalphLoop`
The main Ralph executor.

```rust
pub struct RalphLoop {
    state: RalphState,
    provider: Arc<dyn Provider>,
    model: String,
    config: RalphConfig,
}
```

**Methods:**

- `async fn new(prd_path: PathBuf, provider: Arc<dyn Provider>, model: String, config: RalphConfig) -> anyhow::Result<Self>`
  - Create a new Ralph loop instance

- `async fn run(&mut self) -> anyhow::Result<RalphState>`
  - Run the Ralph loop until completion or max iterations
  - Returns the final state

- `pub fn status(&self) -> &RalphState`
  - Get current status

- `pub fn status_markdown(&self) -> String`
  - Format status as markdown

#### `create_prd_template`
Creates a sample PRD template.

```rust
pub fn create_prd_template(project: &str, feature: &str) -> Prd
```

## Key Features

### 1. Sequential Execution Mode
The original behavior where stories are processed one at a time:
- Gets the next available story (respecting dependencies)
- Builds a prompt with context
- Calls the LLM with tool access
- Runs quality gates
- Commits changes if successful
- Saves updated PRD

### 2. Parallel Execution Mode
Processes independent stories in parallel using worktree isolation:
- Groups stories into stages based on dependencies
- Creates isolated git worktrees for each story
- Uses semaphore to limit concurrency (default: 3 concurrent)
- Merges successful stories back to main branch
- Handles merge conflicts with a dedicated conflict resolver sub-agent

### 3. Worktree Isolation
For parallel execution:
- Each story gets its own git worktree
- Workspace stub injection for Cargo project isolation
- Automatic cleanup on success or failure
- Keeps worktrees for debugging on failure

### 4. Quality Gates
Configurable quality checks run after each story:
- Type checking (e.g., `cargo check`)
- Linting (e.g., `cargo clippy`)
- Testing (e.g., `cargo test`)
- Building (e.g., `cargo build`)

### 5. Git Integration
- Automatic branch switching
- Auto-commit with conventional commit format: `feat({story_id}): {title}`
- Worktree creation and management
- Merge conflict detection and resolution
- Cleanup of orphaned worktrees/branches

### 6. Progress Tracking
- Persistent progress log in `progress.txt`
- PRD state saved after each story
- Learnings extraction from responses
- Structured progress entries with timestamps

### 7. Conflict Resolution
When parallel stories have merge conflicts:
- Spawns a dedicated conflict resolver sub-agent
- Provides context about both sides of the conflict
- Attempts intelligent resolution
- Completes merge if successful, aborts if not

## Tool Interface

Ralph is also exposed as a tool (`src/tool/ralph.rs`) with the following actions:

### Actions

- **`run`** - Start the Ralph loop with a PRD file
  - Parameters: `prd_path`, `max_iterations`
  - Returns metadata: `all_passed`, `ready_to_merge`, `feature_branch`, `passed`, `total`

- **`status`** - Check progress of current Ralph run
  - Parameters: `prd_path`
  - Returns current progress summary

- **`create-prd`** - Create a new PRD template
  - Parameters: `project`, `feature`, `prd_path`
  - Creates a sample PRD file

## Usage Examples

### CLI Usage
```bash
# Create a PRD template
codetether ralph create-prd --project "MyProject" --feature "New Feature"

# Run Ralph on a PRD
codetether ralph run --prd prd.json --max-iterations 10

# Check status
codetether ralph status --prd prd.json
```

### Tool Usage
```rust
// Create PRD
ralph({action: 'create-prd', project: 'MyProject', feature: 'New Feature'})

// Run Ralph
ralph({action: 'run', prd_path: 'prd.json', max_iterations: 10})

// Check status
ralph({action: 'status', prd_path: 'prd.json'})
```

### Programmatic Usage
```rust
use codetether::ralph::{RalphLoop, RalphConfig, Prd};

// Load PRD
let prd = Prd::load(&PathBuf::from("prd.json")).await?;

// Create config
let config = RalphConfig {
    max_iterations: 10,
    parallel_enabled: true,
    max_concurrent_stories: 3,
    worktree_enabled: true,
    ..Default::default()
};

// Create and run Ralph
let mut ralph = RalphLoop::new(
    PathBuf::from("prd.json"),
    provider,
    model,
    config
).await?;

let state = ralph.run().await?;
```

## PRD JSON Format

```json
{
  "project": "project-name",
  "feature": "Feature Name",
  "branch_name": "feature/feature-name",
  "version": "1.0",
  "user_stories": [
    {
      "id": "US-001",
      "title": "Story title",
      "description": "What to implement",
      "acceptance_criteria": ["Criterion 1", "Criterion 2"],
      "priority": 1,
      "depends_on": [],
      "complexity": 3,
      "passes": false
    }
  ],
  "technical_requirements": [],
  "quality_checks": {
    "typecheck": "cargo check",
    "lint": "cargo clippy",
    "test": "cargo test",
    "build": "cargo build"
  },
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

## Memory Persistence

Ralph uses file-based memory (not context accumulation):
- `progress.txt` - Agent writes learnings/blockers
- `prd.json` - Tracks pass/fail status for each story
- Git history - Shows what changed per iteration

## Dependencies

- `crate::provider::Provider` - LLM provider for completions
- `crate::swarm::run_agent_loop` - Agent loop for sub-agents
- `crate::tool::ToolRegistry` - Tool registry for agent tools
- `crate::worktree::WorktreeManager` - Git worktree management
- `crate::agent::builtin::build_system_prompt` - System prompt construction

## Integration Points

1. **CLI** (`src/main.rs`) - Direct command-line interface
2. **Tool** (`src/tool/ralph.rs`) - Tool interface for agents
3. **Swarm** (`src/swarm/executor.rs`) - Used in sub-agent instructions
4. **MCP Server** (`src/mcp/server.rs`) - Documented capability
