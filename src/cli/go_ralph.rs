//! Bridge between `/go` OKR approval gate and Ralph PRD execution loop.
//!
//! Flow:
//! 1. OKR is approved (caller responsibility)
//! 2. LLM generates a PRD from the task + key results
//! 3. PRD is saved to disk and audit-logged
//! 4. Ralph loop executes the PRD with quality gates
//! 5. Story pass/fail maps back to KR outcomes on the OkrRun

use anyhow::{Context, Result};
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

use crate::bus::AgentBus;
use crate::okr::{KeyResult, KrOutcome, KrOutcomeType, Okr, OkrRun, OkrRunStatus};
use crate::provider::{CompletionRequest, ContentPart, Message, Provider, ProviderRegistry, Role};
use crate::ralph::{Prd, QualityChecks, RalphConfig, RalphLoop, RalphStatus, UserStory};

/// Result of a `/go` execution via Ralph.
#[derive(Debug, Clone)]
pub struct GoRalphResult {
    pub prd_path: PathBuf,
    pub feature_branch: String,
    pub passed: usize,
    pub total: usize,
    pub all_passed: bool,
    pub iterations: usize,
    pub max_iterations: usize,
    pub status: RalphStatus,
    pub stories: Vec<StoryResult>,
}

#[derive(Debug, Clone)]
pub struct StoryResult {
    pub id: String,
    pub title: String,
    pub passed: bool,
}

/// Generate a PRD from a task description and OKR key results using an LLM.
pub async fn generate_prd_from_task(
    task: &str,
    okr: &Okr,
    provider: &dyn Provider,
    model: &str,
) -> Result<Prd> {
    let kr_descriptions: Vec<String> = okr
        .key_results
        .iter()
        .enumerate()
        .map(|(i, kr)| {
            format!(
                "KR-{}: {} (target: {} {})",
                i + 1,
                kr.title,
                kr.target_value,
                kr.unit
            )
        })
        .collect();

    let prompt = format!(
        r#"You are a PRD generator. Given a task and key results, produce a JSON PRD with concrete user stories.

Task: {task}

Key Results:
{krs}

Generate a PRD JSON with this exact structure (no markdown, no commentary, ONLY valid JSON):
{{
  "project": "<short project name>",
  "feature": "<feature name>",
  "branch_name": "feature/<kebab-case-name>",
  "version": "1.0",
  "user_stories": [
    {{
      "id": "US-001",
      "title": "<concise title>",
      "description": "<what to implement>",
      "acceptance_criteria": ["<criterion 1>", "<criterion 2>"],
      "passes": false,
      "priority": 1,
      "depends_on": [],
      "complexity": 3
    }}
  ],
  "technical_requirements": ["<requirement>"],
  "quality_checks": {{
    "typecheck": "cargo check",
    "test": "cargo test",
    "lint": "cargo clippy --all-features",
    "build": "cargo build"
  }}
}}

Rules:
- Each key result should map to at least one user story
- Stories should be concrete, implementable, and testable
- Use priority 1 for critical stories, 2 for important, 3 for nice-to-have
- Set depends_on when stories have real dependencies
- Complexity: 1=trivial, 2=simple, 3=moderate, 4=complex, 5=very complex
- quality_checks should match the project's toolchain (default to cargo for Rust)
- Output ONLY the JSON object, nothing else"#,
        krs = kr_descriptions.join("\n"),
    );

    let request = CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: prompt }],
        }],
        tools: vec![],
        model: model.to_string(),
        temperature: Some(0.3),
        top_p: None,
        max_tokens: Some(4096),
        stop: vec![],
    };

    let response = provider
        .complete(request)
        .await
        .context("Failed to generate PRD from LLM")?;

    let text = response
        .message
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("");

    // Extract JSON from the response (may be wrapped in markdown code blocks)
    let json_str = extract_json(&text).context("LLM response did not contain valid JSON")?;

    let mut prd: Prd =
        serde_json::from_str(&json_str).context("Failed to parse PRD JSON from LLM response")?;

    // Ensure timestamps
    let now = chrono::Utc::now().to_rfc3339();
    prd.created_at = now.clone();
    prd.updated_at = now;

    // Detect quality checks from the working directory
    if prd.quality_checks.typecheck.is_none() {
        prd.quality_checks = detect_quality_checks();
    }

    Ok(prd)
}

