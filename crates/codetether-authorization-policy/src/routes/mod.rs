//! Ordered server route-to-permission mapping.
//!
//! match_policy_rule preserves first-match precedence and path boundaries;
//! required_permission rejects unmapped routes before authorization evaluation.
mod activity;
mod administration;
mod configuration;
mod entry;
mod execution;
mod sessions;
mod task_workers;
mod tools;

/// Mapping from (path pattern, HTTP method) → OPA permission action.
/// The first matching rule wins.
struct PolicyRule {
    pattern: &'static str,
    methods: Option<&'static [&'static str]>,
    permission: &'static str,
}
const POLICY_RULE_GROUPS: &[&[PolicyRule]] = &[
    entry::RULES,
    tools::RULES,
    administration::RULES,
    activity::RULES,
    sessions::RULES,
    task_workers::RULES,
    execution::RULES,
    configuration::RULES,
];
mod lookup;
pub use lookup::{match_policy_rule, required_permission};
#[cfg(test)]
mod tests;
