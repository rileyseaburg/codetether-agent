//! Cross-module behavior contracts for model and persistence boundaries.
use crate::*;
use uuid::Uuid;
mod disk_cache;
mod disk_updates;
mod queries;
mod query_fixture;
mod statistics;
mod workflow_progress;
mod workflow_state;
fn objective() -> Okr {
    let mut okr = Okr::new("Objective", "Description");
    okr.add_key_result(KeyResult::new(okr.id, "Success", 10.0, "count"));
    okr
}
