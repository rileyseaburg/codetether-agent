//! Capability registration behavior.

use super::*;

#[test]
fn worker_capabilities_include_env_additions_without_duplicates() {
    with_worker_capability_env(
        Some("persistent,persistent-workspace git-clone"),
        Some("persistent,repo-cache"),
        None,
        || {
            let capabilities = worker_capabilities();
            assert!(capabilities.iter().any(|cap| cap == "persistent"));
            assert!(capabilities.iter().any(|cap| cap == "persistent-workspace"));
            assert!(capabilities.iter().any(|cap| cap == "git-clone"));
            assert!(capabilities.iter().any(|cap| cap == "repo-cache"));
            assert_eq!(
                capabilities
                    .iter()
                    .filter(|cap| cap.as_str() == "persistent")
                    .count(),
                1
            );
        },
    );
}
