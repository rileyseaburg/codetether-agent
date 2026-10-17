use super::*;
#[tokio::test]
async fn invalid_parallel_repair_errors_without_third_request() {
    let host = Mock::new(vec![ok("{}"), ok("{}")]);
    let error = generate_plan("task", "initial".into(), true, &host)
        .await
        .unwrap_err();
    assert_eq!(host.prompts.lock().unwrap().len(), 2);
    assert!(
        error
            .to_string()
            .contains("did not produce concurrent root tasks")
    );
}
