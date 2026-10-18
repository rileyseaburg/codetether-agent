//! Test for branching-factor enforcement.

use super::tests_request::{create_in_swarm, spawn};
use super::tests_support::test_runtime;

#[tokio::test]
async fn branching_limit_is_enforced() {
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

    for id in ["c1", "c2"] {
        runtime
            .spawn_child("root", spawn(id, "worker", "run"))
            .await
            .expect("child within branching limit should spawn");
    }
    assert!(
        runtime
            .spawn_child("root", spawn("c3", "worker", "run"))
            .await
            .is_err(),
        "third child should exceed the branching limit"
    );
}
