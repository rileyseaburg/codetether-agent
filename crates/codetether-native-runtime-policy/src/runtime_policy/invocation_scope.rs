//! Compatibility exports for shared approval scope derivation.

#[cfg(test)]
pub(super) use codetether_runtime_policy::invocation_scope::for_tool;

#[cfg(test)]
#[path = "invocation_scope_tests.rs"]
mod tests;
