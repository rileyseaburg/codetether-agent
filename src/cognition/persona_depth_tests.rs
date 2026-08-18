//! Test for spawn-depth enforcement.

use super::tests_request::{create_in_swarm, spawn};
use super::tests_support::test_runtime;

#[tokio::test]
async fn spawn_depth_limit_is_enforced() {
    let runtime = test_runtime();
    runtime
        .create_persona(create_in_swarm(
            "root",
            "orchestrator",
            "coordinate",
            "swarm-a",
        ))
        .await
        .expect("root should be created");
    runtime
        .spawn_child("root", spawn("c1", "worker", "run"))
        .await
        .expect("depth 1 should be allowed");
    runtime
        .spawn_child("c1", spawn("c1-1", "worker", "run"))
        .await
        .expect("depth 2 should be allowed");
    assert!(
        runtime
            .spawn_child("c1-1", spawn("c1-1-1", "worker", "run"))
            .await
            .is_err(),
        "depth 3 should exceed the spawn-depth limit"
    );
}
