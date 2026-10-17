//! Parent identity propagation through spawn request parsing.
use crate::SpawnRequest;
use crate::test_helpers::params;
use serde_json::json;

#[test]
fn parent_session_id_is_preserved() {
    let p = params(json!({
        "action": "spawn", "name": "reviewer", "instructions": "audit",
        "model": "zai/glm-5.1", "__ct_session_id": "parent-42",
    }));
    let request = SpawnRequest::from_params(&p).expect("spawn request");
    assert_eq!(request.parent_session_id, Some("parent-42"));
}
