//! Shared scoring, prompt normalization, progress and seeding regressions.
//! These tests exercise the crate APIs without application adapters.
use crate::progress::success_progress_increment_ratio;
use crate::seed::{seed_default_okr_if_empty, seed_moonshot_okr_if_no_opportunities};
use crate::{MoonshotRubric, normalize_prompt_field, status_weight, urgency_bonus};
use chrono::{Duration, Utc};
use codetether_okr::{KeyResult, Okr, OkrStatus};
use tempfile::tempdir;
use uuid::Uuid;
mod moonshot_seed_creates_okr_when_no_opportunities_exist;
mod moonshot_seed_is_noop_when_open_moonshot_seed_exists;
mod normalize_prompt_field_compacts_and_truncates;
mod seed_default_okr_is_noop_when_repo_not_empty;
mod seed_default_okr_populates_empty_repo;
mod status_weight_prioritizes_active_okrs;
mod success_progress_increment_ratio_is_bounded;
mod urgency_bonus_increases_for_due_dates;
