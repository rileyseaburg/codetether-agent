use super::until_idle_with_timeout;
use crate::tool::agent::execution_state;
use std::time::Duration;

#[tokio::test]
async fn shutdown_wait_has_a_deadline() {
    let agent_id = format!("shutdown-timeout-{}", uuid::Uuid::new_v4());
    let guard = execution_state::try_start(&agent_id).unwrap();
    assert!(!until_idle_with_timeout(&agent_id, Duration::from_millis(1)).await);
    drop(guard);
}
