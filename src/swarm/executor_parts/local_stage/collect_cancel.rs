use crate::swarm::SwarmControl;
use std::collections::HashMap;
use tokio::task::AbortHandle;

pub(super) async fn requested(control: Option<&SwarmControl>) {
    match control {
        Some(control) => control.cancelled().await,
        None => std::future::pending().await,
    }
}

pub(super) fn abort_all(aborts: &HashMap<String, AbortHandle>) {
    for abort in aborts.values() {
        abort.abort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn aborts_every_inflight_agent() {
        let handle = tokio::spawn(std::future::pending::<()>());
        let mut aborts = HashMap::new();
        aborts.insert("agent".into(), handle.abort_handle());
        abort_all(&aborts);
        assert!(handle.await.expect_err("agent should abort").is_cancelled());
    }
}
