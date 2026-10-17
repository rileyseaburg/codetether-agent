use super::*;
#[tokio::test]
async fn failed_optional_repair_returns_one_truthful_task() {
    let host = Mock::new(vec![ok("{}"), Err(anyhow::anyhow!("repair unavailable"))]);
    let plan = generate_plan("original task", "initial".into(), false, &host)
        .await
        .unwrap();
    assert_eq!(host.prompts.lock().unwrap().len(), 2);
    assert_eq!(plan.len(), 1);
    assert_eq!(plan.values().next().unwrap().instruction, "original task");
}
