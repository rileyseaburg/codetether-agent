use super::*;
#[tokio::test]
async fn accepts_first_plan_without_repair() {
    let host = Mock::new(vec![ok(PARALLEL)]);
    let plan = generate_plan("task", "initial".into(), true, &host)
        .await
        .unwrap();
    assert_eq!(plan.len(), 2);
    assert!(plan.values().all(|task| task.stage == 0));
    assert_eq!(*host.prompts.lock().unwrap(), vec!["initial"]);
}
