//! Application execution and event adapters for forage work selection.
use codetether_forage_core::changes::snapshot_git_changed_files;
#[cfg(test)]
use codetether_forage_core::progress::success_progress_increment_ratio;
use codetether_forage_core::seed::{
    seed_initial_okr_if_empty, seed_moonshot_okr_if_no_opportunities,
};

#[cfg(test)]
use codetether_forage_core::seed::seed_default_okr_if_empty;
use codetether_forage_core::{ForageOpportunity, MoonshotRubric};
pub use codetether_forage_core::{ForageRunSummary, ForageSelectionSummary};
#[cfg(test)]
use codetether_forage_core::{normalize_prompt_field, status_weight, urgency_bonus};

use crate::a2a::types::{Part, TaskState};
use crate::audit::{self, AuditCategory, AuditLog, AuditOutcome};
use crate::bus::{AgentBus, BusHandle, BusMessage};
use crate::cli::ForageArgs;
use crate::okr::{Okr, OkrRepository};
use crate::swarm::{DecompositionStrategy, ExecutionMode, SwarmConfig, SwarmExecutor};
use run_exec::execute_opportunity_with_run;
mod s3_health;
use s3_health::ensure_s3_sync_alive;
mod progress_adapter;
use progress_adapter::record_execution_success_to_okr;
mod go_exec;
use go_exec::execute_opportunity_with_go;
mod discover;
mod events;
mod execute_selected;
mod execution;
mod execution_success;
mod startup;
pub use startup::{execute, execute_with_summary};
mod runtime;
mod start_sink;
use execution::execute_opportunity;
mod swarm_execution;
use swarm_execution::execute_opportunity_with_swarm;
mod swarm_config;
use swarm_config::{build_swarm_config, parse_swarm_strategy};
mod audit_adapter;
use audit_adapter::{ensure_audit_log_initialized, log_audit};
mod observer;
use observer::flush_bus_observer;
mod opportunities;
use opportunities::{build_opportunities, load_moonshot_rubric};
#[cfg(test)]
use opportunities::{collect_opportunities, collect_opportunities_with_rubric};
mod clarify;
pub mod console;
mod console_entry;
mod quality_shell;
mod run_exec;
mod s3_optional;
mod tetherscript_score;
use crate::forage_println;
use anyhow::Result;
use chrono::Utc;
use codetether_forage_core::execution::ExecutionOutcome;
use serde::Serialize;
use serde_json::json;
use std::time::Duration;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::{
        ExecutionOutcome, ForageOpportunity, MoonshotRubric, build_swarm_config,
        collect_opportunities, collect_opportunities_with_rubric, normalize_prompt_field,
        record_execution_success_to_okr, seed_default_okr_if_empty,
        seed_moonshot_okr_if_no_opportunities, status_weight, success_progress_increment_ratio,
        urgency_bonus,
    };
    use crate::cli::ForageArgs;
    use crate::okr::{KeyResult, Okr, OkrStatus};
    use chrono::{Duration, Utc};
    use tempfile::tempdir;
    use uuid::Uuid;

    #[test]
    fn status_weight_prioritizes_active_okrs() {
        assert!(status_weight(OkrStatus::Active) > status_weight(OkrStatus::Draft));
        assert!(status_weight(OkrStatus::Draft) > status_weight(OkrStatus::OnHold));
        assert_eq!(status_weight(OkrStatus::Completed), 0.0);
    }

    #[test]
    fn urgency_bonus_increases_for_due_dates() {
        let now = Utc::now();
        let overdue = urgency_bonus(Some(now - Duration::days(1)), now);
        let soon = urgency_bonus(Some(now + Duration::days(3)), now);
        let later = urgency_bonus(Some(now + Duration::days(45)), now);
        assert!(overdue > soon);
        assert!(soon > later);
    }

    #[test]
    fn collect_opportunities_skips_complete_or_cancelled_work() {
        let mut okr = Okr::new("Ship growth loop", "Increase retained users");
        okr.status = OkrStatus::Cancelled;
        let mut kr = KeyResult::new(okr.id, "Retained users", 100.0, "%");
        kr.update_progress(10.0);
        okr.add_key_result(kr);

        let items = collect_opportunities(&[okr]);
        assert!(items.is_empty());
    }

    #[test]
    fn collect_opportunities_ranks_remaining_work() {
        let mut okr = Okr::new("Ship growth loop", "Increase retained users");
        okr.status = OkrStatus::Active;

        let mut kr_low = KeyResult::new(okr.id, "KR Low Remaining", 100.0, "%");
        kr_low.update_progress(80.0);
        let mut kr_high = KeyResult::new(okr.id, "KR High Remaining", 100.0, "%");
        kr_high.update_progress(10.0);
        okr.add_key_result(kr_low);
        okr.add_key_result(kr_high);

        let items = collect_opportunities(&[okr]);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].key_result_title, "KR High Remaining");
    }

    #[test]
    fn moonshot_rubric_filters_low_alignment_work() {
        let mut okr = Okr::new(
            "Improve parser latency",
            "Reduce p95 latency for parser pipeline",
        );
        okr.status = OkrStatus::Active;
        let mut kr = KeyResult::new(okr.id, "Parser p95 under 50ms", 100.0, "%");
        kr.update_progress(10.0);
        okr.add_key_result(kr);

        let rubric = MoonshotRubric {
            goals: vec!["eliminate billing fraud globally".to_string()],
            required: true,
            min_alignment: 0.4,
        };

        let items = collect_opportunities_with_rubric(&[okr], &rubric);
        assert!(
            items.is_empty(),
            "non-aligned work should be filtered out when moonshot is required"
        );
    }

    #[test]
    fn forage_timeout_bounds_are_clamped() {
        let low = 5u64.clamp(30, 86_400);
        let high = 999_999u64.clamp(30, 86_400);
        assert_eq!(low, 30);
        assert_eq!(high, 86_400);
    }

    #[test]
    fn swarm_config_does_not_force_legacy_model_fallback() {
        let args = ForageArgs {
            top: 3,
            loop_mode: false,
            interval_secs: 120,
            max_cycles: 1,
            execute: true,
            moonshots: Vec::new(),
            moonshot_file: None,
            moonshot_required: false,
            moonshot_min_alignment: 0.10,
            execution_engine: "swarm".to_string(),
            run_timeout_secs: 900,
            fail_fast: false,
            swarm_strategy: "auto".to_string(),
            swarm_max_subagents: 8,
            swarm_max_steps: 100,
            swarm_subagent_timeout_secs: 300,
            model: None,
            json: false,
            no_s3: false,
        };

        let config = build_swarm_config(&args);
        assert!(config.model.is_none());
    }

    #[test]
    fn swarm_config_preserves_explicit_model_override() {
        let args = ForageArgs {
            top: 3,
            loop_mode: false,
            interval_secs: 120,
            max_cycles: 1,
            execute: true,
            moonshots: Vec::new(),
            moonshot_file: None,
            moonshot_required: false,
            moonshot_min_alignment: 0.10,
            execution_engine: "swarm".to_string(),
            run_timeout_secs: 900,
            fail_fast: false,
            swarm_strategy: "auto".to_string(),
            swarm_max_subagents: 8,
            swarm_max_steps: 100,
            swarm_subagent_timeout_secs: 300,
            model: Some("openai-codex/gpt-5-mini".to_string()),
            json: false,
            no_s3: false,
        };

        let config = build_swarm_config(&args);
        assert_eq!(config.model.as_deref(), Some("openai-codex/gpt-5-mini"));
    }

    #[test]
    fn normalize_prompt_field_compacts_and_truncates() {
        let input = "alpha   beta\n\n gamma    delta";
        assert_eq!(normalize_prompt_field(input, 128), "alpha beta gamma delta");

        let long = "x".repeat(400);
        let normalized = normalize_prompt_field(&long, 64);
        assert!(normalized.ends_with("...(truncated)"));
        assert!(normalized.len() > 64);
    }

    #[test]
    fn success_progress_increment_ratio_is_bounded() {
        let mut kr = KeyResult::new(Uuid::new_v4(), "KR", 100.0, "%");
        kr.update_progress(0.0);
        let high_remaining = success_progress_increment_ratio(&kr);
        assert!((high_remaining - 0.15).abs() < f64::EPSILON);

        kr.update_progress(95.0);
        let low_remaining = success_progress_increment_ratio(&kr);
        assert!((low_remaining - 0.05).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn record_execution_success_updates_kr_progress_and_evidence() {
        let dir = tempdir().expect("create tempdir");
        let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

        let mut okr = Okr::new("Autonomous Business-Aligned Execution", "Test objective");
        okr.status = OkrStatus::Active;
        let mut kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
        kr.update_progress(0.0);
        let kr_id = kr.id;
        okr.add_key_result(kr);
        let okr_id = okr.id;
        let _ = repo.create_okr(okr).await.expect("create okr");

        let item = ForageOpportunity {
            score: 1.35,
            okr_id,
            okr_title: "Autonomous Business-Aligned Execution".to_string(),
            okr_status: OkrStatus::Active,
            key_result_id: kr_id,
            key_result_title: "KR1".to_string(),
            progress: 0.0,
            remaining: 1.0,
            target_date: None,
            moonshot_alignment: 0.0,
            moonshot_hits: Vec::new(),
            prompt: "test prompt".to_string(),
        };
        let args = ForageArgs {
            top: 3,
            loop_mode: false,
            interval_secs: 120,
            max_cycles: 1,
            execute: true,
            moonshots: Vec::new(),
            moonshot_file: None,
            moonshot_required: false,
            moonshot_min_alignment: 0.10,
            execution_engine: "run".to_string(),
            run_timeout_secs: 900,
            fail_fast: false,
            swarm_strategy: "auto".to_string(),
            swarm_max_subagents: 8,
            swarm_max_steps: 100,
            swarm_subagent_timeout_secs: 300,
            model: Some("openai-codex/gpt-5.1-codex".to_string()),
            json: false,
            no_s3: false,
        };

        let execution_outcome = ExecutionOutcome {
            detail: "run execution completed".to_string(),
            changed_files: vec!["src/forage/mod.rs".to_string()],
            quality_gates_passed: true,
        };
        record_execution_success_to_okr(&repo, &item, &args, &execution_outcome, 1)
            .await
            .expect("record success");

        let saved = repo
            .get_okr(okr_id)
            .await
            .expect("read okr")
            .expect("okr exists");
        let saved_kr = saved
            .key_results
            .into_iter()
            .find(|k| k.id == kr_id)
            .expect("kr exists");
        assert!(saved_kr.current_value > 0.0);
        assert_eq!(saved_kr.outcomes.len(), 1);
        assert!(
            saved_kr.outcomes[0]
                .evidence
                .iter()
                .any(|entry| entry.starts_with("engine:run"))
        );
    }

    #[tokio::test]
    async fn swarm_success_without_file_evidence_does_not_increment_progress() {
        let dir = tempdir().expect("create tempdir");
        let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

        let mut okr = Okr::new("Autonomous Business-Aligned Execution", "Test objective");
        okr.status = OkrStatus::Active;
        let mut kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
        kr.update_progress(10.0);
        let kr_id = kr.id;
        okr.add_key_result(kr);
        let okr_id = okr.id;
        let _ = repo.create_okr(okr).await.expect("create okr");

        let item = ForageOpportunity {
            score: 1.35,
            okr_id,
            okr_title: "Autonomous Business-Aligned Execution".to_string(),
            okr_status: OkrStatus::Active,
            key_result_id: kr_id,
            key_result_title: "KR1".to_string(),
            progress: 0.10,
            remaining: 0.90,
            target_date: None,
            moonshot_alignment: 0.0,
            moonshot_hits: Vec::new(),
            prompt: "test prompt".to_string(),
        };
        let args = ForageArgs {
            top: 3,
            loop_mode: false,
            interval_secs: 120,
            max_cycles: 1,
            execute: true,
            moonshots: Vec::new(),
            moonshot_file: None,
            moonshot_required: false,
            moonshot_min_alignment: 0.10,
            execution_engine: "swarm".to_string(),
            run_timeout_secs: 900,
            fail_fast: false,
            swarm_strategy: "auto".to_string(),
            swarm_max_subagents: 8,
            swarm_max_steps: 100,
            swarm_subagent_timeout_secs: 300,
            model: Some("openai-codex/gpt-5.1-codex".to_string()),
            json: false,
            no_s3: false,
        };
        let execution_outcome = ExecutionOutcome {
            detail: "swarm execution completed".to_string(),
            changed_files: Vec::new(),
            quality_gates_passed: true,
        };
        record_execution_success_to_okr(&repo, &item, &args, &execution_outcome, 2)
            .await
            .expect("record success");

        let saved = repo
            .get_okr(okr_id)
            .await
            .expect("read okr")
            .expect("okr exists");
        let saved_kr = saved
            .key_results
            .into_iter()
            .find(|k| k.id == kr_id)
            .expect("kr exists");
        assert_eq!(saved_kr.current_value, 10.0);
        assert_eq!(saved_kr.outcomes.len(), 1);
        assert!(
            saved_kr.outcomes[0]
                .evidence
                .iter()
                .any(|entry| entry == "concrete_file_evidence:false")
        );
    }

    #[tokio::test]
    async fn seed_default_okr_populates_empty_repo() {
        let dir = tempdir().expect("create tempdir");
        let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

        seed_default_okr_if_empty(&repo)
            .await
            .expect("seed should succeed");

        let okrs = repo.list_okrs().await.expect("list okrs");
        assert_eq!(okrs.len(), 1);
        assert_eq!(
            okrs[0].title,
            "Mission: Autonomous Business-Aligned Execution"
        );
        assert_eq!(okrs[0].status, OkrStatus::Active);
        assert_eq!(okrs[0].key_results.len(), 3);
    }

    #[tokio::test]
    async fn seed_default_okr_is_noop_when_repo_not_empty() {
        let dir = tempdir().expect("create tempdir");
        let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

        let mut existing = Okr::new("Existing Objective", "Do not overwrite");
        existing.status = OkrStatus::Active;
        existing.add_key_result(KeyResult::new(existing.id, "KR1", 100.0, "%"));
        let _ = repo.create_okr(existing).await.expect("create existing");

        seed_default_okr_if_empty(&repo)
            .await
            .expect("seed should succeed");

        let okrs = repo.list_okrs().await.expect("list okrs");
        assert_eq!(okrs.len(), 1);
        assert_eq!(okrs[0].title, "Existing Objective");
    }

    #[tokio::test]
    async fn moonshot_seed_creates_okr_when_no_opportunities_exist() {
        let dir = tempdir().expect("create tempdir");
        let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

        // Completed objective should not produce forage opportunities.
        let mut completed = Okr::new("Completed Objective", "Already done");
        completed.status = OkrStatus::Completed;
        let mut kr = KeyResult::new(completed.id, "KR done", 100.0, "%");
        kr.update_progress(100.0);
        completed.add_key_result(kr);
        let _ = repo.create_okr(completed).await.expect("create completed");

        let rubric = MoonshotRubric {
            goals: vec![
                "Automate customer acquisition end-to-end".to_string(),
                "Funnel conversion replaces manual sales".to_string(),
            ],
            required: true,
            min_alignment: 0.2,
        };

        let seeded_id = seed_moonshot_okr_if_no_opportunities(&repo, &rubric)
            .await
            .expect("seed should succeed");
        assert!(seeded_id.is_some(), "expected moonshot-derived seed OKR");
        let seeded_id = seeded_id.expect("seeded id");

        let okrs = repo.list_okrs().await.expect("list okrs");
        let seeded = okrs
            .iter()
            .find(|o| o.id == seeded_id)
            .expect("seeded okr exists");
        assert_eq!(seeded.status, OkrStatus::Active);
        assert_eq!(
            seeded.title,
            "Mission: Moonshot-Derived Autonomous Execution"
        );
        assert!(!seeded.key_results.is_empty());
    }

    #[tokio::test]
    async fn moonshot_seed_is_noop_when_open_moonshot_seed_exists() {
        let dir = tempdir().expect("create tempdir");
        let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

        let rubric = MoonshotRubric {
            goals: vec!["Tech stack is the moat".to_string()],
            required: true,
            min_alignment: 0.2,
        };
        let first = seed_moonshot_okr_if_no_opportunities(&repo, &rubric)
            .await
            .expect("first seed should succeed");
        assert!(first.is_some());

        let second = seed_moonshot_okr_if_no_opportunities(&repo, &rubric)
            .await
            .expect("second seed should succeed");
        assert!(
            second.is_none(),
            "should not duplicate active moonshot seed"
        );
    }
}
