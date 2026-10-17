use super::*;
#[tokio::test]
async fn repairs_invalid_plan_once() {
    let host = Mock::new(vec![ok("{}"), ok(PARALLEL)]);
    let plan = generate_plan("original task", "initial".into(), true, &host)
        .await
        .unwrap();
    assert_eq!(plan.len(), 2);
    let prompts = host.prompts.lock().unwrap();
    assert_eq!(prompts.len(), 2);
    assert_eq!(prompts[0], "initial");
    assert!(prompts[1].contains("original task"));
    assert!(prompts[1].contains("Invalid response:\n{}"));
}
