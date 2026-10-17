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

use crate::bus::AgentBus;
use crate::okr::{KrOutcome, KrOutcomeType, Okr, OkrRun, OkrRunStatus};
use crate::provider::{Provider, ProviderRegistry};
#[cfg(test)]
use crate::ralph::QualityChecks;
use crate::ralph::store_http::HttpStore;
use crate::ralph::{Prd, RalphConfig, RalphLoop, RalphStatus};

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

pub use codetether_go_prd::generate_prd_from_task;
mod registry;
/// Generate a PRD with the application's configured provider registry.
///
/// # Arguments
/// * `task` — Task description.
/// * `okr` — Objective whose key results inform the PRD.
/// * `primary_provider` — Provider used for the first and final attempts.
/// * `primary_model` — Caller-selected model name.
/// * `registry` — Optional configured registry for alternative model attempts.
///
/// # Returns
/// The first successfully generated PRD.
///
/// # Errors
/// Returns the final generation error when all attempts fail.
///
/// # Panics
/// Preserves the generator's byte-based repair excerpt behavior.
///
/// # Examples
/// See [codetether_go_prd::generate_prd_with_model_retry] for a generation example.
pub async fn generate_prd_with_model_retry(
    task: &str,
    okr: &Okr,
    primary_provider: Arc<dyn Provider>,
    primary_model: &str,
    registry: Option<&ProviderRegistry>,
) -> Result<Prd> {
    codetether_go_prd::generate_prd_with_model_retry(
        task,
        okr,
        primary_provider,
        primary_model,
        registry.map(|r| r as &dyn codetether_go_prd::Registry),
    )
    .await
}

/// Run Ralph loop for a /go task, mapping results back to OKR.
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
    // Step 1: Generate PRD from task + KRs (with automatic model retry)
    tracing::info!(task = %task, okr_id = %okr.id, "Generating PRD from task and key results");
    let registry_ref = registry.as_ref().map(|r| r.as_ref());
    let prd = generate_prd_with_model_retry(task, okr, Arc::clone(&provider), model, registry_ref)
        .await?;

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
        max_steps_per_story: 30,
        ..Default::default()
    };

    let mut ralph = RalphLoop::new(
        prd_path.clone(),
        Arc::clone(&provider),
        model.to_string(),
        config,
    )
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

    // Attach state store for persistent run tracking
    ralph = ralph.with_store(Arc::new(HttpStore::from_env()));

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
            "\nIncomplete stories:\n{}\n\nNext steps:\n  1. Review progress file for learnings\n  2. Re-run with a clean objective: codetether run -- '/go <concise-task>'\n  3. Or fix manually on branch `{}`",
            failed.join("\n"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okr::KeyResult;
    use crate::ralph::UserStory;
    use uuid::Uuid;

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
                    verification_steps: vec![],
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
                    verification_steps: vec![],
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
        assert!(
            run.outcomes[0]
                .evidence
                .iter()
                .any(|e| e.contains("US-001"))
        );
        assert!(
            run.outcomes[0]
                .evidence
                .iter()
                .any(|e| e.contains("PASSED"))
        );
        assert!(
            run.outcomes[0]
                .evidence
                .iter()
                .any(|e| e.contains("FAILED"))
        );
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
