#[path = "network_approval_support.rs"]
mod network_approval_support;
use super::*;

#[path = "approval_once_tests.rs"]
mod approval_once_tests;
#[path = "buffer_integration_tests.rs"]
mod buffer_integration_tests;
#[path = "direct_policy_tests.rs"]
mod direct_policy_tests;
#[path = "direct_unsafe_tests.rs"]
mod direct_unsafe_tests;
#[cfg(unix)]
#[path = "default_cwd_scope_tests.rs"]
mod default_cwd_scope_tests;
#[path = "escalation_tests.rs"]
mod escalation_tests;
#[path = "liveness_tests.rs"]
mod liveness_tests;
#[path = "owner_isolation_tests.rs"]
mod owner_isolation_tests;
#[cfg(unix)]
#[path = "network_env_support.rs"]
mod network_env_support;
#[cfg(unix)]
#[path = "network_approval_tests.rs"]
mod network_approval_tests;
#[cfg(unix)]
#[path = "network_escalation_tests.rs"]
mod network_escalation_tests;
#[cfg(unix)]
#[path = "network_test_support.rs"]
mod network_test_support;
#[cfg(unix)]
#[path = "network_scope_tests.rs"]
mod network_scope_tests;
#[cfg(unix)]
#[path = "network_session_scope_tests.rs"]
mod network_session_scope_tests;
#[cfg(unix)]
#[path = "pty_tests.rs"]
mod pty_tests;
#[cfg(unix)]
#[path = "sandbox_tests.rs"]
mod sandbox_tests;
#[path = "sandbox_dev_null_tests.rs"]
mod sandbox_dev_null_tests;
#[path = "tests.rs"]
mod tests;
#[path = "tool_tests.rs"]
mod tool_tests;