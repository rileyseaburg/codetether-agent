use super::args::ImagegenArgs;

#[test]
fn accepts_runtime_provenance_fields() {
    let args: ImagegenArgs = serde_json::from_value(serde_json::json!({
        "prompt": "paint a moonlit lake",
        "__ct_agent_name": "build",
        "__ct_session_id": "session-1"
    }))
    .expect("runtime metadata must not invalidate model arguments");
    assert_eq!(args.prompt, "paint a moonlit lake");
}