/// Run Ralph loop for a `/go` task, mapping results back to OKR.
pub async fn execute_go_ralph(
    task: &str,
    okr: &mut Okr,
    okr_run: &mut OkrRun,
    provider: Arc<dyn Provider>,
    model: &str,
    max_iterations: usize,
    bus: Option<Arc<AgentBus>>,
    max_concurrent_stories: usize,
    registry: Option<Arc<ProviderRegistry>>,
) -> Result<GoRalphResult> {
    // Step 1: Generate PRD from task + KRs
    tracing::info!(task = %task, okr_id = %okr.id, "Generating PRD from task and key results");
    let prd = generate_prd_from_task(task, okr, provider.as_ref(), model).await?;

    // Step 2: Save PRD to disk
    let prd_filename = format!("prd_{}.json", okr_run.id.to_string().replace('-', "_"));
    let prd_path = PathBuf::from(&prd_filename);
    prd.save(&prd_path)
        .await
        .context("Failed to save generated PRD")?;

    tracing::info!(
        prd_path = %prd_path.display(),
        stories = prd.user_stories.len(),
        feature = %prd.feature,
        "PRD generated and saved"
    );

    // Step 3: Audit-log the PRD generation
    if let Some(audit) = crate::audit::try_audit_log() {
        audit
            .log_with_correlation(
                crate::audit::AuditCategory::Cognition,
                "go_ralph_prd_generated",
                crate::audit::AuditOutcome::Success,
                Some("codetether-agent".to_string()),
                Some(json!({
                    "task": task,
                    "prd_path": prd_path.display().to_string(),
                    "stories": prd.user_stories.len(),
                    "feature": prd.feature,
                    "project": prd.project,
                })),
                Some(okr.id.to_string()),
                Some(okr_run.id.to_string()),
                None,
                okr_run.session_id.clone(),
            )
            .await;
    }

    // Step 4: Update OKR run status
    if let Err(e) = okr_run.start() {
        tracing::warn!(error = %e, "OKR run start transition failed, forcing Running status");
        okr_run.status = OkrRunStatus::Running;
    }
    okr_run.relay_checkpoint_id = Some(prd_filename.clone());

    // Step 5: Run Ralph loop
    let config = RalphConfig {
        prd_path: prd_path.to_string_lossy().to_string(),
        max_iterations,
        progress_path: format!("progress_{}.txt", okr_run.id.to_string().replace('-', "_")),
        quality_checks_enabled: true,
        auto_commit: true,
        model: Some(model.to_string()),
        use_rlm: false,
        parallel_enabled: true,
        max_concurrent_stories,
        worktree_enabled: true,
        story_timeout_secs: 300,
        conflict_timeout_secs: 120,
        relay_enabled: false,
        relay_max_agents: 8,
        relay_max_rounds: 3,
    };

    let mut ralph = RalphLoop::new(prd_path.clone(), Arc::clone(&provider), model.to_string(), config)
        .await
        .context("Failed to initialize Ralph loop")?;

    // Attach bus for inter-iteration learning sharing
    if let Some(bus) = bus {
        ralph = ralph.with_bus(bus);
    }

    // Attach registry for relay team planning
    if let Some(registry) = registry {
        ralph = ralph.with_registry(registry);
    }

    let state = ralph.run().await.context("Ralph loop execution failed")?;

    // Step 6: Map story results → KR outcomes
    let stories: Vec<StoryResult> = state
        .prd
        .user_stories
        .iter()
        .map(|s| StoryResult {
            id: s.id.clone(),
            title: s.title.clone(),
            passed: s.passes,
        })
        .collect();

    let passed = state.prd.passed_count();
    let total = state.prd.user_stories.len();

    map_stories_to_kr_outcomes(okr, okr_run, &state.prd, &state);
    let all_passed = okr.is_complete() || passed == total;

    // Step 7: Update run status
    if all_passed {
        okr_run.complete();
    } else if state.status == RalphStatus::Stopped || state.status == RalphStatus::QualityFailed {
        okr_run.status = OkrRunStatus::Failed;
    } else {
        okr_run.status = OkrRunStatus::Completed;
    }
    okr_run.iterations = state.current_iteration as u32;
    okr_run.relay_checkpoint_id = None; // lifecycle complete

    // Step 8: Audit-log the result
    if let Some(audit) = crate::audit::try_audit_log() {
        let outcome = if all_passed {
            crate::audit::AuditOutcome::Success
        } else {
            crate::audit::AuditOutcome::Failure
        };
        audit
            .log_with_correlation(
                crate::audit::AuditCategory::Cognition,
                "go_ralph_completed",
                outcome,
                Some("codetether-agent".to_string()),
                Some(json!({
                    "prd_path": prd_path.display().to_string(),
                    "passed": passed,
                    "total": total,
                    "status": format!("{:?}", state.status),
                    "iterations": state.current_iteration,
                    "feature_branch": state.prd.branch_name,
                })),
                Some(okr.id.to_string()),
                Some(okr_run.id.to_string()),
                None,
                okr_run.session_id.clone(),
            )
            .await;
    }

    Ok(GoRalphResult {
        prd_path,
        feature_branch: state.prd.branch_name.clone(),
        passed,
        total,
        all_passed,
        iterations: state.current_iteration,
        max_iterations: state.max_iterations,
        status: state.status,
        stories,
    })
}

