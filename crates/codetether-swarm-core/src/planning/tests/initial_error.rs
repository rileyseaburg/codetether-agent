use super::*;
#[tokio::test]
async fn initial_provider_error_is_returned_without_repair() {
    let host = Mock::new(vec![Err(anyhow::anyhow!("provider unavailable"))]);
    let error = generate_plan("task", "initial".into(), false, &host)
        .await
        .unwrap_err();
    assert_eq!(host.prompts.lock().unwrap().len(), 1);
    assert_eq!(error.to_string(), "provider unavailable");
}
