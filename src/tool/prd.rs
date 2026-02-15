//! PRD Tool - Generate and validate PRD JSON
//!
//! This tool helps structure complex tasks into a PRD (Product Requirements
//! Document) that can be executed by `ralph`.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashSet;
use std::path::PathBuf;

use super::{Tool, ToolResult};
use crate::ralph::{Prd, QualityChecks, UserStory, VerificationStep};

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

    /// Raw PRD JSON string (optional alternative to `prd_path`).
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

    /// Optional explicit story verification (BDD/TDD/E2E evidence).
    #[serde(default)]
    verification_steps: Vec<VerificationStep>,

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

Actions:
- analyze: Analyze a task description and return what questions need answering
- generate: Generate a PRD JSON from provided answers
- validate: Validate a PRD (schema, dependencies, etc.)
- save: Save a PRD JSON to a file for ralph to execute
"#
    }

    fn parameters(&self) -> Value {
        // Keep `verification_steps` permissive; the enum is validated by serde when PRD JSON is parsed.
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
                    "description": "Project name (for generate/save)"
                },
                "feature": {
                    "type": "string",
                    "description": "Feature name (for generate/save)"
                },
                "stories": {
                    "type": "array",
                    "description": "User stories (for generate/validate/save)",
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": {"type": "string"},
                            "title": {"type": "string"},
                            "description": {"type": "string"},
                            "acceptance_criteria": {"type": "array", "items": {"type": "string"}},
                            "verification_steps": {"type": "array", "items": {"type": "object"}},
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
                    "description": "Path to PRD file (validate from file, or save destination)"
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
                    return Ok(ToolResult::structured_error(
                        "missing_field",
                        self.id(),
                        "task_description is required for analyze",
                        Some(vec!["task_description"]),
                        Some(json!({"action":"analyze","task_description":"..."})),
                    ));
                }

                const VERIFICATION_EXAMPLE_JSON: &str = r#"{
  \"verification_steps\": [
    {
      \"type\": \"shell\",
      \"name\": \"cypress_e2e\",
      \"command\": \"npx cypress run\",
      \"expect_files_glob\": [\"cypress/videos/**/*.mp4\"]
    },
    {
      \"type\": \"url\",
      \"name\": \"deployment_live\",
      \"url\": \"https://example.com/health\",
      \"expect_status\": 200,
      \"expect_body_contains\": [\"version:1.2.3\"],
      \"timeout_secs\": 30
    }
  ]
}"#;

                let mut questions = String::new();
                questions.push_str("# Task Analysis\n\n");
                questions.push_str("## Task Description\n");
                questions.push_str(&task);
                questions.push_str("\n\n");
                questions.push_str("## Questions to Answer\n\n");
                questions.push_str("1. **Project Name**\n");
                questions.push_str("2. **Feature Name**\n");
                questions.push_str(
                    "3. **User Stories**: break down the task into discrete, independently verifiable stories.\n",
                );
                questions.push_str("   For each story:\n");
                questions.push_str("   - ID (e.g., US-001)\n");
                questions.push_str("   - Title\n");
                questions.push_str("   - Description\n");
                questions.push_str("   - Acceptance Criteria\n");
                questions.push_str(
                    "   - Verification Steps (machine-verifiable evidence; BDD/TDD/E2E/artifacts/URLs)\n",
                );
                questions.push_str("   - Priority (1=highest)\n");
                questions.push_str("   - Dependencies\n");
                questions.push_str("   - Complexity (1-5)\n\n");
                questions.push_str("4. **Quality Checks** (repo-level gates; optional)\n\n");

                questions.push_str("### Example story verification steps\n\n");
                questions.push_str("```json\n");
                questions.push_str(VERIFICATION_EXAMPLE_JSON);
                questions.push_str("\n```\n");

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
                        verification_steps: s.verification_steps,
                        passes: false,
                        priority: s.priority.unwrap_or(1),
                        depends_on: s.depends_on,
                        complexity: s.complexity.unwrap_or(3),
                    })
                    .collect();

                if stories.is_empty() {
                    return Ok(ToolResult::structured_error(
                        "missing_field",
                        self.id(),
                        "At least one story is required",
                        Some(vec!["stories"]),
                        None,
                    ));
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

                let json_str = serde_json::to_string_pretty(&prd)?;
                Ok(ToolResult::success(format!(
                    "# Generated PRD\n\n```json\n{}\n```\n",
                    json_str
                ))
                .with_metadata("prd", serde_json::to_value(&prd)?))
            }

            "validate" => {
                let prd: Prd = if let Some(json_str) = p.prd_json {
                    serde_json::from_str(&json_str)
                        .context("Failed to parse prd_json - invalid JSON")?
                } else if let Some(path) = &p.prd_path {
                    let prd_path = PathBuf::from(path);
                    if !prd_path.exists() {
                        return Ok(ToolResult::error(format!("PRD file not found: {path}")));
                    }
                    let content = tokio::fs::read_to_string(&prd_path).await?;
                    serde_json::from_str(&content)
                        .context("Failed to parse PRD file - invalid JSON")?
                } else if let Some(stories) = p.stories {
                    let project = p.project.clone().unwrap_or_else(|| "Project".to_string());
                    let feature = p.feature.clone().unwrap_or_else(|| "Feature".to_string());

                    let stories: Vec<UserStory> = stories
                        .into_iter()
                        .map(|s| UserStory {
                            id: s.id,
                            title: s.title,
                            description: s.description,
                            acceptance_criteria: s.acceptance_criteria,
                            verification_steps: s.verification_steps,
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
                    return Ok(ToolResult::structured_error(
                        "missing_field",
                        self.id(),
                        "validate requires one of: prd_json, prd_path, or stories",
                        Some(vec!["prd_json|prd_path|stories"]),
                        None,
                    ));
                };

                let validation = validate_prd(&prd);

                if validation.is_valid {
                    Ok(ToolResult::success(format!(
                        "# PRD Validation: PASSED\n\nProject: {}\nFeature: {}\nStories: {}\nExecution stages: {}\n\n{}",
                        prd.project,
                        prd.feature,
                        prd.user_stories.len(),
                        prd.stages().len(),
                        validation.summary()
                    )))
                } else {
                    Ok(ToolResult::error(format!(
                        "# PRD Validation: FAILED\n\n{}",
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
                        verification_steps: s.verification_steps,
                        passes: false,
                        priority: s.priority.unwrap_or(1),
                        depends_on: s.depends_on,
                        complexity: s.complexity.unwrap_or(3),
                    })
                    .collect();

                if stories.is_empty() {
                    return Ok(ToolResult::structured_error(
                        "missing_field",
                        self.id(),
                        "At least one story is required for save",
                        Some(vec!["stories"]),
                        None,
                    ));
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

                prd.save(&prd_path).await.context("Failed to save PRD")?;
                Ok(ToolResult::success(format!(
                    "PRD saved to: {}",
                    prd_path.display()
                )))
            }

            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Use analyze/generate/validate/save",
                p.action
            ))),
        }
    }
}

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
                lines.push(format!("- ❌ {err}"));
            }
        }

        if !self.warnings.is_empty() {
            if !lines.is_empty() {
                lines.push(String::new());
            }
            lines.push("## Warnings".to_string());
            for warn in &self.warnings {
                lines.push(format!("- ⚠️ {warn}"));
            }
        }

        if self.is_valid && self.warnings.is_empty() {
            lines.push("✅ All checks passed".to_string());
        }

        lines.join("\n")
    }
}

