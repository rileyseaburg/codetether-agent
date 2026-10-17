use super::for_json;

#[test]
fn hashes_the_exact_json_body_with_the_task_id() {
    let payload = serde_json::json!({"task_id": "cttask_1", "status": "completed"});
    assert_eq!(
        for_json("cttask_1", &payload).unwrap(),
        "cttask_1:e31f9069a346d3c36e4731cab9d788c8f5bec6e6c9a39682112d6c330d7ddf2e"
    );
}
