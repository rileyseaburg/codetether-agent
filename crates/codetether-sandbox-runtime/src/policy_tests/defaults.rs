//! Keep sandbox memory limits opt-in by default.
use crate::SandboxPolicy;
#[test]
fn sandbox_memory_limit_is_opt_in_by_default() {
    assert_eq!(SandboxPolicy::default().max_memory_bytes, 0);
}
