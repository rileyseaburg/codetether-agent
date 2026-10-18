//! Relay planning schemas and behavior-preserving transformations.
//!
//! Convert planner JSON into runtime specialists with [build_runtime_profile_from_plan]
//! and measurable objectives with [okr_from_planned_draft]. Default profiles and
//! objectives remain available when planning produces no usable result.
mod build_profile;
mod defaults;
mod instructions;
mod json;
mod names;
mod okr_default;
mod okr_planned;
mod planned_okr;
mod planned_profile;
mod profile;
mod responses;
mod text;
pub use build_profile::build_runtime_profile_from_plan;
pub use defaults::build_relay_profiles;
pub use json::extract_json_payload;
pub use okr_default::default_relay_okr_template;
pub use okr_planned::okr_from_planned_draft;
pub use planned_okr::{PlannedOkrDraft, PlannedOkrKeyResult};
pub use planned_profile::PlannedRelayProfile;
pub use profile::RelayProfile;
pub use responses::{PlannedRelayResponse, RelaySpawnDecision};
pub use text::truncate_with_ellipsis;
#[cfg(test)]
mod tests;
