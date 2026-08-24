use crate::approval::{test_env::ScopedEnv, test_env::lock_env};
use crate::session::SessionEvent;
use crate::tool::Tool;
use serde_json::json;

#[path = "gate_generic_tool.rs"]
mod generic_tool;
#[path = "gate_retry_support.rs"]
mod support;

#[tokio::test]
async fn live_gate_reviews_generic_receipt_without_consuming_it() {
    let _lock = lock_env();
    let _queue = support::QueueGuard::new();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), crate::config::AccessMode::Ask);
    let args = json!({
        "value": "one execution",
        "__ct_parent_workspace": data.path(),
        "__ct_session_id": "generic-retry",
    });
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let workspace = data.path().to_path_buf();
    let gate = tokio::spawn(async move {
        super::gate::gate(&workspace, &tx, "call-generic", "generic_mutator", args).await
    });
    let SessionEvent::ApprovalRequest(request) = rx.recv().await.expect("approval event") else {
        panic!("expected approval request");
    };
    crate::tui::app::state::approval_queue::push(request.clone());
    let mut app = crate::tui::app::state::App::default();
    let command = format!("/approve {} generic retry", request.approval_id);
    assert!(crate::tui::app::input::approval_command::run(
        &mut app, &command
    ));
    let (approved, blocked) = gate.await.expect("gate task").into_parts();
    assert!(blocked.is_none());

    assert!(
        crate::runtime_policy::evaluate_tool_invocation("generic_mutator", &approved)
            .await
            .is_none()
    );
    let tool = generic_tool::CountingTool::default();
    let result = tool.execute(approved.clone()).await.expect("execute");
    assert!(result.success, "{}", result.output);
    assert_eq!(tool.count(), 1);
    assert!(
        crate::runtime_policy::evaluate_tool_invocation("generic_mutator", &approved)
            .await
            .is_some()
    );
}
