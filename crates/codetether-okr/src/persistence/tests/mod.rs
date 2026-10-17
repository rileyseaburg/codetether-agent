//! File-repository regression suite.

use super::*;
use crate::{KeyResult, Okr, OkrRun, OkrRunStatus, OkrStatus};
use uuid::Uuid;

fn temp_repo() -> OkrRepository {
    let temp_dir = std::env::temp_dir().join(format!("okr_test_{}", Uuid::new_v4()));
    OkrRepository::new(temp_dir)
}
mod create_and_get_okr;
mod crud_runs;
mod delete_okr;
mod list_and_stats;
mod query_okrs_by_status;
mod update_okr;