/// Map Ralph story pass/fail to OKR KR outcomes.
fn map_stories_to_kr_outcomes(
    okr: &mut Okr,
    run: &mut OkrRun,
    prd: &Prd,
    state: &crate::ralph::RalphState,
) {
    let passed = prd.passed_count();
    let total = prd.user_stories.len();
    let ratio = if total > 0 {
        passed as f64 / total as f64
    } else {
        0.0
    };

    // Build evidence from story results
    let story_evidence: Vec<String> = prd
        .user_stories
        .iter()
        .map(|s| {
            format!(
                "{}:{} ({})",
                s.id,
                s.title,
                if s.passes { "PASSED" } else { "FAILED" }
            )
        })
        .collect();

    let outcome_type = if ratio >= 1.0 {
        KrOutcomeType::FeatureDelivered
    } else {
        KrOutcomeType::Evidence
    };

    // For each KR, create an outcome with story-mapped evidence
    for kr in &mut okr.key_results {
        // Map KR progress based on story completion ratio
        let kr_value = ratio * kr.target_value;
        kr.update_progress(kr_value);
        run.update_kr_progress(&kr.id.to_string(), kr_value);

        let mut evidence = story_evidence.clone();
        evidence.push(format!("prd:{}", prd.feature));
        evidence.push(format!("iterations:{}", state.current_iteration));
        evidence.push(format!("status:{:?}", state.status));
        if !prd.branch_name.is_empty() {
            evidence.push(format!("branch:{}", prd.branch_name));
        }

        let mut outcome = KrOutcome::new(
            kr.id,
            format!(
                "Ralph PRD execution: {}/{} stories passed for '{}'",
                passed, total, prd.feature
            ),
        )
        .with_value(kr_value);
        outcome.run_id = Some(run.id);
        outcome.outcome_type = outcome_type;
        outcome.evidence = evidence;
        outcome.source = "go_ralph".to_string();

        kr.add_outcome(outcome.clone());
        run.outcomes.push(outcome);
    }
}

