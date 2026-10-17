//! Exercise the real tool adapter rather than duplicating its return formatting.
use super::*;
use codetether_question::Action;

#[tokio::test]
async fn question_tool_roundtrip_and_headless_rejection() {
    let input = json!({"question":"Choose?", "options":["Yes"], "__ct_session_id":"tool-question"});
    let rejected = QuestionTool::new().execute(input.clone()).await.unwrap();
    assert!(!rejected.success);
    let job = tokio::spawn(runtime::scope("tool-question".into(), async move {
        QuestionTool::new().execute(input).await.unwrap()
    }));
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while runtime::pending("tool-question").is_none() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    runtime::act("tool-question", Action::Submit);
    let result = job.await.unwrap();
    assert!(result.success);
    assert_eq!(
        serde_json::from_str::<Value>(&result.output).unwrap(),
        json!({"answers":[["Yes"]]})
    );
    assert!(
        !QuestionTool::new()
            .execute(json!({"questions":[]}))
            .await
            .unwrap()
            .success
    );
}
