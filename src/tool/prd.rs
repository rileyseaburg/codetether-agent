//! PRD Tool - Generate PRD JSON from requirements via Q&A
//!
//! When a task is complex, this tool asks clarifying questions
//! to generate a structured PRD that can be used by ralph.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::PathBuf;

use super::{Tool, ToolResult};
use crate::ralph::{Prd, UserStory, QualityChecks};

/// Tool for generating PRDs from requirements
pub struct PrdTool;

impl Default for PrdTool {
    fn default() -> Self { Self::new() }
}

impl PrdTool {
    pub fn new() -> Self { Self }
}

#[derive(Deserialize)]
struct Params {
    action: String,
    #[serde(default)]
    task_description: Option<String>,
    #[serde(default)]
    project: Option<String>,
    #[serde(default)]
    feature: Option<String>,
    #[serde(default)]
    stories: Option<Vec<StoryInput>>,
    #[serde(default)]
    quality_checks: Option<QualityChecksInput>,
    #[serde(default)]
    prd_path: Option<String>,
}

#[derive(Deserialize)]
struct StoryInput {
    id: String,
    title: String,
    description: String,
    #[serde(default)]
    acceptance_criteria: Vec<String>,
    #[serde(default)]
    priority: Option<u8>,
    #[serde(default)]
    depends_on: Vec<String>,
    #[serde(default)]
    complexity: Option<u8>,
}

#[derive(Deserialize)]
struct QualityChecksInput {
    #[serde(default)]
    typecheck: Option<String>,
    #[serde(default)]
    test: Option<String>,
    #[serde(default)]
    lint: Option<String>,
    #[serde(default)]
    build: Option<String>,
}

#[async_trait]
impl Tool for PrdTool {
    fn id(&self) -> &str { "prd" }
    fn name(&self) -> &str { "PRD Generator" }
    
