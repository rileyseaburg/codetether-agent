//! PRD Tool - Generate PRD JSON from requirements via Q&A
//!
//! When a task is complex, this tool asks clarifying questions
//! to generate a structured PRD that can be used by ralph.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashSet;
use std::path::PathBuf;

use super::{Tool, ToolResult};
use crate::ralph::{Prd, QualityChecks, UserStory};

/// Tool for generating PRDs from requirements
pub struct PrdTool;

impl Default for PrdTool {
    fn default() -> Self {
        Self::new()
    }
}

impl PrdTool {
    pub fn new() -> Self {
        Self
    }
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
    #[serde(default)]
    prd_json: Option<String>,
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
    fn id(&self) -> &str {
        "prd"
    }
    fn name(&self) -> &str {
        "PRD Generator"
    }

    fn description(&self) -> &str {
        r#"Generate and validate structured PRDs (Product Requirements Documents) for complex tasks.

Use this tool when you recognize a task is complex and needs to be broken down into user stories.

Actions:
- analyze: Analyze a task description and return what questions need answering
- generate: Generate a PRD JSON from provided answers
- validate: Validate a PRD before ralph execution (checks schema, dependencies, etc.)
- save: Save a PRD to a file for ralph to execute

The workflow is:
1. Call analyze with the task_description to get questions
2. Answer the questions and call generate with your answers
3. Call validate to ensure the PRD is valid before saving
4. Call save to write the PRD to prd.json
5. Invoke ralph to execute the PRD
"#
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["analyze", "generate", "validate", "save"],
                    "description": "Action to perform"
                },
                "task_description": {
                    "type": "string",
                    "description": "Description of the complex task (for analyze)"
                },
                "project": {
                    "type": "string",
                    "description": "Project name (for generate/validate)"
                },
                "feature": {
                    "type": "string",
                    "description": "Feature name (for generate/validate)"
                },
                "stories": {
                    "type": "array",
                    "description": "User stories (for generate/validate)",
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
                    "description": "Path to PRD file (for validate from file, or save destination)"
                },
                "prd_json": {
                    "type": "string",
                    "description": "Raw PRD JSON string to validate (alternative to prd_path)"
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
                    return Ok(ToolResult::error(
                        "task_description is required for analyze",
                    ));
                }

                let questions = format!(
                    r#"# Task Analysis

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
"#
                );

                Ok(ToolResult::success(questions))
            }

            "generate" => {
                let project = p.project.unwrap_or_else(|| "Project".to_string());
                let feature = p.feature.unwrap_or_else(|| "Feature".to_string());

                let stories: Vec<UserStory> = p
                    .stories
                    .unwrap_or_default()
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
                    "# Generated PRD\n\n```json\n{}\n```\n\nCall `prd({{action: 'validate'}})` to check for errors, then `prd({{action: 'save'}})` to write to file.",
                    json
                )).with_metadata("prd", serde_json::to_value(&prd)?))
            }

            "validate" => {
                // Get PRD from: 1) prd_json string, 2) prd_path file, or 3) inline stories
                let prd: Prd = if let Some(json_str) = p.prd_json {
                    serde_json::from_str(&json_str)
                        .context("Failed to parse prd_json - invalid JSON")?
                } else if let Some(path) = &p.prd_path {
                    let prd_path = PathBuf::from(path);
                    if !prd_path.exists() {
                        return Ok(ToolResult::error(format!("PRD file not found: {}", path)));
                    }
                    let content = tokio::fs::read_to_string(&prd_path).await?;
                    serde_json::from_str(&content)
                        .context("Failed to parse PRD file - invalid JSON")?
                } else if p.stories.is_some() {
                    // Build from inline params
                    let project = p.project.clone().unwrap_or_else(|| "Project".to_string());
                    let feature = p.feature.clone().unwrap_or_else(|| "Feature".to_string());
                    let stories: Vec<UserStory> = p
                        .stories
                        .unwrap_or_default()
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
                    Prd {
                        project,
                        feature,
                        branch_name: String::new(),
                        version: "1.0".to_string(),
                        user_stories: stories,
                        technical_requirements: Vec::new(),
                        quality_checks: QualityChecks::default(),
                        created_at: String::new(),
                        updated_at: String::new(),
                    }
                } else {
                    return Ok(ToolResult::error(
                        "validate requires one of: prd_json, prd_path, or stories",
                    ));
                };

                // Run validation
                let validation = validate_prd(&prd);

                if validation.is_valid {
                    Ok(ToolResult::success(format!(
                        "# PRD Validation: PASSED\n\n\
                        Project: {}\n\
                        Feature: {}\n\
                        Stories: {}\n\
                        Execution stages: {}\n\n\
                        {}\n\n\
                        Ready for: `prd({{action: 'save'}})` then `ralph({{action: 'run'}})`",
                        prd.project,
                        prd.feature,
                        prd.user_stories.len(),
                        prd.stages().len(),
                        validation.summary()
                    )))
                } else {
                    Ok(ToolResult::error(format!(
                        "# PRD Validation: FAILED\n\n{}\n\n\
                        Fix these issues before saving.",
                        validation.summary()
                    )))
                }
            }

            "save" => {
                let project = p.project.unwrap_or_else(|| "Project".to_string());
                let feature = p.feature.unwrap_or_else(|| "Feature".to_string());
                let prd_path = PathBuf::from(p.prd_path.unwrap_or_else(|| "prd.json".to_string()));

                let stories: Vec<UserStory> = p
                    .stories
                    .unwrap_or_default()
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
                "Unknown action: {}. Use 'analyze', 'generate', 'validate', or 'save'",
                p.action
            ))),
        }
    }
}

