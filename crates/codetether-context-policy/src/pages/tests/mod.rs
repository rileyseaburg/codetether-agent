//! Page policy regression coverage.
use super::*;
mod fixtures;
use fixtures::*;
mod classify_all_is_parallel;
mod classify_defaults_user_and_assistant_to_conversation;
mod classify_routes_system_to_bootstrap;
mod classify_routes_tool_results_to_evidence;
mod degradation_paths_start_at_full;
mod min_fidelity_never_drops_below_structured_for_invariant_pages;
