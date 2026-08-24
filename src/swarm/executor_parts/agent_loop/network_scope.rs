//! Trusted network authority scoped to one child agent loop.

use std::future::Future;

tokio::task_local! {
    static ALLOWED: bool;
}

#[cfg(test)]
#[tokio::test]
async fn inherited_authority_is_scoped_and_defaults_to_denied() {
    assert!(!allowed());
    assert!(run_with_network(true, async { allowed() }).await);
    assert!(!allowed());
}

/// Run a child loop with an explicit inherited network decision.
pub async fn run_with_network<T>(allowed: bool, future: impl Future<Output = T>) -> T {
    ALLOWED.scope(allowed, future).await
}

pub(super) fn allowed() -> bool {
    ALLOWED.try_with(|allowed| *allowed).unwrap_or(false)
}