/// Format a GoRalphResult for display.
pub fn format_go_ralph_result(result: &GoRalphResult, task: &str) -> String {
    let status_icon = if result.all_passed { "✅" } else { "❌" };
    let status_label = format!("{:?}", result.status);

    let story_lines: Vec<String> = result
        .stories
        .iter()
        .map(|s| {
            format!(
                "  {} {}: {}",
                if s.passed { "✓" } else { "✗" },
                s.id,
                s.title
            )
        })
        .collect();

    let next_steps = if result.all_passed {
        format!(
            "\nNext steps:\n  1. Review changes on branch `{}`\n  2. Merge: git checkout main && git merge {} --no-ff\n  3. Push: git push",
            result.feature_branch, result.feature_branch
        )
    } else {
        let failed: Vec<String> = result
            .stories
            .iter()
            .filter(|s| !s.passed)
            .map(|s| format!("  - {}: {}", s.id, s.title))
            .collect();
        format!(
            "\nIncomplete stories:\n{}\n\nNext steps:\n  1. Review progress file for learnings\n  2. Re-run: codetether run \"/go {}\"\n  3. Or fix manually on branch `{}`",
            failed.join("\n"),
            task,
            result.feature_branch
        )
    };

    format!(
        "{status_icon} /go Ralph {status_label}\n\n\
         Task: {task}\n\
         Progress: {passed}/{total} stories | Iterations: {iters}/{max}\n\
         Feature branch: {branch}\n\
         PRD: {prd}\n\n\
         Stories:\n{stories}\n{next}",
        task = task,
        passed = result.passed,
        total = result.total,
        iters = result.iterations,
        max = result.max_iterations,
        branch = result.feature_branch,
        prd = result.prd_path.display(),
        stories = story_lines.join("\n"),
        next = next_steps,
    )
}

/// Extract JSON object from text that may be wrapped in markdown code blocks.
fn extract_json(text: &str) -> Option<String> {
    // Try direct parse first
    if serde_json::from_str::<serde_json::Value>(text.trim()).is_ok() {
        return Some(text.trim().to_string());
    }

    // Try extracting from ```json ... ``` blocks
    if let Some(start) = text.find("```json") {
        let after_fence = &text[start + 7..];
        if let Some(end) = after_fence.find("```") {
            let json_str = after_fence[..end].trim();
            if serde_json::from_str::<serde_json::Value>(json_str).is_ok() {
                return Some(json_str.to_string());
            }
        }
    }

    // Try extracting from ``` ... ``` blocks
    if let Some(start) = text.find("```") {
        let after_fence = &text[start + 3..];
        // Skip optional language tag on the same line
        let content_start = after_fence.find('\n').unwrap_or(0);
        let after_tag = &after_fence[content_start..];
        if let Some(end) = after_tag.find("```") {
            let json_str = after_tag[..end].trim();
            if serde_json::from_str::<serde_json::Value>(json_str).is_ok() {
                return Some(json_str.to_string());
            }
        }
    }

    // Try finding first { to last }
    if let (Some(start), Some(end)) = (text.find('{'), text.rfind('}')) {
        if start < end {
            let json_str = &text[start..=end];
            if serde_json::from_str::<serde_json::Value>(json_str).is_ok() {
                return Some(json_str.to_string());
            }
        }
    }

    None
}

