//! Ordered command definitions for the existing forage quality workflow.
/// One quality command and its display/truncation policy.
pub(super) struct CheckSpec {
    pub command: &'static str,
    pub label: &'static str,
    pub limit: usize,
    pub empty: &'static str,
}
pub(super) const CHECKS: [CheckSpec; 2] = [
    CheckSpec {
        command: "cargo check --message-format=short",
        label: "cargo check",
        limit: 200,
        empty: "no errors",
    },
    CheckSpec {
        command: "cargo test --quiet -- --nocapture",
        label: "cargo test",
        limit: 300,
        empty: "no test output",
    },
];
