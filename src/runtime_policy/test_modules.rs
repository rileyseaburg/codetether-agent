// Test module wiring for runtime_policy, kept separate so `mod.rs`
// stays focused on the public policy surface.
#[path = "approval_tests.rs"]
mod approval_tests;
#[path = "command_tests.rs"]
mod command_tests;
#[path = "justification_scope_tests.rs"]
mod justification_scope_tests;
#[path = "justification_tests.rs"]
mod justification_tests;
#[path = "patch_invocation_tests.rs"]
mod patch_invocation_tests;
#[path = "permission_tests.rs"]
mod permission_tests;
#[path = "session_grants_tests.rs"]
mod session_grants_tests;
#[path = "tests.rs"]
mod tests;
#[path = "workspace_tests.rs"]
mod workspace_tests;
