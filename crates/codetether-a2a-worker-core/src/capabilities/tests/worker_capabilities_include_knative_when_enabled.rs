//! Capability registration behavior.

use super::*;

#[test]
fn worker_capabilities_include_knative_when_enabled() {
    with_worker_capability_env(None, None, Some("true"), || {
        let capabilities = worker_capabilities();
        assert!(capabilities.iter().any(|cap| cap == "knative"));
    });
}
