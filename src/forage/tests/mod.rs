//! Regression tests for application configuration, selection hooks and progress adapters.
//! Existing assertions are retained; common argument and opportunity fixtures
//! keep each behavioral test focused.
use super::{
    ExecutionOutcome, ForageOpportunity, MoonshotRubric, build_swarm_config, collect_opportunities,
    collect_opportunities_with_rubric, record_execution_success_to_okr,
};
use crate::cli::ForageArgs;
use crate::okr::{KeyResult, Okr, OkrStatus};
use tempfile::tempdir;
use uuid::Uuid;
mod args;
mod collect_opportunities_ranks_remaining_work;
mod collect_opportunities_skips_complete_or_cancelled_work;
mod forage_timeout_bounds_are_clamped;
mod moonshot_rubric_filters_low_alignment_work;
mod opportunity;
mod record_execution_success_updates_kr_progress_and_evidence;
mod swarm_config_does_not_force_legacy_model_fallback;
mod swarm_config_preserves_explicit_model_override;
mod swarm_success_without_file_evidence_does_not_increment_progress;
