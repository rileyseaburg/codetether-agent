use crate::session::Session;
use serde_json::Value;

#[tokio::test]
async fn agent_runtime_overwrites_model_policy_fields() {
    let mut session = Session::new().await.expect("session");
    session.metadata.prior_context_allowed = Some(false);
    let value: Value = super::tool_input::prepare(
        &session,
        r#"{"__ct_session_id":"fake","__ct_prior_context_allowed":true}"#,
    )
    .expect("prepared input");
    assert_eq!(value["__ct_session_id"], session.id);
    assert_eq!(value["__ct_prior_context_allowed"], false);
}