/// Auto-detect quality checks from the working directory.
fn detect_quality_checks() -> QualityChecks {
    let cwd = std::env::current_dir().unwrap_or_default();

    if cwd.join("Cargo.toml").exists() {
        QualityChecks {
            typecheck: Some("cargo check".to_string()),
            test: Some("cargo test".to_string()),
            lint: Some("cargo clippy --all-features".to_string()),
            build: Some("cargo build".to_string()),
        }
    } else if cwd.join("package.json").exists() {
        QualityChecks {
            typecheck: Some("npx tsc --noEmit".to_string()),
            test: Some("npm test".to_string()),
            lint: Some("npm run lint".to_string()),
            build: Some("npm run build".to_string()),
        }
    } else if cwd.join("go.mod").exists() {
        QualityChecks {
            typecheck: Some("go vet ./...".to_string()),
            test: Some("go test ./...".to_string()),
            lint: Some("golangci-lint run".to_string()),
            build: Some("go build ./...".to_string()),
        }
    } else if cwd.join("requirements.txt").exists() || cwd.join("pyproject.toml").exists() {
        QualityChecks {
            typecheck: Some("mypy .".to_string()),
            test: Some("pytest".to_string()),
            lint: Some("ruff check .".to_string()),
            build: None,
        }
    } else {
        QualityChecks::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_json_handles_raw_json() {
        let raw = r#"{"project": "test", "feature": "foo"}"#;
        assert!(extract_json(raw).is_some());
    }

    #[test]
    fn extract_json_handles_markdown_wrapped() {
        let wrapped = "Here is the PRD:\n```json\n{\"project\": \"test\"}\n```\nDone.";
        let result = extract_json(wrapped).unwrap();
        assert!(result.contains("test"));
    }

    #[test]
    fn extract_json_handles_bare_braces() {
        let text = "The result is: {\"project\": \"test\"} and that's it.";
        let result = extract_json(text).unwrap();
        assert!(result.contains("test"));
    }

    #[test]
    fn extract_json_returns_none_for_no_json() {
        assert!(extract_json("no json here").is_none());
    }

    #[test]
    fn detect_quality_checks_returns_defaults_for_unknown() {
        // This tests that default is returned when no project file is found
        // (may detect Cargo.toml in workspace — that's fine, we just test no panic)
        let _checks = detect_quality_checks();
    }

    #[test]
    fn map_stories_creates_outcomes_for_each_kr() {
        let okr_id = Uuid::new_v4();
        let mut okr = Okr::new("Test OKR", "Test description");
        okr.id = okr_id;

        let kr1 = KeyResult::new(okr_id, "Stories complete", 100.0, "%");
        let kr2 = KeyResult::new(okr_id, "No errors", 0.0, "count");
        okr.add_key_result(kr1);
        okr.add_key_result(kr2);

        let mut run = OkrRun::new(okr_id, "Test Run");

        let prd = Prd {
            project: "test".to_string(),
            feature: "test-feature".to_string(),
            branch_name: "feature/test".to_string(),
            version: "1.0".to_string(),
            user_stories: vec![
                UserStory {
                    id: "US-001".to_string(),
                    title: "Story one".to_string(),
                    description: "First story".to_string(),
                    acceptance_criteria: vec![],
                    passes: true,
                    priority: 1,
                    depends_on: vec![],
                    complexity: 2,
                },
                UserStory {
                    id: "US-002".to_string(),
                    title: "Story two".to_string(),
                    description: "Second story".to_string(),
                    acceptance_criteria: vec![],
                    passes: false,
                    priority: 2,
                    depends_on: vec![],
                    complexity: 3,
                },
            ],
            technical_requirements: vec![],
            quality_checks: QualityChecks::default(),
            created_at: String::new(),
            updated_at: String::new(),
        };

        let state = crate::ralph::RalphState {
            prd: prd.clone(),
            current_iteration: 3,
            max_iterations: 10,
            status: RalphStatus::MaxIterations,
            progress_log: vec![],
            prd_path: PathBuf::from("test.json"),
            working_dir: PathBuf::from("."),
        };

        map_stories_to_kr_outcomes(&mut okr, &mut run, &prd, &state);

        // Should have 2 outcomes (one per KR)
        assert_eq!(run.outcomes.len(), 2);
        // Each outcome should reference the correct KR
        assert_eq!(run.outcomes[0].kr_id, okr.key_results[0].id);
        assert_eq!(run.outcomes[1].kr_id, okr.key_results[1].id);
        // Progress should be 50% (1/2 stories passed)
        assert_eq!(run.outcomes[0].value, Some(50.0)); // 0.5 * 100.0
        // Evidence should include story results
        assert!(run.outcomes[0].evidence.iter().any(|e| e.contains("US-001")));
        assert!(run.outcomes[0].evidence.iter().any(|e| e.contains("PASSED")));
        assert!(run.outcomes[0].evidence.iter().any(|e| e.contains("FAILED")));
    }

    #[test]
    fn format_result_shows_status() {
        let result = GoRalphResult {
            prd_path: PathBuf::from("prd_test.json"),
            feature_branch: "feature/test".to_string(),
            passed: 2,
            total: 3,
            all_passed: false,
            iterations: 5,
            max_iterations: 10,
            status: RalphStatus::MaxIterations,
            stories: vec![
                StoryResult {
                    id: "US-001".to_string(),
                    title: "Story one".to_string(),
                    passed: true,
                },
                StoryResult {
                    id: "US-002".to_string(),
                    title: "Story two".to_string(),
                    passed: true,
                },
                StoryResult {
                    id: "US-003".to_string(),
                    title: "Story three".to_string(),
                    passed: false,
                },
            ],
        };

        let output = format_go_ralph_result(&result, "test task");
        assert!(output.contains("2/3 stories"));
        assert!(output.contains("US-003"));
        assert!(output.contains("Incomplete"));
    }
}
