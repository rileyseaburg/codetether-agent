//! Tests for persona creation, spawning, and lineage graphs.

use super::tests_request::{create_in_swarm, spawn};
use super::tests_support::test_runtime;

#[tokio::test]
async fn create_spawn_and_lineage_work() {
    let runtime = test_runtime();
    let root = runtime
        .create_persona(create_in_swarm(
            "root",
            "orchestrator",
            "coordinate",
            "swarm-a",
        ))
        .await
        .expect("root should be created");
    assert_eq!(root.identity.depth, 0);

    let child = runtime
        .spawn_child("root", spawn("child-1", "analyst", "analyze"))
        .await
        .expect("child should spawn");
    assert_eq!(child.identity.parent_id.as_deref(), Some("root"));
    assert_eq!(child.identity.depth, 1);

    let lineage = runtime.lineage_graph().await;
    assert_eq!(lineage.total_edges, 1);
    assert_eq!(lineage.roots, vec!["root".to_string()]);
}