/// Validation result for a PRD
struct ValidationResult {
    is_valid: bool,
    errors: Vec<String>,
    warnings: Vec<String>,
}

impl ValidationResult {
    fn summary(&self) -> String {
        let mut lines = Vec::new();

        if !self.errors.is_empty() {
            lines.push("## Errors".to_string());
            for err in &self.errors {
                lines.push(format!("- ❌ {}", err));
            }
        }

        if !self.warnings.is_empty() {
            if !lines.is_empty() {
                lines.push(String::new());
            }
            lines.push("## Warnings".to_string());
            for warn in &self.warnings {
                lines.push(format!("- ⚠️ {}", warn));
            }
        }

        if self.is_valid && self.warnings.is_empty() {
            lines.push("✅ All checks passed".to_string());
        }

        lines.join("\n")
    }
}

/// Validate a PRD before ralph execution
fn validate_prd(prd: &Prd) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // 1. Required fields
    if prd.project.is_empty() {
        errors.push("Missing required field: project".to_string());
    }
    if prd.feature.is_empty() {
        errors.push("Missing required field: feature".to_string());
    }
    if prd.user_stories.is_empty() {
        errors.push("PRD must have at least one user story".to_string());
    }

    // Collect all story IDs for reference checks
    let story_ids: HashSet<String> = prd.user_stories.iter().map(|s| s.id.clone()).collect();

    // 2. Story-level validation
    let mut seen_ids = HashSet::new();
    for story in &prd.user_stories {
        // Unique IDs
        if seen_ids.contains(&story.id) {
            errors.push(format!("Duplicate story ID: {}", story.id));
        }
        seen_ids.insert(story.id.clone());

        // ID format
        if story.id.is_empty() {
            errors.push("Story has empty ID".to_string());
        }

        // Title required
        if story.title.is_empty() {
            errors.push(format!("Story {} has empty title", story.id));
        }

        // Description required
        if story.description.is_empty() {
            warnings.push(format!("Story {} has empty description", story.id));
        }

        // Acceptance criteria
        if story.acceptance_criteria.is_empty() {
            warnings.push(format!("Story {} has no acceptance criteria", story.id));
        }

        // Priority range
        if story.priority == 0 {
            warnings.push(format!("Story {} has priority 0 (should be 1+)", story.id));
        }

        // Complexity range
        if story.complexity == 0 || story.complexity > 5 {
            warnings.push(format!(
                "Story {} has complexity {} (should be 1-5)",
                story.id, story.complexity
            ));
        }

        // Dependencies reference valid stories
        for dep in &story.depends_on {
            if !story_ids.contains(dep) {
                errors.push(format!(
                    "Story {} depends on non-existent story: {}",
                    story.id, dep
                ));
            }
            if dep == &story.id {
                errors.push(format!("Story {} depends on itself", story.id));
            }
        }
    }

    // 3. Check for circular dependencies
    if let Some(cycle) = detect_cycle(prd) {
        errors.push(format!(
            "Circular dependency detected: {}",
            cycle.join(" → ")
        ));
    }

    // 4. Quality checks
    let qc = &prd.quality_checks;
    if qc.typecheck.is_none() && qc.test.is_none() && qc.lint.is_none() && qc.build.is_none() {
        warnings.push("No quality checks defined - ralph won't verify work".to_string());
    }

    // 5. Branch name
    if prd.branch_name.is_empty() {
        warnings.push("No branch_name specified - will auto-generate from feature".to_string());
    }

    ValidationResult {
        is_valid: errors.is_empty(),
        errors,
        warnings,
    }
}

/// Detect circular dependencies using DFS
fn detect_cycle(prd: &Prd) -> Option<Vec<String>> {
    let story_map: std::collections::HashMap<&str, &UserStory> = prd
        .user_stories
        .iter()
        .map(|s| (s.id.as_str(), s))
        .collect();

    fn dfs<'a>(
        id: &'a str,
        story_map: &std::collections::HashMap<&str, &'a UserStory>,
        visiting: &mut HashSet<&'a str>,
        visited: &mut HashSet<&'a str>,
        path: &mut Vec<&'a str>,
    ) -> Option<Vec<String>> {
        if visiting.contains(id) {
            // Found cycle - extract it from path
            let start_idx = path.iter().position(|&x| x == id).unwrap_or(0);
            let mut cycle: Vec<String> = path[start_idx..].iter().map(|s| s.to_string()).collect();
            cycle.push(id.to_string());
            return Some(cycle);
        }

        if visited.contains(id) {
            return None;
        }

        visiting.insert(id);
        path.push(id);

        if let Some(story) = story_map.get(id) {
            for dep in &story.depends_on {
                if let Some(cycle) = dfs(dep.as_str(), story_map, visiting, visited, path) {
                    return Some(cycle);
                }
            }
        }

        path.pop();
        visiting.remove(id);
        visited.insert(id);

        None
    }

    let mut visited = HashSet::new();

    for story in &prd.user_stories {
        let mut visiting = HashSet::new();
        let mut path = Vec::new();
        if let Some(cycle) = dfs(
            &story.id,
            &story_map,
            &mut visiting,
            &mut visited,
            &mut path,
        ) {
            return Some(cycle);
        }
    }

    None
}
