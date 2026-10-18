//! Focused process-local worker notification contract regressions.

#[path = "tests/body.rs"]
mod body;
#[path = "tests/filtering.rs"]
mod filtering;
#[path = "tests/fixtures.rs"]
mod fixtures;
#[path = "tests/idle_delivery.rs"]
mod idle_delivery;
#[path = "tests/lag_claim.rs"]
mod lag_claim;
#[path = "tests/lag_payload.rs"]
mod lag_payload;
#[path = "tests/lag_replay.rs"]
mod lag_replay;
#[path = "tests/lag_state.rs"]
mod lag_state;
#[path = "tests/payloads.rs"]
mod payloads;
#[path = "tests/recovery.rs"]
mod recovery;
#[path = "tests/repeated_lag.rs"]
mod repeated_lag;
#[path = "tests/repetition.rs"]
mod repetition;
#[path = "tests/snapshot_race.rs"]
mod snapshot_race;
#[path = "tests/snapshot_state.rs"]
mod snapshot_state;