    fn description(&self) -> &str {
        r#"Generate a structured PRD (Product Requirements Document) for complex tasks.

Use this tool when you recognize a task is complex and needs to be broken down into user stories.

Actions:
- analyze: Analyze a task description and return what questions need answering
- generate: Generate a PRD JSON from provided answers
- save: Save a PRD to a file for ralph to execute

The workflow is:
1. Call analyze with the task_description to get questions
2. Answer the questions and call generate with your answers
3. Call save to write the PRD to prd.json
4. Invoke ralph to execute the PRD
"#
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["analyze", "generate", "save"],
                    "description": "Action to perform"
                },
                "task_description": {
                    "type": "string",
                    "description": "Description of the complex task (for analyze)"
                },
                "project": {
                    "type": "string",
                    "description": "Project name (for generate)"
                },
                "feature": {
                    "type": "string",
                    "description": "Feature name (for generate)"
                },
                "stories": {
                    "type": "array",
                    "description": "User stories (for generate)",
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": {"type": "string"},
                            "title": {"type": "string"},
                            "description": {"type": "string"},
                            "acceptance_criteria": {"type": "array", "items": {"type": "string"}},
                            "priority": {"type": "integer"},
                            "depends_on": {"type": "array", "items": {"type": "string"}},
                            "complexity": {"type": "integer"}
                        },
                        "required": ["id", "title", "description"]
                    }
                },
                "quality_checks": {
                    "type": "object",
                    "properties": {
                        "typecheck": {"type": "string"},
                        "test": {"type": "string"},
                        "lint": {"type": "string"},
                        "build": {"type": "string"}
                    }
                },
                "prd_path": {
                    "type": "string",
                    "description": "Path to save PRD (default: prd.json)"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "analyze" => {
                let task = p.task_description.unwrap_or_default();
                if task.is_empty() {
                    return Ok(ToolResult::error("task_description is required for analyze"));
                }

                let questions = format!(r#"# Task Analysis

## Task Description
{task}

## Questions to Answer

To generate a proper PRD for this task, please provide:

1. **Project Name**: What is the name of this project?

2. **Feature Name**: What specific feature or capability is being implemented?

3. **User Stories**: Break down the task into discrete user stories. For each:
   - ID (e.g., US-001)
   - Title (short description)
   - Description (detailed requirements)
   - Acceptance Criteria (how to verify it's done)
   - Priority (1=highest)
   - Dependencies (which stories must complete first)
   - Complexity (1-5)

4. **Quality Checks**: What commands verify the work?
   - Typecheck command (e.g., `cargo check`)
   - Test command (e.g., `cargo test`)
   - Lint command (e.g., `cargo clippy`)
   - Build command (e.g., `cargo build`)

## Example Response Format

```json
{{
  "project": "codetether",
  "feature": "LSP Integration", 
  "stories": [
    {{
      "id": "US-001",
      "title": "Add lsp-types dependency",
      "description": "Add lsp-types crate to Cargo.toml",
      "acceptance_criteria": ["Cargo.toml has lsp-types", "cargo check passes"],
      "priority": 1,
      "depends_on": [],
      "complexity": 1
    }},
    {{
      "id": "US-002",
      "title": "Implement LSP client",
      "description": "Create LSP client that can spawn language servers",
      "acceptance_criteria": ["Can spawn rust-analyzer", "Can send initialize request"],
      "priority": 2,
      "depends_on": ["US-001"],
      "complexity": 4
    }}
  ],
  "quality_checks": {{
    "typecheck": "cargo check",
    "test": "cargo test",
    "lint": "cargo clippy",
    "build": "cargo build --release"
  }}
}}
```

Once you have the answers, call `prd({{action: 'generate', ...}})` with the data.
"#);

                Ok(ToolResult::success(questions))
            }

            "generate" => {
                let project = p.project.unwrap_or_else(|| "Project".to_string());
                let feature = p.feature.unwrap_or_else(|| "Feature".to_string());
                
                let stories: Vec<UserStory> = p.stories.unwrap_or_default()
                    .into_iter()
                    .map(|s| UserStory {
                        id: s.id,
                        title: s.title,
                        description: s.description,
                        acceptance_criteria: s.acceptance_criteria,
                        passes: false,
                        priority: s.priority.unwrap_or(1),
                        depends_on: s.depends_on,
                        complexity: s.complexity.unwrap_or(3),
                    })
                    .collect();

                if stories.is_empty() {
                    return Ok(ToolResult::error("At least one story is required"));
                }

                let quality_checks = match p.quality_checks {
                    Some(qc) => QualityChecks {
                        typecheck: qc.typecheck,
                        test: qc.test,
                        lint: qc.lint,
                        build: qc.build,
                    },
                    None => QualityChecks::default(),
                };

                let prd = Prd {
                    project: project.clone(),
                    feature: feature.clone(),
                    branch_name: format!("feature/{}", feature.to_lowercase().replace(' ', "-")),
                    version: "1.0".to_string(),
                    user_stories: stories,
                    technical_requirements: Vec::new(),
                    quality_checks,
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                };

                let json = serde_json::to_string_pretty(&prd)?;
                
                Ok(ToolResult::success(format!(
                    "# Generated PRD\n\n```json\n{}\n```\n\nCall `prd({{action: 'save'}})` to write to file, then `ralph({{action: 'run'}})` to execute.",
                    json
                )).with_metadata("prd", serde_json::to_value(&prd)?))
            }

            "save" => {
                let project = p.project.unwrap_or_else(|| "Project".to_string());
                let feature = p.feature.unwrap_or_else(|| "Feature".to_string());
                let prd_path = PathBuf::from(p.prd_path.unwrap_or_else(|| "prd.json".to_string()));
                
                let stories: Vec<UserStory> = p.stories.unwrap_or_default()
                    .into_iter()
                    .map(|s| UserStory {
                        id: s.id,
                        title: s.title,
                        description: s.description,
                        acceptance_criteria: s.acceptance_criteria,
                        passes: false,
                        priority: s.priority.unwrap_or(1),
                        depends_on: s.depends_on,
                        complexity: s.complexity.unwrap_or(3),
                    })
                    .collect();

                if stories.is_empty() {
                    return Ok(ToolResult::error("At least one story is required for save"));
                }

                let quality_checks = match p.quality_checks {
                    Some(qc) => QualityChecks {
                        typecheck: qc.typecheck,
                        test: qc.test,
                        lint: qc.lint,
                        build: qc.build,
                    },
                    None => QualityChecks {
                        typecheck: Some("cargo check".to_string()),
                        test: Some("cargo test".to_string()),
                        lint: Some("cargo clippy".to_string()),
                        build: Some("cargo build".to_string()),
                    },
                };

                let prd = Prd {
                    project: project.clone(),
                    feature: feature.clone(),
                    branch_name: format!("feature/{}", feature.to_lowercase().replace(' ', "-")),
                    version: "1.0".to_string(),
                    user_stories: stories,
                    technical_requirements: Vec::new(),
                    quality_checks,
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                };

                prd.save(&prd_path).await.context("Failed to save PRD")?;

                Ok(ToolResult::success(format!(
                    "PRD saved to: {}\n\nRun with: ralph({{action: 'run', prd_path: '{}'}})",
                    prd_path.display(),
                    prd_path.display()
                )))
            }

            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Use 'analyze', 'generate', or 'save'",
                p.action
            ))),
        }
    }
}
