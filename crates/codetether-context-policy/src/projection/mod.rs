//! Message projection with source coverage and tool-pair alignment.
//!
//! Summary insertion yields messages plus [incremental_types::MessageOrigin]
//! records; pairing repair and coverage computation keep those arrays aligned.
mod derived_context;
/// Derived context record and count-change utility.
pub mod helpers;
/// incremental coverage.
pub mod incremental_coverage;
#[cfg(test)]
mod incremental_coverage_tests;
/// incremental insert.
pub mod incremental_insert;
/// incremental repair.
pub mod incremental_repair;
/// incremental repair drop.
pub mod incremental_repair_drop;
/// incremental repair inject.
pub mod incremental_repair_inject;
#[cfg(test)]
mod incremental_repair_tests;
/// incremental types.
pub mod incremental_types;

/// Pairing-safe pass-through when the whole context fits.
pub mod incremental_below_budget;
/// Active-turn-safe overflow clamping.
pub mod incremental_clamp;
mod incremental_tail_clamp;