fn validate_prd(prd: &Prd) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    if prd.project.is_empty() {
        errors.push("Missing required field: project".to_string());
    }
    if prd.feature.is_empty() {
        errors.push("Missing required field: feature".to_string());
    }
    if prd.user_stories.is_empty() {
        errors.push("PRD must have at least one user story".to_string());
    }

    let story_ids: HashSet<String> = prd.user_stories.iter().map(|s| s.id.clone()).collect();

    let mut seen_ids = HashSet::new();
    for story in &prd.user_stories {
        if seen_ids.contains(&story.id) {
            errors.push(format!("Duplicate story ID: {}", story.id));
        }
        seen_ids.insert(story.id.clone());

        if story.id.is_empty() {
            errors.push("Story has empty ID".to_string());
        }
        if story.title.is_empty() {
            errors.push(format!("Story {} has empty title", story.id));
        }
        if story.description.is_empty() {
            warnings.push(format!("Story {} has empty description", story.id));
        }
        if story.acceptance_criteria.is_empty() {
            warnings.push(format!("Story {} has no acceptance criteria", story.id));
        }
        if story.verification_steps.is_empty() {
            warnings.push(format!(
                "Story {} has no verification_steps; it may pass on quality checks alone",
                story.id
            ));
        }

        if story.priority == 0 {
            warnings.push(format!("Story {} has priority 0 (should be 1+)", story.id));
        }
        if story.complexity == 0 || story.complexity > 5 {
            warnings.push(format!(
                "Story {} has complexity {} (should be 1-5)",
                story.id, story.complexity
            ));
        }

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

    if let Some(cycle) = detect_cycle(prd) {
        errors.push(format!("Circular dependency detected: {}", cycle.join(" → ")));
    }

    let qc = &prd.quality_checks;
    if qc.typecheck.is_none() && qc.test.is_none() && qc.lint.is_none() && qc.build.is_none() {
        warnings.push("No quality checks defined - ralph won't run repo-level gates".to_string());
    }

    ValidationResult {
        is_valid: errors.is_empty(),
        errors,
        warnings,
    }
}

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
    let mut visiting = HashSet::new();
    let mut path = Vec::new();

    for story in &prd.user_stories {
        if let Some(cycle) = dfs(
            story.id.as_str(),
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
